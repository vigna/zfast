/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! Shared immutable byte strings.

use std::alloc::{Layout, alloc, dealloc, handle_alloc_error};
use std::ptr::NonNull;

/// A reference-counted immutable byte string.
///
/// This is essentially an `Rc<[u8]>` represented by a thin pointer: the
/// reference count, the length, and the bytes live in a single allocation.
/// Thus, a `Key` uses eight bytes in a node, and the bytes of the key are
/// reached with a single memory access.
///
/// A key is shared by the leaf it belongs to and, possibly, by an internal
/// node that uses a prefix of the key as its extent: internal nodes store no
/// bytes of their own. An internal node can outlive the leaf of the key it
/// shares.
///
/// The reference count is not atomic. Sharing is always explicit (there is
/// no `Clone` implementation), and it happens only in methods of the trie
/// that take a mutable reference.
pub(super) struct Key(NonNull<u32>);

/// The size in bytes of the header (reference count and length).
const HEADER: usize = 8;

impl Key {
    /// Creates a new key with reference count one by copying the given
    /// bytes.
    ///
    /// # Panics
    ///
    /// If there are more than `u32::MAX` bytes.
    pub fn new(bytes: &[u8]) -> Self {
        let len = u32::try_from(bytes.len()).expect("key too long");
        let layout = Self::layout(bytes.len());
        // SAFETY: the layout has nonzero size
        let ptr = unsafe { alloc(layout) } as *mut u32;
        let Some(ptr) = NonNull::new(ptr) else {
            handle_alloc_error(layout)
        };
        // SAFETY: the allocation is aligned to four bytes and has room for
        // the header and the bytes
        unsafe {
            ptr.as_ptr().write(1);
            ptr.as_ptr().add(1).write(len);
            std::ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                ptr.as_ptr().add(2) as *mut u8,
                bytes.len(),
            );
        }
        Key(ptr)
    }

    #[inline(always)]
    fn layout(len: usize) -> Layout {
        Layout::from_size_align(HEADER + len, 4).unwrap()
    }

    /// Returns the bytes of the key.
    #[inline(always)]
    pub fn bytes(&self) -> &[u8] {
        // SAFETY: the allocation is live as long as self exists, and it
        // contains the length followed by that many initialized bytes
        unsafe {
            let len = *self.0.as_ptr().add(1) as usize;
            std::slice::from_raw_parts(self.0.as_ptr().add(2) as *const u8, len)
        }
    }

    /// Returns a new handle to the same key, increasing the reference
    /// count.
    #[inline]
    pub fn share(&self) -> Self {
        // SAFETY: the allocation is live; no reference to the count exists
        unsafe { *self.0.as_ptr() += 1 };
        Key(self.0)
    }

    /// Returns the number of bytes allocated for this key.
    #[inline]
    pub fn alloc_size(&self) -> usize {
        HEADER + self.bytes().len()
    }

    /// Returns the reference count.
    #[cfg(test)]
    fn count(&self) -> u32 {
        unsafe { *self.0.as_ptr() }
    }
}

impl Drop for Key {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: the allocation is live, and it was obtained with the same
        // layout
        unsafe {
            let count = self.0.as_ptr();
            *count -= 1;
            if *count == 0 {
                let layout = Self::layout(*count.add(1) as usize);
                dealloc(count as *mut u8, layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key() {
        for len in [0, 1, 3, 4, 5, 100] {
            let bytes: Vec<u8> = (0..len).map(|i| i as u8).collect();
            let key = Key::new(&bytes);
            assert_eq!(key.bytes(), bytes);
            assert_eq!(key.count(), 1);
            let shared = key.share();
            assert_eq!(key.count(), 2);
            drop(key);
            assert_eq!(shared.count(), 1);
            assert_eq!(shared.bytes(), bytes);
            assert_eq!(shared.alloc_size(), 8 + len);
        }
    }
}
