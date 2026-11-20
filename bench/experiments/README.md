# Experiments behind the benchmarks

This directory records the side experiments that shaped the benchmarks and the
design of `ZFastPagedSet`: what was measured, what was concluded, and what was
tried and dropped. The comparative results themselves are in
`paper/results-epyc/`; this is the lab notebook.

Everything here was measured on 2026-11-20 on the same machine:

- an EC2 instance with an AMD EPYC 9R45: 96 cores, one thread per core,
  1 MiB of L2 cache per core, 32 MiB of L3 cache per group of 8 cores, 375 GiB
  of memory, Amazon Linux 2023;
- keys are URLs of the eu-2015 crawl of at most 255 bytes; a workload of *n*
  keys has ten million successful and ten million unsuccessful queries (five
  million for the smallest size);
- every process is pinned to a core; processes running at the same time are on
  cores that do not share caches;
- times are nanoseconds per operation, the best of three repetitions for
  queries. Unless stated otherwise the workload has 50 million keys, and the
  columns are: insertion, successful and unsuccessful membership, successor of
  a key in the set and of a key not in the set, scan (per key), deletion.

The raw output of every run quoted below is in `raw/` (one file per run, named
as in the tables). The scripts are `audit.sh`, `audit2.sh` (competitor
configurations, kept as they were run) and `ablation.sh` (top-level ablation,
reusable). `patches/` contains the two instrumented variants of the crate used
in the ablation; they apply with `git apply` to the tree of the commit that
added this directory.

## 1. Cost model of the machine

Measured with a pointer-chasing microbenchmark over 64 GiB:

- a dependent cache miss costs 130–141 ns with 2 MiB pages, and 251 ns with
  4 KiB pages; 1 GiB pages give nothing more;
- eight independent misses issued together cost the same as one.

So at half a billion keys an operation costs, to a first approximation, 130 ns
times the number of *rounds* of misses, where a round is a set of accesses
whose addresses are all known at the same time. This is the reason for the
layout of `ZFastPagedSet`: a router small enough to stay in the cache, then
one round in the segment and one round in the page.

## 2. Configuration of the competitors

The question was whether each competitor was being run in its best
configuration for this workload. It was not: with respect to the stock setup
(compilers of the distribution, allocator of the C library, default
parameters), in its best configuration Wormhole takes about 30% less time on
lookups, and the B-tree 25% less.

### Allocator

Preloading jemalloc (`LD_PRELOAD`) helps every structure that allocates memory
piecewise, including the Rust ones (the Rust allocator calls `malloc`). It does
nothing for `ZFastPagedSet`, which maps its pages and segments itself.

| | add | cont.⁺ | cont.⁻ | succ⁺ | succ⁻ | scan | del |
|---|---|---|---|---|---|---|---|
| Wormhole, gcc 11 (`thp-wh`) | 755 | 526 | 436 | 601 | 1271 | 10.9 | 756 |
| — with jemalloc (`wh-jemalloc`) | 720 | 462 | 369 | 531 | 1168 | 9.8 | 736 |
| B-tree, default nodes (`thp-bt`) | 750 | 969 | 758 | 1104 | 1086 | 17.5 | 1209 |
| — with jemalloc (`bt-jemalloc`) | 714 | 890 | 663 | 1060 | 1045 | 16.3 | 1285 |
| HOT (`thp-hot`) | 942 | 696 | 644 | 835 | 898 | 56.8 | |
| — with jemalloc (`hot-jemalloc`) | 822 | 589 | 533 | 708 | 758 | 48.6 | |
| ART, unsynchronized (`art-unsync`) | 1216 | 1432 | 1268 | | | | 1640 |
| — with jemalloc (`art-unsync-je`) | 1048 | 1139 | 983 | | | | 1477 |
| `BTreeSet` (campaign of `paper/results-epyc`) | 1292 | 1381 | 1415 | 1397 | 1418 | 19.7 | 1470 |
| — with jemalloc (`btreeset-je`) | 1056 | 1094 | 1122 | 1110 | 1128 | 18.6 | 1264 |
| `ZFastSet` (campaign) | 1284 | 1088 | 747 | 1069 | 1360 | 13.1 | 1276 |
| — with jemalloc (`zset-je`) | 1105 | 918 | 640 | 876 | 1113 | 10.0 | 1042 |
| `ZFastTrie` (campaign) | 1024 | 931 | 821 | 928 | 1044 | 141 | 1259 |
| — with jemalloc (`zft-je`) | 941 | 878 | 782 | 884 | 1005 | 135 | 1142 |
| `ZFastPagedSet` (campaign) | 841 | 459 | 280 | 481 | 775 | 9.7 | 567 |
| — with jemalloc (`zpaged-je`) | 837 | 456 | 277 | 479 | 757 | 9.6 | 564 |

Decision: all structures run with jemalloc preloaded (`run.py --preload`).

A caveat on space: with jemalloc and huge pages the resident memory of the
B-tree with 16 KiB nodes is 118.6 bytes per key, against 86.5 with the
allocator of the C library. The space of the B-tree depends on the allocator
more than that of the other structures.

### Transparent huge pages

`ZFastPagedSet` asks for huge pages explicitly; the other structures get them
only if the system setting is `always`. With that setting Wormhole gains 9% at
50 million keys and 16% at 500 million; the B-tree does not change; HOT loses
a few percent.

| lookups (cont.⁺ / cont.⁻) | 50M, `madvise` | 50M, `always` | 500M, `madvise` | 500M, `always` |
|---|---|---|---|---|
| Wormhole | 574 / 473 | 521 / 428 | 898 / 737 | 758 / 659 |
| B-tree | 979 / 765 | 971 / 758 | 1561 / 1357 | 1548 / 1356 |
| HOT | 692 / 641 | 740 / 641 | 1306 / 1262 | 1370 / 1324 |

Decision: `always` for all runs, so that all structures are measured in the
same conditions. (These numbers come from two complete campaigns, not from
files in `raw/`.)

### Wormhole

| compiler, leaf size | add | cont.⁺ | cont.⁻ | succ⁺ | succ⁻ | scan | del |
|---|---|---|---|---|---|---|---|
| gcc 11, 128 (`thp-wh`) | 755 | 526 | 436 | 601 | 1271 | 10.9 | 756 |
| gcc 14, 128 (`wh-gcc14`) | 741 | 498 | 421 | 587 | 1253 | 12.2 | 752 |
| clang 15, 128 (`wh-clang`) | 750 | 424 | 343 | 514 | 1215 | 12.0 | 726 |
| gcc 11, 256 (`wh-kpn256`) | 717 | 468 | 347 | 555 | 1377 | 10.4 | 694 |
| clang 15, 128, jemalloc (`wh-clang-je`) | 721 | 370 | 296 | 458 | 1122 | 10.4 | 705 |
| clang 15, 256, jemalloc (`wh-clang-kpn256-je`) | 695 | 333 | 247 | 437 | 1224 | 10.3 | 654 |

With 64 keys per leaf Wormhole crashes (segmentation fault inside the
library). With 256 keys per leaf it is faster on everything except the
successor of a key that is not in the set.

After building GCC 16.2.0 and clang 23.1.6 from source (`aws.sh`), head to
head with jemalloc, two rounds with the cores swapped (`h2h-*`):

| compiler, leaf size | add | cont.⁺ | cont.⁻ | succ⁺ | succ⁻ | scan | del |
|---|---|---|---|---|---|---|---|
| clang 15, 256 | 692 | 330 | 246 | 433 | 1215 | 10.3 | 647 |
| GCC 16.2, 256 | 704 | 335 | 256 | 419 | 1191 | 10.2 | 648 |
| clang 23.1, 256 | 736 | 351 | 269 | 456 | 1247 | 10.4 | 686 |
| clang 15, 128 | 726 | 374 | 299 | 461 | 1133 | 10.6 | 713 |
| GCC 16.2, 128 | 730 | 380 | 308 | 443 | 1097 | 10.5 | 713 |
| clang 23.1, 128 | 760 | 400 | 325 | 475 | 1156 | 10.6 | 744 |

Decision: GCC 16.2 (it comes from the reproducible setup, and is within 4% of
clang 15 either way), and both leaf sizes are reported (`Wormhole`,
`Wormhole256`).

### Adaptive B-tree (btree-cpp)

Named configurations of btree-cpp, default node size, clang 15, no jemalloc:

| configuration | add | cont.⁺ | cont.⁻ | succ⁺ | succ⁻ | scan | del |
|---|---|---|---|---|---|---|---|
| `baseline` | 1158 | 1304 | 1332 | 1324 | 1373 | 23.8 | 1445 |
| `prefix` | 1023 | 1182 | 1212 | 1191 | 1235 | 18.4 | 1330 |
| `heads` | 945 | 1090 | 1084 | 1093 | 1125 | 19.5 | 1248 |
| `hints` | 959 | 1058 | 1049 | 1068 | 1100 | 19.4 | 1269 |
| `hash` | 744 | 979 | 763 | 1166 | 1210 | 19.5 | 1117 |
| `adapt` | 757 | 973 | 757 | 1154 | 1195 | 19.0 | 1110 |
| `adapt2` (`thp-bt`) | 750 | 969 | 758 | 1104 | 1086 | 17.5 | 1209 |

Node size, configuration `adapt2`:

| node size | add | cont.⁺ | cont.⁻ | succ⁺ | succ⁻ | scan | del |
|---|---|---|---|---|---|---|---|
| 2 KiB | 876 | 1082 | 887 | 1163 | 1167 | 26.8 | 1213 |
| 8 KiB | 669 | 902 | 682 | 1080 | 1080 | 12.5 | 1251 |
| 16 KiB | 635 | 841 | 627 | 1055 | 1074 | 10.0 | 1280 |
| 16 KiB, jemalloc (`bt-16384-je`) | 584 | 731 | 524 | 914 | 931 | 8.3 | 1201 |
| 32 KiB, jemalloc | 594 | 766 | 557 | 1002 | 1025 | 7.0 | 1244 |
| 64 KiB, jemalloc | 591 | 723 | 561 | 1044 | 1066 | 6.2 | 1346 |
| leaves 16 KiB, inner nodes 4 KiB, jemalloc | 610 | 774 | 566 | 952 | 969 | 8.6 | 1238 |

Decision: `adapt2` with 16 KiB nodes (`BT_PAGE_SIZE` in the `Makefile`).
btree-cpp needs clang; with clang 23.1 the result is the same within 3%
(`n-bt`: 605, 750, 537, 950, 950, 8.3, 1221).

### HOT and ART

| | add | cont.⁺ | cont.⁻ | succ⁺ | succ⁻ | scan | del |
|---|---|---|---|---|---|---|---|
| HOT, adapter compiled with gcc 11 (`thp-hot`) | 942 | 696 | 644 | 835 | 898 | 56.8 | |
| HOT, gcc 14 (`hot-gcc14`) | 932 | 685 | 638 | 841 | 899 | 59.1 | |
| HOT, gcc 14, jemalloc (`hot-gcc14-je`) | 798 | 565 | 531 | 702 | 752 | 52.3 | |
| HOT, GCC 16.2 and clang 23.1, jemalloc (`n-hot`) | 824 | 563 | 533 | 636 | 742 | 45.2 | |
| ART with optimistic lock coupling, jemalloc (`art-jemalloc`) | 1069 | 1150 | 984 | 2489 | 2512 | 90.2 | 1495 |
| ART unsynchronized, jemalloc (`art-unsync-je`) | 1048 | 1139 | 983 | | | | 1477 |

- The interface of HOT in btree-cpp has no deletion.
- The unsynchronized ART is no faster than the synchronized one, so we keep
  the latter, which has range scans. Its successor is computed with a scan
  returning two results: it is an artifact of the interface, not of the
  structure, and should be read as such.
- The ART bundled with btree-cpp is not used: it returns wrong answers for
  keys that are not in the set.

## 3. Design choices of `ZFastPagedSet`

### Pages per segment

Compile-time option `ZFAST_SEGMENT_PAGES` (`geom-*`).

| pages per segment | add | cont.⁺ | cont.⁻ | succ⁺ | succ⁻ | scan | del | router |
|---|---|---|---|---|---|---|---|---|
| 32, 50M keys | 853 | 474 | 285 | 481 | 842 | 8.6 | 583 | 131 |
| 64, 50M keys | 827 | 465 | 285 | 484 | 782 | 10.8 | 617 | 129 |
| 128, 50M keys | 837 | 457 | 282 | 477 | 763 | 9.4 | 570 | 123 |
| 64, 500M keys | 1062 | 608 | 456 | 656 | 1041 | 8.9 | 951 | 147 |
| 128, 500M keys | 1030 | 558 | 403 | 596 | 1010 | 8.8 | 799 | 142 |

Decision: 128. Larger segments make the router smaller, and that matters more
than the larger blind directory.

### Things tried on the router and dropped

The router takes 121–125 ns per search at 50 million keys, of an unsuccessful
lookup of 280 ns and a successful one of 460 ns. Attempts to make it faster:

| variant | router | cont.⁺ | cont.⁻ |
|---|---|---|---|
| xxh3 on each prefix (`zpaged-xxh3`, `zpaged-xxh3b`) | 122–125 | 459 | 280 |
| `Chain` (one pass, then two multiplications per prefix; `zpaged-chain`) | 139 | 467 | 290 |
| CRC32C instruction, one pass, then two multiplications (`zpaged-crc`) | 137 | 466 | 291 |
| circular queue for the speculative probes (`zpaged-ring`) | 126 | 458 | 282 |

- Incremental hashers do much less work per prefix and are nonetheless
  *slower* than hashing every prefix from scratch with xxh3. A
  variant of the CRC hasher with a single multiplication is as fast as xxh3
  and no faster (measured only on a laptop: 96 against 94 ns for the search
  loop, 112 with two multiplications): the time depends on the latency of the
  hash, not on the number of instructions.
- The speculative probes (`ZFAST_AHEAD`) help even when the router is in the
  cache: on a laptop, 117 ns without, 108 with one, 107 with two.

## 4. How much of the speed comes from the z-fast trie?

The router is a z-fast trie on the lower fences of the segments. Would a
B-tree do the same job faster? Two experiments.

### The top level alone

`zfast-bench fences` writes the fences of a built set as a workload with the
queries of the original one; every structure is then run on it
(`ablation.sh`). The time to find the segment of a key is that of a successor
query on the fences (`fa-*`):

| ns per search | 5M keys, 1210 fences | 50M keys, 13 724 fences | 500M keys, 130 328 fences |
|---|---|---|---|
| router of `ZFastPagedSet` | 80 | 121 | 142 |
| HOT | 76 | 120 | 148 |
| `BTreeSet` | 66 | 100 | 164 |
| adaptive B-tree, 16 KiB nodes | 100 | 121 | 191 |
| `ZFastTrie` (exact search) | 89 | 141 | 170 |
| binary search on the sorted fences | 94 | 154 | 234 |
| Wormhole, 128 keys per leaf | 112 | 147 | 186 |
| ART (successor by scan) | 396 | 622 | 825 |

The fences are 31, 42 and 50 bytes long on average. (The files of the first
column were not kept.)

### A B-tree inside the set

`patches/top-level.patch` adds a `BTreeMap` from fences to segments, kept in
sync with the router and used in its place when `ZFAST_BTREE_TOP` is set. At
50 million keys (`top-router`, `top-btree`):

| top level | search | cont.⁺ | cont.⁻ | succ⁺ | succ⁻ |
|---|---|---|---|---|---|
| z-fast router | 122 | 460 | 284 | 480 | 762 |
| `BTreeMap` | 131 | 468 | 294 | 488 | 811 |

(In the set the `BTreeMap` is slower than alone, 131 against 100 ns: its nodes
and keys are scattered in a large heap and compete for the cache with the
segments.)

### Conclusions

1. **The speed of `ZFastPagedSet` does not come from the z-fast trie.** The
   top level is a quarter of a lookup, and up to 50 million keys a B-tree does
   that job as well. What makes the structure fast is the two levels below:
   the hash table and the blind directory of the segments, and the pages. Any
   top level would benefit from them.
2. **The router scales better than the B-trees.** From 13 724 to 130 328
   fences it slows down by 17%, `BTreeSet` by 64%, the adaptive B-tree by 58%.
   At half a billion keys it is 13% and 25% faster than them, and as fast as
   HOT. This is the expected behaviour: the number of probes of a fat binary
   search depends on the length of the strings, not on their number.
3. **The absolute gain is small.** At half a billion keys a B-tree on top
   would cost 20–50 ns more per operation, on operations of 400–1000 ns.
4. The exact z-fast trie (`ZFastTrie`, which compares strings) is slower than
   the signature-only router at all sizes: the router can skip verification
   because the segment checks the fences anyway.

### Anatomy of a router search

At 50 million keys (`patches/counters.patch`, `patches/top-level.patch`):

- 10.05 hash evaluations, 6.39 probes of the z-map and 2.51 hits per search,
  on an interval of prefix lengths of 65.3 on average;
- the fat binary search takes 93 ns, the final step (comparison with the bytes
  stored in the node, choice of the child) 28 ns;
- 565 cycles, 1213 instructions, 8.3 branch mispredictions, 22 L1 misses and
  0.5 L2 misses per search (`perf stat`);
- the router proposes one candidate segment in 84.5% of the searches, and two
  in the others (the first is the right one in half of those); in ten million
  searches the exact fallback was never needed.

So a probe costs about 67 cycles, and there is more than one misprediction per
probe. The outcome of each probe is essentially a coin toss, and each
misprediction discards the speculative work in flight, which is why the
latency of the hash matters and its instruction count does not.

Wormhole's own top level (a hash table of all prefixes of the anchors,
searched by binary search on the length) answers an unsuccessful lookup at 5
million keys, with tens of thousands of anchors, in 135–155 ns *including*
the access to the leaf. A hashed trie can therefore be searched considerably
faster than our router is. The price Wormhole pays is space proportional to
the length of the anchors; a z-fast trie stores one handle per node, and pays
with a search that is not a plain binary search (each hit must return the
length of an extent before the next probe can be chosen).

### An idea that was not tried

A fat binary search without unpredictable branches: a z-map made of buckets of
one cache line (six entries with a 32-bit signature, a 16-bit extent length
and a node), compared all at once, and an update of the interval by
conditional moves; the bucket of the next probe on the unsuccessful path is
prefetched. A back-of-the-envelope estimate gives 35–40 cycles per step with
the map in L2, that is, about 55 ns per search instead of 93. It is an
estimate: the two optimizations above that looked safer on paper did not
work.

## 5. Open issues

- `StaticZFastTrie` with sampling ratio 1 exits with status 101 at 500 million
  keys (not investigated; ratio 16 works).
- Wormhole with 64 keys per leaf crashes.
- The successor of ART is measured through a scan (see above).
- The memory of the B-tree depends on the allocator (see above).
