Raw results of the comparative benchmarks reported in the paper, obtained
with `bench/paper.sh` on an AWS EC2 instance with an AMD EPYC 9R45 (96 cores,
375 GB of memory, Amazon Linux 2023 with Linux 6.18, transparent huge pages
enabled for all processes) on 2026-11-20. The keys are URLs of at most 255
bytes from the eu-2015 crawl, in random order; the name of a file gives the
number of strings from which the workload was generated (half of them are
keys of the set, capped by the number of strings available), and `q10000000`
is the maximum number of queries of each kind.

`python3 bench/run.py report --results paper/results-epyc --dataset eu-2015.q10000000`
prints them as tables.
