/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

mod common;

use anyhow::Result;
use common::*;
use rand::RngExt;
use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};
use zfast::utils::{Chain, HashPrefixes, Xxh3};
use zfast::{ZFastPagedSet, ZFastSet, ZFastTrie};

/// The operations common to the dynamic structures we test; keys are
/// returned as vectors, as not all structures return slices.
trait Set {
    fn with_sig_bits(sig_bits: u32) -> Self;
    fn insert(&mut self, x: &[u8]) -> bool;
    fn remove(&mut self, x: &[u8]) -> bool;
    fn contains(&self, x: &[u8]) -> bool;
    fn pred(&self, x: &[u8]) -> Option<Vec<u8>>;
    fn pred_strict(&self, x: &[u8]) -> Option<Vec<u8>>;
    fn succ(&self, x: &[u8]) -> Option<Vec<u8>>;
    fn succ_strict(&self, x: &[u8]) -> Option<Vec<u8>>;
    fn first(&self) -> Option<Vec<u8>>;
    fn last(&self) -> Option<Vec<u8>>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn keys(&self) -> Vec<Vec<u8>>;
    fn keys_rev(&self) -> Vec<Vec<u8>>;
    fn keys_from(&self, x: &[u8]) -> Vec<Vec<u8>>;
    fn check(&self);
}

macro_rules! impl_set {
    ($ty:ident) => {
        impl<H: HashPrefixes> Set for $ty<H> {
            fn with_sig_bits(sig_bits: u32) -> Self {
                <$ty<H>>::with_sig_bits(sig_bits)
            }
            fn insert(&mut self, x: &[u8]) -> bool {
                <$ty<H>>::insert(self, x)
            }
            fn remove(&mut self, x: &[u8]) -> bool {
                <$ty<H>>::remove(self, x)
            }
            fn contains(&self, x: &[u8]) -> bool {
                <$ty<H>>::contains(self, x)
            }
            fn pred(&self, x: &[u8]) -> Option<Vec<u8>> {
                <$ty<H>>::pred(self, x).map(|k| k.to_vec())
            }
            fn pred_strict(&self, x: &[u8]) -> Option<Vec<u8>> {
                <$ty<H>>::pred_strict(self, x).map(|k| k.to_vec())
            }
            fn succ(&self, x: &[u8]) -> Option<Vec<u8>> {
                <$ty<H>>::succ(self, x).map(|k| k.to_vec())
            }
            fn succ_strict(&self, x: &[u8]) -> Option<Vec<u8>> {
                <$ty<H>>::succ_strict(self, x).map(|k| k.to_vec())
            }
            fn first(&self) -> Option<Vec<u8>> {
                <$ty<H>>::first(self).map(|k| k.to_vec())
            }
            fn last(&self) -> Option<Vec<u8>> {
                <$ty<H>>::last(self).map(|k| k.to_vec())
            }
            fn len(&self) -> usize {
                <$ty<H>>::len(self)
            }
            fn is_empty(&self) -> bool {
                <$ty<H>>::is_empty(self)
            }
            fn keys(&self) -> Vec<Vec<u8>> {
                assert_eq!(self.iter().len(), <$ty<H>>::len(self));
                self.iter().map(|k| k.to_vec()).collect()
            }
            fn keys_rev(&self) -> Vec<Vec<u8>> {
                self.iter().rev().map(|k| k.to_vec()).collect()
            }
            fn keys_from(&self, x: &[u8]) -> Vec<Vec<u8>> {
                self.iter_from(x).map(|k| k.to_vec()).collect()
            }
            fn check(&self) {
                <$ty<H>>::check(self)
            }
        }
    };
}

impl_set!(ZFastTrie);
impl_set!(ZFastSet);
impl_set!(ZFastPagedSet);

/// Checks all queries on `x` against a `BTreeSet`.
fn check_query<S: Set>(trie: &S, set: &BTreeSet<Vec<u8>>, x: &[u8]) {
    let x = x.to_vec();
    assert_eq!(trie.contains(&x), set.contains(&x), "contains({x:?})");
    assert_eq!(
        trie.pred(&x),
        set.range(..=x.clone()).next_back().cloned(),
        "pred({x:?})"
    );
    assert_eq!(
        trie.pred_strict(&x),
        set.range(..x.clone()).next_back().cloned(),
        "pred_strict({x:?})"
    );
    assert_eq!(
        trie.succ(&x),
        set.range(x.clone()..).next().cloned(),
        "succ({x:?})"
    );
    assert_eq!(
        trie.succ_strict(&x),
        set.range((Excluded(x.clone()), Unbounded)).next().cloned(),
        "succ_strict({x:?})"
    );
}

/// Checks that the trie and the set have the same content.
fn check_content<S: Set>(trie: &S, set: &BTreeSet<Vec<u8>>) {
    assert_eq!(trie.len(), set.len());
    assert_eq!(trie.is_empty(), set.is_empty());
    assert!(trie.keys().iter().eq(set.iter()));
    assert!(trie.keys_rev().iter().eq(set.iter().rev()));
    assert_eq!(trie.first(), set.first().cloned());
    assert_eq!(trie.last(), set.last().cloned());
}

/// Performs `ops` random insertions and deletions on a trie whose size
/// oscillates between zero and `max_size`, checking after each operation the
/// invariants of the structure (if `check` is true) and a few queries.
fn stress<S: Set>(
    seed: u64,
    shape: Shape,
    sig_bits: u32,
    ops: usize,
    max_size: usize,
    check: bool,
) {
    let mut rng = rng(seed);
    let mut trie = S::with_sig_bits(sig_bits);
    let mut set = BTreeSet::new();
    let mut pool = Vec::<Vec<u8>>::new();
    for i in 0..ops {
        // The target size goes up and down, so the trie is emptied and
        // refilled a few times
        let phase = i as f64 * 6.0 * std::f64::consts::PI / ops as f64;
        let target = (max_size as f64 * (0.5 - 0.5 * phase.cos())) as usize;
        let insert = set.len() < target || (set.len() == target && rng.random());
        if insert {
            let key = gen_key(&mut rng, shape, &pool);
            assert_eq!(
                trie.insert(&key),
                set.insert(key.clone()),
                "insert({key:?})"
            );
            pool.push(key);
        } else if !pool.is_empty() {
            let p = rng.random_range(0..pool.len());
            let key = if rng.random_range(0..10) == 0 {
                gen_query(&mut rng, shape, &pool)
            } else {
                pool.swap_remove(p)
            };
            assert_eq!(trie.remove(&key), set.remove(&key), "remove({key:?})");
        }
        if check {
            trie.check();
        }
        assert_eq!(trie.len(), set.len());
        for _ in 0..3 {
            check_query(&trie, &set, &gen_query(&mut rng, shape, &pool));
        }
        if i % 97 == 0 {
            check_content(&trie, &set);
            let from = gen_query(&mut rng, shape, &pool);
            assert!(trie.keys_from(&from).iter().eq(set.range(from.clone()..)));
        }
    }
    trie.check();
    check_content(&trie, &set);
    // Empty the trie
    for key in &set {
        assert!(trie.remove(key));
        if check {
            trie.check();
        }
    }
    assert!(trie.is_empty());
    trie.check();
}

fn stress_all<S: Set>(sig_bits: u32, ops: usize, max_size: usize) {
    for (i, shape) in SHAPES.into_iter().enumerate() {
        stress::<S>(
            sig_bits as u64 * 100 + i as u64,
            shape,
            sig_bits,
            ops,
            max_size,
            true,
        );
    }
}

#[test]
fn test_small_xxh3() {
    stress_all::<ZFastTrie<Xxh3>>(64, 3000, 12);
}

#[test]
fn test_small_chain() {
    stress_all::<ZFastTrie<Chain>>(64, 3000, 12);
}

#[test]
fn test_medium_xxh3() {
    stress_all::<ZFastTrie<Xxh3>>(64, 4000, 300);
}

#[test]
fn test_medium_chain() {
    stress_all::<ZFastTrie<Chain>>(64, 4000, 300);
}

#[test]
fn test_short_signatures() {
    // With short signatures false positives are common, and many handles
    // have the same signature
    for sig_bits in [0, 1, 2, 4, 8] {
        stress_all::<ZFastTrie<Xxh3>>(sig_bits, 1500, 60);
        stress_all::<ZFastTrie<Chain>>(sig_bits, 1500, 60);
    }
}

#[test]
fn test_no_check() {
    // Larger tries, without checking invariants at each operation
    for (i, shape) in SHAPES.into_iter().enumerate() {
        stress::<ZFastTrie<Xxh3>>(1000 + i as u64, shape, 64, 30000, 5000, false);
        stress::<ZFastTrie<Xxh3>>(2000 + i as u64, shape, 10, 30000, 5000, false);
    }
}

#[cfg(feature = "slow_tests")]
#[test]
fn test_large() {
    for (i, shape) in SHAPES.into_iter().enumerate() {
        stress::<ZFastTrie<Xxh3>>(3000 + i as u64, shape, 64, 2_000_000, 300_000, false);
        stress::<ZFastTrie<Chain>>(4000 + i as u64, shape, 64, 500_000, 100_000, false);
        stress::<ZFastTrie<Xxh3>>(5000 + i as u64, shape, 4, 200_000, 2_000, true);
    }
}

#[test]
fn test_order_of_insertion() {
    // Shape extremes inserted and removed in both orders
    let mut rng = rng(0);
    for shape in SHAPES {
        let keys = gen_set(&mut rng, shape, 300);
        for (insert_rev, remove_rev) in [(false, false), (false, true), (true, false), (true, true)]
        {
            let mut trie = <ZFastTrie>::new();
            let mut order: Vec<&Vec<u8>> = keys.iter().collect();
            if insert_rev {
                order.reverse();
            }
            for key in &order {
                assert!(trie.insert(key));
                trie.check();
            }
            assert!(trie.iter().eq(keys.iter().map(Vec::as_slice)));
            order.sort();
            if remove_rev {
                order.reverse();
            }
            for key in &order {
                assert!(trie.remove(key));
                trie.check();
            }
            assert!(trie.is_empty());
        }
    }
}

#[test]
fn test_basic() -> Result<()> {
    let mut trie = <ZFastTrie>::new();
    assert!(!trie.contains(""));
    assert_eq!(trie.pred("a"), None);
    assert_eq!(trie.succ("a"), None);
    assert!(!trie.remove("a"));
    assert!(trie.insert(""));
    assert!(trie.contains(""));
    assert!(!trie.insert(""));
    assert!(trie.insert("a"));
    assert!(trie.insert([0_u8]));
    assert_eq!(trie.iter().collect::<Vec<_>>(), vec![&b""[..], &[0], b"a"]);
    assert_eq!(format!("{trie:?}"), "{[], [0], [97]}");
    assert!(trie.remove(""));
    assert!(!trie.remove(""));
    assert_eq!(trie.len(), 2);
    trie.clear();
    assert!(trie.is_empty());
    trie.check();

    let trie: ZFastTrie = ["b", "a", "c", "a"].into_iter().collect();
    assert_eq!(trie.len(), 3);
    assert!(trie.mem_size() > 0);
    let trie = std::thread::spawn(move || trie).join().unwrap();
    assert_eq!((&trie).into_iter().count(), 3);
    Ok(())
}

// The same tests for the bucketed set: the sizes are larger, so that
// buckets are split and merged

#[test]
fn test_set_small() {
    stress_all::<ZFastSet<Xxh3>>(64, 3000, 12);
    stress_all::<ZFastSet<Chain>>(64, 3000, 12);
}

#[test]
fn test_set_medium() {
    stress_all::<ZFastSet<Xxh3>>(64, 6000, 1000);
    stress_all::<ZFastSet<Chain>>(64, 3000, 500);
}

#[test]
fn test_set_short_signatures() {
    for sig_bits in [0, 2, 8] {
        stress_all::<ZFastSet<Xxh3>>(sig_bits, 3000, 600);
    }
}

#[test]
fn test_set_no_check() {
    for (i, shape) in SHAPES.into_iter().enumerate() {
        stress::<ZFastSet<Xxh3>>(6000 + i as u64, shape, 64, 60000, 20000, false);
    }
}

#[test]
fn test_set_order_of_insertion() {
    let mut rng = rng(0);
    for shape in SHAPES {
        let keys = gen_set(&mut rng, shape, 2000);
        for (insert_rev, remove_rev) in [(false, false), (false, true), (true, false), (true, true)]
        {
            let mut set = <ZFastSet>::new();
            let mut order: Vec<&Vec<u8>> = keys.iter().collect();
            if insert_rev {
                order.reverse();
            }
            for key in &order {
                assert!(set.insert(key));
            }
            set.check();
            assert!(set.iter().eq(keys.iter().map(Vec::as_slice)));
            order.sort();
            if remove_rev {
                order.reverse();
            }
            for (i, key) in order.iter().enumerate() {
                assert!(set.remove(key));
                if i % 37 == 0 {
                    set.check();
                }
            }
            assert!(set.is_empty());
            set.check();
        }
    }
}

#[test]
fn test_set_basic() {
    let mut set = <ZFastSet>::new();
    assert!(!set.contains(""));
    assert_eq!(set.pred("a"), None);
    assert_eq!(set.succ("a"), None);
    assert_eq!(set.first(), None);
    assert_eq!(set.last(), None);
    assert!(!set.remove("a"));
    assert!(set.insert(""));
    assert!(set.contains(""));
    assert!(!set.insert(""));
    assert!(set.insert("a"));
    assert!(set.insert([0_u8]));
    assert_eq!(set.iter().collect::<Vec<_>>(), vec![&b""[..], &[0], b"a"]);
    assert_eq!(format!("{set:?}"), "{[], [0], [97]}");
    assert!(set.remove(""));
    assert_eq!(set.len(), 2);
    set.clear();
    assert!(set.is_empty());
    set.check();
    let set: ZFastSet = ["b", "a", "c", "a"].into_iter().collect();
    assert_eq!(set.len(), 3);
    assert!(set.mem_size() > 0);
    let set = std::thread::spawn(move || set).join().unwrap();
    assert_eq!((&set).into_iter().count(), 3);
}

// The same tests for the paged set: the sizes are larger still, so that pages
// are split and merged

#[test]
fn test_paged_small() {
    stress_all::<ZFastPagedSet<Xxh3>>(64, 3000, 12);
    stress_all::<ZFastPagedSet<Chain>>(64, 3000, 12);
}

#[test]
fn test_paged_medium() {
    stress_all::<ZFastPagedSet<Xxh3>>(64, 6000, 2000);
    stress_all::<ZFastPagedSet<Chain>>(64, 3000, 1000);
}

#[test]
fn test_paged_short_signatures() {
    for sig_bits in [0, 2, 8] {
        stress_all::<ZFastPagedSet<Xxh3>>(sig_bits, 3000, 1500);
    }
}

#[test]
fn test_paged_no_check() {
    for (i, shape) in SHAPES.into_iter().enumerate() {
        stress::<ZFastPagedSet<Xxh3>>(7000 + i as u64, shape, 64, 100000, 40000, false);
    }
}

#[test]
fn test_paged_order_of_insertion() {
    let mut rng = rng(0);
    for shape in SHAPES {
        let keys = gen_set(&mut rng, shape, 5000);
        for (insert_rev, remove_rev) in [(false, false), (false, true), (true, false), (true, true)]
        {
            let mut set = <ZFastPagedSet>::new();
            let mut order: Vec<&Vec<u8>> = keys.iter().collect();
            if insert_rev {
                order.reverse();
            }
            for key in &order {
                assert!(set.insert(key));
            }
            set.check();
            assert!(set.iter().map(|k| k.to_vec()).eq(keys.iter().cloned()));
            order.sort();
            if remove_rev {
                order.reverse();
            }
            for (i, key) in order.iter().enumerate() {
                assert!(set.remove(key));
                if i % 37 == 0 {
                    set.check();
                }
            }
            assert!(set.is_empty());
            set.check();
        }
    }
}

#[test]
fn test_paged_basic() {
    let mut set = <ZFastPagedSet>::new();
    assert!(!set.contains(""));
    assert!(set.pred("a").is_none());
    assert!(set.succ("a").is_none());
    assert!(set.first().is_none());
    assert!(set.last().is_none());
    assert!(!set.remove("a"));
    assert!(set.insert(""));
    assert!(set.contains(""));
    assert!(!set.insert(""));
    assert!(set.insert("a"));
    assert!(set.insert([0_u8]));
    assert_eq!(
        set.iter().map(|k| k.to_vec()).collect::<Vec<_>>(),
        vec![b"".to_vec(), vec![0], b"a".to_vec()]
    );
    assert_eq!(format!("{set:?}"), "{[], [0], [97]}");
    assert!(set.remove(""));
    assert_eq!(set.len(), 2);
    set.clear();
    assert!(set.is_empty());
    set.check();
    let set: ZFastPagedSet = ["b", "a", "c", "a"].into_iter().collect();
    assert_eq!(set.len(), 3);
    assert!(set.mem_size() > 0);
    let set = std::thread::spawn(move || set).join().unwrap();
    assert_eq!((&set).into_iter().count(), 3);
}
