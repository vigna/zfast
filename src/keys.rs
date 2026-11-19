/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! Random-access sequences of byte strings.

use mem_dbg::{MemDbg, MemSize};
use sux::traits::{IndexedSeq, Types};

/// A random-access sequence of byte strings.
///
/// This trait abstracts the storage of the keys indexed by a
/// [`StaticZFastTrie`]. It is implemented by every [`IndexedSeq`] whose
/// output can be seen as a slice of bytes (e.g., the [rear-coded lists] and
/// the [`SliceSeq`] of [`sux`]), and by the uncompressed, pointerless
/// [`PackedKeys`]; one can also implement it, for example, for the suffixes of
/// a text in the order given by a suffix array.
///
/// Besides access by index, the trait provides access by *position*: a
/// position is an opaque value that can be obtained from an index using
/// [`pos`](KeySeq::pos), and that makes it possible to retrieve a key, using
/// [`key_at`](KeySeq::key_at), possibly faster than its index. Indices
/// themselves are the default positions.
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

    /// Returns the position of the key of given index.
    ///
    /// Positions must be smaller than 2⁴⁰.
    ///
    /// # Panics
    ///
    /// May panic if the index is not smaller than
    /// [`num_keys`](KeySeq::num_keys).
    #[inline(always)]
    fn pos(&self, index: usize) -> usize {
        index
    }

    /// Returns the key at the given position, which must have been returned
    /// by [`pos`](KeySeq::pos).
    #[inline(always)]
    fn key_at(&self, pos: usize) -> Self::Key<'_> {
        self.key(pos)
    }
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

/// The number of bytes used to store the length of a key.
const LEN_BYTES: usize = 4;

/// A sequence of byte strings stored contiguously.
///
/// Each key is stored, preceded by its length, in a single sequence of bytes;
/// a second sequence contains the position at which each key starts. Besides
/// using less space than a vector of vectors, this representation needs no
/// pointer chasing: a key is retrieved by reading its position and then its
/// bytes, and a key whose [position](KeySeq::pos) is known is retrieved
/// directly.
///
/// The type parameters `B` and `O` are the backends for the bytes and for the
/// positions, respectively: the first one must be viewable as a slice of bytes,
/// and the second one must be an [`IndexedSeq`] returning `usize`, which
/// includes vectors, boxed slices, and slices, but also, for example, an
/// [Elias–Fano representation]. Instances are built with [boxed
/// slices](PackedKeys::from_iter) or [vectors](PackedKeys::push) as backends;
/// when the structure is deserialized using ε-serde, the backends are
/// replaced by references to slices.
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
///
/// [Elias–Fano representation]: sux::dict::EliasFano
#[derive(Debug, Clone, PartialEq, Eq, MemSize, MemDbg)]
#[cfg_attr(feature = "epserde", derive(epserde::Epserde))]
pub struct PackedKeys<B = Box<[u8]>, O = Box<[usize]>> {
    /// The keys, each preceded by its length as a little-endian 32-bit
    /// integer.
    bytes: B,
    /// The position in `bytes` of each key (i.e., of its length).
    offsets: O,
}

impl PackedKeys<Vec<u8>, Vec<usize>> {
    /// Creates a new empty sequence with vectors as backends.
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            offsets: Vec::new(),
        }
    }

    /// Appends a key.
    ///
    /// # Panics
    ///
    /// If the key is longer than `u32::MAX` bytes.
    pub fn push(&mut self, key: impl AsRef<[u8]>) {
        let key = key.as_ref();
        let len = u32::try_from(key.len()).expect("key too long");
        self.offsets.push(self.bytes.len());
        self.bytes.extend_from_slice(&len.to_le_bytes());
        self.bytes.extend_from_slice(key);
    }

    /// Converts the backends into boxed slices.
    pub fn into_boxed(self) -> PackedKeys {
        PackedKeys {
            bytes: self.bytes.into_boxed_slice(),
            offsets: self.offsets.into_boxed_slice(),
        }
    }
}

impl Default for PackedKeys<Vec<u8>, Vec<usize>> {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PackedKeys {
    fn default() -> Self {
        PackedKeys::new().into_boxed()
    }
}

impl<B, O> PackedKeys<B, O> {
    /// Replaces the backend of the positions.
    ///
    /// This method can be used, for example, to store the positions in an
    /// [Elias–Fano representation].
    ///
    /// # Safety
    ///
    /// The new backend must return the same values as the old one.
    ///
    /// [Elias–Fano representation]: sux::dict::EliasFano
    pub unsafe fn map_offsets<O2>(self, f: impl FnOnce(O) -> O2) -> PackedKeys<B, O2> {
        PackedKeys {
            bytes: self.bytes,
            offsets: f(self.offsets),
        }
    }

    /// Returns the backends.
    pub fn into_raw_parts(self) -> (B, O) {
        (self.bytes, self.offsets)
    }
}

impl<B: AsRef<[u8]>, O: IndexedSeq> PackedKeys<B, O>
where
    for<'a> O: Types<Output<'a> = usize>,
{
    /// Returns the number of keys.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.offsets.len()
    }

    /// Returns true if there are no keys.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the overall number of bytes in the keys.
    pub fn num_bytes(&self) -> usize {
        self.bytes.as_ref().len() - LEN_BYTES * self.len()
    }

    /// Returns an iterator on the keys.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &[u8]> + '_ {
        (0..self.len()).map(|i| self.key(i))
    }
}

impl<B: AsRef<[u8]>, O: IndexedSeq> KeySeq for PackedKeys<B, O>
where
    for<'a> O: Types<Output<'a> = usize>,
{
    type Key<'a>
        = &'a [u8]
    where
        Self: 'a;

    #[inline(always)]
    fn num_keys(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn key(&self, index: usize) -> &[u8] {
        self.key_at(self.offsets.get(index))
    }

    #[inline(always)]
    fn pos(&self, index: usize) -> usize {
        self.offsets.get(index)
    }

    #[inline(always)]
    fn key_at(&self, pos: usize) -> &[u8] {
        let bytes = self.bytes.as_ref();
        let start = pos + LEN_BYTES;
        let len = u32::from_le_bytes(bytes[pos..start].try_into().unwrap()) as usize;
        &bytes[start..start + len]
    }
}

impl<A: AsRef<[u8]>> FromIterator<A> for PackedKeys<Vec<u8>, Vec<usize>> {
    fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self {
        let mut keys = PackedKeys::new();
        keys.extend(iter);
        keys
    }
}

impl<A: AsRef<[u8]>> FromIterator<A> for PackedKeys {
    fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self {
        iter.into_iter()
            .collect::<PackedKeys<Vec<u8>, Vec<usize>>>()
            .into_boxed()
    }
}

impl<A: AsRef<[u8]>> Extend<A> for PackedKeys<Vec<u8>, Vec<usize>> {
    fn extend<I: IntoIterator<Item = A>>(&mut self, iter: I) {
        for key in iter {
            self.push(key);
        }
    }
}
