#!/bin/sh
#
# Runs the comparative benchmarks reported in the paper.
#
#   bench/paper.sh URLS [CORE]
#
# URLS is a file with one URL per line, in random order (we use ten million
# URLs from the eu-2015 crawl), and CORE is the core the runs are pinned to
# (default: 8). URLs longer than 255 bytes are discarded, because HOT and the
# B-tree of btree-cpp limit the length of the keys. Results are written in
# bench/results; existing results are kept (use `run.py bench --force` to
# rerun a method).

set -e
if [ $# -lt 1 ]; then
	echo "Usage: $0 URLS [CORE]" 1>&2
	exit 1
fi
URLS=$(realpath "$1")
CORE=${2:-8}
cd "$(dirname "$0")"

mkdir -p data
if [ ! -f data/eu-urls-255 ]; then
	LC_ALL=C awk 'length($0) <= 255' "$URLS" > data/eu-urls-255
fi
# Half of the keys are inserted, and half are used for unsuccessful queries
N=$(($(wc -l < data/eu-urls-255) / 100000 * 100000))

# Fetch the competitors in submodules (not needed in a plain copy of the tree)
if git rev-parse --git-dir > /dev/null 2>&1; then
	make init
fi
python3 run.py bench --method all --dataset eu-urls-255 --n $((N / 10)),$N --pin "taskset -c $CORE"
python3 run.py report --dataset eu-urls-255
