/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! A dynamic z-fast trie on byte strings.

mod check;
pub(crate) mod key;
pub(crate) mod map;

use crate::utils::{HashPrefixes, PrefixHasher, Xxh3, fat_binary_search, jump, lcp, two_fattest};
use key::Key;
use map::HandleMap;
use std::fmt;
use std::marker::PhantomData;

/// The bit marking the identifier of a leaf.
const LEAF: u32 = 1 << 31;
/// The identifier of no node.
const NONE: u32 = u32::MAX;
/// The default seed.
const SEED: u64 = 0x05EE_D0F2_FA57;
/// The number of children that are stored in an internal node.
const INLINE: usize = 7;
/// The value of the `len` field of [`Children`] for nodes whose children
/// are stored elsewhere.
const BIG: u8 = u8::MAX;

#[inline(always)]
const fn is_leaf(id: u32) -> bool {
    id & LEAF != 0
}

#[inline(always)]
const fn leaf_index(id: u32) -> usize {
    debug_assert!(is_leaf(id) && id != NONE);
    (id & !LEAF) as usize
}

/// A symbol: zero for the (virtual) terminator that follows every key, and
/// *b* + 1 for the byte *b*.
type Sym = u16;

/// Returns the symbol in position `i` of the key `x` followed by the
/// terminator.
#[inline(always)]
fn sym_at(x: &[u8], i: usize) -> Sym {
    debug_assert!(i <= x.len());
    if i < x.len() { x[i] as Sym + 1 } else { 0 }
}

/// The children of an internal node, in order: up to [`INLINE`] children are
/// stored in the node, and larger sets are stored in a separate
/// [`BigChildren`] instance.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct Children {
    /// The identifiers of the children; if `len` is [`BIG`], the first
    /// element is the index of a [`BigChildren`] instance instead.
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
struct BigChildren {
    ids: Vec<u32>,
    bytes: Vec<u8>,
    has_end: bool,
}

/// A view of the children of an internal node.
#[derive(Clone, Copy)]
struct ChildView<'a> {
    ids: &'a [u32],
    bytes: &'a [u8],
    has_end: bool,
}

impl ChildView<'_> {
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

    #[inline(always)]
    fn first(&self) -> u32 {
        self.ids[0]
    }

    #[inline(always)]
    fn last(&self) -> u32 {
        self.ids[self.ids.len() - 1]
    }

    /// Returns the symbol associated with the child of given index.
    #[inline(always)]
    fn sym(&self, i: usize) -> Sym {
        if i == 0 && self.has_end {
            0
        } else {
            self.bytes[i] as Sym + 1
        }
    }
}

/// An internal node.
///
/// Nodes are aligned to cache lines, and they fill exactly one.
#[repr(C, align(64))]
struct Internal {
    /// A key whose first `extent_len` bytes are the extent of this node, or
    /// `None` if the node is free.
    key: Option<Key>,
    /// The length of the name of this node, that is, the length of the
    /// extent of the parent plus one, or zero for the root.
    name_len: u32,
    /// The length of the extent of this node, that is, of the longest common
    /// prefix of the keys of its leaves.
    extent_len: u32,
    /// The node on the path of first children starting from this node whose
    /// skip interval contains the 2-fat jump of the handle length of this
    /// node, or the leaf at the end of the path.
    jump_left: u32,
    /// As `jump_left`, but for the path of last children.
    jump_right: u32,
    children: Children,
}

const _: () = assert!(size_of::<Internal>() == 64);

impl Internal {
    /// Returns the key providing the extent of this node.
    #[inline(always)]
    fn key(&self) -> &[u8] {
        match &self.key {
            Some(key) => key.bytes(),
            None => unreachable!("free node"),
        }
    }

    /// Returns the extent of this node.
    #[inline(always)]
    fn extent(&self) -> &[u8] {
        &self.key()[..self.extent_len as usize]
    }

    /// Returns the length of the handle of this node, that is, the 2-fattest
    /// number in its skip interval (`name_len - 1`. . `extent_len`].
    ///
    /// The handle of the root has length zero.
    #[inline(always)]
    fn handle_len(&self) -> usize {
        if self.name_len == 0 {
            0
        } else {
            two_fattest(self.name_len as usize - 1, self.extent_len as usize)
        }
    }

    /// Returns the 2-fat jump of the length of the handle of this node.
    #[inline(always)]
    fn jump_len(&self) -> usize {
        jump(self.handle_len())
    }
}

/// A leaf.
struct Leaf<V> {
    /// The key, or `None` if the leaf is free.
    key: Option<Key>,
    /// The leaf of the previous key, or [`NONE`].
    prev: u32,
    /// The leaf of the next key, or [`NONE`].
    next: u32,
    /// The value associated with the key.
    value: V,
}

/// The side of a jump pointer, or of the path it follows.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Side {
    Left,
    Right,
}

/// The place at which a string exits the trie.
#[derive(Clone, Copy, Debug)]
struct Loc {
    /// The parent of `exit`, if it is known, or the node with a missing
    /// child if `exit` is [`NONE`]; it is [`NONE`] if `exit` is the root.
    ///
    /// The parent is always known if the search was provided with a stack.
    parex: u32,
    /// The exit node, that is, the node whose name is a prefix of the string
    /// (followed by the terminator) but whose extent is not a proper prefix
    /// of the string (followed by the terminator); or [`NONE`], if the string
    /// follows a path up to the extent of `parex`, but then `parex` has no
    /// child for the next symbol.
    exit: u32,
    /// The length of the longest common prefix of the string and of the
    /// extent of `exit` (of `parex`, if `exit` is [`NONE`]).
    lcp: usize,
}

/// A dynamic z-fast trie: a sorted set of byte strings.
///
/// The set supports insertions, deletions, membership, predecessor and
/// successor queries, and iteration in lexicographic order (the order of
/// `[u8]`). All operations but iteration take time logarithmic in the *length
/// of the string* involved (besides the time needed to read it), independently
/// of the number of keys, modulo the usual caveats about hashing.
///
/// # Structure
///
/// Conceptually, the structure is the compacted trie of the keys, with
/// branching on bytes: a key that is a prefix of another key is handled by
/// imagining that each key is followed by a terminator smaller than all bytes.
/// Each internal node has a *name* (the string leading to it), an *extent*
/// (the longest common prefix of the keys below it), and a *handle*: the
/// prefix of the extent whose length is the *2-fattest number* (the one with
/// most trailing zeros) in the *skip interval*, which goes from the length of
/// the name to the length of the extent. The *z-map* is a hash table mapping
/// the signatures of the handles to the nodes.
///
/// A *fat binary search* on the length of a string, using only the z-map,
/// locates the node at which the string exits the trie, or its parent. The
/// search can be misled by signature collisions, but its result is verified
/// with a single string comparison, and in case of failure (which is extremely
/// unlikely) the search is repeated by walking down the trie.
///
/// Each internal node has two *jump pointers*, which make it possible to
/// reach the first and last leaf of a subtree in a logarithmic number of
/// steps, and thus to answer predecessor and successor queries, as the leaves
/// form a doubly linked list. The search can provide the stack of the *2-fat
/// ancestors* of a node, which are the only nodes whose jump pointers might
/// need to be fixed after an update.
///
/// Internal nodes store no bytes: each node shares the key of a leaf that
/// was below it when the node was created.
///
/// The type parameter `H` is the [strategy used to hash prefixes](HashPrefixes).
///
/// # Examples
///
/// ```
/// # use zfast::ZFastTrie;
/// let mut trie = <ZFastTrie>::new();
/// assert!(trie.insert("foo"));
/// assert!(trie.insert("bar"));
/// assert!(trie.insert("foobar"));
/// assert!(!trie.insert("foo"));
/// assert_eq!(trie.len(), 3);
///
/// assert!(trie.contains("bar"));
/// assert!(!trie.contains("fo"));
/// assert_eq!(trie.succ("c"), Some("foo".as_bytes()));
/// assert_eq!(trie.pred("c"), Some("bar".as_bytes()));
/// assert_eq!(trie.succ_strict("foo"), Some("foobar".as_bytes()));
///
/// assert!(trie.remove("foo"));
/// let keys: Vec<&[u8]> = trie.iter().collect();
/// assert_eq!(keys, vec!["bar".as_bytes(), "foobar".as_bytes()]);
/// ```
pub struct ZFastTrie<H = Xxh3, V = ()> {
    /// The internal nodes.
    internals: Vec<Internal>,
    /// The indices of the free internal nodes.
    free_internals: Vec<u32>,
    /// The leaves.
    leaves: Vec<Leaf<V>>,
    /// The indices of the free leaves.
    free_leaves: Vec<u32>,
    /// The sets of children that do not fit an internal node.
    big: Vec<BigChildren>,
    /// The indices of the free elements of `big`.
    free_big: Vec<u32>,
    /// The z-map.
    map: HandleMap,
    /// The root, or [`NONE`] if the trie is empty.
    root: u32,
    /// The leaf of the smallest key, or [`NONE`] if the trie is empty.
    first: u32,
    /// The leaf of the largest key, or [`NONE`] if the trie is empty.
    last: u32,
    /// The number of keys.
    len: usize,
    /// An upper bound on the length of the extent of the internal nodes.
    max_extent: usize,
    /// A stack reused by updates.
    stack: Vec<u32>,
    /// The seed of the hash function.
    seed: u64,
    /// A mask applied to signatures (used for testing only).
    sig_mask: u64,
    _marker: PhantomData<H>,
}

// SAFETY: the only reason why the trie is not automatically Send and Sync is
// that the reference counts of the keys are not atomic. All the handles to a
// key are however contained in the nodes of the same trie, so they are always
// moved to another thread together, and reference counts are modified only by
// methods taking a mutable reference (or ownership); methods taking a shared
// reference just read the bytes.
unsafe impl<H, V: Send> Send for ZFastTrie<H, V> {}
unsafe impl<H, V: Sync> Sync for ZFastTrie<H, V> {}

impl<H: HashPrefixes, V: Copy> Default for ZFastTrie<H, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<H: HashPrefixes, V: Copy> ZFastTrie<H, V> {
    /// The maximum length of a key.
    pub const MAX_KEY_LEN: usize = (1 << 31) - 2;

    /// Creates a new empty dynamic z-fast trie.
    pub fn new() -> Self {
        Self::with_sig_bits(64)
    }

    /// Creates a new empty dynamic z-fast trie whose signatures have the
    /// given number of bits.
    ///
    /// This method is useful only to test the behavior of the structure in
    /// the presence of signature collisions.
    #[doc(hidden)]
    pub fn with_sig_bits(sig_bits: u32) -> Self {
        assert!(sig_bits <= 64);
        Self {
            internals: Vec::new(),
            free_internals: Vec::new(),
            leaves: Vec::new(),
            free_leaves: Vec::new(),
            big: Vec::new(),
            free_big: Vec::new(),
            map: HandleMap::new(),
            root: NONE,
            first: NONE,
            last: NONE,
            len: 0,
            max_extent: 0,
            stack: Vec::new(),
            seed: SEED,
            sig_mask: if sig_bits == 64 {
                u64::MAX
            } else {
                (1 << sig_bits) - 1
            },
            _marker: PhantomData,
        }
    }

    /// Returns the number of keys.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if there are no keys.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Removes all keys.
    pub fn clear(&mut self) {
        *self = Self {
            seed: self.seed,
            sig_mask: self.sig_mask,
            ..Self::with_sig_bits(64)
        };
    }

    /// Returns the number of bytes occupied by this structure, including the
    /// keys.
    pub fn mem_size(&self) -> usize {
        size_of::<Self>()
            + self.internals.capacity() * size_of::<Internal>()
            + self.leaves.capacity() * size_of::<Leaf<V>>()
            + (self.free_internals.capacity()
                + self.free_leaves.capacity()
                + self.free_big.capacity()
                + self.stack.capacity())
                * size_of::<u32>()
            + self.big.capacity() * size_of::<BigChildren>()
            + self
                .big
                .iter()
                .map(|big| big.ids.capacity() * size_of::<u32>() + big.bytes.capacity())
                .sum::<usize>()
            + self.map.mem_size()
            + self.key_mem_size()
    }

    /// Returns the number of bytes allocated for the keys (including those
    /// kept alive by internal nodes).
    fn key_mem_size(&self) -> usize {
        let mut size = 0;
        // Keys of live leaves
        let mut leaf = self.first;
        while leaf != NONE {
            let l = &self.leaves[leaf_index(leaf)];
            size += l.key.as_ref().unwrap().alloc_size();
            leaf = l.next;
        }
        size
    }

    // Accessors

    #[inline(always)]
    fn internal(&self, id: u32) -> &Internal {
        debug_assert!(!is_leaf(id));
        &self.internals[id as usize]
    }

    #[inline(always)]
    fn internal_mut(&mut self, id: u32) -> &mut Internal {
        debug_assert!(!is_leaf(id));
        &mut self.internals[id as usize]
    }

    #[inline(always)]
    fn leaf(&self, id: u32) -> &Leaf<V> {
        &self.leaves[leaf_index(id)]
    }

    #[inline(always)]
    fn leaf_mut(&mut self, id: u32) -> &mut Leaf<V> {
        &mut self.leaves[leaf_index(id)]
    }

    #[inline(always)]
    fn leaf_key(&self, id: u32) -> &[u8] {
        match &self.leaf(id).key {
            Some(key) => key.bytes(),
            None => unreachable!("free leaf"),
        }
    }

    /// Returns the bytes of the extent of a node; for a leaf, the terminator
    /// is not included.
    #[inline(always)]
    fn extent(&self, id: u32) -> &[u8] {
        if is_leaf(id) {
            self.leaf_key(id)
        } else {
            self.internal(id).extent()
        }
    }

    #[inline(always)]
    fn view<'a>(&'a self, children: &'a Children) -> ChildView<'a> {
        if children.len == BIG {
            let big = &self.big[children.ids[0] as usize];
            ChildView {
                ids: &big.ids,
                bytes: &big.bytes,
                has_end: big.has_end,
            }
        } else {
            let len = children.len as usize;
            ChildView {
                ids: &children.ids[..len],
                bytes: &children.bytes[..len],
                has_end: children.has_end,
            }
        }
    }

    #[inline(always)]
    fn children(&self, id: u32) -> ChildView<'_> {
        self.view(&self.internal(id).children)
    }

    #[inline(always)]
    fn jump(&self, id: u32, side: Side) -> u32 {
        match side {
            Side::Left => self.internal(id).jump_left,
            Side::Right => self.internal(id).jump_right,
        }
    }

    #[inline(always)]
    fn set_jump(&mut self, id: u32, side: Side, target: u32) {
        match side {
            Side::Left => self.internal_mut(id).jump_left = target,
            Side::Right => self.internal_mut(id).jump_right = target,
        }
    }

    /// Returns the first (if `side` is left) or last (if `side` is right)
    /// child of an internal node.
    #[inline(always)]
    fn extreme_child(&self, id: u32, side: Side) -> u32 {
        let children = self.children(id);
        match side {
            Side::Left => children.first(),
            Side::Right => children.last(),
        }
    }

    /// Returns the first (if `side` is left) or last (if `side` is right)
    /// leaf of the subtree rooted at a node by following jump pointers.
    #[inline]
    fn extreme_leaf(&self, mut id: u32, side: Side) -> u32 {
        while !is_leaf(id) {
            id = self.jump(id, side);
        }
        id
    }

    /// Returns true if the node, which must lie on the path of first or last
    /// children starting from a node whose jump length is `jump_len`, is the
    /// target of the associated jump pointer.
    #[inline(always)]
    fn intercepts(&self, id: u32, jump_len: usize) -> bool {
        is_leaf(id) || jump_len <= self.internal(id).extent_len as usize
    }

    #[inline(always)]
    fn sig(&self, key: &[u8], f: usize) -> u64 {
        H::hash_prefix(key, f, self.seed) & self.sig_mask
    }

    /// Returns true if the handle of the node is the prefix of length `f`
    /// of `x`.
    #[inline]
    fn handle_is(&self, id: u32, x: &[u8], f: usize) -> bool {
        let node = self.internal(id);
        node.handle_len() == f && node.key()[..f] == x[..f]
    }

    // Search

    /// Locates the place at which `x` exits the trie, which must be
    /// nonempty.
    ///
    /// If a stack is provided, it is filled with the 2-fat ancestors of the
    /// parent of the exit node (or of the node with a missing child), from
    /// the root down.
    #[inline]
    fn locate(&self, x: &[u8], mut stack: Option<&mut Vec<u32>>) -> Loc {
        debug_assert!(self.root != NONE);
        if let Some(stack) = stack.as_deref_mut() {
            stack.clear();
        }
        if is_leaf(self.root) {
            return Loc {
                parex: NONE,
                exit: self.root,
                lcp: lcp(x, self.leaf_key(self.root)),
            };
        }
        let root = self.internal(self.root);
        let extent = root.extent_len as usize;
        let l = lcp(x, root.extent());
        if l < extent {
            return Loc {
                parex: NONE,
                exit: self.root,
                lcp: l,
            };
        }
        if let Some(stack) = stack.as_deref_mut() {
            stack.push(self.root);
        }

        // The fat binary search proper: top is the last node that was hit,
        // and f the length of the prefix used to find it.
        let (mut top, mut f_top) = (self.root, 0);
        let b = x.len().min(self.max_extent);
        if extent < b {
            let hasher = H::hasher(x, b, self.seed);
            fat_binary_search(
                &hasher,
                extent,
                b,
                |h| self.map.prefetch(h & self.sig_mask),
                |f, h| {
                    let (extent, id) = self
                        .map
                        .find(h & self.sig_mask, f, |id| self.handle_is(id, x, f))?;
                    top = id;
                    f_top = f;
                    if let Some(stack) = stack.as_deref_mut() {
                        stack.push(id);
                    }
                    Some(extent)
                },
            );
        }

        match self.verify(x, top, f_top, stack.as_deref_mut()) {
            Some(loc) => loc,
            None => self.locate_slow(x, stack),
        }
    }

    /// Verifies the result of a fat binary search for `x` that ended at the
    /// internal node `top`, found using a prefix of length `f`.
    ///
    /// If the node is neither the exit node of `x` nor its parent, returns
    /// `None`.
    #[inline(always)]
    fn verify(&self, x: &[u8], top: u32, f: usize, stack: Option<&mut Vec<u32>>) -> Option<Loc> {
        let node = self.internal(top);
        let extent = node.extent_len as usize;
        // We must have found the node using its handle
        if node.handle_len() != f {
            return None;
        }
        let l = if extent <= x.len() {
            // We guess that top is the parent of the exit node, and thus
            // that the exit node is one of its children
            let children = self.view(&node.children);
            let (i, found) = children.position(sym_at(x, extent));
            if found {
                let child = children.ids[i];
                let child_extent = self.extent(child);
                let l = lcp(x, child_extent);
                if l >= extent {
                    // We guessed right. The extent of an internal child
                    // cannot be a prefix of x.
                    if !is_leaf(child) && l == child_extent.len() {
                        debug_assert!(self.sig_mask != u64::MAX);
                        return None;
                    }
                    return Some(Loc {
                        parex: top,
                        exit: child,
                        lcp: l,
                    });
                }
                l
            } else {
                let l = lcp(x, node.extent());
                if l >= extent {
                    // We guessed right, but there is no exit node
                    return Some(Loc {
                        parex: top,
                        exit: NONE,
                        lcp: l,
                    });
                }
                l
            }
        } else {
            lcp(x, node.extent())
        };

        // Now l is the length of the longest common prefix of x and of the
        // extent of top, which is not a prefix of x: either the handle of
        // top is a prefix of x, and then top is the exit node, or a false
        // positive misled the search.
        if l < f {
            return None;
        }
        debug_assert!(top != self.root);
        let parex = match stack {
            None => NONE,
            Some(stack) => {
                // We have to replace top with the 2-fat ancestors of its
                // parent that are not 2-fat ancestors of top
                let popped = stack.pop();
                debug_assert_eq!(popped, Some(top));
                self.complete(x, node.name_len as usize - 1, stack);
                *stack.last().unwrap()
            }
        };
        Some(Loc {
            parex,
            exit: top,
            lcp: l,
        })
    }

    /// Completes a stack containing some 2-fat ancestors of a node whose
    /// extent has length `b` and is a prefix of `x`, so that it contains all
    /// of them.
    ///
    /// The stack must contain all 2-fat ancestors whose handle is not longer
    /// than the extent of the top of the stack, which must be an ancestor of
    /// the node, or the node itself.
    #[inline]
    fn complete(&self, x: &[u8], b: usize, stack: &mut Vec<u32>) {
        let mut a = self.internal(*stack.last().unwrap()).extent_len as usize;
        if a < b {
            let hasher = H::hasher(x, b, self.seed);
            while a < b {
                let f = two_fattest(a, b);
                // In this interval all probes are successful
                let (extent, id) = self
                    .map
                    .find(hasher.hash(f) & self.sig_mask, f, |id| {
                        self.handle_is(id, x, f)
                    })
                    .expect("missing handle");
                debug_assert!(extent <= b);
                a = extent;
                stack.push(id);
            }
        }
    }

    /// A version of [`locate`](Self::locate) that walks down the trie.
    #[cold]
    #[inline(never)]
    fn locate_slow(&self, x: &[u8], stack: Option<&mut Vec<u32>>) -> Loc {
        let mut path = Vec::new();
        let mut id = self.root;
        let loc = loop {
            let parex = path.last().copied().unwrap_or(NONE);
            let extent = self.extent(id);
            let l = lcp(x, extent);
            if is_leaf(id) || l < extent.len() {
                break Loc {
                    parex,
                    exit: id,
                    lcp: l,
                };
            }
            path.push(id);
            let children = self.children(id);
            let (i, found) = children.position(sym_at(x, l));
            if !found {
                break Loc {
                    parex: id,
                    exit: NONE,
                    lcp: l,
                };
            }
            id = children.ids[i];
        };
        if let Some(stack) = stack {
            stack.clear();
            if let Some(&last) = path.last() {
                let b = self.internal(last).extent_len as usize;
                stack.extend(
                    path.iter()
                        .copied()
                        .filter(|&id| self.internal(id).jump_len() > b),
                );
            }
        }
        loc
    }

    // Queries

    /// Returns true if `x` is a key.
    #[inline]
    pub fn contains(&self, x: impl AsRef<[u8]>) -> bool {
        let x = x.as_ref();
        if self.root == NONE {
            return false;
        }
        let loc = self.locate(x, None);
        loc.exit != NONE && is_leaf(loc.exit) && loc.lcp == x.len() && {
            self.leaf_key(loc.exit).len() == x.len()
        }
    }

    /// Returns the leaf of the largest key smaller than `x` (or [`NONE`]),
    /// and the leaf of `x`, if `x` is a key (or [`NONE`]).
    #[inline]
    fn pred_leaf(&self, x: &[u8]) -> (u32, u32) {
        if self.root == NONE {
            return (NONE, NONE);
        }
        let loc = self.locate(x, None);
        if loc.exit == NONE {
            // The largest smaller key, if any, is the last leaf of the
            // previous child
            let children = self.children(loc.parex);
            let (i, _) = children.position(sym_at(x, loc.lcp));
            return if i == 0 {
                let succ = self.extreme_leaf(children.first(), Side::Left);
                (self.leaf(succ).prev, NONE)
            } else {
                (self.extreme_leaf(children.ids[i - 1], Side::Right), NONE)
            };
        }
        let extent = self.extent(loc.exit);
        if loc.lcp == x.len() && is_leaf(loc.exit) && extent.len() == x.len() {
            return (self.leaf(loc.exit).prev, loc.exit);
        }
        if sym_at(x, loc.lcp) < sym_at(extent, loc.lcp) {
            let succ = self.extreme_leaf(loc.exit, Side::Left);
            (self.leaf(succ).prev, NONE)
        } else {
            (self.extreme_leaf(loc.exit, Side::Right), NONE)
        }
    }

    #[inline(always)]
    fn key_of(&self, leaf: u32) -> Option<&[u8]> {
        (leaf != NONE).then(|| self.leaf_key(leaf))
    }

    /// Returns the leaf following the given one, where [`NONE`] is taken to
    /// precede the first leaf.
    #[inline(always)]
    fn next_leaf(&self, leaf: u32) -> u32 {
        if leaf == NONE {
            self.first
        } else {
            self.leaf(leaf).next
        }
    }

    /// Returns the largest key that is smaller than or equal to `x`, if any.
    #[inline]
    pub fn pred(&self, x: impl AsRef<[u8]>) -> Option<&[u8]> {
        let (pred, found) = self.pred_leaf(x.as_ref());
        self.key_of(if found != NONE { found } else { pred })
    }

    /// Returns the largest key that is smaller than `x`, if any.
    #[inline]
    pub fn pred_strict(&self, x: impl AsRef<[u8]>) -> Option<&[u8]> {
        self.key_of(self.pred_leaf(x.as_ref()).0)
    }

    /// Returns the smallest key that is larger than or equal to `x`, if any.
    #[inline]
    pub fn succ(&self, x: impl AsRef<[u8]>) -> Option<&[u8]> {
        let (pred, found) = self.pred_leaf(x.as_ref());
        self.key_of(if found != NONE {
            found
        } else {
            self.next_leaf(pred)
        })
    }

    /// Returns the smallest key that is larger than `x`, if any.
    #[inline]
    pub fn succ_strict(&self, x: impl AsRef<[u8]>) -> Option<&[u8]> {
        let (pred, found) = self.pred_leaf(x.as_ref());
        self.key_of(self.next_leaf(if found != NONE { found } else { pred }))
    }

    /// Returns the largest key that is smaller than or equal to `x`, and
    /// its value, if any.
    #[inline]
    pub(crate) fn pred_entry(&self, x: &[u8]) -> Option<(&[u8], V)> {
        let (pred, found) = self.pred_leaf(x);
        let leaf = if found != NONE { found } else { pred };
        (leaf != NONE).then(|| (self.leaf_key(leaf), self.leaf(leaf).value))
    }

    /// Returns the smallest key larger than the given one, which must be in
    /// the trie, and its value, if any.
    #[inline]
    pub(crate) fn next_entry(&self, x: &[u8]) -> Option<(&[u8], V)> {
        let (_, found) = self.pred_leaf(x);
        debug_assert!(found != NONE);
        let leaf = self.leaf(found).next;
        (leaf != NONE).then(|| (self.leaf_key(leaf), self.leaf(leaf).value))
    }

    /// Returns the smallest key, if any.
    #[inline]
    pub fn first(&self) -> Option<&[u8]> {
        self.key_of(self.first)
    }

    /// Returns the largest key, if any.
    #[inline]
    pub fn last(&self) -> Option<&[u8]> {
        self.key_of(self.last)
    }

    /// Returns an iterator on the keys in lexicographic order.
    pub fn iter(&self) -> Iter<'_, H, V> {
        Iter {
            trie: self,
            front: self.first,
            back: self.last,
            remaining: self.len,
        }
    }

    /// Returns an iterator on the keys larger than or equal to `x`, in
    /// lexicographic order.
    pub fn iter_from(&self, x: impl AsRef<[u8]>) -> impl Iterator<Item = &[u8]> + '_ {
        let (pred, found) = self.pred_leaf(x.as_ref());
        let mut leaf = if found != NONE {
            found
        } else {
            self.next_leaf(pred)
        };
        std::iter::from_fn(move || {
            (leaf != NONE).then(|| {
                let key = self.leaf_key(leaf);
                leaf = self.leaf(leaf).next;
                key
            })
        })
    }

    // Allocation

    fn new_leaf(&mut self, x: &[u8], value: V) -> u32 {
        let leaf = Leaf {
            key: Some(Key::new(x)),
            prev: NONE,
            next: NONE,
            value,
        };
        match self.free_leaves.pop() {
            Some(i) => {
                self.leaves[i as usize] = leaf;
                i | LEAF
            }
            None => {
                assert!(self.leaves.len() < (LEAF - 1) as usize, "too many keys");
                self.leaves.push(leaf);
                (self.leaves.len() - 1) as u32 | LEAF
            }
        }
    }

    fn free_leaf(&mut self, id: u32) {
        self.leaf_mut(id).key = None;
        self.free_leaves.push(id & !LEAF);
    }

    fn new_internal(&mut self, node: Internal) -> u32 {
        match self.free_internals.pop() {
            Some(i) => {
                self.internals[i as usize] = node;
                i
            }
            None => {
                self.internals.push(node);
                (self.internals.len() - 1) as u32
            }
        }
    }

    fn free_internal(&mut self, id: u32) {
        let node = &mut self.internals[id as usize];
        node.key = None;
        if node.children.len == BIG {
            let big = node.children.ids[0];
            self.big[big as usize] = BigChildren::default();
            self.free_big.push(big);
        }
        self.free_internals.push(id);
    }

    // Children

    /// Adds a child associated with a symbol to an internal node; the node
    /// must have no child associated with the same symbol.
    fn add_child(&mut self, id: u32, sym: Sym, child: u32) {
        let (i, found) = self.children(id).position(sym);
        debug_assert!(!found);
        let byte = sym.saturating_sub(1) as u8;
        let children = &mut self.internals[id as usize].children;
        if children.len == BIG {
            let big = &mut self.big[children.ids[0] as usize];
            big.ids.insert(i, child);
            big.bytes.insert(i, byte);
            big.has_end |= sym == 0;
        } else if (children.len as usize) < INLINE {
            let len = children.len as usize;
            children.ids.copy_within(i..len, i + 1);
            children.bytes.copy_within(i..len, i + 1);
            children.ids[i] = child;
            children.bytes[i] = byte;
            children.len += 1;
            children.has_end |= sym == 0;
        } else {
            let mut big = BigChildren {
                ids: Vec::with_capacity(2 * INLINE),
                bytes: Vec::with_capacity(2 * INLINE),
                has_end: children.has_end || sym == 0,
            };
            big.ids.extend_from_slice(&children.ids);
            big.bytes.extend_from_slice(&children.bytes);
            big.ids.insert(i, child);
            big.bytes.insert(i, byte);
            children.len = BIG;
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
            self.internals[id as usize].children.ids[0] = index;
        }
    }

    /// Removes the child associated with a symbol from an internal node
    /// with at least three children.
    fn remove_child(&mut self, id: u32, sym: Sym) {
        let (i, found) = self.children(id).position(sym);
        debug_assert!(found);
        let children = &mut self.internals[id as usize].children;
        if children.len == BIG {
            let index = children.ids[0];
            let big = &mut self.big[index as usize];
            big.ids.remove(i);
            big.bytes.remove(i);
            big.has_end &= sym != 0;
            if big.ids.len() <= INLINE {
                let len = big.ids.len();
                children.ids[..len].copy_from_slice(&big.ids);
                children.bytes[..len].copy_from_slice(&big.bytes);
                children.len = len as u8;
                children.has_end = big.has_end;
                *big = BigChildren::default();
                self.free_big.push(index);
            }
        } else {
            let len = children.len as usize;
            debug_assert!(len >= 3);
            children.ids.copy_within(i + 1..len, i);
            children.bytes.copy_within(i + 1..len, i);
            children.len -= 1;
            children.has_end &= sym != 0;
        }
    }

    /// Replaces the child associated with a symbol in an internal node.
    fn replace_child(&mut self, id: u32, sym: Sym, child: u32) {
        let (i, found) = self.children(id).position(sym);
        debug_assert!(found);
        let children = &mut self.internals[id as usize].children;
        if children.len == BIG {
            self.big[children.ids[0] as usize].ids[i] = child;
        } else {
            children.ids[i] = child;
        }
    }

    // Leaf list

    fn link_after(&mut self, pred: u32, leaf: u32) {
        let next = self.leaf(pred).next;
        self.leaf_mut(leaf).prev = pred;
        self.leaf_mut(leaf).next = next;
        self.leaf_mut(pred).next = leaf;
        if next == NONE {
            self.last = leaf;
        } else {
            self.leaf_mut(next).prev = leaf;
        }
    }

    fn link_before(&mut self, succ: u32, leaf: u32) {
        let prev = self.leaf(succ).prev;
        self.leaf_mut(leaf).next = succ;
        self.leaf_mut(leaf).prev = prev;
        self.leaf_mut(succ).prev = leaf;
        if prev == NONE {
            self.first = leaf;
        } else {
            self.leaf_mut(prev).next = leaf;
        }
    }

    fn unlink(&mut self, leaf: u32) {
        let (prev, next) = (self.leaf(leaf).prev, self.leaf(leaf).next);
        if prev == NONE {
            self.first = next;
        } else {
            self.leaf_mut(prev).next = next;
        }
        if next == NONE {
            self.last = prev;
        } else {
            self.leaf_mut(next).prev = prev;
        }
    }

    // Jump pointers

    /// Sets the jump pointer on the given side of an internal node by
    /// following the jump pointers of its first or last child.
    fn set_jump_from_child(&mut self, id: u32, side: Side) {
        let jump_len = self.internal(id).jump_len();
        let mut target = self.extreme_child(id, side);
        while !self.intercepts(target, jump_len) {
            target = self.jump(target, side);
        }
        self.set_jump(id, side, target);
    }

    /// Sets the jump pointers of an internal node by following the jump
    /// pointers of its first and last child.
    fn set_jumps(&mut self, id: u32) {
        self.set_jump_from_child(id, Side::Left);
        self.set_jump_from_child(id, Side::Right);
    }

    /// Fixes the jump pointers on the given side of the nodes in the stack
    /// after a leaf has been added as the new extreme child (on the same
    /// side) of a node, or of a new node that has taken the place of
    /// `old`; `old` is the previous extreme child, or the node that has been
    /// replaced, respectively.
    ///
    /// All pointers to `old`, or to a node that can be reached from `old`
    /// following jump pointers, are redirected to the leaf: the path they
    /// follow ends now at the leaf.
    fn redirect_to_leaf(&mut self, side: Side, old: u32, leaf: u32) {
        let mut desc = old;
        while let Some(id) = self.stack.pop() {
            let target = self.jump(id, side);
            while !is_leaf(desc) && desc != target {
                desc = self.jump(desc, side);
            }
            if desc != target {
                break;
            }
            self.set_jump(id, side, leaf);
        }
    }

    /// Fixes the jump pointers on the given side of the nodes in the stack
    /// after the leaf `leaf`, which was the extreme child (on the same side)
    /// of a node, has been removed; `other` is the node that is now in the
    /// same position of the path.
    ///
    /// All pointers to the leaf are redirected to the node that can be
    /// reached from `other` following jump pointers and intercepts the jump.
    fn redirect_from_leaf(&mut self, side: Side, leaf: u32, mut other: u32) {
        while let Some(id) = self.stack.pop() {
            if self.jump(id, side) != leaf {
                break;
            }
            let jump_len = self.internal(id).jump_len();
            while !self.intercepts(other, jump_len) {
                other = self.jump(other, side);
            }
            self.set_jump(id, side, other);
        }
    }

    // Updates

    /// Adds a key with an associated value, returning true if the key was
    /// not already present (otherwise, nothing happens).
    ///
    /// # Panics
    ///
    /// If the key is longer than [`MAX_KEY_LEN`](Self::MAX_KEY_LEN) bytes.
    pub(crate) fn insert_with(&mut self, x: impl AsRef<[u8]>, value: V) -> bool {
        let x = x.as_ref();
        assert!(x.len() <= Self::MAX_KEY_LEN, "key too long");
        if self.root == NONE {
            let leaf = self.new_leaf(x, value);
            self.root = leaf;
            self.first = leaf;
            self.last = leaf;
            self.len = 1;
            return true;
        }
        let mut stack = std::mem::take(&mut self.stack);
        let loc = self.locate(x, Some(&mut stack));
        self.stack = stack;
        #[cfg(debug_assertions)]
        self.check_stack(x, &loc);

        if loc.exit == NONE {
            self.insert_child(x, loc.parex, value);
        } else {
            let extent = self.extent(loc.exit);
            if is_leaf(loc.exit) && loc.lcp == x.len() && extent.len() == x.len() {
                return false;
            }
            self.insert_split(x, loc, value);
        }
        self.len += 1;
        true
    }

    /// Adds the key `x` as a new child of the node `parex`, whose extent is
    /// a prefix of `x`.
    fn insert_child(&mut self, x: &[u8], parex: u32, value: V) {
        let extent = self.internal(parex).extent_len as usize;
        let sym = sym_at(x, extent);
        let children = self.children(parex);
        let (i, _) = children.position(sym);
        let (len, first, last) = (children.ids.len(), children.first(), children.last());
        let prev = if i > 0 { children.ids[i - 1] } else { NONE };
        let leaf = self.new_leaf(x, value);

        if i == 0 {
            self.link_before(self.extreme_leaf(first, Side::Left), leaf);
        } else {
            self.link_after(self.extreme_leaf(prev, Side::Right), leaf);
        }
        self.add_child(parex, sym, leaf);
        // The stack contains the 2-fat ancestors of parex, and thus parex
        // itself
        if i == 0 {
            self.redirect_to_leaf(Side::Left, first, leaf);
        } else if i == len {
            self.redirect_to_leaf(Side::Right, last, leaf);
        }
    }

    /// Adds the key `x`, which exits at `loc.exit`, by splitting the exit
    /// node.
    fn insert_split(&mut self, x: &[u8], loc: Loc, value: V) {
        let Loc {
            parex,
            exit,
            lcp: l,
        } = loc;
        let exit_is_internal = !is_leaf(exit);
        let name_len = if parex == NONE {
            0
        } else {
            self.internal(parex).extent_len as usize + 1
        };
        // The symbol at which x and the exit node diverge
        let sym = sym_at(x, l);
        let exit_sym = sym_at(self.extent(exit), l);
        debug_assert!(sym != exit_sym);
        // The side of the new leaf with respect to the exit node
        let side = if sym < exit_sym {
            Side::Left
        } else {
            Side::Right
        };
        // If the exit node is internal and the cut is not above its handle,
        // the handle (and the associated jumps) pass to the new node, and
        // the exit node gets a new handle.
        let exit_handle_len = if exit_is_internal {
            self.internal(exit).handle_len()
        } else {
            usize::MAX
        };
        let cut_low = exit_is_internal && l >= exit_handle_len;

        // The position of the exit node among its siblings
        let (was_first, was_last) = if parex == NONE {
            (false, false)
        } else {
            let children = self.children(parex);
            (children.first() == exit, children.last() == exit)
        };

        let leaf = self.new_leaf(x, value);
        let (leaf_byte, exit_byte) = (
            sym.saturating_sub(1) as u8,
            exit_sym.saturating_sub(1) as u8,
        );
        let mut children = Children {
            ids: [NONE; INLINE],
            bytes: [0; INLINE],
            len: 2,
            has_end: sym == 0 || exit_sym == 0,
        };
        // On the side of the exit node, if the new node inherits the handle
        // of the exit node it inherits also its jump; otherwise, its jump
        // length is not larger than the handle length of the exit node.
        let inherited = |trie: &Self, side| {
            if cut_low { trie.jump(exit, side) } else { exit }
        };
        let (jump_left, jump_right) = match side {
            Side::Left => {
                children.ids[..2].copy_from_slice(&[leaf, exit]);
                children.bytes[..2].copy_from_slice(&[leaf_byte, exit_byte]);
                (leaf, inherited(self, Side::Right))
            }
            Side::Right => {
                children.ids[..2].copy_from_slice(&[exit, leaf]);
                children.bytes[..2].copy_from_slice(&[exit_byte, leaf_byte]);
                (inherited(self, Side::Left), leaf)
            }
        };
        let key = self.leaf(leaf).key.as_ref().map(Key::share);
        let internal = self.new_internal(Internal {
            key,
            name_len: name_len as u32,
            extent_len: l as u32,
            jump_left,
            jump_right,
            children,
        });
        self.max_extent = self.max_extent.max(l);

        if parex == NONE {
            self.root = internal;
        } else {
            self.replace_child(parex, sym_at(x, name_len - 1), internal);
        }

        // The leaf list
        match side {
            Side::Left => self.link_before(self.extreme_leaf(exit, Side::Left), leaf),
            Side::Right => self.link_after(self.extreme_leaf(exit, Side::Right), leaf),
        }

        // We fix the jump pointers of the 2-fat ancestors of the new node
        // that used to pass through the exit node
        if was_first || was_last {
            let exit_side = if was_first { Side::Left } else { Side::Right };
            // First, the pointers to the exit node that must now point to
            // the new node
            while let Some(&id) = self.stack.last() {
                if self.jump(id, exit_side) != exit || self.internal(id).jump_len() > l {
                    break;
                }
                self.set_jump(id, exit_side, internal);
                self.stack.pop();
            }
            // If the new leaf is on the same side, the remaining pointers
            // to the exit node, or to nodes below it, must point to the leaf
            if side == exit_side {
                self.redirect_to_leaf(exit_side, exit, leaf);
            }
        }

        // The z-map
        if cut_low {
            if parex != NONE {
                self.map
                    .replace(self.sig(x, exit_handle_len), exit, internal, l);
            }
            let node = self.internal_mut(exit);
            node.name_len = l as u32 + 1;
            let (handle_len, extent_len) = (node.handle_len(), node.extent_len as usize);
            let sig = self.sig(self.internal(exit).key(), handle_len);
            self.map.insert(sig, extent_len, exit);
            self.set_jumps(exit);
        } else {
            if exit_is_internal {
                self.internal_mut(exit).name_len = l as u32 + 1;
            }
            if parex != NONE {
                let handle_len = two_fattest(name_len - 1, l);
                self.map.insert(self.sig(x, handle_len), l, internal);
            }
        }
    }

    /// Removes a key from the set, returning true if the key was in the set.
    pub fn remove(&mut self, x: impl AsRef<[u8]>) -> bool {
        let x = x.as_ref();
        if self.root == NONE {
            return false;
        }
        let mut stack = std::mem::take(&mut self.stack);
        let loc = self.locate(x, Some(&mut stack));
        self.stack = stack;
        #[cfg(debug_assertions)]
        self.check_stack(x, &loc);
        let Loc {
            parex,
            exit: leaf,
            lcp: l,
        } = loc;
        if leaf == NONE || !is_leaf(leaf) || l != x.len() || self.leaf_key(leaf).len() != x.len() {
            return false;
        }
        self.len -= 1;
        if parex == NONE {
            let (seed, sig_mask) = (self.seed, self.sig_mask);
            self.clear();
            (self.seed, self.sig_mask) = (seed, sig_mask);
            return true;
        }
        self.unlink(leaf);

        let node = self.internal(parex);
        let (name_len, extent_len) = (node.name_len as usize, node.extent_len as usize);
        let children = self.view(&node.children);
        let (num_children, was_first, was_last) = (
            children.ids.len(),
            children.first() == leaf,
            children.last() == leaf,
        );
        let sym = sym_at(x, extent_len);

        if num_children > 2 {
            // The parent survives: we just have to fix the pointers to the
            // leaf. The stack contains the 2-fat ancestors of the parent,
            // and thus the parent itself.
            self.remove_child(parex, sym);
            if was_first {
                self.redirect_from_leaf(Side::Left, leaf, self.extreme_child(parex, Side::Left));
            } else if was_last {
                self.redirect_from_leaf(Side::Right, leaf, self.extreme_child(parex, Side::Right));
            }
            self.free_leaf(leaf);
            return true;
        }

        // The parent disappears, and the other child takes its place
        let other = if was_first {
            children.last()
        } else {
            children.first()
        };
        let other_is_internal = !is_leaf(other);
        let handle_len = self.internal(parex).handle_len();

        // We need the grandparent, and the stack of its 2-fat ancestors
        let popped = self.stack.pop();
        debug_assert_eq!(popped, Some(parex));
        let (mut parex_was_first, mut parex_was_last) = (false, false);
        if parex == self.root {
            self.root = other;
        } else {
            let mut stack = std::mem::take(&mut self.stack);
            self.complete(x, name_len - 1, &mut stack);
            let grandparent = *stack.last().unwrap();
            self.stack = stack;
            let children = self.children(grandparent);
            parex_was_first = children.first() == parex;
            parex_was_last = children.last() == parex;
            self.replace_child(grandparent, sym_at(x, name_len - 1), other);
        }

        // The z-map: the handle of the merged node is the 2-fattest between
        // the handle of the parent and that of the other node
        if other_is_internal {
            let node = self.internal(other);
            let (other_handle_len, other_extent_len) =
                (node.handle_len(), node.extent_len as usize);
            if handle_len == 0 || handle_len.trailing_zeros() > other_handle_len.trailing_zeros() {
                self.map
                    .remove(self.sig(node.key(), other_handle_len), other);
                if parex != NONE && name_len > 0 {
                    self.map
                        .replace(self.sig(x, handle_len), parex, other, other_extent_len);
                }
                self.internal_mut(other).name_len = name_len as u32;
                // The other node inherits the handle of the parent, and thus
                // its jump length. On its own side the jump pointer of the
                // parent is the right one (and copying it is necessary to
                // bound the time of a deletion); on the side of the removed
                // leaf the pointer has to be recomputed.
                let (leaf_side, other_side) = if was_first {
                    (Side::Left, Side::Right)
                } else {
                    (Side::Right, Side::Left)
                };
                let target = self.jump(parex, other_side);
                self.set_jump(other, other_side, target);
                self.set_jump_from_child(other, leaf_side);
            } else {
                self.map.remove(self.sig(x, handle_len), parex);
                self.internal_mut(other).name_len = name_len as u32;
            }
        } else if name_len > 0 {
            self.map.remove(self.sig(x, handle_len), parex);
        }

        // We fix the jump pointers of the 2-fat ancestors of the grandparent
        // that used to pass through the parent
        if parex_was_first || parex_was_last {
            let side = if parex_was_first {
                Side::Left
            } else {
                Side::Right
            };
            // First, the pointers to the parent, which must now point to the
            // other node
            while let Some(&id) = self.stack.last() {
                if self.jump(id, side) != parex {
                    break;
                }
                self.set_jump(id, side, other);
                self.stack.pop();
            }
            // If the leaf was on the same side, the pointers to the leaf
            // must point to a suitable descendant of the other node
            if (side == Side::Left) == was_first {
                self.redirect_from_leaf(side, leaf, other);
            }
        }

        self.free_internal(parex);
        self.free_leaf(leaf);
        true
    }
}

impl<H: HashPrefixes> ZFastTrie<H> {
    /// Adds a key to the set, returning true if the key was not already in
    /// the set.
    ///
    /// # Panics
    ///
    /// If the key is longer than [`MAX_KEY_LEN`](Self::MAX_KEY_LEN) bytes.
    pub fn insert(&mut self, x: impl AsRef<[u8]>) -> bool {
        self.insert_with(x, ())
    }
}

impl<H: HashPrefixes, V: Copy> fmt::Debug for ZFastTrie<H, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<H: HashPrefixes, B: AsRef<[u8]>> Extend<B> for ZFastTrie<H> {
    fn extend<I: IntoIterator<Item = B>>(&mut self, iter: I) {
        for key in iter {
            self.insert(key);
        }
    }
}

impl<H: HashPrefixes, B: AsRef<[u8]>> FromIterator<B> for ZFastTrie<H> {
    fn from_iter<I: IntoIterator<Item = B>>(iter: I) -> Self {
        let mut trie = Self::new();
        trie.extend(iter);
        trie
    }
}

impl<'a, H: HashPrefixes, V: Copy> IntoIterator for &'a ZFastTrie<H, V> {
    type Item = &'a [u8];
    type IntoIter = Iter<'a, H, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator on the keys of a [`ZFastTrie`] in lexicographic order.
pub struct Iter<'a, H, V = ()> {
    trie: &'a ZFastTrie<H, V>,
    front: u32,
    back: u32,
    remaining: usize,
}

impl<'a, H: HashPrefixes, V: Copy> Iterator for Iter<'a, H, V> {
    type Item = &'a [u8];

    #[inline]
    fn next(&mut self) -> Option<&'a [u8]> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let leaf = self.front;
        self.front = self.trie.leaf(leaf).next;
        Some(self.trie.leaf_key(leaf))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<H: HashPrefixes, V: Copy> DoubleEndedIterator for Iter<'_, H, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let leaf = self.back;
        self.back = self.trie.leaf(leaf).prev;
        Some(self.trie.leaf_key(leaf))
    }
}

impl<H: HashPrefixes, V: Copy> ExactSizeIterator for Iter<'_, H, V> {}

impl<H: HashPrefixes, V: Copy> std::iter::FusedIterator for Iter<'_, H, V> {}

#[cfg(test)]
mod tests {
    use super::*;

    /// A small test exercising all kinds of updates, meant to be run also
    /// under Miri.
    #[test]
    fn test_updates() {
        let mut trie = <ZFastTrie>::new();
        let keys: Vec<Vec<u8>> = (0..60_u32)
            .map(|i| format!("{:b}/{}", (i * 37) % 60, i % 5).into_bytes())
            .collect();
        for key in &keys {
            assert!(trie.insert(key));
        }
        trie.check();
        for key in &keys {
            assert!(trie.contains(key));
            assert_eq!(trie.pred(key), Some(key.as_slice()));
        }
        assert_eq!(trie.iter().count(), keys.len());
        for key in &keys {
            assert!(trie.remove(key));
        }
        trie.check();
        assert!(trie.is_empty());
    }
}
