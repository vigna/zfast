/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

//! A static z-fast trie on a sorted sequence of byte strings.

use crate::keys::KeySeq;
use crate::utils::{HashPrefixes, Xxh3, fat_binary_search, lcp, prefetch, two_fattest};
use mem_dbg::{MemDbg, MemSize};
use std::borrow::Borrow;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Range;
use sux::traits::{IndexedDict, IndexedSeq, Pred, PredUnchecked, Succ, SuccUnchecked, Types};

/// Marks an empty slot (in the `off` field).
const EMPTY: u32 = u32::MAX;
/// The default seed.
const SEED: u64 = 0x05EE_D0F2_FA57;

/// A slot of the z-map: it is self-contained, in the sense that fat binary
/// search never needs to access anything else.
#[derive(Clone, Copy, Debug, MemSize, MemDbg)]
#[mem_size(flat)]
#[repr(C)]
struct Slot {
    /// The signature of the handle.
    sig: u32,
    /// The length of the handle.
    handle: u32,
    /// The length of the extent.
    extent: u32,
    /// The offset of the directory of the node, or [`EMPTY`].
    off: u32,
}

/// The errors that can happen when building a [`StaticZFastTrie`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// The keys are not in strictly increasing lexicographic order: the key at
    /// the given index is not larger than the previous one.
    Unsorted(usize),
    /// There are too many keys, or too many bytes in a key.
    TooLarge,
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::Unsorted(i) => write!(
                f,
                "keys are not strictly increasing: key {i} is not larger than key {}",
                i - 1
            ),
            BuildError::TooLarge => write!(f, "too many keys, or key too long"),
        }
    }
}

impl std::error::Error for BuildError {}

/// A static z-fast trie on a sorted sequence of byte strings.
///
/// The structure indexes any [sequence of byte strings](KeySeq) in strictly
/// increasing lexicographic order (the order of `[u8]`), and answers in a
/// number of steps that is logarithmic in the *length of the query* (and
/// independent of the number of keys):
/// - [rank](Self::rank) queries, and thus [predecessor](Self::pred) and
///   [successor](Self::succ) queries;
/// - [membership](Self::index_of) queries;
/// - [prefix](Self::prefix_range) queries, which return the range of keys
///   that have a given prefix.
///
/// Conceptually, the structure is the compacted trie of the keys, with
/// branching on bytes: a key that is a prefix of another key is handled by
/// imagining that each key is followed by a terminator smaller than all bytes.
/// Each internal node of the trie is represented by the interval of
/// ranks of the keys in its subtree; the *z-map* is a hash table mapping the
/// signature of the *handle* of each internal node (a suitably chosen prefix of
/// the longest common prefix of the keys in the subtree) to the node. A *fat
/// binary search* on the length of the query, using only the z-map, locates the
/// node at which the query exits the trie, or its parent; a single comparison
/// with a key verifies the result, as signatures can return false positives,
/// and completes the query. In the very unlikely case the verification fails,
/// the query is answered by a standard binary search.
///
/// The structure uses about 20 bytes per key, depending on the shape of the
/// trie, besides the space used by the keys.
///
/// The type parameter `K` is the type of the [sequence of keys](KeySeq): any
/// [`IndexedSeq`] whose output can be seen as a slice of bytes will do. The
/// type parameter `H` is the [strategy used to hash prefixes](HashPrefixes).
///
/// Besides the inherent methods, which accept anything that can be seen as a
/// slice of bytes, this structure implements the [`IndexedSeq`],
/// [`IndexedDict`], [`Succ`] and [`Pred`] traits of [`sux`] whenever the
/// underlying sequence of keys makes it possible.
///
/// # Examples
///
/// ```
/// # use zfast::{PackedKeys, StaticZFastTrie};
/// let keys: PackedKeys = ["bar", "baz", "foo", "foobar"].into_iter().collect();
/// let trie = <StaticZFastTrie<_>>::new(keys)?;
/// assert_eq!(trie.index_of("baz"), Some(1));
/// assert_eq!(trie.index_of("ba"), None);
/// assert_eq!(trie.rank("c"), 2);
/// assert_eq!(trie.succ("c"), Some((2, "foo".as_bytes())));
/// assert_eq!(trie.pred("c"), Some((1, "baz".as_bytes())));
/// assert_eq!(trie.prefix_range("ba"), 0..2);
/// assert_eq!(trie.prefix_range("foo"), 2..4);
/// # Ok::<(), zfast::BuildError>(())
/// ```
#[derive(Debug, Clone, MemSize, MemDbg)]
pub struct StaticZFastTrie<K, H = Xxh3> {
    /// The keys.
    keys: K,
    /// The z-map: an open-addressing table with linear probing.
    slots: Box<[Slot]>,
    /// The directories of the internal nodes. The directory of a node with
    /// *k* children is formed by a header (*k* − 2 in the lower 8 bits, and
    /// then a bit that is set if the first child is the leaf of a key equal
    /// to the extent of the node), followed by the ranks at which each child
    /// starts, followed by the rank at which the node ends, followed by the
    /// bytes associated with the children (except for the first one, if the
    /// bit above is set).
    dirs: Box<[u8]>,
    /// The root, or an empty slot if there are fewer than two keys.
    root: Slot,
    /// The extent of the root.
    root_extent: Box<[u8]>,
    /// The maximum length of the extent of an internal node.
    max_extent: usize,
    /// The number of internal nodes.
    num_nodes: usize,
    /// The seed of the hash function.
    seed: u64,
    /// A mask applied to signatures (used for testing only).
    sig_mask: u32,
    _marker: PhantomData<H>,
}

/// The result of a query: the number of keys smaller than the query, whether
/// the query is a key, and the interval of ranks of the keys that have the
/// query as a prefix.
type Located = (usize, bool, Range<usize>);

/// A decoded directory.
#[derive(Clone, Copy)]
struct Dir<'a> {
    /// The ranks at which each child starts, followed by the rank at which
    /// the node ends, as little-endian 32-bit integers.
    starts: &'a [u8],
    /// The bytes associated with the children.
    syms: &'a [u8],
    /// Whether the first child is the leaf of a key equal to the extent.
    has_end: bool,
}

impl Dir<'_> {
    #[inline(always)]
    fn start(&self, i: usize) -> usize {
        u32::from_le_bytes(self.starts[4 * i..4 * i + 4].try_into().unwrap()) as usize
    }

    /// The first rank of the node.
    #[inline(always)]
    fn left(&self) -> usize {
        self.start(0)
    }

    /// The first rank after the node.
    #[inline(always)]
    fn right(&self) -> usize {
        self.start(self.starts.len() / 4 - 1)
    }

    /// Returns the index of the child associated with the byte `c`, if there
    /// is one, or otherwise the index of the first child associated with a
    /// larger byte, and whether the child was found.
    #[inline(always)]
    fn child(&self, c: u8) -> (usize, bool) {
        let j = self.syms.iter().filter(|&&s| s < c).count();
        (
            self.has_end as usize + j,
            j < self.syms.len() && self.syms[j] == c,
        )
    }
}

impl<K: KeySeq, H: HashPrefixes> StaticZFastTrie<K, H> {
    /// The default load factor of the z-map.
    pub const DEFAULT_LOAD_FACTOR: f64 = 0.6;

    /// Creates a new static z-fast trie for the given keys using the
    /// [default load factor](Self::DEFAULT_LOAD_FACTOR).
    ///
    /// # Errors
    ///
    /// This method will return an error if the keys are not in strictly
    /// increasing lexicographic order, or if there are more than 2³² − 2 keys
    /// or a key is longer than 2³¹ − 2 bytes.
    pub fn new(keys: K) -> Result<Self, BuildError> {
        Self::with_load_factor(keys, Self::DEFAULT_LOAD_FACTOR)
    }

    /// Creates a new static z-fast trie for the given keys using the given
    /// load factor for the z-map.
    ///
    /// Smaller load factors make queries faster and the structure larger.
    ///
    /// # Errors
    ///
    /// See [`new`](Self::new).
    ///
    /// # Panics
    ///
    /// If the load factor is not in the interval (0 . . 1).
    pub fn with_load_factor(keys: K, load_factor: f64) -> Result<Self, BuildError> {
        Self::build(keys, load_factor, u32::MAX)
    }

    /// Creates a new static z-fast trie whose signatures have the given
    /// number of bits.
    ///
    /// This method is useful only to test the behavior of the structure in
    /// the presence of signature collisions.
    #[doc(hidden)]
    pub fn with_sig_bits(keys: K, sig_bits: u32) -> Result<Self, BuildError> {
        assert!(sig_bits <= 32);
        let sig_mask = if sig_bits == 32 {
            u32::MAX
        } else {
            (1 << sig_bits) - 1
        };
        Self::build(keys, Self::DEFAULT_LOAD_FACTOR, sig_mask)
    }

    fn build(keys: K, load_factor: f64, sig_mask: u32) -> Result<Self, BuildError> {
        assert!(load_factor > 0.0 && load_factor < 1.0);
        let n = keys.num_keys();
        if n >= EMPTY as usize {
            return Err(BuildError::TooLarge);
        }
        let seed = SEED;
        let mut dirs = Vec::<u8>::new();
        let mut entries = Vec::<(u64, Slot)>::new();
        let mut root_extent = Box::<[u8]>::default();
        let mut root = Slot {
            sig: 0,
            handle: 0,
            extent: 0,
            off: EMPTY,
        };
        let mut max_extent = 0;

        // An open node: the length of its extent, its first rank, and the
        // ranks at which its children start.
        struct Open {
            extent: usize,
            left: usize,
            starts: Vec<u32>,
        }
        let mut stack = Vec::<Open>::new();
        let mut spare = Vec::<Vec<u32>>::new();

        for i in 1..=n {
            // The length of the longest common prefix of key i - 1 and key i
            let cur = if i < n {
                let (prev, next) = (keys.key(i - 1), keys.key(i));
                let (prev, next) = (prev.as_ref(), next.as_ref());
                if next.len() >= (1 << 31) - 1 {
                    return Err(BuildError::TooLarge);
                }
                let l = lcp(prev, next);
                if l == next.len() || l < prev.len() && prev[l] > next[l] {
                    return Err(BuildError::Unsorted(i));
                }
                Some(l)
            } else {
                None
            };
            let mut left = i - 1;
            // Close the nodes whose extent is longer than the current
            // longest common prefix
            while stack.last().is_some_and(|top| Some(top.extent) > cur) {
                let node = stack.pop().unwrap();
                left = node.left;
                // The extent of the parent of the node, if any
                let parent = stack.last().map(|top| top.extent).max(cur);
                let off = dirs.len();
                if off >= EMPTY as usize {
                    return Err(BuildError::TooLarge);
                }
                let k = node.starts.len();
                debug_assert!((2..=257).contains(&k));
                let first = keys.key(node.left);
                let first = first.as_ref();
                let has_end = first.len() == node.extent;
                dirs.extend_from_slice(&((k - 2) as u32 | (has_end as u32) << 8).to_le_bytes());
                for &s in &node.starts {
                    dirs.extend_from_slice(&s.to_le_bytes());
                }
                dirs.extend_from_slice(&(i as u32).to_le_bytes());
                for &s in &node.starts[has_end as usize..] {
                    dirs.push(keys.key(s as usize).as_ref()[node.extent]);
                }
                max_extent = max_extent.max(node.extent);
                match parent {
                    None => {
                        root = Slot {
                            sig: 0,
                            handle: 0,
                            extent: node.extent as u32,
                            off: off as u32,
                        };
                        root_extent = first[..node.extent].into();
                    }
                    Some(parent) => {
                        let handle = two_fattest(parent, node.extent);
                        let h = H::hash_prefix(first, handle, seed);
                        entries.push((
                            h,
                            Slot {
                                sig: h as u32 & sig_mask,
                                handle: handle as u32,
                                extent: node.extent as u32,
                                off: off as u32,
                            },
                        ));
                    }
                }
                let mut starts = node.starts;
                starts.clear();
                spare.push(starts);
            }
            if let Some(cur) = cur {
                match stack.last_mut() {
                    Some(top) if top.extent == cur => top.starts.push(i as u32),
                    _ => {
                        let mut starts = spare.pop().unwrap_or_default();
                        starts.push(left as u32);
                        starts.push(i as u32);
                        stack.push(Open {
                            extent: cur,
                            left,
                            starts,
                        });
                    }
                }
            }
        }
        debug_assert!(stack.is_empty());

        let num_nodes = entries.len() + (root.off != EMPTY) as usize;
        let capacity = ((entries.len() as f64 / load_factor).ceil() as usize).max(1);
        if capacity >= EMPTY as usize {
            return Err(BuildError::TooLarge);
        }
        let mut slots = vec![
            Slot {
                sig: 0,
                handle: 0,
                extent: 0,
                off: EMPTY
            };
            capacity
        ]
        .into_boxed_slice();
        for (h, entry) in entries {
            let mut pos = Self::home(h, sig_mask, capacity);
            while slots[pos].off != EMPTY {
                pos += 1;
                if pos == capacity {
                    pos = 0;
                }
            }
            slots[pos] = entry;
        }
        Ok(Self {
            keys,
            slots,
            dirs: dirs.into_boxed_slice(),
            root,
            root_extent,
            max_extent,
            num_nodes,
            seed,
            sig_mask,
            _marker: PhantomData,
        })
    }

    /// Returns the position in a table of given capacity at which the
    /// probing sequence of a hash starts.
    #[inline(always)]
    fn home(h: u64, sig_mask: u32, capacity: usize) -> usize {
        // When testing with short signatures we want long clusters with
        // many repeated signatures.
        let h = if sig_mask == u32::MAX {
            h >> 32
        } else {
            h & sig_mask as u64
        };
        ((h * capacity as u64) >> 32) as usize
    }

    #[inline(always)]
    fn dir(&self, off: u32) -> Dir<'_> {
        let off = off as usize;
        let header = u32::from_le_bytes(self.dirs[off..off + 4].try_into().unwrap());
        let k = (header & 0xFF) as usize + 2;
        let has_end = header & 0x100 != 0;
        let starts = off + 4;
        let syms = starts + 4 * (k + 1);
        Dir {
            starts: &self.dirs[starts..syms],
            syms: &self.dirs[syms..syms + k - has_end as usize],
            has_end,
        }
    }

    /// Returns the slot associated with a handle of length `f` and hash `h`,
    /// if any.
    #[inline(always)]
    fn find(&self, h: u64, f: usize) -> Option<Slot> {
        let sig = h as u32 & self.sig_mask;
        let capacity = self.slots.len();
        let mut pos = Self::home(h, self.sig_mask, capacity);
        loop {
            let slot = self.slots[pos];
            if slot.off == EMPTY {
                return None;
            }
            if slot.sig == sig && slot.handle as usize == f {
                return Some(slot);
            }
            pos += 1;
            if pos == capacity {
                pos = 0;
            }
        }
    }

    /// Performs a fat binary search for `x`, whose first `self.root.extent`
    /// bytes must be the extent of the root, returning the last slot that
    /// was hit, or the root.
    ///
    /// If there are no false positives, the result is either the exit node
    /// of `x`, or its parent.
    #[inline(always)]
    fn fat_binary_search(&self, x: &[u8]) -> Slot {
        let mut top = self.root;
        let a = top.extent as usize;
        let b = x.len().min(self.max_extent);
        if a < b {
            let hasher = H::hasher(x, b, self.seed);
            let capacity = self.slots.len();
            fat_binary_search(
                &hasher,
                a,
                b,
                |h| {
                    prefetch(self.slots.as_ptr().wrapping_add(Self::home(
                        h,
                        self.sig_mask,
                        capacity,
                    )))
                },
                |f, h| {
                    self.find(h, f).map(|slot| {
                        top = slot;
                        slot.extent as usize
                    })
                },
            );
        }
        top
    }

    /// Returns the number of keys smaller than `x`, whether `x` is a key,
    /// and the interval of ranks of the keys that have `x` as a prefix.
    ///
    /// This is the method on which all queries are based.
    #[inline]
    fn locate(&self, x: &[u8]) -> Located {
        let n = self.keys.num_keys();
        if n < 2 {
            return self.locate_slow(x);
        }
        let l = lcp(x, &self.root_extent);
        if l < self.root_extent.len() {
            // The query exits at the root
            return Self::exit(x, &self.root_extent, l, 0..n);
        }
        let top = self.fat_binary_search(x);
        let (extent, handle) = (top.extent as usize, top.handle as usize);
        let dir = self.dir(top.off);

        let range = if extent < x.len() {
            // We guess that top is the parent of the exit node, and thus
            // that the exit node is one of its children
            let (i, found) = dir.child(x[extent]);
            if found {
                dir.start(i)..dir.start(i + 1)
            } else {
                // If we guessed right, x exits at top between two children
                let key = self.keys.key(dir.left());
                let key = key.as_ref();
                let l = lcp(x, key);
                return if l >= extent {
                    (dir.start(i), false, dir.start(i)..dir.start(i))
                } else if l >= handle {
                    // top is the exit node
                    Self::exit(x, key, l, dir.left()..dir.right())
                } else {
                    self.locate_slow(x)
                };
            }
        } else {
            // Either x is the extent of top, or top can only be the exit
            // node
            dir.left()..dir.right()
        };

        // In all cases, the first key of the range starts with the extent
        // of top
        let key = self.keys.key(range.start);
        let key = key.as_ref();
        let l = lcp(x, key);
        if l >= extent {
            // If extent > x.len() we cannot get here. Thus, either x is the
            // extent of top, or we guessed right; in both cases, range is
            // the interval of the keys starting with the first l bytes of x
            if l == key.len() && l < x.len() && range.len() > 1 {
                // This cannot happen if top is the parent of the exit node
                debug_assert!(self.sig_mask != u32::MAX);
                return self.locate_slow(x);
            }
            Self::exit(x, key, l, range)
        } else if l >= handle {
            // top is the exit node
            Self::exit(x, key, l, dir.left()..dir.right())
        } else {
            // A false positive misled the search
            self.locate_slow(x)
        }
    }

    /// Completes a query given the interval of ranks of the exit node of `x`,
    /// a key in the interval, and the length of the longest common prefix of
    /// `x` and the key.
    #[inline(always)]
    fn exit(x: &[u8], key: &[u8], l: usize, range: Range<usize>) -> Located {
        if l == x.len() {
            // x is a prefix of all keys in the range
            (range.start, l == key.len(), range)
        } else if l < key.len() && x[l] < key[l] {
            (range.start, false, range.start..range.start)
        } else {
            (range.end, false, range.end..range.end)
        }
    }

    /// A version of [`locate`](Self::locate) based on binary search.
    #[cold]
    #[inline(never)]
    fn locate_slow(&self, x: &[u8]) -> Located {
        let n = self.keys.num_keys();
        let lower = |x: &[u8], strict: bool| {
            let (mut lo, mut hi) = (0, n);
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                let key = self.keys.key(mid);
                let key = key.as_ref();
                // In the strict case we look for the first key that is larger
                // than x and does not have x as a prefix
                let less = if strict {
                    key <= x || key.starts_with(x)
                } else {
                    key < x
                };
                if less {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            lo
        };
        let rank = lower(x, false);
        let end = lower(x, true);
        (
            rank,
            rank < n && self.keys.key(rank).as_ref() == x,
            rank..end,
        )
    }

    /// Returns the number of keys.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.keys.num_keys()
    }

    /// Returns true if there are no keys.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.keys.num_keys() == 0
    }

    /// Returns the key of given rank.
    ///
    /// # Panics
    ///
    /// May panic if the rank is not smaller than the number of keys.
    #[inline(always)]
    pub fn key(&self, rank: usize) -> K::Key<'_> {
        self.keys.key(rank)
    }

    /// Returns a reference to the underlying sequence of keys.
    #[inline(always)]
    pub fn keys(&self) -> &K {
        &self.keys
    }

    /// Consumes the structure and returns the underlying sequence of keys.
    pub fn into_inner(self) -> K {
        self.keys
    }

    /// Returns the number of keys smaller than `x`.
    #[inline]
    pub fn rank(&self, x: impl AsRef<[u8]>) -> usize {
        self.locate(x.as_ref()).0
    }

    /// Returns the rank of `x` among the keys, if `x` is a key.
    #[inline]
    pub fn index_of(&self, x: impl AsRef<[u8]>) -> Option<usize> {
        let (rank, found, _) = self.locate(x.as_ref());
        found.then_some(rank)
    }

    /// Returns true if `x` is a key.
    #[inline]
    pub fn contains(&self, x: impl AsRef<[u8]>) -> bool {
        self.locate(x.as_ref()).1
    }

    /// Returns the rank and the value of the smallest key that is larger
    /// than or equal to `x`, if any.
    #[inline]
    pub fn succ(&self, x: impl AsRef<[u8]>) -> Option<(usize, K::Key<'_>)> {
        let (rank, _, _) = self.locate(x.as_ref());
        (rank < self.keys.num_keys()).then(|| (rank, self.keys.key(rank)))
    }

    /// Returns the rank and the value of the smallest key that is larger
    /// than `x`, if any.
    #[inline]
    pub fn succ_strict(&self, x: impl AsRef<[u8]>) -> Option<(usize, K::Key<'_>)> {
        let (rank, found, _) = self.locate(x.as_ref());
        let rank = rank + found as usize;
        (rank < self.keys.num_keys()).then(|| (rank, self.keys.key(rank)))
    }

    /// Returns the rank and the value of the largest key that is smaller
    /// than or equal to `x`, if any.
    #[inline]
    pub fn pred(&self, x: impl AsRef<[u8]>) -> Option<(usize, K::Key<'_>)> {
        let (rank, found, _) = self.locate(x.as_ref());
        let rank = rank + found as usize;
        (rank > 0).then(|| (rank - 1, self.keys.key(rank - 1)))
    }

    /// Returns the rank and the value of the largest key that is smaller
    /// than `x`, if any.
    #[inline]
    pub fn pred_strict(&self, x: impl AsRef<[u8]>) -> Option<(usize, K::Key<'_>)> {
        let (rank, _, _) = self.locate(x.as_ref());
        (rank > 0).then(|| (rank - 1, self.keys.key(rank - 1)))
    }

    /// Returns the range of ranks of the keys that have `x` as a prefix.
    ///
    /// If no key has `x` as a prefix, the range is empty, and it starts at the
    /// [rank](Self::rank) of `x`.
    #[inline]
    pub fn prefix_range(&self, x: impl AsRef<[u8]>) -> Range<usize> {
        self.locate(x.as_ref()).2
    }

    /// Returns the number of internal nodes of the trie.
    pub fn num_nodes(&self) -> usize {
        self.num_nodes
    }

    /// Returns the number of probes of the z-map, the number of slots
    /// examined, and the number of successful probes of a fat binary search
    /// for `x`.
    ///
    /// This method is useful only to analyze the behavior of the structure.
    #[doc(hidden)]
    pub fn search_stats(&self, x: impl AsRef<[u8]>) -> (usize, usize, usize) {
        let x = x.as_ref();
        let (mut probes, mut scanned, mut hits) = (0, 0, 0);
        if self.keys.num_keys() < 2 || lcp(x, &self.root_extent) < self.root_extent.len() {
            return (0, 0, 0);
        }
        let mut a = self.root.extent as usize;
        let mut b = x.len().min(self.max_extent);
        let capacity = self.slots.len();
        while a < b {
            let f = two_fattest(a, b);
            let h = H::hash_prefix(x, f, self.seed);
            let sig = h as u32 & self.sig_mask;
            let mut pos = Self::home(h, self.sig_mask, capacity);
            probes += 1;
            let found = loop {
                let slot = self.slots[pos];
                scanned += 1;
                if slot.off == EMPTY {
                    break None;
                }
                if slot.sig == sig && slot.handle as usize == f {
                    break Some(slot);
                }
                pos += 1;
                if pos == capacity {
                    pos = 0;
                }
            };
            match found {
                Some(slot) => {
                    hits += 1;
                    a = slot.extent as usize;
                }
                None => b = f - 1,
            }
        }
        (probes, scanned, hits)
    }

    /// Returns the number of bytes occupied by this structure, not including
    /// the keys.
    pub fn index_mem_size(&self) -> usize {
        size_of::<Self>() - size_of::<K>()
            + size_of_val(&*self.slots)
            + self.dirs.len()
            + self.root_extent.len()
    }
}

impl<K: KeySeq, H: HashPrefixes> PartialEq for StaticZFastTrie<K, H> {
    /// Two static z-fast tries are equal if they contain the same keys.
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len()
            && (0..self.len()).all(|i| self.key(i).as_ref() == other.key(i).as_ref())
    }
}

impl<K: KeySeq, H: HashPrefixes> Eq for StaticZFastTrie<K, H> {}

// Traits from sux

impl<K: Types, H> Types for StaticZFastTrie<K, H> {
    type Input = K::Input;
    type Output<'a> = K::Output<'a>;
}

impl<K: IndexedSeq, H> IndexedSeq for StaticZFastTrie<K, H> {
    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> Self::Output<'_> {
        unsafe { self.keys.get_unchecked(index) }
    }

    #[inline(always)]
    fn len(&self) -> usize {
        IndexedSeq::len(&self.keys)
    }
}

impl<K: IndexedSeq, H: HashPrefixes> IndexedDict for StaticZFastTrie<K, H>
where
    K::Input: AsRef<[u8]>,
    for<'a> K::Output<'a>: AsRef<[u8]>,
{
    #[inline]
    fn index_of(&self, value: impl Borrow<Self::Input>) -> Option<usize> {
        let (rank, found, _) = self.locate(value.borrow().as_ref());
        found.then_some(rank)
    }
}

impl<K: IndexedSeq, H: HashPrefixes> SuccUnchecked for StaticZFastTrie<K, H>
where
    K::Input: AsRef<[u8]> + for<'a> PartialOrd<K::Output<'a>> + PartialOrd,
    for<'a> K::Output<'a>: AsRef<[u8]> + PartialOrd<K::Input> + PartialOrd,
{
    #[inline]
    unsafe fn succ_unchecked<const STRICT: bool>(
        &self,
        value: impl Borrow<Self::Input>,
    ) -> (usize, Self::Output<'_>) {
        let (rank, found, _) = self.locate(value.borrow().as_ref());
        let rank = rank + (STRICT && found) as usize;
        (rank, unsafe { self.keys.get_unchecked(rank) })
    }
}

impl<K: IndexedSeq, H: HashPrefixes> Succ for StaticZFastTrie<K, H>
where
    K::Input: AsRef<[u8]> + for<'a> PartialOrd<K::Output<'a>> + PartialOrd,
    for<'a> K::Output<'a>: AsRef<[u8]> + PartialOrd<K::Input> + PartialOrd,
{
    #[inline]
    fn succ(&self, value: impl Borrow<Self::Input>) -> Option<(usize, Self::Output<'_>)> {
        StaticZFastTrie::succ(self, value.borrow())
    }

    #[inline]
    fn succ_strict(&self, value: impl Borrow<Self::Input>) -> Option<(usize, Self::Output<'_>)> {
        StaticZFastTrie::succ_strict(self, value.borrow())
    }
}

impl<K: IndexedSeq, H: HashPrefixes> PredUnchecked for StaticZFastTrie<K, H>
where
    K::Input: AsRef<[u8]> + for<'a> PartialOrd<K::Output<'a>> + PartialOrd,
    for<'a> K::Output<'a>: AsRef<[u8]> + PartialOrd<K::Input> + PartialOrd,
{
    #[inline]
    unsafe fn pred_unchecked<const STRICT: bool>(
        &self,
        value: impl Borrow<Self::Input>,
    ) -> (usize, Self::Output<'_>) {
        let (rank, found, _) = self.locate(value.borrow().as_ref());
        let rank = rank + (!STRICT && found) as usize - 1;
        (rank, unsafe { self.keys.get_unchecked(rank) })
    }

    #[inline]
    unsafe fn rank_unchecked(&self, value: impl Borrow<Self::Input>) -> usize {
        self.locate(value.borrow().as_ref()).0
    }
}

impl<K: IndexedSeq, H: HashPrefixes> Pred for StaticZFastTrie<K, H>
where
    K::Input: AsRef<[u8]> + for<'a> PartialOrd<K::Output<'a>> + PartialOrd,
    for<'a> K::Output<'a>: AsRef<[u8]> + PartialOrd<K::Input> + PartialOrd,
{
    #[inline]
    fn pred(&self, value: impl Borrow<Self::Input>) -> Option<(usize, Self::Output<'_>)> {
        StaticZFastTrie::pred(self, value.borrow())
    }

    #[inline]
    fn pred_strict(&self, value: impl Borrow<Self::Input>) -> Option<(usize, Self::Output<'_>)> {
        StaticZFastTrie::pred_strict(self, value.borrow())
    }

    #[inline]
    fn rank(&self, value: impl Borrow<Self::Input>) -> usize {
        self.locate(value.borrow().as_ref()).0
    }
}
