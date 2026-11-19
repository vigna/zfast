/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! Generators of adversarial sets of keys and queries shared by the tests.

#![allow(dead_code)]

use rand::rngs::SmallRng;
use rand::{Rng, RngExt, SeedableRng};
use std::collections::BTreeSet;

/// The shapes of the sets of keys we test.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// Short keys on a binary alphabet: many keys are prefixes of other keys.
    Binary,
    /// Short keys on a small alphabet.
    Small,
    /// Uniform random keys of fixed length.
    Uniform,
    /// Keys obtained by modifying other keys: long extents, deep tries.
    Clustered,
    /// Keys of the form a^i b: a caterpillar.
    Caterpillar,
    /// All prefixes of a few long keys: every internal node has a key
    /// ending at its extent.
    Prefixes,
    /// Long keys with long common prefixes.
    Long,
    /// Keys with large fan-out at every level.
    Wide,
}

pub const SHAPES: [Shape; 8] = [
    Shape::Binary,
    Shape::Small,
    Shape::Uniform,
    Shape::Clustered,
    Shape::Caterpillar,
    Shape::Prefixes,
    Shape::Long,
    Shape::Wide,
];

pub fn rng(seed: u64) -> SmallRng {
    SmallRng::seed_from_u64(seed)
}

fn random_bytes(rng: &mut impl Rng, len: usize, alphabet: u32) -> Vec<u8> {
    (0..len)
        .map(|_| {
            let c = rng.random_range(0..alphabet);
            // We want to see the extremal bytes
            match c {
                0 => 0,
                1 => 255,
                _ => c as u8,
            }
        })
        .collect()
}

/// Returns a key modified in a random way: truncated, extended, or with a
/// byte changed, possibly followed by random bytes.
pub fn mutate(rng: &mut impl Rng, key: &[u8], alphabet: u32) -> Vec<u8> {
    let mut key = key.to_vec();
    match rng.random_range(0..6) {
        0 => {
            let len = rng.random_range(0..=key.len());
            key.truncate(len);
        }
        1 => {
            let len = rng.random_range(1..4);
            key.extend(random_bytes(rng, len, alphabet));
        }
        2 if !key.is_empty() => {
            let p = rng.random_range(0..key.len());
            key[p] = key[p].wrapping_add(1);
        }
        3 if !key.is_empty() => {
            let p = rng.random_range(0..key.len());
            key[p] = key[p].wrapping_sub(1);
        }
        4 if !key.is_empty() => {
            let p = rng.random_range(0..key.len());
            key.truncate(p);
            let len = rng.random_range(0..4);
            key.extend(random_bytes(rng, len, alphabet));
        }
        _ => {
            if !key.is_empty() {
                let p = rng.random_range(0..key.len());
                key[p] = random_bytes(rng, 1, alphabet)[0];
            }
        }
    }
    key
}

/// Generates a key of the given shape, possibly using the keys generated
/// so far.
pub fn gen_key(rng: &mut impl Rng, shape: Shape, pool: &[Vec<u8>]) -> Vec<u8> {
    let pick = |rng: &mut dyn Rng| {
        if pool.is_empty() {
            Vec::new()
        } else {
            pool[rng.random_range(0..pool.len())].clone()
        }
    };
    match shape {
        Shape::Binary => {
            let len = rng.random_range(0..10);
            (0..len)
                .map(|_| rng.random_range(0..2) as u8 + b'a')
                .collect()
        }
        Shape::Small => {
            let len = rng.random_range(0..8);
            random_bytes(rng, len, 4)
        }
        Shape::Uniform => random_bytes(rng, 8, 256),
        Shape::Clustered => {
            if pool.is_empty() || rng.random_range(0..20) == 0 {
                let len = rng.random_range(0..40);
                random_bytes(rng, len, 256)
            } else {
                let key = pick(rng);
                mutate(rng, &key, 256)
            }
        }
        Shape::Caterpillar => {
            let (a, b) = if rng.random() {
                (b'a', b'b')
            } else {
                (b'b', b'a')
            };
            let len = rng.random_range(0..70);
            let mut key = vec![a; len];
            if rng.random_range(0..3) > 0 {
                key.push(b);
            }
            key
        }
        Shape::Prefixes => {
            if pool.len() < 3 {
                let len = rng.random_range(20..60);
                random_bytes(rng, len, 3)
            } else {
                let mut key = pick(rng);
                let len = rng.random_range(0..=key.len());
                key.truncate(len);
                if rng.random_range(0..4) == 0 {
                    key.extend(random_bytes(rng, 1, 3));
                }
                key
            }
        }
        Shape::Long => {
            if pool.is_empty() || rng.random_range(0..10) == 0 {
                let len = rng.random_range(250..700);
                random_bytes(rng, len, 256)
            } else {
                let mut key = pick(rng);
                let p = rng.random_range(0..=key.len());
                key.truncate(p);
                let len = rng.random_range(0..300);
                key.extend(random_bytes(rng, len, 256));
                key
            }
        }
        Shape::Wide => {
            let len = rng.random_range(1..4);
            random_bytes(rng, len, 256)
        }
    }
}

/// Generates a sorted set of about `n` keys of the given shape.
pub fn gen_set(rng: &mut impl Rng, shape: Shape, n: usize) -> Vec<Vec<u8>> {
    let mut pool = Vec::new();
    let mut set = BTreeSet::new();
    for _ in 0..n {
        let key = gen_key(rng, shape, &pool);
        if set.insert(key.clone()) {
            pool.push(key);
        }
    }
    set.into_iter().collect()
}

/// Generates a query for a set of keys of the given shape: a key, a
/// modified key, or a fresh key.
pub fn gen_query(rng: &mut impl Rng, shape: Shape, keys: &[Vec<u8>]) -> Vec<u8> {
    if keys.is_empty() {
        return gen_key(rng, shape, keys);
    }
    let key = &keys[rng.random_range(0..keys.len())];
    match rng.random_range(0..4) {
        0 => key.clone(),
        1 | 2 => mutate(rng, key, if shape == Shape::Binary { 2 } else { 256 }),
        _ => gen_key(rng, shape, keys),
    }
}
