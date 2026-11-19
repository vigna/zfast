# Related work: static and succinct string dictionaries, and the uses of z-fast tries

Survey for the related-work section of *Z-Fast Tries* (Belazzougui, Boldi, Vigna).
Written 2026-11-19. Companion file: `static-and-uses.bib` (keys cited below in
`[Key]` form).

## 0. Scope, method and conventions

**Scope.** Static and compressed string dictionaries and tries; monotone minimal
perfect hashing (MMPHF); weak prefix search; z-fast tries inside text indexes and
other structures; string range filters; external-memory and learned string
indexes. Dynamic in-memory indexes (Wormhole, HOT, ART, Masstree, Cuckoo Trie)
are outside this report, except where a paper measures a z-fast trie.

**What was already known.** `review-2026-11/reports/F-literature-and-applications.md`
and sections 4 and 6 of `review-2026-11/REVIEW.md`. This report verifies, corrects
(section 9) and extends them; it does not repeat their tables on FM-indexes, read
mappers or in-memory ordered indexes.

**How the list of users of z-fast tries was built.** Citing-work lists were pulled
from OpenAlex and Semantic Scholar (with citation contexts) for six papers:
SODA 2009 [Belazzougui2009Monotone], JEA 2011 [Belazzougui2011Theory], ESA 2010
[Belazzougui2010Fast], SPIRE 2010 [Belazzougui2010Dynamic], FUN 2018
[Boldi2018Kings], SEA 2024 [Boldi2024Engineering]. Counts on 2026-11-19:
OpenAlex 107 / 50 / 63 / 36 / 2 / 0, Semantic Scholar 60 (ESA) / 35 (SPIRE) /
3 (FUN) / 0 (SEA). Google Scholar "cited by" pages were not reachable by script
and were not used, so the lists are a lower bound. For every work classified
below as a *user*, I downloaded the arXiv or author version, converted it to
text and read the passage where the z-fast trie (or weak prefix search) is used.

**Bibliographic verification.** DBLP was not reachable (its search API and site
answered with a bot-protection page on every attempt). Every entry in the `.bib`
file was therefore checked against Crossref (publisher-deposited metadata) or
DataCite (for LIPIcs), or, for arXiv-only items, against the first page of the
PDF. Entries I could not check that way are flagged in the `.bib` file with a
`note`.

**Provenance tags.**

- **[read]**: I read the relevant part of the full text (local extract or
  downloaded PDF).
- **[abstract]**: I saw only the abstract, publisher metadata or a citation
  context returned by Semantic Scholar.
- **[inferred]**: my own inference, not a statement of the source.
- **[repo]**: read from the repository or the GitHub API on 2026-11-19.

Numbers are copied from the sources; they come from different machines and
workloads and are not comparable across papers unless stated.

---

## 1. The z-fast lineage, as the literature cites it

Different communities cite different papers for "the z-fast trie", and this
matters for what the journal paper should claim.

| Paper | What citing papers take from it |
|---|---|
| SODA 2009 [Belazzougui2009Monotone] | MMPHFs; the name "z-fast trie"; fat binary search (Kempa and Kosolobov cite it for "the so called fat binary search in z-tries") |
| ALENEX 2009 / JEA 2011 [Belazzougui2009Theory, Belazzougui2011Theory] | Practical MMPHFs, hollow tries, the Sux4J implementations used as baselines |
| ESA 2010 / arXiv 2018 [Belazzougui2010Fast, Belazzougui2018Fast] | Weak prefix search; the *hollow z-fast prefix trie*; this is the paper that the compressed-indexing literature cites as "the z-fast trie" (usually "Theorem 5" or "Appendix H.2/H.3") |
| SPIRE 2010 [Belazzougui2010Dynamic] | The dynamic structure; cited by trie papers (packed c-tries, c-trie++, DynPDT, zip-tries, C2), by streaming papers and by LZ-parsing papers |
| arXiv 2012 [Belazzougui2012Predecessor] | Distance-sensitive predecessor search built on z-fast tries |
| FUN 2018 [Boldi2018Kings] | Non-binary alphabets, zuffix arrays; cited by the suffixient-array and STPD papers as the reference for the z-fast lemma they use |

Bounds of the static results, from the ESA 2010 full version **[read]**
(n strings, average length ℓ, query p, word size w):

- Range location: O(|x|/w) time with O(n log ℓ) bits, or O(|x|/w + log |x|) time
  with O(n log log ℓ) bits (Theorem 4).
- Weak prefix search: HT(S) + O(n log log ℓ) bits with O(|p|/w + log |p|) time,
  or O(n ℓ^{1/c} log ℓ) bits with O(|p|/w) time for any constant c
  (Theorems 5 and 6); HT(S) is the size of the hollow trie. A matching space
  lower bound holds even for approximate prefix counting (Theorem 7).
- All bounds translate to the cache-oblivious model with |p|/B in place of |p|/w.

**Consequence for the paper [inferred].** The most frequent use of z-fast tries
in the last fifteen years is not the dynamic structure of SPIRE 2010 but a
*static, linear-space, word-based* variant: a compacted trie over O(k) strings
whose nodes are found by fat binary search on Karp–Rabin fingerprints of prefixes
of the pattern, used with a verification step because the search is "weak". The
related-work section should say this explicitly and cite Gagie et al., Bille et
al. and Christiansen et al. (section 5.2) as the works that established the
pattern.

---

## 2. Static string dictionaries

### 2.1 Front coding and rear coding with sampled binary search

**Mechanism.** Sort the strings, store every k-th string in full (a block head)
and each other string as (length of the prefix shared with its predecessor,
remaining suffix). *Rear coding* stores the number of characters to drop from the
end of the predecessor instead. Lookup is a binary search on block heads followed
by a sequential scan of one block.

**Costs.** O(log(n/k)) string comparisons, each a potential cache miss, plus a
scan of at most k strings. Space is the front-coded text plus one pointer per
block.

**Relation to z-fast tries.** This is the structure the new static trie is meant
to index: a z-fast trie over the block heads replaces the binary search by
O(log |x|) hash probes whose number does not depend on n **[inferred]**; see
REVIEW.md section 6, row "`RearCodedList` in `sux-rs`".

**Engineering references.** The classical reference for front coding in this
setting is the WebGraph framework (cited as such by Martínez-Prieto et al.,
**[read]**). The reference experimental study is the next item.

### 2.2 Martínez-Prieto, Brisaboa, Cánovas, Claude, Navarro: *Practical compressed string dictionaries* [MartinezPrieto2016Practical] (preliminary version [Brisaboa2011Compressed]) **[read: author preprint]**

- **Mechanism.** A systematic comparison of compressed hashing (HashHF, HashDAC,
  with Huffman or Re-Pair compressed strings), Plain Front Coding (PFC),
  Hu-Tucker Front Coding (HTFC, block contents further compressed with
  Hu-Tucker, Huffman or Re-Pair), Re-Pair plus DAC, and full-text indexes
  (FM-index, XBW) for substring queries.
- **Results.** Compressed dictionaries take about 5%, 10% or 30% of the original
  size depending on shared prefixes and repetitiveness. PFC: 8%–55% of the
  space (more than 80% on Literals), locate in 1–2 µs, extract in 0.2–0.4 µs.
  HTFC, "the dominant technique": 5%–35% of the space, locate in 1–6 µs,
  extract in 0.4–2 µs. HashDAC-rp: 12%–65%, locate in 0.5–3.2 µs. HashB-huff:
  60%–100%, locate in 0.5–2 µs. XBW-rrr is the smallest (3%–20%) but takes
  20–200 µs.
- **Datasets (Table 1).** Geographic names (5.5M strings, 81.6 MB), Words
  (25.7M, 257 MB), Word sequences en/sp (36.7M / 39.2M, about 1 GB each), URIs
  (26.9M, 1312 MB, Uniprot RDF), URLs (18.5M, 1372 MB, uk-2002 from WebGraph),
  Literals (27.6M, 1591 MB), DNA (9.2M strings of average length 13, 114 MB).
- **Hardware.** Lookup and extraction on an Intel Core i7-3820 at 3.6 GHz,
  16 GB RAM.
- **Code.** `libCSD`, <https://github.com/migumar2/libCSD>, LGPL-2.1, last push
  2022 **[repo]**.
- **Cites z-fast tries?** No (no match for "z-fast" or "weak prefix" in the
  text).
- **Follow-up.** *Improved compressed string dictionaries* [Brisaboa2019Improved]
  **[read: arXiv 1911.08372, skimmed]**: no mention of z-fast tries.

### 2.3 Grossi and Ottaviano: path-decomposed tries (PDT) [Grossi2015Fast] (ALENEX version [Grossi2012Fast]) **[read: arXiv 1111.5220]**

- **Mechanism.** The compacted trie is replaced by its (centroid or
  lexicographic) path decomposition: each root-to-leaf path becomes one node, so
  the height is O(log n) for centroid paths. The tree is stored succinctly
  (DFUDS / balanced parentheses) and labels are compressed with a Re-Pair-like
  scheme.
- **Costs.** A lookup visits O(log n) nodes with centroid decomposition instead
  of O(|x|) trie nodes.
- **Measured (third party, C2 paper, section 2.8).** 356–1138 ns per lookup at
  13.0%–33.7% of the data size on six datasets; it is the most compact structure
  in that comparison on three of the six (dna, xml, log), and Marisa with one
  recursion level is on the other three.
- **Code.** <https://github.com/ot/path_decomposed_tries>. The licence file is a
  *Microsoft Research License Agreement, non-commercial use only* **[repo]**.
  Last push 2023.
- **Relation.** The paper cites the MMPHF papers (five mentions of monotone
  hashing; hollow tries are a baseline for its monotone hash variant) but not
  the dynamic z-fast trie. It is the strongest *space* competitor.

### 2.4 Marisa (Yata)

- **Mechanism (from the C2 paper's description and from memory of the project
  documentation).** A recursively nested Patricia trie stored in LOUDS: the
  labels of unary paths are stored in a second trie, and so on for a
  configurable number of recursion levels, with a tail container at the end.
- **Publication.** No paper in English. The C2 paper cites "S. Yata, *Dictionary
  compression by nesting prefix/patricia tries*, Proc. 17th Meeting of the
  Association for Natural Language [Processing], 2011" [Yata2011Dictionary]; I
  took this reference from C2's bibliography and could not check it at the
  source.
- **Measured (C2 paper).** 272–1098 ns per lookup at 32.4%–76.4% of the data with
  no recursion; with one recursion level 301–1538 ns at 22.0%–34.3%.
- **Code.** <https://github.com/s-yata/marisa-trie>, BSD-2-Clause or
  LGPL-2.1-or-later, actively maintained (last push 2026-07) **[repo]**.
- **Cites z-fast tries?** Not applicable (software).

### 2.5 Double-array tries and XCDAT (Kanda et al.)

- **Mechanism.** A double array stores trie transitions in two integer arrays
  (BASE/CHECK), so a child is found with one addition and one comparison.
  Kanda, Morita and Fuketa compress the arrays and move tails to a separate
  compressed store [Kanda2017Compressed]; XCDAT is the resulting library.
- **Status.** **[abstract]** only: I verified the bibliographic data
  (Knowl. Inf. Syst. 51(3), 2017) but could not read the article (see the last
  section). Mechanism above is from the abstract and the repository description.
- **Code.** <https://github.com/kampersanda/xcdat>, MIT, last push 2024
  **[repo]**.
- **Related, read:** *Dynamic path-decomposed tries* [Kanda2020Dynamic]
  **[read: arXiv 1906.06015]**. It benchmarks a z-fast trie (section 2.12).

### 2.6 FST and SuRF (Zhang et al.) [Zhang2018SuRF, Zhang2020Succinct] **[read: TODS extract]**

- **Mechanism.** The Fast Succinct Trie encodes the upper levels with
  LOUDS-Dense (bitmaps of 256 bits per node) and the lower levels with
  LOUDS-Sparse (one byte label plus two bits per edge). SuRF truncates the trie
  at the shortest distinguishing prefixes and optionally adds hashed or real
  suffix bits, turning it into a range filter.
- **Costs.** About 10 bits per trie node; SuRF-Base uses 10 bits per key on
  random 64-bit integers and 14 on e-mail addresses.
- **Measured (C2 paper).** FST as an exact dictionary: 403–4003 ns per lookup at
  35.5%–74.3% of the data.
- **Code.** <https://github.com/efficient/SuRF> (the `LICENSE` file is the
  Apache License 2.0; last push 2022) and the dictionary variant used by CoCo-trie and C2,
  <https://github.com/kampersanda/fast_succinct_trie>, Apache-2.0 **[repo]**.
- **Cites z-fast tries?** No match in the TODS text.

### 2.7 CoCo-trie (Boffa, Ferragina, Tosoni, Vinciguerra) [Boffa2022Compressed, Boffa2024CoCo] **[read: journal version]**

- **Mechanism.** Subtries of ℓ levels are collapsed into macro-nodes whose
  branching strings are encoded as integers with the cheapest of several codes
  (Elias–Fano among them); ℓ and the code are chosen per node by an
  optimiser that minimises total space. The macro-node tree is stored in LOUDS
  (using `sux::SimpleSelectZero` from the C++ Sux library).
- **Measured.** On url, dna, tpcds-id, trec-terms, protein and xml it dominates
  FST, PDT and CART on three datasets and is on the Pareto front on the other
  three, at the price of "on average, 6× more construction time". The dynamic
  baselines ART and
  c-trie++ use 2.2×–130× its space. Intel Xeon Platinum 8260M at 2.30 GHz,
  g++ 11.1. Queries are half hits and half misses generated to share the average
  LCP.
- **Caveat.** Datasets are preprocessed to keep only the shortest distinguishing
  prefix of each string; the C2 paper shows that on full strings the public
  implementation becomes slow and large.
- **Code.** <https://github.com/aboffa/CoCo-trie>, GPL-3.0, maintained (last push
  2026-11) **[repo]**.
- **Cites z-fast tries?** Yes, [Belazzougui2010Dynamic], only to describe
  c-trie++ as "a hybrid between the packed c-trie and the z-fast trie". It does
  not benchmark a z-fast trie.

### 2.8 C2 (Zhang, Zhao, Xu, 2026) [Zhang2026C2] **[read: arXiv 2606.16104v1]**

- **Publication.** arXiv 2606.16104, 15 June 2026; the page header reads
  "EDBT '27, 6–9 April 2027, Lille (France)", and Semantic Scholar lists the DBLP
  key `conf/edbt/ZhangZX27`. Treat it as accepted at EDBT 2027 but not yet in the
  proceedings.
- **Mechanism.** Two changes applied to FST, CoCo-trie and Marisa: (C1) a
  bitvector layout that interleaves rank metadata with the bits so that a rank
  costs one cache line instead of two; (C2) unary paths longer than a link are
  moved to a tail container compressed with FSST [Boncz2020FSST], with an
  adaptive choice of the recursion depth.
- **Measured (Table 6; AMD EPYC 9555 at 3.2 GHz, one thread, g++ 10.3 -O3; all
  keys of the dataset queried in random order, so positive queries only).**
  Query time in ns and size in % of the data:

| Structure | words | url | dna | xml | wiki | log |
|---|---|---|---|---|---|---|
| C2-Marisa | 182 / 37.5% | 308 / 30.9% | 439 / 41.0% | 500 / 27.6% | 732 / 32.9% | 829 / 20.7% |
| C2-FST | 227 / 40.4% | 404 / 36.0% | 561 / 43.8% | 842 / 28.8% | 1258 / 37.7% | 2154 / 24.4% |
| Marisa | 272 / 33.3% | 460 / 35.3% | 663 / 76.4% | 669 / 40.3% | 991 / 32.4% | 1098 / 37.3% |
| PDT | 356 / 33.7% | 560 / 26.2% | 795 / 28.4% | 841 / 19.7% | 1022 / 30.1% | 1138 / 13.0% |
| FST | 403 / 45.5% | 610 / 35.5% | 844 / 74.3% | 1541 / 39.2% | 1424 / 37.9% | 4003 / 39.5% |
| ART | 132 / 346.8% | 347 / 156.2% | 533 / 115.0% | 728 / 59.1% | 713 / 165.7% | 1611 / 27.6% |
| c-trie++ | 217 / 1540.1% | 495 / 747.0% | 683 / 513.5% | 915 / 290.6% | 716 / 788.5% | 1507 / 157.1% |
| z-fast trie | 541 / 1084.5% | 1158 / 586.6% | 1394 / 400.2% | 2016 / 223.9% | 1192 / 546.6% | 2838 / 92.7% |

- **Datasets (Table 3).** words (467K English words, 5 MB), url (1.77M "UK
  private domains", 37 MB, average length 21), dna (3.32M 31-mers, 101 MB), xml
  (2.15M lines of the dblp dump from Pizza&Chili, 117 MB), log (4.45M web-server
  log lines from Kaggle, 586 MB, average length 137), wiki (17.5M Wikipedia
  titles from the 2024-11-20 dump, 359 MB).
- **Cache misses.** 56–82 last-level misses per query on wiki for the succinct
  tries, 23 for C-ART (Table 1).
- **Code.** The paper gives <https://github.com/alexztc/C2>; the GitHub API
  redirects it to `gt-alg-kit/C2`; licence not recognised by GitHub **[repo]**.
- **Cites z-fast tries?** Yes, [Belazzougui2010Dynamic] and
  [Belazzougui2011Theory]. It calls c-trie++ and the z-fast trie "two
  state-of-the-art dynamic tries" and does not say which implementation it ran
  (presumably the "ZFT" port distributed with c-trie++ **[inferred]**).
- **Important for the paper.** These are numbers for a *dynamic, bit-level,
  pointer-based* port measured as a *static dictionary* and with space counted
  against succinct tries. They say nothing about a static byte-level z-fast
  index over a sorted list.

### 2.9 Ferragina, Rotundo, Vinciguerra: two-level massive string dictionaries [Ferragina2023Engineering, Ferragina2025Two] **[abstract]**

- **Mechanism.** Storage and indexing are separated: the sorted strings are
  compressed in (rear-coded) blocks kept on disk, and a succinct Patricia trie
  over the first string of each block is kept in memory.
- **Result (abstract).** 3.5 billion strings (273 GB) indexed with less than
  200 MB of RAM and 83 GB of compressed disk space; "state-of-the-art compressed
  string dictionaries (such as FST, PDT, CoCo-trie) do not provide significant
  benefits if used in an indexing setting compared to Patricia tries".
- **Code.** <https://github.com/mariagiovannarotundo/Two-level-indexing>,
  GPL-3.0, last push 2026-06 **[repo]**.
- **Relation [inferred].** This is exactly the architecture of the new static
  trie when it indexes every k-th key of a rear-coded list, with a Patricia
  (blind) trie in place of the z-fast trie. A blind search needs one string
  access to verify, like a weak prefix search; the difference is that the
  Patricia trie is walked node by node. It is the closest published competitor
  to "z-fast trie over block heads" and must be in the comparison. I could not
  read the full text to check whether it cites z-fast tries or weak prefix
  search (the SPIRE version is not in the citing lists I retrieved).

### 2.10 Learned string indexes

- **RSS** (Spector, Kipf, Vaidya, Wang, Minhas, Kraska) [Spector2021Bounding]
  **[read]**: a tree of radix splines, each indexing a fixed number of bytes,
  with a bounded last-mile search. On URLs 1733 ns (1314 ns with a hash
  corrector) against 786 ns for HOT, at 199–351 MB against 1.37 GB (numbers as
  reported in report F). No public repository found under the obvious names.
- **LITS** (Yang, Chen) [Yang2024LITS] **[read]**, journal version
  [Yang2026Learned] **[abstract]**: a learned model combined with HOT subtries; code at <https://github.com/schencoding/lits>
  (no licence file detected) **[repo]**. OpenAlex lists the TODS 2026 version
  among the works citing [Belazzougui2010Dynamic]; I could not read it.
- **Relation.** These are dynamic or read-mostly indexes that win on space, not
  on time (report F, section 11). None uses a hash-probed trie level.
- **Learned MMPHFs** are in section 3.

### 2.11 Other compressed dictionaries (context only)

- **LZ-compressed dictionaries** (Arz, Fischer) [Arz2014LZ, Arz2018Lempel]
  **[read: arXiv 1305.0674, skimmed]**: an LZ78 parse of the concatenated
  dictionary gives better compression than front coding on repetitive
  dictionaries at microsecond lookup times. No z-fast citation.
- **FSST** (Boncz, Neumann, Leis) [Boncz2020FSST]: a static symbol table for
  random-access string compression; now the tail compressor of C2 and a
  candidate compressor for block contents.
- **Top-k completion** (Hsu, Ottaviano) [Hsu2013Space] **[metadata only; the
  description is from memory and unverified]**: completion tries and
  score-decomposed tries, i.e., a prefix-range query followed by top-k
  retrieval; a consumer of exactly the `prefix_range` primitive.
- **Rust.** `fst` (<https://github.com/BurntSushi/fst>, Unlicense or MIT,
  finite-state transducers, 2.1K stars) is the de-facto static ordered string
  map in Rust and a mandatory baseline for a Rust crate **[repo]**. `sux`
  (<https://github.com/vigna/sux-rs>, Apache-2.0 or MIT) provides
  `RearCodedList`.
  I did not find a published "LOUDS trie on top of sux"; the only uses of Sux in
  tries that I could verify are CoCo-trie and C2-CoCo, which use the C++ Sux
  selection structures for LOUDS navigation **[read]**.

### 2.12 Third-party measurements of z-fast tries

The earlier review states that the C2 table contains "the only third-party
numbers for a z-fast trie". That is not correct; there are at least three
sources, all using the same C++ port ("ZFT", by Tsuruta):

1. **c-trie++** (Tsuruta, Köppl, Kanda, Nakashima, Inenaga, Bannai, Takeda)
   [Tsuruta2022ctrie] **[read: local extract]**: introduces the port ("ZFT: our
   z-fast trie portation from an implementation in Java to C++", as quoted in
   report F); according to the zip-trie paper, c-trie++ outperforms it "in a
   majority of performed experiments".
2. **Dynamic path-decomposed tries** [Kanda2020Dynamic] **[read]**, Table 5
   (one core of an Intel Xeon E5-2680 v2): ZFT uses 1.13 / 1.82 / 2.24 /
   2.57 GiB on GeoNames / AOL / Wiki / DNA, with insertions in 3.25–3.84 µs and
   lookups in 2.42–2.88 µs; c-trie++ lookups take 0.78–1.10 µs, ART
   0.56–0.77 µs, and the authors' DynPDT 0.53–1.22 µs in 0.11–0.84 GiB. For ZFT
   and c-trie++ some results on the four large datasets are missing "because
   the resulting trie was too large to fit into RAM".
3. **C2** (section 2.8).

In addition, **zip-tries** (Eppstein, Gila, Goodrich, Kitagawa)
[Eppstein2025Zip] **[read]** compare against c-trie++ only, which they describe
as "an optimized implementation of the z-fast trie data structure", and state
that z-fast tries "are complex and not easily parallelized".
**LeMonHash** measures Sux4J's z-fast-based MMPHF (section 3).

All these numbers concern a bit-level dynamic port. No published measurement of
a static or byte-level z-fast trie exists **[inferred from the citing lists]**.

### 2.13 Best published trade-offs today

- **Space.** 13%–34% of the data for PDT, 22%–34% for Marisa and 15%–41% for
  C2-Marisa with one recursion level (C2's datasets); 5%–35% for HTFC (the
  2016 datasets, about 5% on URIs and 10% on URLs).
- **Time among succinct structures.** C2-Marisa: 180–830 ns per positive lookup;
  everything else is slower or larger.
- **Time regardless of space.** ART (130–1600 ns) and hash-based dictionaries
  (0.5–2 µs in the 2016 study, on older hardware).
- **Front coding** is the simplest structure in the 1–2 µs range, and the only
  one of these for which the cost is dominated by a binary search that a z-fast
  index can replace.
- **Datasets.** There is no single standard. Recurring ones: URLs (uk-2002,
  18.5M, from WebGraph; "UK private domains"; a footnote of the CoCo-trie paper
  points to LAW's gsh-2015), Wikipedia titles, DNA k-mers, dblp XML lines, English words, GeoNames, AOL queries, TREC GOV2 terms,
  proteins, Uniprot URIs, web-server logs. Query workloads differ (positive
  only in C2; half misses with long shared prefixes in CoCo-trie).

---

## 3. Monotone minimal perfect hashing and its relatives

### 3.1 The original constructions

[Belazzougui2009Monotone], [Belazzougui2009Theory], [Belazzougui2011Theory].
An MMPHF maps the i-th smallest key of a set to i and anything else to an
arbitrary value. The two families are (a) bucketing by longest common prefix,
which is the fast and larger one, and (b) bucketing with a *relative trie*
(hollow trie or z-fast trie as *distributor*), which reaches
O(n log log log u) bits. The summary in the LeMonHash paper **[read]**: the
hollow trie distributor uses O(log log log u) bits per key with O(log u) query
time; the z-fast trie uses O(log log log u) bits per key with
O((log u)/w + log log u) query time.

**Implementation.** Sux4J (<https://github.com/vigna/Sux4J>, LGPL-2.1,
maintained), package `it.unimi.dsi.sux4j.mph`:
`HollowTrieMonotoneMinimalPerfectHashFunction`, `HollowTrieDistributor`,
`ZFastTrieDistributor` (each distributor with its
`...MonotoneMinimalPerfectHashFunction`), `LcpMonotoneMinimalPerfectHashFunction`
with its `TwoSteps` and `VL` variants, `PaCoTrieDistributor` and its `VL`
variant (checked in the local clone). These are the baselines of
every later MMPHF experiment.

### 3.2 Lower bounds

- **Assadi, Farach-Colton, Kuszmaul** [Assadi2023Tight, Assadi2025Tight]
  **[read: arXiv 2207.10556]**: any MMPHF, deterministic or randomised, needs
  Ω(n log log log u) expected bits; "the best upper bound known for the problem
  encodes D in O(n log log log u) bits and performs queries in O(log u) time".
  The paper mentions monotone hashing 52 times and never z-fast tries: it cites
  the SODA 2009 and JEA papers for the upper bound.
- **Kosolobov** [Kosolobov2024Simplified] **[read: arXiv 2403.07760]**: for
  u ≥ (1+ε)n the tight bound is Ω(n min{log log log(u/n), log n}), closing the
  remaining range of u, with a simpler proof of part of the argument.

**Relation.** The lower bound makes the relative-trie construction of 2009, of
which the (hollow) z-fast trie is the fast-query variant, space-optimal. This is
a statement the paper can make with a citation: the z-fast-based distributor is
the known way to get optimal space with query time O((log u)/w + log log u)
rather than O(log u).

### 3.3 LeMonHash and LeMonHash-VL (Ferragina, Lehmann, Sanders, Vinciguerra) [Ferragina2023Learned] **[read: local extract]**

- **Mechanism.** A learned model (PGM-index) maps each key to an estimated rank;
  keys with the same estimate form a bucket; a retrieval structure (BuRR) stores
  the rank of each key inside its bucket. LeMonHash-VL handles strings by
  recursively splitting on chunks after the common prefix.
- **Measured (Table A.1; Intel Xeon E5-2670 v3).** Query throughput in thousands
  of queries per second / bits per key:

| Method | text | dna | urls |
|---|---|---|---|
| LeMonHash-VL | 1278 / 6.08 | 790 / 6.25 | 338 / 6.46 |
| ZFast (Sux4J) | 530 / 8.88 | 269 / 8.71 | 198 / 8.77 |
| Hollow (Sux4J) | 148 / 6.90 | 124 / 9.26 | 73 / 8.41 |
| HTDist (Sux4J) | 92 / 5.40 | 80 / 5.67 | 52 / 5.70 |
| LCP 2-step (Sux4J) | 1176 / 13.12 | 834 / 11.62 | 394 / 17.81 |
| Centroid HT (PDT) | 560 / 6.78 | 294 / 9.05 | 399 / 8.36 |

  On integer datasets LeMonHash reaches 2.6–4.9 bits per key at 1.3–2.7 million
  queries per second; ZFast 7.6–7.9 bits at 345–611 thousand.
- **Code.** <https://github.com/ByteHamster/LeMonHash> and the benchmark harness
  <https://github.com/ByteHamster/MMPHF-Experiments> (which wraps the Java
  competitors), both GPL-3.0, maintained **[repo]**.
- **Cites z-fast tries?** Yes; it describes the z-fast trie in a paragraph of its
  own and benchmarks Sux4J's implementation (labelled "ZFast [3]"). The caption
  of its Figure 3 marks the competitors implemented in Java, which include all
  the Sux4J ones; the comparison is therefore across languages.
- **Relation.** For *rank of keys known to be in the set* LeMonHash-VL is the
  state of the art in both space and time. It does not answer predecessor or
  prefix queries for strings outside the set, which is what the static z-fast
  trie does.

### 3.4 Surveys

Lehmann, Mueller, Pagh, Pibiri, Sanders, Vigna, Walzer, *Modern minimal perfect
hashing: a survey* [Lehmann2026Modern] **[read: arXiv 2506.06536]**: refers to
[Belazzougui2011Theory] for monotone functions and lists their applications
(rank on sparse bit vectors, document retrieval, range queries, model checking);
no mention of z-fast tries.

---

## 4. Weak prefix search: follow-ups and uses

The weak prefix search (wps) structure of ESA 2010 is a hollow z-fast prefix
trie plus a range locator. Works that build on it, each **[read]** unless
tagged:

| Work | Use |
|---|---|
| Ferragina, *On the weak prefix-search problem* [Ferragina2013Weak] **[metadata only]** | The direct follow-up, by title; I could not read it and do not describe it |
| Ferragina, Venturini, *Compressed cache-oblivious string B-tree* [Ferragina2016Compressed] **[abstract: citation contexts]** | Uses the wps formulation and compares with its space lower bound, showing that a refined measure Trie(S) can go below it on some dictionaries |
| Belazzougui, *Approximate full-text indexing with one edit error* [Belazzougui2015Improved] | Replaces a hash dictionary by a wps structure over the length-b factors of the text (the O(1)-time and the O(log b)-time variants) |
| Navarro, Nekrich, *Time-optimal top-k document retrieval* [Navarro2017Time] | wps structures on suffix-tree nodes to descend the last fewer-than-ℓ symbols in O(1) time, o(1) words per node |
| Gagie, Navarro, Prezza, r-index [Gagie2020Fully] | Same device: "weak prefix search (wps) structures on the selected nodes, which allow descending by up to d − 1 symbols in constant time", built in O(rs w^ε) expected time, in the RAM-optimal-time variant |
| Belazzougui, Gagie, Munro, Navarro, Nekrich, range majorities/minorities [Belazzougui2021Range] (arXiv 1210.1765) | wps as a compact range-emptiness index that also returns one element of a non-empty interval |
| Goswami, Grønlund, Larsen, Pagh, *Approximate range emptiness* [Goswami2015Approximate] | The exact-emptiness building block is "the sorted list of points plus ... the weak prefix search data structure of [ESA 2010, Theorem 5, second part]": two wps queries on the longest common prefix of the endpoints decide emptiness in O(1) time |
| Cohen, Fiat, Hershcovitch, Kaplan, *Minimal indices for predecessor search* [Cohen2015Minimal] | Uses a (simplified) probabilistic z-fast trie on w/log w keys as its "α node"; matches the space of the "probe-suboptimal z-fast trie" with O(1) probes |
| Domingues, STOC 2026 [Domingues2026Compressing] | Cites the Ω(n log w) lower bound for the weak prefix problem via Cohen et al. |
| Bille et al., LZ index [Bille2018Time]; Christiansen, Ettienne [Christiansen2018Compressed] | wps as the trie over phrase suffixes/prefixes (section 5.2) |

Minimal perfect hashing papers (PTHash, PHOBIC, ShockHash, MorphisHash, learned
static functions) cite ESA 2010 only as an *application* of perfect hashing
("prefix-search data structures") **[abstract: citation contexts]**.

---

## 5. Z-fast tries in text indexing and stringology

### 5.1 Zuffix arrays

- **Boldi, Vigna**, FUN 2018 [Boldi2018Kings] and **Boldi, Marchini, Vigna**,
  SEA 2024 [Boldi2024Engineering] **[read: local extracts]**. Theory:
  O((m log σ)/w + log m + σ) search on a suffix array endowed with a z-map.
  Experiments: only against the enhanced suffix array
  [Abouelhoda2004Replacing]; up to 7× on large alphabets, about 2× on DNA; the
  z-map takes 5–8 GiB on 200 MiB texts. Code:
  <https://github.com/smarchini/zuffix> (no licence file detected, last push
  2023) **[repo]**.
- **Citations.** OpenAlex and Semantic Scholar report no citation for the SEA
  paper, and for the FUN paper only the suffixient-array and STPD papers
  (section 5.4) and the SEA paper itself.

### 5.2 Dictionary-compressed indexes: the main line of use

The pattern, as summarised in Navarro's survey [Navarro2021Indexing]
**[read]**: an index based on an LZ77 parse, a grammar or an attractor finds
*primary* occurrences by trying each way of cutting the pattern into a prefix
and a suffix and looking up the reversed prefix and the suffix in two compacted
tries of phrase boundaries; a 2D range structure joins the two ranges. "The trie
is actually a version called z-fast trie [...], which allows searching for a
string of length ℓ with O(log ℓ) fingerprint comparisons", and one text
extraction verifies all candidates. The chain of papers:

| Work | What the z-fast trie does there |
|---|---|
| Gagie, Gawrychowski, Kärkkäinen, Nekrich, Puglisi, *A faster grammar-based self-index* [Gagie2012Faster] **[read: arXiv 1109.3954; it cites SODA 2009, ESA 2010 and Ferragina's paper]** and *LZ77-based self-indexing with faster pattern matching* [Gagie2014LZ77] **[not read]** | First use: Karp–Rabin fingerprints plus z-fast tries to find the loci of all m − 1 splits in O(m log m) total time, then batch verification |
| Bille, Ettienne, Gørtz, Vildhøj, *Time-space trade-offs for Lempel–Ziv compressed indexing* [Bille2018Time] | "Lemma 4 (Belazzougui et al., appendix H.3)": wps in O(k(lg ℓ + lg lg σ)) bits and O(m lg σ / w + lg m) time; they add a *batched* variant for h substrings of the pattern and a search that can start at any ancestor |
| Christiansen, Ettienne, *Compressed indexing with signature grammars* [Christiansen2018Compressed] | "We will refer to the data structure of Lemma 2 as a z-fast trie"; two z-fast tries T1, T2 plus 2D range reporting; O(lg m) per split |
| Navarro, Prezza, *Universal compressed text indexing* [Navarro2019Universal] | "We must then build the z-fast trie [ESA 2010, Thm. 5] on the sets X and Y"; uses the simpler variant of Kempa and Kosolobov and fixes its construction; needs a Karp–Rabin function collision-free on substrings whose length is a power of two |
| Christiansen, Ettienne, Kociumaka, Navarro, Prezza, *Optimal-time dictionary-compressed indexes* [Christiansen2021Optimal] | Lemma 6.5: "Z-fast tries [Belazzougui et al. 2010, Sec. H.2] solve the weak part of the lemma in O(m log(σ)/w + τ log m) time"; built in O(g) expected time and space |
| Kociumaka, Navarro, Olivares, δ-optimal space [Kociumaka2024Near] | Uses Lemma 6.5 of the previous paper as a black box (its Lemma 5.5); does not cite the z-fast papers directly |
| Gawrychowski, Gourdel, Starikovskaya, Steiner, *Compressed indexing for consecutive occurrences* [Gawrychowski2023Compressed] | Same lemma as "Fact A.3", noting that it "builds on Belazzougui et al." |
| Tsuruta, Köppl, Nakashima, Inenaga, Bannai, Takeda, *Grammar-compressed self-index with Lyndon words* [Tsuruta2020Grammar] | "we have (static) z-fast tries built on the lists L and L^REV" |
| Gagie, Goga, Jeż, Navarro, *Space-efficient conversions from SLPs* [Gagie2024Space] | "We will also use a variant of a z-fast trie" (Lemma 2); separate z-fast tries on the two sets of rule expansions |
| Gagie, Kashgouli, Langmead, *KATKA* [Gagie2022KATKA] | Suggested extension to MEMs: "if we store O(z)-space z-fast tries for the Patricia trees then ... we can find the loci ... in O(log m) time" |
| Kosolobov, *Compressed index with construction in compressed space*, CPM 2026 [Kosolobov2026Compressed] | Turns a compacted trie "into a z-fast trie by augmenting it with a hash table that, for each node x, maps the fingerprint of a certain prefix of str(x) to x", with *deterministic* fingerprints from a locally consistent parsing instead of Karp–Rabin |
| Prezza, *Optimal rank and select queries on dictionary-compressed text* [Prezza2019Optimal] | Negative use: earlier select structures "use z-fast tries, which introduce a O(log log n) multiplicative factor"; the paper removes it |

All **[read]** except where noted. Claude, Navarro and Pacheco's grammar index
(JCSS 2021, arXiv 2004.01032) appears in the citing list of ESA 2010 but my text
search found no occurrence of "z-fast" or "weak prefix" in the arXiv version; I
do not count it as a user.

### 5.3 What does *not* use z-fast tries

I checked the full texts of the Kempa–Kociumaka line because the task brief asked
which of these works cite z-fast tries. **None of the four does**:

- *String synchronizing sets* (arXiv 1904.04228),
- *Breaking the O(n)-barrier in the construction of compressed suffix arrays and
  suffix trees* (arXiv 2106.12725),
- *Dynamic suffix array with polylogarithmic queries and updates*
  (arXiv 2201.01285),
- *Collapsing the hierarchy of compressed data structures* (arXiv 2308.03635).

Their citations of Belazzougui are to other papers (compressed-index
construction, alphabet-independent indexing, range predecessor). The same holds
for Kempa and Prezza's string-attractor paper (arXiv 1710.10964) and for
Kociumaka, Navarro and Prezza's δ paper (arXiv 1910.02151), which reach z-fast
tries only through later papers. *Deterministic indexing for packed strings*
(Bille, Gørtz, Skjoldjensen, arXiv 1612.01748) does not cite them either. These
papers are therefore not listed in the `.bib` file.

Nishimoto and Tabei's OptBWTR paper [Nishimoto2021Optimal] **[read]** cites the
dynamic z-fast trie only as a row of a comparison table for prefix search
(space (n log σ)/w + O(|D|) words, expected time
O(⌈m log σ / w⌉ + log m + log log σ + occ′)); it does not use it. Munro, Navarro
and Nekrich [Munro2020Text] cite it once, for building "the perfect hashes and
predecessor data structures with the children of each node".

### 5.4 Suffixient sets, suffixient arrays, STPD

- **Suffixient sets** (Depuydt, Gagie, Langmead, Manzini, Prezza)
  [Depuydt2023Suffixient] **[read: arXiv 2312.01359]**: "The two components of
  our compressed index are the SLP-based LCP/LCS data structure ... and a z-fast
  trie for the reversed prefixes of T ending at positions in S. The z-fast trie
  takes O(r̄) space and ... returns in O(log min(m, n)) time an element" of the
  suffixient set. Here the z-fast trie *is* the index.
- **Suffixient arrays** (Cenzato, Depuydt, Gagie, Kim, Manzini, Olivares,
  Prezza) [Cenzato2026Suffixient] **[read: arXiv 2407.18753 extract]**: Lemma 23
  is "z-fast trie, [FUN 2018, Thm. 6–7]", one trie per character, all sharing a
  hash function that is collision-free on the substrings of T; section 4.5
  gives "offline solutions with only random access and z-fast tries". The
  *implemented* index uses binary search on the suffixient array plus text
  access (as low as 3.5 ns per character; code
  <https://github.com/regindex/suffixient-array>, MIT **[repo]**).
- **STPD** (Becker, Cenzato, Gagie, Groot Koerkamp, Kim, Manzini, Prezza)
  [Becker2026Compressing] **[read: local extract]**: "a faster
  solution can be obtained using z-fast tries", again one per character, over
  text prefixes at sampled positions; code
  <https://github.com/regindex/STPD-index>. Published at ICALP 2026 (LIPIcs 374,
  24:1–24:25, DOI 10.4230/LIPIcs.ICALP.2026.24 according to DataCite). Beware:
  the arXiv version (v6) prints the placeholder DOI
  10.4230/LIPIcs.ICALP.2026.119, which belongs to a different paper.

**Observation [inferred].** In this line the z-fast trie is the theoretical
route to O(log m) per step, and binary search is what is implemented. A
practical static z-fast trie over a sorted set of text positions (which is what
`StaticZFastTrie` over an `IndexedSeq` provides) is the missing experiment, and
the suffixient-array code is the natural place to try it.

### 5.5 Streaming and small-space pattern matching

- **Clifford, Fontaine, Porat, Sach, Starikovskaya**, *Dictionary matching in a
  stream* [Clifford2015Dictionary] **[read]**: "we will use a slight
  modification of the probabilistic z-fast trie ... (Theorem 4.1)", with the
  fingerprint of the reversed string as signature, augmented with a
  coloured-ancestor structure; the exit node is found comparing at most
  log m + log log(k + m) signatures.
- **Gawrychowski, Starikovskaya**, *Streaming dictionary matching with
  mismatches* [Gawrychowski2022Streaming] **[read]**: "the z-fast trie can be
  considered as a randomised implementation of the compact trie. In the
  randomised implementation of the k-errata tree, we replace each compact trie
  with the z-fast trie"; they say that z-fast tries "have proved to be useful in
  many streaming applications".

### 5.6 Compressed-space LZ parsing (the dynamic structure as a black box)

- **Badkobeh, Gagie, Inenaga, Kociumaka, Kosolobov, Puglisi**, *On two
  LZ78-style grammars* [Badkobeh2017Two] **[read]**: "the dynamic z-fast trie of
  Belazzougui et al." is one of four tools of a streaming Monte Carlo LZD/LZMW
  parser in O(z log n) space; the trie holds the phrases and is augmented with a
  dynamic marked-ancestor structure. This is the clearest black-box use of the
  *dynamic* structure of SPIRE 2010.
- **Kempa, Kosolobov**, *LZ-End parsing in compressed space* [Kempa2017LZ]
  **[read]**: a compacted trie with hashes of selected prefixes; "our method
  resembles the so called fat binary search in z-tries introduced in [SODA
  2009]". Navarro and Prezza later adopt this simplified variant.

### 5.7 Tries and predecessor structures that build on, compare with or replace the z-fast trie

| Work | Relation |
|---|---|
| Belazzougui, Boldi, Vigna, *Predecessor search with distance-sensitive query time* [Belazzougui2012Predecessor] **[read]** | Builds on z-fast tries to get query time depending on the distance to the answer |
| Ehrhardt, Mulzer, *Delta-fast tries* [Ehrhardt2017Delta] **[abstract: citation contexts]** | Local search in linear space; the handle notion "was introduced for z-fast tries" and its hash map "is defined similarly" |
| Takagi, Inenaga, Sadakane, Arimura, *Packed compact tries* [Takagi2017Packed] **[read]** | Deterministic alternative: the "signed dynamic z-fast trie" answers "only with high probability"; packed c-tries are always correct with O(m/α + log log n) expected time |
| Tsuruta et al., *c-trie++* [Tsuruta2022ctrie] **[read]** | Alphabet-aware z-fast micro-tries under a macro trie; follows "Algorithm 1 and Section 3.3" of the SPIRE paper; ships the ZFT port |
| Kanda et al., *Dynamic path-decomposed tries* [Kanda2020Dynamic] **[read]** | Benchmarks ZFT (section 2.12) |
| Arroyuelo et al., *Engineering practical Lempel–Ziv tries* [Arroyuelo2021Engineering] **[abstract: citation context]** | Mentions augmenting nodes with dictionaries to jump over several levels, "similarly to the word-packed lookup techniques of compacted tries" |
| Bille, Gørtz, Steiner, *String indexing with compressed patterns* [Bille2023String] **[read]** | "Similar results follow from using z-fast tries, however, we present a direct and simple solution": an exponential search on powers of two with Karp–Rabin fingerprints and the ART decomposition, avoiding 2-fattest numbers |
| Bille, Gawrychowski, Gørtz, Landau, Weimann, *Top tree compression of tries* [Bille2021Top] **[read]** | Cites it in a list of trie representations that "use word RAM techniques" |
| Eppstein, Gila, Goodrich, Kitagawa, *Zip-tries* [Eppstein2025Zip] **[read]** | Presents zip-tries as simpler and parallelisable; z-fast tries are "complex and not easily parallelized" |
| Kang et al., *PIM-trie* [Kang2023PIM] **[abstract: citation context]** | "how to address [skew] for z-fast tries is still an open question" in processing-in-memory |
| Navarro, Rojas-Ledesma, *Predecessor search* (survey) [Navarro2020Predecessor] **[abstract: citation contexts]** | "Belazzougui et al. introduced the dynamic z-fast trie, a version of Willard's x-fast tries that achieves linear space and O(log log u) query and update time with high probability"; "introduced in this context of prefix and predecessor search for strings" |

### 5.8 What improves on or replaces z-fast tries in these applications

- **Determinism.** The weak search plus Karp–Rabin needs a collision-free
  function (expensive to build: O(n log n) expected time in Christiansen et
  al.). Kosolobov 2026 replaces Karp–Rabin by fingerprints derived from a
  locally consistent parsing; Bille–Gørtz–Steiner replace 2-fattest numbers by
  powers of two; packed c-tries avoid randomisation altogether.
- **Speed in theory.** For O(m + ...)-time indexes, locally consistent parsing
  reduces the number of splits to O(log m), after which the O(log m) per split
  of the z-fast trie no longer dominates (Christiansen et al.; Kociumaka et
  al.).
- **Speed in practice.** Wherever an implementation exists (suffixient arrays,
  STPD), binary search on a sorted sample plus text access replaced the z-fast
  trie; c-trie++ and DynPDT beat the ZFT port. No implementation in this
  literature uses a tuned z-fast trie.
- **Baseline.** The enhanced suffix array remains the baseline of the zuffix
  experiments; report F and REVIEW.md section 6 list the missing comparisons
  (plain suffix array, FM-index, r-index, suffixient array).

---

## 6. Range filters and prefix filters for strings

Only the points that bear on trie-plus-hash designs; the earlier review
(report F, section 4) has the general table.

- **A z-fast-lineage structure is already inside the theory of range filters.**
  Goswami et al. [Goswami2015Approximate] **[read]** prove the
  n lg(L/ε) − O(n) lower bound and match it with a structure whose exact
  component is the weak prefix search structure of ESA 2010. **Grafite** (Costa,
  Ferragina, Vinciguerra) [Costa2024Grafite] **[read]** cites ESA 2010 in
  exactly this role ("... from [4] to check hash codes for inclusion in a query
  range") and replaces it with Elias–Fano on hashed keys: B bits per key,
  false-positive rate at most ℓ/2^(B−2). This lineage (weak prefix search →
  approximate range emptiness → Grafite) is worth one sentence in the paper.
- **String keys.** According to Table 2 of **Diva** (Eslami, Bercea, Dayan)
  [Eslami2025Diva] **[read]**, only SuRF and Diva support variable-length keys.
  Diva samples keys into a trie and stores distinguishing infixes; code
  <https://github.com/n3slami/Diva>, MIT. **Proteus** (Knorr et al.)
  [Knorr2022Proteus] **[read]** combines an FST of fixed depth with a prefix
  Bloom filter and handles strings by padding to a fixed length. **Memento**
  (Eslami, Dayan) [Eslami2024Memento], **Rosetta** (Luo et al.) [Luo2020Rosetta]
  and **REncoder** (Wang et al.) [Wang2023REncoder] are for fixed-length keys
  (per Diva's table; I verified only their bibliographic data).
- **2026.** A web search returned *Aeris Filter* (Chesetti, Eslami, Zhang,
  Dayan, Pandey, announced for SIGMOD 2026) and an adaptive filter called
  *Hourglass*; I could not obtain either paper, so I cannot say whether they
  handle strings or use prefix hashing. They are not in the `.bib` file.
- **Competing or using trie-plus-hash?** No range filter in this list probes a
  hash table with prefixes chosen by a binary search on length. Rosetta and
  Proteus hash prefixes into Bloom filters (all lengths, or one chosen length);
  SuRF and Diva are tries without hashing. A probabilistic z-fast structure for
  strings remains unexplored **[inferred]**, and would have to compete at
  10–20 bits per key.

---

## 7. External-memory and learned variants

- **Cache-oblivious string dictionaries.** The ESA 2010 structures were designed
  for "very large collections stored in slow-access memory": a weak prefix
  search takes O(|p|/B) I/Os for the index and then one access to the data
  **[read]**. Ferragina and Venturini's compressed cache-oblivious string B-tree
  [Ferragina2016Compressed] is the follow-up on the dictionary side
  **[abstract]**.
- **Two-level dictionaries** (section 2.9): in-memory succinct Patricia trie
  over block heads, rear-coded blocks on disk. No hash-probed level.
- **Hash-probed trie levels in practice.** Within the literature I examined, a
  hash-probed trie level appears in practice only in in-memory indexes (Wormhole
  and the Cuckoo Trie, covered by the companion reports) and in the zuffix
  array. I found no external-memory system that keeps a z-map (or any
  prefix-keyed hash table probed by binary search on length) in RAM over
  on-disk strings **[inferred from the citing lists and searches; absence of
  evidence]**.
- **Learned.** LeMonHash-VL (section 3.3) is the learned counterpart of the
  z-fast-based MMPHF. RSS and LITS (section 2.10) are learned ordered indexes
  for strings. Learned suffix-array indexes for read mapping (LISA, BWA-MEME)
  are discussed in report F and were not re-examined.

---

## 8. Synthesis

### 8.1 Works that use z-fast tries as a component, by purpose

"Uses" means that the structure appears in the construction, not merely in the
related work. S = static variant (usually ESA 2010 Theorem 5 or its Karp–Rabin
simplification), D = dynamic variant (SPIRE 2010), P = probabilistic/hollow
variant (SODA 2009 / ESA 2010).

**A. Locating pattern prefixes/suffixes among phrase boundaries in
dictionary-compressed indexes (S).**
[Gagie2012Faster], [Gagie2014LZ77] (not read; role confirmed by
[Navarro2021Indexing] and [Christiansen2021Optimal]), [Bille2018Time],
[Christiansen2018Compressed], [Navarro2019Universal],
[Christiansen2021Optimal], [Tsuruta2020Grammar], [Gagie2024Space],
[Kosolobov2026Compressed]; through Lemma 6.5 of [Christiansen2021Optimal]:
[Kociumaka2024Near], [Gawrychowski2023Compressed]; as a suggested extension:
[Gagie2022KATKA]. Survey: [Navarro2021Indexing].

**B. Suffix-array-like indexes (S).**
Zuffix arrays [Boldi2018Kings, Boldi2024Engineering]; suffixient sets and
arrays [Depuydt2023Suffixient, Cenzato2026Suffixient]; STPD
[Becker2026Compressing].

**C. Weak prefix search as a constant-time or small-space descent or emptiness
primitive (P/S).**
Top-k document retrieval [Navarro2017Time]; r-index with RAM-optimal search
[Gagie2020Fully]; one-error approximate indexing [Belazzougui2015Improved];
range majorities/minorities [Belazzougui2021Range]; approximate range emptiness
[Goswami2015Approximate] (and, by citation, Grafite [Costa2024Grafite]);
compressed cache-oblivious string B-tree [Ferragina2016Compressed] (problem
formulation and lower bound).

**D. Streaming pattern matching (P).**
[Clifford2015Dictionary], [Gawrychowski2022Streaming].

**E. Compressed-space parsing (D, and fat binary search).**
[Badkobeh2017Two] (dynamic z-fast trie as a black box), [Kempa2017LZ] (fat
binary search on a hashed compacted trie).

**F. Monotone minimal perfect hashing (P).**
The z-fast and hollow-trie distributors of [Belazzougui2009Monotone,
Belazzougui2011Theory], implemented in Sux4J and used as baselines in
[Ferragina2023Learned]; shown space-optimal by [Assadi2023Tight,
Kosolobov2024Simplified].

**G. Predecessor search and tries (D/P).**
[Belazzougui2012Predecessor], [Cohen2015Minimal] (z-fast trie as a node type),
[Ehrhardt2017Delta] (handles and hash map modelled on it), [Tsuruta2022ctrie]
(alphabet-aware z-fast micro-tries).

**Not found.** No use inside a database, key-value store or search engine other
than through Sux4J's monotone hash functions; no implementation in any of the
indexes of groups A, C, D and E (they are theoretical results).

### 8.2 Strongest static competitors for a comparative benchmark

| # | Structure | Repository | Licence | Build notes | Supports | Why include |
|---|---|---|---|---|---|---|
| 1 | Front/rear coding with binary search on block heads | `sux` `RearCodedList` (<https://github.com/vigna/sux-rs>); `libCSD` PFC/HTFC (<https://github.com/migumar2/libCSD>) | Apache-2.0/LGPL; LGPL-2.1 | Rust; libCSD is C++ with a Makefile and depends on libcds | lookup, access, prefix | The structure being indexed; the baseline the z-fast index must beat |
| 2 | Two-level index (succinct Patricia trie over rear-coded blocks) | <https://github.com/mariagiovannarotundo/Two-level-indexing> | GPL-3.0 | C++; not built here | not verified (paper not read) | Same architecture with a blind trie instead of a z-map |
| 3 | C2-Marisa, C2-FST | <https://github.com/gt-alg-kit/C2> (redirect from `alexztc/C2`) | none detected | C++17, g++ 10.3 -O3 in the paper; bundles PDT's string pool and FSST | lookup; range iteration only in C2-FST | Fastest succinct tries published (2026); its harness also runs Marisa, PDT, FST, ART, C-ART, c-trie++ and the ZFT port |
| 4 | Marisa | <https://github.com/s-yata/marisa-trie> | BSD-2-Clause or LGPL-2.1+ | C++ | lookup, reverse lookup, common-prefix and predictive search (from memory of the project documentation, not re-checked) | The practical standard |
| 5 | PDT | <https://github.com/ot/path_decomposed_tries> | Microsoft Research, non-commercial | C++; not built here | lookup, access; also usable as an MMPHF (it is a competitor in the LeMonHash benchmark) | Smallest on half of C2's datasets; mind the licence |
| 6 | FST | <https://github.com/kampersanda/fast_succinct_trie> (dictionary variant of <https://github.com/efficient/SuRF>) | Apache-2.0 | C++ | lookup in the dictionary variant; SuRF itself is a filter with range queries | Reference succinct trie with ordered queries |
| 7 | CoCo-trie | <https://github.com/aboffa/CoCo-trie> | GPL-3.0 | C++20, g++ 11; needs the `vgteam/sdsl-lite` fork, ds2i, Sux; build is slow and memory-hungry on full strings | lookup (rank by design) | Best published space/time on distinguishing prefixes |
| 8 | XCDAT | <https://github.com/kampersanda/xcdat> | MIT | C++17, header-only, CMake | lookup, decode, prefix and predictive search | Fast double-array point of the trade-off |
| 9 | `fst` | <https://github.com/BurntSushi/fst> | Unlicense or MIT | Rust crate | ordered set/map with range queries | The Rust baseline users will ask about |
| 10 | LeMonHash-VL and Sux4J MMPHFs | <https://github.com/ByteHamster/LeMonHash>, <https://github.com/ByteHamster/MMPHF-Experiments> | GPL-3.0 | C++ with CMake; the harness calls the Java competitors | rank of keys in the set only | Lower envelope for "rank of a known key": 6.5 bits per key on URLs |

**Datasets to use, so that numbers can be set beside published ones.** Wikipedia
titles (enwiki dump; C2 uses 2024-11-20, 17.5M titles); uk-2002 URLs from
WebGraph/LAW (18.5M, the dataset of Martínez-Prieto et al.; C2's "url" is a
smaller set of UK private domains and CoCo-trie uses gsh-2015); DNA k-mers
(31-mers in C2, 12-mers in Martínez-Prieto et al.); dblp XML lines from
Pizza&Chili; English words (`dwyl/english-words`); GeoNames; LeMonHash's
"text", "dna" and "urls" string sets (see that paper). Report both positive-only lookups (as C2) and a half-miss workload
with long shared prefixes (as CoCo-trie); report space as a percentage of the
data and as bits per key.

**Build notes common to all.** The C2 paper states "g++ 10.3.0 at -O3" and no
architecture flag; the CoCo-trie paper states g++ 11.1 and C++20. For a fair
comparison with a Rust crate built with `target-cpu=native`, check each build
system and add `-march=native` where it is missing; any Rust harness crate
needs its own `.cargo/config.toml`. None of the repositories above was built
during this survey.

---

## 9. Corrections and additions to the earlier review

1. **"The only third-party numbers for a z-fast trie" (REVIEW.md 4.2; report F
   section 9).** Not so: the ZFT port is also measured in *Dynamic
   path-decomposed tries* [Kanda2020Dynamic], Table 5 (and in the c-trie++ paper
   itself); see section 2.12.
2. **C2's venue.** The arXiv version carries the header of EDBT 2027; it is not
   only a preprint.
3. **STPD** is in ICALP 2026 (DOI 10.4230/LIPIcs.ICALP.2026.24; the DOI printed
   in the arXiv version is wrong); **suffixient arrays** now have a journal
   version (Theory Comput. Syst. 70(3), 2026).
4. **Report F, section 7** says the FUN paper is cited "by the suffixient and
   STPD papers, which use z-fast tries only in their theory". Confirmed, with
   the addition that the earlier *Suffixient sets* paper makes the z-fast trie
   one of the two components of its index.
5. **Range filters.** Report F found "no filter that does binary search on
   prefix length"; still true, but the optimal theoretical structure of Goswami
   et al. is built on weak prefix search, and Grafite cites it in that role.
6. **PDT licence** is non-commercial (Microsoft Research); **C2's repository**
   has moved.

---

## Papers I could not access

DBLP itself (search API and pages) was unreachable throughout; this affected
verification, not reading.

| Paper | Venue, DOI / URL | What I needed |
|---|---|---|
| P. Ferragina, M. Rotundo, G. Vinciguerra, *Two-level massive string dictionaries* | Information Systems 128 (2025) 102490, <https://doi.org/10.1016/j.is.2024.102490>; and the SPIRE 2023 version *Engineering a textbook approach to index massive string dictionaries*, <https://doi.org/10.1007/978-3-031-43980-3_16> | Full text: index layout, query times, bits per string, datasets, whether z-fast tries or weak prefix search are cited or compared (institutional repository copies require login) |
| S. Kanda, K. Morita, M. Fuketa, *Compressed double-array tries for string dictionaries supporting fast lookup* | Knowl. Inf. Syst. 51(3):1023–1042, 2017, <https://doi.org/10.1007/s10115-016-0999-8> | Mechanism and measurements behind XCDAT |
| S. Yata, *Dictionary compression by nesting prefix/patricia tries* | Proc. 17th Annual Meeting of the Association for Natural Language Processing (Japan), 2011 (reference copied from the C2 paper) | Existence and exact bibliographic data; the description of Marisa |
| P. Ferragina, *On the weak prefix-search problem* | Theoret. Comput. Sci. 483:75–84, 2013, <https://doi.org/10.1016/j.tcs.2012.06.011> (CPM 2011: <https://doi.org/10.1007/978-3-642-21458-5_23>) | Its bounds and how they compare with ESA 2010 |
| P. Ferragina, R. Venturini, *Compressed cache-oblivious string B-tree* | ACM Trans. Algorithms 12(4), 2016, <https://doi.org/10.1145/2903141> | Full statement of how it improves on the weak prefix search bound |
| T. Gagie, P. Gawrychowski, J. Kärkkäinen, Y. Nekrich, S. J. Puglisi, *LZ77-based self-indexing with faster pattern matching* | LATIN 2014, LNCS 8392, 731–742, <https://doi.org/10.1007/978-3-642-54423-1_63> | The original statement of the z-fast trie plus Karp–Rabin lemma (its role is confirmed by later papers) |
| B.-J. Hsu, G. Ottaviano, *Space-efficient data structures for top-k completion* | WWW 2013, 583–594, <https://doi.org/10.1145/2488388.2488440> | Space and time numbers; whether it cites z-fast tries |
| Y. Yang, S. Chen, *Learned indices for string keys with optimized performance and O(N) space cost* | ACM Trans. Database Syst., 2026, <https://doi.org/10.1145/3841175> | The context in which it cites *Dynamic z-fast tries* |
| M. Ehrhardt, W. Mulzer, *Delta-fast tries: local searches in bounded universes with linear space* | WADS 2017, LNCS 10389, 361–372, <https://doi.org/10.1007/978-3-319-62127-2_31> | Full text (I saw only citation contexts) |
| D. Arroyuelo et al., *Engineering practical Lempel–Ziv tries* | ACM J. Exp. Algorithmics 26, 2021, <https://doi.org/10.1145/3481638> | Full text (citation context only) |
| H. Kang et al., *PIM-trie: a skew-resistant trie for processing-in-memory* | SPAA 2023, <https://doi.org/10.1145/3558481.3591070> | Full text (citation context only) |
| Chesetti, Eslami, Zhang, Dayan, Pandey, *Aeris Filter* | announced for SIGMOD 2026; the URL returned by the search, <https://prashantpandey.github.io/uploads/sigmod26_aeris.pdf>, answered 404 | Whether it supports string keys; mechanism; exact title and bibliographic data (unverified) |
| *Hourglass* (adaptive range filter, SIGMOD 2026, per a search snippet) | no URL obtained | Same |
| S. Luo et al., *Rosetta*; Z. Wang et al., *REncoder*; N. Eslami, N. Dayan, *Memento filter* (full texts) | <https://doi.org/10.1145/3318464.3389731>, <https://doi.org/10.1109/ICDE55515.2023.00158>, <https://doi.org/10.1145/3698820> | I verified metadata and relied on Diva's Table 2 and report F for their properties; Memento is in the local extracts but I did not re-read it |
| Google Scholar "cited by" lists for the six z-fast papers | not reachable by script | A complete list of citing works (OpenAlex and Semantic Scholar are incomplete, especially for SPIRE 2010) |

Leads not followed for lack of time, which may hide further users: Y. Yu,
D. Belazzougui, C. Qian, Q. Zhang, *A concise forwarding information base for
scalable and fast name lookups* (ICNP 2017, DOI 10.1109/ICNP.2017.8117530);
S. Golan, E. Porat, *Real-time streaming multi-pattern search for constant
alphabet* (ESA 2017, DOI 10.4230/LIPIcs.ESA.2017.41); A. Orlandi, R. Venturini,
*Space-efficient substring occurrence estimation* (Algorithmica 74(1), 2016,
DOI 10.1007/s00453-014-9936-y); T. Takagi et al., *Applications of succinct
dynamic compact tries to some string problems* (2013, no DOI found). All four
appear in the citing lists; I did not read them.
