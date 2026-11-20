#!/bin/sh
#
# Audit of the configurations of the competitors, first round, kept as it was run on
# 2026-11-20 with the compilers of Amazon Linux 2023 (gcc 11, clang 15.0.7,
# gcc14): it must be run inside bench/, after "make all" and "mkdir audit", on
# a workload of 50 million keys. Results: see README.md and raw/.
# Variants of the competitors on 50 million keys, two at a time on cores 24 and 72
P=data/eu-2015.q10000000-100000000
CF="-O3 -DNDEBUG -march=native -flto -fno-stack-protector -std=gnu18 -pthread -w"
XF="-O3 -DNDEBUG -march=native -std=c++17 -w"
B=btree-cpp
BT="$B/btree/anynode.cpp $B/btree/btree2020.cpp $B/btree/debug.cpp $B/btree/dense.cpp $B/btree/hash.cpp $B/in-memory-structures/ART/ART.cpp"
HI=$(find $B/in-memory-structures/hot -type d -name include | sed "s/^/-I/")
JE=/usr/lib64/libjemalloc.so.2
# Wormhole: compilers and keys per node
clang $CF -Iwormhole -o audit/wh-clang wh_bench.c wormhole/lib.c wormhole/kv.c wormhole/wh.c -lm
gcc14-gcc $CF -Iwormhole -o audit/wh-gcc14 wh_bench.c wormhole/lib.c wormhole/kv.c wormhole/wh.c -lm
for k in 64 256; do
  rm -rf audit/wormhole-$k; cp -r wormhole audit/wormhole-$k
  sed -i "s|^#define WH_KPN ((128u))|#define WH_KPN (($k""u))|" audit/wormhole-$k/wh.c
  grep -c "define WH_KPN (($k""u))" audit/wormhole-$k/wh.c
  gcc $CF -Iaudit/wormhole-$k -o audit/wh-kpn$k wh_bench.c audit/wormhole-$k/lib.c audit/wormhole-$k/kv.c audit/wormhole-$k/wh.c -lm
done
# B-tree: configurations and page sizes
for c in adapt hash hints heads prefix baseline; do
  clang++ $XF -I$B -include $B/named-configs/$c.hpp -DNAMED_CONFIG=\"$c\" -DMETHOD=\"$c\" -o audit/bt-$c bt_bench.cpp $BT
done
for ps in 2048 8192 16384; do
  clang++ $XF -DPS_L=$ps -DPS_I=$ps -I$B -include $B/named-configs/adapt2.hpp -DNAMED_CONFIG=\"adapt2\" -DMETHOD=\"adapt2-$ps\" -o audit/bt-adapt2-$ps bt_bench.cpp $BT
done
# HOT: the adapter compiled with a newer g++
gcc14-g++ $XF -c $HI -o audit/hot_adapter14.o $B/btree/hot_adapter.cpp
clang++ $XF -I$B -include $B/named-configs/hot.hpp -DNAMED_CONFIG=\"hot\" -DMETHOD=\"hot-gcc14\" -o audit/hot-gcc14 bt_bench.cpp $BT audit/hot_adapter14.o
# ART: unsynchronized
g++ $XF -DART_UNSYNC -IARTSynchronized -o audit/art-unsync art_bench.cpp ARTSynchronized/ART/Tree.cpp -ltbb -pthread
ls audit | tr "\n" " "
run() { # name core command...
  n=$1; c=$2; shift 2
  taskset -c $c "$@" $P 3 > audit/$n.log 2>&1
}
run wh-clang 24 audit/wh-clang & run wh-gcc14 72 audit/wh-gcc14 & wait
run wh-kpn64 24 audit/wh-kpn64 & run wh-kpn256 72 audit/wh-kpn256 & wait
run wh-jemalloc 24 env LD_PRELOAD=$JE build/wh_bench & run hot-jemalloc 72 env LD_PRELOAD=$JE build/bt_bench-hot & wait
run bt-adapt 24 audit/bt-adapt & run bt-hash 72 audit/bt-hash & wait
run bt-hints 24 audit/bt-hints & run bt-heads 72 audit/bt-heads & wait
run bt-prefix 24 audit/bt-prefix & run bt-baseline 72 audit/bt-baseline & wait
run bt-adapt2-2048 24 audit/bt-adapt2-2048 & run bt-adapt2-8192 72 audit/bt-adapt2-8192 & wait
run bt-adapt2-16384 24 audit/bt-adapt2-16384 & run bt-jemalloc 72 env LD_PRELOAD=$JE build/bt_bench-adapt2 & wait
run hot-gcc14 24 audit/hot-gcc14 & run art-unsync 72 audit/art-unsync & wait
run art-jemalloc 24 env LD_PRELOAD=$JE build/art_bench & wait
