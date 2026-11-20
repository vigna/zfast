#!/bin/sh
#
# Top-level ablation: how fast would each structure be as the top level of a
# ZFastPagedSet?
#
#   bench/experiments/ablation.sh PREFIX [REPS]
#
# PREFIX is a workload written by "zfast-bench gen" (for example,
# data/eu-2015.q10000000-100000000). The script builds a ZFastPagedSet on its
# keys, writes the lower fences of the segments (the strings indexed by the
# router) as the keys of a new workload PREFIX-fences with the same queries,
# and runs every structure on it: the time of a successor query on the fences
# (op=succ-hit, op=succ-miss; almost no query is a fence, so the two are the
# same measurement) is the time the structure would take to find the segment
# of a key. The time of the router is that of op=route-hit, op=route-miss.
#
# The structures are small, so they can run at the same time on cores that do
# not share caches: adjust CORES to your machine. Results are written in
# PREFIX-fences.METHOD.txt.
#
# To replace the router with a B-tree inside the set, see patches/top-level.patch
# and README.md.

set -e
if [ $# -lt 1 ]; then
	echo "Usage: $0 PREFIX [REPS]" 1>&2
	exit 1
fi
P=$1
REPS=${2-3}
F=$P-fences
CORES=${CORES-"8 16 24 32 40 48 56 64 72"}
JEMALLOC=${JEMALLOC-/usr/lib64/libjemalloc.so.2}
cd "$(dirname "$0")/.."

run() { # name command...
	name=$1
	shift
	# The first core of the list, which is then dropped
	core=${CORES%% *}
	CORES=${CORES#"$core"}
	CORES=${CORES# }
	LD_PRELOAD=$JEMALLOC taskset -c "$core" "$@" > "$F.$name.txt" 2>&1 &
}

LD_PRELOAD=$JEMALLOC build/zfast-bench fences "$P" "$F"
run router env ZFAST_STATS=1 ZFAST_ONLY=route build/zfast-bench run zpaged "$P" "$REPS"
run BTreeSet build/zfast-bench run btree "$F" "$REPS"
run BinarySearch build/zfast-bench run bsearch "$F" "$REPS"
run ZFastTrie build/zfast-bench run zft "$F" "$REPS"
run Wormhole build/wh_bench "$F" "$REPS"
run Wormhole256 build/wh_bench-256 "$F" "$REPS"
run AdaptiveBTree build/bt_bench-adapt2 "$F" "$REPS"
run HOT build/bt_bench-hot "$F" "$REPS"
run ART build/art_bench "$F" "$REPS"
wait
rm -f "$F.hits" "$F.misses"
grep -H "op=route\|op=succ" "$F".*.txt | sed 's/RESULT method=[^ ]* //; s/ checksum=.*//'
