/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! The router of a [paged set](super::ZFastPagedSet): a z-fast trie on the
//! lower fences of the segments.
//!
//! The router is a compacted trie whose leaves are the segments (below
//! called *pages*, for historical reasons: the router does not depend on
//! what its leaves are), endowed with a z-map from the signatures of the
//! handles of the internal nodes to the nodes. With respect to the [dynamic
//! z-fast trie](crate::ZFastTrie), there are three differences, all due to
//! the fact that the router is modified only when a segment is created or
//! deleted, whereas it is queried by every operation:
//!
//! - The router does not store strings: the extent of a node is a prefix of
//!   the lower fence of each segment below the node, and that is where we
//!   read it when we update the trie. In particular, *queries never access
//!   the bytes of an extent*: a fat binary search based on signatures
//!   provides a node, from which we derive at most two candidate segments,
//!   and it is the segment that tells us, using its fences, whether it is
//!   the right one. If no candidate is, we fall back to an exact search.
//!
//! - Each internal node stores the first and the last leaf of its subtree,
//!   and the leaf preceding the first one, instead of jump pointers: thus,
//!   the candidates are known as soon as the node is known. These fields are
//!   updated by walking up the trie.
//!
//! - Besides the signature of its handle (in the z-map), each node stores a
//!   signature of its extent, which makes it possible to guess whether the
//!   extent is a prefix of the string we are looking for.

use super::page::NONE;
use crate::dynamic::map::HandleMap;
use crate::utils::{HashPrefixes, PrefixHasher, fat_binary_search, lcp, lcp_from, two_fattest};
use std::marker::PhantomData;

/// The bit marking the identifier of a leaf (i.e., of a page) among the
/// children of a node.
const LEAF: u32 = 1 << 31;
/// The number of children that are stored in an internal node.
const INLINE: usize = 6;
/// The value of the `len` field of [`Kids`] for nodes whose children are
/// stored elsewhere.
const BIG: u8 = u8::MAX;
/// The default seed.
const SEED: u64 = 0x05EE_D0F2_FA57;

/// A source of the strings associated with the leaves of the router.
pub(super) trait Anchors {
    /// Returns the string associated with a leaf.
    fn low(&self, id: u32) -> &[u8];
}

#[inline(always)]
const fn is_leaf(id: u32) -> bool {
    id & LEAF != 0
}

/// A symbol: zero for the (virtual) terminator that follows every string, and
/// *b* + 1 for the byte *b*.
type Sym = u16;

/// Returns the symbol in position `i` of the string `x` followed by the
/// terminator.
#[inline(always)]
fn sym_at(x: &[u8], i: usize) -> Sym {
    debug_assert!(i <= x.len());
    if i < x.len() { x[i] as Sym + 1 } else { 0 }
}

/// The children of an internal node, in order: up to [`INLINE`] children are
/// stored in the node, and larger sets are stored in a separate [`BigKids`]
/// instance.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct Kids {
    /// The identifiers of the children; if `len` is [`BIG`], the first
    /// element is the index of a [`BigKids`] instance instead.
    ids: [u32; INLINE],
    /// The byte associated with each child; the value is immaterial for a
    /// child associated with the terminator.
    bytes: [u8; INLINE],
    /// The number of children, or [`BIG`].
    len: u8,
    /// Whether the first child is associated with the terminator.
    has_end: bool,
}

/// The children of an internal node with more than [`INLINE`] children.
#[derive(Clone, Debug, Default)]
struct BigKids {
    ids: Vec<u32>,
    bytes: Vec<u8>,
    has_end: bool,
}

/// A view of the children of an internal node.
#[derive(Clone, Copy)]
struct KidView<'a> {
    ids: &'a [u32],
    bytes: &'a [u8],
    has_end: bool,
}

impl KidView<'_> {
    /// Returns the index of the child associated with the given symbol, if
    /// there is one, or otherwise the index of the first child associated
    /// with a larger symbol, and whether the child was found.
    #[inline(always)]
    fn position(&self, sym: Sym) -> (usize, bool) {
        if sym == 0 {
            (0, self.has_end)
        } else {
            let b = (sym - 1) as u8;
            let bytes = &self.bytes[self.has_end as usize..];
            let j = bytes.iter().filter(|&&c| c < b).count();
            (self.has_end as usize + j, j < bytes.len() && bytes[j] == b)
        }
    }
}

/// An internal node; it fills a cache line.
#[derive(Clone, Copy, Debug)]
#[repr(C, align(64))]
struct Node {
    /// The parent, or [`NONE`].
    parent: u32,
    /// The length of the name in symbols (zero for the root).
    name_len: u32,
    /// The length of the extent.
    ext_len: u32,
    /// The first page of the subtree.
    first: u32,
    /// The last page of the subtree.
    last: u32,
    /// The page preceding the first page of the subtree, or [`NONE`].
    left: u32,
    /// A signature of the extent.
    ext_sig: u32,
    kids: Kids,
}

const _: () = assert!(size_of::<Node>() == 64);

/// The information associated with a page.
#[derive(Clone, Copy, Debug)]
struct Link {
    /// The parent of the leaf, or [`NONE`].
    parent: u32,
    /// The previous page, or [`NONE`].
    prev: u32,
}

/// The pages that can contain a string, in the order in which they should be
/// tried.
#[derive(Clone, Copy, Debug)]
pub(super) struct Candidates {
    ids: [u32; 2],
    len: usize,
}

impl Candidates {
    #[inline(always)]
    fn new(a: u32, b: u32) -> Self {
        if a == NONE {
            Self::new(b, NONE)
        } else {
            Self {
                ids: [a, b],
                len: 1 + (b != NONE && b != a) as usize,
            }
        }
    }

    #[inline(always)]
    pub(super) fn as_slice(&self) -> &[u32] {
        &self.ids[..self.len]
    }
}

pub(super) struct Router<H> {
    nodes: Vec<Node>,
    free: Vec<u32>,
    big: Vec<BigKids>,
    free_big: Vec<u32>,
    /// The z-map.
    map: HandleMap,
    /// The information associated with each page, indexed by the identifier
    /// of the page.
    links: Vec<Link>,
    /// The root: an internal node, or a page.
    root: u32,
    /// An upper bound on the length of the extents of internal nodes.
    max_ext: usize,
    seed: u64,
    sig_mask: u64,
    _marker: PhantomData<fn() -> H>,
}

impl<H: HashPrefixes> Router<H> {
    /// Creates a router for a set with a single page, whose identifier is
    /// zero, using signatures with the given number of bits.
    pub(super) fn new(sig_bits: u32) -> Self {
        assert!(sig_bits <= 64);
        Self {
            nodes: Vec::new(),
            free: Vec::new(),
            big: Vec::new(),
            free_big: Vec::new(),
            map: HandleMap::new(),
            links: vec![Link {
                parent: NONE,
                prev: NONE,
            }],
            root: LEAF,
            max_ext: 0,
            seed: SEED,
            sig_mask: if sig_bits == 64 {
                u64::MAX
            } else {
                (1 << sig_bits) - 1
            },
            _marker: PhantomData,
        }
    }

    /// Returns the number of bytes used by the router.
    pub(super) fn mem_size(&self) -> usize {
        self.nodes.capacity() * size_of::<Node>()
            + self.free.capacity() * size_of::<u32>()
            + self.big.capacity() * size_of::<BigKids>()
            + self
                .big
                .iter()
                .map(|b| b.ids.capacity() * 4 + b.bytes.capacity())
                .sum::<usize>()
            + self.free_big.capacity() * size_of::<u32>()
            + self.map.mem_size()
            + self.links.capacity() * size_of::<Link>()
    }

    #[inline(always)]
    fn sig(&self, key: &[u8], f: usize) -> u64 {
        H::hash_prefix(key, f, self.seed) & self.sig_mask
    }

    /// Returns the signature of an extent, given the hash of the extent.
    ///
    /// We use the same hash function, and the same seed, of the signatures
    /// of handles, so that a single preprocessing of the string provides
    /// both, but different bits: when the handle of a node is its extent,
    /// the two signatures are equal for the same strings, but in that case
    /// the second one is not necessary.
    #[inline(always)]
    fn ext_sig_of(&self, hash: u64) -> u32 {
        ((hash & self.sig_mask) as u32) ^ (((hash & self.sig_mask) >> 32) as u32)
    }

    #[inline(always)]
    fn ext_sig(&self, key: &[u8], ext: usize) -> u32 {
        self.ext_sig_of(H::hash_prefix(key, ext, self.seed))
    }

    /// Returns the preprocessed state for a string that is going to be
    /// passed to [`candidates`](Self::candidates).
    #[inline(always)]
    pub(super) fn hasher<'a>(&self, x: &'a [u8]) -> H::Hasher<'a> {
        H::hasher(x, x.len(), self.seed)
    }

    /// Returns the length of the handle of a node, which must not be the
    /// root.
    #[inline(always)]
    fn handle_len(&self, id: u32) -> usize {
        let node = &self.nodes[id as usize];
        two_fattest(node.name_len as usize - 1, node.ext_len as usize)
    }

    #[inline(always)]
    fn view<'a>(&'a self, kids: &'a Kids) -> KidView<'a> {
        if kids.len == BIG {
            let big = &self.big[kids.ids[0] as usize];
            KidView {
                ids: &big.ids,
                bytes: &big.bytes,
                has_end: big.has_end,
            }
        } else {
            let len = kids.len as usize;
            KidView {
                ids: &kids.ids[..len],
                bytes: &kids.bytes[..len],
                has_end: kids.has_end,
            }
        }
    }

    #[inline(always)]
    fn kids(&self, id: u32) -> KidView<'_> {
        self.view(&self.nodes[id as usize].kids)
    }

    /// Returns the first page of the subtree of a node or leaf.
    #[inline(always)]
    fn leftmost(&self, id: u32) -> u32 {
        if is_leaf(id) {
            id & !LEAF
        } else {
            self.nodes[id as usize].first
        }
    }

    /// Returns the last page of the subtree of a node or leaf.
    #[inline(always)]
    fn rightmost(&self, id: u32) -> u32 {
        if is_leaf(id) {
            id & !LEAF
        } else {
            self.nodes[id as usize].last
        }
    }

    // Queries

    /// Returns the pages that are likely to contain `x`.
    ///
    /// The result is based on signatures only: the caller must check
    /// whether one of the pages actually contains `x`, and otherwise call
    /// [`locate`](Self::locate).
    #[inline(always)]
    pub(super) fn candidates(&self, x: &[u8], hasher: &H::Hasher<'_>) -> Candidates {
        if is_leaf(self.root) {
            return Candidates::new(self.root & !LEAF, NONE);
        }
        // The extent of the root is empty, as the first page has an empty
        // lower fence
        let mut top = self.root;
        let b = x.len().min(self.max_ext);
        if b > 0 {
            fat_binary_search(
                hasher,
                0,
                b,
                |h| self.map.prefetch(h & self.sig_mask),
                |f, h| {
                    let (extent, id) = self
                        .map
                        .find(h & self.sig_mask, f, |id| self.handle_len(id) == f)?;
                    top = id;
                    Some(extent)
                },
            );
        }
        let node = &self.nodes[top as usize];
        let ext = node.ext_len as usize;
        if ext <= x.len() && (ext == 0 || self.ext_sig_of(hasher.hash(ext)) == node.ext_sig) {
            // We guess that the extent is a prefix of x
            let kids = self.view(&node.kids);
            let (i, found) = kids.position(sym_at(x, ext));
            if found {
                let child = kids.ids[i];
                if is_leaf(child) {
                    // x follows or precedes the lower fence of the page
                    let page = child & !LEAF;
                    Candidates::new(page, self.links[page as usize].prev)
                } else {
                    // We guess that x exits in the child, whose extent is
                    // not a prefix of x: then x follows or precedes all
                    // strings in the subtree
                    let child = &self.nodes[child as usize];
                    Candidates::new(child.last, child.left)
                }
            } else if i == 0 {
                Candidates::new(node.left, NONE)
            } else {
                Candidates::new(self.rightmost(kids.ids[i - 1]), NONE)
            }
        } else {
            // We guess that x exits at this node
            Candidates::new(node.last, node.left)
        }
    }

    /// Returns the page containing `x`, that is, the page with the largest
    /// lower fence that is not larger than `x`, by walking down the trie and
    /// comparing `x` with the extents.
    pub(super) fn locate(&self, x: &[u8], pages: &impl Anchors) -> u32 {
        if is_leaf(self.root) {
            return self.root & !LEAF;
        }
        let mut id = self.root;
        loop {
            let node = &self.nodes[id as usize];
            let ext = node.ext_len as usize;
            let extent = &pages.low(node.first)[..ext];
            let l = lcp(x, extent);
            if l < ext {
                // x exits at this node
                return if l == x.len() || x[l] < extent[l] {
                    node.left
                } else {
                    node.last
                };
            }
            let kids = self.view(&node.kids);
            let (i, found) = kids.position(sym_at(x, ext));
            if !found {
                return if i == 0 {
                    node.left
                } else {
                    self.rightmost(kids.ids[i - 1])
                };
            }
            let child = kids.ids[i];
            if is_leaf(child) {
                let page = child & !LEAF;
                return if x >= pages.low(page) {
                    page
                } else {
                    self.links[page as usize].prev
                };
            }
            id = child;
        }
    }

    // Updates

    fn new_node(&mut self, node: Node) -> u32 {
        match self.free.pop() {
            Some(id) => {
                self.nodes[id as usize] = node;
                id
            }
            None => {
                assert!(self.nodes.len() < LEAF as usize - 1, "too many nodes");
                self.nodes.push(node);
                (self.nodes.len() - 1) as u32
            }
        }
    }

    /// Returns an internal node with two children.
    fn new_binary(
        &mut self,
        parent: u32,
        name_len: usize,
        ext_len: usize,
        ext_sig: u32,
        mut a: (Sym, u32),
        mut b: (Sym, u32),
    ) -> u32 {
        debug_assert!(a.0 != b.0);
        if a.0 > b.0 {
            std::mem::swap(&mut a, &mut b);
        }
        let mut kids = Kids {
            ids: [0; INLINE],
            bytes: [0; INLINE],
            len: 2,
            has_end: a.0 == 0,
        };
        kids.ids[0] = a.1;
        kids.ids[1] = b.1;
        kids.bytes[0] = a.0.saturating_sub(1) as u8;
        kids.bytes[1] = (b.0 - 1) as u8;
        self.new_node(Node {
            parent,
            name_len: name_len as u32,
            ext_len: ext_len as u32,
            first: NONE,
            last: NONE,
            left: NONE,
            ext_sig,
            kids,
        })
    }

    fn free_node(&mut self, id: u32) {
        let kids = self.nodes[id as usize].kids;
        if kids.len == BIG {
            self.big[kids.ids[0] as usize] = BigKids::default();
            self.free_big.push(kids.ids[0]);
        }
        self.free.push(id);
    }

    /// Adds a child associated with a symbol to an internal node; the node
    /// must have no child associated with the same symbol.
    fn add_kid(&mut self, id: u32, sym: Sym, child: u32) {
        let (i, found) = self.kids(id).position(sym);
        debug_assert!(!found);
        let byte = sym.saturating_sub(1) as u8;
        let kids = &mut self.nodes[id as usize].kids;
        if kids.len == BIG {
            let big = &mut self.big[kids.ids[0] as usize];
            big.ids.insert(i, child);
            big.bytes.insert(i, byte);
            big.has_end |= sym == 0;
        } else if (kids.len as usize) < INLINE {
            let len = kids.len as usize;
            kids.ids.copy_within(i..len, i + 1);
            kids.bytes.copy_within(i..len, i + 1);
            kids.ids[i] = child;
            kids.bytes[i] = byte;
            kids.len += 1;
            kids.has_end |= sym == 0;
        } else {
            let mut big = BigKids {
                ids: Vec::with_capacity(2 * INLINE),
                bytes: Vec::with_capacity(2 * INLINE),
                has_end: kids.has_end || sym == 0,
            };
            big.ids.extend_from_slice(&kids.ids);
            big.bytes.extend_from_slice(&kids.bytes);
            big.ids.insert(i, child);
            big.bytes.insert(i, byte);
            kids.len = BIG;
            let index = match self.free_big.pop() {
                Some(index) => {
                    self.big[index as usize] = big;
                    index
                }
                None => {
                    self.big.push(big);
                    (self.big.len() - 1) as u32
                }
            };
            self.nodes[id as usize].kids.ids[0] = index;
        }
    }

    /// Removes the child associated with a symbol from an internal node
    /// with at least three children.
    fn remove_kid(&mut self, id: u32, sym: Sym) {
        let (i, found) = self.kids(id).position(sym);
        debug_assert!(found);
        let kids = &mut self.nodes[id as usize].kids;
        if kids.len == BIG {
            let index = kids.ids[0];
            let big = &mut self.big[index as usize];
            big.ids.remove(i);
            big.bytes.remove(i);
            big.has_end &= sym != 0;
            if big.ids.len() <= INLINE {
                let len = big.ids.len();
                kids.ids[..len].copy_from_slice(&big.ids);
                kids.bytes[..len].copy_from_slice(&big.bytes);
                kids.len = len as u8;
                kids.has_end = big.has_end;
                *big = BigKids::default();
                self.free_big.push(index);
            }
        } else {
            let len = kids.len as usize;
            debug_assert!(len >= 3);
            kids.ids.copy_within(i + 1..len, i);
            kids.bytes.copy_within(i + 1..len, i);
            kids.len -= 1;
            kids.has_end &= sym != 0;
        }
    }

    /// Replaces the child associated with a symbol in an internal node.
    fn replace_kid(&mut self, id: u32, sym: Sym, child: u32) {
        let (i, found) = self.kids(id).position(sym);
        debug_assert!(found);
        let kids = &mut self.nodes[id as usize].kids;
        if kids.len == BIG {
            self.big[kids.ids[0] as usize].ids[i] = child;
        } else {
            kids.ids[i] = child;
        }
    }

    /// Recomputes the first and last page, and the page preceding the first
    /// one, of a node and of all its ancestors.
    fn refresh_up(&mut self, mut id: u32) {
        while id != NONE {
            let kids = self.kids(id);
            let first = self.leftmost(kids.ids[0]);
            let last = self.rightmost(kids.ids[kids.ids.len() - 1]);
            let left = self.links[first as usize].prev;
            let node = &mut self.nodes[id as usize];
            node.first = first;
            node.last = last;
            node.left = left;
            id = node.parent;
        }
    }

    /// Updates the page preceding the first one in all nodes whose first
    /// page is the given one.
    fn fix_left(&mut self, page: u32) {
        if page == NONE {
            return;
        }
        let Link { parent, prev } = self.links[page as usize];
        let mut id = parent;
        while id != NONE && self.nodes[id as usize].first == page {
            self.nodes[id as usize].left = prev;
            id = self.nodes[id as usize].parent;
        }
    }

    /// Adds a page, whose lower fence must have been set, between the pages
    /// `prev` and `next` (the latter can be [`NONE`]).
    pub(super) fn insert(&mut self, page: u32, prev: u32, next: u32, pages: &impl Anchors) {
        if self.links.len() <= page as usize {
            self.links.resize(
                page as usize + 1,
                Link {
                    parent: NONE,
                    prev: NONE,
                },
            );
        }
        self.links[page as usize].prev = prev;
        if next != NONE {
            self.links[next as usize].prev = page;
        }
        let a = pages.low(page);
        let leaf = page | LEAF;

        if is_leaf(self.root) {
            let other = self.root & !LEAF;
            let b = pages.low(other);
            let l = lcp(a, b);
            // The lower fence of the first page is empty
            debug_assert_eq!(l, 0);
            let id = self.new_binary(
                NONE,
                0,
                l,
                0,
                (sym_at(b, l), self.root),
                (sym_at(a, l), leaf),
            );
            self.links[other as usize].parent = id;
            self.links[page as usize].parent = id;
            self.root = id;
            self.refresh_up(id);
            self.fix_left(next);
            return;
        }

        let mut id = self.root;
        loop {
            let node = self.nodes[id as usize];
            let (name_len, ext) = (node.name_len as usize, node.ext_len as usize);
            let extent = &pages.low(node.first)[..ext];
            let l = lcp_from(a, extent, name_len.min(ext));
            if l < ext {
                // The new string exits at this node, which has to be split:
                // the new node keeps the name, and the old one the extent
                debug_assert!(name_len > 0 && l >= name_len);
                let sig = self.sig(extent, two_fattest(name_len - 1, ext));
                self.map.remove(sig, id);
                let ext_sig = self.ext_sig(extent, l);
                let new = self.new_binary(
                    node.parent,
                    name_len,
                    l,
                    ext_sig,
                    (sym_at(extent, l), id),
                    (sym_at(a, l), leaf),
                );
                let sig = self.sig(extent, two_fattest(name_len - 1, l));
                self.map.insert(sig, l, new);
                let sig = self.sig(extent, two_fattest(l, ext));
                self.map.insert(sig, ext, id);
                self.nodes[id as usize].name_len = l as u32 + 1;
                self.nodes[id as usize].parent = new;
                let parent_ext = self.nodes[node.parent as usize].ext_len as usize;
                self.replace_kid(node.parent, sym_at(extent, parent_ext), new);
                self.links[page as usize].parent = new;
                self.refresh_up(new);
                break;
            }
            let sym = sym_at(a, ext);
            let kids = self.view(&self.nodes[id as usize].kids);
            let (i, found) = kids.position(sym);
            if !found {
                self.add_kid(id, sym, leaf);
                self.links[page as usize].parent = id;
                self.refresh_up(id);
                break;
            }
            let child = kids.ids[i];
            if !is_leaf(child) {
                id = child;
                continue;
            }
            // The two leaves become the children of a new node
            debug_assert!(sym != 0, "duplicate lower fence");
            let other = child & !LEAF;
            let b = pages.low(other);
            let l = lcp_from(a, b, ext + 1);
            debug_assert!(l < a.len() || l < b.len(), "duplicate lower fence");
            let ext_sig = self.ext_sig(a, l);
            let new = self.new_binary(
                id,
                ext + 1,
                l,
                ext_sig,
                (sym_at(b, l), child),
                (sym_at(a, l), leaf),
            );
            let sig = self.sig(a, two_fattest(ext, l));
            self.map.insert(sig, l, new);
            self.max_ext = self.max_ext.max(l);
            self.replace_kid(id, sym, new);
            self.links[other as usize].parent = new;
            self.links[page as usize].parent = new;
            self.refresh_up(new);
            break;
        }
        self.fix_left(next);
    }

    /// Removes a page, which must not be the first one, and whose lower fence
    /// must still be set; `next` is the following page, or [`NONE`].
    pub(super) fn remove(&mut self, page: u32, next: u32, pages: &impl Anchors) {
        let Link { parent, prev } = self.links[page as usize];
        debug_assert!(parent != NONE && prev != NONE);
        let a = pages.low(page);
        let node = self.nodes[parent as usize];
        let (name_len, ext) = (node.name_len as usize, node.ext_len as usize);
        let sym = sym_at(a, ext);
        let kids = self.view(&self.nodes[parent as usize].kids);
        let up = if kids.ids.len() > 2 {
            self.remove_kid(parent, sym);
            parent
        } else {
            // The other child replaces the parent
            let (i, found) = kids.position(sym);
            debug_assert!(found);
            let other = kids.ids[1 - i];
            if name_len > 0 {
                let sig = self.sig(a, two_fattest(name_len - 1, ext));
                self.map.remove(sig, parent);
            }
            if is_leaf(other) {
                self.links[(other & !LEAF) as usize].parent = node.parent;
            } else {
                // The node below the root of a trie with at least two leaves
                // cannot become the root, as the extent of the root is empty
                debug_assert!(name_len > 0);
                let child = self.nodes[other as usize];
                let (child_name, child_ext) = (child.name_len as usize, child.ext_len as usize);
                let extent = &pages.low(child.first)[..child_ext];
                let sig = self.sig(extent, two_fattest(child_name - 1, child_ext));
                self.map.remove(sig, other);
                let sig = self.sig(extent, two_fattest(name_len - 1, child_ext));
                self.map.insert(sig, child_ext, other);
                self.nodes[other as usize].name_len = name_len as u32;
                self.nodes[other as usize].parent = node.parent;
            }
            if node.parent == NONE {
                self.root = other;
            } else {
                let parent_ext = self.nodes[node.parent as usize].ext_len as usize;
                self.replace_kid(node.parent, sym_at(a, parent_ext), other);
            }
            self.free_node(parent);
            node.parent
        };
        self.links[page as usize] = Link {
            parent: NONE,
            prev: NONE,
        };
        if next != NONE {
            self.links[next as usize].prev = prev;
        }
        self.refresh_up(up);
        self.fix_left(next);
    }

    // Invariants

    /// Checks the invariants of the router, given the pages in order.
    pub(super) fn check(&self, order: &[u32], pages: &impl Anchors) {
        let mut rank = vec![usize::MAX; self.links.len()];
        for (i, &page) in order.iter().enumerate() {
            rank[page as usize] = i;
            let prev = if i == 0 { NONE } else { order[i - 1] };
            assert_eq!(self.links[page as usize].prev, prev, "wrong previous page");
            assert_eq!(self.locate(pages.low(page), pages), page);
        }
        let mut handles = 0;
        let leaves = self.check_subtree(self.root, NONE, 0, &rank, pages, &mut handles);
        assert_eq!(leaves, order.len());
        assert_eq!(handles, self.map.len());
        self.map.check();
    }

    /// Checks a subtree, returning the number of its leaves.
    fn check_subtree(
        &self,
        id: u32,
        parent: u32,
        name_len: usize,
        rank: &[usize],
        pages: &impl Anchors,
        handles: &mut usize,
    ) -> usize {
        if is_leaf(id) {
            let page = id & !LEAF;
            assert_eq!(self.links[page as usize].parent, parent);
            assert!(rank[page as usize] != usize::MAX);
            return 1;
        }
        let node = &self.nodes[id as usize];
        let ext = node.ext_len as usize;
        assert_eq!(node.parent, parent);
        assert_eq!(node.name_len as usize, name_len);
        let (first, last) = (pages.low(node.first), pages.low(node.last));
        assert_eq!(lcp(first, last), ext, "wrong extent");
        assert!(ext <= self.max_ext);
        let extent = &first[..ext];
        if name_len == 0 {
            assert_eq!(ext, 0);
        } else {
            assert!(name_len <= ext);
            assert_eq!(node.ext_sig, self.ext_sig(extent, ext));
            let f = two_fattest(name_len - 1, ext);
            let found = self.map.find(self.sig(extent, f), f, |other| other == id);
            assert_eq!(found, Some((ext, id)), "handle not found");
            *handles += 1;
        }
        let kids = self.view(&node.kids);
        assert!(kids.ids.len() >= 2);
        if node.kids.len != BIG {
            assert!(kids.ids.len() <= INLINE);
        } else {
            assert!(kids.ids.len() > INLINE);
        }
        let mut leaves = 0;
        let mut prev_sym = None;
        for (i, &kid) in kids.ids.iter().enumerate() {
            let sym = sym_at(pages.low(self.leftmost(kid)), ext);
            assert!(prev_sym < Some(sym), "children out of order");
            prev_sym = Some(sym);
            assert_eq!(kids.position(sym), (i, true));
            leaves += self.check_subtree(kid, id, ext + 1, rank, pages, handles);
        }
        assert_eq!(node.first, self.leftmost(kids.ids[0]));
        assert_eq!(node.last, self.rightmost(kids.ids[kids.ids.len() - 1]));
        assert_eq!(node.left, self.links[node.first as usize].prev);
        // The leaves of a subtree are consecutive pages
        assert_eq!(
            rank[node.last as usize] - rank[node.first as usize] + 1,
            leaves
        );
        leaves
    }
}
