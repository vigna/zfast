/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! Driver for the comparative benchmarks: generates workloads and runs
//! them on the structures of the `zfast` crate and on the baselines from the
//! Rust standard library.
//!
//! ```text
//! zfast-bench gen KEYS N PREFIX [QUERIES]
//! zfast-bench run METHOD PREFIX REPS
//! ```
//!
//! `gen` writes the files `PREFIX.keys`, `PREFIX.hits` and `PREFIX.misses`:
//! each file is a sequence of records formed by a 32-bit little-endian
//! length followed by that many bytes. `KEYS` is the name of a file with one
//! key per line, or `urls`, `u64`, `u64x2` (see `crate/benches/common`); the
//! first file contains N / 2 distinct keys in random order, the second one the
//! same keys in a different random order, and the third one as many strings
//! from the same source that are not keys. If `QUERIES` is given, the second
//! and the third file contain at most that many strings (the second one is
//! then a random sample of the keys): this is necessary on very large sets.
//!
//! `run` executes a fixed sequence of phases, printing for each one a line
//!
//! ```text
//! RESULT method=M op=OP n=N ns=T checksum=C
//! ```
//!
//! where T is the time in nanoseconds per operation (the best of `REPS`
//! repetitions for queries), and C a checksum that must be the same for
//! all methods. Drivers for other structures (e.g., `wh_bench.c`) read the
//! same files and print the same lines.

#[path = "../../../crate/benches/common/mod.rs"]
mod common;

use common::Workload;
use mem_dbg::{MemSize, SizeFlags};
use std::collections::BTreeSet;
use std::hint::black_box;
use std::io::Write;
use std::ops::Bound::{Included, Unbounded};
use std::time::Instant;
use zfast::utils::{Chain, HashPrefixes, Xxh3};
use zfast::{KeySeq, PackedKeys, StaticZFastTrie, ZFastPagedSet, ZFastSet, ZFastTrie};

/// Writes a sequence of keys as length-prefixed records.
fn write(keys: &PackedKeys, path: &str) -> std::io::Result<()> {
    let mut out = std::io::BufWriter::new(std::fs::File::create(path)?);
    for key in keys.iter() {
        out.write_all(&(key.len() as u32).to_le_bytes())?;
        out.write_all(key)?;
    }
    out.flush()
}

/// Reads a sequence of length-prefixed records.
fn read(path: &str) -> PackedKeys {
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("cannot read {path}: {e}"));
    let mut keys = PackedKeys::new();
    let mut pos = 0;
    while pos < bytes.len() {
        let len = u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap()) as usize;
        keys.push(&bytes[pos + 4..pos + 4 + len]);
        pos += 4 + len;
    }
    keys.into_boxed()
}

/// Returns the resident set size of the process in bytes (zero if it is
/// not available).
fn rss() -> usize {
    std::fs::read_to_string("/proc/self/statm")
        .ok()
        .and_then(|s| s.split_whitespace().nth(1)?.parse::<usize>().ok())
        .map_or(0, |pages| pages * 4096)
}

struct Runner<'a> {
    method: &'a str,
    reps: usize,
    /// If nonempty, only the query phases whose name contains this string
    /// are run (useful for profiling).
    only: String,
}

impl Runner<'_> {
    /// Runs `f` on all queries, printing the best time per query over the
    /// given number of repetitions and the sum of the results.
    fn time(&self, op: &str, reps: usize, queries: &PackedKeys, mut f: impl FnMut(&[u8]) -> usize) {
        if reps > 1 && !op.contains(&self.only) {
            return;
        }
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
        self.result(op, queries.len(), best, black_box(sum));
    }

    fn result(&self, op: &str, n: usize, ns: f64, checksum: usize) {
        println!(
            "RESULT method={} op={op} n={n} ns={ns:.1} checksum={checksum}",
            self.method
        );
    }

    fn space(&self, n: usize, bytes: usize, rss: usize) {
        println!(
            "SPACE method={} n={n} bytes_per_key={:.1} rss_per_key={:.1}",
            self.method,
            bytes as f64 / n as f64,
            rss as f64 / n as f64
        );
    }
}

fn run_trie<H: HashPrefixes>(
    r: &Runner,
    keys: &PackedKeys,
    hits: &PackedKeys,
    misses: &PackedKeys,
) {
    let before = rss();
    let mut trie = <ZFastTrie<H>>::new();
    r.time("insert", 1, keys, |k| trie.insert(k) as usize);
    r.space(keys.len(), trie.mem_size(), rss().saturating_sub(before));
    r.time("contains-hit", r.reps, hits, |k| trie.contains(k) as usize);
    r.time("contains-miss", r.reps, misses, |k| {
        trie.contains(k) as usize
    });
    r.time("succ-hit", r.reps, hits, |k| {
        trie.succ(k).map_or(0, <[u8]>::len)
    });
    r.time("succ-miss", r.reps, misses, |k| {
        trie.succ(k).map_or(0, <[u8]>::len)
    });
    let start = Instant::now();
    let sum = trie.iter().map(<[u8]>::len).sum::<usize>();
    r.result(
        "iterate",
        keys.len(),
        start.elapsed().as_nanos() as f64 / keys.len() as f64,
        sum,
    );
    r.time("remove", 1, hits, |k| trie.remove(k) as usize);
}

fn run_set<H: HashPrefixes>(r: &Runner, keys: &PackedKeys, hits: &PackedKeys, misses: &PackedKeys) {
    let before = rss();
    let mut set = <ZFastSet<H>>::new();
    r.time("insert", 1, keys, |k| set.insert(k) as usize);
    r.space(keys.len(), set.mem_size(), rss().saturating_sub(before));
    r.time("contains-hit", r.reps, hits, |k| set.contains(k) as usize);
    r.time("contains-miss", r.reps, misses, |k| {
        set.contains(k) as usize
    });
    r.time("succ-hit", r.reps, hits, |k| {
        set.succ(k).map_or(0, <[u8]>::len)
    });
    r.time("succ-miss", r.reps, misses, |k| {
        set.succ(k).map_or(0, <[u8]>::len)
    });
    let start = Instant::now();
    let sum = set.iter().map(<[u8]>::len).sum::<usize>();
    r.result(
        "iterate",
        keys.len(),
        start.elapsed().as_nanos() as f64 / keys.len() as f64,
        sum,
    );
    r.time("remove", 1, hits, |k| set.remove(k) as usize);
}

fn run_paged<H: HashPrefixes>(
    r: &Runner,
    keys: &PackedKeys,
    hits: &PackedKeys,
    misses: &PackedKeys,
) {
    let before = rss();
    let mut set = <ZFastPagedSet<H>>::new();
    r.time("insert", 1, keys, |k| set.insert(k) as usize);
    r.space(keys.len(), set.mem_size(), rss().saturating_sub(before));
    eprintln!(
        "{} segments, {:.1} pages per segment, {:.1} keys per page, router: {:.1} bytes per segment",
        set.num_segments(),
        set.num_pages() as f64 / set.num_segments() as f64,
        set.len() as f64 / set.num_pages() as f64,
        set.router_mem_size() as f64 / set.num_segments() as f64
    );
    if std::env::var("ZFAST_STATS").is_ok() {
        // How many candidate segments the router proposes, and which one
        // is the right one
        for (name, queries) in [("hits", hits), ("misses", misses)] {
            let mut counts = [[0_usize; 3]; 3];
            for q in queries.iter() {
                let (n, which) = set.locate_stats(q);
                counts[n][which.map_or(2, |i| i)] += 1;
            }
            let first = queries.iter().filter(|q| set.page_stats(q).1).count();
            eprintln!(
                "{name}: candidates x (first, second, none): {counts:?}; first page right: {first}"
            );
        }
    }
    if std::env::var("ZFAST_STATS").is_ok() {
        // The cost of the router alone, and of finding the page
        r.time("route-hit", r.reps, hits, |k| set.route(k) as usize);
        r.time("route-miss", r.reps, misses, |k| set.route(k) as usize);
        r.time("page-hit", r.reps, hits, |k| set.page_stats(k).0 as usize);
        r.time("page-miss", r.reps, misses, |k| {
            set.page_stats(k).0 as usize
        });
    }
    r.time("contains-hit", r.reps, hits, |k| set.contains(k) as usize);
    r.time("contains-miss", r.reps, misses, |k| {
        set.contains(k) as usize
    });
    r.time("succ-hit", r.reps, hits, |k| {
        set.succ(k).map_or(0, |k| k.len())
    });
    r.time("succ-miss", r.reps, misses, |k| {
        set.succ(k).map_or(0, |k| k.len())
    });
    let start = Instant::now();
    let sum = set.iter().map(|k| k.len()).sum::<usize>();
    r.result(
        "iterate",
        keys.len(),
        start.elapsed().as_nanos() as f64 / keys.len() as f64,
        sum,
    );
    r.time("remove", 1, hits, |k| set.remove(k) as usize);
}

fn run_btree(r: &Runner, keys: &PackedKeys, hits: &PackedKeys, misses: &PackedKeys) {
    let before = rss();
    let mut set = BTreeSet::<Box<[u8]>>::new();
    r.time("insert", 1, keys, |k| set.insert(k.into()) as usize);
    r.space(keys.len(), 0, rss().saturating_sub(before));
    let succ = |set: &BTreeSet<Box<[u8]>>, k: &[u8]| {
        set.range::<[u8], _>((Included(k), Unbounded))
            .next()
            .map_or(0, |s| s.len())
    };
    r.time("contains-hit", r.reps, hits, |k| set.contains(k) as usize);
    r.time("contains-miss", r.reps, misses, |k| {
        set.contains(k) as usize
    });
    r.time("succ-hit", r.reps, hits, |k| succ(&set, k));
    r.time("succ-miss", r.reps, misses, |k| succ(&set, k));
    let start = Instant::now();
    let sum = set.iter().map(|k| k.len()).sum::<usize>();
    r.result(
        "iterate",
        keys.len(),
        start.elapsed().as_nanos() as f64 / keys.len() as f64,
        sum,
    );
    r.time("remove", 1, hits, |k| set.remove(k) as usize);
}

/// Returns the given keys in sorted order.
fn sorted(keys: &PackedKeys) -> PackedKeys {
    let mut sorted: Vec<&[u8]> = keys.iter().collect();
    sorted.sort_unstable();
    sorted.into_iter().collect()
}

fn run_static<H: HashPrefixes>(
    r: &Runner,
    keys: &PackedKeys,
    hits: &PackedKeys,
    misses: &PackedKeys,
    ratio: usize,
    ef: bool,
) {
    let packed = sorted(keys);
    let key_bytes = packed.mem_size(SizeFlags::default());
    let start = Instant::now();
    let trie = <StaticZFastTrie<_, H>>::with_ratio(packed, ratio).unwrap();
    macro_rules! phases {
        ($trie:expr) => {
            let trie = $trie;
            r.result(
                "build",
                keys.len(),
                start.elapsed().as_nanos() as f64 / keys.len() as f64,
                trie.len(),
            );
            r.space(
                keys.len(),
                trie.mem_size(SizeFlags::default()) - key_bytes,
                0,
            );
            r.time("contains-hit", r.reps, hits, |k| trie.contains(k) as usize);
            r.time("contains-miss", r.reps, misses, |k| {
                trie.contains(k) as usize
            });
            r.time("succ-hit", r.reps, hits, |k| {
                trie.succ(k).map_or(0, |s| s.1.len())
            });
            r.time("succ-miss", r.reps, misses, |k| {
                trie.succ(k).map_or(0, |s| s.1.len())
            });
        };
    }
    if ef {
        phases!(trie.into_ef());
    } else {
        phases!(trie);
    }
}

fn run_bsearch(r: &Runner, keys: &PackedKeys, hits: &PackedKeys, misses: &PackedKeys) {
    let packed = sorted(keys);
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
    let contains = |x: &[u8]| {
        let rank = rank(x);
        (rank < n && packed.key(rank) == x) as usize
    };
    let succ = |x: &[u8]| {
        let rank = rank(x);
        if rank < n { packed.key(rank).len() } else { 0 }
    };
    r.space(n, 0, 0);
    r.time("contains-hit", r.reps, hits, contains);
    r.time("contains-miss", r.reps, misses, contains);
    r.time("succ-hit", r.reps, hits, succ);
    r.time("succ-miss", r.reps, misses, succ);
}

/// Writes a workload whose keys are the fences of the segments of a
/// [`ZFastPagedSet`] built on the keys of another workload, in random order,
/// and whose queries are those of the other workload: running a structure on
/// it measures how fast the structure would be as the top level of the set.
fn fences(prefix: &str, out: &str) -> std::io::Result<()> {
    let keys = read(&format!("{prefix}.keys"));
    let mut set = <ZFastPagedSet>::new();
    for key in keys.iter() {
        set.insert(key);
    }
    // The first fence is the empty string, which some structures do not support
    let mut fences: Vec<&[u8]> = set.fences().filter(|f| !f.is_empty()).collect();
    let mut state = 0x9E37_79B9_7F4A_7C15_u64;
    for i in (1..fences.len()).rev() {
        state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        fences.swap(i, ((z ^ (z >> 31)) % (i as u64 + 1)) as usize);
    }
    let mut file = std::io::BufWriter::new(std::fs::File::create(format!("{out}.keys"))?);
    for fence in &fences {
        file.write_all(&(fence.len() as u32).to_le_bytes())?;
        file.write_all(fence)?;
    }
    file.flush()?;
    for ext in ["hits", "misses"] {
        std::fs::copy(format!("{prefix}.{ext}"), format!("{out}.{ext}"))?;
    }
    eprintln!(
        "{} fences, {:.1} bytes on average",
        fences.len(),
        fences.iter().map(|f| f.len()).sum::<usize>() as f64 / fences.len() as f64
    );
    Ok(())
}

fn usage() -> ! {
    eprintln!("Usage: zfast-bench gen KEYS N PREFIX [QUERIES]");
    eprintln!("       zfast-bench fences PREFIX OUT");
    eprintln!("       zfast-bench run METHOD PREFIX REPS");
    eprintln!(
        "Methods: zpaged, zset, zft, zft-chain, btree, bsearch, szft:RATIO, szft-ef:RATIO, szft-chain:RATIO"
    );
    std::process::exit(1)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("gen") if args.len() == 5 || args.len() == 6 => {
            let n: usize = args[3].parse().unwrap_or_else(|_| usage());
            let queries: usize = match args.get(5) {
                Some(q) => q.parse().unwrap_or_else(|_| usage()),
                None => n / 2,
            };
            let w = Workload::with_queries(&args[2], n / 2, queries);
            for (keys, ext) in [(&w.keys, "keys"), (&w.hits, "hits"), (&w.misses, "misses")] {
                write(keys, &format!("{}.{ext}", args[4])).expect("cannot write workload");
            }
            eprintln!(
                "{} keys, {} unsuccessful queries",
                w.keys.len(),
                w.misses.len()
            );
        }
        Some("fences") if args.len() == 4 => {
            fences(&args[2], &args[3]).expect("cannot write workload");
        }
        Some("run") if args.len() == 5 => {
            let reps: usize = args[4].parse().unwrap_or_else(|_| usage());
            let prefix = &args[3];
            let (keys, hits, misses) = (
                read(&format!("{prefix}.keys")),
                read(&format!("{prefix}.hits")),
                read(&format!("{prefix}.misses")),
            );
            let method = args[2].as_str();
            let only = std::env::var("ZFAST_ONLY").unwrap_or_default();
            let r = Runner { method, reps, only };
            let (name, ratio) = match method.split_once(':') {
                Some((name, ratio)) => (name, ratio.parse().unwrap_or_else(|_| usage())),
                None => (method, 1),
            };
            match name {
                "zpaged" => run_paged::<Xxh3>(&r, &keys, &hits, &misses),
                "zpaged-chain" => run_paged::<Chain>(&r, &keys, &hits, &misses),
                "zset" => run_set::<Xxh3>(&r, &keys, &hits, &misses),
                "zset-chain" => run_set::<Chain>(&r, &keys, &hits, &misses),
                "zft" => run_trie::<Xxh3>(&r, &keys, &hits, &misses),
                "zft-chain" => run_trie::<Chain>(&r, &keys, &hits, &misses),
                "btree" => run_btree(&r, &keys, &hits, &misses),
                "bsearch" => run_bsearch(&r, &keys, &hits, &misses),
                "szft" => run_static::<Xxh3>(&r, &keys, &hits, &misses, ratio, false),
                "szft-ef" => run_static::<Xxh3>(&r, &keys, &hits, &misses, ratio, true),
                "szft-chain" => run_static::<Chain>(&r, &keys, &hits, &misses, ratio, false),
                _ => usage(),
            }
        }
        _ => usage(),
    }
}
