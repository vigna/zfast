/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

//! The z-map of the dynamic trie: a map from signatures of handles to nodes.

/// Marks an empty slot (in the `node` field).
const EMPTY: u32 = u32::MAX;
/// The bit of the `node` field marking a slot whose signature is shared by
/// another slot.
const DUP: u32 = 1 << 31;

/// A slot of the z-map.
///
/// A slot is self-contained, in the sense that fat binary search does not
/// need to access the node: it just needs the length of its extent.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct Slot {
    /// The signature of the handle.
    sig: u64,
    /// The length of the extent of the node.
    extent: u32,
    /// The index of the (internal) node, possibly with the [`DUP`] bit set,
    /// or [`EMPTY`].
    node: u32,
}

const EMPTY_SLOT: Slot = Slot {
    sig: 0,
    extent: 0,
    node: EMPTY,
};

/// An open-addressing hash table with linear probing mapping 64-bit
/// signatures to pairs given by the length of an extent and the index of a
/// node.
///
/// The table can contain several slots with the same signature: in this case,
/// all of them have the [`DUP`] bit set, and a lookup returns such a slot only
/// if a caller-provided exact check succeeds. Slots without the bit have a
/// signature that is unique in the table. Note that when a slot is removed the
/// bit is not cleared in the remaining slots with the same signature: this
/// just causes some unnecessary exact checks.
///
/// The position of a slot depends only on its signature, so the table can be
/// resized without accessing the nodes, and deletions use backward shifting
/// (there are no tombstones).
#[derive(Debug, Clone)]
pub(super) struct HandleMap {
    slots: Box<[Slot]>,
    /// The number of bits to shift a signature to the right to obtain its
    /// home position.
    shift: u32,
    /// The number of nonempty slots.
    len: usize,
}

impl HandleMap {
    const MIN_CAPACITY: usize = 16;

    pub fn new() -> Self {
        Self::with_capacity(Self::MIN_CAPACITY)
    }

    fn with_capacity(capacity: usize) -> Self {
        debug_assert!(capacity.is_power_of_two() && capacity >= 2);
        Self {
            slots: vec![EMPTY_SLOT; capacity].into_boxed_slice(),
            shift: 64 - capacity.trailing_zeros(),
            len: 0,
        }
    }

    /// Returns the number of nonempty slots.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the number of bytes occupied by the table.
    pub fn mem_size(&self) -> usize {
        size_of_val(&*self.slots)
    }

    #[inline(always)]
    fn home(&self, sig: u64) -> usize {
        (sig >> self.shift) as usize
    }

    #[inline(always)]
    fn mask(&self) -> usize {
        self.slots.len() - 1
    }

    /// Prefetches the first slot that will be examined when looking for the
    /// given signature.
    #[inline(always)]
    pub fn prefetch(&self, sig: u64) {
        crate::utils::prefetch(self.slots.as_ptr().wrapping_add(self.home(sig)));
    }

    /// Returns the length of the extent and the index of a node whose handle
    /// has the given signature and whose extent is not shorter than `f`.
    ///
    /// If the signature appears in more than one slot, the node is returned
    /// only if `exact` returns true on its index.
    #[inline(always)]
    pub fn find(&self, sig: u64, f: usize, exact: impl Fn(u32) -> bool) -> Option<(usize, u32)> {
        let mask = self.mask();
        let mut pos = self.home(sig);
        loop {
            let slot = self.slots[pos];
            if slot.node == EMPTY {
                return None;
            }
            if slot.sig == sig && slot.extent as usize >= f {
                if slot.node & DUP == 0 {
                    return Some((slot.extent as usize, slot.node));
                }
                let node = slot.node & !DUP;
                if exact(node) {
                    return Some((slot.extent as usize, node));
                }
            }
            pos = (pos + 1) & mask;
        }
    }

    /// Adds a new slot.
    ///
    /// The node must not be already in the table.
    pub fn insert(&mut self, sig: u64, extent: usize, node: u32) {
        debug_assert!(node & DUP == 0);
        if (self.len + 1) * 3 > self.slots.len() * 2 {
            self.resize(self.slots.len() * 2);
        }
        let mask = self.mask();
        let mut pos = self.home(sig);
        let mut dup = 0;
        while self.slots[pos].node != EMPTY {
            debug_assert!(self.slots[pos].node & !DUP != node);
            if self.slots[pos].sig == sig {
                self.slots[pos].node |= DUP;
                dup = DUP;
            }
            pos = (pos + 1) & mask;
        }
        self.slots[pos] = Slot {
            sig,
            extent: extent as u32,
            node: node | dup,
        };
        self.len += 1;
    }

    /// Returns the position of the slot with given signature and node.
    ///
    /// # Panics
    ///
    /// If there is no such slot.
    #[inline]
    fn position(&self, sig: u64, node: u32) -> usize {
        let mask = self.mask();
        let mut pos = self.home(sig);
        loop {
            let slot = self.slots[pos];
            assert!(slot.node != EMPTY, "handle not found");
            if slot.sig == sig && slot.node & !DUP == node {
                return pos;
            }
            pos = (pos + 1) & mask;
        }
    }

    /// Assigns the slot with given signature and node to a new node with a
    /// new extent.
    ///
    /// # Panics
    ///
    /// If there is no such slot.
    pub fn replace(&mut self, sig: u64, node: u32, new_node: u32, new_extent: usize) {
        let pos = self.position(sig, node);
        let slot = &mut self.slots[pos];
        slot.node = new_node | (slot.node & DUP);
        slot.extent = new_extent as u32;
    }

    /// Removes the slot with given signature and node.
    ///
    /// # Panics
    ///
    /// If there is no such slot.
    pub fn remove(&mut self, sig: u64, node: u32) {
        let mask = self.mask();
        let mut hole = self.position(sig, node);
        // Backward shifting: we move into the hole the first following slot
        // whose home position does not lie cyclically after the hole
        let mut pos = hole;
        loop {
            pos = (pos + 1) & mask;
            let slot = self.slots[pos];
            if slot.node == EMPTY {
                break;
            }
            let home = self.home(slot.sig);
            // The slot can be moved if hole is cyclically in [home . . pos)
            if (pos.wrapping_sub(home) & mask) >= (pos.wrapping_sub(hole) & mask) {
                self.slots[hole] = slot;
                hole = pos;
            }
        }
        self.slots[hole] = EMPTY_SLOT;
        self.len -= 1;
    }

    fn resize(&mut self, capacity: usize) {
        let old = std::mem::replace(self, Self::with_capacity(capacity));
        let mask = self.mask();
        for slot in old.slots.iter().filter(|slot| slot.node != EMPTY) {
            let mut pos = self.home(slot.sig);
            while self.slots[pos].node != EMPTY {
                pos = (pos + 1) & mask;
            }
            self.slots[pos] = *slot;
        }
        self.len = old.len;
    }

    /// Returns an iterator on the signature, length of the extent, node, and
    /// duplicate flag of all nonempty slots.
    pub fn iter(&self) -> impl Iterator<Item = (u64, usize, u32, bool)> + '_ {
        self.slots
            .iter()
            .filter(|slot| slot.node != EMPTY)
            .map(|slot| {
                (
                    slot.sig,
                    slot.extent as usize,
                    slot.node & !DUP,
                    slot.node & DUP != 0,
                )
            })
    }

    /// Checks the invariants of the table.
    ///
    /// # Panics
    ///
    /// If some invariant is violated.
    pub fn check(&self) {
        let mask = self.mask();
        let mut len = 0;
        for (pos, slot) in self.slots.iter().enumerate() {
            if slot.node == EMPTY {
                continue;
            }
            len += 1;
            // No empty slot between the home position and the slot
            let mut p = self.home(slot.sig);
            let mut same = 0;
            while p != pos {
                assert!(self.slots[p].node != EMPTY, "unreachable slot");
                p = (p + 1) & mask;
            }
            // Duplicate flags
            p = self.home(slot.sig);
            while self.slots[p].node != EMPTY {
                same += (self.slots[p].sig == slot.sig) as usize;
                p = (p + 1) & mask;
            }
            assert!(same == 1 || slot.node & DUP != 0, "missing duplicate flag");
        }
        assert_eq!(len, self.len);
        assert!(self.len * 3 <= self.slots.len() * 2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_handle_map() {
        // A deterministic sequence of insertions and deletions with many
        // repeated signatures and wrap-arounds
        let mut map = HandleMap::new();
        let mut naive = HashMap::<u32, u64>::new();
        let mut state = 0x9E37_79B9_7F4A_7C15_u64;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for i in 0..20000 {
            let r = next();
            let node = (r % 500) as u32;
            if let Some(sig) = naive.remove(&node) {
                map.remove(sig, node);
            } else {
                // Few distinct signatures, some of them at the end of the
                // table
                let sig = match (r >> 32) % 3 {
                    0 => u64::MAX - (r >> 40) % 4,
                    1 => ((r >> 40) % 16) << 60,
                    _ => next(),
                };
                map.insert(sig, node as usize, node);
                naive.insert(node, sig);
            }
            if i % 64 == 0 {
                map.check();
            }
            assert_eq!(map.len(), naive.len());
        }
        map.check();
        for (&node, &sig) in &naive {
            assert_eq!(map.find(sig, 0, |n| n == node), Some((node as usize, node)));
            assert_eq!(map.find(sig, node as usize + 1, |n| n == node), None);
            map.replace(sig, node, node + 1000, 7);
            assert_eq!(
                map.find(sig, 0, |n| n == node + 1000),
                Some((7, node + 1000))
            );
        }
        map.check();
        assert_eq!(map.iter().count(), naive.len());
    }
}
