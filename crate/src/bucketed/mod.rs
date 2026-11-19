/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! A dynamic sorted set of byte strings based on a z-fast trie indexing
//! buckets of keys.

mod check;

use crate::dynamic::ZFastTrie;
use crate::dynamic::key::Key;
use crate::utils::{HashPrefixes, Xxh3, lcp, prefetch};
use std::cmp::Ordering;
use std::fmt;
use xxhash_rust::xxh3::xxh3_64;

/// The maximum number of keys in a bucket.
const CAP: usize = 64;
/// The identifier of no bucket.
const NONE: u32 = u32::MAX;
/// Two adjacent buckets are merged when they contain overall at most this
/// number of keys.
const MERGE: usize = CAP / 2;

/// A bucket: a sorted sequence of at most [`CAP`] keys.
///
/// Besides the keys, a bucket contains two summaries of each key, which make
/// it possible to search without accessing the keys:
///
/// - a *tag*, that is, 16 bits of a hash of the key, used to look for a
///   given key: only the keys with the same tag have to be compared;
///
/// - a *head*, that is, the eight bytes that follow a prefix common to all
///   keys of the bucket, as a big-endian integer: for strings starting with
///   that prefix, the order of heads agrees with the order of the strings, so
///   the position of a string can be found by comparing heads, and only keys
///   with the same head have to be compared.
///
/// A string known to lie between the first key of this bucket and the first
/// key of the next one does not necessarily start with the common prefix: we
/// check that with a hash of the prefix, and confirm with the first key
/// comparison.
#[repr(C)]
struct Bucket {
    /// The number of keys.
    len: u16,
    /// The length of a prefix common to all keys: heads start there.
    prefix_len: u32,
    /// The previous bucket, or [`NONE`].
    prev: u32,
    /// The next bucket, or [`NONE`].
    next: u32,
    /// The hash of the common prefix.
    prefix_hash: u64,
    /// The tag of each key.
    tags: [u16; CAP],
    /// The head of each key.
    heads: [u64; CAP],
    /// The keys, in order; `None` after the first `len` positions.
    keys: [Option<Key>; CAP],
}

/// The number of cache lines containing the first fields of a bucket, up to
/// the tags.
const TAG_LINES: usize = (24 + 2 * CAP).div_ceil(64);
/// The number of cache lines containing the first fields of a bucket, up to
/// the heads.
const HEAD_LINES: usize = (24 + 10 * CAP).div_ceil(64);

/// Returns the tag of a key.
#[inline(always)]
fn tag(x: &[u8]) -> u16 {
    xxh3_64(x) as u16
}

/// Returns the eight bytes of `x` starting at position `p`, padded with
/// zeros, as a big-endian integer.
#[inline(always)]
fn head(x: &[u8], p: usize) -> u64 {
    let tail = &x[p.min(x.len())..];
    if tail.len() >= 8 {
        u64::from_be_bytes(tail[..8].try_into().unwrap())
    } else {
        let mut bytes = [0; 8];
        bytes[..tail.len()].copy_from_slice(tail);
        u64::from_be_bytes(bytes)
    }
}

impl Bucket {
    fn new() -> Self {
        Self {
            keys: [const { None }; CAP],
            heads: [0; CAP],
            tags: [0; CAP],
            len: 0,
            prefix_len: 0,
            prefix_hash: xxh3_64(&[]),
            prev: NONE,
            next: NONE,
        }
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.len as usize
    }

    #[inline(always)]
    fn key(&self, i: usize) -> &[u8] {
        match &self.keys[i] {
            Some(key) => key.bytes(),
            None => unreachable!("missing key"),
        }
    }

    /// Returns a bit mask of the positions whose tag is `t`.
    #[inline(always)]
    fn tag_mask(&self, t: u16) -> u64 {
        let mut mask = 0;
        for i in 0..CAP {
            mask |= ((self.tags[i] == t) as u64) << i;
        }
        // Only the first len positions are meaningful
        let valid = if self.len == 0 {
            0
        } else {
            u64::MAX >> (CAP - self.len())
        };
        mask & valid
    }

    /// Returns the position of `x`, if it is a key of this bucket.
    #[inline(always)]
    fn find(&self, x: &[u8]) -> Option<usize> {
        let mut mask = self.tag_mask(tag(x));
        while mask != 0 {
            let i = mask.trailing_zeros() as usize;
            if self.key(i) == x {
                return Some(i);
            }
            mask &= mask - 1;
        }
        None
    }

    /// Returns the number of keys smaller than `x`, and whether `x` is a
    /// key, by binary search on the keys in the given range of positions,
    /// which must contain all keys that are neither known to be smaller than
    /// `x` nor known to be larger.
    fn search_keys(&self, x: &[u8], mut lo: usize, mut hi: usize) -> (usize, bool) {
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            match self.key(mid).cmp(x) {
                Ordering::Less => lo = mid + 1,
                Ordering::Equal => return (mid, true),
                Ordering::Greater => hi = mid,
            }
        }
        (lo, false)
    }

    /// Returns the number of keys smaller than `x`, and whether `x` is a
    /// key.
    #[inline]
    fn search(&self, x: &[u8]) -> (usize, bool) {
        let n = self.len();
        let p = self.prefix_len as usize;
        if n == 0 {
            return (0, false);
        }
        // If x is a key, tags find it with a single comparison
        if let Some(i) = self.find(x) {
            return (i, true);
        }
        if x.len() >= p && xxh3_64(&x[..p]) == self.prefix_hash {
            // Very likely x starts with the common prefix: we count the
            // heads smaller than, and not larger than, that of x.
            let h = head(x, p);
            let (mut lo, mut hi) = (0, 0);
            for i in 0..CAP {
                lo += (self.heads[i] < h) as usize;
                hi += (self.heads[i] <= h) as usize;
            }
            // Unused heads are zero
            let unused = CAP - n;
            let (lo, hi) = (lo - if h > 0 { unused } else { 0 }, hi - unused);
            // We confirm that x starts with the common prefix using a key
            // that we would have to compare anyway
            let probe = if lo < hi {
                lo + (hi - lo) / 2
            } else {
                lo.min(n - 1)
            };
            let key = self.key(probe);
            if lcp(x, key) >= p {
                return if lo == hi {
                    (lo, false)
                } else {
                    match key.cmp(x) {
                        Ordering::Less => self.search_keys(x, probe + 1, hi),
                        Ordering::Equal => (probe, true),
                        Ordering::Greater => self.search_keys(x, lo, probe),
                    }
                };
            }
        }
        self.search_keys(x, 0, n)
    }

    /// Recomputes the common prefix and the heads.
    fn rebuild(&mut self) {
        let n = self.len();
        let p = if n == 0 {
            0
        } else {
            lcp(self.key(0), self.key(n - 1))
        };
        self.set_prefix(p);
    }

    /// Sets the length of the common prefix, which must be not larger than
    /// the length of the longest common prefix of the keys, and recomputes
    /// the heads.
    fn set_prefix(&mut self, p: usize) {
        let n = self.len();
        self.prefix_len = p as u32;
        self.prefix_hash = if n == 0 {
            xxh3_64(&[])
        } else {
            xxh3_64(&self.key(0)[..p])
        };
        for i in 0..n {
            self.heads[i] = head(self.key(i), p);
        }
        self.heads[n..].fill(0);
    }

    /// Inserts a key in the given position, which must be the number of
    /// keys smaller than the key.
    fn insert(&mut self, i: usize, x: &[u8]) {
        let n = self.len();
        debug_assert!(n < CAP && i <= n);
        self.keys[i..=n].rotate_right(1);
        self.heads.copy_within(i..n, i + 1);
        self.tags.copy_within(i..n, i + 1);
        self.keys[i] = Some(Key::new(x));
        self.tags[i] = tag(x);
        self.len += 1;
        if n == 0 {
            self.set_prefix(x.len());
        } else {
            // The new key might not start with the common prefix
            let neighbor = if i > 0 { i - 1 } else { 1 };
            let p = lcp(x, self.key(neighbor)).min(self.prefix_len as usize);
            if p < self.prefix_len as usize {
                self.set_prefix(p);
            } else {
                self.heads[i] = head(x, p);
            }
        }
    }

    /// Removes the key in the given position.
    fn remove(&mut self, i: usize) {
        let n = self.len();
        self.keys[i] = None;
        self.keys[i..n].rotate_left(1);
        self.heads.copy_within(i + 1..n, i);
        self.tags.copy_within(i + 1..n, i);
        self.heads[n - 1] = 0;
        self.len -= 1;
    }

    /// Moves the keys from position `from` on to the end of another bucket.
    fn move_to(&mut self, from: usize, other: &mut Bucket) {
        let (n, m) = (self.len(), other.len());
        let count = n - from;
        debug_assert!(m + count <= CAP);
        for i in 0..count {
            other.keys[m + i] = self.keys[from + i].take();
            other.tags[m + i] = self.tags[from + i];
        }
        self.len = from as u16;
        other.len = (m + count) as u16;
        self.heads[from..].fill(0);
        self.rebuild();
        other.rebuild();
    }
}

/// A dynamic sorted set of byte strings based on a z-fast trie indexing
/// buckets of keys.
///
/// This structure provides the same operations of a [`ZFastTrie`] (insertions,
/// deletions, membership, predecessor and successor queries, and iteration in
/// lexicographic order), but it is organized in two levels, like a B-tree with
/// a single level of leaves: the keys are stored in *buckets* of at most 64
/// keys, and a [`ZFastTrie`] indexes the buckets, each of which is associated
/// with a string (its *anchor*) that is not larger than its keys and larger
/// than the keys of the previous bucket. Anchors are as short as possible.
///
/// An operation locates a bucket with a predecessor query on the trie, and
/// then works inside the bucket, where searching usually requires accessing a
/// single key (see the implementation notes below). Since the trie contains
/// one string per bucket, it is small, and it usually fits a fast level of the
/// memory hierarchy even when the keys do not: thus, an operation usually
/// causes two cache misses (the bucket and a key), whereas in a [`ZFastTrie`],
/// where every key is a leaf, each probe of the fat binary search is a cache
/// miss when the trie is large. The trie is modified only when a bucket is
/// split or merged with an adjacent one, so updates are much faster, too, and
/// the space overhead per key is much smaller.
///
/// Thus, this is the structure of choice in practice, unless the
/// distribution of the keys is so skewed that the worst-case guarantees of a
/// [`ZFastTrie`] matter: the time bounds of this structure depend on those of
/// the trie only through the cost of locating a bucket.
///
/// # Examples
///
/// ```
/// # use zfast::ZFastSet;
/// let mut set = <ZFastSet>::new();
/// assert!(set.insert("foo"));
/// assert!(set.insert("bar"));
/// assert!(set.insert("foobar"));
/// assert!(!set.insert("foo"));
/// assert_eq!(set.len(), 3);
///
/// assert!(set.contains("bar"));
/// assert!(!set.contains("fo"));
/// assert_eq!(set.succ("c"), Some("foo".as_bytes()));
/// assert_eq!(set.pred("c"), Some("bar".as_bytes()));
/// assert_eq!(set.succ_strict("foo"), Some("foobar".as_bytes()));
///
/// assert!(set.remove("foo"));
/// let keys: Vec<&[u8]> = set.iter().collect();
/// assert_eq!(keys, vec!["bar".as_bytes(), "foobar".as_bytes()]);
/// ```
///
/// # Implementation Notes
///
/// A bucket stores, for each key, a 16-bit *tag* (part of a hash of the key)
/// and a 64-bit *head* (the eight bytes following a prefix common to all keys
/// of the bucket, as a big-endian integer). Membership compares the tag of the
/// query with all tags at once, and then compares the query only with the keys
/// having the same tag (usually, at most one). Ordered searches compare the
/// head of the query with all heads at once, which is sufficient to find the
/// position of the query unless some keys have the same head; in any case, at
/// least one key is compared to confirm that the query starts with the common
/// prefix, which is first checked using a hash.
pub struct ZFastSet<H = Xxh3> {
    /// The trie mapping anchors to buckets. It always contains the empty
    /// string, which is the anchor of the first bucket.
    trie: ZFastTrie<H, u32>,
    /// The buckets.
    buckets: Vec<Bucket>,
    /// The indices of the free buckets.
    free: Vec<u32>,
    /// The first bucket.
    head: u32,
    /// The last bucket.
    tail: u32,
    /// The number of keys.
    len: usize,
}

// SAFETY: see the analogous implementations for ZFastTrie; here keys are not
// even shared.
unsafe impl<H> Send for ZFastSet<H> {}
unsafe impl<H> Sync for ZFastSet<H> {}

impl<H: HashPrefixes> Default for ZFastSet<H> {
    fn default() -> Self {
        Self::new()
    }
}

impl<H: HashPrefixes> ZFastSet<H> {
    /// Creates a new empty set.
    pub fn new() -> Self {
        Self::with_sig_bits(64)
    }

    /// Creates a new empty set whose trie uses signatures with the given
    /// number of bits.
    ///
    /// This method is useful only to test the behavior of the structure in
    /// the presence of signature collisions.
    #[doc(hidden)]
    pub fn with_sig_bits(sig_bits: u32) -> Self {
        let mut trie = ZFastTrie::with_sig_bits(sig_bits);
        trie.insert_with([], 0);
        Self {
            trie,
            buckets: vec![Bucket::new()],
            free: Vec::new(),
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    /// Returns the number of keys.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if there are no keys.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Removes all keys.
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    /// Returns the number of bytes occupied by this structure, including the
    /// keys.
    pub fn mem_size(&self) -> usize {
        let mut keys = 0;
        let mut b = self.head;
        while b != NONE {
            let bucket = &self.buckets[b as usize];
            keys += (0..bucket.len())
                .map(|i| 8 + bucket.key(i).len())
                .sum::<usize>();
            b = bucket.next;
        }
        size_of::<Self>() - size_of_val(&self.trie)
            + self.trie.mem_size()
            + self.buckets.capacity() * size_of::<Bucket>()
            + self.free.capacity() * size_of::<u32>()
            + keys
    }

    /// Returns the number of buckets.
    #[doc(hidden)]
    pub fn num_buckets(&self) -> usize {
        self.buckets.len() - self.free.len()
    }

    /// Returns the bucket that contains, or would contain, `x`, prefetching
    /// its first `lines` cache lines.
    #[inline(always)]
    fn locate(&self, x: &[u8], lines: usize) -> u32 {
        let b = match self.trie.pred_entry(x) {
            Some((_, b)) => b,
            None => unreachable!("the empty anchor is always present"),
        };
        let ptr = &self.buckets[b as usize] as *const Bucket as *const u8;
        for line in 0..lines {
            prefetch(ptr.wrapping_add(64 * line));
        }
        b
    }

    /// Returns true if `x` is a key.
    #[inline]
    pub fn contains(&self, x: impl AsRef<[u8]>) -> bool {
        let x = x.as_ref();
        self.buckets[self.locate(x, TAG_LINES) as usize]
            .find(x)
            .is_some()
    }

    /// Returns the bucket and the position of the smallest key that is
    /// larger than or equal to `x` (larger than `x`, if `strict` is true),
    /// if any.
    #[inline]
    fn succ_pos(&self, x: &[u8], strict: bool) -> Option<(u32, usize)> {
        let mut b = self.locate(x, HEAD_LINES);
        let bucket = &self.buckets[b as usize];
        let (i, found) = bucket.search(x);
        let mut i = i + (found && strict) as usize;
        // Only the first bucket can be empty
        while i >= self.buckets[b as usize].len() {
            b = self.buckets[b as usize].next;
            if b == NONE {
                return None;
            }
            i = 0;
        }
        Some((b, i))
    }

    /// Returns the bucket and the position of the largest key that is
    /// smaller than or equal to `x` (smaller than `x`, if `strict` is true),
    /// if any.
    #[inline]
    fn pred_pos(&self, x: &[u8], strict: bool) -> Option<(u32, usize)> {
        let mut b = self.locate(x, HEAD_LINES);
        let bucket = &self.buckets[b as usize];
        let (i, found) = bucket.search(x);
        let mut i = i + (found && !strict) as usize;
        while i == 0 {
            b = self.buckets[b as usize].prev;
            if b == NONE {
                return None;
            }
            i = self.buckets[b as usize].len();
        }
        Some((b, i - 1))
    }

    #[inline(always)]
    fn key_at(&self, pos: Option<(u32, usize)>) -> Option<&[u8]> {
        pos.map(|(b, i)| self.buckets[b as usize].key(i))
    }

    /// Returns the largest key that is smaller than or equal to `x`, if any.
    #[inline]
    pub fn pred(&self, x: impl AsRef<[u8]>) -> Option<&[u8]> {
        self.key_at(self.pred_pos(x.as_ref(), false))
    }

    /// Returns the largest key that is smaller than `x`, if any.
    #[inline]
    pub fn pred_strict(&self, x: impl AsRef<[u8]>) -> Option<&[u8]> {
        self.key_at(self.pred_pos(x.as_ref(), true))
    }

    /// Returns the smallest key that is larger than or equal to `x`, if any.
    #[inline]
    pub fn succ(&self, x: impl AsRef<[u8]>) -> Option<&[u8]> {
        self.key_at(self.succ_pos(x.as_ref(), false))
    }

    /// Returns the smallest key that is larger than `x`, if any.
    #[inline]
    pub fn succ_strict(&self, x: impl AsRef<[u8]>) -> Option<&[u8]> {
        self.key_at(self.succ_pos(x.as_ref(), true))
    }

    /// Returns the smallest key, if any.
    pub fn first(&self) -> Option<&[u8]> {
        self.iter().next()
    }

    /// Returns the largest key, if any.
    pub fn last(&self) -> Option<&[u8]> {
        self.iter().next_back()
    }

    /// Returns an iterator on the keys in lexicographic order.
    pub fn iter(&self) -> Iter<'_, H> {
        Iter {
            set: self,
            front: (self.head, 0),
            back: (self.tail, self.buckets[self.tail as usize].len()),
            remaining: self.len,
        }
    }

    /// Returns an iterator on the keys larger than or equal to `x`, in
    /// lexicographic order.
    pub fn iter_from(&self, x: impl AsRef<[u8]>) -> impl Iterator<Item = &[u8]> + '_ {
        let mut pos = self.succ_pos(x.as_ref(), false);
        std::iter::from_fn(move || {
            let (b, i) = pos?;
            let bucket = &self.buckets[b as usize];
            pos = if i + 1 < bucket.len() {
                Some((b, i + 1))
            } else if bucket.next != NONE {
                Some((bucket.next, 0))
            } else {
                None
            };
            Some(bucket.key(i))
        })
    }

    /// Adds a key to the set, returning true if the key was not already in
    /// the set.
    ///
    /// # Panics
    ///
    /// If the key is longer than [`ZFastTrie::MAX_KEY_LEN`] bytes.
    pub fn insert(&mut self, x: impl AsRef<[u8]>) -> bool {
        let x = x.as_ref();
        assert!(x.len() <= <ZFastTrie<H>>::MAX_KEY_LEN, "key too long");
        let mut b = self.locate(x, HEAD_LINES);
        let (mut i, found) = self.buckets[b as usize].search(x);
        if found {
            return false;
        }
        if self.buckets[b as usize].len() == CAP {
            // When keys are added at one end of a bucket, as it happens with
            // sorted insertions, we leave the other part almost full
            let at = match i {
                0 => CAP / 8,
                CAP => CAP - CAP / 8,
                _ => CAP / 2,
            };
            self.split(b, at);
            // The key goes into one of the two halves, depending on the
            // new anchor
            b = self.locate(x, HEAD_LINES);
            i = self.buckets[b as usize].search(x).0;
        }
        self.buckets[b as usize].insert(i, x);
        self.len += 1;
        true
    }

    /// Splits a bucket, moving the keys from position `at` on to a new
    /// bucket, which is returned.
    fn split(&mut self, b: u32, at: usize) -> u32 {
        let new = match self.free.pop() {
            Some(new) => new,
            None => {
                self.buckets.push(Bucket::new());
                (self.buckets.len() - 1) as u32
            }
        };
        let (bucket, new_bucket) = self.pair(b, new);
        // The anchor is the shortest prefix of the first key of the new
        // bucket that is larger than the last key of the old one
        let (last, first) = (bucket.key(at - 1), bucket.key(at));
        let anchor = &first[..lcp(last, first) + 1];
        let inserted = self.trie.insert_with(anchor, new);
        debug_assert!(inserted);
        bucket.move_to(at, new_bucket);
        let next = bucket.next;
        new_bucket.prev = b;
        new_bucket.next = next;
        bucket.next = new;
        if next == NONE {
            self.tail = new;
        } else {
            self.buckets[next as usize].prev = new;
        }
        new
    }

    /// Returns mutable references to two distinct buckets.
    ///
    /// The references are not tied to `self`, as we need to access the trie
    /// while they are alive.
    fn pair<'a>(&mut self, a: u32, b: u32) -> (&'a mut Bucket, &'a mut Bucket) {
        let (a, b) = (a as usize, b as usize);
        assert!(a != b && a < self.buckets.len() && b < self.buckets.len());
        let ptr = self.buckets.as_mut_ptr();
        // SAFETY: the indices are distinct and in bounds; callers do not
        // add buckets, or access the vector of buckets in any other way,
        // while they use the references
        unsafe { (&mut *ptr.add(a), &mut *ptr.add(b)) }
    }

    /// Removes a key from the set, returning true if the key was in the set.
    pub fn remove(&mut self, x: impl AsRef<[u8]>) -> bool {
        let x = x.as_ref();
        let b = self.locate(x, TAG_LINES);
        let Some(i) = self.buckets[b as usize].find(x) else {
            return false;
        };
        self.buckets[b as usize].remove(i);
        self.len -= 1;
        // We merge the bucket with the previous one, or the next one with
        // the bucket, if they have few keys
        let (prev, next) = {
            let bucket = &self.buckets[b as usize];
            (bucket.prev, bucket.next)
        };
        let len = |b: u32| self.buckets[b as usize].len();
        // We look at the adjacent buckets only when this one is small
        // enough, as accessing them is expensive
        if len(b) > MERGE / 2 {
            return true;
        }
        // Only the first bucket can be empty
        if prev != NONE && (len(b) == 0 || len(prev) + len(b) <= MERGE) {
            self.merge(prev, b);
        } else if next != NONE && len(b) + len(next) <= MERGE {
            self.merge(b, next);
        }
        true
    }

    /// Moves the keys of the bucket `b`, which must follow `a`, to `a`, and
    /// removes `b`.
    fn merge(&mut self, a: u32, b: u32) {
        // The anchor of b is the smallest key of the trie larger than the
        // anchor of a, which is the largest key not larger than any key of
        // a, or than the first key of b; at least one of the buckets is
        // nonempty, unless a is the first bucket.
        let (bucket_a, bucket_b) = self.pair(a, b);
        let probe = if bucket_b.len() > 0 {
            bucket_b.key(0).to_vec()
        } else {
            // b is empty: its anchor is the successor of the anchor of a
            let anchor_a = if bucket_a.len() > 0 {
                self.trie.pred_entry(bucket_a.key(0)).unwrap().0.to_vec()
            } else {
                Vec::new()
            };
            self.trie.next_entry(&anchor_a).unwrap().0.to_vec()
        };
        let anchor = self.trie.pred_entry(&probe).unwrap().0.to_vec();
        debug_assert_eq!(self.trie.pred_entry(&probe).unwrap().1, b);
        let removed = self.trie.remove(&anchor);
        debug_assert!(removed);
        bucket_b.move_to(0, bucket_a);
        let next = bucket_b.next;
        bucket_a.next = next;
        *bucket_b = Bucket::new();
        if next == NONE {
            self.tail = a;
        } else {
            self.buckets[next as usize].prev = a;
        }
        self.free.push(b);
    }
}

impl<H: HashPrefixes> fmt::Debug for ZFastSet<H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<H: HashPrefixes, B: AsRef<[u8]>> Extend<B> for ZFastSet<H> {
    fn extend<I: IntoIterator<Item = B>>(&mut self, iter: I) {
        for key in iter {
            self.insert(key);
        }
    }
}

impl<H: HashPrefixes, B: AsRef<[u8]>> FromIterator<B> for ZFastSet<H> {
    fn from_iter<I: IntoIterator<Item = B>>(iter: I) -> Self {
        let mut set = Self::new();
        set.extend(iter);
        set
    }
}

impl<'a, H: HashPrefixes> IntoIterator for &'a ZFastSet<H> {
    type Item = &'a [u8];
    type IntoIter = Iter<'a, H>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator on the keys of a [`ZFastSet`] in lexicographic order.
pub struct Iter<'a, H> {
    set: &'a ZFastSet<H>,
    /// The bucket and the position of the next key.
    front: (u32, usize),
    /// The bucket and the position after the next key from the back.
    back: (u32, usize),
    remaining: usize,
}

impl<'a, H: HashPrefixes> Iterator for Iter<'a, H> {
    type Item = &'a [u8];

    #[inline]
    fn next(&mut self) -> Option<&'a [u8]> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let (mut b, mut i) = self.front;
        while i >= self.set.buckets[b as usize].len() {
            b = self.set.buckets[b as usize].next;
            i = 0;
        }
        self.front = (b, i + 1);
        Some(self.set.buckets[b as usize].key(i))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<H: HashPrefixes> DoubleEndedIterator for Iter<'_, H> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let (mut b, mut i) = self.back;
        while i == 0 {
            b = self.set.buckets[b as usize].prev;
            i = self.set.buckets[b as usize].len();
        }
        self.back = (b, i - 1);
        Some(self.set.buckets[b as usize].key(i - 1))
    }
}

impl<H: HashPrefixes> ExactSizeIterator for Iter<'_, H> {}

impl<H: HashPrefixes> std::iter::FusedIterator for Iter<'_, H> {}
