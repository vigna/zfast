# Parallel and non-adaptive predecessor search: statements verified on the primary sources

Checked on 2026-11-20. Subject: the paragraph "Parallel and non-adaptive
predecessor search" of `sec:related`, and the paragraphs "Rounds and probes"
and "Lower bounds" of `sec:rounds` ("Fat binary search in rounds").

**Marks.** [P] printed in the PDF. [I] my inference from printed statements.
[G] general knowledge, not verified on any source read here.

**Which versions were read** (PDFs fetched from open repositories and saved in
`../../related/`, untracked).

| # | PDF | What it is | Read |
|---|---|---|---|
| 1 | `Afshani2025Cell.pdf` | Afshani, Sitchinava, *A Cell Probe Lower Bound for the Predecessor Search Problem in PRAM*, SODA 2025, pp. 3998-4008 (author copy from the NSF public access repository; pages are not numbered) | Sections 1-2 and the statement and proof of Theorem 3.1; not the lemmata of Section 3 |
| 2 | `Larsen2024Optimal.pdf` | Larsen, Pagh, Persiano, Pitassi, Yeo, Zamir, *Optimal Non-Adaptive Cell Probe Dictionaries and Hashing*, ICALP 2024, LIPIcs 297, 104:1-104:12 | Section 1 (introduction and statements of Theorems 1 and 2) |
| 3 | `Boninger2017Nonadaptive.pdf` | Boninger, Brody, Kephart, *Non-Adaptive Data Structure Bounds for Dynamic Predecessor Search*, FSTTCS 2017, LIPIcs 93, 20:1-20:12 (author copy) | Section 1 (definitions, statements of Theorems 1, 3, 4 and Corollary 2) |

Not read, and therefore not cited: Ramamoorthy and Rao (CCC 2018; abstract
only), Brody and Larsen (*Adapt or die*), Snir (*On parallel searching*),
Berkman and Vishkin (*On parallel integer merging*), Brodnik et al. (RAMBO).
The page count and the venue of 1 come from a bibliographic database
(researchr.org), not from the PDF; no DOI was verified.

## 1. Afshani and Sitchinava

- [P] Model (Section 2.1): K synchronized processors, each with a constant
  number of registers of w bits; at each unit of time the processors access K
  memory cells of w bits; "we allow the processors infinite computational
  power as well as infinite communication power"; "a running time of t in this
  model implies t rounds of adaptive probing where at each round we probe K
  cells non-adaptively".
- [P] Theorem 3.1: for n integers of l = c log n bits, c > 1 constant, a data
  structure using S(n) <= alpha n words has a worst-case query lower bound of
  Omega(log_K((c^ - 1) log n / (log alpha + log K + log w))) in the K-processor
  CRCW PRAM with word size w, where c^ = min(2, c). Notation (Section 2.2):
  w >= log U.
- [P] Known upper bounds (Section 1.1): comparison-based search takes
  O(log_K n) on a CREW PRAM (their reference [14], Snir). "These results can be
  combined with the x-fast and y-fast tries, in a relatively straightforward
  way": with x-fast tries, O(log_K(log n)) query time on a CREW PRAM; "By
  using y-fast tries and hashing (assuming a static data structures), the
  space can be reduced to linear". No proof or construction is given.
- [P] "When using x-fast tries (or y-fast tries) one simultaneously probes all
  the l nodes in the root-to-leaf path"; the question asked is "whether there
  can be a better trade-off than 'exponential' between computational power and
  increasing non-adaptability"; the answer is that "with K processors, the
  query time can only improve by a factor of log K".
- [P] The paper does not mention z-fast tries, 2-fattest numbers or strings of
  variable length (checked on the whole text and on the reference list).
- [I] For alpha = O(1), w = O(log n) and K <= l the bound is
  Omega(log_K l): the argument of the logarithm is Omega(l / log l).
- [I] The lower bound applies, up to an additive constant, to the location of
  the exit node in linear space, since a static structure locating exit nodes
  can be augmented with pointers to the leftmost and rightmost leaf of each
  node.

## 2. Larsen et al.

- [P] Definition: non-adaptive means that the cells probed on a query depend
  only on the query. "other solutions (e.g., cuckoo hashing) only need one
  round of adaptivity (i.e., first they read the description of the hash
  function, and then read memory cells that are determined only by the query
  and the hash function)"; "a single round of adaptivity is necessary and
  sufficient to reduce the query time from super-constant to constant".
- [P] Theorem 2: a non-adaptive static dictionary for n pairs from [u] x [u]
  using s cells of w bits satisfies
  t = Omega(min{n lg(u/n)/w, lg(u/n)/lg(sw/(n lg(u/n)))}).
- [P] "For standard parameters of u = n^{1+O(1)} and w = Theta(lg u) [...]
  optimal space constructions with s = O(n) require query time
  t = Omega(lg u)".
- [I] The condition used in the paper is u >= n^{1+epsilon}: with u = O(n) the
  formula gives nothing, so "n^{1+O(1)}" must be read in that sense.
- [P] The paper states no lower bound of its own for predecessor search; it
  reports those of 3 and of Ramamoorthy and Rao.

## 3. Boninger, Brody and Kephart

- [P] "A non-adaptive data structure is a cell probe data structure that has
  non-adaptive queries and updates."
- [P] Assumptions: w >= log m, w <= m^{0.25}, set of up to n elements of [m].
- [P] Corollary 2: for a non-adaptive data structure for dynamic predecessor
  with query time t_q and update time t_u, if n >= w/2 then
  max(t_q, t_u) = Omega(log m / log w).
- [P] Theorem 3: there is a non-adaptive structure with
  t_q, t_u = O(log m / log(w / log m)).

## 4. What is claimed in the paper as new

- [I] Theorem `th:probetree` (the probes of a fat binary search descend along
  a branch of a tree that depends only on |x|) and Theorem `th:rounds` are, as
  far as these sources and those of the other reports show, not in the
  literature; `paper/python/check_rounds.py` checks both by simulation with
  arbitrary answers of the z-map.
- [I] The paper does *not* claim that the trade-off between rounds and probes
  is new for integers (source 1 attributes it to x-fast and y-fast tries); it
  claims it for the z-fast trie as is, in linear space without buckets, for
  strings of arbitrary length, and dynamically for the location of the exit
  node. Predecessor queries in the dynamic structure still follow jump
  pointers, one round each.
- [G] Not checked: whether a dynamic y-fast trie can be searched in
  O(log_K l) rounds (it can if buckets are sorted arrays), and the PRAM
  literature on parallel dictionaries and van Emde Boas trees.
