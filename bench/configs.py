"""
Configuration of the comparative benchmarks of z-fast tries.

Each entry of METHODS specifies:
  - tool: "rust" (the driver zfast-bench, covering the structures of the
    zfast crate and the baselines from the Rust standard library) or
    "wormhole" (the C driver wh_bench), "art" (the C++ driver art_bench) or
    "btree-cpp" (the C++ driver bt_bench, built once for each configuration of
    the code of Mueller, Benson and Leis: their adaptive B-tree, and their
    version of HOT);
  - arg: for "rust", the name of the method passed to the driver; for
    "btree-cpp", the name of the configuration;
  - label: the name used in reports;
  - tex: the name used in the tables of the paper (default: the label);
  - kind: "dynamic" or "static" (static structures have no update phases).

Each entry of DATASETS specifies the source of the keys: either a synthetic
generator of the driver ("urls", "u64", "u64x2"), or the name of a file with
one key per line, relative to the data directory.

To add a new method, add an entry to METHODS and, if it needs a new driver, a
new tool in run.py and a target in the Makefile: a driver must read the
workload files written by `zfast-bench gen` and print the same RESULT and
SPACE lines (see zfast-bench/src/main.rs).
"""

METHODS = {
    "ZFastSet": {
        "tool": "rust", "arg": "zset", "kind": "dynamic",
        "label": "z-fast set (bucketed)", "tex": "\\texttt{ZFastSet}",
    },
    "ZFastTrie": {
        "tool": "rust", "arg": "zft", "kind": "dynamic",
        "label": "z-fast trie", "tex": "\\texttt{ZFastTrie}",
    },
    "ZFastTrieChain": {
        "tool": "rust", "arg": "zft-chain", "kind": "dynamic",
        "label": "z-fast trie (chained hash)",
    },
    "Wormhole": {
        "tool": "wormhole", "kind": "dynamic",
        "label": "Wormhole", "tex": "Wormhole",
    },
    "AdaptiveBTree": {
        "tool": "btree-cpp", "arg": "adapt2", "kind": "dynamic",
        "label": "adaptive B-tree", "tex": "B-tree~\\cite{Muller2025BTrees}",
    },
    "HOT": {
        "tool": "btree-cpp", "arg": "hot", "kind": "dynamic",
        "label": "HOT", "tex": "HOT",
    },
    "ART": {
        "tool": "art", "kind": "dynamic",
        "label": "ART", "tex": "ART",
    },
    "BTreeSet": {
        "tool": "rust", "arg": "btree", "kind": "dynamic",
        "label": "BTreeSet (Rust)", "tex": "\\texttt{BTreeSet}",
    },
    "StaticZFastTrie": {
        "tool": "rust", "arg": "szft:1", "kind": "static",
        "label": "static z-fast trie", "tex": "ratio 1",
    },
    "StaticZFastTrie16": {
        "tool": "rust", "arg": "szft:16", "kind": "static",
        "label": "static z-fast trie (ratio 16)", "tex": "ratio 16",
    },
    "StaticZFastTrie16Ef": {
        "tool": "rust", "arg": "szft-ef:16", "kind": "static",
        "label": "static z-fast trie (ratio 16, Elias-Fano)", "tex": "ratio 16, EF",
    },
    "BinarySearch": {
        "tool": "rust", "arg": "bsearch", "kind": "static",
        "label": "binary search", "tex": "binary search",
    },
}

DATASETS = {
    # Synthetic URL-like keys (no external data needed)
    "urls": {"spec": "urls"},
    # Random 8-byte and 16-byte keys
    "u64": {"spec": "u64"},
    "u64x2": {"spec": "u64x2"},
}

# Default numbers of keys (half are inserted, half are used for unsuccessful
# queries).
SIZES = [2_000_000, 20_000_000]

# Default number of repetitions of the query phases.
REPS = 3

# The operations, in the order of the reports.
OPS = ["insert", "build", "contains-hit", "contains-miss", "succ-hit",
       "succ-miss", "iterate", "remove"]
