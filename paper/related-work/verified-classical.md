# Classical papers: statements verified on the primary sources

Checked on 2026-11-20 against the PDFs in `../../related/`. Subject: the
papers of section 2 of `TO-DOWNLOAD.md`, the journal versions of the two
hashing papers that `verified-hashing.md` read as conference versions, and
the two papers behind open points (Brodnik, Miltersen, Munro for the `\todo`
at line 2084; Dietzfelbinger and Meyer auf der Heide for the dictionary).

**Line numbers** refer to `dynzfast.tex` as it was at the time of the check
(2637 lines). Every proposed edit quotes the old text: locate edits by text.

**Marks.** [P] printed in the PDF (page numbers are those printed on the
pages). [I] my inference from printed statements. [S] taken from a secondary
source, which is named. [G] general knowledge, not verified here.

**What was read.** All fourteen PDFs have a text layer and are legible; six
are scans (van Emde Boas 1975 and 1977, Willard, Mehlhorn-Naher,
Fredman-Willard, Fredman-Saks, Karp-Rabin), read as page images. No PDF is a
different paper from the one expected.

| PDF | What it is [P] | Pages read |
|---|---|---|
| `PreservingOrderForest.pdf` | FOCS 1975 paper, pp. 75-84 (no venue printed on the pages; IEEE Xplore stamp) | 75-79, 83-84 (not Section 5.2 onwards to Section 6, pp. 80-82) |
| `PreservingOrder.pdf` | Inform. Process. Lett., Volume 6, number 3, June 1977, pp. 80-82 | all |
| `DesignImplementationPriorityQueue.pdf` | "Mathematical Systems Theory 10, 99-127 (1977)" | 99-101; the rest searched in the text layer |
| `LogLogarithmic.pdf` | "Information Processing Letters 17 (1983) 81-84", Volume 17, Number 2, 24 August 1983 | all |
| `BoundedOrdered.pdf` | "Information Processing Letters 35 (1990) 183-189", Volume 35, Number 4, 7 August 1990 | all |
| `FusionTrees.pdf` | "Journal of Computer and System Sciences 47, 424-436 (1993)" | 424-426, 430-432, 434-436 |
| `CellProbeComplexityt.pdf` | STOC 1989, pp. 345-354 ("(c) 1989 ACM 0-89791-307-8/89/0005/0345") | 345-347 (statements only) |
| `LowerBoundsPredecessor.pdf` | "Journal of Computer and System Sciences 74 (2008) 364-385" | 364-367 (statements only) |
| `OptimalStaticRange.pdf` | "STOC'01, July 6-8, 2001, Hersonissos, Crete, Greece", pp. 476-482 | 476-477; theorems located in the text layer |
| `Transdichotomous.pdf` | WADS 1997 paper, pp. 426-439 (venue not printed; first page unnumbered, second is 427) | all |
| `NewUniversalClass.pdf` | ICALP 1990 paper, pp. 6-19 (venue not printed; second page is 7) | 6-8, 16-18 |
| `LinearProbingConstantIndependence.pdf` | **SIAM J. Comput. Vol. 39, No. 3, pp. 1107-1120**, (c) 2009 | 1107-1116, 1118-1120 |
| `PowerSimpleTabulation.pdf` | **J. ACM 59, 3, Article 14 (June 2012), 50 pages** | 14:1-14:5, 14:20-14:21; the rest searched in the text layer |
| `EfficientRandomized.pdf` | "IBM J. Res. Develop. Vol. 31 No. 2 March 1987", pp. 249-260 | 249-254, 257-260 |

## Summary

**Wrong or imprecise in `dynzfast.tex`:**

1. Lines 421-423, y-fast tries "at the price of amortized bounds for
   updates": **imprecise**. Willard's paper is static: it proves only
   worst-case bounds for queries, gives no update algorithm, and says only
   that "it should be possible to guarantee expected cost log log N". The
   amortized randomized update bound comes from combining the structure with
   dynamic perfect hashing (so stated by Fredman and Willard, p. 425, and
   proved for their own structure by Mehlhorn and Naher).
2. Lines 426-427, "fusion trees answer queries in time O(lg n / lg w)":
   **imprecise as an attribution**. Fredman and Willard state O(log N / log
   log N) *amortized* time per operation; the form O(lg n / lg w) is the later
   restatement (static case).
3. Lines 384-391, dictionaries with constant time w.h.p.: **incomplete, and
   one sentence is misleading**. The first such dictionary is that of
   Dietzfelbinger and Meyer auf der Heide, and it already uses space linear
   in the *current* number of keys; the text suggests that removing the
   bound N required iceberg hashing.
4. `biblio.bib`: `EKZDIEPQ` has the wrong journal name and year (printed:
   Mathematical Systems Theory 10, 99-127 (1977)); `Patrascu2012Power` has
   `pages = {1--50}` for what is Article 14.

**Correct, with a refinement available:**

5. Lines 2082-2084 (the `\todo`): the statement is **correct**. It is
   Theorem 15 of Brodnik, Miltersen and Munro. The `\todo` can be removed;
   a sharper wording is proposed.
6. Lines 315-316 and 2082, constant-time most significant bit with
   multiplication attributed to Fredman and Willard: **correct** (Section 5,
   pp. 431-432).
7. Everything else attributed to van Emde Boas, Willard, Mehlhorn and Naher,
   Pagh-Pagh-Ruzic and Patrascu-Thorup: correct.

**Not cited at all in `dynzfast.tex`:** Fredman-Saks, Sen-Venkatesh,
Alstrup-Brodal-Rauhe, Karp-Rabin, Dietzfelbinger-Meyer auf der Heide, and the
two single-author papers of van Emde Boas. The statements that the surveys
make about them are checked below.

---

## 1. van Emde Boas, "Preserving order in a forest in less than logarithmic time" (FOCS 1975)

**Version** [P]: P. van Emde Boas, Mathematical Centre, Amsterdam, and
Mathematical Institute, University of Amsterdam; pages numbered 75-84. The
proceedings are not named on the pages; Willard's reference [8] [P] reads
"Proc. 16th Ann. Symp. on the Foundations of Computer Science (1975) pp.
75-84".

**BibTeX.** Not in `biblio.bib`. `vanEmdeBoas1975Preserving` in `theory.bib`
is consistent with the above (the DOI is not printed). Entry at the end.

**What it contains** [P]:

- Abstract: "We present a data structure, based upon a stratified binary
  tree, which enables us to manipulate on-line a priority queue whose
  priorities are selected from the interval 1...n, with an average and worst
  case processing time of O(log log n) per instruction."
- p. 75: "a worst case processing time of O(log log n) per instruction, on a
  Random Access Machine. The storage requirement is of the order O(n log log
  n)." Here n is the size of the *universe*.
- The repertoire (p. 75-76) includes INSERT, DELETE, MEMBER, MIN, MAX,
  PREDECESSOR, SUCCESSOR and NEIGHBOUR: "the complete instruction repertoire
  given above is supported on our data structure with a worst and average
  case processing time of O(log log n) per instruction".
- **The recursion on the universe** (Section 3.2, p. 77): "The universe
  [1...n] is divided into sqrt(n) blocks of size sqrt(n). Each block is made
  a priority queue of size sqrt(n), whereas the blocks themselves form
  another priority queue of this size. [...] the above description yields
  for the run-time a recurrence equation of the type T(n) <= T(sqrt(n)) + 1
  which has as a solution T(n) <= O(log log n)."
- **Binary search on the levels** (p. 77): "If these traversals could be
  executed by means of a 'binary search on the levels' strategy, the
  processing time is reduced from O(k) to O(log k) = O(log log n)."
- The structure actually described (Section 4) is the *stratified tree*: a
  fixed binary tree on the universe decomposed into "canonical subtrees",
  with father pointers by rank and fields `lmin`, `lmax`, `rmin`, `rmax` at
  internal nodes (pointers to the leftmost and rightmost present leaves of
  the two halves of a canonical subtree).
- **No multiplication** (p. 79): the static structure "is initialized
  without having 'illegitimate' instructions like multiplications and
  bit-manipulations".
- Acknowledgements: the algorithms in their final state are due to R. Kaas
  and E. Zijlstra's implementation.

**Sentences of `dynzfast.tex`.** None cites this paper; all statements on van
Emde Boas trees cite `EKZDIEPQ` (see paper 3). The first publication of the
structure and of the O(lg lg u) bound is this one [P: Willard cites both, and
the 1977 journal paper has the same abstract]. Optional edit 6 adds the
citation.

- Line 478-479, "The idea of locating the longest prefix [...] by a binary
  search on its length, using hashing to test whether a prefix appears, goes
  back to x-fast tries": **correct** as written, because of "using hashing".
  The binary search on the levels is van Emde Boas' (quotation above), and
  Willard says so (paper 4). A parenthesis is proposed as optional edit 7.
- Lines 423-424 and 499, "the recursion of van Emde Boas": **correct**; the
  1977 note calls it "the fundamental divide and conquer trick on which the
  structure Q is based" [P, p. 81].

## 2. van Emde Boas, "Preserving order in a forest in less than logarithmic time and linear space" (IPL 1977)

**What it is** [P]: a three-page note, "Volume 6, number 3, Information
Processing Letters, June 1977", pp. 80-82, "Received 9 March 1977"; P. van
Emde Boas, ITW/VPW, University of Amsterdam. It is *not* a version of the
FOCS paper: it reduces the space of the structure of paper 3 (its reference
[2], "Math. Systems Theory, to appear").

**BibTeX.** Not in `biblio.bib`; `vanEmdeBoas1977Preserving` in `theory.bib`
is correct in every field that is printed.

**Content** [P]:

- p. 80: the structure of [2] "requires more than linear space. To be
  precise the storage needed is of order u log log u".
- "Theorem 3. There exists a datastructure R(u) supporting the full
  repertoire in space O(u) with O(log log u) processing time per element
  processed." "Linear" is linear in the universe u, not in the number of
  keys.
- Method (Section 2, "Two-level priority queues"): the universe is cut into
  n "galaxies" of size m = ceil(log log k), with u = k ceil(log log k) and
  n = k; a top stratified tree on the nonempty galaxies and a bottom
  structure for each galaxy; "we might have used for the bottom queues even
  a structure like an unsorted list".
- p. 82: "Since division is not a permitted instruction on a RAM, we use a
  precomputed table".

**Relevance.** This is the origin of the two-level organisation ("buckets")
later applied to the set instead of the universe: Willard, p. 83 [P]: "Van
Emde Boas [9] has noted that the memory space of stratified trees can be
reduced from Theta(M log log M) to Theta(M) by pruning the bottom of the data
structure [...] Now we will apply this technique to x-fast tries"; Mehlhorn
and Naher, p. 185 [P]: "It is easy to reduce the storage requirement to O(n)
as described by van Emde Boas."

**Sentences of `dynzfast.tex`.** None. Line 417-418, "but use space depending
on the size of the universe": **correct** (O(u lg lg u) in papers 1 and 3,
O(u) here).

## 3. van Emde Boas, Kaas, Zijlstra, "Design and implementation of an efficient priority queue"

**Version** [P]: "Mathematical Systems Theory 10, 99-127 (1977)", "(c) 1977
by Springer-Verlag New York Inc."; "Received December 29, 1975 and in
revised form June 7, 1976". The text is the FOCS paper with the PASCAL
program added (same introduction, same sections).

**BibTeX** (`EKZDIEPQ`): **two fields are wrong.** `journal = {Theory
Comput. Systems}` is the later name of the journal: the paper prints
"Mathematical Systems Theory". `year = {1976}`: the paper prints 1977 (twice),
and Willard, Mehlhorn-Naher, Fredman-Willard and Waldvogel et al. all cite
"Math. Systems Theory 10 (1977) 99-127" [P]. The issue number is not printed
(1 according to Crossref [S: `theory.bib`]). Corrected entry at the end.

**Content** [P]:

- Abstract: "a data structure, based upon a hierarchically decomposed tree,
  which enables us to manipulate on-line a priority queue whose priorities
  are selected from the interval 1, ..., n with a worst case processing time
  of O(log log n) per instruction."
- p. 100: "a priority queue with a worst case processing time of O(log log
  n) per instruction, on a Random Access Machine. The storage requirement is
  of the order O(n log log n) RAM-words."
- p. 100: "the theoretical objection that the time is measured using the
  uniform measure on a RAM".
- p. 101, Section 2.2: "the complete instruction repertoire given above
  [including PREDECESSOR and SUCCESSOR] is supported on our data structure
  with a worst case processing time of O(log log n) per instruction".
- p. 110: no "'illegitimate' instructions like multiplications and
  bit-manipulations".

**Sentences of `dynzfast.tex`:**

| Line | Text | Verdict |
|---|---|---|
| 185-186 | "inspired by the research started by the van Emde Boas trees~\cite{EKZDIEPQ} and followed by the development of x-fast tries and y-fast tries~\cite{WilLLWRQ}" | Correct. Willard [P, p. 82]: "Many aspects of our algorithm and data structure will have analogs from the work of Van Emde Boas, Kaas and Zijlstra". |
| 232-234 | "starting from the introduction of van Emde Boas trees~\cite{EKZDIEPQ} and fusion trees~\cite{FrWSITBFT}, which provide asymptotically faster alternatives for large collections" | Correct. |
| 243-244 | "variants of the van Emde Boas tree~\cite{EKZDIEPQ} that answer in time $O(\lg\ell)$ are optimal" | Correct as far as this paper goes (the optimality is checked in `verified-theory.md`). |
| 416-418 | "For $n$ integers of $\ell\leq w$ bits, van Emde Boas trees~\cite{EKZDIEPQ} answer predecessor queries and perform updates in time $O(\lg \ell)$, but use space depending on the size of the universe." | Correct: O(log log n) worst case, n = 2^l the universe; O(n log log n) words. |
| 481-482 | footnote: Waldvogel et al. mention "the stratified trees of van Emde Boas~\cite{EKZDIEPQ}" | Correct; they cite both the FOCS paper and this one [P: `ScalablePrefixMatching.pdf`, "stratified trees [van Emde Boas 1975; van Emde Boas et al. 1977]"]. |

## 4. Willard, "Log-logarithmic worst-case range queries are possible in space Theta(N)"

**Version** [P]: "Information Processing Letters 17 (1983) 81-84", "Volume
17, Number 2", "24 August 1983"; Dan E. Willard, Bell Laboratories;
"Communicated by K. Mehlhorn. Received 24 November 1982. Revised 17 February
1983". The title prints Theta(N).

**BibTeX** (`WilLLWRQ`): correct in every field.

**Model** [P]: "a random access machine" (abstract); footnote 1, p. 82: "The
assumption in [1] and in all the other articles we have cited is that all
arithmetic operations on two nonnegative integers <= M can be performed in
time Theta(1)." M is the bound on the keys, N the number of keys.

**x-fast trie** [P, Section 2, pp. 82-83]:

- A binary trie of height h on the keys (M = 2^h - 1), storing only nodes
  with COUNT(v) > 0; the leaves form "a doubly-linked list"; "If v is a node
  with no left (respectively right) son, then DESCENDANT(v) will be a pointer
  to the leaf with the smallest (respectively largest) key descending from
  it."
- The "level-search structure (LSS)" uses the static scheme of Fredman,
  Komlos and Szemeredi: "a data structure which uses Theta(n) space and
  executes FIND-queries in worst-case time Theta(1) [...] We will use h + 1
  copies of this search structure. The jth copy, denoted LSS(j), will
  represent the set of trie-nodes of height j with their identifiers serving
  as keys."
- "Lemma 1. The x-fast trie makes it possible to find the node BOTTOM(K) in
  worst-case time Theta(log log M)", by "a binary search among the h + 1
  different LSSs" (Fig. 2, "A binary search of the trie-levels for finding
  BOTTOM(K)"); BOTTOM(K) is "the lowest ancestor of [K] stored in the trie".
- "Lemma 2. x-fast tries have a worst-case overall-retrieval complexity
  Theta(log log M) and never occupy more memory than O(N log M)."
  ("Overall-retrieval" covers FIND, SUCCESSOR, PREDECESSOR and SUBSET, the
  latter in time proportional to "f(M) plus the size of the retrieved
  subset".)
- Credit to van Emde Boas et al. (p. 82): "they used analogs of
  descendant-pointers and also employed a binary search similar to that
  described in the next paragraph for finding BOTTOM(K). However, our method
  uses memory space much more economically."

**y-fast trie** [P, p. 83]: "Define an L-separator of the set S, denoted
S_L, as the subset of S including S's largest element, its (L + 1)st smallest
element, its (2L + 1)st smallest element, etc. [...] a y-fast trie of order L
will be defined as a two-part data structure whose top half is an x-fast trie
representing the set S_L, whose bottom half is a forest of binary trees of
height ceil(log L) where the ith tree describes the set S*_i [...] Lemma 3
follows by applying these observations to the case L = log M." "Lemma 3.
y-fast tries make possible a worst-case overall-retrieval complexity
Theta(log log M) within memory space Theta(N)."

**Updates** [P]: there is no update algorithm and no lemma on updates. The
only statements are:

- p. 82: "Our data structure has a good expected insertion-deletion time, but
  it does not control the worst case of insertions and deletions."
- p. 83 (Open questions): "One problem is that y-fast tries do not have a
  good worst-case insertion-deletion time, although it should be possible to
  guarantee expected cost log log N."

The words "hash", "bucket" and "amortized" do not occur in the paper (text
layer of a scan, and my reading of the four pages). The hash tables are the
static perfect hash tables of Fredman, Komlos and Szemeredi, so queries are
deterministic and worst-case; dynamic perfect hashing did not exist in 1983
[I: it is reference [1] of Mehlhorn and Naher, FOCS 1988].

**Sentences of `dynzfast.tex`:**

| Line | Text | Verdict |
|---|---|---|
| 245-250 | "Willard's \emph{y-fast trie}~\cite{WilLLWRQ} and Mehlhorn and N\"aher's structure~\cite{MeNBOD}, which uses dynamic perfect hashing~\cite{DKMDPHULB}. These two solutions are both two-level data structures using buckets; within each bucket some other standard data structure is used to answer queries." | Correct (binary trees in Willard; linked lists with linear search in Mehlhorn-Naher). |
| 418-420 | "Willard's \emph{x-fast trie}~\cite{WilLLWRQ} stores all prefixes of all keys in hash tables, one for each prefix length, and finds the longest prefix of a query that is a prefix of some key by a binary search on the prefix length" | Correct (LSS(j), j = 0..h; BOTTOM(K)). |
| 421-423 | "the \emph{y-fast trie}~\cite{WilLLWRQ} brings space down to linear by storing the keys in buckets of $\Theta(\ell)$ consecutive keys and indexing one key per bucket with an x-fast trie, at the price of amortized bounds for updates." | First part correct (L = log M). **"at the price of amortized bounds for updates" is imprecise**: Willard gives no update bound, and what he conjectures is an *expected* cost; nor does his x-fast trie have update bounds to be compared with. See edit 2. |
| 425-426 | "All these structures reduce the \emph{length} of the keys, and their time bounds do not depend on $n$." | Correct. |
| 453 | "once again, y-fast tries (with dynamic fusion nodes in the buckets)" | Consistent with Willard's definition. |
| 478-479 | "goes back to x-fast tries~\cite{WilLLWRQ}" | Correct (see paper 1). |
| 483-484 | "x-fast tries achieve this by storing \emph{all} prefixes of all keys" | Correct. |
| 588-589 | "the organization of Wormhole is that of a y-fast trie [...]: buckets of keys, and a structure storing all prefixes of one representative per bucket" | Correct description of a y-fast trie. |

**Not in the paper, possibly worth a remark** [I]: Willard's DESCENDANT
pointers, and the `lmin`/`rmax` fields of van Emde Boas, are the ancestors of
the pointers to the leftmost and rightmost leaf that the z-fast trie reaches
through jump pointers (Section 4 of `dynzfast.tex`).

## 5. Mehlhorn, Naher, "Bounded ordered dictionaries in O(log log N) time and O(n) space"

**Version** [P]: "Information Processing Letters 35 (1990) 183-189", "Volume
35, Number 4", "7 August 1990"; Kurt Mehlhorn and Stefan N\"aher, Universit\"at
des Saarlandes; "Received 22 August 1989. Revised 27 March 1990".

**BibTeX** (`MeNBOD`): correct in every field.

**Result** [P, p. 183]: "Theorem 1.1. Let N be a prime. Then a bounded
ordered dictionary D on [0...N - 1] can be implemented such that operations
init, defined, lookup, size, min, max, successor and predecessor take time
O(1) and insert, delete and locate take time O(log log N). The space
requirement is linear in the size of D. The O(1) time bounds and the time
bound for locate are worst case, the bounds for insert and delete are
amortized and randomized, i.e., the averaging involved in the analysis is
over choices made by the algorithm and not over the input sequence."
(`successor` and `predecessor` apply to elements of the set; `locate(x)`
returns the smallest element not smaller than x.)

**How** [P]:

- p. 184: "Our solution is based on van Emde Boas' stratified trees. The O(N)
  storage requirement of stratified trees results from the use of arrays of
  size N. [...] We use dynamic perfect hashing [1] instead of arrays." [1] is
  the FOCS 1988 version of `DKMDPHULB`. "The model of computation is the
  unit-cost RAM where the arithmetic operations +, -, *, mod, div on integers
  of value at most N take unit time."
- The structure is the recursion on the halves of the key (`top` of order
  ceil(k/2), `bot` a dictionary of pointers to structures of order
  floor(k/2)); "Lemma 2.2. A stratified tree for a set of size n uses space
  O(n log log N)."
- **Buckets** (pp. 185-186): "It is easy to reduce the storage requirement to
  O(n) as described by van Emde Boas. We only have to divide S into segments
  of size between log log N and 4 log log N. Segments are stored as linked
  lists and only the first element of each segment is stored in the
  stratified tree." The buckets have Theta(log log N) = Theta(lg l) keys, not
  Theta(l).

**Sentences of `dynzfast.tex`:**

| Line | Text | Verdict |
|---|---|---|
| 246-247 | "Mehlhorn and N\"aher's structure~\cite{MeNBOD}, which uses dynamic perfect hashing~\cite{DKMDPHULB}" | Correct. |
| 423-424 | "Mehlhorn and N\"aher~\cite{MeNBOD} obtain similar results from the recursion of van Emde Boas using dynamic perfect hashing~\cite{DKMDPHULB}, and need buckets to attain linear space." | Correct. "Similar results" is precise only after edit 2: worst-case O(lg l) for locate, amortized randomized O(lg l) for updates. |

## 6. Fredman, Willard, "Surpassing the information theoretic bound with fusion trees"

**Version** [P]: "Journal of Computer and System Sciences 47, 424-436
(1993)"; "Received October 23, 1990; revised September 2, 1991".

**BibTeX** (`FrWSITBFT`): correct. The issue number (3) is not printed.

**The bound** [P]:

- p. 425: "Fusion trees accommodate dynamic search operations in amortized
  time O(log N / log log N) per operation on a random access machine with
  b-bit word size, whose instruction set includes addition, subtraction,
  multiplication, comparison, and bit-wise AND operations. With the use of
  fusion trees, we can construct a linear space O(N log N / log log N) worst
  case time-sorting algorithm." (Search operations return the predecessor,
  p. 424.)
- p. 426: a B-tree whose leaves are "weight balanced binary search trees of
  size approximately B^4", which increases "the search times only by an
  additive amount of log B. [...] Since we will be able to maintain log B =
  Theta(log log N) using routine methods, we achieve a search tree whose
  operations can be performed in O(log N / log log N) amortized time."
- p. 430: fields of the word: "E = floor(b^{1/6})", "the maximum allowable
  degree B of a node is constrained to satisfy B <= E"; "the total amount of
  space required to represent the fusion tree is O(N)".
- p. 434 (Section 6): "by suitably refining our fusion tree structure, we
  obtain the worst case as opposed to amortized time bounds" (no proof); with
  randomization and integer division, "sqrt(lg N) amortized time bounds for
  dynamic search operations", using "dynamic Y-fast tries [3, 6]" for large
  N.

The bound **O(lg n / lg w) is not printed** in the paper. It follows for a
static tree of degree b^{1/6} [I], and it is how later papers report the
result [S: `verified-theory.md`, section 4.3: Patrascu and Thorup, SODA 2007,
p. 555, "fusion trees, which requires linear space and can answer queries in
O(log_l n) time"]. For the dynamic structure the paper's own analysis gives
O(log N / log B + log B) [P, p. 426 and p. 435], which is O(lg n / lg w) only
when lg w = O(sqrt(lg n)) [I]; dynamic O(lg n / lg w) is the result of
`Patrascu2014Dynamic`, as line 437-438 of `dynzfast.tex` says.

**Most significant bit** [P, Section 5, pp. 431-432]: "Our next task is to
describe the constant time computation of msb(u, v), the most significant bit
position in which the b-bit quantities u and v differ. [...] Our computation
trivially reduces to determining the position of the leftmost one in u XOR v,
which we can write as floor(lg(u XOR v)) [...] The computation for floor(lg
x) is divided into two phases." It uses Lemma 3 (compression of a d-sparse
word by one multiplication and one AND), blocks of s = ceil(sqrt(b + 1))
bits, and constants that depend on b only (C_1, C_2, the fused powers of
two). So **the claim "computing the most significant bit requires constant
time with multiplication" can be attributed to Fredman and Willard, Section
5**. Three other papers read here attribute it in the same way [P]:

- Alstrup, Brodal, Rauhe, p. 477: "Theorem 1 (Fredman and Willard [13]).
  Given a w bit word x, the index msb(x) can be computed in constant time,
  provided a constant number of words is known which only depend on the word
  size w." ([13] is this paper.)
- Brodnik, Miltersen, Munro, p. 431: "When multiplication is allowed, both
  operations can be done in constant time [13, 7, 3]" ([13] is this paper),
  and p. 430: "Fredman and Willard's fusion tree [13] is another example of a
  weakly non-uniform algorithm".
- Patrascu and Thorup, `Patrascu2014Dynamic` [S: arXiv:1408.3045v2, from a
  text extraction made in an earlier session; there is no PDF of it in
  `related/`]: "Fredman and Willard [FW93] showed how to implement this in
  constant time using multiplication [...] we avoid the universal constants
  depending on w used in [FW93]."

**Other statements used above** [P]: p. 425: "An alternative is Willard's
Y-fast tries [6] in conjunction with the dynamic perfect hashing methods of
Dietzfelbinger et al. [3], which brings the space down to linear in N, and
accommodates priority queue operations in amortized randomized time log log U
per operation"; p. 435: "the Y-fast trie in conjunction with dynamic perfect
hashing [3, 6] provides a dynamic linear space data structure for searching
with randomized amortized O(log log U) time operations." Pp. 435-436: the
instructions used "belong to the class NC^1", and "our algorithms use
multiplication, which does not belong to AC^0".

**Sentences of `dynzfast.tex`:**

| Line | Text | Verdict |
|---|---|---|
| 232-234 | fusion trees "provide asymptotically faster alternatives for large collections" | Correct. |
| 315-316 | "computing the most significant bit of a word, which requires constant time with multiplication~\cite{FrWSITBFT}, can be avoided" | Correct. Pinpoint: Section 5 (edit 4). |
| 426-427 | "fusion trees~\cite{FrWSITBFT} answer queries in time $O(\lg n/\lg w)$" | **Imprecise**: see above and edit 3. |
| 2082 | "With multiplication, $\lambda$ can be computed in constant time~\cite{FrWSITBFT}" | Correct. |

## 7. Fredman, Saks, "The cell probe complexity of dynamic data structures"

**Version** [P]: Michael L. Fredman, Michael E. Saks; "(c) 1989 ACM
0-89791-307-8/89/0005/0345"; pages 345-354. The proceedings are not named on
the pages (STOC 1989 from the ACM code [I]).

**BibTeX.** Not in `biblio.bib`; `Fredman1989Cell` in `theory.bib` is
consistent (DOI not printed).

**Statements** [P, p. 346]:

- "Subset Rank. This problem concerns the representation of a subset S of U =
  {1, 2, ..., n}. The operations that must be supported are the updates
  'insert item j into the set' and 'delete item j from the set' and the
  queries rank(j)".
- "Theorem 1. If b <= (log n)^t for some t, then any CPROBE(b) implementation
  of either list representation or the subset rank requires Omega(log n / log
  log n) amortized time per operation."
- "Theorem 3. Any CPROBE(b) implementation of PS(n, 2) (partial sums mod 2)
  requires Omega(log n / (log log n + log b)) amortized time per operation",
  and: "a lower bound on PS(n, 2) is also a lower bound for both list
  representation and subset rank (the details [...] are omitted from this
  report)".
- p. 347: "the dynamic subset rank problem requires Omega(log n / log log n)
  time per operation, yet the algorithm in [16] solves the predecessor
  problem in time log log n per operation" ([16] is van Emde Boas et al.).
  Also: "The best known upper bound for the static predecessor problem was
  obtained by Willard [17] who gave a linear space implementation requiring
  O(log log n) time per query."

In these statements n is the size of the universe and b the word size.

**In `dynzfast.tex`:** not cited, and no sentence makes a claim about it. The
statement of `theory.md` (section 3), "Omega(log n / log w) amortised for
dynamic rank and select [S: Patrascu2014Dynamic]", is correct for *rank*
through Theorem 3 with b = w >= lg n [I]; the extension to *select* is not in
this paper: it is claimed by Patrascu and Thorup ("For rank, see Theorem 3
and remark after Theorem 4 in [FS89]. The lower bound for select follows by
another simple reduction to be presented here" [S: footnote 1 of
arXiv:1408.3045v2, same text extraction as in section 6]). If
line 437-438 is extended to say that O(lg n / lg w) is optimal for rank and
select, both papers must be cited (optional edit 9).

## 8. Sen, Venkatesh, "Lower bounds for predecessor searching in the cell probe model"

**Version** [P]: "Journal of Computer and System Sciences 74 (2008) 364-385";
Pranab Sen, S. Venkatesh; "Received 13 November 2003; received in revised
form 28 June 2006. Available online 13 June 2007"; "doi:10.1016/j.jcss.2007.06.016".
Journal version of ICALP 2001 (Sen, Venkatesh) and CCC 2003 (Sen) [P,
footnote].

**BibTeX.** Not in `biblio.bib`. `Sen2008Lower` in `theory.bib`: volume,
pages, year and DOI are printed; the issue number (3) is not; the second
author's first name is printed as "S." only ("Srinivasan" in `theory.bib` is
not verifiable here).

**Statement** [P, p. 366]: "Result 1. Suppose there is a (n^{O(1)}, (log
m)^{O(1)}, t) randomised cell probe scheme for the (m, n)-static predecessor
problem with error probability less than 1/3. Then, there is a constant c
such that: (a) For all sufficiently large m, a value of n can be chosen so
that t >= c log log m / log log log m as a function of m. (b) For all
sufficiently large n, a value of m can be chosen so that t >= sqrt(c log n /
log log n) as a function of n. The same lower bound also holds for
address-only quantum cell probe schemes". Here m is the size of the universe.
That is, the lower bounds of Beame and Fich extended to randomized
(two-sided error) query schemes.

Its account of the upper bounds [P, pp. 365-366] agrees with papers 1-6:
"van Emde Boas et al. [38,39] gave a (Omega(m), O(log m), O(log log m))
deterministic cell probe solution for predecessor. [...] Later, Willard [40]
reduced the number of cells used to O(n)."

**In `dynzfast.tex`:** not cited, and no sentence makes a claim about it.
Lines 431-433 attribute the randomized lower bounds to
`Patrascu2006Timespace,PaTRDNHSP`, which is right for the full trade-off;
line 429 attributes the optimality of sqrt(lg n / lg lg n) "as a function of
$n$ alone" to Beame and Fich, which is right for deterministic schemes. No
edit is needed; if one wants to say that the latter holds for randomized
queries too, this paper is the reference (Result 1(b)).

## 9. Alstrup, Brodal, Rauhe, "Optimal static range reporting in one dimension"

**Version** [P]: Stephen Alstrup, Gerth St{\o}lting Brodal, Theis Rauhe;
"STOC'01, July 6-8, 2001, Hersonissos, Crete, Greece. Copyright 2001 ACM
1-58113-349-9/01/0007"; pages 476-482.

**BibTeX.** Not in `biblio.bib`; `Alstrup2001Optimal` in `theory.bib` is
correct (DOI not printed).

**Statements** [P]:

- p. 476: S a subset of U = {0, ..., 2^w - 1}; "FindAny(a, b): Report any
  element in S intersected with [a, b] or [bottom] if there is no such
  element." "Our main result is a static data structure with space cost O(n)
  that supports the query FindAny in constant time. As a corollary, the data
  structure allows Report in time O(k), where k is the number of elements to
  be reported." "The preprocessing time for the mentioned data structures is
  expected time O(n sqrt(log u))."
- "Theorem 4. The data structure supports FindAny in constant time and Report
  in time O(k), where k is the number of elements reported. The data
  structure requires space O(|S|) words."
- Model (p. 477): "a unit cost RAM with word size w bits, where the set of
  instructions includes the standard boolean operations on words, the
  arbitrary shifting of words, and the multiplication of two words", with
  random bits (for the construction).
- p. 477: "For data structures with linear space cost, Willard [24] provides
  a data structure with time O(log log u) for predecessor queries."

**In `dynzfast.tex`:** not cited. The statement of `theory.md` (section
5.2), "static, linear space, an element of the query interval in O(1), all k
in O(k)", is **correct** (Theorem 4). Lines 462-465 discuss only the dynamic
result of Mortensen, Pagh and Patrascu; optional edit 8 adds the static one.

## 10. Brodnik, Miltersen, Munro, "Trans-dichotomous algorithms without multiplication - some upper and lower bounds"

**Version** [P]: Andrej Brodnik, Peter Bro Miltersen, J. Ian Munro; pages
numbered 427-439 after an unnumbered first page; neither the workshop nor
the volume is printed. Read in full.

**BibTeX** (`Brodnik1997Transdichotomous`): consistent with the PDF (authors,
title, pp. 426-439); WADS '97 and LNCS 1272 cannot be checked on the pages
(Thorup's SODA 2003 paper cites it as its [16], see `verified-hashing.md`).

**Model** [P]:

- p. 426: "a random access machine where each register is capable of holding
  exactly one element of the universe [...] a basic instruction set
  consisting of: Direct and indirect addressing, conditional jump, and a
  number of computational instructions, including addition, subtraction,
  bitwise Boolean operations and left and right shifts. All operations are
  unit cost."
- For the lower bounds (p. 434): "we have a RAM with a certain fixed number
  of CPU-registers [...] and a random access memory which can be accessed
  from the CPU using direct and indirect reads and writes. We have
  conditional jump (if A = 0 then branch) and the following computational
  instructions operating on CPU-registers: +, NAND (this is sufficient to
  simulate all the bitwise Boolean operations) and [the shift]." Shifts are
  by a variable amount, in both directions (p. 428).
- Precomputed tables (p. 434): "Our lower bounds hold even in the presence of
  precomputed tables, as long as the precomputed table has size less than
  2^{w^epsilon} for certain constants epsilon > 0."

**The statement** [P, p. 435]: "**Theorem 15.** Any RAM program computing the
rightmost (or leftmost) set bit of a word uses time Omega(log log w). This is
true, even if a precomputed table of size O(2^{w^{1-epsilon}}) is allowed, for
any constant epsilon > 0."

- The proof is written for the least significant bit (inputs: the w words of
  Hamming weight one); the leftmost bit is covered by the statement of the
  theorem and by the opening of Section 4 ("locating the least (or most)
  significant bit of a word, using the basic instruction set").
- It is a worst-case bound for deterministic programs; randomization is not
  discussed.
- Upper bounds: "Lemma 8. RightmostOne(x) can be computed in time O(log log
  w)" (p. 430; with word-sized constants depending on w, "weakly
  non-uniform"). For the most significant bit there is only a remark (p.
  431): "Gerth Brodal (personal communication) has noted that a slight
  modification of the above algorithm can also be used to find the most
  significant set bit of a word. When multiplication is allowed, both
  operations can be done in constant time [13, 7, 3]." The abstract claims
  tightness only for the least significant bit: "Theta(log log w) time is
  necessary and sufficient for identifying the least significant set bit of a
  word."
- Other results: "Theorem 13" (static dictionary with n + o(n) cells and
  query time sqrt(log n) (log log n)^{1+o(1)} without multiplication);
  "Theorem 17" (reversing a b-bit string needs Omega(log b)); "Theorem 19.
  Any RAM program multiplying two w-bit words using the basic instruction set
  uses time Omega(log w). This is true, even if a precomputed table of size
  O(2^{w^{1/20}}) is allowed."

**Sentence of `dynzfast.tex`** (lines 2082-2084): "With multiplication,
$\lambda$ can be computed in constant time~\cite{FrWSITBFT}; however, on a RAM
providing just additions, shifts and bitwise Boolean operations its
computation requires time $\Omega(\lg \lg w)$, unless the space used is
exponential in $w^{1-\epsilon}$~\cite{Brodnik1997Transdichotomous}\todo{...}."

**Verdict: correct.** Model, bound and space condition are those of Theorem
15, which names the leftmost set bit explicitly. The `\todo` can be removed.
Two refinements: the instruction set of the paper includes subtraction; and
"unless the space used is exponential in $w^{1-\epsilon}$" is better stated
as the theorem does (a precomputed table of size O(2^{w^{1-epsilon}}), for
any constant epsilon > 0). See edit 1. The sentence that follows in the text
(line 2087: an upper bound on lambda(b) + 1 "found by doubling in $O(\lg\lg
b)$ steps") is consistent with the lower bound.

## 11. Dietzfelbinger, Meyer auf der Heide, "A new universal class of hash functions and dynamic hashing in real time"

**Version** [P]: Martin Dietzfelbinger, Friedhelm Meyer auf der Heide,
Universit\"at-GH Paderborn; pages 6-19 (second page numbered 7). Venue and year
are not printed; three reference lists read here give them [P: Dietzfelbinger
et al., ICALP 1992, ref. 5: "In ICALP '90 (LNCS 443), pp. 6-19, July 1990";
Arbitman, Naor, Segev, ICALP 2009, ref. 5: "In: Paterson, M. (ed.) ICALP
1990. LNCS, vol. 443, pp. 6-19. Springer, Heidelberg (1990)"; Arbitman, Naor,
Segev, FOCS 2010, ref. [15]: "in 17th ICALP, 1990, pp. 6-19"]. This settles
the note in `theory.bib` ("1990 and the colloquium number are from memory").

**BibTeX.** Not in `biblio.bib`. Entry at the end (with volume 443).

**The result** [P, p. 7]: "Theorem 1.1. There is a probabilistic (Monte Carlo
type) algorithm for implementing a dictionary with the following features.
Let k >= 1 be constant. (i) If after executing a sequence of instructions n
keys are stored in the dictionary, then the data structure occupies space
O(n) at this time. (ii) Performing one lookup takes constant time in the
worst case. (iii) Performing (1/2)n instructions in a dictionary of size n
takes constant time per operation. (iv) The algorithm is always correct; it
fails to fulfill (iii) with probability O(n^{-k}). (The constants in the
time bounds and the space bound depend on k.)"

- Abstract: "a Monte Carlo type dictionary that uses linear space and needs
  worst case constant time per instruction. [...] Answers to queries given by
  the algorithm are always correct, the space bound is always satisfied, and
  the algorithm fails only with probability O(n^{-k}), where n is the number
  of data items currently stored."
- p. 7: "the probability space underlying the analysis is induced by the
  random choices the algorithm makes; no assumptions are made concerning the
  distribution of the keys occurring in the instructions." The keys carry
  associated information (p. 6: "'Insert x' causes x together with some
  information associated with x to be stored").
- **No bound on the number of keys is needed** (pp. 17-18): "We still have to
  handle the case that the number of instructions to be performed by a
  dictionary may not be known in advance and/or exceed the number of keys
  stored in the dictionary at any one time"; Algorithm C works in phases of
  n/2 instructions, n being the number of keys at the beginning of the phase,
  and rebuilds a background dictionary of capacity n during the phase.
- Failure (p. 18): "If in a phase any one of the dictionaries or procedures
  fails to perform properly, i.e., exceeds its time bounds, we abort the
  phase. This happens with probability O(n^{-k}). All keys currently stored
  in the dictionary are collected and inserted into a new background
  dictionary [...]. No instructions are accepted during this time. The
  expected time needed for this reconstruction is O(n)."
- Universe and model: "Let p be prime, U = {0, 1, ..., p - 1} the universe";
  hash functions are built from polynomials modulo p. The machine model is
  not spelled out; keys are elements of U handled in unit time [I].

**Priority** [S, both printed in PDFs read for `verified-hashing.md` and
`verified-btrees-hashing.md`]: Arbitman, Naor, Segev (FOCS 2010, pp.
788-789): "Dietzfelbinger and Meyer auf der Heide [15] constructed the first
dynamic dictionary with constant-time operations in the worst case with high
probability, using O(n) memory words to store n elements"; Bender et al.
(iceberg hashing, J. ACM 2023, Section 8): "The first hash table to achieve
constant-time operations with high probability was that of Dietzfelbinger et
al. [16] in 1990".

**Sentences of `dynzfast.tex`** (lines 384-391, third item of "The
dictionary"): "There are dictionaries in which every operation takes constant
time in the worst case with high probability. If an upper bound $N$ on $n$ is
known in advance, de-amortized Cuckoo hashing [...] and backyard Cuckoo
hashing [...] use $O(N)$ words [...]. The bound $N$ is not necessary: iceberg
hashing~\cite{Bender2023Iceberg} uses space linear in the current number $n$
of keys [...]".

**Verdict: this paper should be cited there, as the first such dictionary.**
The statements made about the other three dictionaries are not affected, but
"The bound $N$ is not necessary: iceberg hashing" suggests that dictionaries
without a bound known in advance are recent, whereas Theorem 1.1(i) already
gives space O(n) for the current n. Differences that justify keeping the
other citations [P/I]: here the probability is polynomially small in the
*current* number of keys and concerns each phase of n/2 instructions; the
space is O(n) words with constants depending on k (iceberg hashing and
backyard cuckoo hashing are succinct, or nearly so); keys are single
elements of the universe. See edit 5. Line 194-195 of the abstract ("with
high probability using a dictionary with constant-time operations with high
probability") needs no change.

## 12. Pagh, Pagh, Ruzic, "Linear probing with constant independence" (journal version)

**Version** [P]: "SIAM J. Comput. Vol. 39, No. 3, pp. 1107-1120", "(c) 2009
Society for Industrial and Applied Mathematics", "DOI. 10.1137/070702278";
"Received by the editors September 10, 2007; accepted for publication (in
revised form) November 20, 2008; published electronically September 2, 2009."
This is the expected journal version.

**BibTeX** (`Pagh2009Linear`): correct in every field (volume 39, number 3,
pages 1107-1120, year 2009, DOI).

**Theorem numbers and statements against those quoted from STOC 2007 in
`verified-hashing.md`, section 3.** The numbering changed (theorems are
numbered by section) and so did the bounds:

| STOC 2007 (as quoted in `verified-hashing.md`) | SIAM J. Comput. 2009 [P] |
|---|---|
| Theorem 1, p. 321 (Omega(r log r) for H(p, r), r = ceil(p/2)) | **Theorem 3.2**, p. 1112: "For r = ceil(p/2) there exists a set S subset of [p], \|S\| <= r/2, such that the expected cost of inserting the keys of S into a linear probing hash table of size r using a hash function chosen uniformly at random from H(p, r) is Omega(r log r)." Same statement. |
| Corollary 1, p. 321 | **Corollary 3.4**, p. 1114: "Theorem 3.2 holds also if we replace H(p, r) by H*(p, r). In particular, pairwise independence with uniformly distributed function values is not a sufficient condition for linear probing to have expected constant cost per operation." |
| Theorem 2, p. 322: expected cost O(1 + (1 - alpha-bar)^{-3}) per operation, O(1 + (1 - alpha-bar)^{-2}) average for successful lookups | **Theorem 4.3**, pp. 1115-1116: "Consider any sequence of operations (insertions, deletions, and lookups) in a linear probing hash table, where the hash function h used has been chosen uniformly at random from a 5-wise independent family of functions H. Let n and alpha-bar < 1 denote, respectively, the maximum number of keys in the table during a particular operation and the corresponding maximum load. Then the expected number of probes made during that operation is O((1 - alpha-bar)^{-5/2})." **Theorem 4.5**, p. 1118: the same with O((1 - alpha-bar)^{-13/6}). Section 1.2, p. 1108: "This implies a bound of O(1/(1 - alpha)^{7/6}) expected time on average for successful searches." |
| Theorem 3 (total number of probes of n insertions), Theorem 4 (blocked probing), Corollary 2 and Theorem 5 (4-wise independence) [located in the text layer of `LinearProbing.pdf`, not reread] | No theorem with these statements in the journal version, whose Section 5 ("Improving the lookup cost") describes instead a doubling search with expected lookup time O(log(1/(1 - alpha))). The journal version has no result with 4-wise independence. |
| Section 2.2, maximum load "in the range [alpha; (1 + n/p) alpha]" | p. 1110: "in the range [alpha, (1 + r/p) alpha]". |

Other statements [P]: abstract: "linear probing using a pairwise independent
family may have expected logarithmic cost per operation. On the positive
side, we show that 5-wise independence is enough to ensure constant expected
time per operation"; p. 1108: "5-wise independence is enough to ensure
constant expected time per operation for load factor alpha = n/r bounded away
from 1"; p. 1114: the lower bound is tight (O(n log n) by Chebyshev's
inequality); Section 6, p. 1119: "It is unclear whether 4-wise or even 3-wise
independence can guarantee good expected performance for linear probing."
The table size r is fixed, as in the conference version; reference [15] is
Thorup, "String hashing for linear probing" (SODA 2009), "when the domain of
keys is complex, like the set of variable-length strings" (p. 1109).

**Sentences of `dynzfast.tex`** (lines 377-380): "With \emph{linear probing}
and a hash function chosen from a $5$-independent family, each operation on a
table with load factor bounded away from one takes constant expected
time~\cite{Pagh2009Linear} [...] Note that pairwise independence is not
sufficient~\cite{Pagh2009Linear}." **Correct** in the version cited
(Theorems 4.3 and 4.5; Theorem 3.2 and Corollary 3.4). The text quotes no
theorem number, so nothing changes in it; `verified-hashing.md` should be
read with the table above.

## 13. Patrascu, Thorup, "The power of simple tabulation hashing" (journal version)

**Version** [P]: "ACM Reference Format: P\u{a}tra\c{s}cu, M. and Thorup, M.
2012. The power of simple tabulation hashing. J. ACM 59, 3, Article 14 (June
2012), 50 pages. DOI = 10.1145/2220357.2220361"; pages numbered 14:1-14:50;
"A short preliminary version of this article was presented in Proceedings of
the 43rd Symposium on Theory of Computing, 2011, pp. 1-10." This is the
expected journal version.

**BibTeX** (`Patrascu2012Power`): volume 59, number 3, year 2012 and DOI are
correct. **`pages = {1--50}` should be `{14:1--14:50}`** (Article 14; the
form used in the file for `AnTDOSEST` and `Binna2022Height`). This settles
the note of `verified-hashing.md` ("the field pages = {1--50} and the article
number remain unverified").

**Theorem numbers and statements against the STOC 2011 version.**

| STOC 2011 (as quoted in `verified-hashing.md`) | J. ACM 2012 [P] |
|---|---|
| Theorem 1 (Chernoff-type bounds) | **Theorem 1.1**, p. 14:4 |
| Theorem 2 (cuckoo hashing, failure probability O(n^{-1/3})) | **Theorem 1.2**, p. 14:5, same statement; the lower bound is **Observation 4.1**, p. 14:21 |
| Linear probing: stated without proof | Stated on p. 14:5 in the same words ("if the table size is m = (1 + epsilon)n, then the expected time per operation is O(1/epsilon^2) [...] compares positively with the O(1/epsilon^{13/6}) bound of [Pagh et al. 2009] for 5-independent hashing"; "constant variance for constant epsilon"), and **proved in Section 3**: Theorem 3.1 (p. 14:15, concentration in arbitrary intervals), **Corollary 3.2** (p. 14:20, tail bound on the length R(q, S) of the run of filled positions), followed by "for any moment p = O(1), E[R(q, S)^p] = O(1/epsilon^{2p})" for alpha >= 1/2 and "E[R(q, S)^p] = O(alpha) when q not in S" (p. 14:20, using the fourth-moment bounds of Section 6.4). |

Other statements [P]:

- p. 14:2: "It is known that 5-independence is sufficient [Pagh et al. 2009]
  and necessary (some 4-independent schemes do not work) [P\u{a}tra\c{s}cu and
  Thorup 2010] for linear probing."
- p. 14:4: "we assume the number c = O(1) of characters is constant"; "the
  number of bins is generally understood to be a power of two".
- **Long keys** (p. 14:2): "if we work with n keys from a very large universe
  U, we typically first do a universe reduction, applying a universal hash
  function [Carter and Wegman 1979] from U to [u] where u = n^{O(1)}. Thus,
  when we apply simple tabulation, we assume u = n^{O(1)}." And in Section 7
  (p. 14:42, text layer): "To achieve expected O(1) time for linear probing,
  it suffices to map universally to a domain of just O(n) [Thorup 2009]."

**Sentence of `dynzfast.tex`** (line 379): "the same is true of simple
tabulation hashing~\cite{Patrascu2012Power}". **Correct**, and now supported
by a proof in the version cited. A precision that the text does not make
[P]: the result is for keys from a universe polynomial in n, so the keys of
the z-map (handles of O(w) bits, or signatures) must first be reduced by a
universal hash function, as the authors say; no edit is proposed, since the
same reduction is already implicit for the other dictionaries ("the results
are stated for keys of one word", line 388).

## 14. Karp, Rabin, "Efficient randomized pattern-matching algorithms"

**Version** [P]: "IBM J. Res. Develop. Vol. 31 No. 2 March 1987", pp. 249-260;
"Received December 17, 1986; accepted for publication January 6, 1987".

**BibTeX.** Not in `biblio.bib`; `Karp1987Efficient` in `theory.bib` is
correct in every printed field (the DOI is not printed).

**The fingerprints** [P]:

- Section 3, "A family of modular fingerprint functions" (p. 252): "A binary
  string X = x_1 x_2 ... x_n can be regarded as a binary representation of
  the integer H(X) = sum_{i=1}^n x_i 2^{n-i}. For any integer p, the function
  H_p(X) = H(X) mod p is a possible fingerprint function. Let M be a positive
  integer to be specified later. Define S = {p | p is prime and p <= M}". The
  random choice is that of the **prime modulus**; the base is fixed (2).
  The prime is found by "repeatedly choosing random integers in that range,
  testing each for primality"; "The expected number of trials is
  approximately ln M."
- **Error bound** (pp. 252-253): "Theorem 3. If Algorithm 2 or Algorithm 3 is
  executed with S = {p | p <= M and p prime}, then, for every instance
  {(X(r), Y(r)), r in R}, the probability that a false match occurs is <=
  pi(nt)/pi(M), provided nt >= 29", where t = |R| is the number of pairs of
  strings of n bits compared and pi(u) the number of primes not larger than
  u. "Corollary 4(a). If [...] S equal to the set of primes <= nt^2 [...] the
  probability that a false match occurs is <= 2.511/t." For a single pair of
  distinct strings of n >= 29 bits (t = 1) the bound is pi(n)/pi(M) [I].
  Example (p. 253): n = 250, t = 4000, M = nt^2 < 2^32: "the probability that
  a false match occurs will be less than 10^{-3}".
- **Updating** (p. 252): the third required property is "fingerprints are
  easy to update"; p. 254, equation (1): the fingerprint of the next window
  of the text is obtained from the previous one "with a constant number of
  operations".
- **A second family** (Section 6, p. 257): K_p, "a homomorphism from {0, 1}*
  into the group of 2 x 2 unimodular matrices with entries in Z_p", with
  K(X * Y) = K(X) K(Y) ("* denotes concatenation of strings"); again p is a
  random prime ("Corollary 10 [...] the probability that a false match
  occurs is <= 6.971/t"). Section 8 (p. 259) uses the fingerprints of **all
  prefixes** of the text, K_p(P(Y, k)) = K_p(y_1) ... K_p(y_k), and obtains
  the fingerprint of a substring from two of them: "K_p(Y(r)) = K_p(P(Y, r -
  1))^{-1} K_p(P(Y, r + n - 1))".
- Conclusion (p. 259): "the second author has found another class of
  fingerprint functions employing polynomials over finite fields instead of
  integers [13]", [13] being "M. O. Rabin, 'Fingerprinting by Random
  Functions,' Report TR-15-81, Center for Research in Computing Technology,
  Harvard University, Cambridge, MA, 1981" (so printed; the report is
  `Rabin1981Fingerprinting`, whose title is "... by random polynomials").
- The algorithms of the paper that never err (Algorithms 2 and 3) verify
  each match on the strings, as the z-fast trie does at the end of a search.

**The phrase "Karp--Rabin fingerprints" in `dynzfast.tex`.** It occurs twice,
without citation, and both times it describes other people's structures:

- line 558: "by replacing the z-fast trie with Patricia tries endowed with
  Karp--Rabin fingerprints and laid out by centroid decomposition"
  (Ferragina; Ferragina and Venturini);
- line 596: "z-fast tries (in most cases, the static version, often with
  signatures given by Karp--Rabin fingerprints and a final verification)".

**Verdict: appropriate as the name used in that literature, but not as a
description of a family.** What those papers (and `sec:dict`) use is the
value at a *random point* of a polynomial over Z_p with p *fixed*, i.e., the
functions of `Dietzfelbinger1992Polynomial`; neither of the two families of
Karp and Rabin is of this kind (both fix the base, or the matrices, and
choose the prime at random) [I, from the definitions above and from `sec:dict`,
lines 1436-1444]. What is genuinely due to Karp and Rabin is the use the
phrase alludes to: short fingerprints chosen at random at run time,
"easy to update", including fingerprints of all prefixes from which the
fingerprint of a substring is computed in constant time (Section 8), with a
final verification. The paper's own signatures are never called Karp-Rabin
fingerprints, which is right.

**Is a sentence distinguishing the two needed in `sec:dict`?** It is not
needed for correctness. It is advisable for two reasons: the paragraph at
lines 1469-1482 now discusses Rabin's fingerprints by irreducible polynomials
at length, and a reader who knows "Karp--Rabin fingerprints" from the
literature cited at lines 558 and 596 will take them to be either that family
or the one of line 1436, while they are a third one; and the phrase is used
twice with no reference. Edit 10 proposes one sentence and the citation. With
the random-prime family the argument of `sec:dict` goes through as well [I]:
the fingerprint of a prefix is extended by a word with one multiplication by
2^w mod p and one addition modulo p; the price is the generation of a random
prime, and a collision bound that is a ratio of prime counts instead of
L/(wp).

---

## Proposed edits to dynzfast.tex

Edits 1-5 correct or complete the text; edits 6-10 are optional additions.
All new text is ASCII.

### Edit 1 (lines 2082-2084): resolve the `\todo` on Brodnik, Miltersen, Munro

Old:

```latex
iteration. With multiplication, $\lambda$ can be computed in constant time~\cite{FrWSITBFT}; however, on a RAM providing just additions,
shifts and bitwise Boolean operations its computation requires time $\Omega(\lg \lg w)$, unless the space used is exponential in
$w^{1-\epsilon}$~\cite{Brodnik1997Transdichotomous}\todo{This citation is taken from~\cite{Thorup2003AC0}; the paper has not been checked.}.
```

New:

```latex
iteration. With multiplication, $\lambda$ can be computed in constant time~\cite[Section~5]{FrWSITBFT}; however, on a RAM providing just additions,
subtractions, shifts and bitwise Boolean operations its computation requires time $\Omega(\lg \lg w)$, even if a precomputed table of
size $O\bigl(2^{w^{1-\epsilon}}\bigr)$ is available, for any constant $\epsilon>0$~\cite[Theorem~15]{Brodnik1997Transdichotomous}.
```

Reason: the statement is Theorem 15 of the paper, verified; the `\todo` is
resolved. The new wording follows the theorem (table of size
O(2^{w^{1-epsilon}}), any constant epsilon) and the instruction set of the
paper (which has subtraction). Minimal alternative: delete the `\todo` and
change nothing else.

### Edit 2 (lines 421-424): y-fast tries and Mehlhorn-Naher

Old:

```latex
the \emph{y-fast trie}~\cite{WilLLWRQ} brings space down to linear by storing the keys in buckets of 
$\Theta(\ell)$ consecutive keys and indexing one key per bucket with an x-fast trie, at the price of amortized 
bounds for updates. Mehlhorn and N\"aher~\cite{MeNBOD} obtain similar results from the recursion of 
van Emde Boas using dynamic perfect hashing~\cite{DKMDPHULB}, and need buckets to attain linear space. 
```

New:

```latex
the \emph{y-fast trie}~\cite{WilLLWRQ} brings space down to linear by storing the keys in buckets of 
$\Theta(\ell)$ consecutive keys, each represented by a binary search tree, and indexing one key per bucket with an x-fast trie. 
Both structures answer queries in time $O(\lg\ell)$ in the worst case. Willard's hash tables are static: 
with dynamic perfect hashing~\cite{DKMDPHULB}, the y-fast trie supports updates in amortized expected time $O(\lg\ell)$~\cite{FrWSITBFT}. 
Mehlhorn and N\"aher~\cite{MeNBOD} obtain the same bounds applying dynamic perfect hashing directly to the recursion of 
van Emde Boas, and need buckets (of $\Theta(\lg\ell)$ keys) to attain linear space. 
```

Reason: Willard proves only worst-case query bounds (Lemmas 1-3), uses the
static tables of Fredman, Komlos and Szemeredi, and gives no update bound
("it should be possible to guarantee expected cost log log N"); the
"amortized randomized time log log U" for y-fast tries with dynamic perfect
hashing is stated by Fredman and Willard (pp. 425 and 435); Mehlhorn and
Naher's Theorem 1.1 has worst-case `locate` and "amortized and randomized"
updates, and buckets of log log N to 4 log log N keys. Minimal alternative:
replace "at the price of amortized bounds for updates" with "at the price of
amortized bounds for updates, once the hash tables are made
dynamic~\cite{DKMDPHULB}".

### Edit 3 (lines 426-427): the bound of fusion trees

Old:

```latex
and their time bounds do not depend on $n$. The other approach reduces the \emph{number} of keys: fusion
trees~\cite{FrWSITBFT} answer queries in time $O(\lg n/\lg w)$, and the exponential search trees of 
```

New:

```latex
and their time bounds do not depend on $n$. The other approach reduces the \emph{number} of keys: fusion
trees~\cite{FrWSITBFT} answer queries in time $O(\lg n/\lg w)$ in the static case, and support queries and updates in 
amortized time $O(\lg n/\lg\lg n)$, and the exponential search trees of 
```

Reason: the bound printed in the paper is "amortized time O(log N / log log
N) per operation"; O(lg n / lg w) is the later restatement, valid for a
static tree of degree w^{1/6} (not printed in the paper: an inference, and
the form used by Patrascu and Thorup). The dynamic O(lg n / lg w) is
attributed correctly to `Patrascu2014Dynamic` a few lines below.

### Edit 4 (line 316): pinpoint the citation for the most significant bit

Old: `multiplication~\cite{FrWSITBFT}, can be avoided`

New: `multiplication~\cite[Section~5]{FrWSITBFT}, can be avoided`

Reason: the computation of floor(lg x) is on pp. 431-432 (Section 5,
"Fusion"), in a paper whose title does not suggest it. Optional. ("which can
be done in constant time" would read better than "which requires constant
time".)

### Edit 5 (lines 384-389): the first dictionary with constant time w.h.p.

Old:

```latex
  \item There are dictionaries in which every operation takes constant time in the worst case with high probability.
  If an upper bound $N$ on $n$ is known in advance, de-amortized Cuckoo hashing~\cite{Arbitman2009Deamortized} and
```

New:

```latex
  \item There are dictionaries in which every operation takes constant time in the worst case with high probability.
  The first one is the real-time dictionary of Dietzfelbinger and Meyer auf der Heide~\cite{Dietzfelbinger1990New}: 
  it uses space linear in the current number $n$ of keys, lookups always take constant time, and for every constant $k$, 
  with probability $1-O(n^{-k})$ each of the next $n/2$ operations takes constant time.
  If an upper bound $N$ on $n$ is known in advance, de-amortized Cuckoo hashing~\cite{Arbitman2009Deamortized} and
```

and, in the same item, old:

```latex
  (the results are stated for keys of one word). The bound $N$ is not necessary: iceberg hashing~\cite{Bender2023Iceberg} 
```

new:

```latex
  (the results are stated for keys of one word). Iceberg hashing~\cite{Bender2023Iceberg}, like the dictionary of~\cite{Dietzfelbinger1990New}, does not need the bound $N$: it 
```

Reason: Theorem 1.1 of the ICALP 1990 paper; priority as stated by Arbitman,
Naor and Segev and by Bender et al. The old sentence suggests that removing
the bound N is a recent achievement.

### Edit 6 (lines 416-418, optional): the three papers of van Emde Boas

Old:

```latex
\paragraph{Predecessor search on integers.} For $n$ integers of $\ell\leq w$ bits, van Emde Boas 
trees~\cite{EKZDIEPQ} answer predecessor queries and perform updates in time $O(\lg \ell)$, 
but use space depending on the size of the universe. Willard's \emph{x-fast trie}~\cite{WilLLWRQ} 
```

New:

```latex
\paragraph{Predecessor search on integers.} For $n$ integers of $\ell\leq w$ bits, van Emde Boas 
trees~\cite{vanEmdeBoas1975Preserving,EKZDIEPQ} answer predecessor queries and perform updates in time $O(\lg \ell)$ in the worst case, 
but use space depending on the size of the universe ($O(2^\ell)$ words, using buckets~\cite{vanEmdeBoas1977Preserving}). 
Willard's \emph{x-fast trie}~\cite{WilLLWRQ} 
```

Reason: the FOCS 1975 paper is the first publication; the 1977 note reduces
the space from O(u lg lg u) to O(u) by the two-level technique that Willard
and Mehlhorn-Naher then apply to the set of keys. I did not add "without
multiplications": what is printed (FOCS p. 79, journal p. 110) is that the
static structure is *initialized* without multiplications and
bit-manipulations, and (1977 note, p. 82) that division is replaced by a
table; that no operation uses multiplication is the reading of the survey of
Navarro and Rojas-Ledesma [S: `theory.md`, section 2.1], which I did not
check on the PASCAL program.

### Edit 7 (line 479, optional): binary search on the levels

Old: `using hashing to test whether a prefix appears, goes back to x-fast tries~\cite{WilLLWRQ}; it was rediscovered`

New: `using hashing to test whether a prefix appears, goes back to x-fast tries~\cite{WilLLWRQ} (the ``binary search on the levels'' of a trie on the universe is already in~\cite{vanEmdeBoas1975Preserving}); it was rediscovered`

Reason: van Emde Boas, p. 77, and Willard's own acknowledgement (p. 82).

### Edit 8 (line 464-465, optional): static range reporting

Old: `queries) in time $O(\lg\lg w)$ with updates in time $O(\lg w)$, with high probability. The z-fast trie answers`

New: `queries) in time $O(\lg\lg w)$ with updates in time $O(\lg w)$, with high probability; in the static case, constant time is possible in linear space~\cite{Alstrup2001Optimal}. The z-fast trie answers`

Reason: Theorem 4 of Alstrup, Brodal and Rauhe (already suggested in
`verified-theory.md`, sections 2.5 and 4.5; now verified).

### Edit 9 (line 437-438, optional): optimality for rank and select

Old: `updates in time $O(\lg n/\lg w)$, which implies constant time for sets of size $w^{O(1)}$, and they show`

New: `updates in time $O(\lg n/\lg w)$, which is optimal for rank and select~\cite{Fredman1989Cell,Patrascu2014Dynamic} and implies constant time for sets of size $w^{O(1)}$, and they show`

Reason: Fredman and Saks, Theorem 3 (and the reduction to subset rank they
announce) for rank; the reduction for select is in `Patrascu2014Dynamic`.
Only if the authors want to mention rank and select beyond the parenthesis.

### Edit 10 (after line 1478 and at line 596, optional): Karp-Rabin fingerprints

At line 596, old: `often with signatures given by Karp--Rabin fingerprints and a final verification)`

New: `often with signatures given by Karp--Rabin fingerprints~\cite{Karp1987Efficient} and a final verification)`

In `sec:dict`, after the sentence ending "whereas with polynomials over
$\mathbf Z_p$ the multiplication of the RAM model is sufficient." (line
1478), add:

```latex
The fingerprints of Karp and Rabin~\cite{Karp1987Efficient} are, instead, the residues of the integer represented by the string modulo a prime chosen 
uniformly at random among those not larger than a bound $M$: two distinct strings of the same length $n\geq 29$ have the same fingerprint with probability at most 
$\pi(n)/\pi(M)$, where $\pi(u)$ is the number of primes not larger than $u$, and both properties hold, at the price of generating a random prime. 
In the literature, however, the name is commonly used for signatures of the kind used here, in which the prime is fixed and the evaluation point is random.
```

Reason: Section 3 and Theorem 3 of Karp and Rabin (with t = 1). "Both
properties hold" is my inference (extension by a word costs one
multiplication by 2^w mod p and one addition). The last sentence is general
knowledge about the usage of the papers cited in "Uses of z-fast tries"; it
is the reason for adding the sentence at all.

---

## Entries for biblio.bib

### Corrections to existing entries

`EKZDIEPQ`: journal and year as printed on the paper.

```bibtex
@article{EKZDIEPQ,
  title     = {Design and implementation of an efficient priority queue},
  author    = {van Emde Boas, P. and Kaas, R. and Zijlstra, E.},
  journal   = {Mathematical Systems Theory},
  volume    = {10},
  number    = {1},
  pages     = {99--127},
  year      = {1977},
  publisher = {Springer}
}
```

(The issue number is not printed in the PDF; Crossref dates the issue 1976
according to `theory.bib`.)

`Patrascu2012Power`: `pages = {14:1--14:50}` instead of `{1--50}`.

No change: `WilLLWRQ`, `MeNBOD`, `FrWSITBFT`, `Pagh2009Linear`,
`Brodnik1997Transdichotomous`.

### New entries

Fields marked "not printed" in the sections above (DOIs of the scanned
papers, issue numbers) come from `theory.bib`, i.e., from Crossref.

```bibtex
@inproceedings{vanEmdeBoas1975Preserving,
  author    = {van Emde Boas, P.},
  title     = {Preserving order in a forest in less than logarithmic time},
  booktitle = {16th Annual Symposium on Foundations of Computer Science (FOCS 1975)},
  pages     = {75--84},
  publisher = {IEEE},
  year      = {1975},
  doi       = {10.1109/SFCS.1975.26}
}

@article{vanEmdeBoas1977Preserving,
  author  = {van Emde Boas, P.},
  title   = {Preserving order in a forest in less than logarithmic time and linear space},
  journal = IPL,
  volume  = {6},
  number  = {3},
  pages   = {80--82},
  year    = {1977},
  doi     = {10.1016/0020-0190(77)90031-X}
}

@inproceedings{Dietzfelbinger1990New,
  author    = {Dietzfelbinger, Martin and Meyer auf der Heide, Friedhelm},
  title     = {A new universal class of hash functions and dynamic hashing in real time},
  booktitle = {Automata, Languages and Programming, 17th International Colloquium, ICALP 1990},
  series    = {Lecture Notes in Computer Science},
  volume    = {443},
  pages     = {6--19},
  publisher = {Springer},
  year      = {1990},
  doi       = {10.1007/BFb0032018}
}

@article{Karp1987Efficient,
  author  = {Karp, Richard M. and Rabin, Michael O.},
  title   = {Efficient randomized pattern-matching algorithms},
  journal = {IBM Journal of Research and Development},
  volume  = {31},
  number  = {2},
  pages   = {249--260},
  year    = {1987},
  doi     = {10.1147/rd.312.0249}
}

@inproceedings{Alstrup2001Optimal,
  author    = {Alstrup, Stephen and Brodal, Gerth St{\o}lting and Rauhe, Theis},
  title     = {Optimal static range reporting in one dimension},
  booktitle = {Proceedings of the 33rd Annual {ACM} Symposium on Theory of Computing (STOC 2001)},
  pages     = {476--482},
  publisher = {ACM},
  year      = {2001},
  doi       = {10.1145/380752.380842}
}

@inproceedings{Fredman1989Cell,
  author    = {Fredman, Michael L. and Saks, Michael E.},
  title     = {The cell probe complexity of dynamic data structures},
  booktitle = {Proceedings of the 21st Annual {ACM} Symposium on Theory of Computing (STOC 1989)},
  pages     = {345--354},
  publisher = {ACM},
  year      = {1989},
  doi       = {10.1145/73007.73040}
}

@article{Sen2008Lower,
  author  = {Sen, Pranab and Venkatesh, S.},
  title   = {Lower bounds for predecessor searching in the cell probe model},
  journal = JCSS,
  volume  = {74},
  number  = {3},
  pages   = {364--385},
  year    = {2008},
  doi     = {10.1016/j.jcss.2007.06.016}
}
```

Notes on the entries:

- `vanEmdeBoas1975Preserving`: pages verified on the PDF; the name of the
  proceedings from the reference lists of Willard and of Waldvogel et al.
- `vanEmdeBoas1977Preserving`, `Karp1987Efficient`: volume, number, month and
  pages verified on the PDFs. The DOI of the first agrees with the PII in the
  PDF metadata ("PII: 0020-0190(77)90031-X").
- `Dietzfelbinger1990New`: pages verified on the PDF; LNCS 443, the
  colloquium number and the year from three reference lists (section 11).
  The note in `theory.bib` can be dropped.
- `Alstrup2001Optimal`, `Fredman1989Cell`: pages verified on the PDFs.
- `Sen2008Lower`: the second author is printed as "S. Venkatesh"; the issue
  number is not printed.
- `IPL` and `JCSS` are `@string` macros defined at the top of `biblio.bib`.

## Follow-up for the other files of this directory

- `verified-hashing.md`, section 3: the theorem numbers and the exponents
  quoted there are those of STOC 2007; the journal version cited by the paper
  has Theorem 3.2, Corollary 3.4, Theorems 4.3 and 4.5 (section 12 above).
  Section 4: the journal version has Theorems 1.1 and 1.2 and proves the
  statements on linear probing (section 13 above).
- `theory.md`, section 2.2: "y-fast trie [...] updates O(log log u) expected
  amortized" is the bound obtained with dynamic perfect hashing, not a
  statement of Willard's paper. Section 8 (Karp-Rabin, "described here from
  general knowledge, unverified"): now verified, section 14 above.
- `TO-DOWNLOAD.md`, section 2: all the papers listed there have been read,
  except Bayer and Unterauer (not among the fourteen PDFs assigned here).
