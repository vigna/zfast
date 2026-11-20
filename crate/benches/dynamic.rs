/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! Benchmarks for the dynamic structures (`ZFastPagedSet`, `ZFastSet` and
//! `ZFastTrie`), with `BTreeSet` as a baseline.
//!
//! The number of keys can be set with the environment variable `ZFAST_N`
//! (default: one million), and the keys with `ZFAST_KEYS`: the name of a file
//! with one key per line, or `urls` (the default), `u64`, or `u64x2`.
//!
//! ```text
//! cargo bench --bench dynamic
//! ZFAST_N=10000000 ZFAST_KEYS=urls.txt cargo bench --bench dynamic -- contains
//! ```

mod common;

use common::Workload;
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use std::collections::BTreeSet;
use std::hint::black_box;
use std::ops::Bound::{Included, Unbounded};
use zfast::utils::Chain;
use zfast::{KeySeq, PackedKeys, ZFastPagedSet, ZFastSet, ZFastTrie};

/// Benchmarks a query, cycling through the given strings.
fn query<R>(
    c: &mut Criterion,
    group: &str,
    name: &str,
    queries: &PackedKeys,
    mut f: impl FnMut(&[u8]) -> R,
) {
    let mut i = 0;
    c.benchmark_group(group).bench_function(name, |b| {
        b.iter(|| {
            let q = queries.key(i);
            i = (i + 1) % queries.len();
            black_box(f(black_box(q)))
        })
    });
}

/// Benchmarks a structure of the crate: the dynamic structures have the same
/// methods, but no common trait.
macro_rules! bench {
    ($ty:ty, $c:expr, $name:expr, $w:expr) => {{
        let (c, name, w): (&mut Criterion, &str, &Workload) = ($c, $name, $w);
        c.benchmark_group("build")
            .sample_size(10)
            .bench_function(name, |b| b.iter(|| w.keys.iter().collect::<$ty>()));
        let sorted = w.sorted();
        c.benchmark_group("build_sorted")
            .sample_size(10)
            .bench_function(name, |b| b.iter(|| sorted.iter().collect::<$ty>()));

        let mut trie: $ty = w.keys.iter().collect();
        query(c, "contains_hit", name, &w.hits, |q| trie.contains(q));
        query(c, "contains_miss", name, &w.misses, |q| trie.contains(q));
        query(c, "pred_hit", name, &w.hits, |q| {
            trie.pred(q).map(|k| k.len())
        });
        query(c, "pred_miss", name, &w.misses, |q| {
            trie.pred(q).map(|k| k.len())
        });
        query(c, "succ_miss", name, &w.misses, |q| {
            trie.succ(q).map(|k| k.len())
        });
        c.benchmark_group("iter")
            .sample_size(10)
            .bench_function(name, |b| {
                b.iter(|| trie.iter().map(|k| k.len()).sum::<usize>())
            });
        // An insertion followed by a deletion, in a trie of constant size
        let mut i = 0;
        c.benchmark_group("insert_remove")
            .bench_function(name, |b| {
                b.iter(|| {
                    let q = w.misses.key(i);
                    i = (i + 1) % w.misses.len();
                    trie.insert(q);
                    trie.remove(q)
                })
            });
        // The deletion of all keys
        c.benchmark_group("remove_all")
            .sample_size(10)
            .bench_function(name, |b| {
                b.iter_batched(
                    || w.keys.iter().collect::<$ty>(),
                    |mut trie| {
                        for key in w.hits.iter() {
                            trie.remove(key);
                        }
                        trie
                    },
                    BatchSize::LargeInput,
                )
            });
    }};
}

fn bench_btree(c: &mut Criterion, w: &Workload) {
    let name = "BTreeSet";
    let build = |keys: &PackedKeys| keys.iter().map(Box::from).collect::<BTreeSet<Box<[u8]>>>();
    c.benchmark_group("build")
        .sample_size(10)
        .bench_function(name, |b| b.iter(|| build(&w.keys)));
    let sorted = w.sorted();
    c.benchmark_group("build_sorted")
        .sample_size(10)
        .bench_function(name, |b| b.iter(|| build(&sorted)));

    let mut set = build(&w.keys);
    let pred = |set: &BTreeSet<Box<[u8]>>, q: &[u8]| {
        set.range::<[u8], _>((Unbounded, Included(q)))
            .next_back()
            .map(|k| k.len())
    };
    let succ = |set: &BTreeSet<Box<[u8]>>, q: &[u8]| {
        set.range::<[u8], _>((Included(q), Unbounded))
            .next()
            .map(|k| k.len())
    };
    query(c, "contains_hit", name, &w.hits, |q| set.contains(q));
    query(c, "contains_miss", name, &w.misses, |q| set.contains(q));
    query(c, "pred_hit", name, &w.hits, |q| pred(&set, q));
    query(c, "pred_miss", name, &w.misses, |q| pred(&set, q));
    query(c, "succ_miss", name, &w.misses, |q| succ(&set, q));
    c.benchmark_group("iter")
        .sample_size(10)
        .bench_function(name, |b| {
            b.iter(|| set.iter().map(|k| k.len()).sum::<usize>())
        });
    let mut i = 0;
    c.benchmark_group("insert_remove")
        .bench_function(name, |b| {
            b.iter(|| {
                let q = w.misses.key(i);
                i = (i + 1) % w.misses.len();
                set.insert(q.into());
                set.remove(q)
            })
        });
    c.benchmark_group("remove_all")
        .sample_size(10)
        .bench_function(name, |b| {
            b.iter_batched(
                || build(&w.keys),
                |mut set| {
                    for key in w.hits.iter() {
                        set.remove(key);
                    }
                    set
                },
                BatchSize::LargeInput,
            )
        });
}

fn benches(c: &mut Criterion) {
    let n = std::env::var("ZFAST_N").map_or(1_000_000, |s| s.parse().expect("ZFAST_N"));
    let spec = std::env::var("ZFAST_KEYS").unwrap_or("urls".into());
    let w = Workload::new(&spec, n);
    bench!(ZFastPagedSet, c, "ZFastPagedSet", &w);
    bench!(ZFastSet, c, "ZFastSet", &w);
    bench!(ZFastSet<Chain>, c, "ZFastSet<Chain>", &w);
    bench!(ZFastTrie, c, "ZFastTrie", &w);
    bench!(ZFastTrie<Chain>, c, "ZFastTrie<Chain>", &w);
    bench_btree(c, &w);
}

criterion_group!(group, benches);
criterion_main!(group);
