/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

//! Random-access sequences of byte strings.

use mem_dbg::{MemDbg, MemSize};
use sux::traits::IndexedSeq;

/// A random-access sequence of byte strings.
///
/// This trait abstracts the storage of the keys indexed by a
/// [`StaticZFastTrie`]. It is implemented by every [`IndexedSeq`] whose
/// output can be seen as a slice of bytes (e.g., the [rear-coded lists] and
/// the [`SliceSeq`] of [`sux`]), and by the uncompressed, pointerless
/// [`PackedKeys`]; one can also implement it, for example, for the suffixes of
/// a text in the order given by a suffix array.
///
/// [`StaticZFastTrie`]: crate::StaticZFastTrie
/// [rear-coded lists]: sux::dict::RearCodedList
/// [`SliceSeq`]: sux::dict::SliceSeq
pub trait KeySeq {
    /// The type of a key, which may borrow from the sequence or own its
    /// bytes.
    type Key<'a>: AsRef<[u8]>
    where
        Self: 'a;

    /// Returns the number of keys.
    ///
    /// The name of this method avoids ambiguities with
    /// [`IndexedSeq::len`].
    fn num_keys(&self) -> usize;

    /// Returns the key of given index.
    ///
    /// # Panics
    ///
    /// May panic if the index is not smaller than
    /// [`num_keys`](KeySeq::num_keys).
    fn key(&self, index: usize) -> Self::Key<'_>;
}

impl<S: IndexedSeq + ?Sized> KeySeq for S
where
    for<'a> S::Output<'a>: AsRef<[u8]>,
{
    type Key<'a>
        = S::Output<'a>
    where
        Self: 'a;

    #[inline(always)]
    fn num_keys(&self) -> usize {
        IndexedSeq::len(self)
    }

    #[inline(always)]
    fn key(&self, index: usize) -> Self::Key<'_> {
        self.get(index)
    }
}

/// A sequence of byte strings stored contiguously.
///
/// The keys are concatenated in a single allocation, and a second allocation
/// contains the position at which each key ends. Besides using less space than
/// a vector of vectors, this representation needs no pointer chasing: a key is
/// retrieved by reading two adjacent positions and then the bytes.
///
/// # Examples
///
/// ```
/// # use zfast::{KeySeq, PackedKeys};
/// let keys: PackedKeys = ["a", "ab", "b"].into_iter().collect();
/// assert_eq!(keys.len(), 3);
/// assert_eq!(keys.key(1), b"ab");
///
/// // Any indexed sequence from sux whose output is made of bytes works, too
/// use sux::dict::SliceSeq;
/// let keys = SliceSeq::from(["a", "ab", "b"]);
/// assert_eq!(KeySeq::key(&keys, 1), "ab");
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, MemSize, MemDbg)]
pub struct PackedKeys {
    bytes: Vec<u8>,
    /// `ends[i]` is the position at which key `i - 1` ends, and `ends[0]` is
    /// zero.
    ends: Vec<usize>,
}

impl PackedKeys {
    /// Creates a new empty sequence.
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            ends: vec![0],
        }
    }

    /// Returns the number of keys.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.ends.len().saturating_sub(1)
    }

    /// Returns true if there are no keys.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Appends a key.
    pub fn push(&mut self, key: impl AsRef<[u8]>) {
        if self.ends.is_empty() {
            self.ends.push(0);
        }
        self.bytes.extend_from_slice(key.as_ref());
        self.ends.push(self.bytes.len());
    }

    /// Releases the memory that is not used.
    pub fn shrink_to_fit(&mut self) {
        self.bytes.shrink_to_fit();
        self.ends.shrink_to_fit();
    }

    /// Returns the overall number of bytes in the keys.
    pub fn num_bytes(&self) -> usize {
        self.bytes.len()
    }

    /// Returns an iterator on the keys.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &[u8]> + '_ {
        (0..self.len()).map(|i| self.key(i))
    }
}

impl KeySeq for PackedKeys {
    type Key<'a> = &'a [u8];

    #[inline(always)]
    fn num_keys(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn key(&self, index: usize) -> &[u8] {
        &self.bytes[self.ends[index]..self.ends[index + 1]]
    }
}

impl<B: AsRef<[u8]>> FromIterator<B> for PackedKeys {
    fn from_iter<I: IntoIterator<Item = B>>(iter: I) -> Self {
        let mut keys = PackedKeys::new();
        for key in iter {
            keys.push(key);
        }
        keys.shrink_to_fit();
        keys
    }
}

impl<B: AsRef<[u8]>> Extend<B> for PackedKeys {
    fn extend<I: IntoIterator<Item = B>>(&mut self, iter: I) {
        for key in iter {
            self.push(key);
        }
    }
}
