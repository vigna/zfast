/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! Sets of keys for benchmarks.

#![allow(dead_code)]

use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::{RngExt, SeedableRng};
use std::collections::BTreeSet;
use std::io::{BufRead, BufReader};
use zfast::PackedKeys;

/// Generates `n` distinct synthetic URL-like keys, in random order: a few
/// schemes, a skewed set of hosts, and paths made of a few components taken
/// from a small dictionary.
pub fn synthetic_urls(n: usize, rng: &mut SmallRng) -> Vec<Box<[u8]>> {
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
    let mut keys: Vec<_> = keys.into_iter().collect();
    keys.shuffle(rng);
    keys
}

/// Returns at most `n` distinct keys in random order.
///
/// `spec` is the name of a file containing one key per line, or `u64` (random
/// 8-byte keys), `u64x2` (random 16-byte keys), or `urls` (synthetic URL-like
/// keys).
pub fn load(spec: &str, n: usize, rng: &mut SmallRng) -> Vec<Box<[u8]>> {
    let mut keys: Vec<Box<[u8]>> = match spec {
        "u64" => {
            let set: BTreeSet<u64> = (0..n).map(|_| rng.random()).collect();
            set.into_iter().map(|x| x.to_be_bytes().into()).collect()
        }
        "u64x2" => {
            let set: BTreeSet<u128> = (0..n).map(|_| rng.random()).collect();
            set.into_iter().map(|x| x.to_be_bytes().into()).collect()
        }
        "urls" => return synthetic_urls(n, rng),
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

/// A workload: keys to insert, the same keys in a different order, and
/// keys that are not inserted, from the same distribution.
///
/// All sequences are stored contiguously, so that reading them causes no
/// cache misses: we want to measure the structures, not the access to the
/// queries.
pub struct Workload {
    /// The keys, in random order.
    pub keys: PackedKeys,
    /// The keys, in a different random order.
    pub hits: PackedKeys,
    /// As many strings that are not keys.
    pub misses: PackedKeys,
}

impl Workload {
    /// Creates a workload with (about) `n` keys; see [`load`] for the
    /// meaning of `spec`.
    pub fn new(spec: &str, n: usize) -> Self {
        let mut rng = SmallRng::seed_from_u64(0);
        let all = load(spec, 2 * n, &mut rng);
        let (keys, misses) = all.split_at(all.len() / 2);
        let mut hits = keys.to_vec();
        hits.shuffle(&mut rng);
        Self {
            keys: keys.iter().collect(),
            hits: hits.iter().collect(),
            misses: misses.iter().collect(),
        }
    }

    /// Returns the keys in sorted order.
    pub fn sorted(&self) -> PackedKeys {
        let mut sorted: Vec<&[u8]> = self.keys.iter().collect();
        sorted.sort_unstable();
        sorted.into_iter().collect()
    }
}
