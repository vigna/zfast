# zfast

Dynamic and static *z-fast tries* on byte strings.

A z-fast trie is a compacted trie in which a query string is located not by
walking down from the root, but by a *fat binary search* on the length of the
string: a hash table (the *z-map*) maps the signature of a suitably chosen
prefix (the *handle*) of the string associated with each internal node to the
node, and the search probes the table with a logarithmic number of prefixes of
the query. The number of steps thus depends on the logarithm of the length of
the query, and not on the number of keys or on the depth of the trie.

The data structure is described in the paper *Z-Fast Tries* by Djamal
Belazzougui, Paolo Boldi and Sebastiano Vigna, whose source is in the
directory `paper` of the repository containing this crate; the directory
`bench` contains comparative benchmarks against other data structures.

The crate is based on [`sux`], whose traits and succinct data structures it
uses and implements.

## The data structures

There are two dynamic structures and a static one:

| | | |
|---|---|---|
| `ZFastSet` | dynamic | a z-fast trie indexing buckets of keys: the structure of choice in practice |
| `ZFastTrie` | dynamic | the z-fast trie of the paper, with one leaf per key |
| `StaticZFastTrie` | static | an index over a sorted sequence of keys, possibly sampled |

All structures store sets of arbitrary byte strings, in the lexicographic
order of `[u8]`, and branch on bytes rather than on bits: a key that is a
prefix of another key is handled by imagining that each key is followed by a
terminator smaller than all bytes. Signature collisions are handled by
verifying the result of a search with a single string comparison, and by
repeating the search exactly in the (extremely unlikely) case of failure;
answers are always exact.

### `ZFastSet`

A dynamic sorted set of byte strings, with insertions, deletions, membership,
predecessor and successor queries, and bidirectional iteration. The keys are
stored in buckets of at most 64 keys, and a `ZFastTrie` indexes the buckets
through short separators: the trie is thus small, and usually fits a fast level
of the memory hierarchy even when the keys do not, so an operation usually
causes just two cache misses (a bucket and a key). Inside a bucket, 16-bit
hash tags and 8-byte order-preserving summaries of the keys make it possible
to search without accessing the keys.

```rust
use zfast::ZFastSet;

let mut set = <ZFastSet>::new();
set.insert("foo");
set.insert("bar");
set.insert("foobar");

assert!(set.contains("bar"));
assert_eq!(set.succ("c"), Some("foo".as_bytes()));
assert_eq!(set.pred("c"), Some("bar".as_bytes()));
assert!(set.remove("foo"));
assert_eq!(set.iter().count(), 2);
```

### `ZFastTrie`

The dynamic z-fast trie described in the paper, with the same operations of a
`ZFastSet`: every key is a leaf of the trie. It provides the worst-case
guarantees of the paper for every operation, but in a large trie each probe
of a search is a cache miss, and the space overhead per key is significant, so
it is usually slower and larger than a `ZFastSet`.

```rust
use zfast::ZFastTrie;

let mut trie = <ZFastTrie>::new();
trie.insert("foo");
trie.insert("bar");
trie.insert("foobar");

assert!(trie.contains("bar"));
assert_eq!(trie.succ("c"), Some("foo".as_bytes()));
assert_eq!(trie.pred("c"), Some("bar".as_bytes()));
assert!(trie.remove("foo"));
assert_eq!(trie.iter().count(), 2);
```

Predecessor and successor queries use *jump pointers*, which are kept up to
date after each update using the stack of *2-fat ancestors* provided by the
search, as described in the paper.

### `StaticZFastTrie`

A static index over a sorted sequence of byte strings, providing rank,
membership, predecessor, successor and prefix queries. Nodes are intervals of
ranks, so there are no pointers.

```rust
use zfast::{PackedKeys, StaticZFastTrie};

let keys: PackedKeys = ["bar", "baz", "foo", "foobar"].into_iter().collect();
let trie = <StaticZFastTrie<_>>::new(keys)?;

assert_eq!(trie.index_of("baz"), Some(1));
assert_eq!(trie.rank("c"), 2);
assert_eq!(trie.succ("c"), Some((2, "foo".as_bytes())));
assert_eq!(trie.prefix_range("ba"), 0..2);
# Ok::<(), zfast::BuildError>(())
```

The structure is designed along the lines of [`sux`]:

- the keys can be stored in any `IndexedSeq` whose output can be seen as a
  slice of bytes (for example, a rear-coded list), or in the uncompressed
  `PackedKeys`, which gives the fastest queries;

- all the components holding a significant amount of data are type parameters:
  in particular, with the feature `epserde` the structure can be serialized
  with [ε-serde] and then deserialized, or memory-mapped, without copying;

- the trie can index just one key every *k* (`with_ratio`), completing queries
  with a short search among consecutive keys: the index becomes *k* times
  smaller and, for moderate values of *k*, queries on large sets of keys
  become faster, as the index fits a faster level of the memory hierarchy;

- the directories of the nodes can be stored in a packed, fast form
  (`PackedDirs`, the default) or in three monotone sequences (`SeqDirs`), which
  can be represented, for example, by the Elias–Fano representation of [`sux`]
  with the selection structure of choice (`into_ef` is a shortcut for the
  default one), halving the space used by the index;

- the structure implements `IndexedSeq`, `IndexedDict`, `Succ` and `Pred`
  whenever the sequence of keys makes it possible.

### Hashing

The way prefixes are hashed is a type parameter of all structures: `Xxh3`
(the default) hashes each prefix from scratch with xxh3, whereas `Chain`
processes the key once and then hashes each prefix in constant time. In our
experiments the first strategy is faster unless keys are very long.

## Testing and benchmarking

Besides unit tests and documentation tests, the directory `tests` contains
differential tests against naive implementations on adversarial sets of keys:
the tests of the dynamic structures check all invariants (names, extents,
handles, signatures, jump pointers, the list of leaves, the content of
buckets) after every update, and all structures are tested also with
signatures of a few bits, so that false positives and duplicate signatures are
common.

```text
cargo test --features epserde
cargo test --release --features epserde,slow_tests
```

There are [Criterion] benchmarks for the most common operations, with
`BTreeSet` and binary search as baselines:

```text
cargo bench --bench dynamic
cargo bench --bench fixed
ZFAST_N=10000000 ZFAST_KEYS=urls.txt cargo bench --bench fixed -- rank
```

The number of keys is set by `ZFAST_N`, and the keys by `ZFAST_KEYS`: the name
of a file with one key per line, or `urls` (synthetic URL-like keys, the
default), `u64` or `u64x2` (random keys of 8 or 16 bytes). The example `scale`
is a plain timing harness for large-scale experiments, which also reports space
usage, and the example `stats` prints statistics about fat binary search.

Note that the crate has a `.cargo/config.toml` enabling `target-cpu=native`.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-Apache-2.0](LICENSE-Apache-2.0) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <https://opensource.org/licenses/MIT>)

at your option.

[`sux`]: https://crates.io/crates/sux
[ε-serde]: https://crates.io/crates/epserde
[Criterion]: https://crates.io/crates/criterion
