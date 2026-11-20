/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! The pages of a [paged set](super::ZFastPagedSet), and their allocator.
//!
//! A page occupies [`PAGE_SIZE`] bytes and contains the keys of an interval
//! [*low* . . *high*) of strings; *low* and *high* are the *fences* of the
//! page, and *low* is also the string associated with the page in the
//! router. All keys of the page start with the longest common prefix of the
//! fences, which is stored just once (as a prefix of *low*): for each key we
//! store just the remaining suffix.
//!
//! Suffixes (and fences) are stored as *blobs* in a heap that grows from the
//! end of the page; a blob that is too long is stored out of the page. The
//! keys are indexed in two ways, so that both kinds of queries access a
//! number of cache lines that is small, and known as soon as the page is
//! known:
//!
//! - a small open-addressing hash table, with one-byte tags, maps the hash of
//!   a key to the offset of its blob: a membership query accesses the line of
//!   the tag, the line of the offset and the blob;
//!
//! - a sorted array of *heads* (the first four bytes of each suffix), with a
//!   parallel array of offsets, makes it possible to find the position of a
//!   string by a binary search that accesses blobs only to break ties.

use super::PagedKey;
use crate::utils::{lcp, prefetch};
use std::alloc::{Layout, alloc_zeroed, handle_alloc_error};
use std::ptr::NonNull;

/// The size of a page in bytes.
pub(super) const PAGE_SIZE: usize = 4096;
/// The number of slots of the hash table of a page (a power of two).
pub(super) const CAP: usize = 128;
/// The maximum number of keys in a page: it is smaller than [`CAP`] so that
/// searches in the hash table always terminate.
pub(super) const MAX_KEYS: usize = 104;
/// The maximum number of nonempty slots of the hash table.
const MAX_USED: usize = CAP - 8;
const HEADER_LEN: usize = 32;
/// The size of the heap.
pub(super) const HEAP_LEN: usize = PAGE_SIZE - HEADER_LEN - CAP * (1 + 2 + 4 + 2);
/// The maximum length of a blob stored in the page.
const MAX_INLINE: usize = 256;
/// The size of the header of a blob: its length and 16 bits of hash.
const BLOB_HEADER: usize = 4;
/// The value of the length field of a blob stored out of the page.
const OVERFLOW: u16 = u16::MAX;
/// The size of a blob stored out of the page: the header, the length and a
/// pointer.
const OVERFLOW_SIZE: usize = 16;
/// The value of the field `high` for the last page.
const NO_HIGH: u16 = u16::MAX;
/// The tag of an empty slot.
const EMPTY: u8 = 0;
/// The tag of a deleted slot.
const TOMB: u8 = 0xFF;

/// The identifier of no page.
pub(super) const NONE: u32 = u32::MAX;

/// What a query is going to access in a page.
#[derive(Clone, Copy)]
pub(super) enum Hint {
    /// The hash table, for a key with the given hash.
    Point(u16),
    /// The sorted arrays.
    Ordered,
}

/// Returns the tag associated with a hash.
#[inline(always)]
const fn tag_of(h16: u16) -> u8 {
    let t = (h16 >> 8) as u8;
    if t == EMPTY {
        1
    } else if t == TOMB {
        0xFE
    } else {
        t
    }
}

/// Returns the home slot associated with a hash.
#[inline(always)]
const fn home_of(h16: u16) -> usize {
    h16 as usize & (CAP - 1)
}

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

/// A page.
#[repr(C, align(64))]
pub(super) struct Page {
    /// The length of the prefix shared by all keys of the page, which is the
    /// longest common prefix of the fences (zero for the last page).
    plen: u32,
    /// The previous page, or [`NONE`].
    pub(super) prev: u32,
    /// The next page, or [`NONE`].
    pub(super) next: u32,
    /// The number of keys.
    count: u16,
    /// The number of deleted slots of the hash table.
    tombs: u16,
    /// The heap occupies the bytes from this offset to the end.
    top: u16,
    /// The number of bytes of the heap occupied by deleted blobs.
    dead: u16,
    /// The offset of the blob of the lower fence.
    low: u16,
    /// The offset of the blob of the suffix of the upper fence, or
    /// [`NO_HIGH`].
    high: u16,
    /// Whether the page has fences.
    init: bool,
    _pad: [u8; 7],
    /// The tags of the hash table.
    tags: [u8; CAP],
    /// The offsets of the blobs of the hash table.
    hoffs: [u16; CAP],
    /// The heads of the keys, in order.
    heads: [u32; CAP],
    /// The offsets of the blobs of the keys, in order.
    soffs: [u16; CAP],
    heap: [u8; HEAP_LEN],
}

const _: () = assert!(size_of::<Page>() == PAGE_SIZE);
const _: () = assert!(MAX_KEYS < MAX_USED && CAP.is_power_of_two());
// After a split, each page must have room for a new key: it contains at most
// half of the bytes of the keys, plus a key, and two fences
const _: () = assert!(HEAP_LEN / 2 + 4 * blob_size(MAX_INLINE) <= HEAP_LEN);

impl Page {
    /// Returns a new page without fences.
    pub(super) fn new_boxed() -> Box<Page> {
        let layout = Layout::new::<Page>();
        // SAFETY: the layout has nonzero size, and a zeroed page is valid
        unsafe {
            let ptr = alloc_zeroed(layout) as *mut Page;
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            Box::from_raw(ptr)
        }
    }

    #[inline(always)]
    fn rd16(&self, off: usize) -> u16 {
        u16::from_le_bytes([self.heap[off], self.heap[off + 1]])
    }

    /// Returns the content of a blob.
    #[inline(always)]
    fn blob(&self, off: u16) -> &[u8] {
        let off = off as usize;
        let len = self.rd16(off);
        if len != OVERFLOW {
            &self.heap[off + BLOB_HEADER..off + BLOB_HEADER + len as usize]
        } else {
            self.overflow(off)
        }
    }

    #[cold]
    fn overflow(&self, off: usize) -> &[u8] {
        let len = u32::from_le_bytes(self.heap[off + 4..off + 8].try_into().unwrap());
        let ptr = usize::from_le_bytes(self.heap[off + 8..off + 16].try_into().unwrap());
        // SAFETY: the pointer and the length were obtained from a boxed
        // slice that is owned by the page
        unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) }
    }

    /// Returns the 16 bits of hash stored in a blob.
    #[inline(always)]
    fn blob_hash(&self, off: u16) -> u16 {
        self.rd16(off as usize + 2)
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
            let off = off as usize;
            u32::from_le_bytes(self.heap[off + 4..off + 8].try_into().unwrap()) as usize
        }
    }

    /// Stores a blob in the heap, which must have enough room; if `out` is
    /// true, the content is stored out of the page even if it is short.
    fn alloc_blob(&mut self, bytes: &[u8], h16: u16, out: bool) -> u16 {
        let out = out || bytes.len() > MAX_INLINE;
        let size = if out {
            OVERFLOW_SIZE
        } else {
            BLOB_HEADER + bytes.len()
        };
        debug_assert!(self.top as usize >= size);
        let off = self.top as usize - size;
        self.top = off as u16;
        self.heap[off + 2..off + 4].copy_from_slice(&h16.to_le_bytes());
        if !out {
            self.heap[off..off + 2].copy_from_slice(&(bytes.len() as u16).to_le_bytes());
            self.heap[off + BLOB_HEADER..off + size].copy_from_slice(bytes);
        } else {
            assert!(bytes.len() <= u32::MAX as usize, "key too long");
            let ptr = Box::into_raw(bytes.to_vec().into_boxed_slice()) as *mut u8 as usize;
            self.heap[off..off + 2].copy_from_slice(&OVERFLOW.to_le_bytes());
            self.heap[off + 4..off + 8].copy_from_slice(&(bytes.len() as u32).to_le_bytes());
            self.heap[off + 8..off + 16].copy_from_slice(&ptr.to_le_bytes());
        }
        off as u16
    }

    /// Releases the memory of a blob stored out of the page.
    fn free_blob(&mut self, off: u16) {
        let off = off as usize;
        if self.rd16(off) == OVERFLOW {
            let len = u32::from_le_bytes(self.heap[off + 4..off + 8].try_into().unwrap());
            let ptr = usize::from_le_bytes(self.heap[off + 8..off + 16].try_into().unwrap());
            // SAFETY: see `overflow`; the blob is not used any longer
            unsafe {
                drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                    ptr as *mut u8,
                    len as usize,
                )));
            }
        }
    }

    /// Removes fences and keys, releasing the memory out of the page.
    pub(super) fn clear(&mut self) {
        if self.init {
            for i in 0..self.count as usize {
                self.free_blob(self.soffs[i]);
            }
            self.free_blob(self.low);
            if self.high != NO_HIGH {
                self.free_blob(self.high);
            }
        }
        self.init = false;
        self.count = 0;
        self.tombs = 0;
        self.dead = 0;
        self.top = HEAP_LEN as u16;
        self.tags = [EMPTY; CAP];
    }

    /// Empties the page and sets its fences; `high` is `None` for the last
    /// page.
    pub(super) fn reset(&mut self, low: &[u8], high: Option<&[u8]>) {
        self.clear();
        let plen = high.map_or(0, |high| {
            debug_assert!(low < high);
            lcp(low, high)
        });
        self.plen = plen as u32;
        self.low = self.alloc_blob(low, 0, false);
        self.high = match high {
            Some(high) => self.alloc_blob(&high[plen..], 0, false),
            None => NO_HIGH,
        };
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

    /// Returns the lower fence.
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

    /// Returns the upper fence without the prefix shared by all keys, or
    /// `None` for the last page.
    #[inline(always)]
    pub(super) fn high_suffix(&self) -> Option<&[u8]> {
        (self.high != NO_HIGH).then(|| self.blob(self.high))
    }

    /// Returns true if `x` belongs to the interval of the page.
    #[inline(always)]
    pub(super) fn covers(&self, x: &[u8]) -> bool {
        let low = self.low();
        let plen = self.plen as usize;
        if x.len() < plen || x[..plen] != low[..plen] {
            return false;
        }
        let suffix = &x[plen..];
        suffix >= &low[plen..] && self.high_suffix().is_none_or(|high| suffix < high)
    }

    /// Prefetches the lines that a query will access.
    #[inline(always)]
    pub(super) fn prefetch(&self, hint: Hint) {
        let base = self as *const Page as *const u8;
        prefetch(base);
        // The fences are at the end of the heap
        prefetch(base.wrapping_add(PAGE_SIZE - 64));
        prefetch(base.wrapping_add(PAGE_SIZE - 128));
        match hint {
            Hint::Point(h16) => {
                let home = home_of(h16);
                prefetch(self.tags.as_ptr().wrapping_add(home));
                prefetch(self.hoffs.as_ptr().wrapping_add(home));
            }
            Hint::Ordered => {
                for i in (0..MAX_KEYS).step_by(16) {
                    prefetch(self.heads.as_ptr().wrapping_add(i));
                }
                for i in (0..MAX_KEYS).step_by(32) {
                    prefetch(self.soffs.as_ptr().wrapping_add(i));
                }
            }
        }
    }

    /// Returns the suffix of the key of given rank.
    #[inline(always)]
    pub(super) fn suffix(&self, pos: usize) -> &[u8] {
        debug_assert!(pos < self.count as usize);
        self.blob(self.soffs[pos])
    }

    /// Returns the 16 bits of hash of the key of given rank.
    #[inline(always)]
    pub(super) fn hash(&self, pos: usize) -> u16 {
        self.blob_hash(self.soffs[pos])
    }

    /// Returns the key of given rank.
    #[inline(always)]
    pub(super) fn key(&self, pos: usize) -> PagedKey<'_> {
        PagedKey {
            prefix: self.prefix(),
            suffix: self.suffix(pos),
        }
    }

    /// Returns the slot of the hash table containing the key with the given
    /// suffix and hash, if any.
    #[inline(always)]
    pub(super) fn find(&self, suffix: &[u8], h16: u16) -> Option<usize> {
        let tag = tag_of(h16);
        let mut i = home_of(h16);
        loop {
            let t = self.tags[i];
            if t == EMPTY {
                return None;
            }
            if t == tag && self.blob(self.hoffs[i]) == suffix {
                return Some(i);
            }
            i = (i + 1) & (CAP - 1);
        }
    }

    /// Returns the number of keys smaller than the key with given suffix,
    /// and whether the key is in the page.
    #[inline(always)]
    pub(super) fn lower_bound(&self, suffix: &[u8]) -> (usize, bool) {
        let head = head_of(suffix);
        let heads = &self.heads[..self.count as usize];
        // The keys with the same head
        let mut lo = heads.partition_point(|&h| h < head);
        let mut hi = lo + heads[lo..].partition_point(|&h| h == head);
        while lo < hi {
            let mid = (lo + hi) / 2;
            match self.blob(self.soffs[mid]).cmp(suffix) {
                std::cmp::Ordering::Less => lo = mid + 1,
                std::cmp::Ordering::Equal => return (mid, true),
                std::cmp::Ordering::Greater => hi = mid,
            }
        }
        (lo, false)
    }

    /// Returns true if a key with a suffix of given length can be added,
    /// possibly after a [compaction](Self::compact).
    #[inline(always)]
    pub(super) fn fits_compacted(&self, suffix_len: usize) -> bool {
        (self.count as usize) < MAX_KEYS
            && self.top as usize + self.dead as usize >= blob_size(suffix_len)
    }

    /// Adds a key of given rank, which must not be in the page; returns
    /// false if there is no room.
    #[inline(always)]
    pub(super) fn insert(&mut self, pos: usize, suffix: &[u8], h16: u16) -> bool {
        self.insert_blob(pos, suffix, h16, false)
    }

    #[inline(always)]
    fn insert_blob(&mut self, pos: usize, suffix: &[u8], h16: u16, out: bool) -> bool {
        let count = self.count as usize;
        let size = if out {
            OVERFLOW_SIZE
        } else {
            blob_size(suffix.len())
        };
        if count >= MAX_KEYS
            || (self.top as usize) < size
            || count + self.tombs as usize >= MAX_USED
        {
            return false;
        }
        let off = self.alloc_blob(suffix, h16, out);
        self.hash_insert(h16, off);
        self.heads.copy_within(pos..count, pos + 1);
        self.soffs.copy_within(pos..count, pos + 1);
        self.heads[pos] = head_of(suffix);
        self.soffs[pos] = off;
        self.count += 1;
        true
    }

    #[inline(always)]
    fn hash_insert(&mut self, h16: u16, off: u16) {
        let mut i = home_of(h16);
        while self.tags[i] != EMPTY && self.tags[i] != TOMB {
            i = (i + 1) & (CAP - 1);
        }
        if self.tags[i] == TOMB {
            self.tombs -= 1;
        }
        self.tags[i] = tag_of(h16);
        self.hoffs[i] = off;
    }

    /// Adds a key that is larger than all keys of the page; if `out` is
    /// true, the suffix is stored out of the page even if it is short.
    ///
    /// # Panics
    ///
    /// If there is no room.
    pub(super) fn push(&mut self, suffix: &[u8], h16: u16, out: bool) {
        debug_assert!(self.count == 0 || self.suffix(self.count as usize - 1) < suffix);
        let done = self.insert_blob(self.count as usize, suffix, h16, out);
        assert!(done, "page overflow");
    }

    /// Returns true if the suffix of the key of given rank is stored out of
    /// the page.
    #[inline(always)]
    pub(super) fn is_out(&self, pos: usize) -> bool {
        self.rd16(self.soffs[pos] as usize) == OVERFLOW
    }

    /// Returns the number of bytes of the heap occupied by the key of given
    /// rank.
    #[inline(always)]
    pub(super) fn key_size(&self, pos: usize) -> usize {
        self.raw_size(self.soffs[pos])
    }

    /// Removes the key in the given slot of the hash table, which has the
    /// given suffix.
    pub(super) fn remove(&mut self, slot: usize, suffix: &[u8]) {
        let off = self.hoffs[slot];
        let count = self.count as usize;
        let head = head_of(suffix);
        let mut pos = self.heads[..count].partition_point(|&h| h < head);
        while self.soffs[pos] != off {
            pos += 1;
        }
        self.heads.copy_within(pos + 1..count, pos);
        self.soffs.copy_within(pos + 1..count, pos);
        self.count -= 1;
        if self.tags[(slot + 1) & (CAP - 1)] == EMPTY {
            self.tags[slot] = EMPTY;
        } else {
            self.tags[slot] = TOMB;
            self.tombs += 1;
        }
        self.dead += self.raw_size(off) as u16;
        self.free_blob(off);
    }

    /// Returns true if a compaction would reclaim something.
    #[inline(always)]
    pub(super) fn is_dirty(&self) -> bool {
        self.dead != 0 || self.tombs != 0
    }

    /// Rewrites the heap without deleted blobs, and rebuilds the hash table.
    pub(super) fn compact(&mut self) {
        let mut heap = [0; HEAP_LEN];
        let mut top = HEAP_LEN;
        let mut copy = |page: &Page, off: u16| {
            let size = page.raw_size(off);
            top -= size;
            heap[top..top + size].copy_from_slice(&page.heap[off as usize..off as usize + size]);
            top as u16
        };
        self.low = copy(self, self.low);
        if self.high != NO_HIGH {
            self.high = copy(self, self.high);
        }
        for i in 0..self.count as usize {
            self.soffs[i] = copy(self, self.soffs[i]);
        }
        self.heap[top..].copy_from_slice(&heap[top..]);
        self.top = top as u16;
        self.dead = 0;
        self.tombs = 0;
        self.tags = [EMPTY; CAP];
        for i in 0..self.count as usize {
            let off = self.soffs[i];
            self.hash_insert(self.blob_hash(off), off);
        }
    }

    /// Returns the number of bytes of the heap that are in use.
    #[inline(always)]
    pub(super) fn used(&self) -> usize {
        HEAP_LEN - self.top as usize - self.dead as usize
    }

    /// Returns the number of bytes of the heap needed to store the keys of
    /// this page if the length of the shared prefix becomes `plen`, which
    /// must not be larger than the present one (suffixes stored out of the
    /// page remain there).
    pub(super) fn key_bytes_with(&self, plen: usize) -> usize {
        let delta = self.plen as usize - plen;
        (0..self.count as usize)
            .map(|i| {
                if self.is_out(i) {
                    OVERFLOW_SIZE
                } else {
                    blob_size(self.blob_len(self.soffs[i]) + delta)
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
        let mut bytes = ext(self.low);
        if self.high != NO_HIGH {
            bytes += ext(self.high);
        }
        bytes
            + (0..self.count as usize)
                .map(|i| ext(self.soffs[i]))
                .sum::<usize>()
    }

    /// Checks the invariants of the page.
    pub(super) fn check(&self) {
        assert!(self.init);
        let count = self.count as usize;
        assert!(count <= MAX_KEYS);
        let low = self.low();
        let plen = self.plen as usize;
        match self.high_suffix() {
            Some(high) => {
                assert!(plen <= low.len());
                assert!(&low[plen..] < high);
                assert_eq!(lcp(&low[plen..], high), 0);
            }
            None => assert_eq!(plen, 0),
        }
        // The sorted arrays
        let mut live = self.raw_size(self.low);
        if self.high != NO_HIGH {
            live += self.raw_size(self.high);
        }
        for i in 0..count {
            let suffix = self.suffix(i);
            assert_eq!(self.heads[i], head_of(suffix));
            assert!(suffix >= &low[plen..], "key below the lower fence");
            if let Some(high) = self.high_suffix() {
                assert!(suffix < high, "key above the upper fence");
            }
            if i > 0 {
                assert!(self.suffix(i - 1) < suffix, "keys out of order");
            }
            live += self.raw_size(self.soffs[i]);
            // The hash table
            let h16 = self.hash(i);
            let slot = self.find(suffix, h16).expect("key not in the hash table");
            assert_eq!(self.hoffs[slot], self.soffs[i]);
            assert_eq!(self.lower_bound(suffix), (i, true));
        }
        assert_eq!(live, self.used(), "wrong heap accounting");
        let used = self
            .tags
            .iter()
            .filter(|&&t| t != EMPTY && t != TOMB)
            .count();
        let tombs = self.tags.iter().filter(|&&t| t == TOMB).count();
        assert_eq!(used, count);
        assert_eq!(tombs, self.tombs as usize);
        assert!(used + tombs < CAP);
    }
}

/// The number of pages in a chunk.
const CHUNK: usize = if cfg!(miri) {
    16
} else if cfg!(target_os = "linux") {
    16384
} else {
    512
};

/// An allocator of pages, which are identified by 32-bit integers.
///
/// Pages are allocated in chunks, so that the address of a page can be
/// computed from its identifier without accessing memory that is not in the
/// cache. On Linux, chunks are mapped directly, they are aligned to two
/// megabytes, and we ask the kernel to back them with huge pages: on large
/// sets almost every access to a page would otherwise require walking the
/// page table, which is particularly expensive on virtual machines.
pub(super) struct Pages {
    chunks: Vec<NonNull<Page>>,
    /// The number of identifiers in use or in the free list.
    allocated: u32,
    free: Vec<u32>,
}

// SAFETY: the chunks are owned by the allocator
unsafe impl Send for Pages {}
// SAFETY: as above
unsafe impl Sync for Pages {}

impl Pages {
    pub(super) fn new() -> Self {
        Self {
            chunks: Vec::new(),
            allocated: 0,
            free: Vec::new(),
        }
    }

    fn chunk_layout() -> Layout {
        Layout::from_size_align(CHUNK * PAGE_SIZE, PAGE_SIZE).unwrap()
    }

    /// Returns a zeroed chunk.
    #[cfg(all(target_os = "linux", not(miri)))]
    fn alloc_chunk() -> NonNull<Page> {
        /// The alignment of chunks.
        const ALIGN: usize = 1 << 21;
        let size = CHUNK * PAGE_SIZE;
        // SAFETY: we map an anonymous region, and unmap the parts before
        // and after the aligned chunk
        unsafe {
            let raw = libc::mmap(
                std::ptr::null_mut(),
                size + ALIGN,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                -1,
                0,
            );
            if raw == libc::MAP_FAILED {
                handle_alloc_error(Self::chunk_layout());
            }
            let start = raw as usize;
            let chunk = (start + ALIGN - 1) & !(ALIGN - 1);
            if chunk > start {
                libc::munmap(raw, chunk - start);
            }
            if chunk < start + ALIGN {
                libc::munmap((chunk + size) as *mut libc::c_void, start + ALIGN - chunk);
            }
            // Failure just means that the chunk will not use huge pages
            libc::madvise(chunk as *mut libc::c_void, size, libc::MADV_HUGEPAGE);
            NonNull::new_unchecked(chunk as *mut Page)
        }
    }

    #[cfg(all(target_os = "linux", not(miri)))]
    fn free_chunk(chunk: NonNull<Page>) {
        // SAFETY: the chunk was mapped by alloc_chunk
        unsafe { libc::munmap(chunk.as_ptr() as *mut libc::c_void, CHUNK * PAGE_SIZE) };
    }

    /// Returns a zeroed chunk.
    #[cfg(not(all(target_os = "linux", not(miri))))]
    fn alloc_chunk() -> NonNull<Page> {
        let layout = Self::chunk_layout();
        // SAFETY: the layout has nonzero size, and a zeroed page is valid
        let ptr = unsafe { alloc_zeroed(layout) } as *mut Page;
        NonNull::new(ptr).unwrap_or_else(|| handle_alloc_error(layout))
    }

    #[cfg(not(all(target_os = "linux", not(miri))))]
    fn free_chunk(chunk: NonNull<Page>) {
        // SAFETY: the chunk was allocated with the same layout
        unsafe { std::alloc::dealloc(chunk.as_ptr() as *mut u8, Self::chunk_layout()) };
    }

    /// Returns the number of pages in use.
    pub(super) fn len(&self) -> usize {
        self.allocated as usize - self.free.len()
    }

    /// Returns the number of bytes of the pages that have been used at
    /// least once (chunks are larger, but the rest of a chunk is just address
    /// space).
    pub(super) fn mem_size(&self) -> usize {
        self.allocated as usize * PAGE_SIZE
            + self.chunks.capacity() * size_of::<NonNull<Page>>()
            + self.free.capacity() * size_of::<u32>()
    }

    #[inline(always)]
    pub(super) fn get(&self, id: u32) -> &Page {
        debug_assert!(id < self.allocated);
        // SAFETY: the identifier is valid, and chunks are never moved
        unsafe {
            &*self.chunks[id as usize / CHUNK]
                .as_ptr()
                .add(id as usize % CHUNK)
        }
    }

    #[inline(always)]
    pub(super) fn get_mut(&mut self, id: u32) -> &mut Page {
        debug_assert!(id < self.allocated);
        // SAFETY: as above
        unsafe {
            &mut *self.chunks[id as usize / CHUNK]
                .as_ptr()
                .add(id as usize % CHUNK)
        }
    }

    /// Returns mutable references to two distinct pages.
    pub(super) fn pair_mut(&mut self, a: u32, b: u32) -> (&mut Page, &mut Page) {
        assert!(a != b && a < self.allocated && b < self.allocated);
        // SAFETY: the identifiers are valid and distinct
        unsafe {
            (
                &mut *self.chunks[a as usize / CHUNK]
                    .as_ptr()
                    .add(a as usize % CHUNK),
                &mut *self.chunks[b as usize / CHUNK]
                    .as_ptr()
                    .add(b as usize % CHUNK),
            )
        }
    }

    /// Returns the lower fence of a page.
    #[inline(always)]
    pub(super) fn low(&self, id: u32) -> &[u8] {
        self.get(id).low()
    }

    /// Returns the identifier of a page without fences.
    pub(super) fn alloc(&mut self) -> u32 {
        if let Some(id) = self.free.pop() {
            return id;
        }
        if self.allocated as usize == self.chunks.len() * CHUNK {
            self.chunks.push(Self::alloc_chunk());
        }
        assert!(self.allocated < (1 << 31) - 1, "too many pages");
        self.allocated += 1;
        self.allocated - 1
    }

    /// Releases a page.
    pub(super) fn release(&mut self, id: u32) {
        self.get_mut(id).clear();
        self.free.push(id);
    }
}

impl Drop for Pages {
    fn drop(&mut self) {
        for id in 0..self.allocated {
            self.get_mut(id).clear();
        }
        for &chunk in &self.chunks {
            Self::free_chunk(chunk);
        }
    }
}
