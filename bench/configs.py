"""
Configuration of the comparative benchmarks of z-fast tries.

Each entry of METHODS specifies:
  - tool: "rust" (the driver zfast-bench, covering the structures of the
    zfast crate and the baselines from the Rust standard library) or
    "wormhole" (the C driver wh_bench);
  - arg: for "rust", the name of the method passed to the driver;
  - label: the name used in reports;
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
        "label": "z-fast set (bucketed)",
    },
    "ZFastTrie": {
        "tool": "rust", "arg": "zft", "kind": "dynamic",
        "label": "z-fast trie",
    },
    "ZFastTrieChain": {
        "tool": "rust", "arg": "zft-chain", "kind": "dynamic",
        "label": "z-fast trie (chained hash)",
    },
    "Wormhole": {
        "tool": "wormhole", "kind": "dynamic",
        "label": "Wormhole",
    },
    "BTreeSet": {
        "tool": "rust", "arg": "btree", "kind": "dynamic",
        "label": "BTreeSet (Rust)",
    },
    "StaticZFastTrie": {
        "tool": "rust", "arg": "szft:1", "kind": "static",
        "label": "static z-fast trie",
    },
    "StaticZFastTrie16": {
        "tool": "rust", "arg": "szft:16", "kind": "static",
        "label": "static z-fast trie (ratio 16)",
    },
    "StaticZFastTrie16Ef": {
        "tool": "rust", "arg": "szft-ef:16", "kind": "static",
        "label": "static z-fast trie (ratio 16, Elias-Fano)",
    },
    "BinarySearch": {
        "tool": "rust", "arg": "bsearch", "kind": "static",
        "label": "binary search",
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
