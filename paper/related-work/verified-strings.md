# Verification of statements about five papers on strings and prefix search

Checked on 2026-11-19/20 against the PDFs in `/Users/vigna/git/zfast/related/`.
Line numbers refer to `paper/dynzfast.tex` as of that date (2555 lines).

How the PDFs were read. `CacheObliviousStringBTrees.pdf`,
`WexponentialSearchTrees.pdf` and `Wormhole.pdf`: every page as a page image.
`StringBTree.pdf`: pages 236-250 as page images, pages 251-280 as text
extracted with `pdftotext -layout`. `ScalablePrefixMatching.pdf`: the whole
text as extracted by `pdftotext -layout`, plus page images of pages 449-452
and 459-464 (the figures containing pseudocode are bitmaps and do not appear
in the extracted text). Verbatim quotes were checked against the extracted
text, except those from the pseudocode figures of Waldvogel et al., which are
transcribed from the page images. In quotes, `||k||` stands for the length of
key k and `<<D>>` for the size of the front-compressed dictionary (the double
angle brackets of Bender et al.).

"Inference" marks what is my deduction and is not printed in the PDF.

Summary of verdicts:

| Paper | Sentences of `dynzfast.tex` | Verdict |
|---|---|---|
| Bender, Farach-Colton, Kuszmaul | 521-522 | imprecise ("bounds of the same form" hides w.h.p., tall cache, amortization, a `lg^2 n` factor in updates and a term linear in the length of the answer) |
| Ferragina, Grossi | 519-520 | correct in substance; predecessor/successor are implied, not stated; "worst case, deterministic" can be added |
| Fischer, Gawrychowski | 498-500 | correct but vague: the "dynamic variant" is insertion-only and its term is `lg^2 lg sigma / lg lg lg sigma` |
| Wu, Ni, Jiang (Wormhole) | 538-548, 2383-2384 | all correct |
| Waldvogel et al. | 459-469, 542 | 459-460 correct; 462-463 slightly imprecise; 466-467 (clearing the lowest bits set) confirmed verbatim for the basic scheme only, and only in the TOCS 2001 version, which is not the paper cited |

---

## 1. Bender, Farach-Colton, Kuszmaul, *Cache-oblivious string B-trees* (PODS 2006)

BibTeX key `Bender2006Cacheoblivious`. The PDF is the PODS '06 proceedings
version, pages 233-242 (10 pages).

### 1.1 Sentences of `dynzfast.tex`

Only one sentence cites the paper (lines 520-522):

> In the cache-oblivious model, the same bound is obtained for prefix queries
> on a static set by Brodal and Fagerberg~\cite{Brodal2006Cacheoblivious}, and
> bounds of the same form are obtained by the cache-oblivious string B-trees of
> Bender, Farach-Colton and Kuszmaul~\cite{Bender2006Cacheoblivious}, which are
> dynamic.

Lines 523 ("These bounds are in general incomparable to ours, which do not
depend on $n$: we replace $\lg_B n$ with the logarithm of the length of the
strings involved") and 525-526 ("the RAM time bound of structures based on
B-trees is essentially always worse than ours") refer to it collectively.

### 1.2 What the paper states

Model and assumptions.

- Cache-oblivious model with the tall-cache assumption. Every theorem ends
  with "All results hold in the cache-oblivious model with the tall-cache
  assumption" (Theorems 5, 7, 14, 15). The proof of Theorem 5 (p. 238) uses
  "the CO model is transdichotomous (B = Omega(log N))" and "the cache is tall
  (M = Omega(B^2))".
- Randomized. The structure hashes the nodes of a compacted trie with
  "a hash function H (such as for Karp-Rabin fingerprinting [24], CRC, or
  MD5)" such that "unequal strings collide with polynomially small
  probability" (Section 2, p. 236). "The algorithm, as stated, is Monte Carlo;
  we might get a false positive on the matching. To make this algorithm Las
  Vegas, we do a character-by-character match only when the hash values match.
  Since the probability of a mismatch is low, this does not increase our
  running time, with high probability" (p. 236). All bounds are stated "w.h.p."
- Amortized updates. Introduction, p. 234: "The cache-oblivious string B-tree
  we present here also supports keys of unbounded length efficiently. It is
  amortized for updates and randomized, but is efficient with respect to
  compression, prefetching, and disk seeks." The same paragraph says of the
  string B-tree: "The string B-tree is deterministic and the performance bounds
  given are worst-case (that is, not amortized)".
- Unbounded-length keys: yes (quote above; Case 2 of the proof of Theorem 5
  treats keys with `||k|| = Omega(M)`).
- Notation (Figure 1): N is the number of keys, `||D||` the sum of the key
  lengths, `<<D>>` the size of the front-compressed dictionary.

Theorems.

- Lemma 1 (p. 237, attributed to Amir, Farach and Matias [3]), RAM: "The
  dictionary-matching problem can be solved within the bounds
  O(||k|| + log N) for MEMBER(k), O(||k|| + ||PRED(k)|| + log N) for PRED(k),
  and O(||k|| + ||SUCC(k)|| + log N) for SUCC(k) on a RAM." Followed by: "In
  fact, on a RAM, we can solve this problem trivially by direct trie traversals
  without the log N additive factor".
- Theorem 5 (static, no compression, pp. 237-238): "MEMBER(k) runs in
  O(1 + log_B N + ||k||/B) memory transfers w.h.p., and PRED(k), and SUCC(k)
  run in O(1 + log_B N + ||k||/B + ||k'||/B) memory transfers w.h.p., where k'
  is the predecessor (resp. successor) of k. The operation RANGE-QUERY(k, k')
  runs in O(1 + log_B N + (||k|| + ||k'|| + ||Q||)/B) transfers, where Q is set
  of keys in the result."
- Theorem 7 (static, with front compression, p. 238): same bounds with
  `<<Q>>` in place of `||Q||`; "supports member, predecessor, successor,
  prefix, and range queries"; "The compressed keys can be decoded for an
  additional ||Q||/B transfers".
- Theorem 14 (dynamic, p. 242): "MEMBER(k) runs in O(1 + log_B N + ||k||/B)
  memory transfers w.h.p., and PRED(k), and SUCC(k) run in
  O(1 + log_B N + ||k||/B + ||k'||/B) memory transfers w.h.p., where k' is the
  predecessor (resp. successor) of k. The operation RANGE-QUERY(k, k') runs in
  O(1 + log_B N + (||k|| + ||k'|| + <<Q>>)/B) transfers w.h.p.. The compressed
  keys can be decoded for an additional ||Q||/B transfers. Finally INSERT(k)
  and DELETE(k) run in O(1 + log_B N + log^2 N ||k||/B) memory transfers
  w.h.p."
- Theorem 15 (p. 242): "The dynamic COSB-tree augmented with a scanning
  structure achieves the bounds from Theorem 14, except that range queries are
  amortized, and INSERT(k) and DELETE(k) are accelerated to
  O(1 + log_B N + log^{2+eps} log N ||k||/B) amortized memory transfers w.h.p.,
  for any eps > 0. If range queries only need to return pointers to keys, then
  the updates become O(1 + log_B N + ||k||/B) amortized memory transfers
  w.h.p." It is preceded by "details are left for the full version".
- Space (Summary of results, p. 235): "The space usage is O(<<D>>). In
  contrast, string B-trees and per-block front-compressed B-trees use more
  space, O(||D||) and O(min{||D||, B<<D>>}), respectively." Theorem 8: the
  dynamic front compression represents the keys "in size at most
  (1 + eps)<<D>> + N bits".

Internal discrepancies of the PDF (printed, not my inference).

- The "Summary of results" (p. 235) gives insertions as
  "O(1 + ||k||(log^2 <<D>>)/B + log_B N)", Theorem 14 as
  "O(1 + log_B N + log^2 N ||k||/B)" (`log^2 <<D>>` against `log^2 N`), and
  p. 239 gives "O(1 + ||k|| log^2 <<D>>/B) block transfers (amortized)".
- The summary gives "Searches and successor/predecessor queries" as
  "O(1 + ||k'||/B + log_B N)" (no `||k||` term); Theorems 5, 7, 14 have both
  `||k||/B` and `||k'||/B`.
- Theorem 14 does not contain the word "amortized", but its update bound rests
  on Lemmas 9, 12 and 13, all stated as amortized, and the introduction says
  the structure "is amortized for updates".
- The abstract says only: "The COSB-tree searches asymptotically optimally and
  inserts and deletes nearly optimally."

### 1.3 Verdict on lines 521-522: imprecise

What survives: the structure is dynamic, handles unbounded-length keys, and
membership costs `O(||k||/B + log_B N)`.

What "bounds of the same form" hides:

1. the bounds hold with high probability (the structure is randomized),
   whereas string B-trees are deterministic and worst-case;
2. they need the tall-cache assumption;
3. predecessor and successor have an additional term `||k'||/B`, linear in
   the length of the answer (string B-trees have no such term, see 2.2);
4. updates are amortized and, when the keys are kept sorted on disk (the
   version the paper presents), cost `O(log_B N + ||k|| log^2 N / B)`, not
   `O(log_B N + ||k||/B)`; the string B-tree update bound is matched only
   "in an amortized sense" by a variant that does not keep the keys in order
   (Theorem 15, proof deferred to a full version).

Proposed replacement for lines 520-522 (from "In the cache-oblivious model" to
"which are dynamic."):

```latex
In the cache-oblivious model, the same bound is obtained for prefix queries on a static set
by Brodal and Fagerberg~\cite{Brodal2006Cacheoblivious}. The cache-oblivious string B-trees of
Bender, Farach-Colton and Kuszmaul~\cite{Bender2006Cacheoblivious} are dynamic and randomized: under the tall-cache
assumption, with high probability they answer membership queries in $O(|x|/B+\lg_B n)$ I/Os and predecessor and successor
queries in $O((|x|+|x^\pm|)/B+\lg_B n)$ I/Os; updates are amortized, and require $O(|x|\lg^2 n/B+\lg_B n)$ I/Os
if the keys are kept sorted on disk, and $O(|x|/B+\lg_B n)$ I/Os otherwise.
```

(The statement about Brodal and Fagerberg was not checked against their paper;
Bender et al. describe it as "a static cache-oblivious string B-tree. It
supports cache-oblivious searches with the same bounds as the original string
B-tree but does not allow updates", p. 234.)

### 1.4 BibTeX entry

Authors, title ("Cache-Oblivious String B-trees"), pages 233-242 and year agree
with the PDF. The PDF prints only "PODS'06 June 26-28, 2006, Chicago, Illinois
USA" and "Copyright 2006 ACM 1-59593-318-2/06/0006": the ordinal "25th" in the
`booktitle` and the DOI are not printed and could not be checked. No correction.

### 1.5 Relevant and not yet in the Related work

- Dependence on the length of the answer. Both the RAM structure of Lemma 1
  and the COSB-tree have predecessor/successor bounds that are *linear* in the
  length of the predecessor/successor (`||PRED(k)||`, `||k'||/B`); in the
  z-fast trie the dependence is logarithmic (`\lg\max\{|x|,|x^+|,|x^-|\}`).
  This sharpens the remark at lines 514-517 ("nothing is known about the terms
  depending on $|x^-|$ and $|x^+|$"): other hash-based structures have such
  terms, and worse ones.
- Section 2 uses the same device as the signed z-fast trie: hashes of prefixes
  to compare a query with trie nodes, and a character-by-character comparison
  only on a hash match to turn Monte Carlo into Las Vegas. The search, however,
  is on a centroid decomposition of the trie (`O(log N)` steps), not on prefix
  lengths.
- The paper's own comparison supports the phrase "they depend on the knowledge
  of $B$" at line 520: "Both traditional and string B-trees are based on the
  assumption that there is a single block-transfer size B for which the data
  structure should be optimized" (p. 234).

---

## 2. Ferragina, Grossi, *The String B-tree* (JACM 1999)

BibTeX key `FeGSBT`. The PDF is the journal version, J. ACM 46(2), March 1999,
pp. 236-280 (45 pages).

### 2.1 Sentences of `dynzfast.tex`

Lines 519-520:

> \paragraph{Strings in external memory.} In the I/O model, string
> B-trees~\cite{FeGSBT} provide predecessor and successor queries, and updates,
> using $O(|x|/B+\lg_B n)$ I/Os, but they depend on the knowledge of $B$.

Lines 523 and 525-526 (quoted in 1.1) refer to it collectively. Line 237 of the
introduction mentions B-trees in general and cites nothing.

### 2.2 What the paper proves

Model (p. 239): two-level memory; "The external memory is assumed to be
partitioned into transfer blocks, called disk pages, each of which contains B
atomic items, like integers, characters and pointers." Lengths are in
characters. Strings are stored once and represented by "logical pointers".
Bounds are deterministic and worst-case.

Theorem 1, restated on p. 248 as "Theorem (Problem 1)":

> Let Delta be a set of k strings whose total length is N. Prefix Search(P)
> takes O((p + occ)/B + log_B k) worst-case disk accesses, where p = |P|. Range
> Query(K', K'') takes O((k' + k'' + occ)/B + log_B k) worst-case disk
> accesses, where k' = |K'| and k'' = |K''|. Inserting or deleting a string of
> length m takes O((m/B) + log_B k) worst-case disk accesses. The space
> occupied by the String B-tree built on Delta is Theta(k/B) disk pages and the
> space required by string set Delta is Theta(N/B) disk pages.

(The same bounds are listed on p. 239; Prefix Search is proved as Theorem 5 and
Range Query as Corollary 1, p. 260.)

Theorem 2, p. 251 ("Theorem (Problem 2)", all suffixes indexed): "Substring
Search(P) takes O((p + occ)/B + log_B N) worst-case disk accesses [...]
Inserting a string of length m in Delta or deleting it takes
O(m log_B(N + m)) worst-case disk accesses. The space occupied by both the
String B-tree and the string set Delta amounts to Theta(N/B) disk pages."

Predecessor and successor. The paper never names these operations. What it
proves is Lemma 7 (p. 258): `SB-Search-Down(P, pi, l)` "returns triplet
(tau, j, lcp), where tau is the leaf containing P's position in K, j is P's
position in string set S_tau and lcp = max_lcp(P, K). It costs
(lcp - l)/B + O(log_B k) disk accesses", and in the proof of Theorem 5 the
position of P in the sorted set is found with "O((p/B) + log_B k) disk
accesses (because lcp <= p; see Lemma 7)". P's position is defined by its two
neighbours (`K_{j-1} <_L P <=_L K_j`), and leaves are doubly linked
(`next`/`prev`, p. 257). *Inference*: predecessor and successor of x are
obtained in `O(|x|/B + log_B k)` worst-case I/Os, independently of the length
of the answer (the blind search reads at most lcp + 1 characters of one string
per node).

Other statements.

- Lower bound, Lemma 8 (p. 261): "For a large alphabet Sigma whose characters
  can only be accessed by comparisons, searching for a pattern P and listing
  all of its occ occurrences in K's strings requires
  Omega(p/B + max{occ/B, log_B k}) worst-case disk accesses" (external-memory
  pointer machine).
- Dependence on B: each node stores `b <= |S_pi| <= 2b` strings, "where
  b = Theta(B) is an even integer properly chosen to let a single node fit into
  a disk page" (p. 242).
- RAM (Section 1.2, p. 240-241): "Fixing B = O(1), the String B-tree can be
  seen as an augmented 2-3-tree"; search in "O(p + log N + occ)" time
  (characters are not packed).
- CPU time (p. 240): "O(Bd) time if d disk pages are read or written".

### 2.3 Verdict on lines 519-520: correct in substance, slightly imprecise

- `O(|x|/B + \lg_B n)` for updates: correct (Theorem 1), worst case.
- "predecessor and successor queries": not a statement of the paper, but a
  direct consequence of Lemma 7 (see above). The operations the paper states
  are prefix search and range queries, which have an additional `occ/B` term.
- "they depend on the knowledge of $B$": correct.
- Not said in the sentence and worth saying, as it is the main difference with
  the cache-oblivious string B-tree and with the z-fast trie: the bounds are
  deterministic and worst-case, and the tree uses `Theta(n/B)` pages besides
  the strings.
- Units: in the paper `|x|` and `B` are measured in characters (atomic items);
  the sentence is consistent with the use of `|x|/B` elsewhere in
  `dynzfast.tex` only if `B` is read in the same unit as `|x|`.

Proposed replacement for lines 519-520 (optional):

```latex
\paragraph{Strings in external memory.} In the I/O model, string B-trees~\cite{FeGSBT} locate a string $x$ in a sorted set of $n$ strings
(thus providing predecessor and successor queries), and insert or delete it, using
$O(|x|/B+\lg_B n)$ I/Os, deterministically and in the worst case, with $O(k/B)$ additional I/Os to report the $k$ results of a prefix or range query; 
besides the strings, they use $\Theta(n/B)$ blocks. They depend, however, on the knowledge of $B$.
```

### 2.4 BibTeX entry

Authors, title, journal, volume 46, number 2, year 1999, pages 236-280: all
agree with the first page ("Journal of the ACM, Vol. 46, No. 2, March 1999,
pp. 236-280"). The entry has no `doi`; the PDF does not print one (the DOI
10.1145/301970.301973 comes from `TO-DOWNLOAD.md`). Optional additions:
`month = mar`, `doi = {10.1145/301970.301973}`.

### 2.5 Relevant and not yet in the Related work

- The string B-tree predecessor bound has no term depending on the length of
  the predecessor or successor, unlike ours and unlike the COSB-tree: this is
  the one respect in which its bounds are better in form than those of
  Theorem `th:boundslong` (compare the remark on Ruzic at lines 478-480).
- Bender et al. (p. 234, footnote 1) remark that the string B-tree range query
  returns pointers: "actually obtaining the various strings may require one
  extra block transfer per key".

---

## 3. Fischer, Gawrychowski, *Alphabet-dependent string searching with wexponential search trees*

BibTeX key `Fischer2015Alphabet`. **The PDF is the published CPM 2015 version**
(Springer, LNCS 9133, pp. 160-171, 12 pages; the first page prints
"F. Cicalese et al. (Eds.): CPM 2015, LNCS 9133, pp. 160-171, 2015. DOI:
10.1007/978-3-319-19929-0_14"), not the arXiv version. It cites the arXiv
version as its full version: "[7] Fischer, J., Gawrychowski, P.:
Alphabet-dependent string searching with wexponential search trees. CoRR
abs/1302.3347 (2013)".

### 3.1 Sentences of `dynzfast.tex`

Lines 498-500:

> Several results take the size $\sigma$ of the alphabet into account. Fischer
> and Gawrychowski~\cite{Fischer2015Alphabet} describe a deterministic
> compacted trie that answers prefix and predecessor queries for a pattern of
> $m$ characters in time $O(m+\lg\lg\sigma)$, and a dynamic variant with a
> slightly larger additive term; characters, however, are handled one at a
> time.

### 3.2 What the paper proves

"all our results are in the word RAM model" (Section 1.1, p. 161).

Theorem 1 (p. 161):

> A compacted trie storing n static strings over an integer alphabet of size
> sigma can be stored in O(n) space (in addition to the stored strings
> themselves) such that subsequent prefix- or predecessor queries can be
> answered in O(m + lg lg sigma) deterministic worst-case time, for patterns of
> length m. This data structure can be constructed in deterministic O(n) time,
> for sigma = n^{O(1)}.

Theorem 2 (p. 162):

> We can maintain a linear-size (in addition to the stored strings themselves)
> structure for a compacted trie storing strings over an integer alphabet of
> size sigma under adding a new string in deterministic worst-case time
> O(m + lg^2 lg sigma / lg lg lg sigma) so that subsequent on-line prefix- or
> predecessor queries can be answered in O(m + lg^2 lg sigma / lg lg lg sigma)
> deterministic time, for strings of length m and sigma = n^{O(1)}.

- The dynamic bound of the published version is therefore
  `O(m + lg^2 lg sigma / lg lg lg sigma)`, the same as arXiv v1 according to
  `TO-DOWNLOAD.md`; the `O(m + lg lg sigma)` reported there from a survey is
  the *static* bound.
- Deletions: **not supported, and not mentioned**. Theorem 2 covers only
  "adding a new string"; the words "delete", "deletion", "remove" do not occur
  anywhere in the PDF. (The abstract says "per query or update".) The weighted
  search trees of Theorem 3 support only insertions and weight increases.
- Deterministic, worst case: stated in both theorems (for queries of Theorem 2
  the text says "deterministic time"; Section 4.3 is titled "Worst-Case Version
  of Theorem 2").
- Characters: the paper does not say "one at a time" in so many words. The
  additive term is `m`, the number of characters, with no division by the
  number of characters per word; packed strings are never mentioned; and the
  search is described as "matching a pattern P[1, m] reduces to repeatedly
  finding correct edges: assuming inductively that we have already matched
  P[1, i] and are currently at node v, we check if the edge (v, P[i + 1])
  exists" (Section 3, p. 163). So "characters are handled one at a time" is a
  correct reading, but an inference.
- Previous work as stated by the paper (p. 161): "For dynamic tries, Andersson
  and Thorup [2] show how to update or search the trie in
  O(m + sqrt(lg n / lg lg n)) deterministic worst-case time, for n being the
  number of stored strings", which agrees with lines 484-487.

### 3.3 Verdict on lines 498-500: correct, but vague in two points

- "a dynamic variant": it is insertion-only. As the text of the paragraph then
  compares with structures supporting "updates", this should be said.
- "a slightly larger additive term": correct; the term can now be stated.
- "deterministic": correct; worst case can be added. Space is linear in `n`
  besides the strings; both theorems assume `sigma = n^{O(1)}` (for the
  construction time in Theorem 1).

Proposed replacement for lines 498-500:

```latex
Several results take the size $\sigma$ of the alphabet into account. Fischer and Gawrychowski~\cite{Fischer2015Alphabet} 
describe a deterministic compacted trie that answers prefix and predecessor queries for a pattern of $m$ characters in time $O(m+\lg\lg\sigma)$
in the worst case, and a variant supporting insertions (but not deletions) in which queries and insertions 
take time $O\bigl(m+\lg^2\lg\sigma/\lg\lg\lg\sigma\bigr)$; characters, however, are handled one at a time. 
```

### 3.4 BibTeX entry

Authors, title, pages 160-171, year, DOI agree with the first page. Missing:
`volume = {9133}` (the entry has `series` but no volume). The `note`
(arXiv:1302.3347) agrees with reference [7] of the paper. The first page shows
the second author as "Pawel Gawrychowski" with the Polish l-stroke, as in the
entry.

### 3.5 Relevant and not yet in the Related work

- These bounds depend on `m` and `sigma` only, not on `n` (but both theorems
  assume `sigma = n^{O(1)}`). This does not conflict with the priority claim at
  lines 383-385 (the paper is from 2013/2015, and is insertion-only), but it is
  a second dynamic structure for strings whose bounds do not contain `n`; on a
  binary alphabet its bound is `O(m)` with `m` in bits.

---

## 4. Wu, Ni, Jiang, *Wormhole* (EuroSys 2019)

BibTeX key `WNJWormhole`. The PDF is the publisher's version (ACM `acmart`
layout, 16 pages, "EuroSys '19, March 25-28, 2019, Dresden, Germany",
"ACM ISBN 978-1-4503-6281-8/19/03", https://doi.org/10.1145/3302424.3303955).

### 4.1 Sentences of `dynzfast.tex`

Lines 538-548:

> (a) The structure closest to ours is Wormhole~\cite{WNJWormhole}: the keys
> are stored in a linked list of leaves containing at most $128$ keys, each
> leaf is associated with a short string, its \emph{anchor}, that separates its
> keys from those of the previous leaf, and all the prefixes of all anchors are
> stored in a hash table; the leaf associated with a key is found by a binary
> search on the prefix length, which uses short hashes as signatures and
> performs a single string comparison at the end, as we do in
> Section~\ref{sec:sign}.
> (b) The authors credit Waldvogel \textit{et al.}~\cite{Waldvogel1997Scalable}
> for the binary search on prefix lengths; x-fast, y-fast and z-fast tries are
> not mentioned.
> (c) In fact, the organization of Wormhole is that of a y-fast trie for
> variable-length strings of bytes: buckets of keys, and a structure storing
> all prefixes of one representative per bucket.
> (d) Since a hash table contains all the prefixes of the anchors, the number
> of its entries, and the cost of splitting a leaf, depend on the length of the
> anchors, whereas a z-map on the same anchors would contain one entry per
> anchor.
> (e) On the other hand, as all prefixes are present a plain bisection
> suffices.
> (f) As only one string per leaf is indexed, the hash table is a small
> fraction of the whole index~\cite{WNJWormhole}; the same approach can of
> course be applied to z-fast tries.

Lines 2383-2384:

> (g) The third one, \texttt{ZFastSet}, has the organization of a y-fast trie,
> and thus of Wormhole~\cite{WNJWormhole}: the keys are stored in buckets of at
> most $64$ keys, and a \texttt{ZFastTrie} indexes the shortest strings
> separating consecutive buckets.

Lines 2388-2416 contain experimental comparisons with the Wormhole software;
they make no claim about the paper.

### 4.2 What the paper says

Reference list (48 entries, pp. 14-16). It contains **no** reference to
Willard, to van Emde Boas, or to any paper by Belazzougui, Boldi, Pagh or
Vigna; the strings "x-fast", "y-fast", "z-fast" do not occur in the PDF. The
only reference for the search technique is "[45] Marcel Waldvogel, George
Varghese, Jon Turner, and Bernhard Plattner. 1997. Scalable High Speed IP
Routing Lookups. In Proceedings of the ACM SIGCOMM '97 Conference [...]
25-36. https://doi.org/10.1145/263105.263136", cited in Section 2.4:
"Waldvogel et al. proposed to use binary search on prefix lengths to accelerate
the match for routing table lookups [45]. To apply the approach, we insert all
prefixes of each anchor into a hash table."

MetaTrieHT (Section 2.4, p. 5).

- All prefixes of all anchors: "we insert all prefixes of each anchor into a
  hash table. In Figure 4's index, "Jam" is an anchor, and accordingly its
  prefixes ("", "J", "Ja", "Jam") are inserted in the hash table." Tokens are
  bytes in the implementation ("a string of tokens, e.g., a byte string",
  p. 2).
- "Each node in MetaTrie corresponds to an item in the hash table. If the node
  represents an anchor, or a leaf node, the hash item is a leaf item [...]
  Otherwise, the node is an internal node". An item has a bitmap of the
  existing children ("sibling(s) of an unmatched token can be located in O(1)
  time") and pointers to the leftmost and rightmost leaf of its subtree
  ("the second phase takes a constant time").
- Binary search: Algorithm 1 ("Binary Search on Prefix Lengths") is a plain
  bisection between 0 and `min(L_anc, L_key) + 1`, where `L_anc` is "the length
  of the longest anchor key"; "the longest prefix can be found in
  O(log(min(L_anc, L_key))) time".

Anchors (Section 2.2, pp. 3-4). Two conditions: "The Ordering Condition:
left-key < anchor-key <= node-key, where left-key represents any key in the
node (Node_a) immediately left to Node_b, and node-key represents any key in
Node_b" and "The Prefix Condition: An anchor key cannot be a prefix of another
anchor key." "In the formation of an anchor key, we aim to minimize the key
length": if the largest key of the left leaf is `P_1...P_k A_1...A_n` and the
smallest key of the leaf is `P_1...P_k B_1...B_m`, the anchor is
`P_1...P_k B_1` (common prefix plus one token), with the smallest token
appended if needed for the prefix condition; the anchor of the leftmost leaf is
the smallest token alone. Section 3.3 removes the assumption of a reserved
token (trailing zero bytes, and "fat" leaves that are not split when no valid
anchor exists).

Leaves. "As all leaf nodes are doubly linked" (p. 4). "In the Wormhole
prototype we use 128 as the maximum leaf-node size (number of keys in a leaf
node)" (Section 4, p. 9). Each leaf has an array of 16-bit tags sorted by hash
and a key array sorted lazily (Section 3.2).

Stated complexity.

- Lookup: abstract: "takes O(log L) worst-case time for looking up a key with a
  length of L"; Section 1: "a lookup cost of O(log L) memory accesses";
  Section 2.4: `O(log(min(L_anc, L_key)))` for the longest prefix match;
  Section 2.4: "The O(log L) time cost of Wormhole is asymptotically lower than
  O(log N) for B+ tree and O(L) for trie". The cost of hashing is discussed
  separately (Section 3.1): with incremental hashing "the average number of
  tokens used for hashing in a lookup of a search key of length L is reduced
  from (L/2) log_2 L to only L".
- Split and merge: **no asymptotic cost is stated**. Algorithm 4 (`split`)
  executes "for plen : 0 to key.len-1" with one `htGet` and possibly `htSet`
  per prefix of the new anchor, and `merge` a loop "for plen : key.len-1 to 0"
  over the prefixes of the removed anchor. So a split performs a number of
  hash-table operations linear in the anchor length (the statement is in the
  pseudocode, the "linear" is my reading). The text says only: "The only
  operations in the MetaTrie are removing anchors for merged nodes or adding
  new anchors for split nodes" (p. 5).
- Dependence on anchor length: "Its search time is practically independent of
  number of keys in the index, and is only proportional to anchor lengths"
  (for the trie, p. 5); "another factor affecting Wormhole's lookup efficiency
  is anchor length, which determines the MetaTrieHT's size and lookup time in
  MetaTrieHT" (Section 4.2, p. 12); "In Url the URLs often share long common
  prefixes, which leads to long anchors (about 40 B in average as measured)"
  (p. 11; the Url keyset has "avg. length: 82 B", Table 1).

Size of the hash table (Section 2.5, p. 7, in the description of the second
copy of the table kept for RCU): "The extra space used by the second MetaTrieHT
is negligible because a MetaTrieHT, containing only the anchor keys, is
consistently small in size compared with the size of the entire index
structure. As an example, for the eight keysets used in our evaluation (see
Table 1), the extra space consumed by the second table is only 0.34% to 3.7% of
the whole index size."

Hash function and signatures (Section 3.1, p. 8). "CRC-32c is used in our
implementation" (footnote 2), chosen because it is incremental; "Each element
in the array consists of a 16-bit tag hashed from the prefix and a 48-bit
pointer to the original prefix"; "we first optimistically trust all tag-matches
and omit key-comparisons in every hash-table lookup until finding a seemingly
correct LPM. Tag comparisons may produce false-positive matches, which can lead
the binary search to a wrong prefix that is longer than the correct one. To
detect this error, a full key comparison is performed at the last prefix after
the binary search. If it is a mismatch, the search will start over with full
prefix comparisons." Leaf tags: "Using the lower 16-bits of a (CRC-32c) hash
value as the tag" (Section 3.2).

Article number: the PDF prints none (the ACM reference format says only
"ACM, New York, NY, USA, 16 pages"), and its pages carry no page numbers.

### 4.3 Verdicts

| Sentence | Verdict | Notes |
|---|---|---|
| (a) | correct | "128" is the value used "in the Wormhole prototype"; the list is doubly linked; after the binary search there is a constant-time step (bitmap of children, leftmost/rightmost pointers) to reach the leaf. On a failed final comparison the search is repeated "with full prefix comparisons". |
| (b) | correct | Reference [45] is the SIGCOMM '97 paper, i.e., exactly `Waldvogel1997Scalable`. |
| (c) | correct as a description (it is the authors' reading of dynzfast, not a statement of Wormhole) | The "representative" is not a key of the bucket, but a separator derived from two adjacent keys. |
| (d) | correct | Supported by Algorithm 4 and by the quotes on anchor length; the paper states no cost for split. |
| (e) | correct | Algorithm 1. |
| (f) | correct | The paper's figure is 0.34% to 3.7% of the whole index on its eight keysets. |
| (g) | correct | |

No replacement is needed. Two optional sharpenings:

For (d), the paper provides a number that makes the comparison concrete:

```latex
Since a hash table contains all the prefixes of the anchors, the number of its entries, and the cost of splitting a leaf, depend on the 
length of the anchors (about $40$ bytes on average on the URLs used in~\cite{WNJWormhole}), whereas a z-map on the same anchors would contain one entry per anchor.
```

For (f):

```latex
As only one string per leaf is indexed, the hash table is a small fraction of the whole index (from $0.34$\% to $3.7$\% on the data sets of~\cite{WNJWormhole}); 
```

### 4.4 BibTeX entry

Authors, title, `booktitle`, year, DOI agree with the PDF. Remarks:

- `pages = {1--16}`: the PDF prints "16 pages" and no page numbers or article
  number. If the bibliography style allows it, `numpages = {16}` or
  `note = {16 pages}` is closer to what is printed; an article number cannot be
  confirmed from the PDF.
- `address = {Dresden, Germany}` is the place of the conference; the publisher
  address printed is "ACM, New York, NY, USA".
- `note = {arXiv:1805.02200}` cannot be checked from the PDF.

### 4.5 Relevant and not yet in the Related work

- Priority claim (Section 6): "To the best of our knowledge, Wormhole is the
  first ordered key-value index achieving the O(log L) lookup cost, which is
  better than the O(log N) or O(L) cost of other ordered indexes". Line 542
  says only that x-fast, y-fast and z-fast tries "are not mentioned"; the
  paper goes further and claims the bound as new.
- The `O(log L)` of Wormhole counts hash-table probes with `L` in bytes and
  excludes the `O(L)` cost of hashing, which the paper treats in Section 3.1;
  the corresponding bound of Theorem `th:boundslong` includes it
  (`O(|x|/w + \lg|x|)`).
- The search is bounded by the length of the longest anchor, not only by the
  key length (`min(L_anc, L_key)`); the analogous refinement for the z-fast
  trie would be the length of the longest extent of an internal node.
- The paper reports that with 16-bit tags the probability that a lookup has to
  be repeated "is only 0.0153% for keys of 1024-bytes long
  (1 - ((2^16 - 1)/2^16)^10)".

---

## 5. Waldvogel, Varghese, Turner, Plattner, *Scalable high-speed prefix matching* (ACM TOCS 2001)

**The PDF is the journal version** (ACM Transactions on Computer Systems,
Vol. 19, No. 4, November 2001, pages 440-482, 43 pages), whose first page says
"Parts of this paper were presented in ACM SIGCOMM '97 [Waldvogel et al.
1997]". **`dynzfast.tex` cites the SIGCOMM '97 paper** (key
`Waldvogel1997Scalable`); `biblio.bib` has no entry for the journal version
(there is one, `Waldvogel2001Scalable`, in
`related-work/practical-indexes.bib`). Everything below was verified on the
journal version only; whether the SIGCOMM '97 version contains the same
statements (in particular the pseudocode of Figure 19) cannot be decided from
the PDFs available.

### 5.1 Sentences of `dynzfast.tex`

Lines 458-469:

> (a) The idea of locating the longest prefix of a query that appears in a set
> of strings by a binary search on its \emph{length}, using hashing to test
> whether a prefix appears, goes back to x-fast tries~\cite{WilLLWRQ}; it was
> rediscovered in the context of IP routing by Waldvogel
> \textit{et al.}~\cite{Waldvogel1997Scalable}, who use a hash table for each
> prefix length.
> (b) For the search to work, a failed probe must imply that no longer prefix
> exists: x-fast tries achieve this by storing \emph{all} prefixes of all keys,
> whereas Waldvogel \textit{et al.}~store, for each string of the set, a
> \emph{marker} at each length that a binary search might probe before reaching
> the length of the string, which means up to $\lg L$ markers for strings of
> length at most $L$.
> (c) In both cases the number of entries stored per string depends on the
> length of the strings.
> (d) [...] In fact, the lengths at which Waldvogel \textit{et al.}~place the
> markers for a string of length $l$ are obtained by clearing, one at a time,
> the lowest bits set in $l$, and the handle length of a node whose extent has
> length $l$ is the last number of this sequence (starting from $l$ itself)
> that belongs to the skip interval: the fat binary search described in this
> paper makes the search work with this entry only.

Line 542 (sentence (b) of Section 4.1 above) cites the same key for what
Wormhole credits.

### 5.2 What the paper says

Citations. The reference list contains "VAN EMDE BOAS, P. 1975. Preserving
order in a forest in less than logarithmic time. In Proceedings of the 16th
Annual Symposium on Foundations of Computer Science (1975), pp. 75-84" and
"VAN EMDE BOAS, P., KAAS, R., AND ZULSTRA, E. 1977. Design and implementation
of an efficient priority queue. Mathematical Systems Theory 10, 99-127" (sic,
"Zulstra"). They are cited once, in Section 2.3 "Binary Search on Prefix
Lengths" (p. 445): "The prior work closest to binary search on prefix lengths
occurs in computational geometry. De Berg et al. [1995] describe a scheme for
one-dimensional point location based on stratified trees [van Emde Boas 1975;
van Emde Boas et al. 1977]. [...] The actual search is not performed on a
prefix trie, but on a balanced interval tree." **Willard is not cited**, and
x-fast or y-fast tries are not mentioned. The conclusions say (p. 479): "we
have discovered an efficient algorithm that scales with the logarithm of the
address size and so is close to the theoretical limit of O(log log N)".

Hash tables. "a novel algorithmic solution to longest prefix match, using
binary search over hash tables organized by the length of the prefix"
(p. 442); "each set of prefixes of distinct length is organized as a hash
table" (Section 3.1), with a sorted array `L` of the distinct lengths present.
Section 7 adds: "It is possible to put all the hash entries of all prefix
lengths into one big hash table, by using just one more bit for the address and
setting the first bit below the prefix length to 1."

The search (basic scheme, Section 3). Binary search on the array `L` of the
`W_dist <= W` *distinct lengths present in the database*, not on all lengths:
"Let i correspond to the middle level in range R; Extract the first
L[i].length bits of D" (Figure 7); it takes "O(log_2 W_dist) time"; and "the
search can be limited to those prefix lengths which do contain at least one
entry", an improvement "which has already been mentioned and used in the basic
scheme" (Section 4.1). A hit sends the search to longer lengths, a miss to
shorter ones.

Markers.

- Where (Section 3.2, p. 450): "Naively, it seems that for every entry, there
  would be a marker at all other prefix lengths [...] Luckily, markers do not
  need to be placed at all levels. [...] any search that will end up at a given
  node only has a single path to choose from, eliminating the need to place
  markers at any other levels." Section 5.1: "Since markers only need to be set
  at shorter prefix lengths, any parent associated with longer prefixes is
  ignored." So the markers of a prefix are at the nodes of its search path that
  are *shorter* than it.
- How many: "each entry contributes at most, log_2 W markers" (p. 451); "With W
  bits length, each entry could possibly require up to log(W) - 1 markers (the
  entry itself is the log W th entry)" (p. 461). Section 8.1: "adding markers
  could extend the number of entries required by a factor log_2 W, in the
  typical case, many prefixes will share markers (Table VI)", with a worst-case
  example needing "O(log_2 W) markers per prefix". In Table VI, for Mae-East,
  38031 entries request 50877 markers, of which 13584 are distinct.
- Bit clearing. Figure 19 ("Building for the basic scheme", p. 464; transcribed
  from the page image) inserts a prefix P of length L, with "i [...] the index
  for the L's hash table", and then:

  ```
  (* Now insert all necessary markers "to the left" *)
  For ever do
    (* Go up one level in the binary search tree *)
    Clear the least significant set bit in i;
    If i = 0 then break; (* end reached *)
    Set L to the appropriate length for i;
    Shorten P to L bits;
    If there is already an entry for P at i then
      Make it a marker if it isn't already;
      break; (* higher levels already do have markers *)
    Else
      Create a new marker M for P at i's hash table;
      Set M.bmp to B;
    Endif
  Endfor
  ```

  and Section 5.1 continues: "To build a basic search structure that eliminates
  unused levels, or to take advantage of asymmetries, it is necessary to first
  build the binary search tree. Then, instead of clearing the least significant
  bit, as outlined in Figure 19, the build algorithm has to follow the binary
  search tree back up to find the "parent" prefix length. Some of these parents
  may be at longer prefix lengths, as illustrated in Figure 5."
- Markers are not enough: a marker can be a false lead (Section 3.3, "Problems
  with Backtracking": with backtracking "The worst case is O(W)"). Each marker
  M therefore stores "M.bmp, which is the value of the best matching prefix of
  the marker M", "precomputed when the marker M is inserted" (Section 3.4).

The search tree is not the plain bisection in the refined schemes (Section 4).

- Asymmetric binary search (4.1): the tree over the lengths is unbalanced
  according to the distribution of lengths or of traffic.
- Mutating binary search (4.2): "whenever we get a match and move to a new
  subtrie, we only need to do binary search on the levels of new subtrie";
  "each entry E in the search table could contain a description of a search
  tree specialized for all prefixes that start with E".
- Rope search (4.2.2): the description stored in an entry is its *rope*, "the
  sequence of levels which binary search on a given subtrie will follow on
  repeated failures to find a match"; "For example, using 32 bit IPv4
  addresses, the starting rope contains the starting level 16, followed by
  Levels 8, 4, 2, 1." A hit replaces the current rope with that of the entry
  found (Figure 16). Optimal ropes are computed by dynamic programming in
  `O(N W^3)` time (Section 5.2); "A node will be used as a marker if the
  higher-level rope lists its prefix length."
- Updates (Table VII): search `O(log W)`, memory `O(N log W)`, update `O(N)`
  in the worst case, because the `bmp` of all markers below an inserted or
  deleted prefix must change; with marker partitioning (Section 6) update
  `O(alpha N^{1/alpha} W log W)` (as I read the table: the radical appears to
  cover N only) with search `O(alpha + log W)`. In that table "W is the number
  of distinct lengths".

### 5.3 Verdicts

(a), lines 458-460: **correct**. "rediscovered" is supported: neither Willard
nor x-fast tries are mentioned, even in the journal version. The journal
version does cite van Emde Boas (through De Berg et al.) as "the prior work
closest to binary search on prefix lengths"; this answers the open question of
`TO-DOWNLOAD.md` and contradicts nothing in the text
(`related-work/practical-indexes.md` states that the SIGCOMM '97 version cites
neither). "who use a hash table for each prefix length" is correct. Optional
footnote:

```latex
\footnote{The journal version~\cite{Waldvogel2001Scalable} mentions the stratified trees of van Emde Boas~\cite{EKZDIEPQ} as the closest 
previous work, but not x-fast tries.}
```

(b), lines 461-463: **slightly imprecise**. A binary search for a string of
length 5 in the tree of Figure 5 probes lengths 4, 6, 5: it "might probe" 6
"before reaching the length of the string", but no marker is placed there.
Markers are only at the *shorter* lengths of the search path. "up to $\lg L$
markers" agrees with the paper ("at most, log_2 W markers"; more precisely
`log W - 1` besides the entry). The sentence also omits that markers alone do
not make the search correct (a marker can lead to a longer length where nothing
matches): each marker carries the precomputed best matching prefix. This second
point is specific to longest-prefix matching on a set that is not prefix-free,
so it may be left out. Proposed replacement for the clause at lines 462-463:

```latex
\emph{all} prefixes of all keys, whereas Waldvogel \textit{et al.}~store, for each string of the set, a \emph{marker} at each shorter length that the binary search 
probes before reaching the length of the string, which means up to $\lg L$ markers for strings of length at most $L$. 
```

(c), line 464: **correct** for the worst case; the paper adds that markers are
shared among prefixes.

(d), lines 466-469: **confirmed, with two qualifications**.

1. The claim is literally in the paper ("Clear the least significant set bit in
   i", Figure 19), but only for the *basic scheme*, and `i` there is the index
   of a hash table in the array of lengths, which coincides with the length
   only if no length is skipped. For the schemes the paper actually recommends
   (unused lengths eliminated, asymmetric trees, rope search) the markers are
   at the shorter lengths of a search path that is not obtained by clearing
   bits, and Section 5.1 says so explicitly.
2. It is in the TOCS 2001 version, on a page (464) of material on building and
   updating; the citation in the text is to the SIGCOMM '97 paper.

   *Inference*: clearing the lowest set bit yields the shorter ancestors of `l`
   in the search path only when the lengths form a complete tree, i.e., for
   lengths in `[1..2^k)` bisected at `2^(k-1)` (as in Figures 4 and 5, with
   lengths 1-7); for `l = 2^k` itself (length 32 with the starting rope 16, 8,
   4, 2, 1 extended to 32 lengths) the rule gives no marker at all. The paper
   calls Figure 19 an outline and does not discuss this.

The second half of the sentence (the handle length is the last element of the
sequence in the skip interval) is a statement about 2-fattest numbers, not
about the paper; it is consistent with the definition (the 2-fattest number of
`(a..l]` is the smallest number greater than `a` obtained from `l` by clearing
low-order set bits).

Proposed replacement for lines 466-469 (from "In fact, the lengths" to the end
of the paragraph):

```latex
is the $2$-fattest number in the skip interval of the node. In fact, in the basic scheme of Waldvogel \textit{et al.}, when the binary search 
is a plain bisection on all lengths smaller than a power of two, the lengths at which the markers for a string of length $l$ are placed 
are obtained by clearing, one at a time, the lowest bits set in $l$~\cite{Waldvogel2001Scalable} (their optimized variants skip the lengths 
that do not appear in the set and adapt the sequence of probes to the prefixes matched so far, so the markers are placed along a different search path), 
and the handle length of a node whose extent has length $l$ is the last number of this sequence (starting from $l$ itself) that
belongs to the skip interval: the fat binary search described in this paper makes the search work with this entry only.
```

Line 542: **correct** (Wormhole's reference [45] is the SIGCOMM '97 paper).

### 5.4 BibTeX entries

`Waldvogel1997Scalable` describes the SIGCOMM '97 paper, which is not among
the PDFs: it cannot be checked against its own first page. Two reference lists
in the PDFs read give its title differently: the authors' own (TOCS, p. 482)
"Scalable high speed IP routing table lookups. In Proceedings of ACM SIGCOMM
'97 (Sept. 1997), pp. 25-36", and Wormhole's [45] "Scalable High Speed IP
Routing Lookups [...] 25-36. https://doi.org/10.1145/263105.263136". The entry
agrees with the second (which is also the form I remember from the ACM Digital
Library, without "table"; not verified here). Pages agree in both.

The journal version needs an entry; first-page data: authors Marcel Waldvogel
(IBM Zurich Research Laboratory), George Varghese (University of California,
San Diego), Jon Turner (Washington University in St. Louis), Bernhard Plattner
(ETH Zurich); title "Scalable High-Speed Prefix Matching"; "ACM Transactions on
Computer Systems, Vol. 19, No. 4, November 2001, Pages 440-482". The entry in
`related-work/practical-indexes.bib` agrees with all of this; its DOI
(10.1145/502912.502914) is not printed in the PDF.

### 5.5 Relevant and not yet in the Related work

- The scheme is dynamic and its search bound (`O(log W)` probes) does not
  depend on the number of prefixes, but its worst-case update bound does
  (`O(N)`, Table VII), and it solves longest-prefix matching, not predecessor
  search: it does not conflict with the claim at lines 383-385.
- Rope search is a second precedent, besides the markers, for a sequence of
  probes that is not a bisection of the current interval of lengths and that is
  stored in the entry found; in the z-fast trie the next probe is computed from
  the extent length of the node found, with no stored sequence.
- The paper already notes that the per-length tables can be merged into a
  single hash table (Section 7), which is what the z-map does.

---

## Proposed edits to `dynzfast.tex`

Ordered by importance. "Old" texts are abridged with [...]; the full
replacement texts are in the sections above.

| # | Lines | Old text | New text | Reason |
|---|---|---|---|---|
| 1 | 520-522 | "In the cache-oblivious model, the same bound is obtained [...] and bounds of the same form are obtained by the cache-oblivious string B-trees of Bender, Farach-Colton and Kuszmaul~\cite{Bender2006Cacheoblivious}, which are dynamic." | Section 1.3: randomized, tall cache, w.h.p.; membership `O(|x|/B+\lg_B n)`; predecessor/successor `O((|x|+|x^\pm|)/B+\lg_B n)`; amortized updates `O(|x|\lg^2 n/B+\lg_B n)` with sorted keys, `O(|x|/B+\lg_B n)` otherwise. | Theorems 14 and 15: the bounds are not of the same form as those of string B-trees for updates and for predecessor/successor, and they are w.h.p., amortized, under the tall-cache assumption. |
| 2 | 466-469 | "In fact, the lengths at which Waldvogel \textit{et al.}~place the markers for a string of length $l$ are obtained by clearing, one at a time, the lowest bits set in $l$, and [...]" | Section 5.3 (d): restrict to the basic scheme with a plain bisection on all lengths smaller than a power of two, cite `Waldvogel2001Scalable`, mention that the optimized variants place markers along a different search path. | Figure 19 of the TOCS version confirms the claim verbatim, but only for the basic scheme; Section 5.1 of the same paper says the rule does not apply when unused lengths are eliminated or the tree is asymmetric; the statement is not verified in the SIGCOMM '97 paper currently cited. |
| 3 | 498-500 | "[...] and a dynamic variant with a slightly larger additive term; characters, however, are handled one at a time." | Section 3.3: "in the worst case, and a variant supporting insertions (but not deletions) in which queries and insertions take time $O\bigl(m+\lg^2\lg\sigma/\lg\lg\lg\sigma\bigr)$; characters, however, are handled one at a time." | Theorem 2 of the published version: insertion-only, exact term. |
| 4 | 462-463 | "a \emph{marker} at each length that a binary search might probe before reaching the length of the string" | "a \emph{marker} at each shorter length that the binary search probes before reaching the length of the string" | Markers are placed only at shorter lengths (Sections 3.2 and 5.1); the search for a string also probes longer lengths. |
| 5 | 519-520 | "string B-trees~\cite{FeGSBT} provide predecessor and successor queries, and updates, using $O(|x|/B+\lg_B n)$ I/Os, but they depend on the knowledge of $B$." | Section 2.3 (optional): add "deterministically and in the worst case", the `O(k/B)` term of prefix and range queries, and the `\Theta(n/B)` blocks of space. | The sentence is correct; the paper states prefix search, range queries and updates, and the predecessor bound follows from Lemma 7. Worst-case determinism is the contrast with edit 1. |
| 6 | 460 | "[...] by Waldvogel \textit{et al.}~\cite{Waldvogel1997Scalable}, who use a hash table for each prefix length." | Optional footnote of Section 5.3 (a): the journal version mentions van Emde Boas's stratified trees as the closest previous work, but not x-fast tries. | Settles the open point of `TO-DOWNLOAD.md`; "rediscovered" stands. |
| 7 | 545-547 | "[...] depend on the length of the anchors, whereas [...]" and "the hash table is a small fraction of the whole index~\cite{WNJWormhole}" | Optional, Section 4.3: add "(about $40$ bytes on average on the URLs used in~\cite{WNJWormhole})" and "(from $0.34$\% to $3.7$\% on the data sets of~\cite{WNJWormhole})". | Numbers printed in the paper (Sections 4.1 and 2.5); the current text is correct. |
| 8 | 542 | "x-fast, y-fast and z-fast tries are not mentioned." | Optional: "x-fast, y-fast and z-fast tries are not mentioned, and the logarithmic dependence on the key length is presented as new." | Section 6 of Wormhole: "the first ordered key-value index achieving the O(log L) lookup cost". |

No edit is needed at lines 538-541, 543-544, 547-548, 2383-2384.

## Corrections to `biblio.bib`

1. **Add** the journal version of Waldvogel et al. (needed by edits 2 and 6);
   data from the first page of the PDF, DOI from `TO-DOWNLOAD.md` (not printed
   in the PDF):

   ```bibtex
   @article{Waldvogel2001Scalable,
     author  = {Marcel Waldvogel and George Varghese and Jon Turner and Bernhard Plattner},
     title   = {Scalable High-Speed Prefix Matching},
     journal = {ACM Transactions on Computer Systems},
     volume  = {19},
     number  = {4},
     pages   = {440--482},
     year    = {2001},
     doi     = {10.1145/502912.502914}
   }
   ```

2. `Fischer2015Alphabet`: add `volume = {9133}`. Everything else agrees with
   the first page.
3. `WNJWormhole`: the PDF prints "16 pages" and no page range or article
   number; `pages = {1--16}` is a convention, not printed data. `address` holds
   the conference location (the publisher's address printed is New York, NY,
   USA). No error of substance.
4. `FeGSBT`: correct. Optionally add `month = mar` and
   `doi = {10.1145/301970.301973}` (DOI not printed in the PDF).
5. `Bender2006Cacheoblivious`: authors, title, pages, year agree with the PDF;
   the ordinal "25th" and the DOI are not printed and were not checked.
6. `Waldvogel1997Scalable`: cannot be checked (the SIGCOMM '97 PDF was not
   downloaded). Note that the authors' own journal paper cites it with the
   title "Scalable high speed IP routing table lookups", whereas the entry and
   Wormhole's reference list have "Scalable High Speed IP Routing Lookups".
