/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! Directories of the internal nodes of a static trie.
//!
//! A static trie represents each internal node by the interval of ranks of
//! the keys in its subtree. The *directory* of a node contains the interval,
//! and, for each child, the byte associated with the child and the rank at
//! which the child starts (and thus, implicitly, the interval of the child).
//!
//! The trait [`Dirs`] abstracts the access to directories. There are two
//! implementations:
//!
//! - [`PackedDirs`] stores each directory in a contiguous sequence of bytes,
//!   which contains also the [position](KeySeq::pos) of the first key of each
//!   child, so that the key can be accessed directly; it is the fastest
//!   implementation, as usually a directory is contained in a cache line or
//!   two.
//!
//! - [`SeqDirs`] stores the same information (except for positions) in three
//!   monotone sequences, which can be represented, for example, using the
//!   [Elias–Fano representation] of [`sux`] with any underlying selection
//!   structure; it uses much less space, but several memory accesses are
//!   necessary to access a directory.
//!
//! [Elias–Fano representation]: sux::dict::EliasFano

use crate::keys::KeySeq;
use mem_dbg::{MemDbg, MemSize};
use sux::dict::{EfSeq, EliasFanoBuilder};
use sux::traits::{IndexedSeq, Types};

/// Access to the directories of the internal nodes of a static trie.
///
/// Nodes are identified by integers whose meaning depends on the
/// implementation.
pub trait Dirs {
    /// The type of the directory of a node.
    type Node<'a>: DirNode
    where
        Self: 'a;

    /// Returns the directory of the node with given identifier.
    fn node(&self, id: usize) -> Self::Node<'_>;
}

/// The directory of an internal node of a static trie.
pub trait DirNode {
    /// Returns the rank of the first key of the node.
    fn left(&self) -> usize;

    /// Returns the rank of the first key after the node.
    fn right(&self) -> usize;

    /// Returns the index of the child associated with the byte `c`, if there
    /// is one, or otherwise the index of the first child associated with a
    /// larger byte (or the number of children), and whether the child was
    /// found.
    fn child(&self, c: u8) -> (usize, bool);

    /// Returns the rank of the first key of the child of given index, or of
    /// the first key after the node if the index is the number of children.
    fn start(&self, index: usize) -> usize;

    /// Returns the first key of the child of given index.
    fn key<'k, K: KeySeq>(&self, keys: &'k K, index: usize) -> K::Key<'k>;
}

/// Returns the number of elements of `bytes` smaller than `c`, and whether
/// `c` appears in `bytes`, which must be strictly increasing.
#[inline(always)]
fn position(bytes: &[u8], c: u8) -> (usize, bool) {
    let j = bytes.iter().filter(|&&b| b < c).count();
    (j, j < bytes.len() && bytes[j] == c)
}

/// The number of bytes used to store a position.
const POS_BYTES: usize = 5;

/// Directories stored in a contiguous sequence of bytes.
///
/// The directory of a node with *k* children is formed by a header (*k* − 2
/// in the lower 8 bits, and then a bit that is set if the first child is the
/// leaf of a key equal to the extent of the node), followed by the ranks at
/// which each child starts, followed by the rank at which the node ends,
/// followed by the [position](KeySeq::pos) of the first key of each child,
/// followed by the bytes associated with the children (except for the first
/// one, if the bit above is set). Ranks are 32-bit little-endian integers,
/// and positions are 40-bit little-endian integers.
///
/// The identifier of a node is the offset of its directory.
///
/// The type parameter `B` is the backend, which must be viewable as a slice of
/// bytes; when the structure is deserialized using ε-serde, it is replaced by
/// a reference to a slice.
#[derive(Debug, Clone, MemSize, MemDbg)]
#[cfg_attr(feature = "epserde", derive(epserde::Epserde))]
pub struct PackedDirs<B = Box<[u8]>> {
    bytes: B,
}

/// The [directory of a node](DirNode) of a [`PackedDirs`].
#[derive(Debug, Clone, Copy)]
pub struct PackedNode<'a> {
    /// The ranks at which each child starts, followed by the rank at which
    /// the node ends.
    starts: &'a [u8],
    /// The positions of the first key of each child, followed by at least
    /// three more bytes.
    pos: &'a [u8],
    /// The bytes associated with the children.
    syms: &'a [u8],
    /// Whether the first child is the leaf of a key equal to the extent.
    has_end: bool,
}

impl PackedNode<'_> {
    /// Returns the number of children.
    #[inline(always)]
    fn num_children(&self) -> usize {
        self.starts.len() / 4 - 1
    }

    /// Returns the byte associated with the child of given index, if any.
    fn sym(&self, index: usize) -> Option<u8> {
        index
            .checked_sub(self.has_end as usize)
            .map(|i| self.syms[i])
    }
}

impl DirNode for PackedNode<'_> {
    #[inline(always)]
    fn left(&self) -> usize {
        self.start(0)
    }

    #[inline(always)]
    fn right(&self) -> usize {
        self.start(self.num_children())
    }

    #[inline(always)]
    fn child(&self, c: u8) -> (usize, bool) {
        let (j, found) = position(self.syms, c);
        (self.has_end as usize + j, found)
    }

    #[inline(always)]
    fn start(&self, index: usize) -> usize {
        u32::from_le_bytes(self.starts[4 * index..4 * index + 4].try_into().unwrap()) as usize
    }

    #[inline(always)]
    fn key<'k, K: KeySeq>(&self, keys: &'k K, index: usize) -> K::Key<'k> {
        let p = POS_BYTES * index;
        let pos = u64::from_le_bytes(self.pos[p..p + 8].try_into().unwrap());
        keys.key_at((pos & ((1 << (8 * POS_BYTES)) - 1)) as usize)
    }
}

impl<B: AsRef<[u8]>> PackedDirs<B> {
    /// Returns the size in bytes of the directory at the given offset.
    fn size(&self, off: usize) -> usize {
        let node = self.node(off);
        4 + node.starts.len() + POS_BYTES * node.num_children() + node.syms.len()
    }

    /// Returns an iterator on the identifiers of the nodes, in the order in
    /// which they were added.
    pub(super) fn ids(&self) -> impl Iterator<Item = usize> + '_ {
        // The last eight bytes are padding
        let end = self.bytes.as_ref().len() - 8;
        let mut off = 0;
        std::iter::from_fn(move || {
            (off < end).then(|| {
                let id = off;
                off += self.size(id);
                id
            })
        })
    }
}

impl<B: AsRef<[u8]>> Dirs for PackedDirs<B> {
    type Node<'a>
        = PackedNode<'a>
    where
        Self: 'a;

    #[inline(always)]
    fn node(&self, id: usize) -> PackedNode<'_> {
        let bytes = self.bytes.as_ref();
        let header = u32::from_le_bytes(bytes[id..id + 4].try_into().unwrap());
        let k = (header & 0xFF) as usize + 2;
        let has_end = header & 0x100 != 0;
        let starts = id + 4;
        let pos = starts + 4 * (k + 1);
        let syms = pos + POS_BYTES * k;
        PackedNode {
            starts: &bytes[starts..pos],
            // Padding guarantees that there are enough bytes
            pos: &bytes[pos..syms + 3],
            syms: &bytes[syms..syms + k - has_end as usize],
            has_end,
        }
    }
}

/// A builder for [`PackedDirs`].
#[derive(Debug, Default)]
pub(super) struct PackedDirsBuilder {
    bytes: Vec<u8>,
}

impl PackedDirsBuilder {
    /// Adds the directory of a node, returning its identifier.
    ///
    /// # Arguments
    ///
    /// * `starts` - the ranks at which each child starts.
    ///
    /// * `right` - the rank at which the node ends.
    ///
    /// * `extent` - the length of the extent of the node.
    ///
    /// * `keys` - the keys.
    ///
    /// Returns `None` if the directory cannot be represented.
    pub fn push(
        &mut self,
        starts: &[u32],
        right: usize,
        extent: usize,
        keys: &impl KeySeq,
    ) -> Option<usize> {
        let id = self.bytes.len();
        let k = starts.len();
        debug_assert!((2..=257).contains(&k));
        let has_end = keys.key(starts[0] as usize).as_ref().len() == extent;
        self.bytes
            .extend_from_slice(&((k - 2) as u32 | (has_end as u32) << 8).to_le_bytes());
        for &s in starts {
            self.bytes.extend_from_slice(&s.to_le_bytes());
        }
        self.bytes.extend_from_slice(&(right as u32).to_le_bytes());
        for &s in starts {
            let pos = keys.pos(s as usize);
            if pos >= 1 << (8 * POS_BYTES) {
                return None;
            }
            self.bytes
                .extend_from_slice(&pos.to_le_bytes()[..POS_BYTES]);
        }
        for &s in &starts[has_end as usize..] {
            self.bytes.push(keys.key(s as usize).as_ref()[extent]);
        }
        Some(id)
    }

    pub fn build(mut self) -> PackedDirs {
        // Padding, so that positions can be read as 64-bit integers
        self.bytes.extend_from_slice(&[0; 8]);
        PackedDirs {
            bytes: self.bytes.into_boxed_slice(),
        }
    }
}

/// Directories stored in three monotone sequences of integers, plus the
/// bytes associated with the children.
///
/// Nodes are numbered in the order in which they are closed by a depth-first
/// visit of the trie, and the number of a node is its identifier. For a node
/// *j* whose children have overall index *o<sub>j</sub>*, *o<sub>j</sub>* + 1,
/// …:
/// - `offsets` contains 2*o<sub>j</sub>*, plus one if the first child is the
///   leaf of a key equal to the extent of the node, and it ends with twice the
///   overall number of children;
/// - `rights` contains the rank at which the node ends;
/// - `sizes` contains the prefix sums of the number of keys below each child,
///   so that the rank at which a child starts can be recovered from `rights`.
///
/// All three sequences are monotone, so each of them can be stored in any
/// [`IndexedSeq`] returning `usize`: for example, a boxed slice, or the
/// [Elias–Fano representation] of [`sux`] with the selection structure of
/// choice (see [`EfDirs`]). The bytes associated with the children are stored
/// in a backend that must be viewable as a slice of bytes.
///
/// Instances are obtained by conversion from a [`PackedDirs`], and the
/// backends can then be replaced using the mapping methods.
///
/// [Elias–Fano representation]: sux::dict::EliasFano
#[derive(Debug, Clone, MemSize, MemDbg)]
#[cfg_attr(feature = "epserde", derive(epserde::Epserde))]
pub struct SeqDirs<O = Box<[usize]>, P = Box<[usize]>, R = Box<[usize]>, Y = Box<[u8]>> {
    offsets: O,
    sizes: P,
    rights: R,
    syms: Y,
}

/// [`SeqDirs`] using the [Elias–Fano representation] of [`sux`] with its
/// default selection structure.
///
/// [Elias–Fano representation]: sux::dict::EliasFano
pub type EfDirs = SeqDirs<EfSeq, EfSeq, EfSeq, Box<[u8]>>;

/// The [directory of a node](DirNode) of a [`SeqDirs`].
#[derive(Debug, Clone, Copy)]
pub struct SeqNode<'a, P> {
    sizes: &'a P,
    /// The index of the first child in `sizes`.
    first: usize,
    /// The value of `sizes` at `first`.
    base: usize,
    left: usize,
    right: usize,
    /// The bytes associated with the children.
    syms: &'a [u8],
    /// Whether the first child is the leaf of a key equal to the extent.
    has_end: bool,
}

impl<P: IndexedSeq> DirNode for SeqNode<'_, P>
where
    for<'a> P: Types<Output<'a> = usize>,
{
    #[inline(always)]
    fn left(&self) -> usize {
        self.left
    }

    #[inline(always)]
    fn right(&self) -> usize {
        self.right
    }

    #[inline(always)]
    fn child(&self, c: u8) -> (usize, bool) {
        let (j, found) = position(self.syms, c);
        (self.has_end as usize + j, found)
    }

    #[inline(always)]
    fn start(&self, index: usize) -> usize {
        self.left + self.sizes.get(self.first + index) - self.base
    }

    #[inline(always)]
    fn key<'k, K: KeySeq>(&self, keys: &'k K, index: usize) -> K::Key<'k> {
        keys.key(self.start(index))
    }
}

impl<O: IndexedSeq, P: IndexedSeq, R: IndexedSeq, Y: AsRef<[u8]>> Dirs for SeqDirs<O, P, R, Y>
where
    for<'a> O: Types<Output<'a> = usize>,
    for<'a> P: Types<Output<'a> = usize>,
    for<'a> R: Types<Output<'a> = usize>,
{
    type Node<'a>
        = SeqNode<'a, P>
    where
        Self: 'a;

    #[inline(always)]
    fn node(&self, id: usize) -> SeqNode<'_, P> {
        let (offset, next) = (self.offsets.get(id), self.offsets.get(id + 1));
        let has_end = offset & 1 != 0;
        let (first, end) = (offset / 2, next / 2);
        let (base, right) = (self.sizes.get(first), self.rights.get(id));
        SeqNode {
            sizes: &self.sizes,
            first,
            base,
            left: right - (self.sizes.get(end) - base),
            right,
            syms: &self.syms.as_ref()[first + has_end as usize..end],
            has_end,
        }
    }
}

impl SeqDirs {
    /// Creates a new instance containing the same directories as the given
    /// [`PackedDirs`], returning also the identifiers of the nodes of the
    /// latter, in increasing order: the identifier of a node in the new
    /// instance is the index of its old identifier.
    pub fn from_packed(dirs: &PackedDirs<impl AsRef<[u8]>>) -> (Self, Vec<usize>) {
        let ids: Vec<usize> = dirs.ids().collect();
        let (mut offsets, mut sizes, mut rights, mut syms) = (vec![], vec![0], vec![], vec![]);
        let mut sum = 0;
        for &id in &ids {
            let node = dirs.node(id);
            offsets.push(2 * syms.len() + node.has_end as usize);
            rights.push(node.right());
            for i in 0..node.num_children() {
                sum += node.start(i + 1) - node.start(i);
                sizes.push(sum);
                syms.push(node.sym(i).unwrap_or(0));
            }
        }
        offsets.push(2 * syms.len());
        (
            SeqDirs {
                offsets: offsets.into_boxed_slice(),
                sizes: sizes.into_boxed_slice(),
                rights: rights.into_boxed_slice(),
                syms: syms.into_boxed_slice(),
            },
            ids,
        )
    }

    /// Replaces the three sequences with their [Elias–Fano representation].
    ///
    /// [Elias–Fano representation]: sux::dict::EliasFano
    pub fn into_ef(self) -> EfDirs {
        fn ef(values: &[usize]) -> EfSeq {
            let mut builder =
                EliasFanoBuilder::new(values.len(), values.last().copied().unwrap_or(0));
            for &v in values {
                builder.push(v);
            }
            builder.build_with_seq()
        }
        SeqDirs {
            offsets: ef(&self.offsets),
            sizes: ef(&self.sizes),
            rights: ef(&self.rights),
            syms: self.syms,
        }
    }
}

impl<O, P, R, Y> SeqDirs<O, P, R, Y> {
    /// Replaces the backend of the sequence of offsets.
    ///
    /// # Safety
    ///
    /// The new backend must return the same values as the old one.
    pub unsafe fn map_offsets<O2>(self, f: impl FnOnce(O) -> O2) -> SeqDirs<O2, P, R, Y> {
        SeqDirs {
            offsets: f(self.offsets),
            sizes: self.sizes,
            rights: self.rights,
            syms: self.syms,
        }
    }

    /// Replaces the backend of the sequence of sizes.
    ///
    /// # Safety
    ///
    /// The new backend must return the same values as the old one.
    pub unsafe fn map_sizes<P2>(self, f: impl FnOnce(P) -> P2) -> SeqDirs<O, P2, R, Y> {
        SeqDirs {
            offsets: self.offsets,
            sizes: f(self.sizes),
            rights: self.rights,
            syms: self.syms,
        }
    }

    /// Replaces the backend of the sequence of right extremes.
    ///
    /// # Safety
    ///
    /// The new backend must return the same values as the old one.
    pub unsafe fn map_rights<R2>(self, f: impl FnOnce(R) -> R2) -> SeqDirs<O, P, R2, Y> {
        SeqDirs {
            offsets: self.offsets,
            sizes: self.sizes,
            rights: f(self.rights),
            syms: self.syms,
        }
    }

    /// Returns the three sequences and the bytes associated with the
    /// children.
    pub fn into_raw_parts(self) -> (O, P, R, Y) {
        (self.offsets, self.sizes, self.rights, self.syms)
    }
}
