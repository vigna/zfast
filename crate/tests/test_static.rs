/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

mod common;

use anyhow::Result;
use common::*;
use sux::dict::SliceSeq;
use sux::traits::{IndexedDict, IndexedSeq, Pred, Succ};
use zfast::utils::{Chain, HashPrefixes, Xxh3};
use zfast::{BuildError, Dirs, KeySeq, PackedKeys, Slot, StaticZFastTrie};

/// Checks all queries on `x` against a naive implementation.
fn check_query<K: KeySeq, H: HashPrefixes, S: AsRef<[Slot]>, D: Dirs>(
    trie: &StaticZFastTrie<K, H, S, D>,
    keys: &[Vec<u8>],
    x: &[u8],
) {
    let rank = keys.partition_point(|key| key.as_slice() < x);
    let found = rank < keys.len() && keys[rank] == x;
    let end = rank + keys[rank..].partition_point(|key| key.starts_with(x));
    let upper = rank + found as usize;

    assert_eq!(trie.rank(x), rank, "rank({x:?})");
    assert_eq!(trie.index_of(x), found.then_some(rank), "index_of({x:?})");
    assert_eq!(trie.contains(x), found, "contains({x:?})");
    assert_eq!(trie.prefix_range(x), rank..end, "prefix_range({x:?})");
    let get = |i: usize| (i, keys[i].as_slice());
    let unwrap = |r: Option<(usize, K::Key<'_>)>| r.map(|(i, key)| (i, key.as_ref().to_vec()));
    let owned = |r: Option<(usize, &[u8])>| r.map(|(i, key)| (i, key.to_vec()));
    assert_eq!(
        unwrap(trie.succ(x)),
        owned((rank < keys.len()).then(|| get(rank))),
        "succ({x:?})"
    );
    assert_eq!(
        unwrap(trie.succ_strict(x)),
        owned((upper < keys.len()).then(|| get(upper))),
        "succ_strict({x:?})"
    );
    assert_eq!(
        unwrap(trie.pred(x)),
        owned((upper > 0).then(|| get(upper - 1))),
        "pred({x:?})"
    );
    assert_eq!(
        unwrap(trie.pred_strict(x)),
        owned((rank > 0).then(|| get(rank - 1))),
        "pred_strict({x:?})"
    );
}

fn check<H: HashPrefixes>(sig_bits: u32, sizes: &[usize], queries: usize) -> Result<()> {
    let mut rng = rng(sig_bits as u64);
    for shape in SHAPES {
        for &n in sizes {
            let keys = gen_set(&mut rng, shape, n);
            let packed: PackedKeys = keys.iter().collect();
            let trie = <StaticZFastTrie<_, H>>::with_sig_bits(packed, sig_bits)?;
            // The same structure with directories based on Elias-Fano
            let ef = trie.clone().into_ef();
            assert_eq!(trie.len(), keys.len());
            assert_eq!(ef.len(), keys.len());
            for (i, key) in keys.iter().enumerate() {
                assert_eq!(trie.key(i), key);
                check_query(&trie, &keys, key);
                check_query(&ef, &keys, key);
            }
            check_query(&trie, &keys, &[]);
            check_query(&ef, &keys, &[]);
            for _ in 0..queries {
                let query = gen_query(&mut rng, shape, &keys);
                check_query(&trie, &keys, &query);
                check_query(&ef, &keys, &query);
            }
        }
    }
    Ok(())
}

const SIZES: [usize; 9] = [0, 1, 2, 3, 4, 10, 50, 300, 2000];

#[test]
fn test_xxh3() -> Result<()> {
    check::<Xxh3>(32, &SIZES, 2000)
}

#[test]
fn test_chain() -> Result<()> {
    check::<Chain>(32, &SIZES, 2000)
}

#[test]
fn test_short_signatures() -> Result<()> {
    // With short signatures false positives are common, and the slots of a
    // cluster have often the same signature
    for sig_bits in [0, 1, 2, 4, 8] {
        check::<Xxh3>(sig_bits, &SIZES[..8], 500)?;
        check::<Chain>(sig_bits, &SIZES[..8], 500)?;
    }
    Ok(())
}

#[cfg(feature = "slow_tests")]
#[test]
fn test_large() -> Result<()> {
    check::<Xxh3>(32, &[100_000, 1_000_000], 1_000_000)?;
    check::<Chain>(32, &[100_000], 1_000_000)?;
    check::<Xxh3>(12, &[100_000], 1_000_000)
}

#[test]
fn test_unsorted() {
    let keys: PackedKeys = ["b", "a"].into_iter().collect();
    assert_eq!(
        <StaticZFastTrie<_>>::new(keys).err(),
        Some(BuildError::Unsorted(1))
    );
    let keys: PackedKeys = ["a", "b", "b"].into_iter().collect();
    assert_eq!(
        <StaticZFastTrie<_>>::new(keys).err(),
        Some(BuildError::Unsorted(2))
    );
    let keys: PackedKeys = ["a", "ab", "a"].into_iter().collect();
    assert_eq!(
        <StaticZFastTrie<_>>::new(keys).err(),
        Some(BuildError::Unsorted(2))
    );
    let keys: PackedKeys = ["ab", "a"].into_iter().collect();
    assert_eq!(
        <StaticZFastTrie<_>>::new(keys).err(),
        Some(BuildError::Unsorted(1))
    );
}

#[test]
fn test_load_factors() -> Result<()> {
    let mut rng = rng(0);
    let keys = gen_set(&mut rng, Shape::Clustered, 3000);
    for load_factor in [0.1, 0.5, 0.9, 0.99] {
        let packed: PackedKeys = keys.iter().collect();
        let trie = <StaticZFastTrie<_>>::with_load_factor(packed, load_factor)?;
        for _ in 0..3000 {
            check_query(&trie, &keys, &gen_query(&mut rng, Shape::Clustered, &keys));
        }
    }
    Ok(())
}

#[test]
fn test_sux_traits() -> Result<()> {
    // A sequence from sux as backend, and the dictionary traits from sux
    let keys: Vec<&[u8]> = vec![b"bar", b"baz", b"foo", b"foobar"];
    let trie = <StaticZFastTrie<_>>::new(SliceSeq::from(keys))?;
    assert_eq!(IndexedSeq::len(&trie), 4);
    assert_eq!(trie.get(2), b"foo");
    assert_eq!(IndexedDict::index_of(&trie, b"baz".as_slice()), Some(1));
    assert_eq!(IndexedDict::index_of(&trie, b"ba".as_slice()), None);
    assert!(IndexedDict::contains(&trie, b"foobar".as_slice()));
    assert_eq!(
        Succ::succ(&trie, b"c".as_slice()),
        Some((2, b"foo".as_slice()))
    );
    assert_eq!(
        Succ::succ(&trie, b"foo".as_slice()),
        Some((2, b"foo".as_slice()))
    );
    assert_eq!(
        Succ::succ_strict(&trie, b"foo".as_slice()),
        Some((3, b"foobar".as_slice()))
    );
    assert_eq!(Succ::succ(&trie, b"g".as_slice()), None);
    assert_eq!(
        Pred::pred(&trie, b"foo".as_slice()),
        Some((2, b"foo".as_slice()))
    );
    assert_eq!(
        Pred::pred_strict(&trie, b"foo".as_slice()),
        Some((1, b"baz".as_slice()))
    );
    assert_eq!(Pred::pred(&trie, b"a".as_slice()), None);
    assert_eq!(Pred::rank(&trie, b"c".as_slice()), 2);
    assert_eq!(trie.into_inner().len(), 4);
    let trie = <StaticZFastTrie<_>>::new(PackedKeys::new())?;
    assert!(trie.is_empty());
    Ok(())
}

#[test]
fn test_backends() -> Result<()> {
    use sux::dict::{EfSeq, EliasFanoBuilder, RearCodedListBuilder};

    let mut rng = rng(1);
    let keys = gen_set(&mut rng, Shape::Clustered, 2000);

    // Offsets of the keys in an Elias-Fano representation
    let packed: PackedKeys = keys.iter().collect();
    let packed = unsafe {
        packed.map_offsets(|offsets| -> EfSeq {
            let mut builder = EliasFanoBuilder::new(offsets.len(), *offsets.last().unwrap());
            for &offset in offsets.iter() {
                builder.push(offset);
            }
            builder.build_with_seq()
        })
    };
    let trie = <StaticZFastTrie<_>>::new(packed)?.into_ef();

    // A rear-coded list from sux
    let mut builder = <RearCodedListBuilder<[u8], true>>::new(4);
    for key in &keys {
        builder.push(key.as_slice());
    }
    let rcl = <StaticZFastTrie<_>>::new(builder.build())?;

    for _ in 0..5000 {
        let query = gen_query(&mut rng, Shape::Clustered, &keys);
        check_query(&trie, &keys, &query);
        check_query(&rcl, &keys, &query);
    }
    Ok(())
}

#[cfg(feature = "epserde")]
#[test]
fn test_epserde() -> Result<()> {
    use epserde::deser::Deserialize;
    use epserde::prelude::Aligned16;
    use epserde::ser::Serialize;
    use epserde::utils::AlignedCursor;

    let mut rng = rng(2);
    let keys = gen_set(&mut rng, Shape::Clustered, 2000);
    let packed: PackedKeys = keys.iter().collect();
    let trie = <StaticZFastTrie<_>>::new(packed)?;

    // The structure with the default backends
    let mut cursor = <AlignedCursor<Aligned16>>::new();
    unsafe { trie.serialize(&mut cursor)? };
    let len = cursor.len();
    cursor.set_position(0);
    let eps = unsafe { <StaticZFastTrie<PackedKeys>>::read_mem(&mut cursor, len)? };
    let eps = eps.uncase();
    assert_eq!(eps.len(), keys.len());
    for _ in 0..5000 {
        check_query(eps, &keys, &gen_query(&mut rng, Shape::Clustered, &keys));
    }

    // The structure with directories based on Elias-Fano
    let ef = trie.into_ef();
    let mut cursor = <AlignedCursor<Aligned16>>::new();
    unsafe { ef.serialize(&mut cursor)? };
    let len = cursor.len();
    cursor.set_position(0);
    let eps = unsafe {
        <StaticZFastTrie<PackedKeys, Xxh3, Box<[Slot]>, zfast::EfDirs>>::read_mem(&mut cursor, len)?
    };
    let eps = eps.uncase();
    for _ in 0..5000 {
        check_query(eps, &keys, &gen_query(&mut rng, Shape::Clustered, &keys));
    }
    Ok(())
}
