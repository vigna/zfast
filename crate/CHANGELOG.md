# Change Log

## [0.1.0] - unreleased

### New

- `ZFastTrie`, a dynamic z-fast trie on byte strings with insertions,
  deletions, membership, predecessor and successor queries, and iteration.

- `ZFastPagedSet`, a dynamic sorted set of byte strings for large sets:
  keys are stored without their common prefix in pages of four kilobytes,
  grouped in segments that are the leaves of a z-fast trie working on
  signatures only; a segment contains a hash table for membership queries
  and a blind directory of its pages for order queries.

- `ZFastSet`, a dynamic sorted set of byte strings made of buckets of keys
  indexed by a `ZFastTrie`.

- `StaticZFastTrie`, a static z-fast trie over a sorted sequence of byte
  strings with rank, membership, predecessor, successor and prefix queries,
  optional sampling of the keys, type-parameter backends, directories based on
  sequences from `sux`, and optional support for ε-serde.

- `PackedKeys`, an uncompressed sequence of byte strings, and the trait
  `KeySeq`, implemented by all indexed sequences of `sux` returning bytes.

- Prefix-hashing strategies `Xxh3` and `Chain`.
