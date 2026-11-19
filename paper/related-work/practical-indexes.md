# Practical in-memory ordered indexes for variable-length keys, and the "binary search on prefix lengths over a hash table" lineage

Survey for the related-work and experimental sections of *Z-Fast Tries*
(Belazzougui, Boldi, Vigna). Written 2026-11-19. It verifies, deepens and
extends `../review-2026-11/reports/F-literature-and-applications.md`
(sections 1-3 and 11 of that report); it does not repeat its sections on range
filters, FM-indexes, external memory, zuffix arrays, static dictionaries and
read mappers.

## 0. How to read this report

Every statement carries one of these tags, explicitly or by context:

- **[paper]**: read by me in the paper itself (full text, either from
  `review-2026-11/literature-extracts/` or downloaded in this session). Numbers
  are quoted with their table/figure. Numbers read off *bar labels or plot
  axes* are marked "(plot)".
- **[code]**: read by me in the source repository, at the commit stated.
- **[abstract]**: I read only the abstract or a publisher/DBLP/Crossref record.
- **[inference]**: my own reasoning; nobody measured it.

Bibliographic data were checked against the Crossref API (publisher-deposited
metadata), because **DBLP could not be queried in this session**: both
`dblp.org` and `dblp.uni-trier.de` answer scripted requests with an anti-bot
challenge page. Entries without a DOI (USENIX, technical reports, software)
were checked against the publisher's or the project's own page. The
verification status of each entry is in the comments of
`practical-indexes.bib`.

Sources used: the extracts of 36 papers in
`review-2026-11/literature-extracts/` (Wormhole, Cuckoo Trie, HOT, Muller et
al., FB+-tree, LITS, RSS, Waldvogel et al., c-trie++, C2, SuRF read again for
this report); papers downloaded in this session (ART, Masstree, REMIX, HOPE,
PIM-trie, Zip-tries, PlanB, the prefix hash tree, burst tries, the B-tree
compression study, B2-tree, the LITS extended version, Dementiev et al.);
shallow clones of the repositories of Wormhole, the Cuckoo Trie, HOT,
Masstree and btree24; the GitHub, Crossref, OpenAlex and crates.io APIs. No
personal or project information was sent to any service.

Summary of the answers to the three priority questions:

1. **Wormhole's published reference list cites neither x-fast/y-fast tries nor
   z-fast tries**; its only source for the binary search on prefix lengths is
   Waldvogel et al. (Section 1.1). Waldvogel et al. (SIGCOMM '97) in turn cite
   neither van Emde Boas nor Willard (Section 2.2).
2. **Competitors to benchmark**, in order: Wormhole (`whunsafe` API), HOT
   (single-threaded), the adaptive B-tree of Muller et al., an ART, the Cuckoo
   Trie, Masstree, FB+-tree, LITS, plus the customary baselines (Section 7.3).
3. **Datasets**: Section 7.1. The "URL" dataset of SuRF, HOPE and RSS is the
   `uk-2007-05` URL list of the Laboratory for Web Algorithmics.

## 1. Wormhole

**Xingbo Wu, Fan Ni, Song Jiang. "Wormhole: A Fast Ordered Index for In-memory
Data Management." EuroSys '19 (Fourteenth EuroSys Conference), Dresden, March
25-28, 2019, 16 pages. DOI 10.1145/3302424.3303955. arXiv:1805.02200 (v2 of 16
Feb 2019 is the text I read in full).** [paper]

### 1.1 Does Wormhole cite x-fast/y-fast tries or z-fast tries? No.

- The **published** reference list, as deposited by ACM at Crossref
  (`https://api.crossref.org/works/10.1145/3302424.3303955`, field
  `reference`, 48 entries), is entry-by-entry the same list as arXiv v2 (48
  entries, same order). I compared all 48.
- Entry 45 is DOI 10.1145/263105.263136, i.e. Waldvogel, Varghese, Turner,
  Plattner, SIGCOMM '97. It is the **only** reference to the idea of binary
  search on prefix lengths ("Waldvogel et al. proposed to use binary search on
  prefix lengths to accelerate the match for routing table lookups [45]. To
  apply the approach, we insert all prefixes of each anchor into a hash table",
  Section 2.4).
- There is **no** reference to Willard (x-fast/y-fast tries; DOI
  10.1016/0020-0190(83)90075-3), to van Emde Boas, to Belazzougui-Boldi-Pagh-Vigna
  (SODA 2009, ESA 2010) or to Belazzougui-Boldi-Vigna (SPIRE 2010). None of
  those DOIs occurs in the deposited list, and the 20 entries without DOI are
  all software, web pages, textbooks (CLRS) or USENIX papers.
- The conclusions claim: "To the best of our knowledge, Wormhole is the first
  ordered key-value index achieving the O(log L) lookup cost" (Section 6). This
  ignores x-fast tries (1983), y-fast tries and the z-fast trie (2009/2010),
  all of which achieve O(log L) (L in bits) for predecessor search, and the
  z-fast trie does it for variable-length strings with a hash table of n - 1
  entries.
- The connection was made by others: the Cuckoo Trie paper (SOSP 2021, Section
  7, "Hashed representation") says "Hashed representations are used by X-Fast
  tries [51] and Wormhole [52]. Both, however, use serial binary search to
  determine the depth of a key in the trie", citing Willard. I found no systems
  paper that connects Wormhole to the z-fast trie (see Section 2.4).

### 1.2 Mechanism [paper]

Wormhole is a B+-tree whose inner part ("MetaTree") is replaced by a trie of
separators stored in a hash table.

- **LeafList.** All keys live in a doubly linked list of leaf nodes; a leaf
  holds at most 128 keys in the prototype (`WH_KPN` in the code; 64 and 256
  are the suggested alternatives).
- **Anchors.** Each leaf has an *anchor key*, a separator satisfying (i) the
  *ordering condition* (every key of the left neighbour < anchor <= every key
  of the leaf) and (ii) the *prefix condition*: no anchor is a prefix of
  another anchor. When a leaf is split between keys a < b, the new anchor is
  lcp(a, b) plus the next token of b (Algorithm 4, line 6: `alen =
  commonPrefixSize(...) + 1`), extended with a smallest token (binary zero)
  when needed to keep the set prefix-free. In the vocabulary of the z-fast
  trie, an anchor is the *name* of a node of the byte-level compacted trie of
  the keys (the child, on the side of b, of the node whose extent is lcp(a,
  b)): the shortest prefix of b that separates it from a.
- **MetaTrie and MetaTrieHT.** The anchors form a (non-compacted, 256-ary)
  trie. *Every prefix of every anchor* is an item of a hash table, the
  MetaTrieHT. An item is a *leaf item* (the prefix is an anchor: pointer to the
  leaf) or an *internal item* holding a 256-bit bitmap of the existing children
  and two pointers, to the leftmost and the rightmost leaf of the subtrie.
  These two pointers are exactly the role of the z-fast trie's jump pointers
  restricted to their end points.
- **Lookup** (Algorithms 1 and 3). Phase 1: binary search on the prefix length
  in [0 .. min(L_anc, L_key)], where L_anc is the length of the longest anchor,
  for the longest prefix of the key that is in the table (LPM). Because *all*
  prefixes are present, the predicate "prefix of length m is in the table" is
  monotone, so a plain bisection works: this is Waldvogel's scheme with all
  markers present, and it is what the z-fast trie avoids with handles and
  2-fattest numbers. Phase 2: if the LPM item is a leaf item, that leaf is the
  target; otherwise the bitmap gives the sibling of the missing token in O(1),
  one more hash probe fetches the sibling's item, and its leftmost/rightmost
  pointer (plus possibly one step to the left on the LeafList) gives the target
  leaf.
- **Leaf search.** A leaf has a *tag array* (16-bit hash tags, sorted by hash)
  and a *key array* (pointers to out-of-line key-value items, sorted by key
  only lazily: new keys are appended and the array is sorted when a range scan
  or a split needs it). A point lookup jumps to position k*T/(Tmax+1)
  ("direct speculative positioning") and compares 1-3 tags on average; only a
  matching tag triggers a full key comparison.
- **Hashing and tags in the MetaTrieHT.** CRC-32c, computed incrementally when
  the binary search extends a matched prefix, so that a lookup hashes about L
  bytes in total instead of (L/2) log L. A bucket is one 64-byte cache line
  with eight (16-bit tag, 48-bit pointer) pairs. During the binary search the
  tags are *trusted* without comparing the stored prefix; one full comparison
  is made at the end and the search restarts with full comparisons if it
  fails. The paper computes an error probability of 0.0153% for 1024-byte keys
  (Section 3.1). This is the same idea as the signatures of the signed z-fast
  trie with one final verification.

What the current code does beyond the paper [code, commit 9d3d0d0 of
2026-10-11, `wh.c`]:

- The table is split in two arrays: `wmap`, 16-byte slots of eight 16-bit
  tags, and `pmap`, 64-byte buckets of eight pointers. Each prefix has two
  candidate buckets (`hash32 & mask` and `bswap(hash32) & mask`).
  `wormhmap_peek`, used by the optimistic phase, reads **only the 16-byte tag
  slots** (SSE/NEON compare), so a probe of the binary search costs at most two
  16-byte reads and no pointer dereference.
- `wormhole_meta_lcp` is not a textbook bisection: while the gap is larger
  than 7 it advances by `gd >> 3 << 2` bytes (half the gap rounded down to a
  multiple of 4, to use 4-byte CRC steps), then bisects; the fallback path
  (after a failed final verification) uses 3/4 splits and the child bitmap.
- A trie node (`struct wormmeta`) is 32 bytes plus a 32-byte bitmap only when
  it has more than one child.
- Anchors may be up to 65520 bytes; keys up to 2^32 - 1 bytes; binary keys
  with zeros are supported (the restriction of Section 3.3 of the paper was
  removed). Documented limitations: at most 128 keys may share a prefix of
  65520 bytes or more, and certain adversarial prefix patterns overflow the
  two eight-entry candidate buckets (no fallback).

### 1.3 Complexity claims

- Lookup: O(log min(L_anc, L_key)) hash probes, L in **bytes** (tokens), plus
  O(1) for the leaf; the abstract says "O(log L) worst-case time". [paper] The
  O(L) cost of hashing and of the final comparison is not counted (it is the
  |x|/w term of the z-fast trie bounds).
- Insertion/deletion without split/merge: lookup plus O(1) leaf work.
- Split: inserts the new anchor and **all its prefixes** (Algorithm 4, lines
  12-24: a loop over `plen` from 0 to `key.len - 1`, each iteration a hash
  lookup and possibly an insertion or an update of the leftmost/rightmost
  pointers), so Theta(L_anc) hash-table operations per split; merge is
  symmetric (lines 27-34). The paper does not state this bound; it is my
  reading of Algorithm 4. Amortised over the >= 64 insertions needed to fill
  a half-full leaf, this is O(L_anc / 64) table operations per insertion.
  [inference from paper]
- Space of the MetaTrieHT: one item per node of the *uncompacted* trie of the
  anchors, i.e. up to (number of leaves) x (anchor length) items in the worst
  case, less the sharing of common prefixes. The paper gives no bound. With
  about N/96 leaves (leaves between half full and full) and anchors of 40
  bytes this is at most 0.42 N items for the URL keyset; sharing makes it
  smaller. [inference] The z-map of a z-fast trie built on the same anchors
  would have (number of leaves - 1) entries, independently of anchor length.

### 1.4 Memory [paper]

- Figure 16 (plot; no table): "in most cases Wormhole's memory usage is
  comparable to those of B+ tree and skip list"; Masstree is "significantly
  higher" except with 8-byte keys; ART is much higher on 8- and 16-byte keys.
  Method: difference of resident set size before/after building (getrusage),
  huge pages disabled, every key-value item allocated separately; "Baseline" =
  number of keys x (key length + 8).
- The second copy of the MetaTrieHT kept for RCU costs 0.34% to 3.7% of the
  whole index over the eight keysets (Section 2.5), which bounds the size of
  one MetaTrieHT to the same fraction: the table is tiny compared with leaves
  and keys.
- Independent numbers: in the Cuckoo Trie paper (Figure 11, plot; index
  overhead excluding key-value items) the Cuckoo Trie is "lower than that of
  ARTOLC and Wormhole by as much as 28%"; HOT and STX are smaller. In Muller et
  al. (Section 6.5) on dense 32-bit keys, leaf nodes are 32% and records 65% of
  Wormhole's allocations, and a third of the process size is allocator
  overhead.

### 1.5 Concurrency [paper]

Per-leaf reader-writer locks; one mutex serialising all writers of the
MetaTrieHT (splits and merges only); readers of the MetaTrieHT are lock-free
through QSBR RCU over two copies of the table (changes are applied to the
hidden copy, the copies are swapped, and after a grace period the changes are
applied to the other copy); version numbers on the table and on leaves detect
lookups that used a stale table and make them restart. The thread-unsafe
variant is 7.8% faster at 16 threads on Az1 (21.2 vs 19.5 Mops, Section 4.1).

### 1.6 The benchmarks in the paper [paper]

- **Hardware.** Dell R630, two 16-core Xeon E5-2697A v4 (40 MB LLC each), 256
  GB DDR4-2400, Linux 4.15.15, hyper-threading off, one NUMA node used. A
  second identical server and 100 Gb/s Infiniband for the networked
  experiment (indexes ported into HERD).
- **Keysets** (Table 1):

  | Name | Description | Keys (millions) | Size (GB) |
  |---|---|---|---|
  | Az1 | Amazon review metadata, item-user-time, avg 40 B | 142 | 8.5 |
  | Az2 | the same, user-item-time, avg 40 B | 142 | 8.5 |
  | Url | URLs from MemeTracker, avg 82 B | 192 | 20.0 |
  | K3 | random keys, 8 B | 500 | 11.2 |
  | K4 | random keys, 16 B | 300 | 8.9 |
  | K6 | random keys, 64 B | 120 | 8.9 |
  | K8 | random keys, 256 B | 40 | 10.1 |
  | K10 | random keys, 1024 B | 10 | 9.7 |

- **Competitors**: STX B+-tree (fanout 128, "which yields the best result on
  our testbed"), the skip list extracted from LevelDB, ART (`armon/libart`),
  Masstree (`kohler/masstree-beta`), and for point lookups only the cuckoo hash
  table `efficient/libcuckoo`. All indexes store pointers to separately
  allocated key-value items. Only Wormhole and Masstree are thread-safe; the
  others are used read-only or with one writer.
- **Operations**: (i) lookups of keys chosen uniformly at random from the
  keyset, 1 to 16 threads; (ii) the same through a networked key-value store;
  (iii) insertion of a whole keyset into an empty index, one thread; (iv)
  mixed lookups and insertions (5/50/95% insertions), 16 threads, Wormhole vs
  Masstree only; (v) range lookup: seek to a random key and return up to 100
  following keys; (vi) memory. Deletions are not measured. Lookups of absent
  keys are not measured. There is no predecessor operation (and the public
  code has none: "Wormhole supports forward scanning only; there is no
  `seek_for_less_equal()` or `prev()`", README).
- **Numbers.**
  - One thread, Az1: Wormhole 1.266 Mops, ART 0.834 Mops (second best), i.e.
    +52% (Section 4.1 text; that is 790 ns vs 1199 ns per lookup). Sixteen
    threads: 19.5 Mops (15.4x), 43% above ART.
  - Sixteen threads, all keysets (Figure 10, plot): 1.3x to 4.2x the best
    other index on each keyset; on Url, where anchors average about 40 bytes
    ("as measured"), "at least 1.7x".
  - Headline "up to" factors (abstract): 8.4x skip list, 4.9x B+-tree, 4.3x
    ART, 6.6x Masstree, for lookups.
  - Ablation (Figure 11): the basic structure without tags and incremental
    hashing is 1.26x to 2.25x the B+-tree; tag matching and incremental
    hashing bring 1.4x to 2.6x (incremental hashing alone about 3%); sorting
    leaves by tag and direct positioning bring the total to up to 4.9x.
    Direct positioning reduces tag comparisons per leaf from 6-7 to under 3.
    **Most of Wormhole's advantage over a B+-tree therefore comes from the
    leaf layout and from never comparing full keys, not from the trie.**
  - Versus the cuckoo hash table (Section 4.2): 31% to 67% of its throughput
    on the first seven keysets, close to it on K10 (the abstract says
    30-92%). With keys whose first L - 4 bytes are a constant filler
    (anchors as long as the keys), at L = 512 Wormhole drops from about 78% to
    40% of the hash table's throughput (Figure 14): log2(512) = 9 probes.
  - Insertions, one thread (Figure 15, plot): "comparable to that of the skip
    list on most keysets"; faster than comparison-based indexes with 8- and
    16-byte keys; lower with long keys such as Url.
  - Range lookups (Figure 18): 1.05x to 1.59x the B+-tree; Masstree much
    lower.

### 1.7 Later, independent measurements of Wormhole

- **Cuckoo Trie paper (SOSP 2021)** [paper], one thread, 2 x Xeon Gold 6132,
  uniform YCSB-C: the Cuckoo Trie is up to 50% faster than Wormhole (on `osm`,
  200M 8-byte keys); on `az` (Wormhole's Az1, 82M keys after deduplication,
  avg 35.7 B, long shared prefixes) the Cuckoo Trie is *slower* than the
  others. "Wormhole's algorithm allows it to achieve nearly constant
  throughput over the whole range of dataset sizes" (10M to 640M random
  8-byte keys, Figure 9), while HOT, ARTOLC and STX slow down 1.66-1.7x.
  Wormhole's scan throughput needs "a few million operations to stabilize"
  after loading (lazy sorting of leaves).
- **Muller, Benson, Leis (SIGMOD 2025)** [paper], one thread, Ryzen 9 7950X,
  about 300 MB of records, Zipfian (0.99) lookups, `whunsafe` API with the
  B-tree's allocator integrated: for **lookups** the baseline B-tree is 53-60%
  slower than Wormhole and their adaptive B-tree "is still 26-28% slower",
  except on dense integers. For **insertions** of strings the adaptive B-tree
  is 17-27% faster than Wormhole. For **scans** the baseline B-tree is 45-103%
  faster and the adaptive B-tree 88-430% faster, because Wormhole stores
  pointers to records in its leaves. Figure 1 of that paper labels the lookup
  throughput of baseline / optimised B-tree relative to Wormhole as -53% /
  -28% (url), -60% / -26% (sparse int), -60% / +27% (dense int); absolute
  values are only on the plot axis (up to about 6 M lookups/s).
- **FB+-tree (PVLDB 2025)** [paper] includes Wormhole among seven competitors
  (multi-threaded, 2 x Xeon Gold 6248R, 100M keys); results are plots only
  (Figure 17). The text says that on read-only YCSB-C "HOT gives the best
  performance and scalability on all datasets except rand-int".
- **The Wormhole author's own later numbers** (repository issue #5, "benchmark
  results", opened 2019-09-28, updated March 2021; read through the GitHub
  API): single-threaded HOT versus single-threaded Wormhole "on a Skylake
  server (updated 2021)", in Mops:

  | Keyset | HOT | Wormhole |
  |---|---|---|
  | az-art (Amazon, 142M keys) | 1.270 | 1.802 |
  | addr-mx (OpenAddresses, Mexico) | 1.813 | 1.819 |
  | dblp-title (about 4M keys) | 1.746 | 2.501 |
  | dblp-url | 2.155 | 2.506 |
  | dblp-ee | 2.329 | 2.540 |
  | m9urls (MemeTracker URLs) | 0.917 | 1.155 |

  So on URLs Wormhole is 26% faster than HOT (866 ns vs 1091 ns per lookup),
  and on address data they are equal. The issue also documents that the HOT
  code had to be patched to accept keys longer than 255 bytes and cannot
  store keys containing zero bytes.

### 1.8 The public implementation [code]

- <https://github.com/wuxb45/wormhole>, C (gnu18), about 3900 lines in `wh.c`
  plus `lib.c`, `kv.c`. Last commit 9d3d0d0, **2026-10-11** (a burst of
  hardening commits and the relicensing; the previous commit is 2022-04-29).
- **License**: `GPL-3.0-only OR MIT`, at the user's option (README and SPDX
  headers; introduced by the commit "converting to dual license" of
  2026-10-11; I did not check the licence of earlier trees, so a copy taken
  before that date may be GPL-only).
- **Platforms**: Linux, FreeBSD, macOS on x86-64 (needs SSE 4.2) and AArch64
  (needs NEON and CRC).
- **Build**: `make` (clang by default; `make CCC=gcc`). `Makefile.common`
  compiles with `-O3 -flto -DNDEBUG` and `-march=native -mtune=native` on
  x86-64 (`ARCH ?= native`; on AArch64 `-march=armv8-a+crc`), so the library
  is natively compiled by default, which matters for a comparison with a Rust
  crate built with `target-cpu=native`. It produces `libwh.so` and the demo
  programs (`easydemo.out`, `concbench.out`, `stresstest.out`); `make M=j` or
  `M=t` links jemalloc or tcmalloc. Tunables in `wh.c`: `WH_KPN` (keys per leaf, default 128; "256 can
  offer 5-10%+ higher point query and update speed", "range queries prefer a
  smaller node size such as 64"), `WH_SLABLEAF_SIZE` (2 MB slabs; 1 GB if
  huge pages of that size exist). It uses huge pages when available.
- **Single-threaded API** (`wh.h`): `whunsafe_create(mm)`, `whunsafe_put(map,
  kv)`, `whunsafe_get(map, kref, out)`, `whunsafe_probe(map, kref)`,
  `whunsafe_del(map, kref)`, `whunsafe_delr` (range delete),
  `whunsafe_iter_create/seek/peek/skip1/skip/next/destroy`. "It offers the best
  speed and efficiency but does not perform internal concurrency control."
  Keys are passed as `struct kref` (pointer, length, 32-bit CRC that **the
  caller must compute**, e.g. with `kref_ref_hash32`), values as `struct kv`.
  The memory manager `struct kvmap_mm` decides whether `put` copies the item
  (`wormhole_create(NULL)` copies) or stores the caller's pointer
  (`kvmap_mm_ndf`). The friendlier, thread-safe `wh_*` API (`wh_put(ref, key,
  klen, val, vlen)`, `wh_get`, `wh_probe`, `wh_del`, `wh_iter_seek`, ...) costs
  "roughly 5%-10%".
- **Ordered operations**: `seek` positions on the smallest key >= the argument
  (lower bound); iteration is **forward only**; no predecessor, no reverse
  iteration. A fair comparison of predecessor queries therefore needs either a
  successor formulation or a wrapper.
- A z-fast benchmark should (i) use `whunsafe_*`, (ii) precompute or include
  the CRC consistently on both sides, (iii) try `WH_KPN` 64/128/256, (iv)
  state the allocator and the page size, (v) store the same payload (a
  pointer or nothing) in both structures.

## 2. The lineage: binary search on prefix lengths over hash tables

### 2.1 x-fast and y-fast tries

**Dan E. Willard. "Log-logarithmic worst-case range queries are possible in
space Theta(N)." Information Processing Letters 17(2):81-84, 1983. DOI
10.1016/0020-0190(83)90075-3.** [bibliographic record only; not re-read]

- *x-fast trie*: for w-bit keys, one hash table per level containing all the
  prefixes of all keys, each with a pointer to the smallest/largest leaf below
  it; the deepest existing prefix of a query is found by binary search on the
  w + 1 levels, O(log w) probes; O(n w) space, O(w) update.
- *y-fast trie*: the keys are split into buckets of Theta(w) consecutive keys
  kept in balanced trees, and an x-fast trie indexes one representative per
  bucket: O(n) space, O(log w) expected amortised update.
- **Wormhole is, structurally, a y-fast trie for variable-length byte
  strings**: buckets (leaves of at most 128 keys), one representative per
  bucket (the anchor, shortened to a separator), all prefixes of all
  representatives in a hash table with leftmost/rightmost pointers, binary
  search on the prefix length. The differences are engineering: one table
  instead of one per level, byte tokens, child bitmaps, tags. [inference; the
  Wormhole paper does not say this, the Cuckoo Trie paper relates Wormhole to
  x-fast tries, and PIM-trie describes y-fast tries in exactly these terms]
- The z-fast trie differs from both in the same way: one table entry per
  *compacted* node (the handle) instead of one per prefix, no bucketing needed
  for linear space, variable-length keys.

### 2.2 Waldvogel, Varghese, Turner, Plattner

**Marcel Waldvogel, George Varghese, Jon Turner, Bernhard Plattner. "Scalable
High Speed IP Routing Lookups." SIGCOMM '97, pp. 25-36. DOI
10.1145/263105.263136 (also Computer Communication Review 27(4), DOI
10.1145/263109.263136).** [paper]

**Journal version: "Scalable high-speed prefix matching." ACM Transactions on
Computer Systems 19(4):440-482, 2001. DOI 10.1145/502912.502914.** [abstract]

- Mechanism: one hash table per prefix length; binary search over the
  lengths; a failed probe must mean "no longer prefix exists", so each prefix
  leaves *markers* in the tables that the binary search visits before
  reaching its length, and each marker stores the best matching prefix of
  itself to avoid backtracking. Worst case log2 W probes for W-bit addresses
  ("only 5 hash lookups are needed for IPv4 and 7 for IPv6", abstract); at
  most log2 W markers per prefix. The paper is precise about where the
  markers go (Section 3, between 3.1 and 3.3): "if the Level(P) is written down
  in binary as a1, a2, ..., an, then we need a marker at each level a1 a2 ...
  ak 0 0 ... 0 such that ak = 1 ... the number of marker nodes is limited by
  the number of 1 bits in Level(P)"; sharing of markers kept the growth to
  25% on their backbone table, but "it is easy to give a worst case example
  where the storage needs require O(log2 W) markers per prefix". "Mutating
  binary search" specialises the search tree after each match and brings the
  average below 2 probes on a 33,000-entry backbone table.
- **It cites neither van Emde Boas nor Willard**: the reference list of the
  SIGCOMM paper has about twenty entries, all from networking (I listed the
  citation keys in the extract); the conclusions only say that the algorithm
  "is very close to the theoretical limit of O(log log N)". I could not read
  the TOCS version in full, so I do not know whether the journal version
  repairs this.
- Relation to the z-fast trie [inference from the passage just quoted]:
  Waldvogel's marker lengths for a prefix of length l are obtained by
  clearing the low-order bits of l, one 1 bit at a time: they are exactly the
  lengths with more trailing zeros than l that a binary search visits on its
  way to l. A z-fast trie keeps *exactly one* prefix per compacted node, the
  one whose length is the 2-fattest number (most trailing zeros) in the
  node's skip interval; since the 2-fattest number in (a..b] is b with its
  bits below the most significant bit of a xor b cleared, this is, taking l
  to be the length of the node's extent, the coarsest of Waldvogel's marker
  lengths that still falls inside the skip interval (or l itself). The
  search is made to work with that single entry (fat binary search). The handle is a Waldvogel marker; the contribution of
  the z-fast trie is that one per node suffices. So the three designs can be
  ordered by table size for n strings of length up to L: all prefixes (x-fast,
  Wormhole on anchors: up to nL entries), markers (Waldvogel: up to n log L),
  handles (z-fast: n - 1).

### 2.3 Descendants in networking

- **Sarang Dharmapurikar, Praveen Krishnamurthy, David E. Taylor. "Longest
  prefix matching using Bloom filters." SIGCOMM 2003, pp. 201-212. DOI
  10.1145/863955.863979.** [bibliographic record; mechanism from memory, not
  re-read]: one Bloom filter per prefix length queried in parallel, then hash
  probes from the longest candidate; it replaces the binary search by
  parallelism.
- **Haowei Yuan, Patrick Crowley. "Reliably scalable name prefix lookup." ANCS
  2015, pp. 111-121. DOI 10.1109/ANCS.2015.7110125.** [abstract, from the
  earlier review]: binary search on the number of name components for
  named-data networking.
- **Donghong Jiang, Yanbiao Li, Yuxuan Chen, Jing Hu, Yi Huang, Gaogang Xie.
  "Heuristic Binary Search: Adaptive and Fast IPv6 Route Lookup with
  Incremental Updates." APNet 2023, pp. 47-53, DOI 10.1145/3600061.3600077;
  journal version IEEE Transactions on Networking 33(2):554-569, 2025, DOI
  10.1109/TNET.2024.3504244.** [not read; described in PlanB]: the current
  representative of the "binary search on prefix lengths" family for IPv6.
- **Zhihao Zhang et al. "PlanB: Efficient Software IPv6 Lookup with Linearized
  B+-Tree." arXiv:2604.14650 (v2, 28 April 2026); preprint.** [paper, read in
  part]. It converts prefixes to elementary intervals and searches a
  cache-aligned, pointer-free B+-tree with SIMD. Reported: DPDK forwarding
  throughput 1.7x-2.5x that of HBS (text accompanying Figure 7), 1.7x-4.8x at
  maximum core count on a 24-core Intel server; its explanation is that the
  B+-tree stays cache-resident for FIBs of up to a million prefixes while HBS
  "requires substantial extra space to maintain low hash collision rates" and
  its speed "is sensitive to the number of distinct prefix lengths and the
  efficiency of its hash function". **This is the same phenomenon the authors
  observed on 5 million URLs: a cache-resident comparison-based structure
  beats a few independent-looking but DRAM-bound hash probes.**
- I did not find evidence of a production router or kernel using binary
  search on prefix lengths, and I did not investigate patents.

### 2.4 Descendants and relatives elsewhere

- **Sriram Ramabhadran, Sylvia Ratnasamy, Joseph M. Hellerstein, Scott
  Shenker. "Brief announcement: Prefix hash tree." PODC 2004, p. 368. DOI
  10.1145/1011767.1011823 (I read the longer technical report "Prefix Hash
  Tree: An Indexing Data Structure over Distributed Hash Tables").** [paper,
  read in part]. A binary trie whose nodes are stored in a distributed hash
  table under their prefix; a lookup finds the leaf by "binary search on
  prefix lengths" (their words, Section 2.2.1): a probe that finds an
  internal node tries longer prefixes, a failed probe tries shorter ones.
  log D DHT lookups for D-bit keys. Leaves are buckets of keys, linked for
  range queries: again a y-fast-like design, here because each probe is a
  network round trip.
- **Roman Dementiev, Lutz Kettner, Jens Mehnert, Peter Sanders. "Engineering a
  Sorted List Data Structure for 32 Bit Keys." ALENEX 2004, pp. 142-151.**
  [abstract; pages from two citing papers, no DOI found]. A three-level
  non-recursive van Emde Boas-like structure (root array, hash tables below)
  that beats comparison-based structures for searching 32-bit keys: the first
  engineering evidence that the hashed-levels idea can win in practice, for
  short fixed-length keys.
- **Rotem Oshman, Nir Shavit. "The SkipTrie: low-depth concurrent search
  without rebalancing." PODC 2013, pp. 23-32. DOI 10.1145/2484239.2484270.**
  [bibliographic record only]: a concurrent y-fast-style structure (skip list
  plus x-fast trie).
- **Hongbo Kang, Yiwei Zhao, Guy E. Blelloch, Laxman Dhulipala, Yan Gu,
  Charles McGuffey, Phillip B. Gibbons. "PIM-trie: A Skew-resistant Trie for
  Processing-in-Memory." SPAA 2023. DOI 10.1145/3558481.3591070.** [paper,
  read in part]. The only systems-flavoured paper I found that *builds on* the
  z-fast trie: "The family of fast tries (x-fast tries, y-fast tries, z-fast
  tries) are explicitly designed to address challenge C2 [long strings], but
  x-fast tries and y-fast tries support only fixed-length keys"; PIM-trie
  "builds upon the idea in z-fast tries of combining both a radix tree and a
  hash table", and uses "a z-fast trie of height w ... for every pivot node"
  as a component. It is a theory paper for the processing-in-memory model;
  an implementation is listed as future work.
- **REMIX** (Wenshao Zhong, Chen Chen, Xingbo Wu, Song Jiang. "REMIX:
  Efficient Range Query for LSM-trees." FAST 2021, USENIX; no DOI) [paper,
  read in part]: despite the shared authors with Wormhole it does **not** use
  prefix lengths; a seek is a comparison-based binary search on the anchor
  keys of a globally sorted view over several runs, then on the keys of a
  segment.
- **Hyperion** (Section 3) is a trie without hashing [abstract].
- **Cuckoo Trie and MlpIndex** (Section 3) store trie nodes in a hash table
  keyed by prefix but descend linearly and prefetch.
- LSM range filters: see the earlier review (Section 4 there); no filter does
  binary search on prefix lengths.
- **I found no LSM engine, key-value store or database index, other than
  Wormhole, that uses binary search on prefix lengths over a hash table.**

### 2.5 Who cites the z-fast trie papers among systems papers

Method: OpenAlex `cites:` lists on 2026-11-19 for the SPIRE 2010 paper (39
citing works), the SODA 2009 monotone hashing paper (116), the ESA 2010 paper
(71) and the JEA 2011 paper (56), filtered by venue; plus a text search for
"z-fast" in all 36 extracts. OpenAlex's coverage is incomplete, so absence is
weak evidence; presence was checked in the paper when I had it.

- *Dynamic Z-Fast Tries* (SPIRE 2010) is cited by: PIM-trie (SPAA 2023, see
  above) [paper]; c-trie++ (DCC 2020; Information and Computation 285, 2022)
  [paper], whose library contains a C++ port of the Sux4J z-fast trie;
  CoCo-trie (Information Systems 120, 2024) [paper]; dynamic path-decomposed
  tries (ACM JEA 25, 2020); Zip-tries (ACDA 2025) [paper]; C2 (arXiv
  2606.16104, 2026) [paper], which benchmarks a z-fast trie; and the journal
  version of LITS (ACM TODS, 2026, DOI 10.1145/3841175) [abstract only: I
  could not see in which context]. The rest are string-algorithms and
  predecessor-search papers.
- The monotone-hashing papers are cited by systems papers for *static
  functions*, not for the trie: SILT (SOSP 2011), Grafite (SIGMOD 2024), "Can
  Learned Models Replace Hash Functions?" (PVLDB 2022), Othello hashing
  (ICNP 2017 / ToN 2018), "Learned Static Function Data Structures" (PVLDB
  2026). [OpenAlex records only]
- **None** of Wormhole, Cuckoo Trie, HOT, ART, Masstree, Muller et al.,
  FB+-tree, LITS (PVLDB version), RSS mentions z-fast tries (text search of
  the extracts and of the papers I downloaded).
- A sentence in Zip-tries is worth knowing because it is wrong in a way that
  affects the paper: "The best previous sequential trie library, c-trie++, is
  an optimized implementation of the z-fast trie data structure ... c-trie++
  does not provide predecessor/successor queries, prefix search, or range
  queries." Third parties are evaluating "the z-fast trie" through c-trie++
  and its bit-oriented port.

## 3. Tries and trie hybrids

For each: data, mechanism, costs, measurements, memory, code, relation.
L is the key length in bytes, n the number of keys.

### 3.1 Judy

- **Doug Baskins. "A 10-minute description of how Judy arrays work and why
  they are so fast." October 16, 2001, revised July 2002.
  <http://judy.sourceforge.net/doc/10minutes.htm>** [web page, beginning
  read]; the detailed description is Alan Silverstein's "Judy IV Shop Manual"
  (2002) [not read].
- 256-ary digital tree with many adaptive, compressed node layouts chosen to
  minimise cache-line fills; JudySL handles strings as a tree of word-indexed
  arrays [from the Judy documentation as I remember it; not re-read]. O(L)
  node visits.
- No measurements taken from the source. Code: <http://judy.sourceforge.net/>,
  C, LGPL (stated on the page); ordered (first/next/prev). Hyperion's abstract
  names it with ART and HOT as the tries to beat.
- Relation: a classical top-down trie; historical baseline only.

### 3.2 Burst tries and HAT-tries

- **Steffen Heinz, Justin Zobel, Hugh E. Williams. "Burst tries: a fast,
  efficient data structure for string keys." ACM Transactions on Information
  Systems 20(2):192-223, 2002. DOI 10.1145/506309.506312.** [abstract]: an
  access trie whose leaves are small containers that "burst" into a trie node
  plus containers when they grow; "maintains the strings in sorted or
  near-sorted order".
- **Nikolas Askitis, Ranjan Sinha. "HAT-trie: A Cache-conscious Trie-based
  Data Structure for Strings." ACSC 2007, CRPIT vol. 62, pp. 97-105.** [not
  read; pages from the CRPIT site through a search result]; journal version
  **"Engineering scalable, cache and space efficient tries for strings." The
  VLDB Journal 19(5):633-660, 2010. DOI 10.1007/s00778-010-0183-9.** [not
  read]: a burst trie whose containers are cache-conscious array hash tables.
- Code: <https://github.com/Tessil/hat-trie> (C++, MIT, third party, active).
  Its README states "Keys are not ordered as they are partially stored in a
  hash map"; it offers prefix enumeration (`equal_prefix_range`) and
  `longest_prefix`, not `lower_bound`.
- Relation: bucketing below a trie, like Wormhole, but the trie is walked top
  down. Not an ordered index in the sense needed here; cite, do not benchmark.

### 3.3 ART

- **Viktor Leis, Alfons Kemper, Thomas Neumann. "The adaptive radix tree:
  ARTful indexing for main-memory databases." ICDE 2013, pp. 38-49. DOI
  10.1109/ICDE.2013.6544812.** [paper, read in part]. Synchronisation:
  **Viktor Leis, Florian Scheibner, Alfons Kemper, Thomas Neumann. "The ART of
  practical synchronization." DaMoN 2016. DOI 10.1145/2933349.2933352.**
  [bibliographic record]
- Byte-span radix tree with four node sizes (4, 16, 48, 256 children), path
  compression and lazy expansion. Lookup O(L) node visits, independent of n.
- Measurements in the paper: Core i7-3930K, **32-bit integer keys only**
  (dense and sparse; 65K, 16M, 256M keys; Figure 10) and TPC-C in HyPer; no
  string micro-benchmark. Space: "often as low as 8.1 bytes per key", worst
  case 52 bytes per key.
- Later string measurements of ART by others: Wormhole paper, Az1, one
  thread: 0.834 Mops. HOT paper, 50M URLs: 0.6 Mops (plot). RSS paper: 1592
  ns on about 100M URLs, 15.6 GB. Muller et al.: 34-37% slower than the
  adaptive B-tree on string lookups. Memory explodes on long keys when keys
  are stored in the tree (RSS), or it needs the keys elsewhere.
- Code: `armon/libart` (C, BSD-style licence, last push 2022; no range scan
  according to the Wormhole and LITS papers, which is why LITS "add[s] a scan
  procedure"); `flode/ARTSynchronized` (the authors' OLC and ROWEX versions,
  Apache-2.0); `wangziqi2016/index-microbench` (ARTOLC used by the Cuckoo
  Trie and FB+-tree papers; no licence file); `unodb-dev/unodb` (C++,
  Apache-2.0, active). Rust: `blart` (BTreeMap-like API, active), `rart`,
  `art`, `congee` (ART-OLC, fixed-size keys) on crates.io [crates.io records
  only; none appears in a paper].
- Relation: the canonical O(L) competitor; the z-fast trie replaces its
  top-down walk with O(log L) probes but each probe is a cache miss, whereas
  the top levels of an ART are cached.

### 3.4 Masstree

- **Yandong Mao, Eddie Kohler, Robert Tappan Morris. "Cache craftiness for
  fast multicore key-value storage." EuroSys 2012, pp. 183-196. DOI
  10.1145/2168836.2168855.** [paper, read in part]
- A trie with fanout 2^64 whose nodes are B+-trees (width 15) over 8-byte key
  slices; a lookup costs O(L/8) layers, each a B+-tree descent; keys sharing
  long prefixes create many layers. Optimistic concurrency with version
  numbers; permutation-based leaf updates.
- Measurements: 48-core AMD Opteron 8431 server, 16 cores used; 140M keys of
  "1-to-10-byte decimal" strings: 8.03 Mreq/s for gets on 16 cores without
  network (footnote 5); with 80M 8-byte keys 9.84 Mreq/s, 0.8% below a
  fixed-key B-tree. Later: "its throughput drops quickly with longer keys"
  (Wormhole paper, Figure 10); HOT paper, 50M URLs: 0.9 Mops (plot), and the
  largest memory of the four structures there.
- Code: <https://github.com/kohler/masstree-beta> (C++, MIT-style licence,
  last push 2023-10-12; `./configure && make`). Concurrent by design; a
  single-threaded benchmark must go through its `threadinfo` machinery. Get,
  put, remove, forward and reverse scan (`scan`, `rscan` in
  `masstree_scan.hh`).
- Relation: long-key behaviour is its weak point; the z-fast trie is the
  opposite design (cost in log L rather than L/8). A benchmark with long
  shared prefixes should include it for that contrast only.

### 3.5 HOT

- **Robert Binna, Eva Zangerle, Martin Pichl, Gunther Specht, Viktor Leis.
  "HOT: A Height Optimized Trie Index for Main-Memory Database Systems."
  SIGMOD 2018, pp. 521-534. DOI 10.1145/3183713.3196896.** [paper]. Journal
  version: **"Height Optimized Tries." ACM Transactions on Database Systems
  47(1), 2022, 46 pages. DOI 10.1145/3506692.** [abstract]
- A binary Patricia trie whose nodes are grouped into compound nodes of
  fanout at most 32 by a height-optimising rule; a compound node stores the
  discriminating bit positions and "sparse partial keys" searched with
  PEXT/AVX2. Leaves hold tuple identifiers only, so a lookup must verify the
  key in the record. "For a fixed maximum node fanout, the overall tree
  height is minimal and its structure is deterministically defined" (TODS
  abstract).
- Measurements: i7-6700 (3.4 GHz, 8 MB L3), GCC 7.2; index-microbench/YCSB;
  load 50M keys in random order, then 100M operations. Datasets: `url`
  (72.7M distinct DBpedia 2016-10 URLs, at most 255 bytes, average 55 B),
  `email` (average 23 B), `yago` (63-bit triples), `integer`. URL, workload C
  (plot labels): HOT 1.3, Masstree 0.9, ART 0.6, B-tree 0.3 Mops; "at least
  25% higher throughput across all evaluated data sets" on lookups; workload E
  (scans of up to 100) on URLs 200% higher than the others. Memory: 11.4 to
  14.4 bytes per key (Section 6.3), below the raw key size on strings.
- Independent: 786 ns on about 100M `uk-2007-05` URLs, 1.37 GB (RSS paper,
  Table 1); 1.27 Mops on 63M CommonCrawl URLs (LITS paper); 5% faster to 14%
  slower than the adaptive B-tree on string lookups (Muller et al.); 0.917
  Mops on MemeTracker URLs versus Wormhole 1.155 (Wormhole issue #5); "HOT
  gives the best performance and scalability" on YCSB-C in the FB+-tree
  paper (multi-threaded).
- Code: <https://github.com/speedskater/hot> (C++14, ISC licence, last commit
  2018-03-26; header-only for the single-threaded version). Requirements:
  x86-64 with AVX2 and BMI2 (it will not run on ARM). `HOTSingleThreaded`
  offers `insert`, `upsert`, `lookup`, `remove`, `scan`, `lower_bound`,
  `upper_bound`, `begin/end` [code]. Keys are produced from the stored value
  by a key extractor; C-string keys must be NUL-terminated and the maximal
  key length is a compile-time constant (255 by default for strings; the
  Wormhole author raised it to 2047, Muller et al. use 256).
- Relation: the strongest pure trie on strings, with the smallest footprint.
  It is the competitor on *memory*: 11-14 bytes per key against the dozen
  words per key of the z-fast trie.

### 3.6 Cuckoo Trie

- **Adar Zeitak, Adam Morrison. "Cuckoo Trie: Exploiting Memory-Level
  Parallelism for Efficient DRAM Indexing." SOSP 2021, pp. 147-162. DOI
  10.1145/3477132.3483551. arXiv:2201.09331.** [paper]
- A path-compressed trie of the *unique prefixes* of the keys (5-bit symbols)
  whose nodes are entries of a bucketised cuckoo hash table addressed by the
  hash of the node's name; entries are 15 bytes and store only the last
  symbol plus metadata ("key elimination", verified through a "peelable"
  hash). Because the names of all nodes on the search path are prefixes of the
  key, the buckets of the next D = 5 levels are prefetched at once: the
  descent is linear in the depth but the DRAM accesses overlap.
- Costs: O(depth) node probes, two buckets each, about D in flight; jump
  nodes do **not** save probes ("Path compression only saves space, not DRAM
  accesses"), so keys with long shared prefixes are its worst case.
- Measurements: 2 x Xeon Gold 6132, 2 MB pages, jemalloc, GCC 7.5; datasets
  rand-8 and rand-16 (200M random keys), `osm` (200M 64-bit OpenStreetMap
  cells), `az` (82M Amazon keys, average 35.7 B, average unique prefix 138
  bits), `reddit` (71M user names, average 10.9 B). YCSB A-F and LOAD,
  uniform, single- and multi-threaded. On YCSB-C with `osm` it is faster than
  HOT by 35%, ARTOLC by 20%, Wormhole by 50%, STX by 360%; on `az` it is
  slower than the others. Effective DRAM latency 33.5 cycles per access,
  "about 3x faster than in the indexes with serial accesses". Scans (YCSB-E)
  are slower than the others. Memory (Figure 11, plot): up to 28% below
  ARTOLC and Wormhole, up to 156% above HOT and STX.
- Code: <https://github.com/cuckoo-trie/cuckoo-trie-code> (C, public domain
  (Unlicense), single commit history ending 2021-10-23; x86-64 Linux only;
  requires 2 MB huge pages). API: `ct_alloc(num_cells)`, `ct_insert`,
  `ct_update`, `ct_upsert`, `ct_lookup`, `ct_iter_goto` (lower bound),
  `ct_iter_next`. **No deletion, no resizing** ("once the trie is allocated
  its size remains fixed"), a maximal key length, and the README warns "It is
  not robust enough to be used as an index in your program". A single-threaded
  build needs `#define MULTITHREADING` commented out in `config.h`. The
  competitors as benchmarked are in
  <https://github.com/cuckoo-trie/other-benchmarked-indexes>.
- Relation: the same "trie nodes in a hash table keyed by prefix" as the
  z-map, without the binary search. Its lesson for the z-fast trie: the number
  of *dependent* DRAM accesses matters, not the number of probes. The first
  probes of a fat binary search are data-independent only at the first step
  (the first length probed depends only on |x|); after that each probe
  depends on the outcome of the previous one, but there are only two possible
  next lengths, so both can be prefetched. [inference]

### 3.7 Hyperion

- **Markus Masker, Tim Suss, Lars Nagel, Lingfang Zeng, Andre Brinkmann.
  "Hyperion: Building the Largest In-memory Search Tree." SIGMOD 2019, pp.
  1207-1222. DOI 10.1145/3299869.3319870.** [abstract only; see Section 9]
- A trie-based key-value store that trades vectorised node search for
  density: it "scans the data structure linearly" inside containers and uses
  a custom allocator; claims a "performance-to-memory ratio ... more than two
  times better than the best implemented alternative strategy for randomized
  string data sets". No hashing, no binary search on lengths.
- Relation: a memory-efficiency point of comparison only.

### 3.8 SuRF / FST used as an index

- **Huanchen Zhang, Hyeontaek Lim, Viktor Leis, David G. Andersen, Michael
  Kaminsky, Kimberly Keeton, Andrew Pavlo. "SuRF: Practical Range Query
  Filtering with Fast Succinct Tries." SIGMOD 2018, pp. 323-336. DOI
  10.1145/3183713.3196931; "Succinct Range Filters." ACM Transactions on
  Database Systems 45(2), 2020. DOI 10.1145/3375660.** [paper, TODS version,
  through the extract]
- The Fast Succinct Trie is a *static* LOUDS-encoded trie (bitmap-encoded
  upper levels, sparse lower levels) of 10 bits per node that "matches the
  point and range query performance of state-of-the-art order-preserving
  indexes" such as B+-tree and ART (abstract). Code:
  <https://github.com/efficient/SuRF> (C++, Apache-2.0).
- Relation: static, so it competes with `StaticZFastTrie`, not with the
  dynamic structure. The earlier review (Section 8 there) has the current
  static numbers (C2-Marisa, PDT, FST, ART, c-trie++, z-fast trie).

### 3.9 Dynamic string dictionaries from the string-algorithms community

- **c-trie++**: Kazuya Tsuruta, Dominik Koppl, Shunsuke Kanda, Yuto
  Nakashima, Shunsuke Inenaga, Hideo Bannai, Masayuki Takeda. DCC 2020, pp.
  243-252, DOI 10.1109/DCC47342.2020.00032; Information and Computation 285,
  104794, 2022, DOI 10.1016/j.ic.2021.104794. [paper] "A hybrid of the z-fast
  trie and the packed c-trie"; its repository holds the only C++ z-fast trie
  ("ZFT", a port of Sux4J). No predecessor or range queries (per Zip-tries).
- **Dynamic path-decomposed tries**: Shunsuke Kanda, Dominik Koppl, Yasuo
  Tabei, Kazuhiro Morita, Masao Fuketa. ACM JEA 25, 2020. DOI
  10.1145/3418033. [bibliographic record]
- **Zip-tries**: David Eppstein, Ofek Gila, Michael T. Goodrich, Ryuto
  Kitagawa. "Zip-Tries: Simple Dynamic Data Structures for Strings." ACDA
  2025, pp. 130-143. DOI 10.1137/1.9781611979084.10. arXiv:2505.04953.
  [paper, read in part]. Randomised search trees with LCP metadata, O(k/alpha
  + log n); they describe the z-fast trie as achieving "O(k/alpha + log m)
  time, returning a correct result w.h.p., but ... complex and not easily
  parallelized" and compare experimentally only with c-trie++ on long genes
  (median length about 18,000 nucleotides, median LCP about 7). Code:
  <https://github.com/ofekih/ZipAndSkipTries> (MIT).
- **C2** (Kepan Zhang, Tiancheng Zhao, Helen Xu; arXiv:2606.16104, 2026)
  [paper, through the extract and the earlier review]: on 17.5M Wikipedia
  titles a "z-fast trie" takes 1192 ns per query at 546.6% of the data size,
  against ART 713 ns at 165.7% and c-trie++ 716 ns at 788.5%. This is the
  only third-party measurement of a z-fast trie in the systems literature,
  and it is unflattering.

## 4. B-trees done right for strings

### 4.1 The classical techniques

- **Rudolf Bayer, Karl Unterauer. "Prefix B-trees." ACM Transactions on
  Database Systems 2(1):11-26, 1977. DOI 10.1145/320521.320530.** [not read;
  described through Sun et al.]: *tail compression* (simple prefix B-trees:
  a split posts the shortest separator, not a full key, to the parent) and
  *head compression* (the common prefix of a page is stored once). Wormhole's
  anchors are Bayer-Unterauer shortest separators.
- **Goetz Graefe. "Modern B-Tree Techniques." Foundations and Trends in
  Databases 3(4):203-402, 2011. DOI 10.1561/1900000028**; update **"More
  Modern B-Tree Techniques." Foundations and Trends in Databases
  13(3):169-249, 2024. DOI 10.1561/1900000070.** [not read]: the reference
  for key normalisation (binary-comparable keys), prefix and suffix
  truncation, and poor man's normalised keys (a fixed-size key head in the
  slot); cited as such by Muller et al. See also **Goetz Graefe, Per-Ake
  Larson. "B-tree indexes and CPU caches." ICDE 2001, pp. 349-358. DOI
  10.1109/ICDE.2001.914847.** [not read]
- Cache-conscious layouts: **Jun Rao, Kenneth A. Ross. "Making B+-trees cache
  conscious in main memory." SIGMOD 2000, pp. 475-486. DOI
  10.1145/342009.335449** (CSB+-trees); **Philip Bohannon, Peter McIlroy,
  Rajeev Rastogi. "Main-memory index structures with fixed-size partial
  keys." SIGMOD 2001, pp. 163-174. DOI 10.1145/375663.375681** (partial-key
  B-trees: a fixed-size fragment of the key next to the pointer avoids most
  dereferences; cited by the Masstree paper for exactly that); **Changkyu Kim
  et al. "FAST: fast architecture sensitive tree search on modern CPUs and
  GPUs." SIGMOD 2010, pp. 339-350. DOI 10.1145/1807167.1807206** (integers
  only). [bibliographic records only]
- The String B-tree (Ferragina and Grossi, J. ACM 46(2), 1999, DOI
  10.1145/301970.301973) is already in the paper's bibliography.

### 4.2 Widely used libraries

- **STX B+-tree** (Timo Bingmann, <https://panthema.net/2007/stx-btree/>,
  2007-2013), since merged into **TLX** (<https://github.com/tlx/tlx>, Boost
  Software License 1.0; the old repository says "OBSOLETE, contained in
  tlx"). It is the "B+ tree" of the Wormhole, HOT, Cuckoo Trie and FB+-tree
  papers. It is a fixed-size-key container: with strings each slot holds a
  pointer or an owning object, and every comparison dereferences it.
  Measured: 0.3 Mops on 50M URLs (HOT paper, plot); Muller et al. find their
  *baseline* B-tree 62-108% faster than TLX on strings and conclude "Widely
  used generic B-Trees perform poorly" (Insight 6).
- **cpp-btree / Abseil** (`absl::btree_map`, `absl::btree_set`,
  <https://github.com/abseil/abseil-cpp>, Apache-2.0). The paper's C
  experiments use Google's cpp-btree, which Abseil's containers descend from;
  the old `google/cpp-btree` repository no longer exists on GitHub. Like the
  earlier review, **I found no published measurement of the Abseil B-tree on
  string keys**.
- Rust's `std::collections::BTreeMap`/`BTreeSet` (B = 6, i.e. up to 11 keys
  per node) is the in-language baseline of the crate's own benchmarks; no
  literature.

### 4.3 Muller, Benson, Leis

- **Marcus Muller, Lawrence Benson, Viktor Leis. "B-Trees Are Back:
  Engineering Fast and Pageable Node Layouts." Proceedings of the ACM on
  Management of Data 3(1) (SIGMOD), article 14, 2025. DOI 10.1145/3709664.**
  [paper]
- Slotted-page B-tree for variable-size records with six layout
  optimisations: prefix truncation (per-node common prefix), *heads* (the
  first 4 bytes of the truncated key in the slot), *hints* (a sampled array of
  heads searched first), fingerprinting leaves (hash-based point lookups in
  the leaf, like Wormhole's tags), semi-dense and fully dense leaves for
  integers; an *adaptive* tree picks the leaf layout per node from the
  observed operation mix.
- Setup: Ryzen 9 7950X at 4.5 GHz, 300 MB of records, 8-byte values; `urls`
  (average 62 B), `wiki` (Wikipedia titles, average 23 B), dense and sparse
  32-bit integers; insert 90%, measure inserts on the remaining 10%, then 5M
  lookups and 5M scans (1-50 records), Zipfian 0.99; medians of five runs.
- Results: Section 1.7 for Wormhole. HOT is 5% faster to 14% slower on string
  lookups, ART 34-37% slower, TLX far slower; LITS is similar on `wiki` and
  42% slower on `urls` and "depends strongly on the size of the training set"
  (-69% to +41% when the bulk-load sample is reduced from 50% to 5%). For
  scans even the baseline B-tree is 18-73% faster than the better trie and
  the adaptive tree 85-219% faster. "Insight 5: B-Trees can compete with both
  tries for string keys but fall behind ART for integer keys. For scans, they
  are always superior." Truncated prefixes average 49 bytes on `urls` and 7.0
  on `wiki`; on `urls` only 38% of adjacent leaf keys have distinct heads.
- Code: <https://github.com/m-mueller678/btree24> (adaptive tree inside
  vmcache; contains `WhAdapter.cpp`, `HotBTreeAdapter.cpp`, `TlxWrapper.cpp`
  and vendored copies of Wormhole and HOT, i.e. a ready-made harness) and
  <https://github.com/m-mueller678/btree-cpp> (the individual layouts); both
  C++, last push 2025-01-09, **no licence file**, "not suitable for
  production use" (README). Lookup, insert, remove, scan with a callback.
- Follow-ups: I found none that changes the picture. Related evaluations:
  **Chuqing Gao, Shreya Ballijepalli, Jianguo Wang. "Revisiting B-tree
  Compression: An Experimental Study." Proc. ACM Manag. Data 2(3) (SIGMOD),
  article 169, 2024. DOI 10.1145/3654972**, extended as **Sikang Sun, Chuqing
  Gao, Shreya Ballijepalli, Jianguo Wang. "An Evaluation of B-tree Compression
  Techniques." The VLDB Journal 35, article 4, 2026 (online 28 November
  2025). DOI 10.1007/s00778-025-00950-8** [read in part]: seven compression
  schemes, "Head+Tail compression performs best"; datasets TPC-H strings (60M,
  average 130 B), WEBSPAM-UK2007 URLs (25M, average 112 B), Wikipedia titles
  (25M, average 19 B), MemeTracker URLs (5.5M, average 76 B); code
  <https://github.com/chuqingG/BtreeComp> (no licence file). And **Josef
  Schmeisser, Maximilian E. Schule, Viktor Leis, Thomas Neumann, Alfons
  Kemper. "B2-Tree: Cache-Friendly String Indexing within B-Trees." BTW 2021
  (LNI; pages not verified); journal version "B2-Tree:
  Page-Based String Indexing in Concurrent Environments", Datenbank-Spektrum
  22:11-22, 2022, DOI 10.1007/s13222-022-00409-y** [abstract]: a small trie
  embedded in every B-tree page.

### 4.4 FB+-tree

- **Yuan Chen, Ao Li, Wenhai Li, Lingfeng Deng. "FB+-tree: A Memory-Optimized
  B+-tree with Latch-Free Update." PVLDB 18(6):1579-1592, 2025. DOI
  10.14778/3725688.3725691. arXiv:2503.23397.** [paper, read in part]
- Inner nodes store the common prefix of their anchors and then compare a few
  following bytes ("features") of all anchors at once with SIMD, falling back
  to suffix comparison; leaves are unsorted with hash tags and sorted lazily
  for scans (as in Wormhole); optimistic, partly latch-free concurrency.
- Setup: 2 x Xeon Gold 6248R, jemalloc, GCC 11.4; 100M keys loaded in random
  order, then 100M operations, YCSB LOAD/A/C/E, Zipfian 0.99; datasets
  rand-int (8 B), 3-gram (average 15.8 B), ycsb (22.9 B), twitter cluster-27
  (52.7 B), url (DBpedia, 76.8 B). Single-threaded it beats STX and
  B+-treeOLC "by up to 2.5x (LOAD), 2.9x (YCSB-A), 2.9x (YCSB-C), and 2.2x
  (YCSB-E)" (Figure 11). Against tries (multi-threaded, Figure 17, plots) it
  "performs almost as fast as other trie-based structures" on reads, where
  HOT is best. No learned index is included ("these hardly support insert").
- Code: <https://github.com/Spear-Neil/IndexResearch> (C++17, MIT, last push
  2026-06-10; needs AVX2); get, put, update, remove, `lower_bound`,
  `upper_bound`, iterators per the paper (not checked in the code).

### 4.5 What is the strongest B-tree baseline for strings today?

- Measured: the **adaptive B-tree of Muller et al.** is the only B-tree shown
  within 30% of Wormhole on string lookups and ahead of it on inserts and
  scans, on one machine and two datasets, by its own authors. FB+-tree claims
  2-3x over STX/B+-treeOLC but was not compared with Muller's tree, and
  neither paper cites the other (both appeared in early 2025).
- [inference] A fair "B-tree" baseline for the z-fast paper therefore needs
  at least prefix truncation and in-slot key heads; a pointer-per-key B-tree
  (STX/TLX with string objects, Rust's `BTreeSet<Vec<u8>>`, and probably the
  cpp-btree used for the paper's tables, if it stored pointers to the URLs)
  is the *weak* baseline. The paper's claim that the z-fast trie is "more
  than two times faster" than a B-tree on a billion URLs is a claim against
  that weak baseline.

## 5. Learned and hybrid string indexes

- **SIndex**: Youyun Wang, Chuzhe Tang, Zhaoguo Wang, Haibo Chen. "SIndex: a
  scalable learned index for string keys." APSys 2020, pp. 17-24. DOI
  10.1145/3409963.3410496. [abstract; LITS's description]. Groups keys by
  shared prefix and trains piecewise linear models on the distinguishing
  part; keys must be padded to a uniform length (per LITS). Claims "up to 91%
  better performance" than other indexes. In LITS's measurements it is the
  slowest structure (LITS is 2.26x-3.91x faster on reads). Code: in
  <https://github.com/curtis-sun/TLI> (GPL-3.0) as used by LITS; the original
  repository was not reachable from here.
- **RSS**: Benjamin Spector, Andreas Kipf, Kapil Vaidya, Chi Wang, Umar Farooq
  Minhas, Tim Kraska. "Bounding the Last Mile: Efficient Learned String
  Indexing." AIDB workshop at VLDB 2021. arXiv:2111.14905 (no DOI). [paper;
  author list from the earlier review, I re-read the tables]. Static tree of
  radix splines over 8- or 16-byte chunks with redirection on collisions and
  an optional hash corrector. Table 1, lookup (lower bound) in ns: Wiki (13M
  titles) ART 785, HOT 494 (584), RSS 629, RSS+HC 477 (629); URL (about 100M
  `uk-2007-05` URLs, about 10 GB) ART 1592, HOT 786 (920), RSS 1733, RSS+HC
  1314 (1733). Memory on URL: ART 15,573 MB, HOT 1,372 MB, RSS 199 MB, RSS+HC
  351 MB. Static, no public code found by LITS's authors.
- **LITS**: Yifan Yang, Shimin Chen. "LITS: An Optimized Learned Index for
  Strings." PVLDB 17(11):3415-3427, 2024. DOI 10.14778/3681954.3682010;
  extended version arXiv:2407.11556; journal version "Learned Indices for
  String Keys with Optimized Performance and O(N) Space Cost", ACM TODS, 2026
  (online 17 August 2026), DOI 10.1145/3841175 [abstract only]. [paper]
  A LIPP-style collision-driven learned index whose model is a global
  hash-enhanced prefix table (conditional distribution of the next character
  given a hashed prefix) plus per-node linear models, with compact leaves and
  HOT subtries where the model fails. Setup: 2 x Xeon Platinum 8380, GCC 9.4,
  one thread; eleven datasets (Section 7.1); YCSB A-F with 80% of the keys
  bulk-loaded, plus insert-only and delete-only. Results: up to 1.93x HOT and
  2.23x ART on reads, "up to 2.43x and 2.27x" on mixed point workloads,
  comparable scans (slightly slower than HOT on E for `url`). The TODS
  abstract revises this to "up to 2.19x and 1.85x". Independent check (Muller
  et al.): comparable to the adaptive B-tree on `wiki`, 42% slower on `urls`,
  worse on scans, sensitive to the bulk-load sample. Code:
  <https://github.com/schencoding/lits> (C++, **no licence file**, last push
  2026-09-30); supports only bytes 1-127 in keys (Muller et al. had to
  escape); needs a bulk load to train.
- **HOPE** (Huanchen Zhang, Xiaoxuan Liu, David G. Andersen, Michael Kaminsky,
  Kimberly Keeton, Andrew Pavlo. "Order-Preserving Key Compression for
  In-Memory Search Trees." SIGMOD 2020, pp. 1601-1615. DOI
  10.1145/3318464.3380583) [paper, read in part] is orthogonal: an
  order-preserving dictionary encoder applied to keys before indexing, tested
  with SuRF, ART, HOT, B+-tree and prefix B+-tree on the Email, Wiki and URL
  sets. It shortens keys, which helps O(L) structures more than an O(log L)
  one. [the last clause is inference]
- 2023-2026: besides LITS and its journal version I found no learned index
  for strings with an evaluation against HOT/Wormhole. A search returned
  titles from 2026 venues that I could not verify to be about string keys, so
  I do not list them.
- Relation: learned string indexes win on space when static (RSS) and are
  unproven as dynamic ordered indexes; none is a necessary competitor. LITS's
  hash-enhanced prefix table is the only use of prefix hashing among them,
  and it predicts positions rather than locating trie nodes.

## 6. Skip lists (baseline only)

- **William Pugh. "Skip lists: a probabilistic alternative to balanced
  trees." Communications of the ACM 33(6):668-676, 1990. DOI
  10.1145/78973.78977.**
- In key-value stores: the memtable of LevelDB
  (<https://github.com/google/leveldb>, BSD-3-Clause) and of RocksDB
  (`InlineSkipList`); Redis sorted sets (hash table plus skip list). O(log n)
  key comparisons, each a pointer dereference to a key.
- Measured: Wormhole's lookups are up to 8.4x faster than LevelDB's skip list
  and its insertions "comparable ... on most keysets" (Wormhole paper); with
  the Cuckoo Trie inside Redis, lookup throughput equals the default hash
  table + skip list combination (Cuckoo Trie paper, Section 6.8).
- Use it only as the "what LSM engines do" baseline; in Rust,
  `crossbeam-skiplist`.

## 7. Synthesis 1: benchmark design

### 7.1 Datasets used in the literature

| Dataset | Keys, average length | Used by | Where |
|---|---|---|---|
| `uk-2007-05` URLs (LAW) | about 100M (RSS, about 10 GB); 25M sample, avg 104 B (HOPE) | SuRF/HOPE line, RSS | <http://data.law.di.unimi.it/webdata/uk-2007-05/uk-2007-05.urls.gz> (1.05 GB compressed; the URL printed in HOPE and RSS, `law.di.unimi.it/webdata/...urls.gz`, now returns 404) |
| WEBSPAM-UK2007 URLs (LAW) | 25M, avg 112 B | Sun et al. | LAW |
| MemeTracker URLs | 192M, avg 82 B (Wormhole); 5.5M, avg 76 B, max 21 KB (Sun et al.) | Wormhole, its issue #5 (`m9urls`), Sun et al. | <https://snap.stanford.edu/data/memetracker9.html> (Leskovec, Backstrom, Kleinberg, KDD 2009) |
| DBpedia 2016-10 URLs | 72.7M distinct, <= 255 B, avg 55 B (HOT); avg 76.8 B (FB+-tree) | HOT, FB+-tree | DBpedia downloads (the host did not answer from here) |
| CommonCrawl URLs | 63M, avg 64 B | LITS | <https://data.commoncrawl.org/> (crawl CC-MAIN-2023-14) |
| "urls" of Muller et al. | 300 MB, avg 62 B; a larger set avg 90 B | Muller et al. | source not stated in the part I read |
| Wikipedia titles | 14M, avg 21 B (HOPE); 13M (RSS); 43M, avg 15 B (LITS); avg 23 B (Muller); 25M, avg 19 B (Sun); 17.5M (C2) | many | <https://dumps.wikimedia.org/enwiki/latest/enwiki-latest-all-titles-in-ns0.gz> (110 MB) |
| Email addresses | 25M, avg 22-23 B, host-reversed, "real-world" with no source given (SuRF, HOPE, HOT); 45M synthetic from the Faker package (LITS) | SuRF, HOPE, HOT, LITS | no public source found; synthetic only |
| Amazon review metadata | 142M, avg 40 B, two field orders (Wormhole); 82M distinct, avg 35.7 B (Cuckoo Trie) | Wormhole, Cuckoo Trie | <https://nijianmo.github.io/amazon/index.html> ("2014 version", per Wormhole issue #5); McAuley and Yang, WWW 2016 |
| Reddit user names | 71M, avg 10.9 B (Cuckoo Trie); 26M, avg 11 B (LITS) | Cuckoo Trie, LITS | pushshift `RA_2018-09.gz` (host unreachable from here); the Kaggle copy cited by LITS returned 404 to a scripted request |
| OpenAddresses | 34M, avg 24 B (US West) | LITS, Wormhole issue #5 | <https://openaddresses.io/> |
| DBLP titles / URLs / ee | 7M, avg 76 B (LITS); about 4M (Wormhole issue) | LITS, Wormhole issue #5 | <https://dblp.org/xml/dblp.xml.gz> |
| GeoNames, IMDb names | 7M avg 13 B; 9M avg 13 B | LITS | <https://datasets.imdbws.com/name.basics.tsv.gz>; GeoNames |
| Twitter cache trace, cluster 27 | avg 52.7 B, anonymised keys | FB+-tree | <https://github.com/twitter/cache-trace> (Yang, Yue, Rashmi; ACM TOS 17(3), 2021) |
| 3-gram | avg 15.8 B | FB+-tree | <https://www.statmt.org/lm-benchmark/> |
| YCSB keys | avg 22.9 B ("user" + number) | FB+-tree, index-microbench | <https://github.com/brianfrankcooper/YCSB> |
| Yago2 triples, OSM cells, random 8/16-byte integers | 8 B | HOT, Cuckoo Trie, all | synthetic or SOSD |
| Random strings of fixed length 8-1024 B; long constant prefix + 4 random bytes | fixed | Wormhole (K3-K10, K_short/K_long) | synthetic |
| Synthetic id-card (18 B), phone (17 B), random strings (32 B) | 50-63M | LITS | synthetic |
| TPC-H strings | 60M, avg 130 B | Sun et al. | dbgen |
| Biosynthetic gene clusters (ABC HuMi) | median 18,000 nucleotides, LCP about 7 | Zip-tries | see that paper |
| `eu-2015` URLs (LAW) | 1M and 1 billion | the z-fast paper | <https://law.di.unimi.it/webdata/eu-2015/> |

Observations:

- Sizes are 25M to 200M keys in almost every paper; nobody else reports a
  billion strings. The z-fast paper's billion-URL experiment is outside the
  range where the competitors have been measured, which is both its
  opportunity (the Cuckoo Trie paper shows tree-shaped indexes slowing by
  1.7x from 10M to 640M keys while Wormhole stays flat) and a risk (a
  reviewer will ask for 50-200M, where everything is published).
- URL sets differ a lot in shared-prefix structure: `uk-2007-05` and
  WEBSPAM-UK2007 are single-country crawls with long shared prefixes (average
  104-112 B), MemeTracker and DBpedia are shorter. Report the average LCP of
  adjacent sorted keys (or LITS's "group partial key length") with each set.
- The standard trio since SuRF is **email / wiki / url**; with emails not
  public, use wiki and two URL sets, plus one "compound key" set (Amazon
  item-user-time is the customary one and is the Cuckoo Trie's worst case)
  and fixed-length random keys (8, 16, 64, 256, 1024 bytes) to plot the
  dependence on L, and Wormhole's K_long construction (long shared prefix) to
  plot the dependence on anchor/extent length.

### 7.2 Operations and metrics that are standard

- **Workloads**: YCSB core workloads through `index-microbench` (Zhang et
  al., SIGMOD 2016; Wang et al., SIGMOD 2018;
  <https://github.com/wangziqi2016/index-microbench>, no licence file): LOAD
  (insert all, random order), C (lookups of existing keys), A/B/F (lookups
  with updates of existing keys), D (95% lookups of recent keys, 5% inserts),
  E (95% scans of up to 100 keys from a random start, 5% inserts); uniform and
  Zipfian (0.99). Muller et al. use scans of 1-50; Wormhole uses seek + 100.
- **Rarely measured, and worth measuring because they are the z-fast trie's
  selling points**: lookups of *absent* keys (only RSS's lower bound and the
  z-fast paper's Contains-), **predecessor** (no competitor paper measures
  it; Wormhole and the Cuckoo Trie have no backward operation), prefix
  counting/enumeration, deletions (only LITS; the Cuckoo Trie has none),
  existential range queries.
- **Metrics**: throughput in Mops (single-threaded first; most papers then
  scale threads), sometimes ns per operation; index memory excluding the
  key-value records (HOT, Cuckoo Trie, FB+-tree, LITS) or including them
  (Wormhole, Muller et al.: state which); bytes per key; hardware counters
  per operation (LLC misses, branch misses, instructions, cycles: HOT
  appendix, Cuckoo Trie Figure 2, FB+-tree Figure 16, Muller et al.); build
  or bulk-load time; throughput as a function of n (Cuckoo Trie Figure 9) and
  of key length (Wormhole Figure 14).
- **Methodology details that change results**: 2 MB huge pages (Cuckoo Trie:
  required; Wormhole: used when available; the paper's 32-billion-integer
  table also used them) and the allocator (jemalloc in Cuckoo Trie and
  FB+-tree); whether values are 8-byte tuple identifiers or pointers to
  records that must be dereferenced to verify the key (HOT and ART need the
  key from the record; Wormhole and B-trees hold or point to it); whether the
  hash of the key is computed inside the timed region (Wormhole's low-level
  API expects it precomputed); order of insertion (random everywhere);
  medians of several runs; pinning and frequency scaling.

### 7.3 Competitors to include, ranked

All URLs were checked on 2026-11-19 through the GitHub API or a clone.

1. **Wormhole**, `whunsafe` API. <https://github.com/wuxb45/wormhole>. C,
   `GPL-3.0-only OR MIT`, active (2026-10-11). `make` (clang, or `CCC=gcc`),
   link `libwh.so` or compile `wh.c lib.c kv.c` in. Run with `WH_KPN` 64, 128
   and 256. Lookup, insert, delete, seek (lower bound), forward scan. The bar
   for point lookups on strings; no predecessor.
2. **HOT**, single-threaded. <https://github.com/speedskater/hot>. C++14,
   ISC, unmaintained since 2018 but header-only and stable. `git submodule
   update --init --recursive; ./releaseBuild.sh`, or include `libs/hot` and
   `libs/idx/content-helpers` and compile with `-march=native` (AVX2 and BMI2
   required, so x86-64 only). Lookup, insert, upsert, remove, scan,
   `lower_bound`/`upper_bound`. Raise the maximal string key length if keys
   exceed 255 bytes; keys with zero bytes are not supported. The bar for
   memory and for tries.
3. **The adaptive B-tree of Muller et al.**
   <https://github.com/m-mueller678/btree24> (and `btree-cpp` for single
   layouts). C++, no licence (ask the authors before redistributing
   anything), research code built by `script/build-tagged.sh`. It already
   wraps Wormhole, HOT and TLX behind one interface, so the cheapest path to
   a credible comparison may be to add a z-fast adapter to *their* harness
   (through a C ABI of the Rust crate). The strongest B-tree.
4. **ART**. For C/C++: `armon/libart` (C; the choice of the Wormhole and
   LITS papers, weakest variant, no ordered seek in its API as far as those
   papers say) or `unodb-dev/unodb` (C++, Apache-2.0, maintained) or ARTOLC
   from `wangziqi2016/index-microbench`. In Rust: `blart` (crates.io), which
   has a `BTreeMap`-like ordered API and removes the FFI and allocator
   differences. The reference top-down O(L) structure.
5. **Cuckoo Trie**. <https://github.com/cuckoo-trie/cuckoo-trie-code>. C,
   public domain, x86-64 Linux, huge pages mandatory, `make` after commenting
   out `MULTITHREADING`; preallocate with `ct_alloc(2n)`. No delete, no
   growth, bounded key length, prototype quality. Include on 8/16-byte and
   short string keys, where it is the fastest published structure, and on one
   long-prefix set to reproduce its known weakness.
6. **Masstree**. <https://github.com/kohler/masstree-beta>. C++, MIT-style.
   `./bootstrap.sh; ./configure --disable-assertions; make`. Only for the
   long-key contrast and because every paper includes it.
7. **FB+-tree**. <https://github.com/Spear-Neil/IndexResearch>. C++17, MIT.
   The newest B+-tree variant with string support; AVX2.
8. **LITS**. <https://github.com/schencoding/lits>. C++, no licence. Only if
   a learned index is wanted; needs bulk loading and ASCII keys without zero.
9. **Baselines**: Rust `BTreeSet<Box<[u8]>>` (already used); Abseil
   `btree_set<std::string>` or the cpp-btree already used by the C code; TLX
   B+-tree (to connect with the numbers in other papers); a skip list
   (LevelDB's or `crossbeam-skiplist`); a red-black tree if the old tables are
   kept; and an unordered hash table (hashbrown, or `libcuckoo` as in the
   Wormhole paper) as the upper bound for point lookups.
10. **For the static structure** (`StaticZFastTrie`), a different list: FST
    (`efficient/SuRF`), path-decomposed tries, C2-Marisa, plain binary search
    and a static B-tree/Eytzinger layout over the same packed keys; see the
    earlier review, Section 8.

Not worth including: Judy, burst tries/HAT-tries (not ordered or obsolete),
Hyperion (no code located), RSS (static, no code), SIndex (dominated),
c-trie++ (no ordered queries), Bw-tree.

## 8. Synthesis 2: positioning of the z-fast trie

"Measured" below means measured in the cited literature for the competitor;
nothing in the literature measures a byte-level z-fast trie. Everything said
about the z-fast trie itself is inference unless it comes from the paper's
own tables.

### 8.1 Why one key per leaf only matches a B-tree at 5 million URLs [inference]

- A fat binary search on a 60-100-byte key makes about log2(8 x 80) = 9 or 10
  probes at bit level, 6 or 7 at byte level, plus the final node and key
  accesses. Each probe lands in a z-map with n - 1 entries (5M entries of at
  least 16 bytes: 80+ MB) and then, when it hits, in a node: nothing is
  cache-resident, and the probes are sequentially dependent. That is the
  observed "about ten sequential cache misses".
- A B-tree needs log_f n node visits. Rust's `BTreeSet` has at most 11 keys
  per node, i.e. 7 or 8 levels for 5M keys, of which the top ones are cached;
  with out-of-line keys each comparison may add a miss, with key heads or
  prefix truncation it does not. So "about ten misses" on both sides is what
  one expects, and it says that the baseline is weak as much as that the
  trie is slow.
- Wormhole makes about the same *number* of probes, log2(40) = 5 or 6 on URLs
  with 40-byte anchors, but into a table that only holds the prefixes of n/96
  anchors, and its optimistic pass reads only 16-byte tag slots: the whole
  metadata table is 0.34%-3.7% of the index (measured, Section 1.4), i.e.
  mostly in L2/L3. Its DRAM-bound accesses are essentially two: the leaf's
  tag array and the key. This, not the asymptotics, is why it is the fastest
  ordered string index (measured: Section 1.7).

### 8.2 Against each family

- **Wormhole.** Measured for Wormhole, one thread: 790 ns (paper) and 555 ns
  (author's 2021 rerun) per lookup on 142M Amazon keys of 40 bytes, 866 ns on
  MemeTracker URLs; 30-92% of a hash table's throughput, degraded to 40% when
  anchors are 512 bytes long; no predecessor; scans 1.5x to 5x slower than an
  engineered B-tree. From the algorithm, not measured: split cost
  proportional to anchor length. Inference for the z-fast trie: with one key
  per leaf it will lose on point lookups by a factor of 3 or more at any n
  where Wormhole's table is cache-resident. A *bucketed* z-fast trie (leaves
  of 64-256 keys with tag arrays; z-map over the handles of the compacted
  trie of the separators) would have the same two DRAM accesses, a table of
  (number of leaves - 1) entries instead of one entry per anchor prefix, O(1)
  expected table updates per split instead of Theta(anchor length), and exact
  bidirectional predecessor/successor. Where it could win: long anchors
  (URLs within one site, Wormhole's K_long, keys of hundreds of bytes), where
  Wormhole's table grows linearly with anchor length and falls out of cache;
  build/insert time on such keys; table size in general. Where it would
  still tie or lose: short keys (8-16 bytes), where Wormhole needs 3-4 probes
  in a tiny table and the fat binary search has no room to be better.
- **HOT, ART, Masstree (top-down tries).** Measured: HOT 0.8-1.1
  microseconds per lookup on 50-190M URLs, 11-14 bytes per key; ART about
  twice slower on URLs and much larger; Masstree degrades with key length;
  all slow down by about 1.7x from 10M to 640M keys (Cuckoo Trie, Figure 9,
  for HOT and ARTOLC on 8-byte keys). Inference: the z-fast trie's O(log L)
  is insensitive to n and to L, so its best cases are huge n and long keys
  with long shared prefixes that are *not* compressible into few nodes; on
  typical URL sets HOT's height is 7-8 compound nodes (LITS, Table 3) with
  the top levels cached, so a one-key-per-leaf z-fast trie at about ten
  misses loses. On memory it loses to HOT by an order of magnitude ("about a
  dozen machine words per element" versus 11-14 bytes; third-party
  measurement in C2: 547% of data size versus 166% for ART).
- **Cuckoo Trie.** Measured: fastest on 8/16-byte keys thanks to overlapped
  DRAM accesses; slower than the others on Amazon keys with long shared
  prefixes; no deletion. Inference: on long-prefix keys the z-fast trie makes
  O(log L) probes where the Cuckoo Trie makes O(L) (overlapped five at a
  time), so this is the regime where it should beat it; on short keys it will
  lose unless it also overlaps its probes (prefetch the two possible next
  probes of the fat binary search).
- **B-trees.** Measured: an engineered B-tree is within 30% of Wormhole and
  within 15% of HOT on string lookups at 300 MB, and best on scans by a wide
  margin; generic B-trees (TLX with string objects) are 2.5-3.5x slower than
  that. Measured by the authors (to be rerun): the z-fast trie is 2.1-2.4x
  faster than cpp-btree at a billion URLs and equal at a million. Inference:
  against a pointer-per-key B-tree the z-fast trie wins at large n because
  every comparison is a miss on a long key; against a prefix-truncating
  B-tree with key heads the crossover moves far to the right, perhaps beyond
  practical n, because the B-tree's inner nodes become cache-resident and
  comparisons stop touching the keys. Scans: a z-fast trie with one key per
  leaf walks a linked list of separately allocated strings and will lose to
  any B-tree; bucketing fixes this.
- **Learned indexes.** Measured: LITS up to about 2x HOT on point operations
  on its own machine, not confirmed by Muller et al.; static RSS 7-70x
  smaller than ART/HOT. Inference: not a threat for dynamic ordered
  workloads; for the static structure, RSS-like space (hundreds of MB for 100M
  URLs) is the number to compare `StaticZFastTrie` with.
- **Skip lists, red-black trees.** Measured: 5-8x slower than Wormhole. The
  z-fast trie beats them at scale (the paper's tables); this is no longer an
  interesting claim.

### 8.3 What the z-fast trie has that none of them has

- **Exact predecessor and successor, prefix and range-emptiness queries with
  O(log L) probes and no dependence on n**, in a structure with one table
  entry per key. Wormhole and the Cuckoo Trie only seek forward; HOT, ART and
  B-trees have both directions at O(height) cost. No paper in this survey
  benchmarks predecessor queries, existential prefix queries or range
  emptiness on strings; the z-fast paper can own those columns.
- **Bounds.** It is the only structure here with proved worst-case/expected
  bounds for variable-length keys; Wormhole's "O(log L) worst-case" ignores
  the O(L_anc) split cost and bucket overflows (its README documents
  unsupported key patterns), and none of the systems papers cites the theory.
- **A table independent of key length.** x-fast/Wormhole: all prefixes;
  Waldvogel: log L markers; z-fast: one handle. This is the one-sentence
  positioning with respect to the whole lineage.

### 8.4 Suggested claims for the paper [inference]

1. Relate Wormhole to y-fast tries and to Waldvogel et al. explicitly, and
   state that neither Wormhole nor Waldvogel et al. (SIGCOMM version) cites
   Willard: the systems lineage reinvented hashing of prefixes with binary
   search on lengths twice, each time storing all prefixes or markers.
2. Do not claim to beat "B-trees" without naming the layout; include an
   engineered B-tree or say plainly that the baseline stores pointers.
3. Report the bucketed variant if it is built: it is the apples-to-apples
   comparison with Wormhole, and the place where "one handle per node"
   becomes a measurable advantage (table size, split cost, long anchors).
4. Add the operations nobody else measures (predecessor, prefix, range
   emptiness, absent keys) and the two sweeps nobody else combines (n up to a
   billion; key/shared-prefix length up to kilobytes).

## 9. Papers I could not access

For each: what I had, and what I needed.

1. **Marcel Waldvogel, George Varghese, Jon Turner, Bernhard Plattner.
   "Scalable high-speed prefix matching." ACM Transactions on Computer
   Systems 19(4):440-482, 2001. DOI 10.1145/502912.502914.** Abstract only
   (the copies at `arl.wustl.edu` and Konstanz did not resolve or returned
   another document). Needed: whether the journal version cites van Emde
   Boas/Willard; the refined marker and update bounds; measurements.
2. **Markus Masker, Tim Suss, Lars Nagel, Lingfang Zeng, Andre Brinkmann.
   "Hyperion: Building the Largest In-memory Search Tree." SIGMOD 2019, pp.
   1207-1222. DOI 10.1145/3299869.3319870.** Abstract only (ACM, core.ac.uk
   and the Loughborough repository refused the download). Needed: mechanism,
   datasets, throughput and bytes per key on strings, code availability.
3. **Youyun Wang, Chuzhe Tang, Zhaoguo Wang, Haibo Chen. "SIndex: a scalable
   learned index for string keys." APSys 2020, pp. 17-24. DOI
   10.1145/3409963.3410496.** Abstract only. Needed: mechanism details,
   datasets, numbers, code URL.
4. **Yifan Yang, Shimin Chen. "Learned Indices for String Keys with Optimized
   Performance and O(N) Space Cost." ACM Transactions on Database Systems,
   2026. DOI 10.1145/3841175.** Abstract only. Needed: **the context in which
   it cites "Dynamic Z-Fast Tries"** (OpenAlex lists the citation), and
   whether the journal version adds Wormhole or B-tree baselines.
5. **Robert Binna, Eva Zangerle, Martin Pichl, Gunther Specht, Viktor Leis.
   "Height Optimized Tries." ACM Transactions on Database Systems 47(1),
   2022. DOI 10.1145/3506692.** Abstract only. Needed: whether the journal
   version has new experiments (Wormhole, larger data) and updated numbers.
6. **Xingbo Wu, Fan Ni, Song Jiang. "Wormhole", the publisher's PDF. DOI
   10.1145/3302424.3303955.** I read arXiv v2 and compared the reference list
   deposited at Crossref (identical, 48 entries); I did not see the
   publisher's PDF, so its article number and any wording changes in the body
   are unchecked.
7. **Dan E. Willard. "Log-logarithmic worst-case range queries are possible
   in space Theta(N)." IPL 17(2):81-84, 1983. DOI
   10.1016/0020-0190(83)90075-3.** Not read; the description of x-fast and
   y-fast tries is from memory and from PIM-trie's summary.
8. **Rudolf Bayer, Karl Unterauer. "Prefix B-trees." ACM TODS 2(1):11-26,
   1977. DOI 10.1145/320521.320530.** Not read (described through Sun et
   al.).
9. **Goetz Graefe. "Modern B-Tree Techniques", FnT Databases 3(4), 2011, DOI
   10.1561/1900000028; "More Modern B-Tree Techniques", FnT Databases 13(3),
   2024, DOI 10.1561/1900000070; Graefe and Larson, "B-tree indexes and CPU
   caches", ICDE 2001, DOI 10.1109/ICDE.2001.914847.** Not read. Needed: the
   precise statements on normalised keys, prefix/suffix truncation and poor
   man's normalised keys, to cite the right sections.
10. **Jun Rao, Kenneth A. Ross, SIGMOD 2000 (DOI 10.1145/342009.335449);
    Philip Bohannon, Peter McIlroy, Rajeev Rastogi, SIGMOD 2001 (DOI
    10.1145/375663.375681); Changkyu Kim et al., SIGMOD 2010 (DOI
    10.1145/1807167.1807206).** Bibliographic records only.
11. **Nikolas Askitis, Ranjan Sinha. "HAT-trie", ACSC 2007, CRPIT 62, pp.
    97-105; "Engineering scalable, cache and space efficient tries for
    strings", VLDB Journal 19(5):633-660, 2010, DOI
    10.1007/s00778-010-0183-9.** Not read (the CRPIT host did not resolve).
12. **Steffen Heinz, Justin Zobel, Hugh E. Williams. "Burst tries", ACM TOIS
    20(2), 2002, DOI 10.1145/506309.506312.** Abstract only (I downloaded an
    author copy but read only the first page).
13. **Sarang Dharmapurikar, Praveen Krishnamurthy, David E. Taylor, SIGCOMM
    2003 (DOI 10.1145/863955.863979); Haowei Yuan, Patrick Crowley, ANCS 2015
    (DOI 10.1109/ANCS.2015.7110125); Donghong Jiang et al., APNet 2023 (DOI
    10.1145/3600061.3600077) and IEEE ToN 2025 (DOI
    10.1109/TNET.2024.3504244).** Not read; needed only if the related work
    discusses IP/name lookup in more than a sentence.
14. **Rotem Oshman, Nir Shavit. "The SkipTrie", PODC 2013. DOI
    10.1145/2484239.2484270.** Not read.
15. **Alan Silverstein. "Judy IV Shop Manual", 2002.** Not read.
16. **Josef Schmeisser et al. "B2-Tree", BTW 2021 and Datenbank-Spektrum 22,
    2022 (DOI 10.1007/s13222-022-00409-y).** Abstract only; the BTW page
    range in the report is unverified.
17. **Chuqing Gao, Shreya Ballijepalli, Jianguo Wang. "Revisiting B-tree
    Compression", SIGMOD 2024. DOI 10.1145/3654972.** First page only (I read
    the VLDB Journal extension in part).
18. **DBLP records for all entries.** `dblp.org` and `dblp.uni-trier.de`
    returned an anti-bot page to scripted requests; verification used
    Crossref instead, so DBLP-specific data (BibTeX keys, article numbers of
    ACM proceedings such as Wormhole's) are not checked.

Papers read only in part (enough for the statements made, not for more): ART,
Masstree, REMIX, HOPE, PIM-trie, Zip-tries, PlanB, the prefix hash tree
report, FB+-tree, LITS, Sun et al., Dementiev et al. (abstract; the PDF text
layer is damaged).
