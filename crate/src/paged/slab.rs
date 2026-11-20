/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! An allocator of blocks of the same type, identified by 32-bit integers.

use std::alloc::{Layout, handle_alloc_error};
use std::marker::PhantomData;
use std::ptr::NonNull;

/// A block that can be allocated by a [`Slab`]: a zeroed block must be a
/// valid instance, and [`clear`](Block::clear) must release the memory owned
/// by the block that is not part of the block itself.
///
/// # Safety
///
/// The all-zeros bit pattern must be a valid instance of the type, on which
/// [`clear`](Block::clear) does nothing.
pub(super) unsafe trait Block: Sized {
    fn clear(&mut self);
}

/// The size in bytes of a chunk.
const CHUNK_BYTES: usize = if cfg!(miri) {
    1 << 16
} else if cfg!(target_os = "linux") {
    1 << 26
} else {
    1 << 21
};

/// An allocator of blocks of type `T`.
///
/// Blocks are allocated in chunks, so that the address of a block can be
/// computed from its identifier without accessing memory that is not in the
/// cache. On Linux, chunks are mapped directly, they are aligned to two
/// megabytes, and we ask the kernel to back them with huge pages: on large
/// sets almost every access to a block would otherwise require walking the
/// page table, which is particularly expensive on virtual machines.
pub(super) struct Slab<T: Block> {
    chunks: Vec<NonNull<u8>>,
    /// The number of identifiers in use or in the free list.
    allocated: u32,
    free: Vec<u32>,
    _marker: PhantomData<T>,
}

// SAFETY: the chunks are owned by the allocator
unsafe impl<T: Block + Send> Send for Slab<T> {}
// SAFETY: as above
unsafe impl<T: Block + Sync> Sync for Slab<T> {}

impl<T: Block> Slab<T> {
    /// The number of blocks in a chunk.
    const PER_CHUNK: usize = {
        assert!(size_of::<T>() <= CHUNK_BYTES && align_of::<T>() <= 4096);
        CHUNK_BYTES / size_of::<T>()
    };

    pub(super) fn new() -> Self {
        Self {
            chunks: Vec::new(),
            allocated: 0,
            free: Vec::new(),
            _marker: PhantomData,
        }
    }

    fn chunk_layout() -> Layout {
        Layout::from_size_align(CHUNK_BYTES, 4096).unwrap()
    }

    /// Returns a zeroed chunk.
    #[cfg(all(target_os = "linux", not(miri)))]
    fn alloc_chunk() -> NonNull<u8> {
        /// The alignment of chunks.
        const ALIGN: usize = 1 << 21;
        // SAFETY: we map an anonymous region, and unmap the parts before
        // and after the aligned chunk
        unsafe {
            let raw = libc::mmap(
                std::ptr::null_mut(),
                CHUNK_BYTES + ALIGN,
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
                libc::munmap(
                    (chunk + CHUNK_BYTES) as *mut libc::c_void,
                    start + ALIGN - chunk,
                );
            }
            // Failure just means that the chunk will not use huge pages
            libc::madvise(chunk as *mut libc::c_void, CHUNK_BYTES, libc::MADV_HUGEPAGE);
            NonNull::new_unchecked(chunk as *mut u8)
        }
    }

    #[cfg(all(target_os = "linux", not(miri)))]
    fn free_chunk(chunk: NonNull<u8>) {
        // SAFETY: the chunk was mapped by alloc_chunk
        unsafe { libc::munmap(chunk.as_ptr() as *mut libc::c_void, CHUNK_BYTES) };
    }

    /// Returns a zeroed chunk.
    #[cfg(not(all(target_os = "linux", not(miri))))]
    fn alloc_chunk() -> NonNull<u8> {
        let layout = Self::chunk_layout();
        // SAFETY: the layout has nonzero size
        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
        NonNull::new(ptr).unwrap_or_else(|| handle_alloc_error(layout))
    }

    #[cfg(not(all(target_os = "linux", not(miri))))]
    fn free_chunk(chunk: NonNull<u8>) {
        // SAFETY: the chunk was allocated with the same layout
        unsafe { std::alloc::dealloc(chunk.as_ptr(), Self::chunk_layout()) };
    }

    /// Returns the number of blocks in use.
    pub(super) fn len(&self) -> usize {
        self.allocated as usize - self.free.len()
    }

    /// Returns the number of bytes of the blocks that have been used at
    /// least once (chunks are larger, but the rest of a chunk is just address
    /// space).
    pub(super) fn mem_size(&self) -> usize {
        self.allocated as usize * size_of::<T>()
            + self.chunks.capacity() * size_of::<NonNull<u8>>()
            + self.free.capacity() * size_of::<u32>()
    }

    #[inline(always)]
    fn ptr(&self, id: u32) -> *mut T {
        debug_assert!(id < self.allocated);
        let id = id as usize;
        // SAFETY: the identifier is valid, and chunks are never moved
        unsafe { (self.chunks[id / Self::PER_CHUNK].as_ptr() as *mut T).add(id % Self::PER_CHUNK) }
    }

    #[inline(always)]
    pub(super) fn get(&self, id: u32) -> &T {
        // SAFETY: see ptr
        unsafe { &*self.ptr(id) }
    }

    #[inline(always)]
    pub(super) fn get_mut(&mut self, id: u32) -> &mut T {
        // SAFETY: see ptr
        unsafe { &mut *self.ptr(id) }
    }

    /// Returns mutable references to two distinct blocks.
    pub(super) fn pair_mut(&mut self, a: u32, b: u32) -> (&mut T, &mut T) {
        assert!(a != b);
        // SAFETY: the identifiers are valid and distinct
        unsafe { (&mut *self.ptr(a), &mut *self.ptr(b)) }
    }

    /// Returns the identifier of a cleared block.
    pub(super) fn alloc(&mut self) -> u32 {
        if let Some(id) = self.free.pop() {
            return id;
        }
        if self.allocated as usize == self.chunks.len() * Self::PER_CHUNK {
            self.chunks.push(Self::alloc_chunk());
        }
        assert!(self.allocated < (1 << 31) - 1, "too many blocks");
        self.allocated += 1;
        self.allocated - 1
    }

    /// Clears and releases a block.
    pub(super) fn release(&mut self, id: u32) {
        self.get_mut(id).clear();
        self.free.push(id);
    }
}

impl<T: Block> Drop for Slab<T> {
    fn drop(&mut self) {
        for id in 0..self.allocated {
            self.get_mut(id).clear();
        }
        for &chunk in &self.chunks {
            Self::free_chunk(chunk);
        }
    }
}
