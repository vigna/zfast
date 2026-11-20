Raw results of the comparative benchmarks with every structure in its best
configuration, obtained with `bench/paper.sh` on 2026-11-20 on the machine of
`../results-epyc` (AWS EC2, AMD EPYC 9R45, 96 cores, 375 GB of memory, Amazon
Linux 2023, transparent huge pages enabled for all processes). With respect to
`../results-epyc`: GCC 16.2.0 and clang 23.1.6 built from source by `aws.sh`,
jemalloc preloaded in all processes, Wormhole also with 256 keys per leaf,
adaptive B-tree with 16 KiB nodes, and a later version of `ZFastPagedSet`.
The choices are motivated in `bench/experiments/README.md`.

`python3 bench/run.py report --results paper/results-epyc-tuned --dataset eu-2015.q10000000`
prints the results as tables.
