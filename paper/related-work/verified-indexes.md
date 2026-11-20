# Practical string indexes: statements verified on the primary sources

Checked on 2026-11-20 against seven PDFs in `../../related/`. Subject: the
paragraph "Ordered indices for main memory" of the section "Related work"
(`sec:related`) and the section "A byte-oriented implementation in Rust"
(`sec:rust`) of `dynzfast.tex`, and the statements of `practical-indexes.md`
and `static-and-uses.md` about these seven papers.

**Line numbers** refer to `dynzfast.tex` as it was at the end of this work
(2605 lines). The file was edited by someone else while the report was being
prepared (it had 2596 lines at the first reading: five lines were added
before the paragraph "Ordered indices for main memory" and four in the
paragraph "Static dictionaries of strings"; the sentences examined here did
not change). Every proposed edit quotes the old text: locate edits by text,
not by number.

**Marks.** [P] printed in the PDF. [F] read off a plot in the PDF (no number
printed: approximate). [I] my inference. [D] my own computation from printed
numbers. [S] taken from a secondary source, which is named.

**How the PDFs were read.** Text layer extracted with `pdftotext` and read;
the pages with the plots and tables that carry the numbers quoted below were
also looked at as images (HOT pp. 3:40-3:43; Hyperion manuscript pp. 11-15;
SIndex pp. 21-22; LITS pp. 29-31). Read in full: SIndex, the HOT sections
1-3.1, 4 (statements), 5 (introduction), 7, 8 and reference list; Hyperion
sections 1-3.1, 4, 5; LITS sections 1-3.1, 3.6, 5.1-5.6, 6 and reference
list; PIM-trie sections 1, 3, 4.4.2, 5 (beginning), 6 and every occurrence of
"fast trie"; burst tries sections 1, 3 (definitions), 4, 5 (reference
structures and summary), 6 (end), 7; HAT-trie sections 1-5 (text) and
conclusions. Not read: the proofs of HOT section 4, the node layouts of HOT
section 5 beyond the introduction, LITS sections 3.2-3.5 and 4, the PIM-trie
proofs, the parameter studies of the burst-trie paper (Tables VI-VII were
not read: only the text that summarises them).

**Which versions these are.**

| PDF | What it is |
|---|---|
| `HOT.pdf` | The journal version, as expected: ACM TODS 47(1), **Article 3**, April 2022, 46 pages, DOI 10.1145/3506692 [P] |
| `Hyperion.pdf` | The **accepted manuscript** (Loughborough repository cover page, 16 manuscript pages numbered 1-16) of the SIGMOD 2019 paper; DOI 10.1145/3299869.3319870 is printed on the cover; the proceedings page numbers are not [P] |
| `SIndex.pdf` | Publisher's version, APSys '20, pp. 17-24, DOI 10.1145/3409963.3410496 [P] |
| `LearnedIndices.pdf` | The **"Just Accepted"** version (watermark on every page) of the ACM TODS paper: no volume, issue or article number; footer "ACM 1557-4644/2026/8-ART", DOI 10.1145/3841175; "Received 24 October 2025; revised 20 April 2026; accepted 26 July 2026" [P] |
| `PIMTrie.pdf` | SPAA '23, 14 pages numbered 1-14, DOI 10.1145/3558481.3591070 [P] |
| `BurstTries.pdf` | ACM TOIS 20(2), April 2002, pp. 192-223 [P] |
| `Hat-trie.pdf` | The **conference paper** (ACSC 2007, CRPIT vol. 62, pp. 97-105, page numbers printed), not the VLDB Journal paper of 2010 whose DOI is listed in `TO-DOWNLOAD.md` [P] |

## Summary

1. **Nothing in `dynzfast.tex` about these papers is wrong.** Three sentences
   concern them (lines 561-562, 567, 581): the first two are correct; the
   third (PIM-trie) is correct but can be made precise, and it sits in the
   wrong paragraph.
2. **None of HOT, Hyperion, SIndex, burst tries, HAT-trie cites x-fast,
   y-fast or z-fast tries, Willard, van Emde Boas or Waldvogel et al.** [P:
   text search of each PDF and reading of the reference lists of HOT, SIndex,
   LITS]. Wormhole is cited only by SIndex (as a competitor). PIM-trie cites
   x-fast/y-fast tries (Willard) and two z-fast papers; it does not cite
   Wormhole or Waldvogel et al.
3. **The citation of *Dynamic Z-Fast Tries* in the LITS journal paper is
   indirect:** it occurs once, in the description of c-trie++, a baseline
   that is new in the journal version (section 6 below).
4. **PIM-trie uses small z-fast tries as a black box** (strings shorter than
   w bits, lookup in O(log w) with high probability), cites the SPIRE 2010
   paper for the structure and the SODA 2009 paper for fat binary search, and
   reports no implementation (section 5 below).
5. **One statement of `practical-indexes.md` is contradicted:** "nobody else
   reports a billion strings" (section 7.1 there). Hyperion reports 7.95
   billion string keys [P]. Other corrections to the surveys are in the
   synthesis.
6. Six BibTeX entries are ready for `biblio.bib` (end of this file); the
   entry `Binna2022Height` of `practical-indexes.bib` lacks the article
   number (3).

## 1. Binna, Zangerle, Pichl, Specht, Leis, "Height Optimized Tries" (`HOT.pdf`)

**Version.** ACM Trans. Database Syst. 47, 1, Article 3 (April 2022), 46
pages, DOI 10.1145/3506692; "Received March 2021; revised November 2021;
accepted December 2021" [P]. `biblio.bib` has only the SIGMOD 2018 paper
(`Binna2018HOT`), whose data agree with the way the journal paper cites it
(reference [6]) and with SIndex's reference [3] (pp. 521-534, DOI
10.1145/3183713.3196896) [P]. `practical-indexes.bib` has `Binna2022Height`
with `pages = {1--46}` and the comment "article number not checked": the
article number is 3.

**What the journal version adds** (p. 3:2) [P]: "a detailed explanation of
the deletion operation"; the proof of "three theoretical properties of HOT:
(I) minimal height, (II) deterministic structure, and (III) recursive
design"; "the new family of hierarchical node layouts" (section 5.2), which
trade "access performance for memory efficiency"; a discussion of the
synchronization protocol, an upper bound on space (section 3.3) and a
discussion of disk-optimized layouts (section 5.3). The additional material
"was previously submitted as part of a PhD thesis".

**Mechanism** [P]. "HOT combines multiple nodes of a binary Patricia trie
into compound nodes having a maximum node fanout of a predefined value k such
that the height of the resulting structure is optimized" (p. 3:4); "Instead
of having a fixed span and data-dependent fanout as in a conventional trie,
HOT features a data-dependent span and a fixed maximum fanout k" (p. 3:5).
Property (II), section 4: "For a given set of keys and a maximum fanout k, no
set of compound nodes can be found such that that the height of the resulting
tree is lower". The fast layout (ALL, "adaptive linearized node layout") has
maximum fanout 32, chosen because "a single AVX2 instruction can process"
that many partial keys, and extracts key bits with PEXT; the compact layout
(LOL, "leaf optimized") has maximum fanout 256. Leaves hold 64-bit tuple
identifiers and the lookup ends with a comparison with the stored key
("Validate the candidate to prevent false positives", Listing in section
5.1; footnote 1: "keys are not necessarily stored fully in the trie and every
key must therefore be available at its corresponding leaf node").

**Citations.** 42 references; no x-fast, y-fast or z-fast tries, no
Wormhole, no Waldvogel et al., no hashing-based index [P]. Burst tries and
HAT-tries are mentioned as hybrids that "use binary trees and hash tables,
respectively, for sparsely populated areas" (p. 3:4). Hash tables are
excluded from the evaluation "as these do not support range scans and
related operations" (p. 3:38).

**Experiments** (section 7) [P unless marked].

- Hardware: "Intel i7-6700 CPU, which has 4 cores and is running at 3.4 GHz
  ... (32 KB L1, 256 KB L2, and 8 MB L3 cache)"; scalability on an i9-7900X
  (10 cores); Linux, GCC 7.2.
- Competitors: ART, Masstree, STX B+-tree ("default node size is 256
  bytes"). All structures store "64-bit pointers as tuple identifiers"; the
  memory of the tuples (keys included) is not counted for any structure.
- Datasets: `url`, "72,701,109 distinct URLs, which we collected from the
  2016-10 DBPedia data set", without those "longer than 255 characters"
  (average 55 bytes, captions of Figures 22-24); `email`, described in
  section 7.1 as "30 byte long email addresses originating from a real-world
  email data set" and in the captions as "email: 23 bytes" (the source is
  not named); `yago` (63-bit triples); `integer` (uniform 63-bit). 50 million
  entries of each are inserted, then "100 million randomly distributed
  lookup operations".
- **The evaluation of the journal version is not that of the SIGMOD paper.**
  It consists of a read-only micro-benchmark of the node layouts (section
  7.2, Figures 22-24) and of a scalability experiment (section 7.3, Figure
  25). The strings "YCSB" and "workload C/E" do not occur in the PDF, and no
  scan is measured, although the Summary says that HOT "generally
  outperforms them in terms of lookup and scan performance".
- Figure 22 (numbers printed on the bars), 50 million keys:

  | | ALL (HOT) | LOL (HOT) | ART | Masstree | STX B+-tree |
  |---|---|---|---|---|---|
  | `url`, million lookups/s | 1.66 | 0.70 | 0.65 | 1.13 | 0.39 |
  | `email`, million lookups/s | 2.15 | 1.20 | 1.02 | 1.55 | 0.38 |
  | `yago`, million lookups/s | 4.88 | 2.08 | 2.42 | 1.69 | 1.77 |
  | random integers, million lookups/s | 5.16 | 2.80 | 5.36 | 1.89 | 1.72 |
  | `url`, tree height | 9 | 5 | 62 | 44 | 7 |
  | `email`, tree height | 7 | 4 | 25 | 15 | 7 |
  | `url`, bytes per entry | 14.4 | 9.7 | 35.2 | 104.5 | 27.0 |
  | `email`, bytes per entry | 13.5 | 9.7 | 32.1 | 95.7 | 27.0 |
  | `yago`, bytes per entry | 12.6 | 9.5 | 29.9 | 33.1 | 27.0 |
  | random integers, bytes per entry | 11.5 | 9.2 | 23.5 | 33.1 | 27.0 |

  The bytes per entry include the 8-byte tuple identifier and exclude the
  keys: the text gives 6.45 bytes per entry of "structural information" for
  ALL on `url` and "LOL never requires more than 2 bytes per entry to store
  its structural information" (p. 3:41). "The HOT variations are the only
  index structures that for both textual datasets require less space than the
  actual raw keys (email: 43%, url: 74%)". On lookups ALL is 4% slower than
  ART on random integers and "between 93% and 161%" faster on the other
  sets.
- The same figure prints instructions, L1 misses, LLC misses and branch
  misses "per lookup operation"; the printed LLC misses (54 for ALL on `url`)
  exceed the printed L1 misses (36), so the unit of these rows is unclear
  [I]: do not quote them as cache misses per lookup.
- Scalability (Figure 25, `url`, 50M inserts then 100M lookups, median of
  seven runs): about 17 million lookups/s and 10 million inserts/s for HOT
  with 10 threads [F]; mean speedups over all data sets 9.96 (HOT), 9.91
  (ART), 10.1 (Masstree) for lookups and 9.00, 9.51, 7.87 for inserts [P].
- Inserts, deletions and scans on strings: no single-thread numbers in this
  version [P by absence].

**Sentences of `dynzfast.tex`.** Lines 560-562:

> Tries with adaptive node layouts, such as ART~\cite{Leis2013Adaptive},
> Masstree~\cite{Mao2012Cache} (a trie whose nodes are B$^+$-trees indexing eight bytes of the keys) and HOT~\cite{Binna2018HOT} (a binary Patricia trie whose nodes are
> grouped into compound nodes of bounded fanout so as to minimize the height), perform a number of steps proportional to the depth of the trie.

Verdict: **correct**. The description of HOT is the one printed at p. 3:4,
and "so as to minimize the height" is exactly what the journal version
proves, so it should be cited too. Only remark: for HOT the "depth of the
trie" is the height of the tree of compound nodes, not the depth of the
binary trie; "height of the resulting tree" is unambiguous. Proposed text in
"Proposed edits" (E1).

**Addition to the Related work.** One optional clause with the heights
measured in Figure 22 (E1); nothing else is justified.

## 2. Masker, Suss, Nagel, Zeng, Brinkmann, "Hyperion" (`Hyperion.pdf`)

**Version.** Accepted manuscript of "Hyperion: Building the Largest
In-memory Search Tree", SIGMOD '19, DOI 10.1145/3299869.3319870 [P, cover
page]. The pages 1207-1222 of the entry `Masker2019Hyperion` in
`practical-indexes.bib` are not printed in this PDF; they come from Crossref
[S: the comment in that file] and agree with the length of the manuscript
(16 pages) [I]. The authors' names as printed: Markus M\"asker, Tim S\"u\ss,
Lars Nagel, Lingfang Zeng, Andr\'e Brinkmann: the entry is correct. Not in
`biblio.bib`, not cited in `dynzfast.tex`.

**Mechanism** (section 3) [P]. "Hyperion is therefore an m-ary trie with m =
65,536"; "Each node of the trie encodes up to 16 bits of the overall key,
leading to fewer, but bigger nodes"; a node is a *container*, a byte array
that stores an internal two-level trie on the two bytes in pre-order
("T-Nodes" and "S-Nodes") with delta-encoded partial keys, jump tables and
recursively embedded child containers; "Lookups perform a pre-order traversal
of the container's data structure. The linear scanning procedure is slower
than SIMD operations or direct offset calculations but also not constrained
to static offsets and fixed sizes". Containers grow "in small 32 byte
increments" and are managed by a custom allocator with 5-byte "Hyperion
Pointers". Abstract: "Hyperion does not depend on CPU vector units, but
scans the data structure linearly". The descent is one container per two
bytes of key: no hashing, no search on lengths [P]. It is a key-value store
(keys and values are inside the structure), unlike ART and HOT, which point
to records.

**Citations.** No x-fast, y-fast, z-fast tries, no Wormhole, no Waldvogel et
al. [P: text search]. Related structures discussed: Judy, burst trie,
HAT-trie ("Hash tables allow fast access rates, but complicate range
queries"), ART, HOT, C-trie, generalized prefix tree, KISS-tree.

**Experiments** (section 4) [P].

- Hardware: "two Intel Xeon E5-2630 v4 processors running at 2.2 GHz ...
  25 MiB L3 cache and 1 TiB of main memory, out of which 978 GiB are
  usable"; CentOS 7; "GCC-7.2.0 -O3"; "memory consumption was parsed from
  /proc/self/status" (so it includes keys and values).
- Competitors: Judy (JudySL for strings), HAT-trie (D. C. Jones's
  implementation), ART (the authors' code) and ART_C (libart), HOT, STL `map`
  (red-black tree) and `unordered_map`; "Neither the source code nor the
  individual configuration parameters of these data structures were
  modified". For ART and HOT "an array of all k/v pairs" was added.
- String dataset: "the Google Books n-gram corpus"; "the English 2-grams a-d
  consisting of 7.948 billion entries. The keys and values have a total size
  of 167.7 GiB and 59.1 GiB, respectively. The average key size is 22.65
  bytes, and keys larger than 255 were skipped, as larger keys are not
  supported by HOT"; inserted once in lexicographic order and once in random
  order. Lookups "were performed for all values in the same order as they
  were inserted". Integer datasets: 16 billion sequential and 13 billion
  random 64-bit key-value pairs.
- Table 1, 7.95 billion string keys (puts and gets in million operations per
  second, memory in GiB and bytes per key including keys and values):

  | | seq. puts | seq. gets | seq. GiB (B/key) | rand. puts | rand. gets | rand. GiB (B/key) |
  |---|---|---|---|---|---|---|
  | Hyperion | 0.83 | 1.10 | 85 (11.4) | 0.26 | 0.32 | 98 (13.2) |
  | JudySL | 2.06 | 2.77 | 202 (27.3) | 0.32 | 0.33 | 205 (27.7) |
  | HAT | 0.83 | 1.86 | 281 (37.9) | 0.33 | 0.45 | 281 (38.0) |
  | ART_C | 2.36 | 2.70 | 622 (84.1) | 0.30 | 0.32 | 622 (84.1) |
  | ART | 2.37 | 2.76 | 435 (58.8) | 0.37 | 0.37 | 444 (59.9) |
  | HOT | 1.75 | 2.46 | 333 (45.0) | 0.45 | 0.53 | 339 (45.7) |
  | RB-tree | 1.29 | 1.44 | 944 (127.5) | 0.12 | 0.15 | 944 (127.5) |
  | Hash | 0.92 | 0.85 | 954 (128.8) | 0.66 | 0.69 | 954 (128.8) |

  "A memory consumption of 11.27 bytes per key means that Hyperion compresses
  the average key size of 22.65 bytes to only 3.27 bytes, as 8 bytes are used
  for the value". In random order HOT is the fastest ordered structure (0.53
  million gets per second, i.e., 1.9 microseconds per get [D]); the red-black
  tree takes 6.7 microseconds [D].
- Table 3, one range query "covering the full index", duration in seconds,
  strings (sequential / random insertion order): Hyperion 184 / 205, Judy
  1,661 / 2,257, HAT 3,303 / 4,103, ART_C 245 / 1,472, HOT 215 / (no value),
  RB-tree 650 / 3,752. Per key [D]: Hyperion 23-26 ns, HOT 27 ns, RB-tree
  82-472 ns. "The hash table and ART did not offer an ordered iterator".
- Deletions were not measured ("the cost of updates is equivalent to lookups,
  and a key deletion results in case of Hyperion in a memmove").
- Headline claims: "the memory footprint is reduced by up to a factor of 2.1
  times and Hyperion processes range queries up to 40% faster than the best
  alternative" (conclusion); "previous results on ranking data structures do
  not hold at this scale"; "Extrapolating average performance to larger
  indices can therefore result in false assumptions".
- No code URL is printed [P], as `practical-indexes.md` says.

**Sentences of `dynzfast.tex`.** None.

**Addition to the Related work.** Marginal as a data structure (a top-down
trie that trades speed for space). It matters for the experiments: it is the
only paper of this batch, and of `practical-indexes.md`, that measures
dynamic ordered indexes on billions of string keys, which is the scale of
Table `tab:urlsb`. If the
authors want to say so, one sentence (E4, optional).

## 3. Wang, Tang, Wang, Chen, "SIndex" (`SIndex.pdf`)

**Version.** "SIndex: A Scalable Learned Index for String Keys", 11th ACM
SIGOPS Asia-Pacific Workshop on Systems (APSys '20), August 24-25, 2020,
Tsukuba, Japan, pp. 17-24, DOI 10.1145/3409963.3410496 [P]. The entry
`Wang2020SIndex` of `practical-indexes.bib` is correct. Not in `biblio.bib`,
not cited in `dynzfast.tex`.

**Mechanism** (section 3) [P]. A two-layer structure derived from XIndex: a
root with a piecewise linear model indexing the *pivots* of the groups, and
"group nodes", each with a sorted array indexed by a linear model and a
"delta index" for insertions, merged by background compaction. "SIndex
greedily clusters keys with common prefixes into groups"; in each group the
model and the comparisons use the *partial key* `k[pl:pl+el]`, where `pl` is
the length of the common prefix of the group and `el` the shortest length
that keeps the keys distinct. The model predicts a position and "a binary
search" within the error bounds follows. Keys are vectors of fixed length:
"We tokenize URLs into 128-length feature vectors. For URLs with length n <
128, we set x_i = 0".

**Citations.** 19 references; Wormhole is [19], a competitor, described as
"an index structure that hybrids B-tree, trie, and hash table. It replaces
the internal nodes of B-tree with a trie, which is then encoded with a hash
table for efficient access" (section 5). No x-fast, y-fast, z-fast tries, no
Waldvogel et al., no mention of a search on prefix lengths [P].

**Experiments** (section 4) [P unless marked].

- Hardware: "a server with two 12-core Intel Xeon E5-2650 v4 CPU, each with
  30 MB LLC".
- Datasets: random strings "R[key length]-[dataset size]" (R64-100M,
  R64-200M, R128-100M); "a real-world dataset containing 92M URLs of quotes
  from Memetracker [14]. The maximum length of URLs is 128 bytes and the
  average length is 62 bytes".
- Competitors: XIndex, Masstree, Wormhole, with "default configurations".
- Read-only, one thread (Figure 3): "SIndex outperforms XIndex, Masstree and
  Wormhole by up to 88%, 91% and 43% respectively under random datasets.
  Under the URL dataset, SIndex still shows relatively good performance
  compared with XIndex (1.5x) and Masstree (1.3x). However, SIndex's
  performance is worse than Wormhole by 25%." Bars on URL [F]: Wormhole about
  0.66, SIndex about 0.5, Masstree about 0.4, XIndex about 0.33 million
  operations per second; on R64-100M: SIndex about 1.2, Wormhole about 0.85.
- Read-write (R128-100M only; writes are updates, inserts and deletes 2:1:1):
  with 40% writes "comparable performance as Wormhole"; with 24 threads and
  10% writes "worse than Wormhole and Masstree ... (19% and 13% worse)".
- No scan, no successor query, no lookup of absent keys, no memory
  measurement [P by absence].
- Code: `https://ipads.se.sjtu.edu.cn:1312/opensource/xindex/-/tree/sindex`
  [P].

**Sentences of `dynzfast.tex`.** Not cited. Line 567, "learned indices for
strings have been proposed, too~\cite{Spector2021Bounding,Yang2024LITS}", is
**correct**; SIndex is the earliest of them and can be added to the list
(E2).

**Addition to the Related work.** Marginal: only the citation key. Its use
for this paper is as an independent measurement of Wormhole on URLs (see the
synthesis).

## 4. Yang, Chen, "Learned Indices for String Keys with Optimized Performance and O(N) Space Cost" (`LearnedIndices.pdf`)

**Version.** ACM TODS, "Just Accepted", DOI 10.1145/3841175, accepted 26 July
2026, footer "2026/8" [P]. The entries `Yang2026Learned` of the survey
`.bib` files are consistent; no volume, issue or article number can be added
from this PDF. `biblio.bib` has `Yang2024LITS` (PVLDB 17(11):3415-3427, DOI
10.14778/3681954.3682010), which agrees with reference [44] of the journal
paper [P].

**Extensions of the journal version** (p. 3) [P]: "a more complete discussion
of the index space overhead"; "two novel space optimization techniques,
namely Dynamic Scale Factor and the cell-based root"; "new multi-threaded
designs, including root-slot locking and a non-blocking index rebuild
mechanism"; "an extended experimental evaluation". The baselines c-trie++
and DynPDT are not in the PVLDB text extract of the earlier review
(`review-2026-11/literature-extracts/lits.txt`, text search), so they are new
in the journal version [I].

**The citation of *Dynamic Z-Fast Tries*.** Reference [12] is "Djamal
Belazzougui, Paolo Boldi, and Sebastiano Vigna. 2010. Dynamic Z-Fast Tries.
In ... SPIRE 2010 ... 159-172". It is cited **once**, in section 2.2
("Existing Indices Optimized for Strings"), in the item describing c-trie++
(pp. 5-6):

> "c-trie++ [38] is a dynamic compact trie tailored for fast prefix search.
> Starting from a vanilla compact trie, c-trie++ creates a macro trie and
> potentially a micro trie per macro node. [...] A micro trie stores the
> string suffixes of the descendants as an alphabet-aware z-fast trie [12],
> which supports binary search of string prefixes. With a span of [alpha]
> bytes, the macro trie tends to be shallower than ART. However, its height
> can be quite large for long keys."

So the z-fast trie is neither a baseline nor an ingredient of LITS: it is
cited as the component of a baseline. "z-fast" occurs nowhere else; x-fast
and y-fast tries, Wormhole, Masstree and Waldvogel et al. are not cited [P:
text search and reference list of 47 entries]. B+-trees are excluded
(footnote 1): "previous work has shown that trie-based indices significantly
outperform B+-Trees for string data sets [13]" ([13] is the SIGMOD HOT
paper).

**Mechanism** (sections 1 and 3.1, 3.6) [P]. A collision-driven learned index
in the style of LIPP ("creating a child node to store the keys that are
mapped to the same slot", which avoids the "last-mile search"); the model
combines "a global Hash-enhanced Prefix Table (HPT) and a per-node local
linear model", where "the HPT approximates the conditional probability of the
next character given a prefix in the string key" (2 MB: 1024 rows, 128
columns); compact leaf nodes of up to 64 entries; HOT subtries where a
performance model predicts that a trie is better (LIT is the structure
without subtries, LITS the hybrid). Height O(log N), point operations O(log
N), space O(N) with the dynamic scale factor (section 3.6). The only hashing
is that of prefixes to select a row of the HPT: it predicts positions, it
does not locate nodes.

**Experiments** (section 5) [P unless marked].

- Hardware: "two 3.4GHz Intel Xeon Platinum 8380 CPUs (with 40 cores / 80
  threads per CPU and 60MB L3 cache) and 256GB memory", Ubuntu 20.04, "GCC
  9.4.0 using the O3 optimization level", single thread except in sections
  5.7-5.8. Two more platforms in section 5.6: an Intel Core i7-9700 and "a
  2.7 GHz AMD EPYC 9334 processor ... 32 MB L3 cache".
- Baselines: ART (`armon/libart`, with a scan procedure added), HOT (the
  authors' code), SIndex ("requires all strings to be padded to a uniform
  length"), RSS (reimplemented, static), SLIPP (search only), c-trie++ (the
  authors' code), DynPDT (`poplar-trie`, variant `plain_bonsai_map`).
- Datasets (Table 1; "pre-processed to remove duplicate strings, strings
  containing non-ASCII characters, and strings longer than 255 characters"):

  | Data set | Avg length | Keys | Source |
  |---|---|---|---|
  | `addr` | 24 B | 34M | OpenAddresses, US West |
  | `dblp` | 76 B | 7M | paper titles from `dblp.xml.gz` |
  | `geoname` | 13 B | 7M | GeoNames (Kaggle) |
  | `imdb` | 13 B | 9M | `name.basics.tsv.gz` |
  | `reddit` | 11 B | 26M | Reddit user names (Kaggle) |
  | `url` | 64 B | 63M (4.6 GB) | CommonCrawl `CC-MAIN-2023-14` |
  | `wiki` | 15 B | 43M | English Wikipedia titles |
  | `email`, `idcard`, `phone`, `rands` | 23, 18, 17, 32 B | 45M, 63M, 50M, 50M | synthetic |

- Workloads: the six YCSB core workloads, with 80% of the keys bulk-loaded
  (100% for C) and 20 million operations; "Search keys are randomly selected
  from the bulkload keys"; "Scan range is fixed as 50", and a scan is "a
  positive point search followed by a series of getNext calls"; plus
  insert-only (10% bulk-loaded) and scan-insert-read. No lookup of absent
  keys, no lower bound of absent keys, no deletion workload in the part read.
- Results: "Under the read-only workload, LITS outperforms HOT and ART by up
  to 1.78x and 1.59x, respectively. Under the write-only workload, the
  performance gains further increase to up to 2.19x over HOT and 1.85x over
  ART. Although LIT does not outperform HOT on skewed data sets such as url
  and dblp, LITS effectively narrows the performance gap through its hybrid
  index structure. For scan-related workloads, including YCSB-E and the
  scan-insert-read workload, HOT achieves the best overall performance."
- On `url` (Figure 13, bars) [F]: lookups about 1.0 (LIT), 1.4 (LITS), 1.4
  (HOT), 1.1 (ART), 0.45 (SIndex), 0.4 (RSS) million operations per second;
  inserts about 0.8 for LIT, LITS and HOT, 0.6 for ART. YCSB-E on `url`:
  about 0.4-0.55, HOT best [F].
- Table 4, average leaf depth after bulk-loading:

  | Data set | LIT | HOT | ART | SIndex | RSS | SLIPP | c-trie++ | DynPDT |
  |---|---|---|---|---|---|---|---|---|
  | `addr` | 2.9 | 7.0 | 10.2 | 2.0 | 2.0 | 4.9 | 2.9 | 6.7 |
  | `dblp` | 4.6 | 6.8 | 14.1 | 2.0 | 2.2 | 7.3 | 3.8 | 6.2 |
  | `url` | 6.3 | 7.8 | 16.1 | 2.0 | 3.7 | 9.1 | 6.4 | 7.6 |
  | `wiki` | 3.9 | 7.8 | 11.6 | 2.0 | 2.1 | 5.7 | 3.1 | 6.9 |

- **c-trie++ and DynPDT** (Figure 18 and text, p. 31): "LIT achieves an
  average speedup of 1.58x over c-trie++ and 1.87x over DynPDT. LITS further
  improves this to 1.66x over c-trie++ and 1.93x over DynPDT. [...] during
  lookup, each node access in c-trie++ incurs 1.64 to 3.07 L3 cache misses on
  average, while DynPDT incurs only 0.60 to 1.27 misses per node access. A
  possible reason is their different hashing schemes. In c-trie++, locating a
  child node may require probing up to three candidate positions in its
  node-local DicChunk hash map due to cuckoo hashing. [...] insertions into
  micro-tries account for 25.2-59.2% of the total insertion time in
  c-trie++." On `url` [F]: lookups about 0.5 (c-trie++) and 0.9 (DynPDT)
  million per second; inserts about 0.3 and 0.6.
- Space: index size "excludes key-value payload but includes tuple
  identifiers". Table 5, LIT-D after bulk-loading all keys: 1,194 MB (`addr`),
  227 MB (`dblp`), 1,934 MB (`url`), 1,753 MB (`wiki`), i.e., about 35, 32,
  31 and 41 bytes per key [D]. Table 6: SIndex 17.03 GiB on `url` against
  5.07 GiB of raw data (padding is "at least 91% of the entire index size");
  RSS 0.06 GiB; SLIPP 103.09 GiB.
- Section 5.6: on the other two platforms "HOT shows clear advantage for the
  read-only workloads on skewed data sets (e.g., dblp and url)".
- Abstract: "LITS achieves up to 2.19x and 1.85x improvement over HOT and ART
  for point operations with comparable scan performance and space cost".
- Code: `https://github.com/schencoding/lits.git`; pre-processed datasets on
  a Baidu share [P].

**Sentences of `dynzfast.tex`.** Line 567:

> learned indices for strings have been proposed, too~\cite{Spector2021Bounding,Yang2024LITS}.

Verdict: **correct**. Lines 611-612 ("The measurements of a z-fast trie
reported in the literature we are aware of (e.g.,~\cite{TKKctriepp,Kanda2020Dynamic,Zhang2026C2})
use a C++ port of our Java implementation") are not affected: the LITS
journal paper measures c-trie++ and DynPDT, not a z-fast trie.

**Addition to the Related work.** Add the journal version to the citation at
line 567 (E2). Optionally one clause saying that c-trie++ is a baseline of
that evaluation (E5); more is not justified.

## 5. Kang, Zhao, Blelloch, Dhulipala, Gu, McGuffey, Gibbons, "PIM-trie" (`PIMTrie.pdf`)

**Version.** "PIM-trie: A Skew-resistant Trie for Processing-in-Memory",
SPAA '23, June 17-19, 2023, Orlando, FL, USA, "14 pages", DOI
10.1145/3558481.3591070 [P]. The entry `Kang2023PIM` of `biblio.bib` is
correct.

**What it is** [P]. A theory paper in the Processing-in-Memory model (a CPU
with a small cache and P memory modules with processors): "the first
batch-parallel radix-based index for PIM systems that provides load balance
and low communication under adversary-controlled workloads". It supports
"LongestCommonPrefix, Insert, and Delete in O(log P) communication rounds per
batch and O(l/w) communication volume per string" on variable-length bit
strings. No implementation: "Future work includes ... implementing PIM-trie
on real PIM systems" (section 6).

**How it refers to fast tries** [P].

- Section 1 (p. 2): "The family of fast tries (x-fast tries [62], y-fast
  tries [62], z-fast tries [8]) are explicitly designed to address challenge
  C2, but x-fast tries and y-fast tries support only fixed-length keys, and
  how to address challenge C1 for z-fast tries is still an open problem."
  (C2: "how to avoid serial bottlenecks when dealing with long strings"; C1:
  how to map nodes to modules with load balance and low communication.) And:
  "It builds upon the idea in z-fast tries of combining both a radix tree and
  a hash table, addressing challenge C1 and other challenges."
- Section 3, paragraph "Fast Trie" (p. 3): "Fast trie is another family of
  tries that leverage hashing to achieve logarithmic query costs relative to
  key length." After x-fast and y-fast tries: "The dynamic z-fast trie [8]
  was proposed to support arbitrary-length key strings. For machine word size
  w, it can handle strings of length up to 2^w, and queries/updates of key x
  are supported in O(|x|/w + log |x|) time and O(|x|/B + log |x|) I/Os (in
  the cache-oblivious model [21]). The key mechanism is fat binary search
  [7]." [8] is the SPIRE 2010 paper; [7] is the SODA 2009 paper on monotone
  minimal perfect hashing. (The bounds quoted omit the terms in the lengths
  of the predecessor and successor [I].)
- Section 3 (p. 4): "The z-fast trie algorithm is a serial algorithm with
  good bounds in both work and space by combining a radix tree and a hash
  table. We are motivated by this combination, and realize that a hash table
  can act like a load distributor that parallelizes the workload to utilize
  PIM modules. This new use does not exist in serial z-fast tries."
- Table 1 compares with a "Distributed x-fast trie" (fixed-size strings
  only).

**How z-fast tries are used** (section 4.4.2, "Optimizations to Reduce
Work", pp. 9-11) [P]. *Pivot nodes* are the positions of the compacted trie
whose depth is a multiple of w bits; their hashes are stored in a hash table.
Z-fast tries appear in two places, both as a way to go from a pivot to the
nodes of the compacted trie in the following w bits in O(log w):

1. On the CPU side, in the *query trie* built for each batch: "(1) a hash
   table is built to map pivot hashes to pivot nodes, (2) a z-fast trie of
   height w is built for every pivot node as shortcuts to all its hosting
   compressed nodes. (Z-fast tries are compressed binary radix trees
   supporting lookup with cost O(log h) whp for height h and linear
   additional space.)" A block root is then located by a lookup of the hash
   of its longest prefix of length multiple of w in the hash table, "then
   its S_rem in the relative z-fast trie to find the exact match position,
   each taking O(log w) work whp".
2. On the PIM side, in each *data block* ("Efficient Local Matching"): "we
   pick pivot nodes in the data block (with length kw, k in Z), and build a
   z-fast trie of height w bits on every pivot as a shortcut to compressed
   nodes hosted by this pivot and pivots of the next level"; in the proof of
   Lemma 4.11, "the number of z-fast trie lookups is the total number of
   pivot nodes and compressed nodes in the query block".

In the same section the meta-blocks use "a two-layered index" whose second
layer "combines a y-fast trie and a table" (strings shorter than w bits
padded to w-bit integers, with validity vectors).

**Static or dynamic?** The paper does not say [P: "z-fast" occurs only in the
passages quoted above]. The tries of use 1 are built once per batch and then
only queried [I]. Those of use 2 live in data blocks that Insert and Delete
modify, so they must be updated or rebuilt, but section 5.2 does not discuss
it [P by absence]; the only reference given for the structure is the dynamic
paper [8]. What is taken is therefore a black box: a compacted binary trie on
strings shorter than w bits with linear space and lookups in time logarithmic
in the height, plus the general idea of pairing a trie with a hash table.

**Other citations.** HOT ([10, 11], both versions): "there is no nontrivial
bound on the height of a compound tree". No Wormhole, no Waldvogel et al.
[P: text search].

**Sentence of `dynzfast.tex`.** Line 581:

> The z-fast trie has also been used as a building block of a trie for the processing-in-memory model~\cite{Kang2023PIM}.

Verdict: **correct, but imprecise and misplaced**. It does not say what is
used (small z-fast tries on strings shorter than a word, as shortcuts from
the nodes at depths multiple of w), and it closes the paragraph on Wormhole,
in the part on engineered main-memory indexes, whereas PIM-trie is a
theoretical structure with no implementation: it belongs to the list of
"Uses of z-fast tries" (lines 583-601). Replacement in "Proposed edits"
(E3).

Corrections to the surveys: `static-and-uses.md` (line 632) quotes "how to
address [skew] for z-fast tries is still an open question"; the PDF has "open
problem", and challenge C1 is load balance with low communication, of which
skew is the cause. The quotations in `practical-indexes.md` (lines 495-502)
are exact.

## 6. Heinz, Zobel, Williams, "Burst Tries" (`BurstTries.pdf`)

**Version.** "Burst Tries: A Fast, Efficient Data Structure for String Keys",
ACM Transactions on Information Systems, Vol. 20, No. 2, April 2002, Pages
192-223; "Received April 2001; revised September 2001; accepted November
2001" [P]. The entry `Heinz2002Burst` of `practical-indexes.bib` is correct;
the DOI 10.1145/506309.506312 is not printed in this PDF but is printed in
the reference lists of HOT ([16]) and Hyperion ([27]) [P]. Not in
`biblio.bib`, not cited in `dynzfast.tex`.

**Mechanism** (sections 1 and 3) [P]. "This structure is a collection of
small data structures, which we call containers, that are accessed via a
conventional trie, which in this context we call an access trie. Searching
involves using the first few characters of a query string to identify a
particular container, then using the remainder of the query string to find a
record in the container." "When a container is deemed to be inefficient, it
is burst, that is, replaced by a trie node and a set of child containers
that, between them, partition the original container's strings." Access-trie
nodes are arrays of |A| pointers; containers are linked lists (possibly
move-to-front), binary search trees or splay trees; three heuristics decide
when to burst (Ratio, Limit, Trend). The descent is one character per level.

**Order.** The abstract says that a burst trie "maintains the strings in
sorted or near-sorted order". Precisely (section 4, "Methodology"): "The data
structures tested keep the terms in alphabetical order, with the exception of
hash tables and of burst tries with lists representing containers. To have a
fair comparison of all data structures we use an in-memory quicksort at the
end of each run with hash tables and with burst tries with list containers,
and include the time of sorting in the reported times." So containers are
ordered among themselves, and inside only if they are trees [P].

**Citations.** No x-fast or y-fast tries, no van Emde Boas, no hashing of
prefixes [P: text search]. The related work (section 6) discusses Patricia
tries, LC-tries, ternary search trees, and earlier proposals to keep suffixes
in containers (Sussenguth 1963; Ramesh et al. 1989; Szpankowski's b-tries).

**Experiments** (sections 4-5) [P]. The task is *vocabulary accumulation*:
every word occurrence of a text is searched and inserted if absent, so the
workload is a skewed mix of lookups and insertions; there are no ordered
queries, no deletions.

- Hardware: "a Pentium III 750 MHz with 512 Mb of memory for the text
  collections and a Pentium III 500 MHz with 256 Mb".
- Datasets (Table I): TREC web collections from Web S (100 MB, 114,109
  distinct words, 6 million occurrences) to Web XXL (45,229 MB, 8,634,056
  distinct words, 2,704 million occurrences) and TREC1 (503,242 distinct
  words, 179 million occurrences); Table II: genomic 9-grams (262,146
  distinct) and music 7-grams (3,147,961 distinct). Keys are words: average
  length is not tabulated.
- Competitors: compact trie, ternary search tree, binary search tree, splay
  tree, chained hash table with 2^20 slots (Table III: on Web XXL, BST
  3,230.3 s and 285 MB, hashing 921.4 s and 256 MB; the runs of tries and
  TSTs on the largest collections "were impractical due to space
  requirements").
- Headline claims: "a burst trie is more than twice as fast as a conventional
  tree, only 25% slower than a hash table, uses no more space than a
  conventional tree or hash table, and maintains strings in sort order"
  (section 1); "burst tries are faster than compact tries and use around
  one-sixth of the space" (section 7); "The worst case we observed was
  genomic data, where the strings are of equal length".

**Sentences of `dynzfast.tex`.** None.

**Addition to the Related work.** Marginal: no addition proposed. It is the
origin, in the experimental literature, of "containers below a trie", which
is also the organization of `ZFastSet` and of Wormhole, but the trie is
walked from the root and the paper measures no ordered query. Cite it only if
the text gets a sentence on tries with containers; in that case the fact to
state is the one above (a trie on the first characters whose leaves are
small containers that are split when they grow).

## 7. Askitis, Sinha, "HAT-trie" (`Hat-trie.pdf`)

**Version.** The conference paper: "HAT-trie: A Cache-conscious Trie-based
Data Structure for Strings"; footnote: "This paper appeared at Thirtieth
Australasian Computer Science Conference (ACSC2007), Ballarat, Australia.
Conferences in Research and Practice in Information Technology (CRPIT), Vol.
62. Gillian Dobbie, Ed."; pages numbered 97-105; "Copyright 2007, Australian
Computer Society, Inc." [P]. The entry `Askitis2007HAT` of
`practical-indexes.bib` (marked "paper not read", pages from a web search) is
confirmed; the editor and the address can be added. The dates (January 30 -
February 2, 2007) are printed in the reference lists of PIM-trie ([6]) and
are not in this PDF. The journal paper (`Askitis2010Engineering`, VLDB
Journal 19(5)) was **not** downloaded and remains unread. Not in
`biblio.bib`, not cited in `dynzfast.tex`.

**Mechanism** (sections 1 and 3) [P]. A burst trie whose containers
("buckets") are cache-conscious *array hash tables* (Askitis and Zobel 2005):
an array of slots, each pointing to a dynamic array of length-encoded strings
grown in an exact-fit manner. "The HAT-tries are built and searched in a
top-down manner"; in a bucket "the required slot is found by first hashing
what remains of the query string". Two variants: *pure* (a full bucket is
burst into at most |A| buckets under a new trie node, and the leading
character is removed) and *hybrid* (a full bucket is split in two with the
B-trie algorithm, and a bucket can have several parent pointers). Order:
"Buckets are internally unsorted, but they can be accessed in lexicographic
order"; "buckets can be rapidly sorted on demand". The paper presents the
HAT-trie as "the most efficient trie-based data structure for managing
variable-length strings in-memory while maintaining sort order", but it
measures no ordered operation.

**Citations.** No x-fast or y-fast tries, no van Emde Boas [P: text search].
Cache-oblivious structures are discussed and dismissed as untested (Brodal
and Fagerberg's static string dictionary: "the study is from a theoretical
perspective").

**Experiments** (sections 4-5) [P].

- Hardware: "a 2.8 GHz Pentium IV machine, with 1 MB of L2 cache ... 2 GB of
  RAM", Linux 2.6.17.
- Datasets (Table 1): `distinct` (28,772,169 distinct words, average length
  9.59, 304.56 MB), `trec` (612,219 distinct, 177,999,203 occurrences,
  average 5.06), `urls` (1,287,597 distinct, 9,997,487 occurrences, average
  30.93, 45.82 MB of distinct strings; "extracted from TREC web data"),
  `genome` (262,084 distinct 9-grams, 31,623,000 occurrences).
- Competitors: burst trie, array hash, chained hash table with move-to-front,
  compact binary search tree.
- Operations: construction and "self-search" (searching the same strings),
  elapsed time, memory, L2 and TLB misses (Figures 2-4, as functions of the
  bucket threshold: 1,024 to 16,384 strings).
- Numbers printed in the text for `distinct`: hybrid HAT-trie with buckets of
  1,024 strings "about 37 seconds to build and 19 seconds to search,
  consuming about 586 megabytes"; with buckets sixteen times larger, 270 MB,
  "which is less than was required by the dataset"; the burst trie needs
  "over a gigabyte of memory to reach its optimal search time of 53 seconds";
  the binary search tree "over 430 seconds to construct and search, using 764
  megabytes".
- Headline claim: "For both construction and search, the HAT-tries were up to
  80% faster than the burst-trie, while simultaneously reducing space
  consumption by as much as 70%."

**Sentences of `dynzfast.tex`.** None.

**Addition to the Related work.** Marginal: no addition proposed, for the
same reasons as burst tries. Hyperion's remark applies: hash-table buckets
"complicate range queries" (and its Table 3 shows the HAT-trie as the slowest
structure on a full ordered scan).

## Synthesis for the benchmarks

### What these papers establish as standard for string keys

- **Datasets.** URLs in every recent paper, each from a different source:
  DBpedia 2016-10 (72.7M distinct, at most 255 characters, average 55 B;
  HOT), MemeTracker (92M, at most 128 B, average 62 B; SIndex), CommonCrawl
  `CC-MAIN-2023-14` (63M, at most 255 B, average 64 B; LITS), TREC web data
  (1.3M distinct, average 31 B; HAT-trie). Then: email addresses (HOT, real,
  source not named; LITS, synthetic), Wikipedia titles, addresses, DBLP
  titles, names (LITS), Google Books n-grams (Hyperion, average 22.65 B),
  words of TREC collections and genomic n-grams (burst tries, HAT-trie),
  random strings of fixed length 64 and 128 (SIndex) or variable length
  (LITS), and 64-bit integers, random and dense (HOT, Hyperion). No paper of
  this batch uses a LAW crawl. Keys are capped at 255 bytes wherever HOT is a
  competitor ("larger keys are not supported by HOT", Hyperion; the same cap
  in HOT and LITS) and at 128 bytes in SIndex.
- **Sizes.** 50M keys (HOT), 7M-63M (LITS), 92M-200M (SIndex), 7.95 billion
  strings and 13-16 billion integers (Hyperion). The five million URLs of
  `sec:rust` are an order of magnitude below the customary size; the billion
  URLs of Table `tab:urlsb` have a counterpart only in Hyperion.
- **Competitors.** ART is measured by HOT, Hyperion and LITS; HOT by Hyperion
  and LITS; Masstree by HOT and SIndex; a B+-tree only by HOT (STX), and LITS
  explicitly drops B+-trees citing HOT; Wormhole only by SIndex; Judy,
  HAT-trie, `std::map` and `std::unordered_map` by Hyperion; c-trie++ and
  DynPDT by LITS. For a paper on string indexes submitted today, **ART and
  HOT are the two baselines that every reader of these papers expects**;
  Wormhole is, in this batch, the less common one.
- **Workloads and metrics.** Lookups of keys that are present, in random
  order (all); inserts in random order (HOT, LITS, Hyperion, which also
  inserts in sorted order); YCSB A-F with scans of 50 keys starting from a
  key that is present (LITS); one scan of the whole index (Hyperion);
  million operations per second; index bytes per key excluding the keys but
  including 8-byte tuple identifiers (HOT, LITS) or total process memory
  including keys and values (Hyperion); tree height or average leaf depth
  (HOT, LITS). **No paper of this batch measures lookups of absent keys,
  successor or predecessor queries for absent keys, or (as a separate
  workload) deletions**: the rows Contains-, Successor- and Delete of
  `sec:rust` have no published counterpart here, which confirms section 7.2
  of `practical-indexes.md`.

### What qualifies or contradicts `sec:rust`

Nothing printed in these papers contradicts a number or a statement of
`sec:rust`. The following qualify it.

1. **The B-tree baseline is weak by the standards of these papers** (lines
   2438-2439, 2451-2456). In HOT's Figure 22 the STX B+-tree, which like
   `BTreeSet` reaches the keys through pointers, makes 0.39 million lookups
   per second on 50M URLs against 1.66 for HOT, 1.13 for Masstree and 0.65
   for ART [P], and LITS omits B+-trees for this reason [P]. So "the z-fast
   trie performs like the B-tree" (line 2451) places the one-key-per-leaf
   `ZFastTrie` well below HOT in a reader's mind [I: different hardware, data
   and sizes; no measurement of a z-fast trie against HOT exists]. The
   `\todo` at lines 2406-2408 already says that two competitors are not
   enough; these papers say which ones are missing: HOT and ART.
2. **Wormhole on URLs is confirmed as the bar, by a third party** (lines
   2461-2466). SIndex, one thread, 92M MemeTracker URLs: Wormhole about 0.66
   million lookups per second [F]; SIndex is "worse than Wormhole by 25%" and
   1.3 times faster than Masstree [P], so Wormhole is about 1.7 times faster
   than Masstree [D]. No paper of this batch measures Wormhole against HOT.
3. **Memory** (lines 2447-2448, 2460, 2464, and the last row of the tables).
   `sec:rust` reports the growth of the resident set per key, keys included:
   131.1 bytes (`ZFastSet`), 168.4 (`ZFastTrie`), 134.4 (Wormhole), 122.7
   (`BTreeSet`) for keys of 78 bytes on average, i.e., an overhead of about
   53, 90, 56 and 45 bytes per key [D]. The convention of HOT and LITS is
   index size excluding keys and including an 8-byte tuple identifier: on
   URLs, HOT 14.4 (9.7 with the compact layout), ART 35.2, Masstree 104.5,
   STX B+-tree 27.0 bytes per entry (HOT, Figure 22), LIT about 31 (LITS,
   Table 5 [D]). The two conventions are not directly comparable (allocator
   overhead for the keys is in the first and not in the second), but the
   sentence "the space used is essentially the same" (line 2464) holds only
   with respect to Wormhole: against HOT the overhead is several times larger
   [I].
4. **Scans** (line 2444, row "Scan"). The same metric, a full ordered
   iteration, is in Hyperion's Table 3, on 7.95 billion keys: 23-27 ns per
   key for Hyperion and HOT, 82-472 ns for a red-black tree [D]. The 10.9 ns
   per key of `ZFastSet` and 10.5 of Wormhole on five million URLs are thus
   in the range of the best published structures, and the 122.8 ns of
   `ZFastTrie` in the range of a pointer-based tree [I: different sizes and
   hardware].
5. **Hardware.** LITS reruns its experiments on an AMD EPYC 9334, of the
   same family as the EPYC 9R45 of `sec:rust`, and reports that there "HOT
   shows clear advantage for the read-only workloads on skewed data sets
   (e.g., dblp and url)" [P]: on that hardware the trie to beat on URLs is
   HOT [I].
6. **Scale.** Hyperion's conclusion, "previous results on ranking data
   structures do not hold at this scale" and "Extrapolating average
   performance to larger indices can therefore result in false assumptions",
   supports the remark of lines 2455-2456 that the advantage of Table
   `tab:urlsb` appears only on much larger sets, and is a published
   precedent for measuring at a billion keys. For reference, on 7.95 billion
   string keys inserted in random order HOT takes 1.9 microseconds per get,
   ART 2.7, a red-black tree 6.7 [D from Table 1]; Table `tab:urlsb` has
   3.4 (z-fast trie), 7.3 (B-tree) and 7.7 (red-black tree) microseconds on
   a billion URLs, with keys more than three times longer, on unspecified
   hardware.
7. **Probes versus height** (lines 2453-2454: "between five and six probes
   per query"). The heights of the top-down competitors on URLs are printed:
   HOT 9 (maximum, 50M DBpedia URLs) and 7.8 (average leaf depth, 63M
   CommonCrawl URLs), ART 62 and 16.1, Masstree 44, c-trie++ 6.4 [P]. A fat
   binary search with 5.5 probes followed by a node, a leaf and a key is not
   shorter than a descent of HOT, whose upper levels are moreover shared by
   all queries and hence cached [I].

### Corrections to `practical-indexes.md` and `static-and-uses.md`

- Section 7.1, "nobody else reports a billion strings": **false**. Hyperion:
  7.95 billion 2-grams, and an experiment that fills 978 GiB with 3-grams
  (Figure 13). The row for n-gram datasets is missing from the table of
  datasets (Google Books Ngram version 2, English 2-grams a-d, average 22.65
  B).
- Section 3.5 (HOT): the measurements listed (YCSB, workload C and E, "1.3,
  0.9, 0.6, 0.3 Mops", "Section 6.3") are those of the SIGMOD paper and are
  not in the journal version, whose read-only numbers on `url` are 1.66
  (HOT), 1.13 (Masstree), 0.65 (ART), 0.39 (B+-tree). The memory range "11.4
  to 14.4 bytes per key" becomes 11.5 to 14.4 for the fast layout and 9.2 to
  9.7 for the compact one. "Fanout at most 32" holds for the fast layout
  only (256 for the compact one). The item of `TO-DOWNLOAD.md` ("New
  experiments of the journal version") is answered: the new experiments are
  the comparison of node layouts.
- Section 3.7 (Hyperion): the description from the abstract is accurate;
  "no code located" agrees with the PDF, which prints no URL.
- Section 5 (SIndex): "up to 91% better performance" is against Masstree on
  random strings; on URLs SIndex is 25% slower than Wormhole in its own
  paper. The repository printed in the paper is the SJTU GitLab URL above.
- Section 5 (LITS): the journal version reports up to 1.78x (HOT) and 1.59x
  (ART) on read-only and up to 2.19x and 1.85x on write-only workloads; HOT
  is best on scans; the table of depths cited in section 8.2 as "Table 3" is
  Table 4 in the journal version (HOT 7.8 on `url`). Section 2.5: the context
  of the citation of *Dynamic Z-Fast Tries* is the description of c-trie++
  (section 4 above).
- Section 3.2 and 7.3 (burst tries, HAT-tries, "not ordered or obsolete"):
  a burst trie with tree containers is ordered; with list containers, and in
  a HAT-trie, the containers are ordered among themselves and unsorted
  inside. The HAT-trie paper read is the ACSC 2007 one (pages confirmed).
- `static-and-uses.md`, line 632: "open question" should be "open problem"
  (section 5 above).

## Proposed edits to `dynzfast.tex`

All new text is pure ASCII and uses only macros already used in the file.
E1-E3 are recommended; E4 and E5 are optional.

**E1. Lines 561-562** (HOT: cite the journal version, name the quantity).

Old:

```latex
Masstree~\cite{Mao2012Cache} (a trie whose nodes are B$^+$-trees indexing eight bytes of the keys) and HOT~\cite{Binna2018HOT} (a binary Patricia trie whose nodes are 
grouped into compound nodes of bounded fanout so as to minimize the height), perform a number of steps proportional to the depth of the trie.
```

New:

```latex
Masstree~\cite{Mao2012Cache} (a trie whose nodes are B$^+$-trees indexing eight bytes of the keys) and HOT~\cite{Binna2018HOT,Binna2022Height} (a binary Patricia trie whose nodes are 
grouped into compound nodes of bounded fanout so as to minimize the height), perform a number of steps proportional to the height of the resulting tree
(on $50$ million URLs,~\cite{Binna2022Height} reports a height of $62$ for ART, $44$ for Masstree and $9$ for HOT).
```

Reason: the minimality of the height is proved in the journal version
(section 4), which is the version to cite for it; "depth of the trie" is
ambiguous for HOT. The parenthesis is optional (Figure 22 of the journal
version, row "Tree Height", column `url`); drop it if the paragraph must stay
short.

**E2. Line 567** (learned indices: add the first one and the journal
version).

Old:

```latex
learned indices for strings have been proposed, too~\cite{Spector2021Bounding,Yang2024LITS}. 
```

New:

```latex
learned indices for strings have been proposed, too~\cite{Wang2020SIndex,Spector2021Bounding,Yang2024LITS,Yang2026Learned}. 
```

Reason: SIndex (2020) is the first learned index for strings and is a
baseline of the other two; the journal version of LITS is the one that cites
this line of work (through c-trie++).

**E3. Line 581** (PIM-trie: say what is used; move to the list of uses).

Old:

```latex
The z-fast trie has also been used as a building block of a trie for the processing-in-memory model~\cite{Kang2023PIM}.
```

New (delete the sentence at line 581 and add an item to the `itemize` of the
paragraph "Uses of z-fast tries", for instance after the item on parsings in
compressed space, line 597):

```latex
\item in the \emph{PIM-trie} of Kang \textit{et al.}~\cite{Kang2023PIM}, a batch-parallel trie for the processing-in-memory model 
whose design, in the words of the authors, ``builds upon the idea in z-fast tries of combining both a radix tree and a hash table'': 
z-fast tries on strings shorter than $w$ bits, attached to the nodes whose depth is a multiple of $w$, lead with high probability in 
time $O(\lg w)$ to the nodes of the compacted trie in the following $w$ levels;
```

If the sentence must stay where it is, replace it with:

```latex
Small z-fast tries (on strings shorter than $w$ bits) are also a component of the PIM-trie~\cite{Kang2023PIM}, a batch-parallel 
trie for the processing-in-memory model, for which no implementation is reported.
```

Reason: the present sentence is correct but does not say what is used
(section 4.4.2 of the paper), and PIM-trie is a theoretical structure, not
an engineered main-memory index.

**E4 (optional). After line 567** (Hyperion and the scale of the
experiments). Add:

```latex
Hyperion~\cite{Masker2019Hyperion}, a trie designed for space efficiency, is to our knowledge the only dynamic structure of this kind that 
has been evaluated on billions of strings ($7.95$ billion $n$-grams with an average length of $23$ bytes).
```

Reason: it is the only published point of comparison for the scale of Table
`tab:urlsb`. "To our knowledge" is the authors' statement to make: what is
verified here is that no other paper of this batch or of the survey goes
beyond 200 million strings. If added, the sentence fits equally well in the
section on experiments, where the billion URLs are discussed (after line
2401). Note that the paragraph "Static dictionaries of strings" now mentions
static "dictionaries of billions of strings" (`Ferragina2025Two`, a sentence
added while this report was being written): the claim of E4 concerns dynamic
structures only, hence the word "dynamic" in the proposed sentence.

**E5 (optional). Lines 611-612** (measurements in the literature). Old:

```latex
The measurements of a z-fast trie reported in the literature we are aware of (e.g.,~\cite{TKKctriepp,Kanda2020Dynamic,Zhang2026C2}) 
use a C++ port of our Java implementation, which works on bits and stores one key per leaf.
```

New:

```latex
The measurements of a z-fast trie reported in the literature we are aware of (e.g.,~\cite{TKKctriepp,Kanda2020Dynamic,Zhang2026C2}) 
use a C++ port of our Java implementation, which works on bits and stores one key per leaf; c-trie++ itself is one of the baselines of~\cite{Yang2026Learned}.
```

Reason: this is the only way in which z-fast tries enter the evaluation of a
database paper of this batch. Not needed if E2 is applied and the paragraph
is to stay short.

No edit is proposed for `sec:rust`: the points of the synthesis concern which
experiments to add (HOT and ART as competitors, a larger set, index size
excluding keys), not the wording of the present text. If one sentence is
wanted there, the natural place is after "with the B-tree of the Rust
standard library" (line 2438), to say that tries such as ART and HOT, which
on URLs are reported to be 1.7 to 4.3 times faster than a B+-tree with
pointers to the keys~\cite{Binna2022Height}, are not included yet.

## Entries for `biblio.bib`

Already in `biblio.bib` and verified: `Binna2018HOT`, `Yang2024LITS`,
`Kang2023PIM` (no change). To add (ASCII; the first corrects, and the others
confirm, the entries of `practical-indexes.bib`):

```bibtex
% Verified on the PDF: article number 3, 46 pages, April 2022.
@article{Binna2022Height,
  author  = {Robert Binna and Eva Zangerle and Martin Pichl and G{\"u}nther Specht and Viktor Leis},
  title   = {Height Optimized Tries},
  journal = {ACM Transactions on Database Systems},
  volume  = {47},
  number  = {1},
  pages   = {3:1--3:46},
  year    = {2022},
  doi     = {10.1145/3506692}
}

% Authors, title, venue and DOI verified on the accepted manuscript; the
% page numbers are from Crossref (not printed in the manuscript).
@inproceedings{Masker2019Hyperion,
  author    = {Markus M{\"a}sker and Tim S{\"u}{\ss} and Lars Nagel and Lingfang Zeng and Andr{\'e} Brinkmann},
  title     = {Hyperion: Building the Largest In-memory Search Tree},
  booktitle = {Proceedings of the 2019 International Conference on Management of Data ({SIGMOD}~'19)},
  year      = {2019},
  pages     = {1207--1222},
  publisher = {ACM},
  doi       = {10.1145/3299869.3319870}
}

% Verified on the PDF (pages 17-24 printed).
@inproceedings{Wang2020SIndex,
  author    = {Youyun Wang and Chuzhe Tang and Zhaoguo Wang and Haibo Chen},
  title     = {{SIndex}: A Scalable Learned Index for String Keys},
  booktitle = {Proceedings of the 11th {ACM} {SIGOPS} Asia-Pacific Workshop on Systems ({APSys}~'20)},
  year      = {2020},
  pages     = {17--24},
  publisher = {ACM},
  doi       = {10.1145/3409963.3410496}
}

% Verified on the "Just Accepted" PDF (accepted 26 July 2026): no volume,
% issue or article number yet. Update when the issue is assigned.
@article{Yang2026Learned,
  author  = {Yifan Yang and Shimin Chen},
  title   = {Learned Indices for String Keys with Optimized Performance and {$O(N)$} Space Cost},
  journal = {ACM Transactions on Database Systems},
  year    = {2026},
  doi     = {10.1145/3841175},
  note    = {Just accepted}
}

% Verified on the PDF (volume, number, pages printed); DOI as printed in the
% reference lists of the HOT and Hyperion papers.
@article{Heinz2002Burst,
  author  = {Steffen Heinz and Justin Zobel and Hugh E. Williams},
  title   = {Burst Tries: A Fast, Efficient Data Structure for String Keys},
  journal = {ACM Transactions on Information Systems},
  volume  = {20},
  number  = {2},
  pages   = {192--223},
  year    = {2002},
  doi     = {10.1145/506309.506312}
}

% Verified on the PDF (conference, volume 62, editor, pages 97-105 printed).
% No DOI.
@inproceedings{Askitis2007HAT,
  author    = {Nikolas Askitis and Ranjan Sinha},
  title     = {{HAT}-trie: A Cache-conscious Trie-based Data Structure for Strings},
  booktitle = {Proceedings of the Thirtieth Australasian Computer Science Conference ({ACSC}~2007)},
  editor    = {Gillian Dobbie},
  series    = {Conferences in Research and Practice in Information Technology},
  volume    = {62},
  year      = {2007},
  pages     = {97--105},
  publisher = {Australian Computer Society},
  address   = {Ballarat, Australia}
}
```

`Askitis2010Engineering` (the VLDB Journal version of the HAT-trie) stays
unverified: that paper is not among the PDFs.
