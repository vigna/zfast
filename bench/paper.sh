#!/bin/sh
#
# Runs the comparative benchmarks reported in the paper.
#
#   bench/paper.sh URLS
#
# URLS is a file containing at least 510 million distinct URLs of at most 255
# bytes, one per line, in random order (HOT and the B-tree of btree-cpp limit
# the length of the keys). We obtained it from the URLs of the eu-2015 crawl
# (http://data.law.di.unimi.it/webdata/eu-2015/eu-2015.urls.gz) with
#
#   zcat eu-2015.urls.gz | LC_ALL=C awk 'length($0) <= 255' > all.urls
#   shuf -n 530000000 all.urls > URLS
#
# The script runs all methods on 5 and 50 million keys, and the main ones on
# 500 million keys, with ten million queries of each kind (five million for
# the smallest size); at most two structures run at the same time, pinned
# to cores that do not share caches (adjust the cores to your machine). The
# largest size needs 250 GB of memory. Results are written in bench/results
# under the name eu-2015.q10000000; print them with
#
#   python3 run.py report --dataset eu-2015.q10000000
#
# Transparent huge pages should be enabled for all processes, so that all
# structures are measured in the same conditions (ZFastPagedSet asks for huge
# pages explicitly):
#
#   echo always | sudo tee /sys/kernel/mm/transparent_hugepage/enabled

set -e
if [ $# -lt 1 ]; then
	echo "Usage: $0 URLS" 1>&2
	exit 1
fi
URLS=$(realpath "$1")
cd "$(dirname "$0")"

# Fetch the competitors in submodules (not needed in a plain copy of the tree)
if git rev-parse --git-dir > /dev/null 2>&1; then
	make init
fi
make all
mkdir -p data
ln -sf "$URLS" data/eu-2015

# The driver reads the first lines of the file, instead of deduplicating and
# shuffling it in memory
export ZFAST_PRESHUFFLED=1
Q=10000000

run() {
	python3 run.py bench --no-build --method "$1" --dataset eu-2015 --n "$2" --queries $Q \
		--pin "taskset -c $3" > "paper-$1-$2.log" 2>&1
}

# Runs the given methods on a workload of the given size, two at a time
pairs() {
	n=$1
	shift
	while [ $# -gt 0 ]; do
		run "$1" "$n" 8 &
		if [ $# -gt 1 ]; then
			run "$2" "$n" 56 &
			shift
		fi
		shift
		wait
	done
}

ALL="ZFastPagedSet Wormhole AdaptiveBTree HOT ZFastSet ZFastTrie ART BTreeSet
	StaticZFastTrie StaticZFastTrie16 StaticZFastTrie16Ef BinarySearch"
# Half of the strings are inserted
pairs 10000000 $ALL
pairs 100000000 $ALL
pairs 1000000000 ZFastPagedSet Wormhole AdaptiveBTree HOT StaticZFastTrie16 BinarySearch \
	StaticZFastTrie16Ef StaticZFastTrie ZFastSet BTreeSet

python3 run.py report --dataset eu-2015.q$Q
