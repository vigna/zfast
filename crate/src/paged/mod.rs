/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! A dynamic ordered set of byte strings stored in pages indexed by a z-fast
//! trie.

mod page;
mod router;
mod segment;
mod slab;

use crate::utils::{HashPrefixes, PrefixHasher, Xxh3, lcp};
use page::{NONE, Page, ROOM, blob_size};
use router::{Anchors, Router};
use segment::{MAX_PAGES, Segment, home_tag};
use slab::Slab;
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

impl PartialEq for PagedKey<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.bytes().eq(other.bytes())
    }
}

impl Eq for PagedKey<'_> {}

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

/// A page using fewer bytes than this threshold is merged with a neighbor,
/// if the resulting page does not use more bytes than the second threshold.
const MERGE_BYTES: usize = ROOM / 4;
const MERGED_BYTES: usize = ROOM * 2 / 3;
/// A segment with a single page is merged with the previous one if the
/// result has at most this number of keys.
const MERGED_KEYS: usize = segment::MAX_KEYS / 2;

impl Anchors for Slab<Segment> {
    #[inline(always)]
    fn low(&self, id: u32) -> &[u8] {
        self.get(id).low()
    }
}

/// A dynamic ordered set of byte strings, stored in pages that are grouped
/// in segments indexed by a z-fast trie.
///
/// The keys are partitioned into consecutive intervals, each stored in a
/// *page* of four kilobytes: the keys of a page are stored in the page,
/// without the prefix they share. Consecutive pages (up to 128) form a
/// *segment*, and the smallest strings of the intervals of the segments,
/// which are short separators, are indexed by a z-fast trie, the *router*.
///
/// On a large set the cost of an operation is essentially the number of
/// cache misses it causes, one after the other, and the structure is
/// designed so that this number is small and does not depend on the size of
/// the set:
///
/// - The router has one leaf every few thousand keys, so it fits the cache
///   even for sets with billions of keys, whatever their length. It performs
///   a fat binary search using just signatures, without ever comparing the
///   string with the extent of a node. The search provides at most two
///   candidate segments, and the extremes of the interval of each segment,
///   which are stored in the segment, tell us which is the right one, if
///   any. If none is, which happens with negligible probability, an exact
///   search is performed. Thus, as in all structures of this crate, hashing
///   influences only the running time.
///
/// - A segment contains a hash table mapping its keys to their position, so
///   a membership query accesses the segment, and then the key, which is
///   stored in a page with no further indirection.
///
/// - A segment contains also a *blind directory* of its pages, in the style
///   of the nodes of a string B-tree, taking just four bytes per page: the
///   page containing a string is usually found by comparing the string with
///   the smallest string of a single page. Thus, order queries access the
///   segment, usually one page, and a key.
///
/// - In all cases, the cache lines that will be accessed in a segment or in
///   a page are known as soon as the segment or the page is known, so they
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
    pages: Slab<Page>,
    segs: Slab<Segment>,
    router: Router<H>,
    /// A page used when splitting and merging.
    scratch: Box<Page>,
    /// The last segment.
    last: u32,
    len: usize,
    sig_bits: u32,
}

/// The first segment; it is never deleted.
const FIRST: u32 = 0;

/// The position of a key: a segment, the rank of a page in the segment, and
/// the rank of the key in the page.
type Pos = (u32, usize, usize);

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
        let mut pages = Slab::new();
        let mut segs = Slab::new();
        let page = pages.alloc();
        let page_ref: &mut Page = pages.get_mut(page);
        page_ref.reset(b"", None);
        let first = segs.alloc();
        debug_assert_eq!(first, FIRST);
        let seg: &mut Segment = segs.get_mut(first);
        seg.reset(b"", None);
        seg.prev = NONE;
        seg.next = NONE;
        seg.insert_page(0, 0, page);
        Self {
            pages,
            segs,
            router: Router::new(sig_bits),
            scratch: Page::new_boxed(),
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

    /// Returns the number of segments.
    pub fn num_segments(&self) -> usize {
        self.segs.len()
    }

    /// Returns an estimate of the number of bytes used by the structure,
    /// including the keys.
    pub fn mem_size(&self) -> usize {
        let mut overflow = 0;
        let mut id = FIRST;
        while id != NONE {
            let seg = self.segs.get(id);
            overflow += seg.overflow_bytes();
            for pos in 0..seg.npages() {
                overflow += self.pages.get(seg.page(pos)).overflow_bytes();
            }
            id = seg.next;
        }
        size_of::<Self>()
            + size_of::<Page>()
            + self.pages.mem_size()
            + self.segs.mem_size()
            + overflow
            + self.router.mem_size()
    }

    /// Returns the number of bytes used by the router.
    pub fn router_mem_size(&self) -> usize {
        self.router.mem_size()
    }

    /// Returns the segment whose interval contains `x`, after calling
    /// `prefetch` on the candidates proposed by the router.
    #[inline(always)]
    fn segment(&self, x: &[u8], hasher: &H::Hasher<'_>, prefetch: impl Fn(&Segment)) -> u32 {
        let candidates = self.router.candidates(x, hasher);
        let candidates = candidates.as_slice();
        for &id in candidates {
            prefetch(self.segs.get(id));
        }
        for &id in candidates {
            if self.segs.get(id).covers(x) {
                return id;
            }
        }
        self.segment_slow(x)
    }

    #[cold]
    #[inline(never)]
    fn segment_slow(&self, x: &[u8]) -> u32 {
        let id = self.router.locate(x, &self.segs);
        debug_assert!(self.segs.get(id).covers(x));
        id
    }

    /// Returns the number of candidate segments that the router proposes for
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
                .position(|&id| self.segs.get(id).covers(x)),
        )
    }

    /// Returns the first candidate segment that the router proposes for `x`,
    /// without accessing any segment (for profiling).
    #[doc(hidden)]
    #[inline]
    pub fn route(&self, x: impl AsRef<[u8]>) -> u32 {
        let x = x.as_ref();
        self.router.candidates(x, &self.router.hasher(x)).as_slice()[0]
    }

    /// Returns the segment containing `x`, and whether the first page
    /// examined by an order query is the right one (for profiling).
    #[doc(hidden)]
    #[inline]
    pub fn page_stats(&self, x: impl AsRef<[u8]>) -> (u32, bool) {
        let x = x.as_ref();
        let id = self.segment(x, &self.router.hasher(x), |s| s.prefetch_ordered());
        let seg = self.segs.get(id);
        let from = seg.candidate(x);
        (id, seg.locate(x, from, &self.pages) == from)
    }

    /// Returns the slot of the page of a segment containing `x`, and the
    /// offset of the blob of `x`, if `x` is in the segment.
    #[inline(always)]
    fn probe(&self, seg: &Segment, x: &[u8], home: u16, tag: u32) -> Option<(usize, u16)> {
        let mut found = None;
        seg.probe(home, tag, |slot, off| {
            let page = self.pages.get(seg.page_in(slot));
            page.prefetch_blob(off);
            if page.is(off, x) {
                found = Some((slot, off));
            }
            found.is_some()
        });
        found
    }

    /// Returns the segment whose interval contains `x` and, if `x` is in the
    /// set, the slot of the page containing it and the offset of its blob,
    /// after calling `prefetch` on the candidates proposed by the router.
    #[inline(always)]
    fn find(
        &self,
        x: &[u8],
        hasher: &H::Hasher<'_>,
        home: u16,
        tag: u32,
        prefetch: impl Fn(&Segment),
    ) -> (u32, Option<(usize, u16)>) {
        let candidates = self.router.candidates(x, hasher);
        let candidates = candidates.as_slice();
        for &id in candidates {
            prefetch(self.segs.get(id));
        }
        for &id in candidates {
            let seg = self.segs.get(id);
            if seg.covers(x) {
                return (id, self.probe(seg, x, home, tag));
            }
        }
        let id = self.segment_slow(x);
        (id, self.probe(self.segs.get(id), x, home, tag))
    }

    /// Returns true if the set contains `x`.
    #[inline]
    pub fn contains(&self, x: impl AsRef<[u8]>) -> bool {
        let x = x.as_ref();
        let hasher = self.router.hasher(x);
        let (home, tag) = home_tag(hasher.hash(x.len()));
        self.find(x, &hasher, home, tag, |s| s.prefetch_point(home))
            .1
            .is_some()
    }

    /// Returns `x` as a key of the set, if it is one, and otherwise the
    /// position of the first key that is larger than `x` in the page
    /// containing `x` (possibly just after the last key of the page).
    ///
    /// Membership is tested first, as it is faster than finding a position.
    #[inline(always)]
    fn key_or_pos(&self, x: &[u8]) -> Result<PagedKey<'_>, Pos> {
        let hasher = self.router.hasher(x);
        let (home, tag) = home_tag(hasher.hash(x.len()));
        let (id, found) = self.find(x, &hasher, home, tag, |s| {
            s.prefetch_ordered();
            s.prefetch_point(home);
        });
        let seg = self.segs.get(id);
        if let Some((slot, off)) = found {
            return Ok(self.pages.get(seg.page_in(slot)).key_at(off));
        }
        let pos = self.page_of(seg, x);
        let page = self.pages.get(seg.page(pos));
        let (key, _) = page.lower_bound(&x[page.plen()..]);
        Err((id, pos, key))
    }

    /// Returns the position of the first key at or after the given position,
    /// which can be just after the last key of a page.
    #[inline(always)]
    fn forward(&self, (mut id, mut page, mut key): Pos) -> Option<Pos> {
        loop {
            let seg = self.segs.get(id);
            if key < self.pages.get(seg.page(page)).len() {
                return Some((id, page, key));
            }
            key = 0;
            page += 1;
            if page == seg.npages() {
                id = seg.next;
                if id == NONE {
                    return None;
                }
                page = 0;
            }
        }
    }

    /// Returns the position of the last key before the given position.
    #[inline(always)]
    fn backward(&self, (mut id, mut page, mut key): Pos) -> Option<Pos> {
        loop {
            if key > 0 {
                return Some((id, page, key - 1));
            }
            if page > 0 {
                page -= 1;
            } else {
                id = self.segs.get(id).prev;
                if id == NONE {
                    return None;
                }
                page = self.segs.get(id).npages() - 1;
            }
            key = self.pages.get(self.segs.get(id).page(page)).len();
        }
    }

    /// Returns the position just after the last key.
    fn end(&self) -> Pos {
        let seg = self.segs.get(self.last);
        let page = seg.npages() - 1;
        (self.last, page, self.pages.get(seg.page(page)).len())
    }

    #[inline(always)]
    fn key_at(&self, pos: Option<Pos>) -> Option<PagedKey<'_>> {
        pos.map(|(id, page, key)| self.pages.get(self.segs.get(id).page(page)).key(key))
    }

    /// Returns the rank in a segment of the page containing `x`, which must
    /// belong to the interval of the segment.
    #[inline(always)]
    fn page_of(&self, seg: &Segment, x: &[u8]) -> usize {
        let from = seg.candidate(x);
        self.pages.get(seg.page(from)).prefetch_slots(true);
        // When the candidate is not the right page, usually that is
        // because x diverges from the separator of the candidate in the
        // part that the directory does not know, and it is smaller
        if from > 0 {
            self.pages.get(seg.page(from - 1)).prefetch_slots(false);
        }
        seg.locate(x, from, &self.pages)
    }

    /// Returns the position of the first key that is not smaller than `x`
    /// in the page containing `x` (possibly just after the last key of the
    /// page), and whether `x` is in the set.
    #[inline(always)]
    fn search(&self, x: &[u8]) -> (Pos, bool) {
        let id = self.segment(x, &self.router.hasher(x), |s| s.prefetch_ordered());
        let seg = self.segs.get(id);
        let pos = self.page_of(seg, x);
        let page = self.pages.get(seg.page(pos));
        let (key, found) = page.lower_bound(&x[page.plen()..]);
        ((id, pos, key), found)
    }

    /// Returns the largest key that is less than or equal to `x`.
    #[inline]
    pub fn pred(&self, x: impl AsRef<[u8]>) -> Option<PagedKey<'_>> {
        match self.key_or_pos(x.as_ref()) {
            Ok(key) => Some(key),
            Err(pos) => self.key_at(self.backward(pos)),
        }
    }

    /// Returns the largest key that is less than `x`.
    #[inline]
    pub fn pred_strict(&self, x: impl AsRef<[u8]>) -> Option<PagedKey<'_>> {
        let (pos, _) = self.search(x.as_ref());
        self.key_at(self.backward(pos))
    }

    /// Returns the smallest key that is greater than or equal to `x`.
    #[inline]
    pub fn succ(&self, x: impl AsRef<[u8]>) -> Option<PagedKey<'_>> {
        match self.key_or_pos(x.as_ref()) {
            Ok(key) => Some(key),
            Err(pos) => self.key_at(self.forward(pos)),
        }
    }

    /// Returns the smallest key that is greater than `x`.
    #[inline]
    pub fn succ_strict(&self, x: impl AsRef<[u8]>) -> Option<PagedKey<'_>> {
        let ((id, page, key), found) = self.search(x.as_ref());
        self.key_at(self.forward((id, page, key + found as usize)))
    }

    /// Returns the smallest key.
    pub fn first(&self) -> Option<PagedKey<'_>> {
        self.key_at(self.forward((FIRST, 0, 0)))
    }

    /// Returns the largest key.
    pub fn last(&self) -> Option<PagedKey<'_>> {
        self.key_at(self.backward(self.end()))
    }

    /// Returns an iterator on the keys, in order.
    pub fn iter(&self) -> Iter<'_, H> {
        Iter {
            set: self,
            front: (FIRST, 0, 0),
            page: None,
            back: self.end(),
            remaining: self.len,
        }
    }

    /// Returns an iterator on the keys that are greater than or equal to
    /// `x`, in order.
    pub fn iter_from(&self, x: impl AsRef<[u8]>) -> impl Iterator<Item = PagedKey<'_>> + '_ {
        let (pos, _) = self.search(x.as_ref());
        let mut next = self.forward(pos);
        std::iter::from_fn(move || {
            let (id, page, key) = next?;
            next = self.forward((id, page, key + 1));
            self.key_at(Some((id, page, key)))
        })
    }

    /// Adds a key; returns true if the key was not in the set.
    pub fn insert(&mut self, x: impl AsRef<[u8]>) -> bool {
        let x = x.as_ref();
        let hasher = self.router.hasher(x);
        let (home, tag) = home_tag(hasher.hash(x.len()));
        let mut id = self.segment(x, &hasher, |s| s.prefetch_ordered());
        loop {
            let seg = self.segs.get(id);
            if !seg.has_room() {
                if seg.is_dirty() {
                    self.rebuild_table(id);
                } else {
                    let new = self.split_segment(id);
                    if x >= self.segs.get(new).low() {
                        id = new;
                    }
                }
                continue;
            }
            let pos = self.page_of(seg, x);
            let slot = seg.slot(pos);
            let page = self.pages.get_mut(seg.page_in(slot));
            let suffix = &x[page.plen()..];
            let (key, found) = page.lower_bound(suffix);
            if found {
                return false;
            }
            if let Some(off) = page.insert(key, suffix, home) {
                self.segs.get_mut(id).add(home, tag, slot, off);
                self.len += 1;
                return true;
            }
            if page.is_dirty() && page.fits_compacted(suffix.len()) {
                self.compact_page(id, slot);
            } else if seg.npages() == MAX_PAGES {
                let new = self.split_segment(id);
                if x >= self.segs.get(new).low() {
                    id = new;
                }
            } else {
                self.split_page(id, pos);
            }
        }
    }

    /// Returns the indices of the entries of the keys of a page in the
    /// table of its segment, in order.
    fn entries(&self, id: u32, slot: usize) -> Vec<usize> {
        let seg = self.segs.get(id);
        let page = self.pages.get(seg.page_in(slot));
        (0..page.len())
            .map(|i| seg.entry(page.hash(i), slot, page.off(i)))
            .collect()
    }

    /// Compacts a page, updating the table of its segment.
    fn compact_page(&mut self, id: u32, slot: usize) {
        let entries = self.entries(id, slot);
        let seg = self.segs.get_mut(id);
        let mut entries = entries.into_iter();
        self.pages
            .get_mut(seg.page_in(slot))
            .compact(|_, _, off| seg.relocate(entries.next().unwrap(), slot, off));
    }

    /// Returns the separator of the page following the page of given rank
    /// of a segment, if any.
    fn high_of(&self, id: u32, pos: usize) -> Option<Vec<u8>> {
        let seg = self.segs.get(id);
        if pos + 1 < seg.npages() {
            Some(self.pages.get(seg.page(pos + 1)).low().to_vec())
        } else {
            seg.high().map(<[u8]>::to_vec)
        }
    }

    /// Splits the page of given rank of a segment, which must contain at
    /// least two keys, moving its largest keys to a new page of the same
    /// segment, which must have fewer than [`MAX_PAGES`] pages.
    fn split_page(&mut self, id: u32, pos: usize) {
        let high = self.high_of(id, pos);
        let seg = self.segs.get(id);
        let slot = seg.slot(pos);
        let new_slot = seg.free_slot();
        let page = seg.page_in(slot);
        let entries = self.entries(id, slot);
        let new_page = self.pages.alloc();
        std::mem::swap(self.pages.get_mut(page), &mut *self.scratch);
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

        // The new separator is the shortest string separating the two pages
        let old_plen = old.plen();
        let (a, b) = (old.suffix(at - 1), old.suffix(at));
        let mut sep = old.prefix().to_vec();
        sep.extend_from_slice(&b[..lcp(a, b) + 1]);

        let seg = self.segs.get_mut(id);
        let (left, right) = self.pages.pair_mut(page, new_page);
        left.reset(old.low(), Some(&sep));
        let delta = left.plen() - old_plen;
        for (i, &entry) in entries.iter().enumerate().take(at) {
            let off = left.push(&old.suffix(i)[delta..], old.hash(i), old.is_out(i));
            seg.relocate(entry, slot, off);
        }
        right.reset(&sep, high.as_deref());
        let delta = right.plen() - old_plen;
        for (i, &entry) in entries.iter().enumerate().skip(at) {
            let off = right.push(&old.suffix(i)[delta..], old.hash(i), old.is_out(i));
            seg.relocate(entry, new_slot, off);
        }
        seg.insert_page(pos + 1, new_slot, new_page);
        seg.set_blind(pos + 1, old.low(), &sep);
        if let Some(high) = high.as_deref().filter(|_| pos + 2 < seg.npages()) {
            seg.set_blind(pos + 2, &sep, high);
        }
        self.scratch.clear();
    }

    /// Returns the home position, the bits of hash, the slot and the offset
    /// of the keys of the pages of a segment whose rank is in the given
    /// interval.
    fn keys_of(&self, id: u32, ranks: std::ops::Range<usize>) -> Vec<(u16, u32, usize, u16)> {
        let seg = self.segs.get(id);
        let mut keys = Vec::new();
        for pos in ranks {
            let slot = seg.slot(pos);
            let page = self.pages.get(seg.page_in(slot));
            for i in 0..page.len() {
                let (home, off) = (page.hash(i), page.off(i));
                keys.push((home, seg.tag(seg.entry(home, slot, off)), slot, off));
            }
        }
        keys
    }

    /// Rebuilds the table of a segment, getting rid of deleted entries.
    fn rebuild_table(&mut self, id: u32) {
        let keys = self.keys_of(id, 0..self.segs.get(id).npages());
        let seg = self.segs.get_mut(id);
        seg.clear_table();
        for (home, tag, slot, off) in keys {
            seg.add(home, tag, slot, off);
        }
    }

    /// Splits a segment with at least two pages, moving its last pages to a
    /// new segment, which is returned.
    fn split_segment(&mut self, id: u32) -> u32 {
        let n = self.segs.get(id).npages();
        debug_assert!(n >= 2);
        let at = n / 2;
        let keep = self.keys_of(id, 0..at);
        let moved = self.keys_of(id, at..n);
        let new = self.segs.alloc();
        let (old, right) = self.segs.pair_mut(id, new);
        let sep = self.pages.get(old.page(at)).low();
        right.reset(sep, old.high());
        // The slot of a moved page in the new segment is its rank
        let mut new_slot = [0; MAX_PAGES];
        for pos in at..n {
            new_slot[old.slot(pos)] = pos - at;
            right.insert_page(pos - at, pos - at, old.page(pos));
            if pos > at {
                right.copy_blind(pos - at, old, pos);
            }
        }
        for (home, tag, slot, off) in moved {
            right.add(home, tag, new_slot[slot], off);
        }
        right.rebuild_tree();
        for pos in (at..n).rev() {
            old.remove_page(pos);
        }
        old.clear_table();
        for (home, tag, slot, off) in keep {
            old.add(home, tag, slot, off);
        }
        old.set_high(Some(sep));
        let next = old.next;
        right.prev = id;
        right.next = next;
        old.next = new;
        if next == NONE {
            self.last = new;
        } else {
            self.segs.get_mut(next).prev = new;
        }
        self.router.insert(new, id, next, &self.segs);
        new
    }

    /// Removes a key; returns true if the key was in the set.
    pub fn remove(&mut self, x: impl AsRef<[u8]>) -> bool {
        let x = x.as_ref();
        let hasher = self.router.hasher(x);
        let (home, tag) = home_tag(hasher.hash(x.len()));
        let (id, found) = self.find(x, &hasher, home, tag, |s| s.prefetch_point(home));
        let seg = self.segs.get(id);
        let Some((slot, off)) = found else {
            return false;
        };
        let page = self.pages.get_mut(seg.page_in(slot));
        page.remove(page.rank(off, &x[page.plen()..]));
        let small = page.used() < MERGE_BYTES;
        self.segs.get_mut(id).remove(home, slot, off);
        self.len -= 1;
        if small {
            self.shrink(id, slot);
        }
        true
    }

    /// Tries to merge a page that has become small with a neighbor in the
    /// same segment, or its segment with the previous one.
    fn shrink(&mut self, id: u32, slot: usize) {
        let seg = self.segs.get(id);
        let n = seg.npages();
        let pos = seg.rank_of(slot);
        if (pos > 0 && self.merge_pages(id, pos - 1)) || (pos + 1 < n && self.merge_pages(id, pos))
        {
            return;
        }
        if n == 1 && id != FIRST {
            self.merge_segment(id);
        }
    }

    /// Moves the keys of the page of rank `pos` + 1 of a segment to the page
    /// of rank `pos`, and deletes the former, if the result is not too
    /// large; returns true if the pages were merged.
    fn merge_pages(&mut self, id: u32, pos: usize) -> bool {
        let high = self.high_of(id, pos + 1);
        let seg = self.segs.get(id);
        let (a_slot, b_slot) = (seg.slot(pos), seg.slot(pos + 1));
        let (a, b) = (seg.page_in(a_slot), seg.page_in(b_slot));
        let (left, right) = (self.pages.get(a), self.pages.get(b));
        let low = left.low();
        let plen = high.as_ref().map_or(0, |high| lcp(low, high));
        let bytes = blob_size(low.len()) + left.key_bytes_with(plen) + right.key_bytes_with(plen);
        if bytes > MERGED_BYTES {
            return false;
        }

        let a_entries = self.entries(id, a_slot);
        let b_entries = self.entries(id, b_slot);
        std::mem::swap(self.pages.get_mut(a), &mut *self.scratch);
        let old = &*self.scratch;
        let seg = self.segs.get_mut(id);
        let (left, right) = self.pages.pair_mut(a, b);
        left.reset(old.low(), high.as_deref());
        debug_assert_eq!(left.plen(), plen);
        let mut suffix = Vec::new();
        for (page, entries) in [(old, a_entries), (&*right, b_entries)] {
            // The part of the prefix of the page that is not shared any
            // longer
            let lost = &page.prefix()[plen..];
            for (i, entry) in entries.into_iter().enumerate() {
                suffix.clear();
                suffix.extend_from_slice(lost);
                suffix.extend_from_slice(page.suffix(i));
                let off = left.push(&suffix, page.hash(i), page.is_out(i));
                seg.relocate(entry, a_slot, off);
            }
        }
        seg.remove_page(pos + 1);
        if let Some(high) = high.as_deref().filter(|_| pos + 1 < seg.npages()) {
            seg.set_blind(pos + 1, old.low(), high);
        }
        self.scratch.clear();
        self.pages.release(b);
        true
    }

    /// Moves the only page of a segment to the previous segment, and deletes
    /// the segment, if the previous segment has room.
    fn merge_segment(&mut self, id: u32) {
        let seg = self.segs.get(id);
        let prev = seg.prev;
        let before = self.segs.get(prev);
        if before.npages() == MAX_PAGES || before.keys() + seg.keys() > MERGED_KEYS {
            return;
        }
        if before.is_dirty() || !before.has_room() {
            return;
        }
        let keys = self.keys_of(id, 0..1);
        let next = seg.next;
        self.router.remove(id, next, &self.segs);
        let (before, seg) = self.segs.pair_mut(prev, id);
        let page = seg.page(0);
        let n = before.npages();
        let slot = before.free_slot();
        let last = self.pages.get(before.page(n - 1)).low();
        before.insert_page(n, slot, page);
        before.set_blind(n, last, self.pages.get(page).low());
        for (home, tag, _, off) in keys {
            before.add(home, tag, slot, off);
        }
        before.set_high(seg.high());
        before.next = next;
        if next == NONE {
            self.last = prev;
        } else {
            self.segs.get_mut(next).prev = prev;
        }
        self.segs.release(id);
    }

    /// Checks the invariants of the structure.
    ///
    /// # Panics
    ///
    /// If an invariant is violated.
    pub fn check(&self) {
        let mut order = Vec::new();
        let mut len = 0;
        let mut pages = 0;
        let mut id = FIRST;
        let mut prev = NONE;
        let mut last_key: Option<Vec<u8>> = None;
        let mut low = Some(Vec::new());
        while id != NONE {
            let seg = self.segs.get(id);
            assert_eq!(seg.prev, prev, "wrong link");
            // The lower fence is the upper fence of the previous segment
            assert_eq!(Some(seg.low()), low.as_deref(), "fences do not match");
            low = seg.high().map(<[u8]>::to_vec);
            seg.check(&self.pages);
            let mut keys = 0;
            for pos in 0..seg.npages() {
                let slot = seg.slot(pos);
                let page = self.pages.get(seg.page_in(slot));
                let high = self.high_of(id, pos);
                page.check(high.as_deref());
                assert!(seg.covers(page.low()));
                for i in 0..page.len() {
                    let key = page.key(i).to_vec();
                    assert!(last_key.as_ref().is_none_or(|last| *last < key));
                    assert!(seg.covers(&key));
                    assert_eq!(seg.locate(&key, seg.candidate(&key), &self.pages), pos);
                    let (home, tag) = home_tag(self.router.hasher(&key).hash(key.len()));
                    assert_eq!(page.hash(i), home, "wrong hash");
                    let off = page.off(i);
                    assert_eq!(seg.tag(seg.entry(home, slot, off)), tag);
                    assert!(page.is(off, &key));
                    assert!(seg.probe(home, tag, |s, o| (s, o) == (slot, off)));
                    last_key = Some(key);
                }
                keys += page.len();
            }
            assert_eq!(keys, seg.keys());
            len += keys;
            pages += seg.npages();
            order.push(id);
            prev = id;
            id = seg.next;
        }
        assert_eq!(low, None, "the last segment has an upper fence");
        assert_eq!(prev, self.last);
        assert_eq!(len, self.len);
        assert_eq!(pages, self.pages.len());
        assert_eq!(order.len(), self.segs.len());
        self.router.check(&order, &self.segs);
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
    front: Pos,
    /// The page of the next key, if it has been accessed.
    page: Option<&'a Page>,
    /// The position after the next key from the back.
    back: Pos,
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
        loop {
            if let Some(page) = self.page {
                if self.front.2 < page.len() {
                    self.front.2 += 1;
                    return Some(page.key(self.front.2 - 1));
                }
                // The next page
                let seg = self.set.segs.get(self.front.0);
                self.front.2 = 0;
                self.front.1 += 1;
                if self.front.1 == seg.npages() {
                    debug_assert!(seg.next != NONE);
                    self.front = (seg.next, 0, 0);
                }
            }
            let seg = self.set.segs.get(self.front.0);
            let page = self.set.pages.get(seg.page(self.front.1));
            self.page = Some(page);
            // Pages are scattered: we fetch the following one while we
            // return the keys of this one
            if self.front.1 + 1 < seg.npages() {
                self.set
                    .pages
                    .get(seg.page(self.front.1 + 1))
                    .prefetch_all();
            }
        }
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
        self.back = self.set.backward(self.back)?;
        self.set.key_at(Some(self.back))
    }
}

impl<H: HashPrefixes> ExactSizeIterator for Iter<'_, H> {}

impl<H: HashPrefixes> std::iter::FusedIterator for Iter<'_, H> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segments() {
        // Enough keys to split segments, with long common prefixes and
        // keys that are prefixes of other keys
        let mut set = <ZFastPagedSet>::new();
        let n = if cfg!(miri) { 4000 } else { 60000 };
        let key = |i: usize| {
            let j = (i * 7919) % n;
            format!(
                "{}/{:06}{}",
                "p".repeat(j % 7 * 20),
                j / 3,
                "x".repeat(j % 3 * 9)
            )
        };
        for i in 0..n {
            assert!(set.insert(key(i)), "{}", key(i));
            if i % 7001 == 0 {
                set.check();
            }
        }
        set.check();
        assert!(set.num_segments() > 1);
        assert_eq!(set.iter().count(), n);
        for i in (0..n).step_by(7) {
            let k = key(i);
            assert!(set.contains(&k));
            assert!(!set.contains(format!("{k}y")));
            assert!(set.succ(&k).unwrap() == k.as_bytes());
            assert!(set.pred(&k).unwrap() == k.as_bytes());
            assert!(set.pred(format!("{k}\0")).unwrap() == k.as_bytes());
        }
        // Deletions in a different order, so that pages and segments are
        // merged
        for i in 0..n {
            let i = (i * 104729) % n;
            assert!(set.remove(key(i)));
            if i % 7001 == 0 {
                set.check();
            }
        }
        assert!(set.is_empty());
        assert_eq!(set.num_segments(), 1);
        set.check();
    }

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
