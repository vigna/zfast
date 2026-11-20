# Z-fast tries

This repository contains the paper *Z-Fast Tries* by Djamal Belazzougui, Paolo
Boldi and Sebastiano Vigna, a Rust implementation of the data structures it
describes, and a system for comparative benchmarks.

- `paper`: the LaTeX source of the paper (`latexmk -pdf dynzfast.tex`), a
  Python script that checks its update algorithms by executing them
  (`python/check_updates.py`), a notebook with an explicit implementation, the
  record of the November 2026 review, the list of corrections made after it
  (`CORRECTIONS.md`), and the literature surveys for the related-work section
  (`related-work`).

- `crate`: the Rust crate `zfast`, with a dynamic z-fast trie (`ZFastTrie`), a
  bucketed dynamic set indexed by a z-fast trie (`ZFastSet`), and a static
  z-fast trie (`StaticZFastTrie`); see its [README](crate/README.md).

- `bench`: comparative benchmarks against other data structures (at this
  time, [Wormhole](https://github.com/wuxb45/wormhole), the adaptive B-tree
  of Müller, Benson and Leis, [HOT](https://github.com/speedskater/hot),
  [ART](https://github.com/flode/ARTSynchronized) and structures from the
  Rust standard library); see below.

## Comparative benchmarks

The benchmarks are driven by `bench/run.py`. Competitors are git submodules
(run `make -C bench init` after cloning), with the exception of the code of
*B-Trees Are Back* (<https://github.com/m-mueller678/btree-cpp>), whose
repository contains several hundred megabytes of data: the parts we need are
fetched on demand, at a fixed commit, into `bench/btree-cpp`. The drivers of
the competitors need Linux on x86-64 with AVX2 and BMI2, `clang++`, `g++`, and
the headers of Boost and TBB.

```text
python3 bench/run.py bench --method all --dataset urls --n 2000000
python3 bench/run.py bench --method ZFastSet,Wormhole --dataset my-urls.txt \
    --n 20000000 --pin "taskset -c 8"
python3 bench/run.py report --dataset urls
```

A dataset is a synthetic source defined in `bench/configs.py` (`urls`, `u64`,
`u64x2`) or the name of a file in `bench/data` with one key per line. Half of
the keys are inserted, and half are used for unsuccessful queries; the
workload is written once to `bench/data` in a simple binary format read by all
drivers, so that all structures perform exactly the same operations in the
same order, and each driver prints checksums that `report` compares. Results
are stored per data point in `bench/results`, so a campaign can be interrupted
and resumed.

The runs of the paper are started by `bench/paper.sh URLS [CORE]`, which
discards the URLs longer than 255 bytes (HOT and the B-tree limit the length
of the keys) and runs all methods on two sizes. The tables of the paper are
generated from the results stored in `paper/results-epyc` (obtained on an AMD
EPYC 9R45):

```text
python3 bench/run.py table --results paper/results-epyc --dataset eu-urls \
    --n 10000000 --kind dynamic --method ZFastSet,ZFastTrie,Wormhole,BTreeSet \
    --out paper/fig/bench-dynamic-10000000.tex
```

The drivers are `bench/zfast-bench` (Rust: the structures of the crate and the
baselines from the standard library), `bench/wh_bench.c` (Wormhole),
`bench/bt_bench.cpp` (the adaptive B-tree and HOT, through the common
interface of btree-cpp) and `bench/art_bench.cpp` (ART with optimistic lock
coupling). To add a structure, write a driver that reads the same workload
files and prints the same lines (`bench/harness.hpp` contains the code shared
by the C++ drivers), and add an entry to `bench/configs.py`.

Some limitations of the competitors are documented at the top of each driver:
in particular, the adapter of HOT has no deletions, ART needs keys without
zeros, and the ART bundled with btree-cpp is not used because it returns a few
wrong answers for keys not in the set.

## License

The crate and the benchmark code are licensed under either of the Apache
License, Version 2.0, or the MIT license, at your option. Competitors in
submodules have their own licenses.
