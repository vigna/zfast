# Literature check: competitors and applications

Sub-review commissioned on 2026-11-16 during the z-fast trie project review; report delivered 2026-11-16/17.
It was produced by a separate reviewing agent that read the material in full. Claims marked as simulated were
checked by that agent with throwaway scripts (copied to `../scripts/` where they survived); they were not all
independently re-verified. See `../REVIEW.md` for which claims were spot-checked.

## Report

# Literature check: z-fast / zuffix indexes versus existing systems

All numbers below were read from the linked papers or repositories unless marked *(inference)* or *(unverified)*.

## 1. Wormhole
- **MetaTrieHT stores all prefixes of every anchor key**, not one entry per compacted node: "we insert all prefixes of each anchor into a hash table". There is one anchor per leaf, not per key. Internal items hold a child bitmap plus leftmost/rightmost leaf pointers; buckets are 64-byte slots of 8 × (16-bit tag + 48-bit pointer). [arXiv 1805.02200](https://arxiv.org/abs/1805.02200)
- **Lookup** is binary search on prefix lengths (citing Waldvogel) in O(log min(L_anc, L_key)) probes. Tags are trusted optimistically with one full comparison at the end; hashing is incremental CRC32c.
- **Leaves** hold at most 128 keys, with a tag array sorted by hash. A split inserts every prefix of the new anchor, so O(L_anc) hash insertions (my reading of Algorithm 4).
- **Memory** is "comparable to B+ tree and skip list"; the second RCU copy of the table is 0.34–3.7% of the index.
- **Speedups**: up to 8.4× skip list, 4.9× STX B+tree, 4.3× ART, 6.6× Masstree; 30–92% of a cuckoo hash table. On URLs (192M keys, avg 82 B, anchors about 40 B) it is at least 1.7× the best competitor.
- **Independent check**: Wormhole is still the fastest string lookup in [Müller et al., SIGMOD 2025](https://www.cs.cit.tum.de/fileadmin/w00cfj/dis/papers/btrees-are-back.pdf); their adaptive B-tree is 26–28% slower.
- **Implementation**: [wuxb45/wormhole](https://github.com/wuxb45/wormhole) is maintained (last commit 2026-10-11, anchors ≤ 65520 bytes). I found no follow-up that shrinks the hash table.

Bottom line: Wormhole is the direct competitor. A z-fast index stores one handle per node instead of every anchor prefix, so it should win on table size and build cost when anchors are long *(inference)*; Wormhole's lookup speed is the bar.

## 2. Other hash-on-prefix / binary-search-on-length indexes
- **Cuckoo Trie (SOSP 2021)**: trie nodes sit in a bucketized cuckoo table keyed by prefix, with 5-bit symbols and 15-byte entries. Descent is linear, not binary search, but about 5 node probes are prefetched in parallel. It is up to 35% faster than HOT, 50% faster than Wormhole and 360% faster than STX on 8-byte OSM keys, with up to 28% less memory than Wormhole; long shared prefixes hurt it. [arXiv 2201.09331](https://arxiv.org/abs/2201.09331)
- **Waldvogel et al. (SIGCOMM 1997)**: one hash table per prefix length, worst case log₂W probes, at most log₂W markers per prefix (26% extra entries on their backbone table). "Mutating" search brings the average below 2 probes. [paper](https://conferences.sigcomm.org/sigcomm/1997/papers/p182.pdf)
- **Named-data networking**: Yuan & Crowley (ANCS 2015) apply the same idea to name components: O(log k) string lookups, 10 Gbps with 10⁹ rules (abstract only). [IEEE](https://ieeexplore.ieee.org/document/7110125)
- **c-trie++**: a C++ dynamic trie described as "a hybrid of the z-fast trie and the packed c-trie". [arXiv 1904.07467](https://arxiv.org/abs/1904.07467)
- I found no production x-fast/y-fast system, only toy repositories.

Bottom line: the Cuckoo Trie shows that parallel prefetching of independent hash probes matters as much as probe count. Fat binary search probes are dependent, so that trick is harder to apply *(inference)*.

## 3. Comparison set for in-memory string indexes
- **HOT (SIGMOD 2018)**: variable bit span per node. On 50M URLs (avg 55 B), single-thread lookups run at 1.3 Mops for HOT, 0.9 Masstree, 0.6 ART, 0.3 B-tree (bar labels), using 11.4–14.4 bytes/key. [paper](https://dbis-informatik.uibk.ac.at/sites/default/files/2018-06/hot-height-optimized.pdf)
- **Cross-checks**: on about 100M URLs, HOT takes 786 ns and ART 1592 ns ([RSS paper](https://arxiv.org/abs/2111.14905)); on 63M URLs, HOT 1.27 Mops and ART 0.78 ([LITS, VLDB 2024](https://vldb.org/pvldb/vol17/p3415-chen.pdf)).
- **ART / Masstree**: ART uses byte-span adaptive nodes; Masstree is a trie of B+trees over 8-byte slices, and its throughput "drops quickly with longer keys" (Wormhole paper).
- **2022 onwards**: LITS (learned plus HOT subtries) claims up to 2.43× HOT. FB+-tree compares against ART, HOT, Masstree and Wormhole ([arXiv 2503.23397](https://arxiv.org/abs/2503.23397)). In Müller et al., HOT is 5% faster to 14% slower than the adaptive B-tree, ART is 34–37% slower, and TLX is much slower.
- I found no abseil/cpp-btree string numbers.
- **Benchmarks**: YCSB A–F via index-microbench; URLs (DBpedia, CommonCrawl, MemeTracker), emails, Wikipedia titles, Amazon reviews, reddit names, Twitter cache trace.

Bottom line: the bar is roughly 0.8 µs per lookup on 50–100M URLs (HOT), and Wormhole is faster still.

## 4. Range filters
- **String support**: [Diva (VLDB 2025)](https://www.vldb.org/pvldb/vol18/p3923-eslami.pdf), Table 2, lists only SuRF and Diva as supporting variable-length keys. Rosetta, REncoder, bloomRF, SNARF, Oasis+, Grafite and Memento are fixed-length.
- **Proteus** (FST plus prefix Bloom filter) handles strings by null-padding to a fixed length; it is the closest thing to prefix hashing on strings. [arXiv 2207.01503](https://arxiv.org/abs/2207.01503)
- **SuRF**: 10 bits per trie node; SuRF-Base uses 14 bits/key on emails at 25% false positives. [TODS](https://people.iiis.tsinghua.edu.cn/~huanchen/publications/surf-tods.pdf)
- **Grafite**: B bits/key, O(1) query, false-positive rate ≤ ℓ/2^(B−2). [arXiv 2311.15380](https://arxiv.org/abs/2311.15380)
- **Lower bound**: n·lg(L/ε) − O(n) bits. [arXiv 1407.2907](https://arxiv.org/abs/1407.2907)
- **Rosetta**: one Bloom filter per prefix length with dyadic decomposition (secondary sources). I found no filter that does binary search on prefix length.

Bottom line: string range filters are a thin field (SuRF, Diva, padded Proteus). A probabilistic z-fast structure is plausible there, but it has to compete at 10–20 bits/key.

## 5. FM-index / CSA cost on long patterns
- **Backward search costs Ω(m) cache misses** ([Grabowski & Cisłak](https://arxiv.org/abs/1512.01996)). Measured RAM latency is about 80 ns per miss ([QuadRank, SEA 2026](https://arxiv.org/abs/2602.04103)).
- **Measured, n = 10⁸, m = 100** ([STPD, ICALP 2026](https://arxiv.org/abs/2506.14734)):

| Index | ns per pattern character | Size |
|---|---|---|
| r-index | 572 | 0.9 MiB |
| Suffix array | 200 | 0.3 GiB |
| Suffix tree | 60 | 9.9 GiB |

- **Enhanced suffix array, 200 MiB texts**: 1.5–7.4 µs per query for m = 10–10⁴, at 4.7 GiB (about 24× the text) ([Engineering Zuffix Arrays](https://doi.org/10.4230/LIPIcs.SEA.2024.2)).
- **Mitigations**:
  - k-step FM-index: 1.5× at 2× space.
  - "Bloated" FM-index: O(m/|CL| + log n log m) misses at 80–95n bytes.
  - Batching and prefetching (QuadFm): 4× over genedex.
  - Move-r: 2–35× over r-index variants at about 2× size ([SEA 2024](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.SEA.2024.1)).
  - Movi: about 16× over r-index at 3–11× space; PFP-FM: 3.9–7.1× over RLCSA ([survey](https://doi.org/10.4230/OASIcs.Manzini.6)).
- **Suffixient arrays**: binary search plus text access, O((1+m/B)·d·log χ) I/Os, as low as 3.5 ns/char, and 1–2 orders of magnitude faster than the r-index. [arXiv 2407.18753](https://arxiv.org/abs/2407.18753)
- I could not extract verified ns/char for sdsl `csa_wt`; the sources I found give plots only.

Bottom line: any suffix-array-style index beats FM/r-index by 10–100× on long patterns. The real competitors on time are the enhanced suffix array, suffixient/STPD indexes and learned suffix arrays; on space a zuffix array loses heavily.

## 6. External memory
- **RoSA**: in-memory blind tree over on-disk suffix blocks; 64 GB of text on a 4 GB laptop. It notes the FM-index "potentially make[s] m disk accesses". [arXiv 1303.6481](https://arxiv.org/abs/1303.6481)
- **infini-gram**: on-disk suffix array at 7 bytes/token, under 20 ms per count. [arXiv 2401.17377](https://arxiv.org/abs/2401.17377)
- **SoftMatcha 2 (2026)**: a staged suffix array with a sparse in-RAM level needs one disk access per exact lookup. It is about 33× faster than infini-gram, with p95 of 0.34 ms on 1.4T tokens. [arXiv 2602.10908](https://arxiv.org/abs/2602.10908)
- **Two-level dictionaries** (SPIRE 2023): succinct Patricia trie in RAM over rear-coded disk blocks; 3.5B strings (273 GB) indexed in under 200 MB (abstract only). [DOI](https://doi.org/10.1007/978-3-031-43980-3_16)
- I found no NVMe-era String B-tree evaluation.

Bottom line: the target is one random I/O per query with a small RAM index. The zuffix paper itself notes the tree is accessed only once, so the z-map-in-RAM, array-on-disk layout is the natural fit *(inference)*.

## 7. Zuffix arrays
- **Introduced** in "Kings, Name Days, Lazy Servants and Magic", FUN 2018 ([LIPIcs](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.FUN.2018.10)). Theory only: O((m log σ)/w + log m + σ) time.
- **Engineered** in "Engineering Zuffix Arrays", SEA 2024. Pizza&Chili texts (200 MiB), random and Fibonacci texts. The **only baseline is the enhanced suffix array**; there is no FM-index, r-index or plain suffix array.
- **Results**: up to 7× on large alphabets, about 2× on DNA, 1.5–3.5× even with one occurrence. At m = 10⁴ on DNA the gain disappears, and it is slower on Fibonacci words.
- **Space**: the z-map is 5–8 GiB on top of the 4.7 GiB enhanced suffix array. Hardware CRC32 was best; the signature variant was slower than memcmp. Code: [smarchini/zuffix](https://github.com/smarchini/zuffix).
- **Citations** (Semantic Scholar, may lag): 0 for the SEA paper; the FUN paper is cited by the suffixient and STPD papers, which use z-fast tries only in their theory. Their implementations use binary search plus tabulation.

Bottom line: there is no published comparison against FM-index, plain suffix array, suffixient or learned indexes. That gap is open.

## 8. Static string dictionaries
- **Weak prefix search** (ESA 2010): O(n log ℓ) bits, optimal. [arXiv 1804.04720](https://arxiv.org/abs/1804.04720)
- **Current numbers** from [C² (2026)](https://arxiv.org/abs/2606.16104), Wikipedia titles (17.5M keys, 359 MB), ns per query and size as % of data:

| Structure | ns/query | Size |
|---|---|---|
| C²-Marisa | 732 | 32.9% |
| PDT | 1022 | 30.1% |
| FST | 1424 | 37.9% |
| ART | 713 | 165.7% |
| c-trie++ | 716 | 788.5% |
| z-fast trie | 1192 | 546.6% |

- **Cache misses**: succinct tries take 56–82 LLC misses per query on that dataset (C-ART: 23).
- **Monotone hashing on URLs** ([LeMonHash](https://arxiv.org/abs/2304.11012)): LeMonHash-VL does 338 kq/s at 6.46 bits/key; Sux4J's ZFast does 198 kq/s at 8.77 bits/key.

Bottom line: to matter, a static byte z-fast index over front coding needs about 700 ns or less on Wikipedia titles at well under 100% of data size. The existing z-fast baseline loses on both axes. With a handful of hash probes plus one block it could beat the 50–80 misses of succinct tries *(inference)*.

## 9. Existing z-fast implementations
- **Sux4J** (Java) is used directly in the LeMonHash benchmark.
- **C++ port**: "ZFT: our z-fast trie portation from an implementation in Java to C++" ships with [c-trie++](https://gitlab.com/habatakitai/ctriepp). C² benchmarks a dynamic z-fast trie, presumably this port (it does not say).
- **Rust**: [MrBrionix/z-fast-trie-sux](https://github.com/MrBrionix/z-fast-trie-sux), undocumented, 0 stars.
- I found no use inside a database or key-value system.

Bottom line: the only third-party benchmark uses a bit-oriented dynamic port and shows it as slow and large. A tuned byte-based static version has no published precedent.

## 10. Seeding in read mappers
- **STAR**: maximal mappable prefix search "implemented through uncompressed suffix arrays". About 27 GB RAM for human; the sparse option drops below 16 GB at about 25% speed loss. [paper](https://academic.oup.com/bioinformatics/article/29/1/15/272537)
- **BWA-MEM2**: occ access "has no spatial or temporal locality… memory latency bound". The fix is η = 32 so one entry fits a cache line, plus prefetching and an uncompressed suffix array, giving SMEM about 2×. SMEM is 21–44% of runtime. [arXiv 1907.12931](https://arxiv.org/abs/1907.12931)
- **Index-for-speed trades**:

| System | Human index | Seeding speedup vs BWA-MEM2 |
|---|---|---|
| BWA-MEM2 (2019 design) | 42 GB (vs 4.3 GB BWA-MEM) | baseline |
| [ERT](https://www.biorxiv.org/content/10.1101/2020.03.23.003897) | about 60 GB | 2.1× (1.28× end to end) |
| [LISA](https://www.biorxiv.org/content/10.1101/2020.12.22.423964) | +33 GB exact / +92 GB SMEM | up to 4.7× |
| [BWA-MEME](https://www.biorxiv.org/content/10.1101/2021.09.01.457579) | 118 GB (38 GB minimum mode) | 3.45× |

- **MUMmer4**: 48-bit suffix array from essaMEM, 13–15 bytes/base, 39 GB suffix array for human. [paper](https://doi.org/10.1371/journal.pcbi.1005944)
- **copMEM**: hashing of sampled k-mers; human–mouse MEMs in under 2 minutes with under 10 GB. [arXiv 1805.08816](https://arxiv.org/abs/1805.08816)

Bottom line: yes, practitioners accept 10–40× the text (bytes per base) for 2–3.5× faster seeding. A hash-accelerated suffix array must beat BWA-MEME and ERT at 60–118 GB or less.

## 11. Learned / hybrid static string indexes and SSTable indexes
- **RSS** on URLs: 1733 ns (1314 ns with hash corrector) versus HOT 786 ns and ART 1592 ns. Memory is 199–351 MB versus 1.37 GB (HOT) and 15.6 GB (ART). On wiki it takes 629 / 477 ns versus HOT 494 ns.
- **LITS** reports that RSS spends over 70% of its time in last-mile search; SIndex is slower still.
- **SSTables**: RocksDB binary-searches restart points. The [data-block hash index](https://rocksdb.org/blog/2018/08/23/data-block-hash-index.html) cuts Seek CPU by 21.8% and raises throughput by about 10% for 4.6% space. Upstream now has an **experimental FST/LOUDS trie index** for SST index blocks ([source](https://github.com/facebook/rocksdb/tree/main/utilities/trie_index)); I found no published benchmark of it.

Bottom line: learned string indexes win on space, not time. SSTable index blocks are an open, practical target for a static prefix-hash index.

## Summary table

| Application | Strongest competitor | Could win on | Will lose on | Confidence |
|---|---|---|---|---|
| In-memory ordered string index | Wormhole, HOT | hash-table size and build with long shared prefixes | raw lookup vs Wormhole; scans vs B-trees | Medium (mechanism sourced, win inferred) |
| Static string dictionary / monotone hash | C²-Marisa, PDT, LeMonHash-VL | cache misses and latency | space vs 20–35% of data | Medium |
| Pattern matching, long patterns, in RAM | enhanced suffix array, suffixient/STPD, Move-r | time vs FM/r-index (10–100×), vs enhanced suffix array (2–7×) | space (about 50× text); long DNA patterns | High vs enhanced suffix array (measured); medium vs others |
| On-disk text index | SoftMatcha 2, RoSA, infini-gram | random I/Os per query | RAM for the z-map | Low–medium (no experiments exist) |
| Read-mapper seeding | BWA-MEME, ERT, LISA | probes per seed | index size; SMEM-specific machinery | Low (no direct evidence) |
| String range filter / SSTable index | SuRF, Diva, RocksDB FST index | probe count on long keys | bits/key | Low (inference) |

Text extracts of the papers I read are in `/private/tmp/claude-501/-Users-vigna-pub-zfast/955cf1ac-f832-4902-86ae-acdd82779e8c/scratchpad/txt`.

## Brief given to the reviewer

You are doing a focused literature check using web search and by reading the primary sources (papers, official repositories). Report only what you can support with a source; give a URL for every claim, and say explicitly when you could not verify something. Do not rely on memory alone.

PRIVACY: keep search queries generic (paper titles, system names, technical terms). Do not put any person's name that is not part of a published paper title/citation you are looking up, and do not mention any unpublished project, in queries or URLs.

## Context
A "z-fast trie" is a string predecessor/prefix-search structure: a compacted trie plus ONE hash-table entry per internal node, keyed by a specially chosen prefix (the "handle", whose length is the number with the most trailing zeros in the node's skip interval). A query string x is located by a binary search on prefix LENGTHS ("fat binary search") that needs only O(log |x|) hash probes, given a hash function whose value on any prefix of x is available in O(1) after one linear pass over x. A "zuffix array" applies the same idea to a suffix array, to locate a pattern of length m with O(log m) random accesses plus sequential scanning, instead of the Θ(m) rank operations of backward search in an FM-index.

I want to know (a) which practical systems already use closely related ideas, so that a new byte-oriented implementation can be compared against them and learn from them, and (b) where such a structure could plausibly beat compressed suffix arrays / FM-indexes or the standard in-memory ordered indexes.

## Questions — answer each separately
1. **Wormhole** ("Wormhole: A Fast Ordered Index for In-memory Data Management", EuroSys 2019). How exactly does its MetaTrieHT work? Does it store ALL prefixes of each anchor key in the hash table, or one per trie node? What are its stated lookup/insert complexities, leaf-node size, memory overhead, and its measured speedups and against which baselines? Is there a maintained open-source implementation? Any follow-up work that reduces its hash-table size?
2. **Other hash-on-prefix or binary-search-on-length indexes**: Cuckoo Trie (SOSP 2021), and any other ordered index that hashes key prefixes (e.g. variants of x-fast/y-fast tries in practice, "binary search on prefix lengths" for IP lookup by Waldvogel et al. 1997 and its IPv6 / named-data-networking descendants). One paragraph each: idea, bounds, reported performance.
3. **The comparison set for an in-memory ordered string index today**: ART, HOT (Height Optimized Trie), Masstree, B-tree variants (e.g. cpp-btree/abseil btree), and anything newer that is considered state of the art for variable-length string keys (please check for work from 2022 onwards). For each: key idea, and typical reported lookup cost on string keys such as URLs. What benchmark suites/datasets are standard?
4. **Range filters for LSM-trees**: SuRF, Rosetta, SNARF, Proteus, REncoder, Grafite, Memento filter, and anything newer. Which support variable-length string keys, and what are their space (bits/key) and query-time characteristics? Is there published work using prefix hashing + binary search on prefix length for range emptiness (e.g. "approximate range emptiness" structures, prefix Bloom filters)?
5. **FM-index / compressed suffix array locate-and-count cost for LONG patterns**: what do the literature and benchmarks say about cache misses per pattern symbol in backward search, and which techniques mitigate it (e.g. k-step/"fixed-block" FM-indexes, cache-friendly rank layouts, bidirectional indexes, the r-index and move structure, learned or sampled suffix arrays, "suffix array + LCP" enhanced indexes, sparse suffix arrays)? I am looking for honest numbers: ns per character or per query, and index size relative to the text, for representative indexes (sdsl-lite csa_wt / FM-index, r-index, plain suffix array with binary search, enhanced suffix array).
6. **External-memory / on-disk string indexing**: String B-trees and practical successors; any recent work on pattern matching or string dictionaries on NVMe where the number of random I/Os per query (rather than space) is the bottleneck.
7. **Zuffix arrays in the literature**: find the paper "Engineering Zuffix Arrays" and the earlier paper that introduced zuffix arrays ("Kings, name days, lazy servants and magic", FUN 2018). Report venue/year, the headline experimental claims, the baselines used, and whether anyone has cited, reimplemented, or compared against them since.
8. **Static string dictionaries with prefix search in little space**: "Fast prefix search in little space, with applications" (ESA 2010) and the hollow / probabilistic z-fast trie; path-decomposed tries; Fast Succinct Tries (FST/LOUDS-DS); CoCo-trie; and any recent (2022+) compressed string dictionary considered state of the art. What are the space/time numbers that a new static byte-based z-fast index over a front-coded string list would have to beat?
9. **Existing z-fast trie implementations** outside the original authors' Java library (Sux4J): any open-source implementation in C++/Rust/Go, or use inside another system (e.g. as a component of a monotone minimal perfect hash function benchmark)?

## Output format
For each question: 3–8 bullet points with facts and URLs, then one line "Bottom line:" stating what it implies for where a byte-based z-fast/zuffix index could win or lose. Finish with a short table: candidate application → strongest existing competitor → metric on which a z-fast/zuffix approach could plausibly win → metric on which it will lose → how confident you are (based on sources, not speculation). At most ~1500 words overall. Flag clearly anything that is your inference rather than a sourced fact.
