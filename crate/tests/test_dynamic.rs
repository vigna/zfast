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
use zfast::ZFastTrie;
use zfast::utils::{Chain, HashPrefixes, Xxh3};

/// Checks all queries on `x` against a `BTreeSet`.
fn check_query<H: HashPrefixes>(trie: &ZFastTrie<H>, set: &BTreeSet<Vec<u8>>, x: &[u8]) {
    let x = x.to_vec();
    assert_eq!(trie.contains(&x), set.contains(&x), "contains({x:?})");
    assert_eq!(
        trie.pred(&x),
        set.range(..=x.clone()).next_back().map(Vec::as_slice),
        "pred({x:?})"
    );
    assert_eq!(
        trie.pred_strict(&x),
        set.range(..x.clone()).next_back().map(Vec::as_slice),
        "pred_strict({x:?})"
    );
    assert_eq!(
        trie.succ(&x),
        set.range(x.clone()..).next().map(Vec::as_slice),
        "succ({x:?})"
    );
    assert_eq!(
        trie.succ_strict(&x),
        set.range((Excluded(x.clone()), Unbounded))
            .next()
            .map(Vec::as_slice),
        "succ_strict({x:?})"
    );
}

/// Checks that the trie and the set have the same content.
fn check_content<H: HashPrefixes>(trie: &ZFastTrie<H>, set: &BTreeSet<Vec<u8>>) {
    assert_eq!(trie.len(), set.len());
    assert_eq!(trie.is_empty(), set.is_empty());
    assert!(trie.iter().eq(set.iter().map(Vec::as_slice)));
    assert!(trie.iter().rev().eq(set.iter().rev().map(Vec::as_slice)));
    assert_eq!(trie.iter().len(), set.len());
    assert_eq!(trie.first(), set.first().map(Vec::as_slice));
    assert_eq!(trie.last(), set.last().map(Vec::as_slice));
}

/// Performs `ops` random insertions and deletions on a trie whose size
/// oscillates between zero and `max_size`, checking after each operation the
/// invariants of the structure (if `check` is true) and a few queries.
fn stress<H: HashPrefixes>(
    seed: u64,
    shape: Shape,
    sig_bits: u32,
    ops: usize,
    max_size: usize,
    check: bool,
) {
    let mut rng = rng(seed);
    let mut trie = <ZFastTrie<H>>::with_sig_bits(sig_bits);
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
            assert!(
                trie.iter_from(&from)
                    .eq(set.range(from.clone()..).map(Vec::as_slice))
            );
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

fn stress_all<H: HashPrefixes>(sig_bits: u32, ops: usize, max_size: usize) {
    for (i, shape) in SHAPES.into_iter().enumerate() {
        stress::<H>(
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
    stress_all::<Xxh3>(64, 3000, 12);
}

#[test]
fn test_small_chain() {
    stress_all::<Chain>(64, 3000, 12);
}

#[test]
fn test_medium_xxh3() {
    stress_all::<Xxh3>(64, 4000, 300);
}

#[test]
fn test_medium_chain() {
    stress_all::<Chain>(64, 4000, 300);
}

#[test]
fn test_short_signatures() {
    // With short signatures false positives are common, and many handles
    // have the same signature
    for sig_bits in [0, 1, 2, 4, 8] {
        stress_all::<Xxh3>(sig_bits, 1500, 60);
        stress_all::<Chain>(sig_bits, 1500, 60);
    }
}

#[test]
fn test_no_check() {
    // Larger tries, without checking invariants at each operation
    for (i, shape) in SHAPES.into_iter().enumerate() {
        stress::<Xxh3>(1000 + i as u64, shape, 64, 30000, 5000, false);
        stress::<Xxh3>(2000 + i as u64, shape, 10, 30000, 5000, false);
    }
}

#[cfg(feature = "slow_tests")]
#[test]
fn test_large() {
    for (i, shape) in SHAPES.into_iter().enumerate() {
        stress::<Xxh3>(3000 + i as u64, shape, 64, 2_000_000, 300_000, false);
        stress::<Chain>(4000 + i as u64, shape, 64, 500_000, 100_000, false);
        stress::<Xxh3>(5000 + i as u64, shape, 4, 200_000, 2_000, true);
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
