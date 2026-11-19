/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! Benchmarks for the static z-fast trie, with binary search on the same
//! keys as a baseline.
//!
//! The number of keys can be set with the environment variable `ZFAST_N`
//! (default: one million), and the keys with `ZFAST_KEYS`: the name of a file
//! with one key per line, or `urls` (the default), `u64`, or `u64x2`.
//!
//! ```text
//! cargo bench --bench fixed
//! ZFAST_N=10000000 ZFAST_KEYS=urls.txt cargo bench --bench fixed -- rank
//! ```

mod common;

use common::Workload;
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use zfast::utils::{Chain, HashPrefixes};
use zfast::{Dirs, KeySeq, PackedKeys, Slot, StaticZFastTrie};

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

fn bench_queries<H: HashPrefixes, S: AsRef<[Slot]>, D: Dirs>(
    c: &mut Criterion,
    name: &str,
    trie: &StaticZFastTrie<PackedKeys, H, S, D>,
    w: &Workload,
) {
    query(c, "index_of_hit", name, &w.hits, |q| trie.index_of(q));
    query(c, "index_of_miss", name, &w.misses, |q| trie.index_of(q));
    query(c, "rank_hit", name, &w.hits, |q| trie.rank(q));
    query(c, "rank_miss", name, &w.misses, |q| trie.rank(q));
    query(c, "succ_miss", name, &w.misses, |q| {
        trie.succ(q).map(|(_, key)| key.len())
    });
    // Prefixes of keys of about half the length
    query(c, "prefix_range", name, &w.hits, |q| {
        trie.prefix_range(&q[..q.len() / 2 + 1])
    });
}

fn bench_trie<H: HashPrefixes>(c: &mut Criterion, name: &str, w: &Workload) {
    let sorted = w.sorted();
    c.benchmark_group("build")
        .sample_size(10)
        .bench_function(name, |b| {
            b.iter_batched(
                || sorted.clone(),
                |sorted| <StaticZFastTrie<_, H>>::new(sorted).unwrap(),
                BatchSize::LargeInput,
            )
        });
    let trie = <StaticZFastTrie<_, H>>::new(sorted).unwrap();
    bench_queries(c, name, &trie, w);
    // The same structure with directories based on Elias-Fano
    bench_queries(c, &format!("{name} (EfDirs)"), &trie.into_ef(), w);
}

fn bench_binary_search(c: &mut Criterion, w: &Workload) {
    let name = "binary search";
    let sorted = w.sorted();
    let rank = |q: &[u8]| {
        let (mut lo, mut hi) = (0, sorted.len());
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if sorted.key(mid) < q {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    };
    query(c, "rank_hit", name, &w.hits, rank);
    query(c, "rank_miss", name, &w.misses, rank);
}

fn benches(c: &mut Criterion) {
    let n = std::env::var("ZFAST_N").map_or(1_000_000, |s| s.parse().expect("ZFAST_N"));
    let spec = std::env::var("ZFAST_KEYS").unwrap_or("urls".into());
    let w = Workload::new(&spec, n);
    bench_trie::<zfast::utils::Xxh3>(c, "StaticZFastTrie", &w);
    bench_trie::<Chain>(c, "StaticZFastTrie<Chain>", &w);
    bench_binary_search(c, &w);
}

criterion_group!(group, benches);
criterion_main!(group);
