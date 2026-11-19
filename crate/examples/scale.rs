/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! A plain timing harness for large-scale experiments.
//!
//! ```text
//! cargo run --release --example scale -- STRUCTURE KEYS [N] [REPS]
//! ```
//!
//! `STRUCTURE` is one of `zft`, `zft-chain`, `btree`, `szft`, `szft-chain`,
//! `bsearch`, or `all`. `KEYS` is the name of a file containing one key per
//! line, or `u64` (random 8-byte keys), `u64x2` (random 16-byte keys), or
//! `urls` (synthetic URL-like keys). `N` is the number of keys to use (half of
//! them are inserted, half are used for unsuccessful queries); `REPS` is the
//! number of repetitions of each query phase (the best time is reported).
//!
//! The output has one line per structure and operation, with the time in
//! nanoseconds per operation; the last column of the first line of each
//! structure is the space in bytes per key (including the keys).

use mem_dbg::{MemSize, SizeFlags};
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::{RngExt, SeedableRng};
use std::collections::BTreeSet;
use std::hint::black_box;
use std::io::{BufRead, BufReader};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::time::Instant;
use zfast::utils::{Chain, HashPrefixes, Xxh3};
use zfast::{KeySeq, PackedKeys, StaticZFastTrie, ZFastTrie};

type Keys = Vec<Box<[u8]>>;

/// Generates synthetic URL-like keys: a few schemes, a skewed set of hosts,
/// and paths made of a few components taken from a small dictionary.
fn synthetic_urls(n: usize, rng: &mut SmallRng) -> Keys {
    let words: Vec<String> = (0..2000)
        .map(|_| {
            let len = rng.random_range(3..10);
            (0..len)
                .map(|_| (b'a' + rng.random_range(0..26)) as char)
                .collect()
        })
        .collect();
    let hosts: Vec<String> = (0..(n / 50).max(10))
        .map(|_| {
            format!(
                "http{}://{}{}.{}.{}/",
                if rng.random_range(0..4) == 0 { "s" } else { "" },
                if rng.random_range(0..2) == 0 {
                    "www."
                } else {
                    ""
                },
                words[rng.random_range(0..words.len())],
                words[rng.random_range(0..words.len())],
                ["com", "org", "net", "eu", "it"][rng.random_range(0..5)],
            )
        })
        .collect();
    let mut keys = BTreeSet::new();
    while keys.len() < n {
        // Hosts have a skewed distribution
        let h = (rng.random::<f64>().powi(3) * hosts.len() as f64) as usize;
        let mut url = hosts[h].clone();
        for _ in 0..rng.random_range(1..6) {
            let w = (rng.random::<f64>().powi(2) * words.len() as f64) as usize;
            url.push_str(&words[w]);
            url.push('/');
        }
        url.push_str(&format!("{}.html", rng.random_range(0..1000)));
        keys.insert(url.into_bytes().into_boxed_slice());
    }
    keys.into_iter().collect()
}

fn load(spec: &str, n: usize, rng: &mut SmallRng) -> Keys {
    let mut keys: Keys = match spec {
        "u64" => {
            let set: BTreeSet<u64> = (0..n).map(|_| rng.random()).collect();
            set.into_iter().map(|x| x.to_be_bytes().into()).collect()
        }
        "u64x2" => {
            let set: BTreeSet<u128> = (0..n).map(|_| rng.random()).collect();
            set.into_iter().map(|x| x.to_be_bytes().into()).collect()
        }
        "urls" => synthetic_urls(n, rng),
        file => {
            let reader = BufReader::new(std::fs::File::open(file).expect("cannot open key file"));
            let set: BTreeSet<Box<[u8]>> = reader
                .split(b'\n')
                .map(|line| line.expect("read error").into_boxed_slice())
                .collect();
            set.into_iter().collect()
        }
    };
    keys.shuffle(rng);
    keys.truncate(n);
    keys
}

/// Runs `f` on all queries `reps` times, returning the best time in
/// nanoseconds per query and a checksum.
///
/// The queries are stored contiguously, so that reading them causes no cache
/// misses: we want to measure the structures, not the access to the queries.
fn time(queries: &PackedKeys, reps: usize, mut f: impl FnMut(&[u8]) -> usize) -> (f64, usize) {
    let mut best = f64::INFINITY;
    let mut sum = 0;
    for _ in 0..reps {
        sum = 0;
        let start = Instant::now();
        for q in queries.iter() {
            sum += f(black_box(q));
        }
        best = best.min(start.elapsed().as_nanos() as f64 / queries.len() as f64);
    }
    (best, black_box(sum))
}

/// Returns the given keys in sorted order.
fn sorted(keys: &PackedKeys) -> PackedKeys {
    let mut sorted: Vec<&[u8]> = keys.iter().collect();
    sorted.sort_unstable();
    sorted.into_iter().collect()
}

fn report(name: &str, op: &str, (ns, sum): (f64, usize), extra: &str) {
    println!("{name:12} {op:14} {ns:9.1} ns  [{sum:20}] {extra}");
}

fn bench_dynamic<H: HashPrefixes>(
    name: &str,
    ins: &PackedKeys,
    hits: &PackedKeys,
    misses: &PackedKeys,
    reps: usize,
) {
    let mut trie = <ZFastTrie<H>>::new();
    let t = time(ins, 1, |k| trie.insert(k) as usize);
    report(
        name,
        "insert",
        t,
        &format!("{:.1} B/key", trie.mem_size() as f64 / ins.len() as f64),
    );
    report(
        name,
        "contains-hit",
        time(hits, reps, |k| trie.contains(k) as usize),
        "",
    );
    report(
        name,
        "contains-miss",
        time(misses, reps, |k| trie.contains(k) as usize),
        "",
    );
    report(
        name,
        "pred-hit",
        time(hits, reps, |k| trie.pred(k).map_or(0, |p| p.len())),
        "",
    );
    report(
        name,
        "pred-miss",
        time(misses, reps, |k| trie.pred(k).map_or(0, |p| p.len())),
        "",
    );
    report(
        name,
        "succ-miss",
        time(misses, reps, |k| trie.succ_strict(k).map_or(0, |p| p.len())),
        "",
    );
    let start = Instant::now();
    let sum = trie.iter().map(|k| k.len()).sum::<usize>();
    report(
        name,
        "iterate",
        (start.elapsed().as_nanos() as f64 / ins.len() as f64, sum),
        "",
    );
    report(
        name,
        "remove-miss",
        time(misses, 1, |k| trie.remove(k) as usize),
        "",
    );
    report(
        name,
        "remove",
        time(hits, 1, |k| trie.remove(k) as usize),
        "",
    );
    assert!(trie.is_empty());
    // Sorted insertion
    let sorted = sorted(ins);
    let mut trie = <ZFastTrie<H>>::new();
    report(
        name,
        "insert-sorted",
        time(&sorted, 1, |k| trie.insert(k) as usize),
        "",
    );
}

fn bench_btree(ins: &PackedKeys, hits: &PackedKeys, misses: &PackedKeys, reps: usize) {
    let name = "btree";
    let mut set = BTreeSet::<Box<[u8]>>::new();
    let t = time(ins, 1, |k| set.insert(k.into()) as usize);
    // A rough estimate: B-tree nodes are at least half full (11 keys of 16
    // bytes per leaf, plus internal nodes), and each key is a separate
    // allocation with 16 bytes of overhead
    let bytes: usize = ins.num_bytes() + ins.len() * (16 + 24);
    report(
        name,
        "insert",
        t,
        &format!("~{:.1} B/key", bytes as f64 / ins.len() as f64),
    );
    report(
        name,
        "contains-hit",
        time(hits, reps, |k| set.contains(k) as usize),
        "",
    );
    report(
        name,
        "contains-miss",
        time(misses, reps, |k| set.contains(k) as usize),
        "",
    );
    let pred = |set: &BTreeSet<Box<[u8]>>, k: &[u8]| {
        set.range::<[u8], _>((Unbounded, Included(k)))
            .next_back()
            .map_or(0, |p| p.len())
    };
    report(name, "pred-hit", time(hits, reps, |k| pred(&set, k)), "");
    report(name, "pred-miss", time(misses, reps, |k| pred(&set, k)), "");
    report(
        name,
        "succ-miss",
        time(misses, reps, |k| {
            set.range::<[u8], _>((Excluded(k), Unbounded))
                .next()
                .map_or(0, |p| p.len())
        }),
        "",
    );
    let start = Instant::now();
    let sum = set.iter().map(|k| k.len()).sum::<usize>();
    report(
        name,
        "iterate",
        (start.elapsed().as_nanos() as f64 / ins.len() as f64, sum),
        "",
    );
    report(
        name,
        "remove-miss",
        time(misses, 1, |k| set.remove(k) as usize),
        "",
    );
    report(
        name,
        "remove",
        time(hits, 1, |k| set.remove(k) as usize),
        "",
    );
    let sorted = sorted(ins);
    let mut set = BTreeSet::<Box<[u8]>>::new();
    report(
        name,
        "insert-sorted",
        time(&sorted, 1, |k| set.insert(k.into()) as usize),
        "",
    );
}

fn bench_static<H: HashPrefixes>(
    name: &str,
    ins: &PackedKeys,
    hits: &PackedKeys,
    misses: &PackedKeys,
    reps: usize,
) {
    let packed = sorted(ins);
    let key_bytes = packed.mem_size(SizeFlags::default());
    let start = Instant::now();
    let trie = <StaticZFastTrie<_, H>>::new(packed).unwrap();
    let extra = format!(
        "{:.1} B/key index + {:.1} B/key keys, {:.3} nodes/key",
        (trie.mem_size(SizeFlags::default()) - key_bytes) as f64 / ins.len() as f64,
        key_bytes as f64 / ins.len() as f64,
        trie.num_nodes() as f64 / ins.len() as f64
    );
    report(
        name,
        "build",
        (
            start.elapsed().as_nanos() as f64 / ins.len() as f64,
            trie.len(),
        ),
        &extra,
    );
    report(
        name,
        "index_of-hit",
        time(hits, reps, |k| trie.index_of(k).unwrap_or(0)),
        "",
    );
    report(
        name,
        "index_of-miss",
        time(misses, reps, |k| trie.index_of(k).unwrap_or(0)),
        "",
    );
    report(name, "rank-hit", time(hits, reps, |k| trie.rank(k)), "");
    report(name, "rank-miss", time(misses, reps, |k| trie.rank(k)), "");
    report(
        name,
        "succ-miss",
        time(misses, reps, |k| trie.succ(k).map_or(0, |p| p.1.len())),
        "",
    );

    // The same structure with directories based on Elias-Fano
    let trie = trie.into_ef();
    let name = &format!("{name}-ef");
    let extra = format!(
        "{:.1} B/key index",
        (trie.mem_size(SizeFlags::default()) - key_bytes) as f64 / ins.len() as f64,
    );
    let t = time(hits, reps, |k| trie.index_of(k).unwrap_or(0));
    report(name, "index_of-hit", t, &extra);
    report(name, "rank-miss", time(misses, reps, |k| trie.rank(k)), "");
}

fn bench_bsearch(ins: &PackedKeys, hits: &PackedKeys, misses: &PackedKeys, reps: usize) {
    let name = "bsearch";
    let packed = sorted(ins);
    let n = packed.len();
    let rank = |x: &[u8]| {
        let (mut lo, mut hi) = (0, n);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if packed.key(mid) < x {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    };
    report(name, "rank-hit", time(hits, reps, rank), "");
    report(name, "rank-miss", time(misses, reps, rank), "");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} STRUCTURE KEYS [N] [REPS]", args[0]);
        std::process::exit(1);
    }
    let n = args
        .get(3)
        .map_or(2_000_000, |s| s.parse().expect("N must be a number"));
    let reps = args
        .get(4)
        .map_or(3, |s| s.parse().expect("REPS must be a number"));
    let mut rng = SmallRng::seed_from_u64(0);
    let keys = load(&args[2], n, &mut rng);
    let (ins, misses) = keys.split_at(keys.len() / 2);
    let mut hits = ins.to_vec();
    hits.shuffle(&mut rng);
    let (ins, hits, misses): (PackedKeys, PackedKeys, PackedKeys) = (
        ins.iter().collect(),
        hits.iter().collect(),
        misses.iter().collect(),
    );
    let avg = ins.num_bytes() as f64 / ins.len() as f64;
    println!(
        "# {} keys from {}, average length {avg:.1}, {} unsuccessful queries",
        ins.len(),
        args[2],
        misses.len()
    );
    let all = args[1] == "all";
    if all || args[1] == "zft" {
        bench_dynamic::<Xxh3>("zft", &ins, &hits, &misses, reps);
    }
    if all || args[1] == "zft-chain" {
        bench_dynamic::<Chain>("zft-chain", &ins, &hits, &misses, reps);
    }
    if all || args[1] == "btree" {
        bench_btree(&ins, &hits, &misses, reps);
    }
    if all || args[1] == "szft" {
        bench_static::<Xxh3>("szft", &ins, &hits, &misses, reps);
    }
    if all || args[1] == "szft-chain" {
        bench_static::<Chain>("szft-chain", &ins, &hits, &misses, reps);
    }
    if all || args[1] == "bsearch" {
        bench_bsearch(&ins, &hits, &misses, reps);
    }
}
