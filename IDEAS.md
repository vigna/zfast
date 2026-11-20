# Ideas for the future

## A two-level string dictionary with a z-fast trie on the block heads

*Recorded 2026-11-20, from the discussion of Ferragina, Rotundo, Vinciguerra,
"Two-level massive string dictionaries" (Information Systems 128, 2025; notes
in `paper/related-work/verified-static-uses.md`, section 5).*

Their dictionary stores the sorted strings in fixed-size blocks (4-32 KiB) on
disk, rear-coded, with the first string of each block explicit, and indexes the
block heads in RAM with a succinct Patricia trie (LOUDS or DFUDS), descended
node by node (blind search) and validated with one string access. They neither
use nor cite z-fast tries.

The idea: the same organisation with a z-fast trie in place of the Patricia
trie to delimit the blocks. Their indexing level solves weak prefix search on
the block heads, which is the problem of the ESA 2010 paper: the same order of
space, `O(n lg l)` bits against their `(20 + lg l)` bits per indexed string,
with a logarithmic number of probes instead of a descent of `O(|q|)` nodes.

What exists: `StaticZFastTrie::with_ratio(k)` in `crate/` indexes one key
every `k` of any `KeySeq`, and is tested over a sux `RearCodedList`.

What is missing for a fair comparison:

- a key store with blocks of fixed byte size and an explicit first string,
  possibly memory-mapped, instead of one sample every `k` strings;
- a succinct index: the crate uses 16-byte slots (about 256 bits per sampled
  key with packed directories, 128 with Elias-Fano), against about 78 bits per
  block head for their Patricia trie and about 430 for their array of heads
  (both computed from their Table 2, not printed there). The succinct variant
  exists only in Java (`ZFastTrieDistributor*` in Sux4J);
- their data (URLs from the LAW crawls, 3.65 billion; Software Heritage file
  names) and their baselines (the Patricia trie, binary search on the heads).

Expectations, not measurements: on disk the index is a few microseconds out
of 30-110, so only its size matters; in RAM, at 20 million block heads, a
z-fast search should cost six or seven cache misses, under a microsecond,
against the 2.5-3.5 microseconds read off their plots for the Patricia trie
on LOUDS (on a slower machine).
