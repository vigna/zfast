/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! The segments of a [paged set](super::ZFastPagedSet).
//!
//! A segment is a sequence of at most [`MAX_PAGES`] consecutive pages, and it
//! is a leaf of the router. Its purpose is to keep the router small enough to
//! fit the cache, whatever the size of the set, while making it possible to
//! reach a key of the segment with a number of cache misses that does not
//! depend on the number of pages. To this purpose, a segment contains:
//!
//! - its *fences*, that is, the separator of its first page and that of the
//!   first page of the next segment, which tell us whether a string belongs
//!   to the segment, as the router provides just candidates;
//!
//! - a hash table mapping each key of the segment to the page containing it
//!   and to the offset of its blob in the page: a membership query accesses
//!   one line of the table and then the key;
//!
//! - a *blind directory* of the separators of its pages, in the style of the
//!   nodes of a string B-tree: for each pair of adjacent separators we store
//!   the length of their longest common prefix and the two symbols following
//!   it. These data, which take four bytes per page, are sufficient to
//!   find the page containing a string after comparing the string with a
//!   single separator, except in rare cases.

use super::page::{NONE, Page};
use super::slab::{Block, Slab};
use crate::utils::{lcp, lcp_from, prefetch};

/// The maximum number of pages in a segment.
pub(super) const MAX_PAGES: usize = 64;
/// The number of entries of the hash table (a power of two).
const TABLE: usize = 4096;
/// The maximum number of keys in a segment.
pub(super) const MAX_KEYS: usize = TABLE * 3 / 4;
/// If the number of nonempty entries reaches this value, the table must be
/// rebuilt.
const MAX_USED: usize = TABLE * 7 / 8;
const EMPTY: u32 = 0;
/// The value of a deleted entry.
const TOMB: u32 = 1;
/// The maximum length of a fence stored in the segment.
const FENCE_INLINE: usize = 60;
/// The value representing a longest common prefix that is too long to be
/// stored, and the mask extracting the length from an element of `dist`.
const BIG: usize = 0x7FFF;
/// The bit of an element of `dist` saying that the first separator of the
/// pair is a prefix of the second one.
const END: u16 = 0x8000;

/// Returns the symbol in position `i` of the string `x` followed by the
/// (virtual) terminator: zero for the terminator, and *b* + 1 for the byte
/// *b*.
#[inline(always)]
fn sym_at(x: &[u8], i: usize) -> u16 {
    debug_assert!(i <= x.len());
    if i < x.len() { x[i] as u16 + 1 } else { 0 }
}

/// A fence: a string stored in the segment, if it is short, or elsewhere.
#[repr(C)]
struct Fence {
    /// The length of the string, or `u32::MAX` if there is no string.
    len: u32,
    /// The string, or a pointer to a boxed slice.
    data: [u8; FENCE_INLINE],
}

const NO_FENCE: u32 = u32::MAX;
/// The value representing no position in the descent tree.
const NO_NODE: u8 = u8::MAX;

impl Fence {
    #[inline(always)]
    fn get(&self) -> Option<&[u8]> {
        if self.len as usize <= FENCE_INLINE {
            Some(&self.data[..self.len as usize])
        } else if self.len == NO_FENCE {
            None
        } else {
            let ptr = usize::from_le_bytes(self.data[..8].try_into().unwrap());
            // SAFETY: the pointer was obtained from a boxed slice of the
            // given length, owned by the fence
            Some(unsafe {
                std::slice::from_raw_parts(
                    std::ptr::with_exposed_provenance::<u8>(ptr),
                    self.len as usize,
                )
            })
        }
    }

    fn clear(&mut self) {
        if self.len as usize > FENCE_INLINE && self.len != NO_FENCE {
            let ptr = usize::from_le_bytes(self.data[..8].try_into().unwrap());
            // SAFETY: see get; the string is not used any longer
            unsafe {
                drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                    std::ptr::with_exposed_provenance_mut::<u8>(ptr),
                    self.len as usize,
                )));
            }
        }
        self.len = NO_FENCE;
    }

    fn set(&mut self, s: Option<&[u8]>) {
        self.clear();
        let Some(s) = s else { return };
        assert!(s.len() < NO_FENCE as usize, "key too long");
        if s.len() <= FENCE_INLINE {
            self.data[..s.len()].copy_from_slice(s);
        } else {
            let ptr = (Box::into_raw(s.to_vec().into_boxed_slice()) as *mut u8).expose_provenance();
            self.data[..8].copy_from_slice(&ptr.to_le_bytes());
        }
        self.len = s.len() as u32;
    }

    fn overflow_bytes(&self) -> usize {
        if self.len as usize > FENCE_INLINE && self.len != NO_FENCE {
            self.len as usize
        } else {
            0
        }
    }
}

/// A segment.
#[repr(C, align(64))]
pub(super) struct Segment {
    /// The previous segment, or [`NONE`].
    pub(super) prev: u32,
    /// The next segment, or [`NONE`].
    pub(super) next: u32,
    /// The slots of `pages` that are in use.
    slots: u64,
    /// The number of keys.
    keys: u16,
    /// The number of nonempty entries of the table.
    used: u16,
    /// The number of pages.
    npages: u8,
    init: bool,
    /// The root of the descent tree, or [`NO_NODE`].
    root: u8,
    _pad: [u8; 41],
    /// The separator of the first page.
    low: Fence,
    /// The separator of the first page of the next segment, if any.
    high: Fence,
    /// The slots of the pages, in order.
    order: [u8; MAX_PAGES],
    /// The identifiers of the pages; the index of a page in this array (its
    /// *slot*) does not change as long as the page is in the segment.
    pages: [u32; MAX_PAGES],
    /// For each page but the first one, in order, the length of the longest
    /// common prefix of its separator and of that of the previous page (or
    /// [`BIG`]), possibly with the bit [`END`].
    dist: [u16; MAX_PAGES],
    /// The byte following the common prefix in the separator of the
    /// previous page (if the bit [`END`] is not set).
    syma: [u8; MAX_PAGES],
    /// The byte following the common prefix in the separator of the page.
    symb: [u8; MAX_PAGES],
    /// The descent tree: the Cartesian tree of `dist`, in which the root of
    /// each subtree is the leftmost position of minimum distance among the
    /// positions of the subtree. An element is the left (right) child of
    /// the position, or [`NO_NODE`].
    left: [u8; MAX_PAGES],
    right: [u8; MAX_PAGES],
    /// The hash table: an entry contains 14 bits of hash, the slot of a page
    /// and the offset of a blob.
    table: [u32; TABLE],
}

const _: () = assert!(size_of::<Segment>() % 64 == 0 && size_of::<Fence>() == 64);
const _: () = assert!(std::mem::offset_of!(Segment, low) == 64);

// SAFETY: a zeroed segment has short, empty fences and no pages
unsafe impl Block for Segment {
    fn clear(&mut self) {
        if self.init {
            self.low.clear();
            self.high.clear();
        }
        self.init = false;
        self.npages = 0;
        self.slots = 0;
        self.keys = 0;
        self.used = 0;
    }
}

/// Returns the home position in the table and the bits stored in an entry
/// for a key with the given hash.
#[inline(always)]
pub(super) fn home_tag(hash: u64) -> (u16, u32) {
    (
        ((hash >> 20) as usize & (TABLE - 1)) as u16,
        ((hash >> 32) as u32 & 0x3FFF) << 18,
    )
}

#[inline(always)]
fn loc(slot: usize, off: u16) -> u32 {
    debug_assert!(slot < MAX_PAGES && (off as usize) < 4096 && off > 1);
    (slot as u32) << 12 | off as u32
}

const LOC_MASK: u32 = (1 << 18) - 1;

impl Segment {
    /// Initializes an empty segment with the given fences.
    pub(super) fn reset(&mut self, low: &[u8], high: Option<&[u8]>) {
        Block::clear(self);
        self.low.set(Some(low));
        self.high.set(high);
        self.table = [EMPTY; TABLE];
        self.init = true;
    }

    /// Returns the separator of the first page.
    #[inline(always)]
    pub(super) fn low(&self) -> &[u8] {
        self.low.get().unwrap()
    }

    /// Returns the separator of the first page of the next segment, if any.
    #[inline(always)]
    pub(super) fn high(&self) -> Option<&[u8]> {
        self.high.get()
    }

    pub(super) fn set_high(&mut self, high: Option<&[u8]>) {
        self.high.set(high);
    }

    /// Returns true if `x` belongs to the interval of the segment.
    #[inline(always)]
    pub(super) fn covers(&self, x: &[u8]) -> bool {
        x >= self.low() && self.high().is_none_or(|high| x < high)
    }

    /// Returns the number of pages.
    #[inline(always)]
    pub(super) fn npages(&self) -> usize {
        self.npages as usize
    }

    /// Returns the number of keys.
    #[inline(always)]
    pub(super) fn keys(&self) -> usize {
        self.keys as usize
    }

    /// Returns the slot of the page of given rank.
    #[inline(always)]
    pub(super) fn slot(&self, pos: usize) -> usize {
        debug_assert!(pos < self.npages as usize);
        self.order[pos] as usize
    }

    /// Returns the identifier of the page of given rank.
    #[inline(always)]
    pub(super) fn page(&self, pos: usize) -> u32 {
        self.pages[self.slot(pos)]
    }

    /// Returns the identifier of the page in the given slot.
    #[inline(always)]
    pub(super) fn page_in(&self, slot: usize) -> u32 {
        self.pages[slot]
    }

    /// Returns the rank of the page in the given slot.
    pub(super) fn rank_of(&self, slot: usize) -> usize {
        self.order[..self.npages as usize]
            .iter()
            .position(|&s| s as usize == slot)
            .unwrap()
    }

    /// Prefetches the lines accessed by a membership query for a key with
    /// the given home position.
    #[inline(always)]
    pub(super) fn prefetch_point(&self, home: u16) {
        let base = self as *const Segment as *const u8;
        prefetch(base);
        prefetch(base.wrapping_add(64));
        prefetch(base.wrapping_add(128));
        for i in (0..MAX_PAGES).step_by(16) {
            prefetch(self.pages.as_ptr().wrapping_add(i));
        }
        prefetch(self.table.as_ptr().wrapping_add(home as usize));
    }

    /// Prefetches the lines accessed when looking for the page containing a
    /// string.
    #[inline(always)]
    pub(super) fn prefetch_ordered(&self) {
        let base = self as *const Segment as *const u8;
        // Everything but the table
        for i in (0..std::mem::offset_of!(Segment, table)).step_by(64) {
            prefetch(base.wrapping_add(i));
        }
    }

    // The hash table

    /// Calls `f` on the slot and offset of each key with the given home
    /// position and bits of hash, until `f` returns true; returns true if
    /// that happens.
    #[inline(always)]
    pub(super) fn probe(&self, home: u16, tag: u32, mut f: impl FnMut(usize, u16) -> bool) -> bool {
        let mut i = home as usize;
        loop {
            let entry = self.table[i];
            if entry == EMPTY {
                return false;
            }
            if entry & !LOC_MASK == tag
                && entry != TOMB
                && f(
                    (entry >> 12) as usize & (MAX_PAGES - 1),
                    (entry & 0xFFF) as u16,
                )
            {
                return true;
            }
            i = (i + 1) & (TABLE - 1);
        }
    }

    /// Returns the index of the entry of the key with the given home
    /// position, slot and offset.
    #[inline(always)]
    pub(super) fn entry(&self, home: u16, slot: usize, off: u16) -> usize {
        let loc = loc(slot, off);
        let mut i = home as usize;
        loop {
            let entry = self.table[i];
            assert!(entry != EMPTY, "entry not found");
            if entry & LOC_MASK == loc && entry != TOMB {
                return i;
            }
            i = (i + 1) & (TABLE - 1);
        }
    }

    /// Returns the bits of hash of an entry.
    #[inline(always)]
    pub(super) fn tag(&self, index: usize) -> u32 {
        self.table[index] & !LOC_MASK
    }

    /// Changes the slot and offset of an entry.
    #[inline(always)]
    pub(super) fn relocate(&mut self, index: usize, slot: usize, off: u16) {
        self.table[index] = self.table[index] & !LOC_MASK | loc(slot, off);
    }

    /// Returns true if a key can be added.
    #[inline(always)]
    pub(super) fn has_room(&self) -> bool {
        (self.keys as usize) < MAX_KEYS && (self.used as usize) < MAX_USED
    }

    /// Returns true if the table contains too many deleted entries, and
    /// must be rebuilt before adding a key.
    #[inline(always)]
    pub(super) fn is_dirty(&self) -> bool {
        (self.keys as usize) < MAX_KEYS && self.used as usize >= MAX_USED
    }

    /// Adds an entry.
    #[inline(always)]
    pub(super) fn add(&mut self, home: u16, tag: u32, slot: usize, off: u16) {
        debug_assert!((self.used as usize) < TABLE - 1);
        let mut i = home as usize;
        while self.table[i] > TOMB {
            i = (i + 1) & (TABLE - 1);
        }
        if self.table[i] == EMPTY {
            self.used += 1;
        }
        self.table[i] = tag | loc(slot, off);
        self.keys += 1;
    }

    /// Removes the entry of the key with the given home position, slot and
    /// offset.
    #[inline(always)]
    pub(super) fn remove(&mut self, home: u16, slot: usize, off: u16) {
        let i = self.entry(home, slot, off);
        if self.table[(i + 1) & (TABLE - 1)] == EMPTY {
            self.table[i] = EMPTY;
            self.used -= 1;
        } else {
            self.table[i] = TOMB;
        }
        self.keys -= 1;
    }

    /// Empties the table.
    pub(super) fn clear_table(&mut self) {
        self.table = [EMPTY; TABLE];
        self.keys = 0;
        self.used = 0;
    }

    // The pages

    /// Returns a free slot.
    pub(super) fn free_slot(&self) -> usize {
        debug_assert!((self.npages as usize) < MAX_PAGES);
        (!self.slots).trailing_zeros() as usize
    }

    /// Adds a page with the given rank in the given slot, which must be
    /// free. The elements of the blind directory for the page and for the
    /// following one must be set afterwards.
    pub(super) fn insert_page(&mut self, pos: usize, slot: usize, page: u32) {
        let n = self.npages as usize;
        debug_assert!(n < MAX_PAGES && pos <= n && self.slots & (1 << slot) == 0);
        self.order.copy_within(pos..n, pos + 1);
        self.dist.copy_within(pos..n, pos + 1);
        self.syma.copy_within(pos..n, pos + 1);
        self.symb.copy_within(pos..n, pos + 1);
        self.order[pos] = slot as u8;
        self.pages[slot] = page;
        self.slots |= 1 << slot;
        self.npages += 1;
        self.rebuild_tree();
    }

    /// Removes the page of given rank. The element of the blind directory
    /// for the page that takes its place, if any, must be set afterwards.
    pub(super) fn remove_page(&mut self, pos: usize) {
        let n = self.npages as usize;
        let slot = self.order[pos] as usize;
        self.order.copy_within(pos + 1..n, pos);
        self.dist.copy_within(pos + 1..n, pos);
        self.syma.copy_within(pos + 1..n, pos);
        self.symb.copy_within(pos + 1..n, pos);
        self.pages[slot] = NONE;
        self.slots &= !(1 << slot);
        self.npages -= 1;
        self.rebuild_tree();
    }

    /// Rebuilds the descent tree; it must be called after each change to the
    /// blind directory.
    pub(super) fn rebuild_tree(&mut self) {
        let n = self.npages as usize;
        let mut stack = [0_u8; MAX_PAGES];
        let mut len = 0;
        for k in 1..n {
            let d = self.dist[k] & BIG as u16;
            let mut last = NO_NODE;
            while len > 0 && self.dist[stack[len - 1] as usize] & BIG as u16 > d {
                last = stack[len - 1];
                len -= 1;
            }
            self.left[k] = last;
            self.right[k] = NO_NODE;
            if len > 0 {
                self.right[stack[len - 1] as usize] = k as u8;
            }
            stack[len] = k as u8;
            len += 1;
        }
        self.root = if n > 1 { stack[0] } else { NO_NODE };
    }

    /// Sets the element of the blind directory for the page of given rank,
    /// which must not be the first one, given the separator of the previous
    /// page and that of the page.
    pub(super) fn set_blind(&mut self, pos: usize, prev: &[u8], sep: &[u8]) {
        debug_assert!(pos > 0 && prev < sep);
        let d = lcp(prev, sep);
        if d >= BIG {
            self.dist[pos] = BIG as u16;
        } else {
            self.symb[pos] = sep[d];
            if d < prev.len() {
                self.dist[pos] = d as u16;
                self.syma[pos] = prev[d];
            } else {
                self.dist[pos] = d as u16 | END;
                self.syma[pos] = 0;
            }
        }
        self.rebuild_tree();
    }

    /// Copies the element of the blind directory of given rank from another
    /// segment; [`rebuild_tree`](Self::rebuild_tree) must be called
    /// afterwards.
    pub(super) fn copy_blind(&mut self, pos: usize, from: &Segment, from_pos: usize) {
        self.dist[pos] = from.dist[from_pos];
        self.syma[pos] = from.syma[from_pos];
        self.symb[pos] = from.symb[from_pos];
    }

    #[inline(always)]
    fn sep<'a>(&self, pos: usize, pages: &'a Slab<Page>) -> &'a [u8] {
        pages.get(self.page(pos)).low()
    }

    /// Returns the rank of the page that contains `x` if the symbols of
    /// `x` that the blind directory does not know are those of the
    /// separators (in any case, the rank of a page).
    #[inline(always)]
    pub(super) fn candidate(&self, x: &[u8]) -> usize {
        let mut k = self.root;
        if k == NO_NODE {
            return 0;
        }
        // The first rank of the interval of pages of the current subtree
        let mut lo = 0;
        loop {
            let pos = k as usize;
            let d = self.dist[pos];
            let depth = d as usize & BIG;
            if depth == BIG {
                return lo;
            }
            if depth > x.len() {
                // If x is a prefix of the separators, it precedes them
                return lo.saturating_sub(1);
            }
            let sym = sym_at(x, depth);
            if sym > self.symb[pos] as u16 {
                // The symbol of x is not smaller than that of the separator
                // of the page: we move to the pages from this one on
                lo = pos;
                k = self.right[pos];
                if k == NO_NODE {
                    return pos;
                }
            } else {
                // The pages before this one share the symbol of the previous
                // separator
                let prev = if d & END != 0 {
                    0
                } else {
                    self.syma[pos] as u16 + 1
                };
                if sym > prev {
                    return pos - 1;
                }
                if sym < prev {
                    return lo.saturating_sub(1);
                }
                k = self.left[pos];
                if k == NO_NODE {
                    return lo;
                }
            }
        }
    }

    /// Returns the rank of the page containing `x`, which must belong to
    /// the interval of the segment, given the rank of a page: the page
    /// with the largest separator that is not larger than `x`.
    ///
    /// The separator of the given page is compared with `x`; then, in most
    /// cases the blind directory is sufficient to find the result.
    #[inline(always)]
    pub(super) fn locate(&self, x: &[u8], from: usize, pages: &Slab<Page>) -> usize {
        let n = self.npages as usize;
        let sep = self.sep(from, pages);
        let mut l = lcp(x, sep);
        if l == sep.len() || (l < x.len() && x[l] > sep[l]) {
            // The separator is not larger than x: we move right, keeping
            // the length l of the longest common prefix of x and of the
            // separator of the page of rank pos
            let mut pos = from;
            if l == x.len() {
                return pos;
            }
            for k in pos + 1..n {
                let d = self.dist[k] as usize & BIG;
                if d < l {
                    break;
                }
                if d == l && l < BIG {
                    let sym = self.symb[k] as u16 + 1;
                    match sym_at(x, l).cmp(&sym) {
                        std::cmp::Ordering::Less => break,
                        std::cmp::Ordering::Greater => {}
                        std::cmp::Ordering::Equal => {
                            let sep = self.sep(k, pages);
                            let m = lcp_from(x, sep, l + 1);
                            if !(m == sep.len() || (m < x.len() && x[m] > sep[m])) {
                                break;
                            }
                            l = m;
                        }
                    }
                } else if d >= BIG || l >= BIG {
                    // The directory does not help
                    let sep = self.sep(k, pages);
                    let m = lcp(x, sep);
                    if !(m == sep.len() || (m < x.len() && x[m] > sep[m])) {
                        break;
                    }
                    l = m;
                }
                pos = k;
            }
            pos
        } else {
            // The separator is larger than x: we move left, keeping the
            // length l of the longest common prefix of x and of the
            // separator of the page of rank k
            for k in (1..=from).rev() {
                let d = self.dist[k] as usize & BIG;
                if d >= BIG || l >= BIG {
                    let sep = self.sep(k - 1, pages);
                    let m = lcp(x, sep);
                    if m == sep.len() || (m < x.len() && x[m] > sep[m]) {
                        return k - 1;
                    }
                    l = m;
                } else if d < l {
                    return k - 1;
                } else if d == l {
                    let sym = if self.dist[k] & END != 0 {
                        0
                    } else {
                        self.syma[k] as u16 + 1
                    };
                    match sym_at(x, l).cmp(&sym) {
                        std::cmp::Ordering::Greater => return k - 1,
                        std::cmp::Ordering::Less => {}
                        std::cmp::Ordering::Equal => {
                            if sym == 0 {
                                // x is the separator
                                return k - 1;
                            }
                            let sep = self.sep(k - 1, pages);
                            let m = lcp_from(x, sep, l + 1);
                            if m == sep.len() || (m < x.len() && x[m] > sep[m]) {
                                return k - 1;
                            }
                            l = m;
                        }
                    }
                }
            }
            debug_assert!(false, "the string precedes the segment");
            0
        }
    }

    /// Returns the number of bytes of memory out of the segment.
    pub(super) fn overflow_bytes(&self) -> usize {
        if self.init {
            self.low.overflow_bytes() + self.high.overflow_bytes()
        } else {
            0
        }
    }

    /// Checks the invariants of the segment that do not depend on the keys.
    pub(super) fn check(&self, pages: &Slab<Page>) {
        assert!(self.init);
        let n = self.npages as usize;
        assert!((1..=MAX_PAGES).contains(&n));
        assert_eq!(self.slots.count_ones() as usize, n);
        let mut seen = 0_u64;
        for pos in 0..n {
            let slot = self.order[pos] as usize;
            assert!(self.slots & (1 << slot) != 0 && seen & (1 << slot) == 0);
            seen |= 1 << slot;
        }
        assert_eq!(self.low(), self.sep(0, pages));
        for pos in 1..n {
            let (prev, sep) = (self.sep(pos - 1, pages), self.sep(pos, pages));
            assert!(prev < sep, "separators out of order");
            let d = lcp(prev, sep);
            if d >= BIG {
                assert_eq!(self.dist[pos] as usize, BIG);
            } else {
                assert_eq!(self.dist[pos] as usize & BIG, d);
                assert_eq!(self.symb[pos], sep[d]);
                assert_eq!(self.dist[pos] & END != 0, d == prev.len());
                if d < prev.len() {
                    assert_eq!(self.syma[pos], prev[d]);
                }
            }
            // The directory finds each separator, from any page
            assert_eq!(self.locate(sep, self.candidate(sep), pages), pos);
            for from in 0..n {
                assert_eq!(self.locate(sep, from, pages), pos);
            }
        }
        if let Some(high) = self.high() {
            assert!(self.sep(n - 1, pages) < high);
        }
        let live = self.table.iter().filter(|&&e| e > TOMB).count();
        let used = self.table.iter().filter(|&&e| e != EMPTY).count();
        assert_eq!(live, self.keys as usize);
        assert_eq!(used, self.used as usize);
        assert!(used < TABLE);
    }
}
