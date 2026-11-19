/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

//! Prints statistics about fat binary search on a set of keys.
//!
//! ```text
//! cargo run --release --example stats -- FILE [N]
//! ```
//!
//! Half of the first `N` lines of the file (in random order) are used as keys,
//! the other half as unsuccessful queries.

use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use std::io::{BufRead, BufReader};
use zfast::{PackedKeys, StaticZFastTrie};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n = args.get(2).map_or(usize::MAX, |s| s.parse().unwrap());
    let reader = BufReader::new(std::fs::File::open(&args[1]).unwrap());
    let mut keys: Vec<Vec<u8>> = reader.split(b'\n').map(Result::unwrap).collect();
    keys.sort();
    keys.dedup();
    keys.shuffle(&mut SmallRng::seed_from_u64(0));
    keys.truncate(n);
    let misses = keys.split_off(keys.len() / 2);
    keys.sort();
    let packed: PackedKeys = keys.iter().collect();
    let trie = <StaticZFastTrie<_>>::new(packed).unwrap();
    for (name, queries) in [("hits", &keys), ("misses", &misses)] {
        let (mut probes, mut scanned, mut hits) = (0, 0, 0);
        for q in queries {
            let (p, s, h) = trie.search_stats(q);
            probes += p;
            scanned += s;
            hits += h;
        }
        let m = queries.len() as f64;
        println!(
            "{name}: {:.2} probes, {:.2} slots, {:.2} successful probes per query",
            probes as f64 / m,
            scanned as f64 / m,
            hits as f64 / m
        );
    }
}
