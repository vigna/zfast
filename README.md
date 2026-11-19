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
  time, [Wormhole](https://github.com/wuxb45/wormhole) and structures from
  the Rust standard library); see below.

## Comparative benchmarks

The benchmarks are driven by `bench/run.py`. Competitors are git submodules:
run `make -C bench init` after cloning.

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

`bench/zfast-bench` is the Rust driver for the structures of the crate and for
the baselines from the standard library; `bench/wh_bench.c` is the driver for
Wormhole. To add a structure, write a driver that reads the same workload
files and prints the same lines, and add an entry to `bench/configs.py`.

## License

The crate and the benchmark code are licensed under either of the Apache
License, Version 2.0, or the MIT license, at your option. Competitors in
submodules have their own licenses.
