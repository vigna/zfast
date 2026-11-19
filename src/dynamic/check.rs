/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

//! Invariant checking for the dynamic trie.

use super::*;
use std::collections::HashMap;

impl<H: HashPrefixes> ZFastTrie<H> {
    /// Checks all the invariants of the structure, recomputing naively
    /// names, extents, handles, signatures, and jump pointers.
    ///
    /// This method is very slow, and it is meant to be used in tests.
    ///
    /// # Panics
    ///
    /// If some invariant is violated.
    #[doc(hidden)]
    pub fn check(&self) {
        self.map.check();
        if self.root == NONE {
            assert_eq!(self.len, 0);
            assert_eq!(self.first, NONE);
            assert_eq!(self.last, NONE);
            assert_eq!(self.map.len(), 0);
            return;
        }

        let mut slots = HashMap::new();
        for (sig, extent, id, _) in self.map.iter() {
            assert!(
                slots.insert(id, (sig, extent)).is_none(),
                "node {id} appears twice in the z-map"
            );
        }

        // A visit in preorder: for each node we know the length of its name
        let mut leaves = Vec::new();
        let mut num_internals = 0;
        let mut num_big = 0;
        let mut todo = vec![(self.root, 0_usize)];
        while let Some((id, name_len)) = todo.pop() {
            if is_leaf(id) {
                assert!(self.leaf(id).key.is_some(), "free leaf {id:x} in the trie");
                leaves.push(id);
                continue;
            }
            num_internals += 1;
            let node = self.internal(id);
            assert!(node.key.is_some(), "free node {id} in the trie");
            assert_eq!(node.name_len as usize, name_len, "name length of node {id}");
            let extent_len = node.extent_len as usize;
            assert!(extent_len >= name_len, "skip interval of node {id}");
            assert!(extent_len <= self.max_extent);
            let extent = node.extent();
            let children = self.view(&node.children);
            assert!(
                children.ids.len() >= 2,
                "node {id} has less than two children"
            );
            assert_eq!(children.ids.len(), children.bytes.len());
            if node.children.len == BIG {
                num_big += 1;
                assert!(children.ids.len() > INLINE);
            }
            for (i, &child) in children.ids.iter().enumerate() {
                let sym = children.sym(i);
                if i > 0 {
                    assert!(
                        children.sym(i - 1) < sym,
                        "children of node {id} out of order"
                    );
                }
                let child_extent = self.extent(child);
                assert!(child_extent.len() >= extent_len);
                assert_eq!(&child_extent[..extent_len], extent, "extent of node {id}");
                assert_eq!(
                    sym_at(child_extent, extent_len),
                    sym,
                    "symbol of child {i} of node {id}"
                );
                assert_eq!(children.position(sym), (i, true));
            }

            // Jump pointers
            let jump_len = node.jump_len();
            for side in [Side::Left, Side::Right] {
                let mut target = self.extreme_child(id, side);
                while !is_leaf(target) && jump_len > self.internal(target).extent_len as usize {
                    target = self.extreme_child(target, side);
                }
                assert_eq!(self.jump(id, side), target, "{side:?} jump of node {id}");
            }

            // The z-map
            if id == self.root {
                assert!(!slots.contains_key(&id), "the root is in the z-map");
            } else {
                let handle_len = node.handle_len();
                assert!(handle_len >= name_len && handle_len <= extent_len);
                let slot = slots.remove(&id);
                assert_eq!(
                    slot,
                    Some((self.sig(extent, handle_len), extent_len)),
                    "slot of node {id}"
                );
            }

            // Children are pushed in reverse order so that leaves are visited
            // in order
            for &child in children.ids.iter().rev() {
                todo.push((child, extent_len + 1));
            }
        }
        assert!(slots.is_empty(), "spurious slots: {slots:?}");

        // Leaves and their list
        assert_eq!(leaves.len(), self.len);
        assert_eq!(
            num_internals + self.free_internals.len(),
            self.internals.len()
        );
        assert_eq!(leaves.len() + self.free_leaves.len(), self.leaves.len());
        assert_eq!(num_big + self.free_big.len(), self.big.len());
        assert_eq!(self.first, leaves[0]);
        assert_eq!(self.last, leaves[leaves.len() - 1]);
        for (i, &leaf) in leaves.iter().enumerate() {
            let prev = if i == 0 { NONE } else { leaves[i - 1] };
            let next = leaves.get(i + 1).copied().unwrap_or(NONE);
            assert_eq!(self.leaf(leaf).prev, prev);
            assert_eq!(self.leaf(leaf).next, next);
            if i > 0 {
                assert!(
                    self.leaf_key(prev) < self.leaf_key(leaf),
                    "leaves out of order"
                );
            }
        }
        for &i in &self.free_leaves {
            assert!(self.leaves[i as usize].key.is_none());
        }
        for &i in &self.free_internals {
            assert!(self.internals[i as usize].key.is_none());
        }
    }

    /// Checks that the result of a search with stack for `x` is the same
    /// that one would obtain by walking down the trie, and that the stack
    /// contains exactly the 2-fat ancestors of the parent of the exit node.
    #[cfg(debug_assertions)]
    pub(super) fn check_stack(&self, x: &[u8], loc: &Loc) {
        let mut stack = Vec::new();
        let slow = self.locate_slow(x, Some(&mut stack));
        assert_eq!(
            (loc.parex, loc.exit, loc.lcp),
            (slow.parex, slow.exit, slow.lcp),
            "search for {x:?}"
        );
        assert_eq!(self.stack, stack, "stack for {x:?}");
    }
}
