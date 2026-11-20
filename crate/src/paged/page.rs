/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! The pages of a [paged set](super::ZFastPagedSet).
//!
//! A page occupies [`PAGE_SIZE`] bytes and contains the keys of an interval
//! [*low* . . *high*) of strings. The string *low* is the *separator* of the
//! page, and it is stored in the page; *high* is the separator of the next
//! page. All keys of the page start with the longest common prefix of *low*
//! and *high*, which is stored just once (as a prefix of *low*): for each key
//! we store just the remaining suffix.
//!
//! Suffixes (and the separator) are stored as *blobs* in a heap that grows
//! from the end of the page; a blob that is too long is stored out of the
//! page. An array of *slots* in increasing order of key grows from the start
//! of the page: a slot contains the *head* of a suffix (its first four
//! bytes), the offset of its blob, and a few bits of the hash of the key.
//! Thus, the position of a string is found by a binary search on the heads
//! that accesses blobs only to break ties.
//!
//! Membership queries do not use the slots: the [segment](super::segment)
//! containing the page maps the hash of a key directly to the offset of its
//! blob.

use super::PagedKey;
use super::slab::Block;
use crate::utils::{lcp, prefetch};

/// The size of a page in bytes.
pub(super) const PAGE_SIZE: usize = 4096;
const HEADER_LEN: usize = 16;
const SLOT_LEN: usize = size_of::<Slot>();
/// The maximum length of a blob stored in the page.
const MAX_INLINE: usize = 256;
/// The size of the header of a blob (its length).
const BLOB_HEADER: usize = 2;
/// The value of the length field of a blob stored out of the page.
const OVERFLOW: u16 = u16::MAX;
/// The size of a blob stored out of the page: the header, the length and a
/// pointer.
const OVERFLOW_SIZE: usize = 16;
/// The number of bytes available for slots and blobs.
pub(super) const ROOM: usize = PAGE_SIZE - HEADER_LEN;

/// The identifier of no page, segment or node.
pub(super) const NONE: u32 = u32::MAX;

/// Returns the head of a suffix: its first four bytes, padded with zeros, as
/// a big-endian integer, so that the order of heads is compatible with the
/// order of suffixes.
#[inline(always)]
fn head_of(suffix: &[u8]) -> u32 {
    if suffix.len() >= 4 {
        u32::from_be_bytes(suffix[..4].try_into().unwrap())
    } else {
        let mut b = [0; 4];
        b[..suffix.len()].copy_from_slice(suffix);
        u32::from_be_bytes(b)
    }
}

/// Returns the size of the blob of a string of given length.
#[inline(always)]
pub(super) const fn blob_size(len: usize) -> usize {
    if len <= MAX_INLINE {
        BLOB_HEADER + len
    } else {
        OVERFLOW_SIZE
    }
}

/// The largest number of bytes that a key, or a separator, can occupy.
pub(super) const MAX_BLOB: usize = blob_size(MAX_INLINE) + SLOT_LEN;

/// A slot.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct Slot {
    /// The head of the suffix.
    head: u32,
    /// The offset of the blob of the suffix.
    off: u16,
    /// Bits of the hash of the key (see the segment).
    hash: u16,
}

/// A page.
#[repr(C, align(64))]
pub(super) struct Page {
    /// The length of the prefix shared by all keys of the page, which is the
    /// longest common prefix of the separator of this page and of that of
    /// the next page (zero for the last page).
    plen: u32,
    /// The number of keys.
    count: u16,
    /// The heap occupies the bytes from this offset to the end.
    top: u16,
    /// The number of bytes of the heap occupied by deleted blobs.
    dead: u16,
    /// The offset of the blob of the separator.
    low: u16,
    /// Whether the page has a separator.
    init: bool,
    _pad: [u8; 3],
    /// The slots, followed by free space and by the heap; offsets are
    /// relative to the start of the page.
    data: [u8; ROOM],
}

const _: () = assert!(size_of::<Page>() == PAGE_SIZE);
// After a split, each page must have room for a new key: it contains at most
// half of the bytes of the keys, plus a key, and a separator
const _: () = assert!(ROOM / 2 + 3 * MAX_BLOB <= ROOM);

// SAFETY: a zeroed page has no separator and no keys
unsafe impl Block for Page {
    fn clear(&mut self) {
        Page::clear(self)
    }
}

impl Page {
    /// Returns a new page without separator.
    pub(super) fn new_boxed() -> Box<Page> {
        let layout = std::alloc::Layout::new::<Page>();
        // SAFETY: the layout has nonzero size, and a zeroed page is valid
        unsafe {
            let ptr = std::alloc::alloc_zeroed(layout) as *mut Page;
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            let mut page = Box::from_raw(ptr);
            page.clear();
            page
        }
    }

    #[inline(always)]
    fn slots(&self) -> &[Slot] {
        // SAFETY: the data start at an offset that is a multiple of the
        // alignment of a slot, and the slots are followed by the heap
        unsafe {
            std::slice::from_raw_parts(self.data.as_ptr() as *const Slot, self.count as usize)
        }
    }

    /// Returns the first `n` slots, which must be in the page.
    #[inline(always)]
    fn slots_mut(&mut self, n: usize) -> &mut [Slot] {
        debug_assert!(HEADER_LEN + n * SLOT_LEN <= self.top as usize);
        // SAFETY: as above
        unsafe { std::slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut Slot, n) }
    }

    /// Returns the bytes of the page starting at the given offset.
    #[inline(always)]
    fn at(&self, off: usize) -> &[u8] {
        &self.data[off - HEADER_LEN..]
    }

    #[inline(always)]
    fn at_mut(&mut self, off: usize) -> &mut [u8] {
        &mut self.data[off - HEADER_LEN..]
    }

    #[inline(always)]
    fn rd16(&self, off: usize) -> u16 {
        let b = self.at(off);
        u16::from_le_bytes([b[0], b[1]])
    }

    /// Returns the content of the blob at the given offset.
    #[inline(always)]
    pub(super) fn blob(&self, off: u16) -> &[u8] {
        let off = off as usize;
        let len = self.rd16(off);
        if len != OVERFLOW {
            &self.at(off + BLOB_HEADER)[..len as usize]
        } else {
            self.overflow(off)
        }
    }

    #[cold]
    fn overflow(&self, off: usize) -> &[u8] {
        let b = self.at(off);
        let len = u32::from_le_bytes(b[4..8].try_into().unwrap());
        let ptr = usize::from_le_bytes(b[8..16].try_into().unwrap());
        // SAFETY: the pointer and the length were obtained from a boxed
        // slice that is owned by the page
        unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) }
    }

    /// Returns the number of bytes of the heap occupied by a blob.
    #[inline(always)]
    fn raw_size(&self, off: u16) -> usize {
        let len = self.rd16(off as usize);
        if len != OVERFLOW {
            BLOB_HEADER + len as usize
        } else {
            OVERFLOW_SIZE
        }
    }

    /// Returns the length of the content of a blob.
    #[inline(always)]
    fn blob_len(&self, off: u16) -> usize {
        let len = self.rd16(off as usize);
        if len != OVERFLOW {
            len as usize
        } else {
            let b = self.at(off as usize);
            u32::from_le_bytes(b[4..8].try_into().unwrap()) as usize
        }
    }

    /// Returns the number of free bytes between the slots and the heap.
    #[inline(always)]
    fn free(&self) -> usize {
        self.top as usize - HEADER_LEN - self.count as usize * SLOT_LEN
    }

    /// Stores a blob in the heap, which must have enough room; if `out` is
    /// true, the content is stored out of the page even if it is short.
    fn alloc_blob(&mut self, bytes: &[u8], out: bool) -> u16 {
        let out = out || bytes.len() > MAX_INLINE;
        let size = if out {
            OVERFLOW_SIZE
        } else {
            BLOB_HEADER + bytes.len()
        };
        debug_assert!(self.free() >= size);
        let off = self.top as usize - size;
        self.top = off as u16;
        let b = self.at_mut(off);
        if !out {
            b[..2].copy_from_slice(&(bytes.len() as u16).to_le_bytes());
            b[BLOB_HEADER..size].copy_from_slice(bytes);
        } else {
            assert!(bytes.len() <= u32::MAX as usize, "key too long");
            let ptr = Box::into_raw(bytes.to_vec().into_boxed_slice()) as *mut u8 as usize;
            b[..2].copy_from_slice(&OVERFLOW.to_le_bytes());
            b[4..8].copy_from_slice(&(bytes.len() as u32).to_le_bytes());
            b[8..16].copy_from_slice(&ptr.to_le_bytes());
        }
        off as u16
    }

    /// Releases the memory of a blob stored out of the page.
    fn free_blob(&mut self, off: u16) {
        let off = off as usize;
        if self.rd16(off) == OVERFLOW {
            let b = self.at(off);
            let len = u32::from_le_bytes(b[4..8].try_into().unwrap());
            let ptr = usize::from_le_bytes(b[8..16].try_into().unwrap());
            // SAFETY: see `overflow`; the blob is not used any longer
            unsafe {
                drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                    ptr as *mut u8,
                    len as usize,
                )));
            }
        }
    }

    /// Removes separator and keys, releasing the memory out of the page.
    pub(super) fn clear(&mut self) {
        if self.init {
            for i in 0..self.count as usize {
                self.free_blob(self.slots()[i].off);
            }
            self.free_blob(self.low);
        }
        self.init = false;
        self.count = 0;
        self.dead = 0;
        self.top = PAGE_SIZE as u16;
    }

    /// Empties the page and sets its separator; `high` is the separator of
    /// the next page, or `None` for the last page.
    pub(super) fn reset(&mut self, low: &[u8], high: Option<&[u8]>) {
        self.clear();
        let plen = high.map_or(0, |high| {
            debug_assert!(low < high);
            lcp(low, high)
        });
        self.plen = plen as u32;
        self.low = self.alloc_blob(low, false);
        self.init = true;
    }

    /// Returns the number of keys.
    #[inline(always)]
    pub(super) fn len(&self) -> usize {
        self.count as usize
    }

    /// Returns the length of the prefix shared by all keys.
    #[inline(always)]
    pub(super) fn plen(&self) -> usize {
        self.plen as usize
    }

    /// Returns the separator.
    #[inline(always)]
    pub(super) fn low(&self) -> &[u8] {
        debug_assert!(self.init);
        self.blob(self.low)
    }

    /// Returns the prefix shared by all keys.
    #[inline(always)]
    pub(super) fn prefix(&self) -> &[u8] {
        &self.low()[..self.plen as usize]
    }

    /// Prefetches the lines accessed when comparing a string with the
    /// separator, or with the prefix.
    #[inline(always)]
    pub(super) fn prefetch_low(&self) {
        let base = self as *const Page as *const u8;
        prefetch(base);
        // The separator is at the end of the heap
        prefetch(base.wrapping_add(PAGE_SIZE - 64));
        prefetch(base.wrapping_add(PAGE_SIZE - 128));
    }

    #[inline(always)]
    fn prefetch_at(&self, off: u16) {
        let base = self as *const Page as *const u8;
        prefetch(base.wrapping_add(off as usize));
        prefetch(base.wrapping_add(off as usize + 63));
    }

    /// Prefetches the lines accessed when comparing a string with the key
    /// whose blob has the given offset.
    #[inline(always)]
    pub(super) fn prefetch_blob(&self, off: u16) {
        self.prefetch_at(off);
        self.prefetch_low();
    }

    /// Returns the key whose blob has the given offset.
    #[inline(always)]
    pub(super) fn key_at(&self, off: u16) -> PagedKey<'_> {
        PagedKey {
            prefix: self.prefix(),
            suffix: self.blob(off),
        }
    }

    /// Prefetches the whole page.
    #[inline(always)]
    pub(super) fn prefetch_all(&self) {
        let base = self as *const Page as *const u8;
        for i in 0..PAGE_SIZE / 64 {
            prefetch(base.wrapping_add(64 * i));
        }
    }

    /// Prefetches the lines accessed by a binary search.
    #[inline(always)]
    pub(super) fn prefetch_slots(&self) {
        let base = self as *const Page as *const u8;
        for i in 1..9 {
            prefetch(base.wrapping_add(64 * i));
        }
        self.prefetch_low();
    }

    /// Returns true if the key whose blob has the given offset is `x`.
    #[inline(always)]
    pub(super) fn is(&self, off: u16, x: &[u8]) -> bool {
        let plen = self.plen as usize;
        x.len() >= plen && self.blob(off) == &x[plen..] && self.low()[..plen] == x[..plen]
    }

    /// Returns the suffix of the key of given rank.
    #[inline(always)]
    pub(super) fn suffix(&self, pos: usize) -> &[u8] {
        self.blob(self.slots()[pos].off)
    }

    /// Returns the offset of the blob of the key of given rank.
    #[inline(always)]
    pub(super) fn off(&self, pos: usize) -> u16 {
        self.slots()[pos].off
    }

    /// Returns the bits of hash of the key of given rank.
    #[inline(always)]
    pub(super) fn hash(&self, pos: usize) -> u16 {
        self.slots()[pos].hash
    }

    /// Returns the key of given rank.
    #[inline(always)]
    pub(super) fn key(&self, pos: usize) -> PagedKey<'_> {
        PagedKey {
            prefix: self.prefix(),
            suffix: self.suffix(pos),
        }
    }

    /// Returns the number of keys smaller than the key with given suffix,
    /// and whether the key is in the page.
    #[inline(always)]
    pub(super) fn lower_bound(&self, suffix: &[u8]) -> (usize, bool) {
        let head = head_of(suffix);
        let slots = self.slots();
        // The keys with the same head
        let mut lo = slots.partition_point(|s| s.head < head);
        let mut hi = lo + slots[lo..].partition_point(|s| s.head == head);
        // The suffixes with the same head will be compared with the given
        // one, and the following one may be the result: we fetch them at
        // the same time
        if hi - lo <= 8 {
            for slot in &slots[lo..slots.len().min(hi + 1)] {
                self.prefetch_at(slot.off);
            }
        } else {
            for i in 1..8 {
                self.prefetch_at(slots[lo + (hi - lo) * i / 8].off);
            }
        }
        while lo < hi {
            let mid = (lo + hi) / 2;
            match self.blob(slots[mid].off).cmp(suffix) {
                std::cmp::Ordering::Less => lo = mid + 1,
                std::cmp::Ordering::Equal => return (mid, true),
                std::cmp::Ordering::Greater => hi = mid,
            }
        }
        (lo, false)
    }

    /// Returns the rank of the key with the given suffix, whose blob has the
    /// given offset.
    #[inline(always)]
    pub(super) fn rank(&self, off: u16, suffix: &[u8]) -> usize {
        let head = head_of(suffix);
        let slots = self.slots();
        let mut pos = slots.partition_point(|s| s.head < head);
        while slots[pos].off != off {
            pos += 1;
        }
        pos
    }

    /// Returns true if a key with a suffix of given length can be added,
    /// possibly after a [compaction](Self::compact).
    #[inline(always)]
    pub(super) fn fits_compacted(&self, suffix_len: usize) -> bool {
        self.free() + self.dead as usize >= blob_size(suffix_len) + SLOT_LEN
    }

    /// Adds a key of given rank, which must not be in the page; returns the
    /// offset of its blob, or `None` if there is no room.
    #[inline(always)]
    pub(super) fn insert(&mut self, pos: usize, suffix: &[u8], hash: u16) -> Option<u16> {
        self.insert_blob(pos, suffix, hash, false)
    }

    #[inline(always)]
    fn insert_blob(&mut self, pos: usize, suffix: &[u8], hash: u16, out: bool) -> Option<u16> {
        let count = self.count as usize;
        let size = if out {
            OVERFLOW_SIZE
        } else {
            blob_size(suffix.len())
        };
        if self.free() < size + SLOT_LEN {
            return None;
        }
        let off = self.alloc_blob(suffix, out);
        let slots = self.slots_mut(count + 1);
        slots.copy_within(pos..count, pos + 1);
        slots[pos] = Slot {
            head: head_of(suffix),
            off,
            hash,
        };
        self.count += 1;
        Some(off)
    }

    /// Adds a key that is larger than all keys of the page, and returns the
    /// offset of its blob; if `out` is true, the suffix is stored out of the
    /// page even if it is short.
    ///
    /// # Panics
    ///
    /// If there is no room.
    pub(super) fn push(&mut self, suffix: &[u8], hash: u16, out: bool) -> u16 {
        debug_assert!(self.count == 0 || self.suffix(self.count as usize - 1) < suffix);
        self.insert_blob(self.count as usize, suffix, hash, out)
            .expect("page overflow")
    }

    /// Returns true if the suffix of the key of given rank is stored out of
    /// the page.
    #[inline(always)]
    pub(super) fn is_out(&self, pos: usize) -> bool {
        self.rd16(self.slots()[pos].off as usize) == OVERFLOW
    }

    /// Returns the number of bytes occupied by the key of given rank.
    #[inline(always)]
    pub(super) fn key_size(&self, pos: usize) -> usize {
        self.raw_size(self.slots()[pos].off) + SLOT_LEN
    }

    /// Removes the key of given rank.
    pub(super) fn remove(&mut self, pos: usize) {
        let count = self.count as usize;
        let off = self.slots()[pos].off;
        self.dead += self.raw_size(off) as u16;
        self.free_blob(off);
        self.slots_mut(count).copy_within(pos + 1..count, pos);
        self.count -= 1;
    }

    /// Returns true if a compaction would reclaim something.
    #[inline(always)]
    pub(super) fn is_dirty(&self) -> bool {
        self.dead != 0
    }

    /// Rewrites the heap without deleted blobs; for each key, calls `moved`
    /// with the bits of hash, the old offset and the new offset of the blob.
    pub(super) fn compact(&mut self, mut moved: impl FnMut(u16, u16, u16)) {
        let mut heap = [0; PAGE_SIZE];
        let mut top = PAGE_SIZE;
        let mut copy = |page: &Page, off: u16| {
            let size = page.raw_size(off);
            top -= size;
            heap[top..top + size].copy_from_slice(&page.at(off as usize)[..size]);
            top as u16
        };
        self.low = copy(self, self.low);
        let count = self.count as usize;
        for i in 0..count {
            let slot = self.slots()[i];
            let off = copy(self, slot.off);
            moved(slot.hash, slot.off, off);
            self.slots_mut(count)[i].off = off;
        }
        self.top = top as u16;
        self.at_mut(top).copy_from_slice(&heap[top..]);
        self.dead = 0;
    }

    /// Returns the number of bytes that are in use.
    #[inline(always)]
    pub(super) fn used(&self) -> usize {
        ROOM - self.free() - self.dead as usize
    }

    /// Returns the number of bytes needed to store the keys of this page if
    /// the length of the shared prefix becomes `plen`, which must not be
    /// larger than the present one (suffixes stored out of the page remain
    /// there).
    pub(super) fn key_bytes_with(&self, plen: usize) -> usize {
        let delta = self.plen as usize - plen;
        (0..self.count as usize)
            .map(|i| {
                SLOT_LEN
                    + if self.is_out(i) {
                        OVERFLOW_SIZE
                    } else {
                        blob_size(self.blob_len(self.slots()[i].off) + delta)
                    }
            })
            .sum()
    }

    /// Returns the number of bytes of out-of-page memory.
    pub(super) fn overflow_bytes(&self) -> usize {
        if !self.init {
            return 0;
        }
        let ext = |off: u16| {
            if self.rd16(off as usize) == OVERFLOW {
                self.blob_len(off)
            } else {
                0
            }
        };
        ext(self.low)
            + (0..self.count as usize)
                .map(|i| ext(self.slots()[i].off))
                .sum::<usize>()
    }

    /// Checks the invariants of the page; `high` is the separator of the
    /// next page, if any.
    pub(super) fn check(&self, high: Option<&[u8]>) {
        assert!(self.init);
        let count = self.count as usize;
        let low = self.low();
        let plen = self.plen as usize;
        match high {
            Some(high) => {
                assert!(low < high);
                assert_eq!(plen, lcp(low, high));
            }
            None => assert_eq!(plen, 0),
        }
        let mut live = self.raw_size(self.low);
        let slots = self.slots();
        debug_assert_eq!(slots.len(), count);
        for (i, slot) in slots.iter().enumerate() {
            let suffix = self.suffix(i);
            assert_eq!(slot.head, head_of(suffix));
            assert!(suffix >= &low[plen..], "key below the separator");
            if let Some(high) = high {
                assert!(suffix < &high[plen..], "key above the next separator");
            }
            if i > 0 {
                assert!(self.suffix(i - 1) < suffix, "keys out of order");
            }
            live += self.raw_size(slot.off) + SLOT_LEN;
            assert_eq!(self.lower_bound(suffix), (i, true));
            assert_eq!(self.rank(slot.off, suffix), i);
        }
        assert_eq!(live, self.used(), "wrong accounting");
    }
}
