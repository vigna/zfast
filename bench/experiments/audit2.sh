#!/bin/sh
#
# Audit of the configurations of the competitors, second round, kept as it was run on
# 2026-11-20 with the compilers of Amazon Linux 2023 (gcc 11, clang 15.0.7,
# gcc14): it must be run inside bench/, after "make all" and "mkdir audit", on
# a workload of 50 million keys. Results: see README.md and raw/.
# Second round: combinations of the best options, on 50 million keys
P=data/eu-2015.q10000000-100000000
CF="-O3 -DNDEBUG -march=native -flto -fno-stack-protector -std=gnu18 -pthread -w"
XF="-O3 -DNDEBUG -march=native -std=c++17 -w"
B=btree-cpp
BT="$B/btree/anynode.cpp $B/btree/btree2020.cpp $B/btree/debug.cpp $B/btree/dense.cpp $B/btree/hash.cpp $B/in-memory-structures/ART/ART.cpp"
JE=/usr/lib64/libjemalloc.so.2
clang $CF -Iaudit/wormhole-256 -o audit/wh-clang-kpn256 wh_bench.c audit/wormhole-256/lib.c audit/wormhole-256/kv.c audit/wormhole-256/wh.c -lm
for ps in 32768 65536; do
  clang++ $XF -DPS_L=$ps -DPS_I=$ps -I$B -include $B/named-configs/adapt2.hpp -DNAMED_CONFIG=\"adapt2\" -DMETHOD=\"adapt2-$ps\" -o audit/bt-adapt2-$ps bt_bench.cpp $BT
done
clang++ $XF -DPS_L=16384 -DPS_I=4096 -I$B -include $B/named-configs/adapt2.hpp -DNAMED_CONFIG=\"adapt2\" -DMETHOD=\"adapt2-16384-4096\" -o audit/bt-adapt2-l16-i4 bt_bench.cpp $BT
run() { n=$1; c=$2; shift 2; taskset -c $c "$@" $P 3 > audit/$n.log 2>&1; }
run wh-clang-je 24 env LD_PRELOAD=$JE audit/wh-clang & run wh-clang-kpn256-je 72 env LD_PRELOAD=$JE audit/wh-clang-kpn256 & wait
run bt-16384-je 24 env LD_PRELOAD=$JE audit/bt-adapt2-16384 & run bt-32768-je 72 env LD_PRELOAD=$JE audit/bt-adapt2-32768 & wait
run bt-65536-je 24 env LD_PRELOAD=$JE audit/bt-adapt2-65536 & run bt-l16-i4-je 72 env LD_PRELOAD=$JE audit/bt-adapt2-l16-i4 & wait
run hot-gcc14-je 24 env LD_PRELOAD=$JE audit/hot-gcc14 & run art-unsync-je 72 env LD_PRELOAD=$JE audit/art-unsync & wait
run zpaged-je 24 env LD_PRELOAD=$JE build/zfast-bench run zpaged & run btreeset-je 72 env LD_PRELOAD=$JE build/zfast-bench run btree & wait
run zset-je 24 env LD_PRELOAD=$JE build/zfast-bench run zset & run zft-je 72 env LD_PRELOAD=$JE build/zfast-bench run zft & wait
