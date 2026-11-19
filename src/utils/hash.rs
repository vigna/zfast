/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

//! Prefix hashing.
//!
//! Fat binary search needs the hash of a few prefixes of the same key. The
//! trait [`HashPrefixes`] abstracts the way such hashes are computed. There
//! are two implementations:
//!
//! - [`Xxh3`] simply hashes each prefix from scratch using
//!   [xxh3](xxhash_rust::xxh3), which is very fast on short strings;
//!
//! - [`Chain`] processes the key eight bytes at a time with a chain of folded
//!   multiplications, storing the state after each word; the hash of a prefix
//!   of length *f* is then obtained in constant time from the state after
//!   ⌊*f* / 8⌋ words, the last eight bytes of the prefix (which may overlap
//!   bytes that are already part of the state), and *f* itself.

use mem_dbg::{MemDbg, MemSize};
use xxhash_rust::xxh3::xxh3_64_with_seed;

/// A strategy to hash the prefixes of a key.
///
/// The method [`hasher`](HashPrefixes::hasher) preprocesses a key and returns
/// a [`PrefixHasher`] that can be used to hash its prefixes;
/// [`hash_prefix`](HashPrefixes::hash_prefix) hashes directly a single prefix,
/// and must return the same value.
pub trait HashPrefixes {
    /// The type of the preprocessed state of a key.
    type Hasher<'a>: PrefixHasher;

    /// Preprocesses `key` so that prefixes of length at most `limit` can be
    /// hashed.
    ///
    /// # Panics
    ///
    /// May panic if `limit` is greater than the length of the key.
    fn hasher(key: &[u8], limit: usize, seed: u64) -> Self::Hasher<'_>;

    /// Returns the hash of the prefix of length `f` of `key`.
    fn hash_prefix(key: &[u8], f: usize, seed: u64) -> u64;
}

/// The preprocessed state of a key, which makes it possible to hash its
/// prefixes.
pub trait PrefixHasher {
    /// Returns the hash of the prefix of length `f` of the key.
    ///
    /// # Panics
    ///
    /// May panic if `f` is greater than the limit provided when
    /// [preprocessing](HashPrefixes::hasher) the key.
    fn hash(&self, f: usize) -> u64;
}

/// A [prefix-hashing strategy](HashPrefixes) that hashes each prefix from
/// scratch using [xxh3](xxhash_rust::xxh3).
///
/// # Examples
///
/// ```
/// # use zfast::utils::{HashPrefixes, PrefixHasher, Xxh3};
/// let key = b"A key that is longer than a word";
/// let hasher = Xxh3::hasher(key, key.len(), 0);
/// for f in 0..=key.len() {
///     assert_eq!(hasher.hash(f), Xxh3::hash_prefix(key, f, 0));
/// }
/// ```
#[derive(Debug, Clone, Copy, Default, MemSize, MemDbg)]
#[mem_size(flat)]
pub struct Xxh3;

/// The [prefix hasher](PrefixHasher) of [`Xxh3`].
#[derive(Debug, Clone, Copy)]
pub struct Xxh3Hasher<'a> {
    key: &'a [u8],
    seed: u64,
}

impl HashPrefixes for Xxh3 {
    type Hasher<'a> = Xxh3Hasher<'a>;

    #[inline(always)]
    fn hasher(key: &[u8], _limit: usize, seed: u64) -> Xxh3Hasher<'_> {
        Xxh3Hasher { key, seed }
    }

    #[inline(always)]
    fn hash_prefix(key: &[u8], f: usize, seed: u64) -> u64 {
        xxh3_64_with_seed(&key[..f], seed)
    }
}

impl PrefixHasher for Xxh3Hasher<'_> {
    #[inline(always)]
    fn hash(&self, f: usize) -> u64 {
        xxh3_64_with_seed(&self.key[..f], self.seed)
    }
}

/// A [prefix-hashing strategy](HashPrefixes) that processes the key once,
/// eight bytes at a time, with a chain of folded multiplications, and then
/// hashes each prefix in constant time.
///
/// # Examples
///
/// ```
/// # use zfast::utils::{HashPrefixes, PrefixHasher, Chain};
/// let key = b"A key that is longer than a word";
/// let hasher = Chain::hasher(key, key.len(), 0);
/// for f in 0..=key.len() {
///     assert_eq!(hasher.hash(f), Chain::hash_prefix(key, f, 0));
/// }
/// ```
#[derive(Debug, Clone, Copy, Default, MemSize, MemDbg)]
#[mem_size(flat)]
pub struct Chain;

const K1: u64 = 0xA076_1D64_78BD_642F;
const K2: u64 = 0xE703_7ED1_A0B4_28DB;
const K3: u64 = 0x8EBC_6AF0_9C88_C6E3;
const K4: u64 = 0x5899_65CC_7537_4CC3;
const K5: u64 = 0x1D8E_4E27_C47D_124F;

/// The number of states that are kept on the stack: keys whose relevant
/// prefix is shorter than `8 * (INLINE - 1)` bytes cause no allocation.
const INLINE: usize = 33;

#[inline(always)]
const fn fold(a: u64, b: u64) -> u64 {
    let p = (a as u128).wrapping_mul(b as u128);
    (p as u64) ^ ((p >> 64) as u64)
}

/// Mixes a word into the state.
///
/// Both operands of the folded multiplication are masked with a constant, as
/// in wyhash: without masks, zero is an absorbing state, and the word made of
/// all ones is a fixed point for every multiplier.
#[inline(always)]
const fn step(state: u64, word: u64) -> u64 {
    fold(state ^ K1, word ^ K2)
}

#[inline(always)]
fn word(key: &[u8], i: usize) -> u64 {
    u64::from_le_bytes(key[i..i + 8].try_into().unwrap())
}

/// Returns the last eight bytes of the prefix of length `f` of `key`, or the
/// whole prefix padded with zeros if `f` is smaller than eight.
#[inline(always)]
fn tail(key: &[u8], f: usize) -> u64 {
    if f >= 8 {
        word(key, f - 8)
    } else {
        let mut t = 0;
        for (i, &b) in key[..f].iter().enumerate() {
            t |= (b as u64) << (8 * i);
        }
        t
    }
}

#[inline(always)]
const fn finish(state: u64, tail: u64, f: usize) -> u64 {
    fold(fold(state ^ K3, tail ^ K4) ^ f as u64, K5)
}

/// The [prefix hasher](PrefixHasher) of [`Chain`]: it contains the state of
/// the chain after each word of the key.
pub struct ChainHasher<'a> {
    key: &'a [u8],
    limit: usize,
    inline: [u64; INLINE],
    heap: Vec<u64>,
}

impl HashPrefixes for Chain {
    type Hasher<'a> = ChainHasher<'a>;

    #[inline]
    fn hash_prefix(key: &[u8], f: usize, seed: u64) -> u64 {
        let mut state = seed;
        for i in 0..f / 8 {
            state = step(state, word(key, 8 * i));
        }
        finish(state, tail(key, f), f)
    }

    #[inline]
    fn hasher(key: &[u8], limit: usize, seed: u64) -> ChainHasher<'_> {
        assert!(limit <= key.len());
        let n = limit / 8;
        let mut inline = [0; INLINE];
        let mut heap = Vec::new();
        let states = if n < INLINE {
            &mut inline[..]
        } else {
            heap.resize(n + 1, 0);
            &mut heap[..]
        };
        let mut state = seed;
        states[0] = state;
        for (i, w) in key[..8 * n].chunks_exact(8).enumerate() {
            state = step(state, u64::from_le_bytes(w.try_into().unwrap()));
            states[i + 1] = state;
        }
        ChainHasher {
            key,
            limit,
            inline,
            heap,
        }
    }
}

impl PrefixHasher for ChainHasher<'_> {
    #[inline(always)]
    fn hash(&self, f: usize) -> u64 {
        debug_assert!(f <= self.limit);
        let k = f / 8;
        let state = if self.heap.is_empty() {
            self.inline[k]
        } else {
            self.heap[k]
        };
        finish(state, tail(self.key, f), f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn check<H: HashPrefixes>() {
        let key: Vec<u8> = (0..1000_u32).map(|i| (i * 7 + i / 13) as u8).collect();
        for len in [
            0, 1, 7, 8, 9, 15, 16, 17, 255, 256, 257, 263, 264, 265, 1000,
        ] {
            let key = &key[..len];
            for limit in [0, len / 2, len] {
                let hasher = H::hasher(key, limit, 42);
                for f in 0..=limit {
                    assert_eq!(
                        hasher.hash(f),
                        H::hash_prefix(key, f, 42),
                        "len {len} f {f}"
                    );
                }
            }
        }
        // Prefixes of a constant string differ only by their length
        let zeros = [0_u8; 512];
        let ones = [0xFF_u8; 512];
        let mut seen = HashSet::new();
        for f in 0..=512 {
            assert!(seen.insert(H::hash_prefix(&zeros, f, 0)));
            if f > 0 {
                assert!(seen.insert(H::hash_prefix(&ones, f, 0)));
            }
        }
        // Low and high bits are both usable
        let low: HashSet<u16> = seen.iter().map(|&h| h as u16).collect();
        let high: HashSet<u16> = seen.iter().map(|&h| (h >> 48) as u16).collect();
        assert!(low.len() > 1000);
        assert!(high.len() > 1000);
    }

    #[test]
    fn test_xxh3() {
        check::<Xxh3>();
    }

    #[test]
    fn test_chain() {
        check::<Chain>();
    }
}
