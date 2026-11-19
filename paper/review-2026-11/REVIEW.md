# Z-fast trie project: review record

Written 2026-11-17, covering work done on 2026-11-16 and 2026-11-17. This file
is the master record; it is meant to let a new session (or a co-author) resume
without redoing anything. Details live in the six reports under `reports/`.

## 0. Status and how to resume

**What was asked** (user, 2026-11-16): review the paper, the code, everything,
and suggest improvements; first priority is to improve the Rust implementation;
both a fully dynamic and a static implementation are needed; both must be
byte-based ("much more interesting in practice", e.g. for zuffix trees); get the
best of the ideas scattered over three languages; find applications where the
structure beats compressed suffix arrays or FM-indexes. The user allowed using
the machine `blew` for testing.

**Decision** (user, 2026-11-17): on Thursday 2026-11-19 new repositories will be
created, one for the paper and one for the code, and work starts then. This
answers the one question that was open (fresh crate versus evolving the
MrBrionix repository): fresh repositories.

**State**: review complete; nothing designed or built yet. No design document
exists. The agreed process is: write a short design for the first deliverable,
get it approved, then write an implementation plan, then build.

**What was changed on disk**: `/Users/vigna/pub/zfast/CLAUDE.md` (new,
unversioned), this directory (new, unversioned), two Claude memory notes.
Nothing in any repository was modified. `blew` was only probed (`uname`,
`lscpu`, `free`, tool versions).

**Suggested first steps on Thursday**
1. Create the two repositories. Move or copy this directory into one of them.
2. Paper repository: apply the mechanical fixes of section 3.1 and decide how to
   handle the algorithmic errors (section 3.2).
3. Code repository: write the short design for the core and the static index
   (section 7), get approval, plan, build.
4. First experiment on `blew`: pruned z-map over a plain suffix array against
   plain suffix array, current zuffix and an FM-index (section 5.2, 6.1).

## 1. Inventory of material

| What | Where | Notes |
|---|---|---|
| Paper *Z-Fast Tries* | `/Users/vigna/pub/zfast/dynzfast.tex` | Subversion working copy rooted at `/Users/vigna/pub` (r24015; last change r23968, 2023-04-08). See `../CLAUDE.md`. Generated figures are not versioned (hidden by `global-ignores`) and cannot be regenerated here (`mpost` missing). |
| Notebook | `python/explicitZFast.ipynb` | Explicit static z-fast trie with randomised checks. Has two bugs (section 3.7). A corrected copy made by the referee is `scripts/paper-referee/nbfixed.py` (`nbcode.py` is the original code). |
| C, current | `~/git/zfast` (git, private remote; also checked out on `blew` at `/home/vigna/git/zfast`) | 114 commits 2021-03-04 to 2022-04-12; the first commit is byte-identical to the 2017 snapshot. `zfasttrie.c`, `intzfasttrie.c`, `zcommon.c`, `murmur.h`, baselines `rbtree.c`+`strprb.c`, `intrbtree.c`+`intprb.c` (libavl), `btree.cpp`, `intbtree.cpp` (Google cpp-btree), `comp.sh`, `benchmarks/`. Datasets `test-1000000.urls`, `test-10000000.urls` (eu-2015 URLs) are in the directory. |
| C, old snapshot | `~/svn/c/sux/{zfasttrie,intzfasttrie}.c` | r12379, 2017-03-20. Superseded. Builds on macOS with `cc -std=gnu99 -O3 -DNDEBUG` and passes its self-checks; segfaults when run with no argument. |
| Java | `~/git/Sux4J`: `src/it/unimi/dsi/sux4j/util/ZFastTrie.java`, `src/it/unimi/dsi/sux4j/mph/ZFastTrieDistributor*.java`, tests in `test/` | The original dynamic implementation and its static, succinct relative. |
| Rust, static | <https://github.com/MrBrionix/z-fast-trie-sux> (`static_implementation/`) | Last commit 2025-02-13. Does not resolve (unpinned git `rand`, no lockfile). No `.cargo/config.toml`. |
| zuffix C++ | <https://github.com/smarchini/zuffix> | HEAD 968957e (2023-08-03). Results in its `blew/` directory. |
| zuffix Java prototype | `~/git/zuffix` | 2018; `ZuffixArray.java`, `CyclicHash.java`. Not a paper. |
| *Engineering Zuffix Arrays* (SEA 2024) | `~/git/zarr-exp/zarr.tex` | With `reviews.txt` (referee reports) and `Related/tesi-marchini.pdf`. |
| *Kings, Name Days, Lazy Servants and Magic* (FUN 2018) | `~/svn/tex/zarr/zarr.tex` | Theory of static z-fast tries on arbitrary alphabets. |
| PhD thesis on zuffix arrays | `~/git/thesis/thesis.tex` | Includes the unpublished z-map over sdsl CST++ experiment. |
| `sux-rs` | `~/git/sux-rs` (v0.15.4, edition 2024, MSRV 1.85) | No z-fast code. Has its own `CLAUDE.md`. |
| Rust guidelines | `~/git/rust-dev-guidelines/README.md` | |
| Monotone hashing benchmark | `~/git/MMPHF-Experiments` | Uses Sux4J's `ZFastTrieDistributor` as a contender. |

**Corrections to initial assumptions**: the C code to look at is `~/git/zfast`,
not `~/svn/c/sux`; the zuffix paper is in `~/git/zarr-exp`, not `~/git/zuffix`.

**Environment.** This Mac: arm64, Apple clang 21, MacPorts TeX Live
(`pdflatex`, `latexmk`, `bibtex`, `mptopdf` but no `mpost`), no Jupyter,
`cargo` 1.99.1, `python3` with numpy 2.5.2, Ruby 2.6. The paper builds with
`latexmk -pdf dynzfast.tex`: 30 pages, three known warnings. `blew` (`ssh
blew`): Intel i7-12700KF (8 P-cores with hyper-threading + 4 E-cores, so pin
timing runs to P-cores), 62 GB RAM, Fedora 43, `cargo`/`rustc` 1.99.1, gcc 15.3,
AVX2/BMI2/SSE4.2/PCLMULQDQ, no AVX-512; `/home` had 39 GB free.

## 2. Reports and supporting files

| File | Content |
|---|---|
| `reports/A-paper-referee.md` | Line-by-line referee pass on `dynzfast.tex`: correctness, consistency, completeness, typos. |
| `reports/B-c-code-and-benchmarks.md` | C code 2017 to 2022, memory layout, hot paths, risks, benchmark methodology, paper tables against stored outputs. |
| `reports/C-java-sux4j-and-zuffix-prototype.md` | Sux4J `ZFastTrie`, `ZFastTrieDistributor`, zuffix Java prototype; technique table; suspected bugs. |
| `reports/D-rust-static-crate.md` | Full review of the MrBrionix crate. |
| `reports/E-zuffix-cpp.md` | zuffix C++: representation, z-map, query loop, hashes, benchmark numbers, bugs. |
| `reports/F-literature-and-applications.md` | Sourced literature check: Wormhole, Cuckoo Trie, HOT/ART, range filters, FM-index costs, external memory, read mappers, string dictionaries. |
| `scripts/measurements/` | My two measurement scripts and their outputs (section 5). |
| `scripts/paper-referee/` | The referee's simulation scripts (`zf.py`, `t1.py` … `t9.py`, `gen.py`) and notebook code. Undocumented throwaways. |
| `scripts/c-benchmarks/` | Simulation of the non-uniform shuffle. |
| `scripts/rust-model/` | Python model of the Rust crate used to confirm its bugs. |
| `scripts/zuffix/` | Simulation of the zuffix off-by-one and a table-extraction script. |
| `literature-extracts/` | Text extracts of the 36 papers read for report F (PDFs were not kept; URLs are in the report). |

Paths inside the reports that point into `/private/tmp/.../scratchpad/` refer
to a temporary directory; the surviving files were copied to `scripts/` and
`literature-extracts/` here. The cloned repositories were not kept.

**Verification legend used below.** *[V]* verified by me by reading the source
or running something. *[S]* found by a reviewer through simulation; I checked
the underlying text by reading where noted, but did not rerun the simulation.
*[L]* sourced from the literature by reviewer F; I spot-checked two claims
(Wormhole's all-prefixes design; the C² benchmark row for the z-fast trie).

## 3. Findings

### 3.1 Paper: mechanical defects *[V]*

- `\url` is redefined as `\cite{myurl}` (line 140) and no `url` package is
  loaded: this is the cause of the undefined citation. The URL in that footnote
  (line 1932) says `unini` instead of `unimi`.
- Labels `def:twofattest` (lines 419 and 677) and `fig:nodesbis` are defined
  twice.
- Algorithm LocateExit (line 1074) takes σ as input but its body uses η.
- "Section ??" at line 1637 (the target is `sec:updates`); Conclusions empty
  (line 2025); a sentence is cut off at lines 1934–1935; two `\todo{LINK}`.
- 15 `\todo` notes in all; report A gives a one-line resolution for each.
- Report A, section D, has the full typo list with line numbers; section B has
  the inconsistencies (variable names reused with different meanings, a swapped
  pair in `fig/example-query-parex.tex:267`, a wrong handle at lines 902–903).

### 3.2 Paper: algorithmic and theoretical errors

1. **ParentStack / Theorem `th:parent`** (lines 1755–1790) *[S; text read]*:
   `b ← |e_σ|−1` should be `|n_σ|−1`, as the prose at line 1752 says. As
   written the search interval includes σ's own skip interval; simulated
   failure on 61% of nodes, none with the fix. The proof's formula for the
   previous handle length is also false.
2. **Insertion, Algorithm `algo:jumpupda`** (lines 1539–1565) *[S; text
   read]*: the inner loop guards on "α is internal" while advancing β (α comes
   from the stack and is always internal; β can reach a leaf); `|h|′ < n_λ`
   compares a number with a string; and a step is missing: when x cuts low at
   an internal exit node, that node gets a new handle and its jump pointers
   must be recomputed (852 of 4905 simulated insertions wrong, none after the
   fixes). The proof needs Lemma `lemma:updown`, which is never cited.
3. **Deletion, Algorithm `algo:jumpupdd`** (lines 1567–1590) *[V by
   reading]*: the assumption says "λ used to be a left child", the body then
   tests "if λ is a right child", and the caption says "the parex node was a
   left child". The second loop uses `J^+` and `|e_α|′` where `J^-` and
   `|h_α|′` are meant; `β ← exit x` is dead. Read literally it leaves dangling
   pointers in 34% of deletions *[S]*. The Deletions section (lines 1635–1643)
   is a stub with no theorem or bound.
4. **Enumerative range queries** (lines 1424–1429) *[V]*: "the entire subtree
   under exit x is part of the output" ignores the upper bound.
   Counterexample: S = {000, 001, 010, 011}, x = 0, y = 001. The bound in the
   body (+O(k)) also differs from the abstract and introduction.
5. **Exit node definition** (lines 646–651) *[S]*: no node satisfies it when
   x properly extends a key.
6. **Signatures** (lines 1160–1198): O(1) prefix signatures are incompatible
   with the full-randomness assumption, so a hash family must be named and the
   collision bound redone; duplicate handling is vague (nothing on unmarking
   after deletion).
7. **Bounds**: "at most lg |x|" iterations is off by up to 2; Theorem
   `th:boundslong` gives updates no |x|/w term while the abstract does;
   worst-case query bounds are claimed while a linear-probing table is used.
8. **Space** is counted four different ways (theorems: 12n pointers + n
   integers; Section 9: 11p+2w and 12p+2w; Figure `nodesbis`: 9 pointers + 2
   integers; comment at lines 217–220). The code has an unused 8-byte field
   (`key_length`), so string nodes cost 96 bytes against the paper's 88.
9. **Related work** dates from 2010. To add: Pătraşcu–Thorup (2014), c-trie++
   (section 4.2), Wormhole, the two zuffix papers, and the SPIRE 2010
   conference version of this very paper (`BBVDZT`), which is never cited.

### 3.3 Paper: experiments *[V unless noted]*

- **Swapped columns in both URL tables.** The "red-black tree" column of
  `tab:urlsm` and `tab:urlsb` holds the output of the `btree` binary and the
  "B-tree" column that of `rbtree` (checked against
  `~/git/zfast/benchmarks/{btree,rbtree,zfasttrie}-test-*.urls-std.out`).
  Corrected values, ns per operation, in the order Add, Contains+, Contains−,
  Predecessor, Delete:

  | | z-fast | B-tree | red-black |
  |---|---|---|---|
  | 10⁶ URLs | 1741, 1307, 1421, 1453, 2162 | 1288, 1532, 1579, 1547, 1255 | 1780, 1437, 1814, 1832, 1607 |
  | 10⁹ URLs | 4458, 3444, 3391, 3795, 5622 | 6937, 7261, 8018, 8121, 6798 | 10045, 7686, 8677, 8701, 11652 |

  At 10⁹ the z-fast trie beats the B-tree by 1.56×, 2.11×, 2.36×, 2.14×,
  1.21× and the red-black tree by 2.07–2.56×. At 10⁶ it wins queries by
  7–17% and loses Add (1.35× slower) and Delete (1.72× slower). "More than
  twice faster" at a billion holds for queries only.
- "Within 10% of a B-tree" at 32 billion integers: the table gives the z-fast
  trie 24%, 8%, 17%, 16% and 59% slower.
- "Two times faster than a red-black tree" at 8 billion: 1.60–2.05×.
- "Single-shot allocation, no `free`" is false for the z-fast programs: the
  hash table doubles with allocate-and-free, and the string version allocates
  and frees its hash state on every operation.
- The shuffle in `zcommon.c:75-80` draws `p = next() % (n - i)` and swaps
  `x[i]` with `x[p]`; it should use `i + next() % (n - i)`. The insertion order
  of the integer benchmarks is therefore skewed (simulation in
  `scripts/c-benchmarks/`).
- The timer is user CPU time truncated to 1 ms, single run. The outputs behind
  `tab:intbb` are untracked files in `~/git/zfast`. The B-tree's "predecessor"
  phase is a successor query (`lower_bound(x[i] - 1)`). No hardware, compiler,
  page-size or space information in the paper; table values are truncated, not
  rounded. Many stored runs are unused (10⁷ to 16·10⁹, huge pages). Details in
  report B, sections 5–6.

### 3.4 C code (`~/git/zfast`)

- **Space**: 48-byte nodes. Integers 96–112 bytes per key; strings 128–160
  bytes per key excluding the keys. Baselines: red-black 40, B-tree about 11.
  Confirmed by the 32-billion-key log (3.11 TB).
- **Cost** (inferred from the code, not measured): about 15 cold cache misses
  per integer query at 2³³ keys, about 20 for strings, with four passes over
  the key.
- **Why**: the slot stores no lengths, so the search dereferences a node at
  every probe (integers) or on every signature match (strings); lcp is computed
  twice; hash state is `calloc`ed per call; the table runs at load 0.25–0.5;
  half of the nodes straddle a cache line.
- **Risks**: over-read up to 7 bytes past a query's terminator; unaligned
  type-punned loads; 32-bit bit lengths (keys of 2²⁸ bytes wrap); fixed
  `stack[64]` and `stack[8]`.
- Compile switches worth copying into tests: `SHORT_SIGNATURES` (2-bit
  signatures to force collisions), `NO_DUPS`, `NO_EXACT`, `COUNT_SCANS`,
  `ASSERTS`.

### 3.5 Java (Sux4J)

- `ZFastTrie` is the reference implementation and the richest source of
  techniques (table in report C, summarised in section 6.3).
- Suspected bugs: predecessor/successor off by one leaf when the query
  properly extends a stored key (outside the documented prefix-free contract);
  `isNonempty` on an empty trie dereferences a null root; `removeExisting` and
  `replaceExisting` loop forever on an absent node.
- No test forces signature collisions, so the duplicate-flag and exact-fallback
  paths are effectively untested; the invariant checkers are compiled out.
- `ZFastTrieDistributor`: static function from handle to (extent signature,
  extent length); one "behaviour" bit per key; bucket index through three
  ranker strings per internal node in a monotone hash; mistakes found at
  construction go into a whitelist with a correction function. A root-exit
  corner case looks wrong (medium confidence, not run).

### 3.6 Rust crate (MrBrionix)

- The trie and search logic is correct (confirmed by an independent model).
- The hash layer is broken *[V by reading]*: in `RollingHash` a full word
  contributes the digit `(x + 1) % modulo` and a single bit contributes 1 or 2,
  with no length mixed in, so the one-bit string `0` and sixty-four `0`s
  collide for every seed; `x + 1` overflows on an all-ones word.
- Consequences: `ZFastTrie` keys its map by a 30-bit hash and silently
  overwrites colliding handles (wrong answers); `ZFastTrieSux::build` can loop
  forever; building from zero or one key panics; `Rc` cycles leak every node.
- Cost: O(w) divisions per probe, extent copies in the exact search, about ten
  allocations per query; 180–224 bytes per key.
- Tests: the seven tests in `src/lib.rs` assert nothing *[V]*; prefix queries
  are never called; inputs are uniform random.
- Worth keeping: differential testing against a naive oracle, and the
  observation that internal node i of the trie corresponds to the gap between
  sorted keys i and i+1 (its extent is their LCP), which gives a pointer-free
  static layout.

### 3.7 Notebook

- `search(needsExit=False)` restarts with `a = len(extent)` instead of `+1`
  and returns the wrong parex in 248 of 42,924 queries *[S]*.
- `rangeExistential` does not implement Algorithm `algo:range` and is wrong:
  S = {1000, 11110}, range [10000, 1111) returns True *[S]*.
- It has no jump pointers, updates or ParentStack, so those algorithms in the
  paper never had an executable check.

### 3.8 zuffix C++

- **Space**: SA, LCP and child table all 64-bit (192 bits per symbol), plus a
  z-map of 24-byte slots (signature + interval) with one entry per internal
  node (0.62–0.68 n) in a table at load about 0.5: 438 bits per symbol in
  total, the z-map alone about 6.4 GB for a 200 MB text.
- **Off-by-one** *[V by reading]*: `fatBinarySearch` computes `hlen` from
  `getlcp(...) + 1` while `ZFillByDFS` uses `lcp[r]`; genuine hits are rejected
  whenever e + 1 is fatter than the handle. Correctness is preserved, speed is
  lost; a simulation puts it at one lost hit in about 40% of queries *[S]*.
  The published speedups are probably understated.
- Each probe recomputes name and extent lengths from `lcp` and the child table:
  4–5 extra random reads. A comment in `MemcmpZSdsl.hpp` already notes that
  "storing 'everything' is faster".
- Duplicate signatures are overwritten: with 32-bit CRC32C, 1.6% of nodes are
  lost on dna. The root's CRC signature is 0, which is also the empty marker.
- The exit node is found by walking down with a linear sibling scan. The
  signature variant has no repair path. Only 1-byte symbols actually work. The
  terminator 0xFF is not unique in `english`, which aborts the sdsl runs there.
- **Timings from `blew/20230424`** (ns per query, existing patterns):

  | | m = 10 | 100 | 1000 | 10⁴ |
  |---|---|---|---|---|
  | dna: plain SA | 5608 | 2673 | 3039 | 6829 |
  | dna: enhanced SA | 1498 | 2027 | 2045 | 2232 |
  | dna: zuffix (memcmp, CRC32C) | 731 | 1138 | 1478 | 1784 |
  | english: plain SA | 3898 | 2977 | 3518 | 8102 |
  | english: enhanced SA | 6122 | 6362 | 6458 | 6703 |
  | english: zuffix (memcmp, CRC32C) | 844 | 1174 | 1535 | 2090 |

- **Unpublished sdsl numbers** (`blew/20230516`, `sdsl::cst_sct3`, dna):
  backward search 882 ns at m = 10, 19,740 ns at 100, 189,600 ns at 1000;
  forward search 27,900 / 164,500 / 952,500 ns; forward with z-map 5,448 /
  98,750 / 855,100 ns. Different run and harness from the table above.

## 4. Notes from the zuffix papers, thesis and referee reports

### 4.1 What the papers establish

- FUN 2018 gives the theory of *static* z-fast tries on arbitrary alphabets.
  Key statements: fat binary search "is exactly identical to the binary
  version, since the alphabet has no role in such searches"; it "does not use
  the trie structure: it only queries the z-map". The only alphabet-dependent
  step is the child lookup when the search returns the parex (the "+σ" in
  O((m log σ)/w + log m + σ)). If the z-map also contains the handles of the
  leaves, a search for a key in the set never returns the parex.
- The dynamic, signature-based, jump-pointer machinery exists only for the
  binary case (`dynzfast.tex`). A byte-based dynamic structure with predecessor
  search is not in the literature.
- SEA 2024 lessons: hardware CRC32C is the best hash and the only fast one with
  O(1) substring hashing (linearity plus carry-less multiplication); wyhash is
  as fast when only prefix hashing is needed; signature-only verification is
  slower than `memcmp`; false positives hit under 0.3% of queries with 32-bit
  hashes; z-map size depends on the tree shape. Its conclusion suggests pruning
  the longer handles, because the z-map works on any prefix of the suffix tree.
- The published experiments compare only against the enhanced suffix array.
- Prefix hashing is done by "pause and resume": store the hash state every B
  bytes of the pattern (lazily), resume from the nearest checkpoint.

### 4.2 Prior art that must be cited

**c-trie++** (Tsuruta, Köppl, Kanda, Nakashima, Inenaga, Bannai, Takeda; arXiv
1904.07467) defines *alphabet-aware z-fast tries*: non-binary, dynamic, each
internal node with a child dictionary, but only as word-packed micro tries for
strings of O(w / lg σ) characters under a macro trie, and only for lookup and
prefix search. Its authors ported Sux4J's `ZFastTrie` to C++ ("ZFT"); that port
is what the 2026 C² paper benchmarks: 1192 ns per query and 546.6% of the data
size on Wikipedia titles, against 732 ns and 32.9% for C²-Marisa. These are the
only third-party numbers for a z-fast trie.

### 4.3 Referee reports on *Engineering Zuffix Arrays*

Scores +1, −1, 0. Objections: space ("5 GB for 200 MB data", about 25×);
little algorithm-engineering depth ("the choices of hash functions … are what
naturally come to mind"); large overlap with the FUN paper; too little on how
fat binary search was actually implemented. One referee noted that the z-map
gives "interesting space/time tradeoffs in large alphabets".

### 4.4 Thesis

- Section "A Z-Map for SDSL CST++": a z-map over sdsl's compressed suffix tree
  made forward search 3–4× faster, but on-demand text decompression became the
  bottleneck. Conclusion there: "maintaining a non-compressed text might be a
  strong requirement"; the z-map may be "unsuited to self-indices in general".
  A note admits the benchmark kept the plain text for `memcmp`.
- Folklore recorded there: on real texts the suffix tree is shallow, so a long
  pattern either mismatches early or pins down one suffix, after which the rest
  is a sequential `memcmp`. An FM-index cannot do that: backward search pays
  random-access rank operations for every symbol.
- Future work listed: minimal-perfect-hash z-maps for minimal space; a small
  query-dependent z-map behaving like an LRU cache.

## 5. Measurements made during the review

### 5.1 Byte-level versus bit-level tries

Script `scripts/measurements/trie_stats.py`, output `trie_stats.out`. Keys are
made prefix-free with a 0x00 terminator. Sanity checks passed (the binary trie
has n − 1 internal nodes; leaf counts match).

| | Byte trie: internal nodes per key | Fan-out 2 / ≤4 / >16 (max) | Probes per search | Bit trie: probes per search | Bit trie: byte-aligned handles |
|---|---|---|---|---|---|
| 235,976 English words (avg 10.6 bytes) | 0.527 | 63.7% / 89.8% / 0.4% (52) | 3.34 + 1 child lookup | 5.05 | 52.7% |
| 1,000,000 URLs (avg 79 bytes) | 0.545 | 67.9% / 90.4% / 0.4% (45) | 5.24 + 1 child lookup | 6.65 | 54.5% |
| URLs, every 16th key | 0.445 | 62.6% / 86.3% / 1.2% (37) | 6.04 + 1 | 7.66 | 44.5% |
| URLs, every 64th key | 0.375 | 56.0% / 81.8% / 2.1% (36) | 6.41 + 1 | 8.10 | 37.5% |

A URL has 10.9 internal ancestors on average in the byte trie (28.5 in the bit
trie); a word has 6.4.

**Equivalence.** The fraction of byte-aligned handles in the bit trie equals
the number of byte-trie internal nodes per key, and this is exact: a bit-level
node's handle length is a multiple of 8 if and only if its skip interval
contains a byte boundary; those nodes are the top nodes of the binarisation of
each byte-level branching, hence in bijection with the byte-level internal
nodes; and the handle is the same string (the 2-fattest multiple of 8 in the
bit interval is 8 times the 2-fattest number of the byte interval, because the
bit-level name length lies in [8ℓ′+1, 8ℓ′+8] where ℓ′ is the parent's extent in
bytes). So the byte-level z-map is the bit-level z-map with the sub-byte
entries removed, and the paper's binary theory transfers to bytes through
binarisation.

**Consequences.** Going byte-level removes about 45% of the hash entries and
about 1.4 probes per search (not 3). Fan-out is small on real keys, so a small
sorted child array suffices. With sampled keys the probe count approaches
log₂ |x| because most probes miss; capping the search at the maximum handle
length helps. Fat binary search turns about 11 pointer chases into about 5
hash probes on URLs: a factor of two, not ten. Large gains need deep tries
(suffix trees, long keys with many branch points) or very large n.

### 5.2 Pruned z-map over a plain suffix array

Script `scripts/measurements/sa_stats.py`, output `sa_stats.out`. For each
threshold t: *heavy* = internal suffix-tree nodes whose interval holds at least
t suffixes; *kids* = all children of heavy nodes. Entries per text symbol:

| t | Java sources (1.4 MB) | man pages (3 MB) | URLs (3 MB) |
|---|---|---|---|
| all nodes | 0.775 | 0.596 | 0.456 |
| 16: heavy / heavy + kids | 0.067 / 0.335 | 0.057 / 0.391 | 0.036 / 0.389 |
| 64: heavy / heavy + kids | 0.014 / 0.109 | 0.015 / 0.145 | 0.008 / 0.150 |
| 256: heavy / heavy + kids | 0.003 / 0.037 | 0.003 / 0.049 | 0.002 / 0.053 |
| max extent of a heavy node, t = 64 | 658 | 527 | 232 |

Caveat: small texts; the ratios must be remeasured on the Pizza & Chili corpus.

**The idea.** Index = text + plain suffix array + a hash table holding (a) the
handles of heavy nodes and (b) the names (parent extent plus one byte) of all
children of heavy nodes. No LCP array, no child table. Every slot is
self-contained: signature with the length mixed in, extent length, interval.
Fat binary search needs only an ancestor-closed node set, and heavy nodes are
ancestor-closed. A query does: at most log₂ min(m, max extent) probes; one
`memcmp` against the text at the node's first suffix; one probe for the child
name; then a binary search over fewer than t suffixes.

**Estimates.** At t = 64 with roughly 12-byte entries the table is 1.3–1.8
bytes per symbol (0.5 at t = 256), for a total of about 6–8 bytes per symbol
against 55 for the current zuffix array and 5–9 for a plain suffix array.
Expected cost about 19 cache misses worst case against 35–40 for plain binary
search on 200 MB. Storing children in a compact per-node directory instead of
the hash table would shrink it further. All of this is untested.

## 6. Applications: where it can win

| Application | Strongest competitor | Could win on | Will lose on | Evidence |
|---|---|---|---|---|
| Long patterns in RAM | FM-index, r-index | Time: 17× at m = 100 and 128× at m = 1000 on dna against sdsl backward search (section 3.8) | Space, by two orders of magnitude | Measured, different runs |
| Same, against uncompressed indexes | Plain suffix array, enhanced SA, suffixient arrays, Move-r | 2.3–2.5× over plain SA, 2–7× over enhanced SA | Plain SA has a seventh of the space | Measured; this is why pruning matters |
| Read-mapper seeding | BWA-MEME (118 GB), ERT (60 GB), LISA | Probes per seed; STAR's maximal-mappable-prefix search is the exit-node problem | Index size; SMEM-specific machinery | None direct. Practitioners do accept 10–40× the text for 2–3.5× |
| On-disk text index | SoftMatcha 2, infini-gram, RoSA | Random I/Os per query (table in RAM, SA and text on disk) | RAM for the table | No experiments exist |
| In-memory ordered string index | Wormhole, HOT (about 0.8 µs on 50–100M URLs) | Table size and insert cost: Wormhole hashes every prefix of every anchor, a z-fast trie one handle per node | Lookup parity at best; scans against B-trees | Mechanism sourced, win inferred |
| Static string dictionary | C²-Marisa, PDT, FST (56–82 cache misses per query), LeMonHash-VL | Cache misses; a z-fast index over front-coded blocks needs about 6 probes plus one block | Space against 20–35% of the data | Inferred |
| `RearCodedList` in `sux-rs` | Its own binary search over block heads | log₂(n / ratio) cache-missing comparisons become about 6 probes; adds pred/succ/rank | Small extra space | Read in the source; unmeasured |
| SSTable index blocks, string range filters | RocksDB's experimental trie index; SuRF, Diva | Probe count on long keys | Bits per key | Speculative |

It will not win on small integer sets (the B-tree is faster below billions of
keys), on highly repetitive DNA with very long patterns (the gain disappears at
m = 10⁴), or where space is the constraint. A z-map inside a compressed
self-index is not promising (section 4.4): keep the text uncompressed.

## 7. Design notes for the Rust implementations

### 7.1 Proposed shape

A new crate depending on `sux`, following `~/git/rust-dev-guidelines`, with its
own `.cargo/config.toml` (`target-cpu=native`), in three layers:

1. **Core**: a prefix-hasher trait (per-key state, O(1) prefix hash) and fat
   binary search over self-contained slots, with one final verification and an
   exact fallback.
2. **Static index** over any sorted sequence of byte strings, with nodes as
   rank intervals and a pruning threshold. One implementation, two uses: an
   index over the block heads of a front-coded list, and a zuffix array over
   text plus plain suffix array.
3. **Dynamic byte-level trie** with small sorted child arrays; later, bucketed
   leaves.

Build order: 1 and 2 first (simpler, exercises the hashing, yields the
FM-index experiment and a `RearCodedList` speed-up), then 3.

### 7.2 Constraints from `sux-rs` and the guidelines

- Hashing in `sux-rs` goes through `ToSig` (whole key, xxh3). Fat binary search
  needs the hash of any prefix in O(1) after one pass, so a new abstraction is
  required.
- Expected conventions: `MemSize`/`MemDbg`, optional ε-serde for static
  structures, checked and unchecked method pairs, `into_inner`, the `Succ` and
  `Pred` traits of `traits/indexed_dict.rs` (they return `(index, value)`,
  which fits a static index whose answer is a rank).
- Available building blocks: `VFunc`, `CompVFunc`, `SignedFunc`, `PHastR`,
  `LcpMmphf`, `Lcp2Mmphf`, `RearCodedList` (plain and memory-mapped),
  `BitFieldVec`, `sig_store`.
- Guidelines: modules as directories with short names, `impl Trait`
  parameters, slices instead of `&Vec`, tests named `test_*` returning
  `anyhow::Result`, actual value before expected value in assertions,
  `slow_tests` feature, `CHANGELOG.md`, clippy-clean with all features.

### 7.3 Techniques to carry over, by source

**Search**
- Self-contained slots: signature (with the handle length mixed in), extent
  length, payload. No dereference during the search. (C review; zuffix.)
- Verify once at the end against one key, then rerun exactly on failure.
  (Java, C, zuffix.) A false positive cannot cause a wrong answer provided the
  table has no false negatives, so duplicates must never be dropped.
- Mask-shift loop for successive 2-fattest numbers: `f = b & mask` is
  2-fattest iff `(a & mask) != f`; one shift per iteration. (Java, zuffix.)
- Cap the search at the maximum handle length; start from the root without
  probing. (zuffix, C review.)
- Strict containment of rank intervals as a free false-positive filter.
  (zuffix.)
- Optionally also store leaf handles, so that members never need a child
  lookup. (FUN paper.)

**Hashing**
- Checkpointed prefix hashing with scratch state owned by the caller, never
  allocated per query. (zuffix, C review.)
- Hardware CRC32C (also on arm64) or a wyhash-style function; at least 64 bits,
  or a duplicate bit with handle comparison. Mix the length in; never let a
  valid signature equal the empty marker. (zuffix bugs.)
- On updates, resume hashing from the query's state at the shared prefix.
  (Java.)

**Dynamic structure**
- Borrowed extents: internal nodes store no bytes, each points to a descendant
  leaf, kept bijective in O(1) per update. (Java.)
- In-place owner swap when a handle moves to another node; backward-shift
  deletion without tombstones; the `cutLow` test to decide which node changes
  handle. (Java.)
- Byte-level nodes with small sorted child arrays (section 5.1).
- Bucketed leaves with tag arrays and direct leftmost/rightmost leaf pointers,
  as in Wormhole, to bring the per-key overhead down from 100+ bytes. Jump
  pointers remain the worst-case-efficient alternative from the paper.

**Static structure**
- Nodes as rank intervals; internal node i corresponds to the gap between
  sorted keys i and i+1. (zuffix, Rust crate.)
- Z-map as a static function from handle to (extent signature, extent length)
  with a construction-time correction list (`ZFastTrieDistributor`), as the
  space-minimal alternative to a table.
- Pruning by subtree size (section 5.2).

**Throughput**
- Prefetch both possible next slots at each step, or interleave a batch of
  queries: each step is a single table probe. (Cuckoo Trie and BWA-MEM2 show
  how much this matters.)

**Testing**
- Differential tests against a naive oracle with adversarial generators: runs,
  long shared prefixes, queries equal to, prefixes of and extensions of keys,
  empty and single-key sets.
- A forced-collision mode (tiny signatures) so that the duplicate and fallback
  paths are exercised; a deliberately bad hash for differential tests.
- A full invariant checker callable after every update; shape extremes
  (complete tries, caterpillars) inserted and removed in both orders.

### 7.4 Bugs seen elsewhere, to avoid

Dropping or overwriting duplicate signatures; signature 0 colliding with the
empty marker; hash without length mixing; per-query allocation; recursion as
deep as the trie; fixed-size stacks without checks; reading past a key's end in
word-at-a-time loops; 32-bit lengths; a table sized for one load factor and
grown at another; tests that assert nothing; benchmarks replaying ten cached
queries; non-uniform shuffles; unpinned git dependencies without a lockfile.

### 7.5 Benchmark plan

- **Dynamic**: against cpp-btree or abseil, ART, HOT, Wormhole, `BTreeMap`;
  datasets URLs (eu-2015, DBpedia, CommonCrawl), Wikipedia titles, emails;
  report space per key as well as time.
- **Static dictionary**: against `RearCodedList` binary search, FST, PDT,
  C²-Marisa.
- **Full text**: Pizza & Chili (dna, english, dblp-xml, sources, proteins)
  against plain suffix array, enhanced suffix array, the current zuffix code
  (with its off-by-one fixed), and an FM-index (sdsl, and a modern
  cache-friendly one); report bits per symbol.
- Method: wall-clock time, several runs, pinned P-cores on `blew`, uniform
  shuffles, hardware and compiler recorded.

## 8. Decisions still open

1. Whether the paper's algorithmic errors (section 3.2) are fixed in the
   existing text or the dynamic part is rewritten around the byte-level
   structure.
2. Whether the dynamic implementation follows the paper (one key per leaf, jump
   pointers) or the bucketed design first.
3. Whether the static pieces go upstream into `sux-rs` once stable.
4. Signature width and hash function, to be settled by measurement.
