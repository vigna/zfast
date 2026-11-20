/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! A dynamic ordered set of byte strings stored in pages indexed by a z-fast
//! trie.

mod page;
mod router;

use crate::utils::{HashPrefixes, PrefixHasher, Xxh3, lcp};
use page::{HEAP_LEN, Hint, MAX_KEYS, NONE, Page, Pages, blob_size};
use router::Router;
use std::cmp::Ordering;
use std::fmt;

/// A key of a [`ZFastPagedSet`].
///
/// Keys are stored without the prefix they share with the other keys of the
/// same page, so a key is returned as a pair of slices whose concatenation
/// is the key.
#[derive(Clone, Copy)]
pub struct PagedKey<'a> {
    prefix: &'a [u8],
    suffix: &'a [u8],
}

impl<'a> PagedKey<'a> {
    /// Returns the length of the key.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.prefix.len() + self.suffix.len()
    }

    /// Returns true if the key is the empty string.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the two slices whose concatenation is the key.
    #[inline(always)]
    pub fn as_slices(&self) -> (&'a [u8], &'a [u8]) {
        (self.prefix, self.suffix)
    }

    /// Returns an iterator on the bytes of the key.
    pub fn bytes(&self) -> impl Iterator<Item = u8> + 'a {
        self.prefix.iter().chain(self.suffix).copied()
    }

    /// Returns a copy of the key.
    pub fn to_vec(&self) -> Vec<u8> {
        let mut key = Vec::with_capacity(self.len());
        self.append_to(&mut key);
        key
    }

    /// Appends the key to a vector.
    pub fn append_to(&self, key: &mut Vec<u8>) {
        key.extend_from_slice(self.prefix);
        key.extend_from_slice(self.suffix);
    }

    /// Compares the key with a string.
    pub fn cmp_bytes(&self, other: &[u8]) -> Ordering {
        let n = self.prefix.len().min(other.len());
        self.prefix[..n]
            .cmp(&other[..n])
            .then_with(|| match self.prefix.len().cmp(&other.len()) {
                // The prefix is longer than the string
                Ordering::Greater => Ordering::Greater,
                _ => self.suffix.cmp(&other[n..]),
            })
    }
}

impl PartialEq<[u8]> for PagedKey<'_> {
    fn eq(&self, other: &[u8]) -> bool {
        self.cmp_bytes(other) == Ordering::Equal
    }
}

impl PartialEq<&[u8]> for PagedKey<'_> {
    fn eq(&self, other: &&[u8]) -> bool {
        self.cmp_bytes(other) == Ordering::Equal
    }
}

impl PartialEq<Vec<u8>> for PagedKey<'_> {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.cmp_bytes(other) == Ordering::Equal
    }
}

impl fmt::Debug for PagedKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.bytes()).finish()
    }
}

/// A page with fewer keys and fewer bytes than these thresholds is merged
/// with a neighbor, if possible.
const MERGE_KEYS: usize = MAX_KEYS / 4;
const MERGE_BYTES: usize = HEAP_LEN / 4;
/// A merge is performed only if the resulting page does not exceed these
/// thresholds.
const MERGED_KEYS: usize = MAX_KEYS * 2 / 3;
const MERGED_BYTES: usize = HEAP_LEN * 2 / 3;

/// A dynamic ordered set of byte strings, stored in pages that are indexed
/// by a z-fast trie.
///
/// The keys are partitioned into consecutive intervals, each stored in a
/// *page* of four kilobytes: the keys of a page are stored in the page,
/// without the prefix they share, together with the extremes of the
/// interval (the *fences*). The lower fences of the pages are short
/// separators, and they are indexed by a z-fast trie, the *router*.
///
/// This organization is that of a B-tree whose internal nodes have been
/// replaced by a z-fast trie, and it shares with [`ZFastSet`](crate::ZFastSet)
/// the idea of indexing with a trie one string every few dozen keys. There
/// are however three differences with respect to [`ZFastSet`](crate::ZFastSet)
/// that make this structure much faster on large sets, where the cost of an
/// operation is essentially the number of cache misses it causes, one after
/// the other:
///
/// - The router performs a fat binary search using just signatures, without
///   ever comparing the string with the extent of a node, and thus accessing
///   only a hash table and one node, which fit the cache even for very large
///   sets (there is one node every [`MAX_KEYS`] keys at most, whatever the
///   length of the keys). The search provides at most two candidate pages,
///   and the fences tell us which is the right one, if any. If none is,
///   which happens with negligible probability, an exact search is performed.
///   Thus, as in all structures of this crate, hashing influences only
///   the running time.
///
/// - Keys are stored in the page, so there is no pointer to follow.
///
/// - Each page contains a small hash table, used by membership queries,
///   and sorted arrays, used by order queries. In both cases, the cache lines
///   that will be accessed are known as soon as the page is known, so they
///   can be fetched at the same time.
///
/// Since keys are stored without a prefix, methods return keys as instances
/// of [`PagedKey`].
///
/// # Examples
///
/// ```
/// # use zfast::ZFastPagedSet;
/// let mut set = <ZFastPagedSet>::new();
/// for i in 0..10000 {
///     set.insert(format!("http://example.com/{i}"));
/// }
/// assert!(set.contains("http://example.com/42"));
/// assert!(!set.contains("http://example.com/x"));
/// assert_eq!(set.succ("http://example.com/x"), None);
/// assert_eq!(
///     set.pred("http://example.com/x").unwrap().to_vec(),
///     b"http://example.com/9999"
/// );
/// assert!(set.remove("http://example.com/9999"));
/// assert_eq!(set.len(), 9999);
/// ```
pub struct ZFastPagedSet<H = Xxh3> {
    pages: Pages,
    router: Router<H>,
    /// A page used when splitting and merging.
    scratch: Box<Page>,
    /// The last page.
    last: u32,
    len: usize,
    sig_bits: u32,
}

/// The first page; it is never deleted.
const FIRST: u32 = 0;

impl<H: HashPrefixes> Default for ZFastPagedSet<H> {
    fn default() -> Self {
        Self::new()
    }
}

impl<H: HashPrefixes> ZFastPagedSet<H> {
    /// Creates an empty set.
    pub fn new() -> Self {
        Self::with_sig_bits(64)
    }

    /// Creates an empty set whose router uses signatures with the given
    /// number of bits.
    ///
    /// Short signatures make false positives likely: this method is useful
    /// only to test their handling.
    ///
    /// # Panics
    ///
    /// If `sig_bits` is greater than 64.
    pub fn with_sig_bits(sig_bits: u32) -> Self {
        let mut pages = Pages::new();
        let first = pages.alloc();
        debug_assert_eq!(first, FIRST);
        let page = pages.get_mut(first);
        page.reset(b"", None);
        page.prev = NONE;
        page.next = NONE;
        let mut scratch = Page::new_boxed();
        scratch.clear();
        Self {
            pages,
            router: Router::new(sig_bits),
            scratch,
            last: first,
            len: 0,
            sig_bits,
        }
    }

    /// Returns the number of keys.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the set contains no key.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Removes all keys.
    pub fn clear(&mut self) {
        *self = Self::with_sig_bits(self.sig_bits);
    }

    /// Returns the number of pages.
    pub fn num_pages(&self) -> usize {
        self.pages.len()
    }

    /// Returns an estimate of the number of bytes used by the structure,
    /// including the keys.
    pub fn mem_size(&self) -> usize {
        let mut overflow = 0;
        let mut id = FIRST;
        while id != NONE {
            let page = self.pages.get(id);
            overflow += page.overflow_bytes();
            id = page.next;
        }
        size_of::<Self>()
            + size_of::<Page>()
            + self.pages.mem_size()
            + overflow
            + self.router.mem_size()
    }

    /// Returns the number of bytes used by the router.
    pub fn router_mem_size(&self) -> usize {
        self.router.mem_size()
    }

    /// Returns the 16 bits of hash used inside pages, given the hash of the
    /// key.
    #[inline(always)]
    fn hash16(hash: u64) -> u16 {
        (hash >> 24) as u16
    }

    /// Returns the page whose interval contains `x`, and the 16 bits of hash
    /// of `x` used inside pages.
    #[inline(always)]
    fn locate(&self, x: &[u8], ordered: bool) -> (u32, u16) {
        let hasher = self.router.hasher(x);
        let h16 = Self::hash16(hasher.hash(x.len()));
        let hint = if ordered {
            Hint::Ordered
        } else {
            Hint::Point(h16)
        };
        let candidates = self.router.candidates(x, &hasher);
        let candidates = candidates.as_slice();
        for &id in candidates {
            self.pages.get(id).prefetch(hint);
        }
        for &id in candidates {
            if self.pages.get(id).covers(x) {
                return (id, h16);
            }
        }
        (self.locate_slow(x), h16)
    }

    #[cold]
    #[inline(never)]
    fn locate_slow(&self, x: &[u8]) -> u32 {
        let id = self.router.locate(x, &self.pages);
        debug_assert!(self.pages.get(id).covers(x));
        id
    }

    /// Returns the number of candidate pages that the router proposes for
    /// `x`, and the index of the one containing `x`, if any (otherwise, an
    /// exact search is necessary).
    #[doc(hidden)]
    pub fn locate_stats(&self, x: impl AsRef<[u8]>) -> (usize, Option<usize>) {
        let x = x.as_ref();
        let candidates = self.router.candidates(x, &self.router.hasher(x));
        let candidates = candidates.as_slice();
        (
            candidates.len(),
            candidates
                .iter()
                .position(|&id| self.pages.get(id).covers(x)),
        )
    }

    /// Returns the first candidate page that the router proposes for `x`,
    /// without accessing any page (for profiling).
    #[doc(hidden)]
    #[inline]
    pub fn route(&self, x: impl AsRef<[u8]>) -> u32 {
        let x = x.as_ref();
        self.router.candidates(x, &self.router.hasher(x)).as_slice()[0]
    }

    /// Returns the page containing `x`, without accessing the keys of the
    /// page (for profiling).
    #[doc(hidden)]
    #[inline]
    pub fn page_of(&self, x: impl AsRef<[u8]>) -> u32 {
        self.locate(x.as_ref(), false).0
    }

    /// Returns true if the set contains `x`.
    #[inline]
    pub fn contains(&self, x: impl AsRef<[u8]>) -> bool {
        let x = x.as_ref();
        let (id, h16) = self.locate(x, false);
        let page = self.pages.get(id);
        page.find(&x[page.plen()..], h16).is_some()
    }

    /// Returns the position of the first key at or after the given position,
    /// which can be just after the last key of a page.
    #[inline(always)]
    fn forward(&self, mut id: u32, mut pos: usize) -> Option<(u32, usize)> {
        loop {
            let page = self.pages.get(id);
            if pos < page.len() {
                return Some((id, pos));
            }
            id = page.next;
            if id == NONE {
                return None;
            }
            pos = 0;
        }
    }

    /// Returns the position of the last key before the given position.
    #[inline(always)]
    fn backward(&self, mut id: u32, mut pos: usize) -> Option<(u32, usize)> {
        loop {
            if pos > 0 {
                return Some((id, pos - 1));
            }
            id = self.pages.get(id).prev;
            if id == NONE {
                return None;
            }
            pos = self.pages.get(id).len();
        }
    }

    #[inline(always)]
    fn key_at(&self, pos: Option<(u32, usize)>) -> Option<PagedKey<'_>> {
        pos.map(|(id, pos)| self.pages.get(id).key(pos))
    }

    /// Returns the page containing `x`, the number of keys of the page that
    /// are smaller than `x`, and whether `x` is in the set.
    #[inline(always)]
    fn search(&self, x: &[u8]) -> (u32, usize, bool) {
        let (id, _) = self.locate(x, true);
        let page = self.pages.get(id);
        let (pos, found) = page.lower_bound(&x[page.plen()..]);
        (id, pos, found)
    }

    /// Returns the largest key that is less than or equal to `x`.
    #[inline]
    pub fn pred(&self, x: impl AsRef<[u8]>) -> Option<PagedKey<'_>> {
        let (id, pos, found) = self.search(x.as_ref());
        self.key_at(self.backward(id, pos + found as usize))
    }

    /// Returns the largest key that is less than `x`.
    #[inline]
    pub fn pred_strict(&self, x: impl AsRef<[u8]>) -> Option<PagedKey<'_>> {
        let (id, pos, _) = self.search(x.as_ref());
        self.key_at(self.backward(id, pos))
    }

    /// Returns the smallest key that is greater than or equal to `x`.
    #[inline]
    pub fn succ(&self, x: impl AsRef<[u8]>) -> Option<PagedKey<'_>> {
        let (id, pos, _) = self.search(x.as_ref());
        self.key_at(self.forward(id, pos))
    }

    /// Returns the smallest key that is greater than `x`.
    #[inline]
    pub fn succ_strict(&self, x: impl AsRef<[u8]>) -> Option<PagedKey<'_>> {
        let (id, pos, found) = self.search(x.as_ref());
        self.key_at(self.forward(id, pos + found as usize))
    }

    /// Returns the smallest key.
    pub fn first(&self) -> Option<PagedKey<'_>> {
        self.key_at(self.forward(FIRST, 0))
    }

    /// Returns the largest key.
    pub fn last(&self) -> Option<PagedKey<'_>> {
        self.key_at(self.backward(self.last, self.pages.get(self.last).len()))
    }

    /// Returns an iterator on the keys, in order.
    pub fn iter(&self) -> Iter<'_, H> {
        Iter {
            set: self,
            front: (FIRST, 0),
            back: (self.last, self.pages.get(self.last).len()),
            remaining: self.len,
        }
    }

    /// Returns an iterator on the keys that are greater than or equal to
    /// `x`, in order.
    pub fn iter_from(&self, x: impl AsRef<[u8]>) -> impl Iterator<Item = PagedKey<'_>> + '_ {
        let (id, pos, _) = self.search(x.as_ref());
        let mut next = self.forward(id, pos);
        std::iter::from_fn(move || {
            let (id, pos) = next?;
            next = self.forward(id, pos + 1);
            Some(self.pages.get(id).key(pos))
        })
    }

    /// Adds a key; returns true if the key was not in the set.
    pub fn insert(&mut self, x: impl AsRef<[u8]>) -> bool {
        let x = x.as_ref();
        let (mut id, h16) = self.locate(x, true);
        loop {
            let page = self.pages.get_mut(id);
            let suffix = &x[page.plen()..];
            let (pos, found) = page.lower_bound(suffix);
            if found {
                return false;
            }
            if page.insert(pos, suffix, h16) {
                self.len += 1;
                return true;
            }
            if page.is_dirty() && page.fits_compacted(suffix.len()) {
                page.compact();
                continue;
            }
            let new = self.split(id);
            if x >= self.pages.low(new) {
                id = new;
            }
        }
    }

    /// Splits a page, which must contain at least two keys, moving its
    /// largest keys to a new page, which is returned.
    fn split(&mut self, id: u32) -> u32 {
        let new = self.pages.alloc();
        std::mem::swap(self.pages.get_mut(id), &mut *self.scratch);
        let old = &*self.scratch;
        let n = old.len();
        debug_assert!(n >= 2);
        // The first page contains about half of the bytes
        let total: usize = (0..n).map(|i| old.key_size(i)).sum();
        let (mut at, mut bytes) = (0, 0);
        while at < n - 1 && bytes < total / 2 {
            bytes += old.key_size(at);
            at += 1;
        }
        let at = at.max(1);

        // The new fence is the shortest string separating the two pages
        let old_plen = old.plen();
        let (a, b) = (old.suffix(at - 1), old.suffix(at));
        let mut fence = old.prefix().to_vec();
        fence.extend_from_slice(&b[..lcp(a, b) + 1]);
        let high = old.high_suffix().map(|suffix| {
            let mut high = old.prefix().to_vec();
            high.extend_from_slice(suffix);
            high
        });

        let (left, right) = self.pages.pair_mut(id, new);
        left.reset(old.low(), Some(&fence));
        let delta = left.plen() - old_plen;
        for i in 0..at {
            left.push(&old.suffix(i)[delta..], old.hash(i), old.is_out(i));
        }
        right.reset(&fence, high.as_deref());
        let delta = right.plen() - old_plen;
        for i in at..n {
            right.push(&old.suffix(i)[delta..], old.hash(i), old.is_out(i));
        }
        let next = old.next;
        left.prev = old.prev;
        left.next = new;
        right.prev = id;
        right.next = next;
        if next == NONE {
            self.last = new;
        } else {
            self.pages.get_mut(next).prev = new;
        }
        self.scratch.clear();
        self.router.insert(new, id, next, &self.pages);
        new
    }

    /// Removes a key; returns true if the key was in the set.
    pub fn remove(&mut self, x: impl AsRef<[u8]>) -> bool {
        let x = x.as_ref();
        let (id, h16) = self.locate(x, false);
        let page = self.pages.get_mut(id);
        let suffix = &x[page.plen()..];
        let Some(slot) = page.find(suffix, h16) else {
            return false;
        };
        page.remove(slot, suffix);
        self.len -= 1;
        if page.len() < MERGE_KEYS && page.used() < MERGE_BYTES {
            let (prev, next) = (page.prev, page.next);
            if !(prev != NONE && self.merge(prev, id)) && next != NONE {
                self.merge(id, next);
            }
        }
        true
    }

    /// Moves the keys of the page `b`, which must follow the page `a`, to
    /// the page `a`, and deletes `b`, if the result is not too large;
    /// returns true if the pages were merged.
    fn merge(&mut self, a: u32, b: u32) -> bool {
        let (left, right) = (self.pages.get(a), self.pages.get(b));
        debug_assert_eq!(left.next, b);
        let low = left.low();
        let high = right.high_suffix().map(|suffix| {
            let mut high = right.prefix().to_vec();
            high.extend_from_slice(suffix);
            high
        });
        let plen = high.as_ref().map_or(0, |high| lcp(low, high));
        let bytes = blob_size(low.len())
            + high.as_ref().map_or(0, |high| blob_size(high.len() - plen))
            + left.key_bytes_with(plen)
            + right.key_bytes_with(plen);
        if left.len() + right.len() > MERGED_KEYS || bytes > MERGED_BYTES {
            return false;
        }

        let next = right.next;
        self.router.remove(b, next, &self.pages);
        std::mem::swap(self.pages.get_mut(a), &mut *self.scratch);
        let old = &*self.scratch;
        let (left, right) = self.pages.pair_mut(a, b);
        left.reset(old.low(), high.as_deref());
        debug_assert_eq!(left.plen(), plen);
        let mut suffix = Vec::new();
        for page in [old, &*right] {
            // The part of the prefix of the page that is not shared any
            // longer
            let lost = &page.prefix()[plen..];
            for i in 0..page.len() {
                suffix.clear();
                suffix.extend_from_slice(lost);
                suffix.extend_from_slice(page.suffix(i));
                left.push(&suffix, page.hash(i), page.is_out(i));
            }
        }
        left.prev = old.prev;
        left.next = next;
        if next == NONE {
            self.last = a;
        } else {
            self.pages.get_mut(next).prev = a;
        }
        self.scratch.clear();
        self.pages.release(b);
        true
    }

    /// Checks the invariants of the structure.
    ///
    /// # Panics
    ///
    /// If an invariant is violated.
    pub fn check(&self) {
        let mut order = Vec::new();
        let mut len = 0;
        let mut id = FIRST;
        let mut prev = NONE;
        let mut high: Option<Vec<u8>> = Some(Vec::new());
        let mut last_key: Option<Vec<u8>> = None;
        while id != NONE {
            let page = self.pages.get(id);
            page.check();
            assert_eq!(page.prev, prev, "wrong link");
            // The lower fence is the upper fence of the previous page
            assert_eq!(Some(page.low()), high.as_deref(), "fences do not match");
            high = page.high_suffix().map(|suffix| {
                let mut high = page.prefix().to_vec();
                high.extend_from_slice(suffix);
                high
            });
            for i in 0..page.len() {
                let key = page.key(i).to_vec();
                assert!(last_key.as_ref().is_none_or(|last| *last < key));
                let hash = self.router.hasher(&key).hash(key.len());
                assert_eq!(page.hash(i), Self::hash16(hash), "wrong hash");
                assert!(page.covers(&key));
                last_key = Some(key);
            }
            len += page.len();
            order.push(id);
            prev = id;
            id = page.next;
        }
        assert_eq!(high, None, "the last page has an upper fence");
        assert_eq!(prev, self.last);
        assert_eq!(len, self.len);
        assert_eq!(order.len(), self.pages.len());
        self.router.check(&order, &self.pages);
    }
}

impl<H: HashPrefixes> fmt::Debug for ZFastPagedSet<H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<H: HashPrefixes, B: AsRef<[u8]>> Extend<B> for ZFastPagedSet<H> {
    fn extend<I: IntoIterator<Item = B>>(&mut self, iter: I) {
        for key in iter {
            self.insert(key);
        }
    }
}

impl<H: HashPrefixes, B: AsRef<[u8]>> FromIterator<B> for ZFastPagedSet<H> {
    fn from_iter<I: IntoIterator<Item = B>>(iter: I) -> Self {
        let mut set = Self::new();
        set.extend(iter);
        set
    }
}

impl<'a, H: HashPrefixes> IntoIterator for &'a ZFastPagedSet<H> {
    type Item = PagedKey<'a>;
    type IntoIter = Iter<'a, H>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator on the keys of a [`ZFastPagedSet`].
pub struct Iter<'a, H> {
    set: &'a ZFastPagedSet<H>,
    /// The position of the next key.
    front: (u32, usize),
    /// The position after the next key from the back.
    back: (u32, usize),
    remaining: usize,
}

impl<'a, H: HashPrefixes> Iterator for Iter<'a, H> {
    type Item = PagedKey<'a>;

    #[inline]
    fn next(&mut self) -> Option<PagedKey<'a>> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let (id, pos) = self.set.forward(self.front.0, self.front.1)?;
        self.front = (id, pos + 1);
        Some(self.set.pages.get(id).key(pos))
    }

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
        let (id, pos) = self.set.backward(self.back.0, self.back.1)?;
        self.back = (id, pos);
        Some(self.set.pages.get(id).key(pos))
    }
}

impl<H: HashPrefixes> ExactSizeIterator for Iter<'_, H> {}

impl<H: HashPrefixes> std::iter::FusedIterator for Iter<'_, H> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_merge() {
        // A small test that can be run under Miri
        let mut set = <ZFastPagedSet>::new();
        let n = if cfg!(miri) { 400 } else { 3000 };
        let key = |i: usize| format!("{:05}/{}", (i * 7919) % n, "x".repeat(i % 40));
        for i in 0..n {
            assert!(set.insert(key(i)));
        }
        set.check();
        assert!(set.num_pages() > 1);
        for i in 0..n {
            assert!(set.contains(key(i)));
            assert!(set.succ(key(i)).unwrap() == key(i).as_bytes());
        }
        // A key longer than a page
        let long = vec![b'z'; 10000];
        assert!(set.insert(&long));
        assert!(set.contains(&long));
        assert!(set.last().unwrap() == long.as_slice());
        set.check();
        assert!(set.remove(&long));
        for i in 0..n {
            assert!(set.remove(key(i)));
            if i % 97 == 0 {
                set.check();
            }
        }
        assert!(set.is_empty());
        set.check();
    }
}
