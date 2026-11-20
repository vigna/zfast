# B-trees for strings, high-probability dictionaries, Rabin fingerprints: statements verified on the primary sources

Checked on 2026-11-20 against seven PDFs in `../../related/`. Subject: the
sentence on B-trees for strings in the paragraph "Ordered indices for main
memory" of `sec:related`, the third level of the paragraph "The dictionary"
of the introduction, and the relation between the analysis of signatures in
`sec:dict` and CRC-like hashes.

**Line numbers** refer to `dynzfast.tex` as it was when this report was
written (2596 lines). Every proposed edit quotes the old text: locate edits
by text.

**Marks** (as in `verified-hashing.md`). [P] printed in the PDF (page numbers
are those printed on the pages). [I] my inference from printed statements.
[D] my own derivation or computation (written so that it can be checked).
[G] general knowledge, not verified on any source read here. [S] taken from a
secondary source, which is named.

**None of the seven papers is cited in `dynzfast.tex` at present** (no
`\cite` contains `Graefe`, `Gao2024`, `Arbitman2009`, `Bender2023`, `Rabin`
or `Broder`). The sentences listed below under each paper are therefore
sentences that make a claim the paper bears on, not sentences citing it.

**Which versions were read.**

| PDF | What it is | Read |
|---|---|---|
| `ModernBTreeTechniques.pdf` | Foundations and Trends in Databases 3(4), 203-402; the publisher's version (203 pages) | Contents, abstract, Sections 3.3-3.6 (pp. 235-246), reference list; text search of the whole file |
| `BTreeIndexes.pdf` | ICDE 2001, pp. 349-358 (IEEE Xplore scan) | In full |
| `RevisingBTrees.pdf` | Proc. ACM Manag. Data 2(3) (SIGMOD), Article 169, June 2024, 25 pages. **The title is "Revisiting", not "Revising"** | pp. 1-10 and 18-23 as pages; pp. 11-17 (synthetic data) as text only |
| `DeamortizedCuckooHashing.pdf` | ICALP 2009, Part I, LNCS 5555, pp. 107-118: an extended abstract without proofs | In full |
| `IcebergHashing.pdf` | Journal of the ACM 70(6), Article 40, November 2023, 51 pages | pp. 40:1-40:8, 40:14-40:15, 40:18-40:20, 40:31-40:32, 40:41-40:43; statements of Theorems 9-11 and text search for the rest. Proofs not read |
| `FingerprintingByRandomPolynomials.pdf` | Scan of the report TR-15-81: two title pages and 12 pages of text. No text layer: read as images | In full |
| `SomeApplications.pdf` | An author's version of 10 pages numbered 1-10 (DEC Systems Research Center), not the pages of the Springer volume | In full |

## Summary

- **B-trees.** The sentence at lines 560-561 is accurate, and the attribution
  of prefix truncation and short separators to Bayer and Unterauer is
  confirmed by all three papers (their paper itself was not read). What is
  missing is the origin of the key bytes stored in the node: they are the
  "poor man's normalized keys" named by Graefe and Larson (2001), who say the
  idea comes from sorting (AlphaSort, 1995). Fingerprints and hints do not
  appear in any of the three papers: they belong to the 2025 papers already
  cited. The only measurements are those of Gao et al.: prefix truncation
  plus short separators ("Head+Tail") is the fastest of seven schemes, 25% to
  120% faster on point queries than a B-tree with whole keys in the nodes.
  No paper measures a B-tree whose nodes contain bare pointers to the keys;
  the closest design measured (pkB-tree: pointer plus partial key) loses to
  Head+Tail on every dataset.
- **Dictionaries.** De-amortized cuckoo hashing has the same restrictions as
  backyard cuckoo hashing (capacity n fixed in advance, keys of one word,
  polynomially many operations) and uses more space, 2(1 + epsilon)n +
  n^epsilon words. Iceberg hashing removes the restriction: Theorem 5 gives
  constant time per operation w.h.p. in the *current* size n and (1 + o(1))n
  words, with dynamic resizing; the paper credits Liu, Yin and Yu (ICALP
  2020, not read) with the first solution of the open problem posed by
  Arbitman, Naor and Segev.
- **Fingerprints.** Rabin's fingerprint is the residue of the string, seen as
  a polynomial over Z_2, modulo a random irreducible polynomial of degree k.
  Rabin never discusses lengths (all his strings have the same length);
  Broder prefixes every string with a 1. The collision bound per pair is
  about L/2^k for strings of at most L bits. The union-bound argument of
  `sec:dict` goes through unchanged with lg w more bits of signature; what
  does *not* go through in the paper's model is the constant-time
  computation of prefix signatures, which needs tables or carry-less
  multiplication. A CRC is the same computation with a fixed polynomial, for
  which no bound holds. A short remark is justified; it does not resolve the
  `\todo` at line 1443, because the paper's own implementations use
  MurmurHash2 and xxh3, not CRCs.

---

## 1. Graefe, "Modern B-tree techniques"

**Version** [P, first page]: "Foundations and Trends in Databases Vol. 3,
No. 4 (2010) 203-402", "(c) 2011 G. Graefe", "DOI: 10.1561/1900000028";
author's affiliation Hewlett-Packard Laboratories. It is the publisher's
version. The volume is dated 2010 and the copyright 2011; Gao et al. cite it
as 2011 (their reference [29]).

**Entry.** `Graefe2011Modern` in `practical-indexes.bib` is correct in every
field (the comment "Full text not read" can be updated). Not in `biblio.bib`.

**Sentences of `dynzfast.tex`.**

- Lines 560-561: "Current B-trees for strings truncate the prefixes common to
  all keys of a node and use short
  separators~\cite{Bayer1977Prefix}, and they store in each node a few bytes
  of the keys after the common prefix, and hash-based fingerprints, so as to
  avoid accessing the keys~\cite{Muller2025BTrees,Chen2025FB};". Verdict:
  **accurate, attribution incomplete** (see question A).

**What the paper says that matters here.**

- Section 3.3, "Variable-length Records" (pp. 235-237) [P]: "B-trees in
  practically all database systems support variable-length records and
  variable-length separator keys"; the standard design is "an indirection
  vector (also known as slot array) with entries of fixed size", each
  containing "the byte offset of the record" within the page. The keys are
  *in the page*; the "pointer" is a byte offset.
- Section 3.4, "Normalized Keys" (pp. 237-239) [P]: "many implementations of
  B-trees transform keys into a binary string such that simply binary
  comparisons suffice"; "Key normalization is a very old technique. It is
  already mentioned by Singleton [118] without citation" (Singleton, CACM
  12(3), 1969). No inventor is named.
- Section 3.5, "Prefix B-trees" (pp. 239-244) [P]: "prefix and suffix
  truncation or compression [10]", where [10] is Bayer and Unterauer, ACM
  TODS 2(1):11-26, 1977. "Prefix truncation analyzes the keys in a B-tree
  node and stores the common prefix only once"; "suffix truncation pertains
  specifically to separator keys in branch nodes [10]"; "the separator is
  chosen as the shortest string that separates those two keys in the
  leaves". The prefix can be defined by the two *fence keys* of the node
  rather than by its current keys (p. 240). "Dynamic prefix truncation"
  skips, during a search, the bytes shared by the bounds of the current
  interval (p. 241). "normalized keys, prefix truncation, suffix truncation,
  and interpolation search are a likely combination in an implementation"
  (p. 241). On tries: "a trie representation could attempt to combine the
  advantages of prefix truncation and binary search, but it is used in very
  few database systems. The probable reasons are code complexity and update
  overhead" (p. 241).
- Section 3.6, "CPU Caches" (pp. 244-246) [P]: "After prefix truncation has
  been applied, many comparisons in a binary search are decided by the first
  few bytes. [...] If only those few bytes are stored, not the entire
  normalized key, such that they can decide many but not all comparisons,
  they are called 'poor man's normalized keys' [41]. In order to enable key
  comparisons and search without cache faults for data records, poor man's
  normalized keys can be an additional field in the elements of the
  indirection vector. This design has been employed successfully in the
  implementation of AlphaSort [101] and can be equally beneficial in B-tree
  pages [87]"; "2 or 4 bytes seem more appropriate, depending on the page
  size" (p. 245). [87] is Lomet, SIGMOD Record 30(3), 2001; [101] is Nyberg
  et al., VLDB Journal 4(4), 1995.
- The reference [41] is misprinted in the survey as 'G. Graefe, "Per-Ake
  Larson: B-tree indexes and CPU caches," International Conference on Data
  Engineering, pp. 349-358, 2001' [P]: the second author has become part of
  the title.
- The words "fingerprint" and "partial key" do not occur in the text layer
  of the survey, and Bohannon et al. are not in its reference list [P, text
  search]. The survey contains no measurement on strings.

## 2. Graefe, Larson, "B-tree indexes and CPU caches"

**Version** [P]: title "B-tree Indexes and CPU Caches", Goetz Graefe and
Per-Ake Larson (printed with the ring on the A), Microsoft; pages 349-358;
footer "1063-6382/01 $10.00 (c) 2001 IEEE". The name of the conference is not
printed on the pages (Gao et al. cite it as "In ICDE. 349-358").

**Entry.** `Graefe2001Btree` in `practical-indexes.bib` is correct; the first
names, which its comment gives as "from memory", are confirmed by the first
page. Not in `biblio.bib`.

**Sentences of `dynzfast.tex`.** Lines 560-561, as above.

**What the paper says.** It is a survey without experiments [P, abstract]:
"Rather than providing a detailed performance evaluation for one or two of
them on some specific contemporary hardware, the purpose here is to survey
and to make widely available this heretofore-folkloric knowledge".

- Attributions (Section 2, p. 350) [P]: "The idea of storing short
  separators instead of complete keys in internal pages is due to Bayer and
  Unterauer [6]"; "Extracting (normalized) key prefixes to speed up
  comparisons and reduce cache misses, here referred to as 'poor man's
  normalized keys', was introduced in the context of sorting [14]", where
  [14] is AlphaSort (Nyberg, Barclay, Cvetanovic, Gray, Lomet, VLDB Journal
  4(4):603-627, 1995).
- Section 5.3, "Prefix and suffix truncation" (p. 353) [P]: "it makes sense
  to store those equal bytes only once, e.g., for an entire B-tree node, and
  then to remove those leading bytes (the common prefix) from all individual
  entries [6]"; "it might be useful to split nodes not precisely in the
  middle but by the shortest possible separator near the middle, as
  originally proposed in [6]".
- Section 6.1, "Poor man's normalized keys" (p. 355) [P]: "most B-tree
  implementations use indirection vectors containing byte offsets and, in
  many implementations, record lengths. A third candidate field in these
  indirection records is a prefix of the normalized key. If, for example,
  each indirection record contains the first 2, 4, 6, or even 8 bytes of the
  key, many searches never have to inspect the remaining bytes in the B-tree
  entries, and therefore do not incur cache faults for these bytes. We call
  this fixed-size prefix a poor man's normalized key. This technique works
  well only if common key prefixes are truncated before the poor man's
  normalized keys are extracted."
- Section 6.4 (p. 357) [P]: for keys of varying length, "It might be more
  effective to focus on prefix truncation and poor man's normalized keys
  that suffice for most comparisons in the search."
- Open issue 3 of the conclusions (p. 358) [P]: "The effectiveness of poor
  man's normalized keys (and other techniques) in B-tree pages compared to
  their effects in sorting": in 2001 the technique had not been measured in
  B-trees.
- Fingerprints and hash values stored next to the keys are not mentioned
  [P, read in full]; the only use of hashing is "a B-tree structure [...]
  used to implement an index on hash values" (p. 356).

## 3. Gao, Ballijepalli, Wang, "Revisiting B-tree compression: an experimental study"

**Version** [P, first page]: "Chuqing Gao, Shreya Ballijepalli, and Jianguo
Wang. 2024. Revisiting B-tree Compression: An Experimental Study. Proc. ACM
Manag. Data 2, 3 (SIGMOD), Article 169 (June 2024), 25 pages.
https://doi.org/10.1145/3654972"; all authors at Purdue University;
"Received October 2023; revised January 2024; accepted February 2024"
(p. 169:25). The file name ("Revising") is wrong; title and venue are as
expected.

**Entry.** `Gao2024Revisiting` in `practical-indexes.bib` is correct. Not in
`biblio.bib`.

**Sentences of `dynzfast.tex`.** Lines 560-561 (as above), and the
descriptions of the baselines: lines 2353-2354 ("Google's publicly available
implementation of B-trees, both of which are considered of very high
quality") and lines 2429-2430 ("the B-tree of the Rust standard library
(\texttt{BTreeSet}, whose nodes contain up to $11$ pointers to keys)"). No
verdict of error: see question A for what the paper implies about such
baselines.

**What the paper says.**

- Attribution (pp. 169:1-169:4) [P]: "a technique that was initially
  proposed in 1977 [15] [...] Two types of B-tree compression techniques
  were proposed in [15]: Tail Compression and Head Compression", [15] being
  Bayer and Unterauer. Tail compression posts to the parent "the shortest
  possible separator that can distinguish between the left and right nodes"
  and may move the split point; head compression stores once per node the
  common prefix "between these bounds", the bounds being the "largest lower
  bound and smallest upper bound from the parent nodes" (Graefe's fence
  keys), so that "adding new keys to a B-tree node does not need the
  re-computation of the prefix".
- Seven schemes (Section 2) [P]: Tail, Head, Head+Tail, IBM DB2 (several
  prefixes per page), MongoDB WiredTiger and MySQL MyISAM (each key is coded
  as a difference from the previous one), and the pkB-tree of Bohannon,
  McIlroy and Rastogi (SIGMOD 2001, their [18]): "a pointer to the key is
  stored instead of the entire key. PkB was proposed to reduce CPU cache
  misses instead of saving the total space overhead. Since dereferencing the
  pointer increases cache misses, the partial key can help avoid the full
  comparison" (p. 169:7). **PkB is the only scheme in which the nodes
  contain pointers to the keys**; in all the others, including the
  "uncompressed" baseline, the keys are stored in the pages.
- Setup (Section 3) [P]: Xeon Gold 6330 at 2.00 GHz, 128 GB, GCC 11.4.0 with
  O3; all schemes reimplemented by the authors in C++; no values stored;
  page size 512 B in memory by default (Table 2), 1 KB for TPC-H ("within a
  1KB-length page", p. 169:19), 4 KB for WEBSPAM (p. 169:19); averages of
  three runs; 20% of the keys are used to warm up and 80% for the measured
  insertions, searches are made on the full set.
- Datasets (Table 1, p. 169:8) [P]:

  | Dataset | Keys | Min | Max | Average |
  |---|---|---|---|---|
  | TPC-H (LINEITEM, composite key as one string) | 60M | 96 B | 156 B | 129.58 B |
  | WEBSPAM-UK2007 (URLs) | 25M | 16 B | 2047 B | 112.33 B |
  | WikiTitles | 25M | 1 B | 255 B | 19.34 B |
  | MemeTracker URLs | 5.5M | 8 B | 21.2 KB | 75.90 B |

  plus synthetic strings of random digits (default: 100M keys of 32 bytes).
- Results, as stated in the text [P]: on the default synthetic set Head+Tail
  "exhibits the highest throughput, surpassing the uncompressed B-tree by
  40.7%"; Head alone gives 5.7%, Tail alone 27.2% (p. 169:10). Summary
  (Section 7.1, p. 169:23): "Head+Tail Compression, although proposed in the
  1970s [15], consistently exhibits the best search performance among all
  B-tree compression techniques. It achieves a 25% ~ 120% performance
  improvement over the uncompressed B-tree across different datasets"; it is
  "21.7% ~ 90.2% faster than DB2", "129% ~ 684% faster than MongoDB
  WiredTiger", "7.3% ~ 139% faster than MySQL MyISAM"; for insertions "a 10%
  ~ 30% performance improvement over the uncompressed B-tree"; compression
  ratio of Head+Tail and WiredTiger "from 1.2X ~ 3.5X". "Overall, we
  recommend Head+Tail Compression for B-trees" (Section 7.2).
- Results on strings, **read from the bar charts** of Figures 19 and 21-23
  (in-memory point queries, million operations per second; my readings,
  accurate to about 0.05) [P, plots]:

  | Dataset | Uncompressed | Head | Tail | Head+Tail | MyISAM | PkB |
  |---|---|---|---|---|---|---|
  | TPC-H | 0.75 | 0.85 | 0.8 | 1.0 | 0.75 | 0.5 |
  | WEBSPAM-UK2007 | 0.4 | 0.9 | 0.35 | 0.85 | 0.8 | 0.55 |
  | MemeTracker | 0.55 | 0.55 | 0.65 | 1.0 | 0.9 | 0.55 |
  | WikiTitles | 1.5 | 2.0 | 1.6 | 2.05 | 1.05 | 0.5 |

  On WEBSPAM the bar of Head is slightly higher than that of Head+Tail,
  which does not agree with "consistently" in the summary [I, from the
  plot]. The absolute figures (one to two and a half microseconds per
  query on at most 25M keys) are those of a page-based prototype and are
  not comparable with those of `dynzfast.tex`.
- Table 6 (p. 169:21, uncompressed B-tree on 10M keys sampled from each
  dataset) [P]: WikiTitles key length 25.19, fan-out 57.42, height 4;
  MemeTracker 79.65, 22.11, 6; WEBSPAM 109.60, 11.81, 8; TPC-H 131.69,
  13.77, 6. "From TPC-H to MemeTracker, the average comparison length
  changed from 8.92 to 25.67, resulting in an increase in proportion of
  comparison time in the total query time from 62.5% to 71.2%" (p. 169:22):
  comparing keys is where the time of a B-tree on URLs goes.
- Not evaluated [P, text search]: poor man's normalized keys, fingerprints,
  normalized keys, SIMD, tries (ART is mentioned in the related work only).
  Graefe's survey and Graefe and Larson are cited once each, in the
  introduction.

## 4. Arbitman, Naor, Segev, "De-amortized cuckoo hashing"

**Version** [P, first page]: "De-amortized Cuckoo Hashing: Provable
Worst-Case Performance and Experimental Results"; Yuriy Arbitman, Moni Naor,
Gil Segev; "S. Albers et al. (Eds.): ICALP 2009, Part I, LNCS 5555, pp.
107-118, 2009. (c) Springer-Verlag Berlin Heidelberg 2009". It is an extended
abstract: "Due to space limitations we refer the reader to a longer version
available at http://www.wisdom.weizmann.ac.il/~naor". The theorem is stated
without proof.

**Entry.** `Arbitman2009Deamortized` in `theory.bib` is correct; the volume
(5555) and "Part I" can be added (see the last section). Not in `biblio.bib`.

**Sentences of `dynzfast.tex`.** Lines 384-387, third item of "The
dictionary": "If an upper bound $N$ on $n$ is known in advance, there are
dictionaries using $O(N)$ words in which, for every sequence of $N^{O(1)}$
operations, with probability $1-1/N^{O(1)}$ every operation takes constant
time in the worst case~\cite{Arbitman2010Backyard} (the result is stated for
keys of one word)". Verdict: **correct**; this paper satisfies the same
statement and can be cited next to the other.

**What the paper says** [P].

- "Theorem 2.2. For any polynomial p(n) and constant epsilon > 0, the
  parameters of the dictionary can be set such that the following properties
  hold: 1. For any sequence of at most p(n) insertions, deletions, and
  lookups, in which at any point in time at most n elements are stored in
  the dictionary, with probability at least 1 - 1/p(n) each operation is
  performed in constant time, where the probability is over the randomness
  of the initialization phase. 2. The dictionary utilizes 2(1 + epsilon)n +
  n^epsilon words." (p. 114; the same in Section 1.1, p. 110.)
- All three operations are covered. The structure is cuckoo hashing with two
  tables of (1 + epsilon)n entries and a queue: each insertion performs at
  most L moves (a constant) and leaves the pending element in the queue,
  which holds O(log n) elements with overwhelming probability; "we never
  perform rehashing" (p. 110), elements closing a second cycle being stashed
  in the queue.
- **The capacity n is fixed**: the tables are allocated for n, and the words
  "resize" and "resizing" do not occur [P, text search]. Space is counted
  with respect to n, not to the current number of elements.
- Keys of one word: "each word is represented using log u bits where u is
  the size of the universe of elements" (p. 117).
- Hash functions: the analysis assumes truly random functions and then shows
  that "the same performance guarantees hold when instantiating our scheme
  with hash functions that are only polylog(n)-wise independent (see [7,15]
  for efficient constructions of such functions with succinct
  representations and constant evaluation time)", through Braverman's
  theorem on AC0 circuits (pp. 114-115); [7] is Dietzfelbinger and Woelfel
  (STOC 2003), [15] Ostlin and Pagh (STOC 2003).
- Data associated with the keys are not mentioned.
- On the predecessor: "Dietzfelbinger and Meyer auf der Heide [5]
  constructed the first dynamic dictionary with worst case time per
  operation and linear space [...]. Specifically, for any constant c > 0
  (determined prior to the initialization phase) and for any sequence of
  operations involving n elements, with probability at least 1 - n^{-c} each
  operation is performed in constant time (that depends on c)" (p. 109).
  Reference [5] is printed as "ICALP 1990. LNCS, vol. 443, pp. 6-19.
  Springer, Heidelberg (1990)": this settles the year and the pages that the
  comment of `Dietzfelbinger1990New` in `theory.bib` marks as unchecked.
- Experiments: with two tables of 1.2n entries, "3 moves are sufficient per
  insertion" up to n = 5 * 10^7 (Section 3, Figure 2).

## 5. Bender, Conway, Farach-Colton, Kuszmaul, Tagliavini, "Iceberg hashing"

**Version** [P, first page]: "Iceberg Hashing: Optimizing Many Hash-Table
Criteria at Once"; "Journal of the ACM, Vol. 70, No. 6, Article 40.
Publication date: November 2023"; "51 pages"; "https://doi.org/10.1145/3625817".

**Entry.** `Bender2023Iceberg` in `theory.bib` is correct; its note "The
article number was not checked" can go: it is Article 40, pages 40:1-40:51.
Not in `biblio.bib`.

**Sentences of `dynzfast.tex`.** Lines 384-387 (as above). Verdict on the
sentence: correct for the paper it cites, but **the proviso "If an upper
bound $N$ on $n$ is known in advance" is no longer necessary**: see
question B.

**What the paper says** [P].

- The restriction and who removed it (p. 40:5): "A hash table supports
  dynamic resizing if the space consumption is a function of the current
  number of records n, rather than some upper bound N on the number of
  records that could ever be in the data structure. Arbitman, Naor, and
  Segev [4] pose the open question of how to maintain a constant-time,
  space-efficient hash table that supports dynamic resizing. Recently, Liu,
  Yin, and Yu [39] gave an elegant solution to this problem". [39] is Liu,
  Yin, Yu, "Succinct filters for sets of unknown sizes", ICALP 2020 (not
  read). Section 8 (p. 40:41) says of backyard cuckoo hashing: "the first
  hash table to both be succinct and provide all worst-case costs (although
  it is not dynamically resizable)".
- Static capacity, Theorem 2 (p. 40:19): "Consider an Iceberg hash table
  that never contains more than N elements and suppose that the
  average-bin-fill parameter satisfies h = O(log N / log log N). Suppose
  that the backyard table supports constant-time operations (w.h.p. in n),
  supports load factor at least 1/poly h, and is stable. [...] Time
  Efficiency. The operation on x takes constant time in the RAM model,
  w.h.p. in N. [...] Space Efficiency. The total space in machine words
  consumed by the table is, w.h.p. in N, (1 + O(sqrt(log h)/sqrt(h))) N =
  (1 + o(1))N. Stability. The hash table is stable."
- Dynamic capacity, Theorem 5 (p. 40:31): "Consider a dynamic Iceberg hash
  table with average-bin-fill parameter h. Suppose that the number n of
  elements stays in the range such that log n / log log n >= Omega(h).
  Suppose that the table used in the backyard supports constant-time
  operations (w.h.p. in n), has load factor at least 1/poly(h), and is
  stable. Finally, set the resize granularity s = sqrt(h). Consider an
  operation on a key x. The following guarantees hold. Time Efficiency. The
  operation on x takes constant time in the RAM model, w.h.p. in n. [...]
  Space Efficiency. The total space in machine words consumed by the table,
  w.h.p. in n, is (1 + O(sqrt(log h)/sqrt(h))) n = (1 + o(1))n." Here n is
  the current number of records. The range condition is removed by
  rebuilding the table with a new h "whenever log n changes by more than a
  factor of two" (Remark 5, p. 40:32). Resizing is de-amortized: "the work
  for the resize will be spread across O(n/s) operations" (Remark 1,
  p. 40:5).
- Operations: "insertions/queries/deletions take O(1) time w.h.p."
  (p. 40:3). The guarantee is **per operation**, not for all the operations
  of a sequence at once.
- "w.h.p." is not defined in the paper [P, text search]; that it means
  probability 1 - 1/poly(n) follows from Appendix A, where an "additional
  1/poly n probability of failure" is said to prevent "anything better than
  w.h.p. guarantees" [I]. The sequence of operations is that of an
  "oblivious adversary" (p. 40:9).
- The structure is not self-contained: the *backyard*, which holds
  N/poly(h) records, is "an arbitrary hash table implementation that
  supports constant-time operations (w.h.p.) and load factor at least
  1/poly h" (p. 40:14), and under resizing "can be maintained using any
  (deamortized) resizing scheme" (Section 4.3). The front yard stores, for
  each bin, a routing table of fingerprints of Theta(log h) bits packed in
  O(1) words.
- Hash functions: Sections 2-7 "assume access to constant-time fully random
  hash functions" (p. 40:14). Appendix A (pp. 40:42-40:43) gives explicit
  families "using O(n^alpha log n) random bits for a positive constant alpha
  > 0 of our choice" (the family of Pagh and Pagh, and tabulation hashing),
  which "introduce an additional 1/poly n probability of failure";
  "Theorem 11. Consider a low-randomness Iceberg hash table whose size stays
  in the range [N^{1-epsilon/4}, N]. Furthermore, suppose that the
  description bits for g and f fit in cache. Then the guarantees from
  Theorems 2, 5, 6, and 7 hold", and "the size restriction on the hash table
  can be removed by performing random rebuilds very rarely" (Remark 7).
- Key size (p. 40:42): "In general, Iceberg hashing allows for keys as large
  as Theta(w) bits, where w is the machine word size. We can assume without
  loss of generality, however, that keys are Theta(log n) bits. In
  particular, prior to computing the hash of a key x, we can use
  pairwise-independent hashing to map x to an intermediate value x' that is
  Theta(log n) bits [...]. The intermediate values introduce a 1/poly n
  probability of collision between pairs of keys, but this is easily
  absorbed into the failure probability of a hash table."
- Values: "we will often treat a record as a key (rather than a key-value
  pair)" (footnote 13, p. 40:14); a record is a key with its value.
- Not relevant here: stability, cache optimality, the succinct version
  (Theorem 10, for |U| <= poly n), and the sub-polynomial failure
  probability 2^{-n^{1-epsilon}} of Theorem 9, which needs keys of
  Theta(log n) bits and fully random hash functions.
- History (Section 8, p. 40:41): "The first hash table to achieve
  constant-time operations with high probability was that of Dietzfelbinger
  et al. [16] in 1990"; [16] is Dietzfelbinger and Meyer auf der Heide,
  ICALP 1990, pp. 6-19.

## 6. Rabin, "Fingerprinting by random polynomials"

**Version** [P]: a scan. The cover says "FINGERPRINTING BY RANDOM
POLYNOMIALS by Michael O. Rabin TR-15-81"; the second page gives the
affiliations (Department of Mathematics, The Hebrew University of Jerusalem,
and Department of Computer Science, Harvard University) and the abstract.
**Neither the institution issuing the report nor the year is printed**: both
come from reference [4] of Broder's paper, "Center for Research in Computing
Technology, Harvard University, Report TR-15-81, 1981" [S]. The pages of the
text are numbered 1-12 (some numbers are not legible).

**Entry.** None exists: see `Rabin1981Fingerprinting` in the last section.

**Sentences of `dynzfast.tex`.**

- Line 579: "often with signatures given by Karp--Rabin fingerprints and a
  final verification". Verdict: **consistent**. Rabin's introduction (p. 1)
  describes the earlier method as the one in which "a randomly chosen prime
  p is used to 'fingerprint' a long character-string by computing the
  residue of that string, viewed as a large integer, modulo p", with
  reference to Karp and Rabin ("Submitted for publication"), and proposes
  polynomials "instead of the prime integer p": Karp-Rabin fingerprints and
  Rabin fingerprints are two different families, and the text names the
  first one, as the cited papers do. No change.
- Lines 1411-1426 (the definition of the signatures and the collision bound)
  and the `\todo` at line 1443: "The implementations use different hash
  functions, for which no such bound is available." Verdict: **true**, and
  not changed by this paper: see question C.

**What the paper says** [P].

- The family (abstract): "Randomly chosen irreducible polynomials p(t) in
  Z_2[t] are used to 'fingerprint' bit-strings." Section 3 (pp. 4-5): for a
  string x_1 ... x_n, "Let g(t) = x_1 t^{n-1} + x_2 t^{n-2} + ... + x_n",
  and the fingerprint is "the residue Res(g,p) of g when divided by p",
  "represented by the bit-vector (c_1,...,c_k)"; "For the small k <= 61
  which will actually be used, one or at most two computer words will
  suffice".
- Counting (Section 2, p. 2): "Lemma 1. Let k be prime. The number of
  irreducible polynomials p(x) = x^k + a_{k-1}x^{k-1} + ... + a_0 in Z_2[x],
  is (2^k - 2)/k." A random one is obtained from a random element of
  GF(2^k) \ Z_2, whose minimal polynomial is found by Gaussian elimination
  (Lemma 2); this needs a fixed irreducible polynomial of degree k, "tabulated
  once and for all". "in practice we can use k = 17, 19, ..., 31, 61".
- Incremental computation (p. 5): the fingerprint of the string extended by
  a bit x is obtained by "a left-shift of the word c_1 c_2 ... c_k,
  introduction of x on the right, followed by the bit-wise exclusive-or with
  b_1 b_2 ... b_k if the overflow c_1 is 1"; "one or two operations for
  every incoming bit". The cost is per *bit*.
- The bound (Section 4, p. 7): "Theorem 3. For any pattern pi = x_1...x_n
  and text tau = y_1...y_m, if k > log_2(nm epsilon^{-1}) then the
  probability that the above randomized algorithm will produce one or more
  errors is smaller than epsilon. Furthermore the algorithm will discover
  all actual matches." Proof: the product H(t) of the nonzero differences
  a(t) - a_i(t) has degree at most nm, so "at most nm/k irreducible
  polynomial p(t) of degree k will produce an error", out of (2^k - 2)/k.
- Files (Section 5, p. 11): "Proposition. If a file F consists of a thousand
  pages each containing up to 4000 bytes (32,000 bits) and we use
  polynomials of degree 61, then the probability of any unauthorized change
  in F to go undetected is at most 2^{-46}", because a nonzero difference of
  degree at most 2^25 "has at most 2^25/61 irreducible divisors of degree
  61. Since there are 2^61/61 irreducible polynomials of degree 61" (the
  exact number is (2^61 - 2)/61).
- **Lengths and leading zeroes are never discussed.** In Section 4 all the
  strings compared have the same length n, which is why "a match for index
  i is equivalent to a(t) = a_i(t)". In Section 5 pages hold "up to n bits",
  and nothing is said of pages of different lengths. As defined, the
  fingerprint does not distinguish a string from the same string preceded by
  zeroes: both give the same polynomial [I].
- There is no statement about words, the RAM, or the fingerprints of all the
  prefixes of a string.

## 7. Broder, "Some applications of Rabin's fingerprinting method"

**Version** [P]: ten pages numbered 1-10, Andrei Z. Broder, "DEC Systems
Research Center, 130 Lytton Ave, Palo Alto, CA", with the footnote
"Published in R. Capocelli, A. De Santis, U. Vaccaro (eds), Sequences II:
Methods in Communications, Security, and Computer Science, Springer-Verlag,
1993". It is an author's version: **the pages 143-152 of the entry are not
printed in the PDF** and remain as given by the DOI record quoted in
`theory.bib` [S].

**Entry.** `Broder1993Some` in `theory.bib`: correct as far as the PDF goes;
editors and the full title of the volume can be added (last section). Not in
`biblio.bib`.

**Sentences of `dynzfast.tex`.** Lines 1411-1426 and 1443, as for Rabin.

**What the paper says** [P].

- Definition (Section 2, p. 2): "Let A = (a_1, a_2, ..., a_m) be a binary
  string. We assume that a_1 = 1. (In certain application this assumption
  might be false, hence the implementation must actually prefix every string
  to be fingerprinted by a 1. We ignore this technicality for the rest of
  the paper.) We associate to the string A a polynomial A(t) of degree m - 1
  with coefficients in Z_2, A(t) = a_1 t^{m-1} + a_2 t^{m-2} + ... + a_m.
  Let P(t) be an irreducible polynomial of degree k, over Z_2. [...] we
  define the fingerprint of A to be the polynomial f(A) = A(t) mod P(t)."
  The leading 1 is what makes the map from strings to polynomials
  injective.
- The bound (p. 2): "Assume that an adversary chooses a set S of n distinct
  binary strings. After the adversary chooses S, we choose uniformly at
  random an irreducible polynomial P of degree k." With Q the product of
  A(t) - B(t) over the unordered pairs of distinct strings, "deg Q <= [...]
  <= n sum_{A in S} |A|. In particular if all the strings in S have length
  less than m, then the degree of Q is less than n^2 m. [...] This Q can not
  have more than deg(Q)/k irreducible factors of degree k. However the total
  number of irreducible polynomials of degree k with coefficients in Z_2 is
  greater than (2^k - 2^{k/2})/k. Hence the probability that a randomly
  chosen irreducible polynomial of degree k divides Q, which is the same as
  the probability that two distinct strings have the same fingerprint, is
  less than (deg Q / k) * k/(2^k - 2^{k/2}) ~ deg Q / 2^k <= nm^2/2^k. (7)"
  Example: "any collection of 2^15 binary strings of total length less than
  2^25 bits", degree 64: "less than 2^{-23}".
- Two slips in that passage. (i) Formula (7) ends with "nm^2/2^k", while the
  text two lines above gives deg Q < n^2 m: it should read n^2 m/2^k [I].
  (ii) "greater than (2^k - 2^{k/2})/k" is false for every even k: there are
  9 irreducible polynomials of degree 6 and (2^6 - 2^3)/6 = 9.33, and for k
  = 32 and k = 64 the number is exactly (2^k - 2^{k/2})/k [D: computed from
  the formula (1/k) sum_{d | k} mu(d) 2^{k/d} for k <= 128]. A bound valid
  for every k is (2^k - 2^{k/2+1})/k, which is at least 2^{k-1}/k for k >= 4
  [D]. Neither slip changes the order of magnitude.
- Incremental computation (Section 3, pp. 3-4): "f([b_1,...,b_{l+1}]) =
  (f(b_1,...,b_l) t + b_{l+1}) mod P(t)"; "f(concat(A, B)) =
  f(concat(f(A), B))" (9); "If we are given f(A) and f(B), and the length l
  of B then f(concat(A,B)) = [...] = f(f(A) * f(t^l)) + f(B)" (10), with the
  advice "to precompute f(t^l) for a suitable range of values".
- Software (Section 4, pp. 4-5): bit by bit, "two word bit shifts and two
  word xors (exclusive ors) per bit [...] This is too slow"; with k = 64 and
  four tables of 256 double words, a 32-bit word is consumed with four table
  lookups: "This algorithm requires 2 xors and 1 table indexing per byte."
- A fixed polynomial (p. 5): "the polynomial P(t) can be 'wired-in' [...]
  The idea is to assume that the virtual adversary has committed to all the
  strings that will fingerprinted by the package in the future [...] it is
  crucial to ensure that no string depends on our choice for P(t). In
  particular, we must maintain the rule that no fingerprinting of strings
  that contain fingerprints is allowed."
- Lengths (p. 6): "If fingerprints are computed piecemeal via equations (9)
  and (10) then it makes sense to make the length of the underlying string a
  part of the fingerprint, and have the fingerprint package keep track of
  it. Since x^{2^k - 1} = 1 mod P(t), it suffices to record the length
  modulo 2^k - 1 even if larger lengths might appear. (However notice that
  in this case the bound (7) becomes useless.)" The length is stored to
  apply (10), not to distinguish strings.
- Nothing is said about the word RAM or about the fingerprints of all the
  prefixes of a string.

---

## Question A. B-trees for strings

**Is the sentence accurate and correctly attributed?** Lines 560-561:

> Current B-trees for strings truncate the prefixes common to all keys of a
> node and use short separators~\cite{Bayer1977Prefix}, and they store in
> each node a few bytes of the keys after the common prefix, and hash-based
> fingerprints, so as to avoid accessing the keys~\cite{Muller2025BTrees,Chen2025FB};

| Technique | Who, according to the three papers | Mark |
|---|---|---|
| Prefix B-trees: common prefix stored once per node | Bayer and Unterauer 1977 (Graefe [10], Section 3.5; Graefe and Larson [6], p. 353; Gao et al. [15], "Head Compression") | [P] in all three; Bayer and Unterauer not read |
| Suffix truncation, short separators | Bayer and Unterauer 1977: "The idea of storing short separators instead of complete keys in internal pages is due to Bayer and Unterauer" (Graefe and Larson, p. 350); "Tail Compression" in Gao et al. | [P] |
| Normalized keys | No inventor: "a very old technique", mentioned by Singleton in 1969 (Graefe, p. 237) | [P] |
| Poor man's normalized keys (key bytes in the fixed-size slot) | Named by Graefe and Larson 2001, Section 6.1 ("We call this fixed-size prefix a poor man's normalized key"), who say that the idea "was introduced in the context of sorting [14]", i.e. AlphaSort (Nyberg et al. 1995); Graefe 2011, Section 3.6, cites [41] for the name, AlphaSort for the design and Lomet 2001 for its use in B-tree pages | [P] |
| Partial keys next to a pointer to the key | Bohannon, McIlroy, Rastogi, SIGMOD 2001, as described by Gao et al., Section 2.7 | [S] |
| Fingerprints, hints | In none of the three papers. They are the "fingerprinting leaves" and "hints" of Muller et al. and the hash tags of the FB+-tree | [S: `practical-indexes.md`, Sections 4.3-4.4] |

So: the first half of the sentence is correct and correctly attributed. The
second half is correct as a description of the 2025 designs, but a reader
takes the citation as the source of the idea of storing key bytes in the
node, which is 24 years older and has a name. "Current" is also misleading
for the first half: the techniques are those of 1977, and Gao et al. find
that they still win.

**The strongest B-tree design for strings, according to these papers.**

- The two surveys recommend, without measurements, the combination:
  normalized keys, prefix truncation (on fence keys), suffix truncation and
  poor man's normalized keys of 2 to 4 bytes in the slot array, possibly
  with interpolation search (Graefe, pp. 241 and 245; Graefe and Larson,
  p. 357). In 2001 the effectiveness of poor man's normalized keys in B-tree
  pages was listed among the open issues.
- Gao et al. measure the first two ingredients and find Head+Tail the best
  on every count they consider except space on URLs and titles; numbers in
  section 3 above. They do not measure key bytes in the slots, so the three
  papers together do not establish by experiment what a B-tree with all the
  techniques does on strings: that measurement is in Muller et al. [S].

**What this means for the experiments of `dynzfast.tex`** [I].

- In every design discussed by the three papers the keys are *inside* the
  node, and the whole point of Section 6.1 of Graefe and Larson and of the
  pkB-tree is to avoid the cache miss of following a reference to the key.
  A B-tree whose nodes contain bare pointers to the keys (the Rust
  `BTreeSet` of lines 2429-2430, and Google's B-tree of line 2353 on
  strings) uses none of the techniques: every comparison dereferences a
  pointer and compares from the first byte.
- The nearest design that was measured is PkB (pointer plus two bytes of
  partial key): on the four real datasets it is 1.5 to 4 times slower than
  Head+Tail, and on WikiTitles three times slower than the plain B-tree
  with keys in the nodes (table in section 3, read from plots).
- On URLs the plain B-tree spends 62% to 71% of the query time comparing
  keys, with 9 to 26 bytes compared on average before a difference is found
  (p. 169:22): prefix truncation removes exactly that cost, and it is why
  Head+Tail gains up to 120%.
- Hence the comparison of lines 2442-2451 is with a weak representative of
  B-trees for strings, and the phrase "the gold standard of comparison-based
  data structures" (line 2325) should not be read as applying to the
  baseline actually measured. An honest sentence is proposed as edit 4.

## Question B. The third level of "The dictionary"

**What each structure guarantees.**

| | De-amortized cuckoo hashing (Theorem 2.2) | Backyard cuckoo hashing (Theorem IV.1; from `verified-hashing.md`) | Iceberg hashing (Theorems 5 and 11) |
|---|---|---|---|
| Operations in constant worst-case time | insertions, deletions, lookups | all | insertions, queries, deletions |
| Probability | 1 - 1/p(n), for all the operations of a sequence of at most p(n), p any polynomial | the same | "w.h.p. in n" for each operation, n the current size |
| Space | 2(1 + epsilon)n + n^epsilon words | (1 + epsilon)n words, epsilon = Theta((log log n / log n)^{1/2}) | (1 + o(1))n words |
| n | a bound fixed at initialization | a bound fixed at initialization | the current number of records |
| Resizing | not considered | posed as an open problem | de-amortized, by partial expansions |
| Keys | one word of log u bits | one word | up to Theta(w) bits, reduced to Theta(log n) bits by pairwise-independent hashing |
| Hash functions | polylog(n)-wise independent | n^alpha-wise independent (Siegel; Dietzfelbinger and Woelfel) | fully random in the theorems; explicit families with O(n^alpha log n) random bits in Appendix A |
| Self-contained | yes | yes | needs a constant-time w.h.p. table for the backyard |

**Does either remove the restriction that an upper bound on the number of
keys must be known in advance?** De-amortized cuckoo hashing does not.
Iceberg hashing does (Theorem 5, and Theorem 11 with Remark 7 for explicit
hash functions), and says that the problem was first solved by Liu, Yin and
Yu in 2020 [P, p. 40:5; that paper was not read].

**Caveats for the z-fast trie** [I].

- The guarantee of iceberg hashing is per operation and with respect to the
  current n, and it is only as good as 1 - 1/poly(n). This is the same kind
  of statement as the bound 1 - 3/n^{c-1} of `sec:dict`, so "with high
  probability" in `th:boundslong` keeps its meaning; a query performs
  O(lg |x|) = O(w) accesses, and the union bound over them costs a factor w,
  which is absorbed if w is polynomial in n (the paper's analysis of
  signatures makes the same step at line 1440, where the factor w is
  compensated by the 1/w gained from hashing words).
- The keys of the z-map take a constant number of words (signatures have
  (c+1)w + O(1) bits): this is within "keys as large as Theta(w) bits"; the
  reduction to Theta(log n) bits adds a collision probability of 1/poly(n).
- The result is a composition of several components (a backyard table,
  rebuilds when log n doubles, random rebuilds for the explicit hash
  functions) and the paper does not state a single theorem "there is a
  dictionary with explicit hash functions, no bound on n, constant time
  w.h.p."; Theorem 11 with Remarks 5 and 7 is the nearest. I would cite it
  as showing that the bound N is not needed, not rely on its constants.
- The space bound "at most 3n buckets" of the theorems is unaffected.

**Proposed wording**: edit 2 below.

## Question C. Rabin fingerprints, CRCs and the analysis of `sec:dict`

**What Rabin's fingerprints are** [P]. A polynomial P of degree k is chosen
uniformly at random among the irreducible polynomials of that degree with
coefficients in Z_2 (Rabin: k prime, via a random element of GF(2^k)); the
fingerprint of a bit string is the residue modulo P of the polynomial whose
coefficients are the bits of the string, the first bit being the leading
coefficient; it has k bits. Broder adds a 1 in front of the string.

**The collision bound.**

- Rabin, Theorem 3 [P]: for a pattern of n bits and a text of m bits, k >
  log_2(nm/epsilon) gives error probability smaller than epsilon. The proof
  gives, for a single pair [I]: two distinct polynomials of degree less than
  D have the same residue for at most D/k irreducible polynomials of degree
  k, that is, with probability at most D/(2^k - 2) when k is prime.
- Broder, formula (7) [P]: for a set of n distinct strings (each starting
  with 1) the probability that some pair collides is less than
  (deg Q / k) * k/(2^k - 2^{k/2}), where deg Q <= n sum_{A in S} |A|. With
  the corrected count of irreducible polynomials (section 7), for every k >=
  4: at most 2 deg Q / 2^k [D].
- For the paper's use [D]: let the signature of x be the residue of
  t^{|x|} + x_1 t^{|x|-1} + ... + x_{|x|}. Two distinct strings of at most L
  bits give distinct polynomials of degree at most L (if the lengths differ,
  so do the degrees), so they have the same signature with probability at
  most (L/k) / (2^{k-1}/k) = 2L/2^k, and at most L/(2^k - 2) if k is prime.
  The paper's bound is ceil(L/w)/p <= 2L/(wp): Rabin's bound is the same
  with 2^k in place of wp, because the degree is counted in bits instead of
  words.

**How lengths and leading zeroes are handled.** Rabin: not at all (equal
lengths). Broder: by the leading 1, a "technicality" of the implementation;
the length is carried along only to concatenate fingerprints. With the
leading 1 the signature is exact at the bit level: there is no padding of
the last word, so the length need not be appended as in line 1418 [D]. (A
CRC as standardized initializes its register to all ones, which has the same
purpose [G].)

**Prefix fingerprints in constant time after linear preprocessing?** Not
stated by either paper. My analysis [D]:

- Preprocessing: F_0 = 1 and F_{i+1} = (F_i t^w + x_i) mod P give the
  fingerprints of the prefixes of iw bits; the fingerprint of the prefix of
  f bits is (F_{floor(f/w)} t^r + y) mod P, where r = f mod w and y is given
  by the next r bits. Both steps are of the same kind: multiply a residue by
  t^r, r <= w, and reduce.
- With the instructions of the paper's RAM (addition, subtraction, Boolean
  operations, shifts, multiplication) this step costs r operations by
  Rabin's method (one bit at a time) and ceil(r/b) operations with a table
  of 2^b residues (Broder's method with b = 8; in general, (F t^b) mod P is
  the low part of F shifted by b, xored with the table entry indexed by the
  b leading coefficients of F). With b = (lg n)/2 the table has sqrt(n)
  entries and a word costs O(w/lg n) operations: the preprocessing takes
  O(|x|/lg n) time, which is O(|x|/w) only if w = O(lg n). This is the same
  proviso under which, at line 319, a table replaces multiplication.
- In constant time for every w, the step needs the product of two
  polynomials over Z_2 packed in words (carry-less multiplication): two such
  products suffice, by Barrett reduction, which is exact for polynomials.
  This instruction exists on current processors (and the `crc32` instruction
  computes the step directly for one fixed polynomial) [G], but it is not in
  the model of the paper, it is not in AC0 (one output bit of the product of
  x and the word of all ones is the parity of x), and I know of no
  simulation in constant time by integer multiplication: spacing the
  coefficients so that carries do not interfere costs a factor
  polylogarithmic in w.
- Choosing P is not a constant-time operation either: Rabin's method costs
  about k^2 operations on k-bit vectors, once. The paper's scheme has no
  such step, as p is fixed and only the point a is random.

**Would an analysis based on Rabin fingerprints go through with the same
structure?** [D]

1. *Collisions and union bound: yes, verbatim.* A probed prefix is bad with
   probability at most 2nL/2^k; with 2^k >= 2wLn^c this is at most
   1/(wn^{c-1}), and no probed prefix is bad with probability at least 1 -
   3/n^{c-1}, as at lines 1435-1442. Signatures have c lg n + lg L + lg w +
   O(1) bits: lg w more than now, but without the lower bound of w bits that
   p > 2^w imposes.
2. *Adaptivity: yes.* The probed prefixes are determined by x and S in the
   execution with an exact z-map, and P is independent of both.
3. *Different lengths: yes*, by the leading 1.
4. *Duplicate signatures, verification, fallback: unchanged*, they do not
   depend on the function.
5. *Time: no, not in the stated model.* The bound O(|x|/w + lg |x|) of
   `th:boundslong` would hold only for w = O(lg n), or in a RAM with
   carry-less multiplication. This is the real reason to keep polynomials
   over Z_p in the paper.

A variant that keeps the paper's proof word for word is evaluation at a
random point of a polynomial with coefficients in GF(2^k), k >= w, for a
fixed irreducible P: lines 1423-1426 use only that Z_p is a field. It has the
same computational problem (multiplication in GF(2^k)) and is not a CRC.

**CRC-based implementations.** A CRC is the residue modulo a *fixed*
polynomial [G]; Wormhole uses CRC-32c and keeps 16 bits of it as a tag
[S: `verified-strings.md`, from Section 3.1 of the Wormhole paper]. Then:

- No bound of the kind above holds for a fixed polynomial and arbitrary
  keys: Broder's justification of a "wired-in" polynomial requires that "no
  string depends on our choice for P(t)", and assumes that the polynomial
  was chosen at random once.
- With k = 32 the bound 2nL/2^k on a bad prefix is void as soon as nL >=
  2^31, and with 16-bit tags false positives are the norm: they are handled
  by the final verification, as in `sec:dict`.
- The polynomial of CRC-32C, t^32 + ... + 1 = 0x11EDC6F41, is not even
  irreducible: it has 18 terms, hence it is divisible by t + 1 [D: checked
  by computation; that this is the polynomial used by Wormhole is G]. The
  polynomial of the IEEE CRC-32, 0x104C11DB7, is irreducible [D].

**The paper's own implementations are not CRC-based** [P, `dynzfast.tex`]:
the C code uses "a hash function derived from Austin Appleby's MurmurHash2"
(line 2334) and the Rust code "xxh3" (line 2415). For these nothing is
proved, and Rabin's and Broder's papers do not help: the `\todo` at line
1443 states a fact and stays true.

**Is a remark justified?** Yes, a short one, for three reasons: the reader
who knows CRC-based implementations (Wormhole is discussed in
`sec:related`) will ask why the analysis uses a different family; the answer
(the model) is not obvious; and the remark makes explicit that only the
running time depends on the signatures. Proposed text: edit 3.

---

## Proposed edits to dynzfast.tex

No file was modified. All new text is ASCII and uses the macros of the file.

### Edit 1. Lines 560-562 (paragraph "Ordered indices for main memory")

Old:

```latex
Current B-trees for strings truncate the prefixes common to all keys of a node 
and use short separators~\cite{Bayer1977Prefix}, and they store in each node a few bytes of the keys after the common prefix, and hash-based fingerprints, so as to avoid accessing the keys~\cite{Muller2025BTrees,Chen2025FB}; 
learned indices for strings have been proposed, too~\cite{Spector2021Bounding,Yang2024LITS}. 
```

New:

```latex
B-trees for strings are still based on the \emph{prefix B-trees} of Bayer and Unterauer~\cite{Bayer1977Prefix}, which store 
only once the prefix common to all keys of a node, and use as separators the shortest strings separating the keys of adjacent
leaves (see~\cite[Section~3.5]{Graefe2011Modern}): in a recent comparison of seven techniques for compressing the keys of a B-tree, 
the combination of these two is reported to be the fastest, answering point queries from $25$\% to $120$\% faster 
than a B-tree storing whole keys~\cite{Gao2024Revisiting}. Moreover, a few bytes of each key following the common prefix can be stored in the 
fixed-size entries of a node, so that most comparisons are decided without accessing the key: these are the ``poor man's normalized keys'' 
of Graefe and Larson~\cite{Graefe2001Btree}, who trace the idea back to cache-conscious sorting. Recent designs for main memory
add hash-based fingerprints, which speed up point queries~\cite{Muller2025BTrees,Chen2025FB}. 
Learned indices for strings have been proposed, too~\cite{Spector2021Bounding,Yang2024LITS}. 
```

Reason: the attribution of the first half is confirmed; the key bytes in the
node have an origin and a name (Graefe and Larson, Sections 2 and 6.1;
Graefe, Section 3.6) that the sentence does not give; "Current" hides the
fact that the winning techniques are those of 1977; the figures of Gao et
al. are the only measurements on strings in the three papers. "is reported
to be" because on one dataset the plot shows Head alone slightly ahead.
Optional addition after "cache-conscious sorting", to be made only if the
authors accept a citation through Gao et al. (the paper was not read): "(the
partial keys of~\cite{Bohannon2001Main} are a similar device for nodes
containing pointers to the keys)"; the entry is in `practical-indexes.bib`.

### Edit 2. Lines 384-387 (third item of "The dictionary")

Old:

```latex
  \item If an upper bound $N$ on $n$ is known in advance, there are dictionaries using $O(N)$ words in which, for every
  sequence of $N^{O(1)}$ operations, with probability $1-1/N^{O(1)}$ every operation takes constant time in the worst
  case~\cite{Arbitman2010Backyard} (the result is stated for keys of one word): with such a dictionary all bounds of 
  Theorems~\ref{th:bounds} and~\ref{th:boundslong}, including those for updates, hold with high probability.
```

New:

```latex
  \item There are dictionaries in which every operation takes constant time in the worst case with high probability.
  If an upper bound $N$ on $n$ is known in advance, de-amortized Cuckoo hashing~\cite{Arbitman2009Deamortized} and
  backyard Cuckoo hashing~\cite{Arbitman2010Backyard} use $O(N)$ words, and for every
  sequence of $N^{O(1)}$ operations, with probability $1-1/N^{O(1)}$ every operation takes constant time
  (the results are stated for keys of one word). The bound $N$ is not necessary: iceberg hashing~\cite{Bender2023Iceberg} 
  uses space linear in the current number $n$ of keys, resizing the table incrementally, and each operation takes constant time 
  with probability $1-1/n^{O(1)}$. With such dictionaries all bounds of 
  Theorems~\ref{th:bounds} and~\ref{th:boundslong}, including those for updates, hold with high probability.
```

Reason: Theorem 2.2 of the ICALP 2009 paper satisfies the statement already
in the text; Theorem 5 of the iceberg paper (with Theorem 11 for explicit
hash functions) removes the proviso, which the text presents as necessary.
"Space linear" rather than the printed (1 + o(1))n words because the keys of
the z-map take more than one word. "$1-1/n^{O(1)}$" is my reading of "w.h.p.
in n", which the paper does not define. If the authors prefer not to depend
on a result assembled from several theorems and remarks, the minimal edit is
to add `Arbitman2009Deamortized` to the existing citation and append "(the
bound $N$ can be dispensed with~\cite{Bender2023Iceberg})".

### Edit 3. After line 1442, before the `\todo` of line 1443 (in `sec:dict`)

Old (line 1442, unchanged, given to locate the insertion):

```latex
has its signature, and a prefix that is not a handle is not, as no handle has its signature. In particular, there are no false positives.
```

New text to insert after it:

```latex

The argument uses just two properties of the signatures: two distinct strings collide with small probability over a random choice that is
independent of the strings, and the signature of a prefix can be computed in constant time from preprocessed values. 
Rabin's fingerprints~\cite{Rabin1981Fingerprinting,Broder1993Some} have the first property: one chooses uniformly at random an irreducible polynomial $P$ of 
degree $k$ with coefficients in $\mathbf Z_2$, and the fingerprint of a string is the remainder of the division by $P$ of the polynomial whose 
coefficients are the bits of the string, preceded by a one, so that strings of different lengths are associated with different polynomials. 
As for $k\geq 4$ there are at least $2^{k-1}/k$ such $P$, and at most $L/k$ of them divide a nonzero polynomial of degree at most $L$, two distinct strings of 
length at most $L$ have the same fingerprint with probability at most $2L/2^k$, and the argument above goes through for $2^k\geq 2wLn^c$.
The fingerprint of a string extended by one bit is obtained by a shift and an exclusive or~\cite{Rabin1981Fingerprinting}, but
we know of no way of extending it by a word in constant time in our model: implementations use tables~\cite{Broder1993Some}, 
or an instruction multiplying polynomials with coefficients in $\mathbf Z_2$. This is why we use polynomials over $\mathbf Z_p$.
The cyclic redundancy checks used as signatures by some implementations of similar structures, such as Wormhole~\cite{WNJWormhole}, are computed 
in the same way, but the polynomial is fixed and has degree $32$, so no such bound is available. 
In any case, the function computing the signatures affects only the running time, as the result of the search is always verified.
```

Reason: see question C. The count $2^{k-1}/k$ and the bound $2L/2^k$ are my
derivation from the standard formula for the number of irreducible
polynomials (Rabin proves (2^k - 2)/k for prime k; Broder's count is
slightly wrong for even k): the authors should check them, or restrict the
remark to prime k and write "$(2^k-2)/k$" and "$L/(2^k-2)$", which is
exactly what Rabin proves. The statements about tables and about the
instruction are from Broder (tables) and from general knowledge
(instruction). The `\todo` of line 1443 should stay, or become a sentence
at the end of the remark: "The same is true of the hash functions used by
our implementations." (MurmurHash2 at line 2334, xxh3 at line 2415.)

### Edit 4 (optional). Line 2430 (the Rust baseline)

Old:

```latex
nodes contain up to $11$ pointers to keys), and, in the static case, with a binary search on the sorted keys, 
```

New:

```latex
nodes contain up to $11$ pointers to keys: it uses none of the techniques for strings discussed in Section~\ref{sec:related}), and, in the static case, with a binary search on the sorted keys, 
```

Reason: after edit 1 the related work says that prefix truncation alone
gives up to 120% on point queries and that key bytes in the node avoid the
dereference; the reader should be told that the baseline has neither. The
same holds for Google's B-tree at line 2353 when its keys are strings. No
edit is proposed for line 2325 ("the gold standard of comparison-based data
structures"), but the authors may want to qualify it.

### No edit

- Line 579 ("Karp--Rabin fingerprints"): consistent with Rabin's own
  distinction between the two families.
- Lines 1411-1426: nothing in the two papers on fingerprints bears on the
  correctness of the present analysis.

## Entries for biblio.bib

None of the seven is in `biblio.bib`. The first six below are the entries of
`practical-indexes.bib` and `theory.bib`, checked on the PDFs, with the
fields that could be added; the seventh is new. All ASCII.

```bibtex
@article{Graefe2011Modern,
  author  = {Goetz Graefe},
  title   = {Modern {B}-Tree Techniques},
  journal = {Foundations and Trends in Databases},
  volume  = {3},
  number  = {4},
  pages   = {203--402},
  year    = {2011},
  doi     = {10.1561/1900000028}
}

@inproceedings{Graefe2001Btree,
  author    = {Goetz Graefe and Per-{\AA}ke Larson},
  title     = {{B}-Tree Indexes and {CPU} Caches},
  booktitle = {Proceedings of the 17th International Conference on Data Engineering ({ICDE})},
  year      = {2001},
  pages     = {349--358},
  publisher = {IEEE Computer Society},
  doi       = {10.1109/ICDE.2001.914847}
}

@article{Gao2024Revisiting,
  author  = {Chuqing Gao and Shreya Ballijepalli and Jianguo Wang},
  title   = {Revisiting {B}-tree Compression: An Experimental Study},
  journal = {Proceedings of the ACM on Management of Data},
  volume  = {2},
  number  = {3},
  pages   = {169:1--169:25},
  year    = {2024},
  doi     = {10.1145/3654972}
}

@inproceedings{Arbitman2009Deamortized,
  author    = {Arbitman, Yuriy and Naor, Moni and Segev, Gil},
  title     = {De-amortized cuckoo hashing: Provable worst-case performance and experimental results},
  booktitle = {Automata, Languages and Programming, 36th International Colloquium, ICALP 2009, Part I},
  series    = {Lecture Notes in Computer Science},
  volume    = {5555},
  pages     = {107--118},
  publisher = {Springer},
  year      = {2009},
  doi       = {10.1007/978-3-642-02927-1_11}
}

@article{Bender2023Iceberg,
  author  = {Bender, Michael A. and Conway, Alex and Farach-Colton, Mart{\'\i}n and Kuszmaul, William and Tagliavini, Guido},
  title   = {Iceberg hashing: Optimizing many hash-table criteria at once},
  journal = {Journal of the ACM},
  volume  = {70},
  number  = {6},
  pages   = {40:1--40:51},
  year    = {2023},
  doi     = {10.1145/3625817}
}

@incollection{Broder1993Some,
  author    = {Broder, Andrei Z.},
  title     = {Some applications of {R}abin's fingerprinting method},
  booktitle = {Sequences {II}: Methods in Communications, Security, and Computer Science},
  editor    = {Capocelli, Renato and De Santis, Alfredo and Vaccaro, Ugo},
  pages     = {143--152},
  publisher = {Springer},
  year      = {1993},
  doi       = {10.1007/978-1-4613-9323-8_11}
}

@techreport{Rabin1981Fingerprinting,
  author      = {Rabin, Michael O.},
  title       = {Fingerprinting by random polynomials},
  institution = {Center for Research in Computing Technology, Harvard University},
  number      = {TR-15-81},
  year        = {1981}
}
```

What is not verified on the PDFs:

- `Graefe2001Btree`: the name of the conference and the DOI (the pages print
  only "1063-6382/01 [...] (c) 2001 IEEE" and the page numbers).
- `Graefe2011Modern`: the first page dates the volume 2010 and the copyright
  2011.
- `Broder1993Some`: the pages 143-152 and the DOI (the PDF is an author's
  version); the first names of the editors, which the PDF gives as initials
  (R. Capocelli, A. De Santis, U. Vaccaro), are from general knowledge.
- `Rabin1981Fingerprinting`: institution and year, which are those of
  Broder's reference [4]; the report itself prints only "TR-15-81".
- `Arbitman2009Deamortized`, `Bender2023Iceberg`: the DOI of the first is
  not printed on its pages; every other field is.

Also settled by these PDFs, for `theory.bib`: `Dietzfelbinger1990New` is
ICALP 1990, LNCS 443, pp. 6-19, Springer 1990 (reference [5] of Arbitman et
al. 2009 and [16] of Bender et al.).
