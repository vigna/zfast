Raw results of a rehearsal of the comparative benchmarks on `blew` (Intel
Core i7-12700KF, runs pinned to a performance core with `taskset -c 4`; Linux
6.19, clang 21.1.8, GCC 15.3.1, Rust 1.99.1), 2026-11-20, commit af491f3:
`bench/paper.sh` on ten million URLs from eu-2015, of which the 9,917,318 of
at most 255 bytes are kept (workloads of 990,000 and 9,900,000 keys).

These are not the numbers of the paper, which come from an AMD EPYC 9R45
(`../results-epyc`): they were used to validate the drivers (all checksums
agree) before the final runs.
