# Verification of the theory citations against the downloaded PDFs

Scope: four papers of section 1 of `TO-DOWNLOAD.md`, checked against the PDFs
in `/Users/vigna/git/zfast/related/`. Every PDF was read in full (all pages).
Line numbers refer to `paper/dynzfast.tex` as of 2026-11-20. Nothing but this
file was written.

Conventions: "PDF" introduces what is printed in the paper; **[I]** marks an
inference of mine that is not printed there. Proposed LaTeX keeps the
conventions of the file (ASCII, `\lg`, `P\v{a}tra\c{s}cu` as currently
spelled; see the note on the diacritic in the last section).

Summary of verdicts:

| Lines | Sentence about | Verdict |
|---|---|---|
| 413-415 | exponential search trees, sqrt(lg n / lg lg n) "optimal as a function of n alone" | correct; attribution of the upper bound imprecise |
| 245-251 | static problem solved; O(lg l) optimal in linear space even with randomization | correct (wording ambiguous) |
| 417-420 | Patrascu-Thorup settle the static problem; polynomial universes | correct; the promised statement on randomization is missing |
| 447-450 | Mortensen-Pagh-Patrascu; "not optimally" | correct; omits linear space and "with high probability" |
| 451-454 | Delta-sensitive bounds, Delta-fast tries | imprecise (definition of Delta; worst case vs expected; updates) |

---

## 1. Ehrhardt, Mulzer, *Delta-fast tries* (`DeltaFastTries.pdf`)

**Which version.** The published WADS 2017 paper, not the arXiv version: 12
pages numbered 361-372, footer of p. 361 "(c) Springer International
Publishing AG 2017 / F. Ellen et al. (Eds.): WADS 2017, LNCS 10389, pp.
361-372, 2017. / DOI: 10.1007/978-3-319-62127-2_31".

### 1.1 Sentences in `dynzfast.tex`

Only one sentence cites it (key `Ehrhardt2017Delta`), lines 451-454:

> Finally, a few structures provide bounds for predecessor search that depend
> on the distance $\Delta$ between the query and the answer, rather than on
> the length of the keys: Bose \textit{et al.}~\cite{Bose2013Fast} attain
> $O(\lg\lg\Delta)$ expected time, but not linear space; the same query time
> is obtained in the static case by applying fat binary search to a suitably
> restricted interval~\cite{Belazzougui2012Predecessor}, and in the dynamic
> case, in linear space, by the $\Delta$-fast tries of Ehrhardt and
> Mulzer~\cite{Ehrhardt2017Delta}, which the
> survey~\cite{Navarro2020Predecessor} describes as a variant of the z-fast
> trie.

### 1.2 What the paper states

- **Abstract (p. 361).** "Let w in N and U = {0, 1, ..., 2^w - 1} be a bounded
  universe of w-bit integers. We present a dynamic data structure for
  predecessor searching in U. Our structure needs O(log log Delta) time for
  queries and O(log log Delta) expected time for updates, where Delta is the
  difference between the query element and its nearest neighbor in the
  structure. Our data structure requires linear space. This improves a result
  by Bose et al. [CGTA, 46(2), pp. 181-189]."
- **Theorem 3.2 (static, pp. 366-367).** "The static Delta-fast trie for S
  requires O(n) words of space, and it can answer a static predecessor query
  for an element q in U on S in time O(log log min{|q - q^-|, |q - q^+|}),
  where q^- and q^+ denote the predecessor and successor of q in S. The
  preprocessing time is O(n log log log |U|), assuming that S is sorted."
- **Theorem 4.3 (dynamic, p. 370), the main result.** "Let r in U be picked
  uniformly at random. After performing a shift of U by r, the Delta-fast
  trie provides a data structure for the dynamic predecessor problem such
  that the query operations take O(log log Delta) worst-case time and the
  update operations need O(log log Delta) expected time, for Delta = min{|q -
  q^+|, |q - q^-|}, where q is the requested element and q^+ and q^- are the
  predecessor and successor of q in the current set S. At any point in time,
  the data structure needs O(n) words of space, where n = |S|."
  - Queries: worst case. Updates: expected, "where the expectation is over
    the random choice of the shift r" (Lemma 4.2, for a fixed q); no
    amortization is mentioned. The random shift is "a trick of Bose et al."
    (Section 4.1, Lemma 4.1 = Lemma 4 of Bose et al.).
  - The dictionary is that of Demaine, Meyer auf der Heide, Pagh and
    Patrascu (LATIN 2006), Theorem 2.1 (p. 364): "The bounds for the space
    and the queries are worst-case, the bounds for the updates hold with high
    probability."
  - Delta is the distance from the query to its *nearest neighbor*, i.e. the
    minimum of the distances to the predecessor and to the successor, whatever
    the query (the two searches are interleaved, proof of Theorem 3.2).
- **Bose et al. as described here (pp. 361-362).** With "Delta = min{|q -
  q^-|, |q - q^+|} ... the distance between q and its nearest neighbor in S.
  Then, the structure by Bose et al. can answer predecessor and successor
  queries in O(log log Delta) time. Their solution requires O(n log log log
  |U|) words of space". Linear space "solves an open problem from [3]".
- **Relation with the z-fast trie.** The paper never calls its structure a
  variant of the z-fast trie. It says:
  - Section 2, "Associated Keys" (p. 363): "Our algorithm uses the notion of
    associated keys. This notion was introduced in the context of z-fast tries
    [1, 16], and it is also useful in our data structure." The associated key
    of an edge e = (u, v) is "the first time that the binary search encounters
    a prefix that lies on e"; it is computed from the longest common prefix
    l' of the binary expansions of |p_u| and |p_v|, "followed by 1 and enough
    0's" (p. 364), "see [1, 8, 16] for more details". **[I]** This is the
    2-fattest number of (|p_u| .. |p_v|], i.e. the handle.
  - Section 3.1 (p. 364): "Next, we describe the hash map H_z. It is defined
    similarly as the hash map used for z-fast tries [1, 16]. For each edge e
    of T, let alpha_e be the associated key of e ... Then, H_z stores the
    entry alpha_e -> e." **[I]** H_z is the z-map.
  - What is new with respect to the z-fast trie: a second hash map H_Delta
    with the prefixes of the keys at depths w - 2^{2^i} (a doubly exponential
    search from the bottom, then the binary search of the z-fast trie on the
    remaining O(log^2 Delta) bits), and a third hash map H_b for a level of
    indirection ("a trick due to Patrascu [15]", a blog post) that brings the
    space of H_Delta from O(n log log w) words down to O(n).
- **Which z-fast papers are cited.** [1] = Belazzougui, Boldi, Vigna, "Dynamic
  z-fast tries", SPIRE 2010, pp. 159-172. [16] = Ruzic, "Making deterministic
  signatures quickly", ACM TALG 5(3), 2009. [2] = Belazzougui, Boldi, Vigna,
  "Predecessor search with distance-sensitive query time", arXiv:1209.5441.
  The ESA 2010 and SODA 2009 papers (`Belazzougui2010Fast`, `BBPMMPH`) are not
  cited.
- **Ruzic and SPIRE 2010.** They are never described separately: they appear
  only together, as the joint source "[1, 16]" of "z-fast tries" and of
  associated keys (three occurrences, pp. 363-364). The paper does not say
  that the two works are independent (that statement is in Ehrhardt's thesis
  and in the survey, not here).
- **arXiv:1209.5441 (p. 362).** "Belazzougui et al. give a linear space bound
  for distance-sensitive queries in the static setting, using almost the same
  techniques as in the present paper [2]. Our result was obtained
  independently from the work of Belazzougui et al." Acknowledgments (p. 371):
  "we would like to thank the anonymous reviewers for pointing us to [2]."

### 1.3 Verdict on lines 451-454: imprecise

1. "the distance $\Delta$ between the query and the answer": for
   Ehrhardt-Mulzer (and for Bose et al. as they describe them) Delta is the
   distance to the nearest neighbor, i.e. the smaller of the distances to
   predecessor and successor. The sentence is not false (the stated bound is
   weaker than the theorem) but it is not the definition.
2. "the same query time ... in the dynamic case": the query time of
   Ehrhardt-Mulzer is *worst case*, not expected; only updates are expected.
   Update time is not mentioned in the sentence.
3. "Bose et al. attain O(lg lg Delta) expected time, but not linear space":
   the space claim is confirmed by this PDF (O(n log log log |U|) words).
   "Expected" is **unverifiable from this PDF**: Ehrhardt-Mulzer write plain
   "O(log log Delta) time" for the queries of Bose et al.; `theory.md` took
   "expected" from the abstract of the CCCG version of Bose et al., and
   reports there Delta as the distance to the successor. The paper of Bose et
   al. is needed to settle both points.
4. "which the survey describes as a variant of the z-fast trie":
   **unverifiable from this PDF** (it is a statement about the survey). It is
   consistent with the paper, which builds on a z-map; the direct statement
   would be stronger than the reported one.

Proposed replacement for lines 451-454:

```latex
Finally, a few structures provide bounds for predecessor search that depend on the distance $\Delta$ between the query and its nearest neighbor in the set
(i.e., the closer of its predecessor and its successor), rather than on the length of the keys: Bose \textit{et al.}~\cite{Bose2013Fast} attain $O(\lg\lg\Delta)$ expected time, but not linear space; 
the same query time is obtained in the static case by applying fat binary search to a suitably restricted interval~\cite{Belazzougui2012Predecessor},
and in the dynamic case by the $\Delta$-fast tries of Ehrhardt and Mulzer~\cite{Ehrhardt2017Delta}, which use linear space, answer queries in time $O(\lg\lg\Delta)$ in the worst case 
and perform updates in expected time $O(\lg\lg\Delta)$. The $\Delta$-fast trie is built on a compacted trie on integers of $w$ bits with a z-map (the hash map of ``associated keys'' of~\cite{Ehrhardt2017Delta}), 
and adds hash tables that make it possible to start the fat binary search close to the bottom of the trie; indeed, the survey~\cite{Navarro2020Predecessor} describes it as a variant of the z-fast trie. 
```

(Leave "expected" for Bose et al. as it is until their paper is checked.)

### 1.4 BibTeX entry `Ehrhardt2017Delta`

Authors, title, pages (361-372), year, DOI match p. 361. Missing: `volume =
{10389}` (printed: "LNCS 10389"). The editors are printed only as "F. Ellen et
al. (Eds.)". "15th International Symposium" is not printed in the PDF
(unverifiable here).

### 1.5 Relevant and not yet in the Related work section

- **An alternative to jump pointers (Section 4.2, pp. 368-369).** The dynamic
  structure must "maintain for each node v in T the leftmost and the
  rightmost element in the subtree T_v", exactly the role of jump pointers.
  It uses a path decomposition: each leaf is the leftmost (rightmost) leaf of
  the subtrees of the nodes of "a subpath (more precisely, a prefix) of the
  path from v to the root"; each subpath is kept in a list of balanced trees,
  one for each range of heights [2^i, 2^{i+1}), with `split` and `join` in
  O(log h) time, h being the height (number of remaining bits) of the node.
  "This decomposition of T into dynamically changing subpaths is similar to
  the preferred paths decomposition of Tango trees [6]." The bound is in terms
  of the height in a trie of fixed depth w, so it does not give bounds in
  terms of string lengths **[I]**. `dynzfast.tex` does not mention it (no
  occurrence of "tango" or "path decomposition"). `theory.md` recommended it
  from the thesis; it is in the WADS paper too. Possible sentence, after the
  one above:

  ```latex
  To reach the leftmost and rightmost leaf under a node, instead of jump pointers they use a decomposition of the trie into paths, each represented by a sequence of balanced trees.
  ```
- The static Delta-fast trie (Theorem 3.2) is a second static, linear-space
  solution, independent of `Belazzougui2012Predecessor` by the authors' own
  statement.
- The paper attributes "z-fast tries" jointly to SPIRE 2010 and to Ruzic,
  which supports the placement of Ruzic's structure in the paragraph "Binary
  search on prefix lengths" (lines 471-480). It does *not* support the word
  "independently" at line 477, which rests on the survey only.

---

## 2. Mortensen, Pagh, Patrascu, *On dynamic range reporting in one dimension* (`DynamicRangeReporting.pdf`)

**Which version.** The STOC 2005 extended abstract: 8 pages numbered 104-111,
"STOC'05, May 22-24, 2005, Baltimore, Maryland, USA. Copyright 2005 ACM
1-58113-960-8/05/0005". Title page: "On Dynamic Range Reporting in One
Dimension (Extended Abstract)"; footnote: "A version of this paper containing
all proofs is available as arXiv:cs.DS/0502032." The tradeoffs and the
greater-than lower bounds are only stated here ("All tradeoffs, as well as the
lower bounds for the greater-than problem, appear in the full version of this
paper", Section 1.3), and "This extended abstract only discusses insertions;
deletions follow parallel steps uneventfully" (Section 4.3).

### 2.1 Sentences in `dynzfast.tex`

Lines 447-450 (key `Mortensen2005Dynamic`):

> Other queries on integers are easier than predecessor search: for instance,
> Mortensen, Pagh and P\v{a}tra\c{s}cu~\cite{Mortensen2005Dynamic} show that
> one can report an element of a query interval (i.e., answer existential
> range queries, and thus existential prefix queries) in time $O(\lg\lg w)$
> with updates in time $O(\lg w)$. The z-fast trie answers such queries within
> the bounds of predecessor search, and thus not optimally.

### 2.2 What the paper states

- **The query (p. 104).** "The query FINDANY(a, b) should return an arbitrary
  value in S intersected with [a, b], or report that [the intersection is
  empty]. This is a form of existential range query. In fact, since we only
  consider update times above the predecessor bound, updates can maintain a
  linked list of the values in S in increasing order. Given a value x in S
  [in the interval], one can traverse this list in both directions starting
  from x and list all values in the interval [a, b] in constant time per
  value. Thus, the FINDANY query is equivalent to one-dimensional range
  reporting." So: one witness (hence emptiness), and all k elements in
  additional O(k) time.
- **Model and kind of bound (p. 104).** Word RAM, elements fit in a word, w
  bits per word, universe size u = 2^w. "Our data structure uses Las Vegas
  randomization (through hashing), and the bounds stated will hold with high
  probability in n." Neither worst-case deterministic nor merely expected.
- **Main bound (abstract; Section 1, p. 105).** "For the range reporting
  problem, we show how to perform updates in O(lg w) time, while supporting
  queries in O(lg lg w) time. The space usage is optimal, i.e. O(n) words. The
  update time is identical to the one given by the van Emde Boas structure,
  but the query time is exponentially faster."
- **Theorem 4 (p. 106), the general statement.** With "t_pred ... the time
  needed by one update and one query in the dynamic predecessor problem":
  "There is a data structure for the dynamic range reporting problem, which
  uses O(n) space and supports updates in time O(t_u), and queries in time
  O(t_q), for all t_u, t_q satisfying: [t_q >= lg lg w, lg w / lg lg w <= t_u
  <= lg w : t_u = O(lg_{t_q} w) + t_pred] [t_q <= lg lg w, t_u >= lg w :
  2^{t_q} = O(lg_{t_u} w)]". The bound of the abstract is "the cross-over of
  the two curves". Also: constant query time with O(w^epsilon) updates; in
  terms of n, "t_q = O(lg lg n) and t_u = O(lg n / lg lg n)".
- **Optimality is conjectured, not proved.** Abstract: "We present some
  arguments supporting the conjecture that our solution is optimal." Section
  1.3: "we view the lower bounds for the greater-than function as giving an
  indication that our range reporting data structure is likewise optimal."
  The proved optimality results concern dynamic perfect hashing and Bloomier
  filters (Theorems 1 and 2) and the greater-than problem (Theorem 3).
- **Relation with predecessor search.**
  - p. 104: "In one dimension, the problem is easily solved using predecessor
    search." "Our problem is more general than the lookup problem, and less
    general than the predecessor problem."
  - Abstract: "Existing lower bounds show that achieving our query time for
    predecessor search requires doubly-exponentially slower updates." p. 105:
    "Beame and Fich [2, Theorem 3.7] show that achieving any query time that
    is o(lg w / lg lg w) for the predecessor problem requires update time
    Omega(2^{w^{1-epsilon}})". (In the JCSS text of Beame and Fich the dynamic
    statements are Theorems 3.13-3.15; Theorem 3.7 is the static one.)
  - p. 105: "In the dynamic case, however, no solution better than the
    predecessor problem was known. For this problem, the fastest known
    solution in terms of w is the classic van Emde Boas structure [13], which
    achieves O(lg w) time per operation."
  - The structure *contains* a dynamic predecessor structure (Section 4.2,
    items 1 and 2: "a linked list with all elements of S in increasing order,
    and a predecessor structure for S"; "for van Emde Boas, this can be
    achieved through hashing [14]", i.e. Willard), and the update time is
    "T_pred + O(lg w) = O(lg w)" (Section 4.3).
  - Static case (p. 104): Miltersen et al. gave O(nw) space and constant
    query time, and "Alstrup, Brodal and Rauhe [1] ... gave an optimal
    solution for the static case, achieving linear space and constant query
    time".
- **Relation with van Emde Boas.** Abstract: "Whereas van Emde Boas uses a
  simple recursion (repeated halving) on each path in a trie, we use a
  nontrivial, van Emde Boas-like recursion on every such path." Section 1.3:
  "The van Emde Boas structure examines each level of detail consecutively,
  halving w in each step. On the other hand, our query algorithm does a binary
  search on these representations of the trie, examining just a logarithmic
  number of them." (lg w tries T_t over alphabets of 2^t bits; the query
  bisects on t, Section 4.4.)

### 2.3 Verdict on lines 447-450: correct, with two omissions

- The bounds O(lg lg w) and O(lg w) and the kind of query are as stated.
- Omitted: the space is linear (O(n) words), and the bounds hold with high
  probability (Las Vegas randomization). Both matter in a section that
  distinguishes carefully worst-case, expected and high-probability bounds.
- "easier than predecessor search" is supported by the quotes above (the
  comparison is on the query time for a given update time).
- "(... and thus existential prefix queries)" is not in the PDF; it is a
  correct inference for integers of fixed length **[I]**.
- "and thus not optimally" is justified in the sense that the query time
  O(lg lg w) is attained with the same update time, O(lg w), and in linear
  space. It must not be read as "the bound of Mortensen et al. is optimal":
  that is only conjectured. The current wording is compatible with this.

Proposed replacement for lines 447-450:

```latex
Other queries on integers are easier than predecessor search: for instance, Mortensen, Pagh and P\v{a}tra\c{s}cu~\cite{Mortensen2005Dynamic} 
show that in linear space one can report an element of a query interval (i.e., answer existential range queries, and thus existential prefix
queries) in time $O(\lg\lg w)$ with updates in time $O(\lg w)$, with high probability. The z-fast trie answers 
such queries within the bounds of predecessor search, and thus not optimally.
```

### 2.4 BibTeX entry `Mortensen2005Dynamic`

Authors, title, venue (STOC 2005), pages (104-111: first page printed, eight
pages), publisher and the arXiv identifier match. The DOI is not printed in
the PDF (unverifiable here). No correction. The PDF carries the subtitle
"(Extended Abstract)", which need not be in the entry.

### 2.5 Relevant and not yet in the Related work section

- Since the structure keeps the keys in a sorted linked list, enumerative
  range queries cost O(lg lg w + k), to be compared with the O(lg max{...} +
  k) of the abstract of `dynzfast.tex`.
- The static problem is solved in linear space and constant time by Alstrup,
  Brodal and Rauhe (STOC 2001), as stated both here (p. 104) and by Beame and
  Fich (p. 43). It is not cited in `dynzfast.tex`; the entry
  `Alstrup2001Optimal` exists in `related-work/theory.bib` but not in
  `biblio.bib`. I have not read that paper.
- The structure is for integers of one word; nothing is said about strings.

---

## 3. Patrascu, Thorup, *Randomization does not help searching predecessors* (`RandomizationPredecessors.pdf`)

**Which version.** Ten pages numbered 555-564, which is the pagination of the
SODA 2007 proceedings; no venue, year, copyright line or DOI is printed
anywhere in the PDF (PDF produced 2 October 2006). Section 2.5: "This paper
needs to be understood as an extended abstract. Essentially, it contains only
a description of the new ideas needed for the proof."

### 3.1 Sentences in `dynzfast.tex`

Key `PaTRDNHSP`, always together with `Patrascu2006Timespace`.

Lines 244-251:

> For strings that fit in a machine word, the static version of this problem
> has been completely solved, with matching upper and lower
> bounds~\cite{Patrascu2006Timespace,PaTRDNHSP}, and so has the dynamic
> version, in expectation~\cite{Patrascu2014Dynamic}. If only linear space is
> available, in a significant number of cases, which include polynomial
> universes, variants of the van Emde Boas tree~\cite{EKZDIEPQ} that answer in
> time $O(\lg\ell)$ are optimal even in the static setting using randomization
> (see Section~\ref{sec:related} for a precise statement).

Lines 417-420:

> P\v{a}tra\c{s}cu and Thorup settled the static
> problem~\cite{Patrascu2006Timespace,PaTRDNHSP}, providing matching upper and
> lower bounds for every combination of $n$, $\ell$, $w$ and of the available
> space. In particular, they note that when $w=\ell=\gamma\lg n$ for some
> $\gamma>1$ (i.e., for polynomial universes) and space is near linear the
> optimal search time is $\Theta(\lg \ell)$, so van Emde Boas trees and their
> variants are optimal.

### 3.2 What the paper states

There is no numbered theorem about predecessor search (the only numbered
statements are Lemmas 3.1 and 4.1-4.6 on cell-probe elimination). The result
is stated in prose:

- **Setting (pp. 555-556).** Static problem ("We focus on the static case,
  where Y is given in advance for preprocessing"): n integers of l bits, word
  (cell) length b, "lg n <= l <= b"; "The case b > l models the external
  memory model with B = floor(b/l) keys per page"; "Unless otherwise stated,
  we assume below b = l."
- **The bound (1.1), p. 556.** "Assuming S bits of space are available, and
  defining a = lg(S/n), we showed that the optimal search time is, up to
  constant factors:"

      min { log_b n,
            lg((l - lg n) / a),
            lg(l/a) / lg( (a / lg n) * lg(l/a) ),
            lg(l/a) / lg( lg(l/a) / lg(lg n / a) ) }

  (space in *bits* here; `theory.md` reports the STOC 2006 formulation with S
  words and a = lg(S/n) + lg w, which is the same quantity).
- **Main result (Section 2, p. 556).** "The main result of this paper is that
  the deterministic predecessor bound from (1.1) cannot be improved even if we
  allow randomization. As mentioned above, the unresolved case was with small
  space n^{1+o(1)}, which is the most interesting case in practice." Abstract:
  "Our trade-off lower bounds are tight for any combination of parameters."
- **Error probability (Section 2.1).** "In this paper, 'randomized algorithm'
  refers to an algorithm with a fixed running time, which may make a
  two-sided error with probability bounded away from 1/2. This model is
  stronger that one-sided or zero-error (Las Vegas) randomization, so lower
  bounds also apply in these settings." Abstract: "lower bounds for randomized
  query algorithms with constant error probability". (In the proof sketch the
  initial error is assumed to be at most 1/3, Section 3.2.)
- **Range of space.** Every space bound. For "S >= n^{1+epsilon} for some
  constant epsilon > 0" the randomized lower bound was already in the STOC
  2006 paper ("via the stronger communication game model, and allowed
  randomization with two-sided error"); what is new here is space n^{1+o(1)},
  where the STOC 2006 bounds "only held for the deterministic case".
- **Model.** Lower bounds: cell-probe model with cells of b bits, for the
  *colored* predecessor problem (a decision problem), hence also for the
  problems it reduces to. Upper bounds: "achieved by a deterministic query
  algorithm on a RAM. The data structure can be constructed in expected time
  O(S) by a randomized algorithm, starting from a sorted list of integers."
- **Polynomial universes (p. 556).** "The most important consequence is that
  for a polynomial universe with b = l = Theta(lg n) and near-linear space S
  = O~(n), the classic van Emde Boas bound of O(lg l) = O(lg lg n) is
  optimal." (Footnote: O~(n) = n lg^{O(1)} n.) Abstract: "for near linear
  space, the classic van Emde Boas search time of O(lg l) cannot be improved,
  even if we allow randomization. This is a separation from polynomial space,
  since Beame and Fich [STOC'02] give a predecessor search time of O(lg l / lg
  lg l) using quadratic space."
- **Dynamic structures.** Not covered: the word "dynamic" does not occur in
  the paper (checked on the extracted text), nor do updates. **[I]** The bounds apply to the query time of any dynamic
  structure as a function of the space it uses, since such a structure is in
  particular a static one.

### 3.3 Verdicts

**Lines 244-251: correct.** "Completely solved, with matching upper and lower
bounds" is the paper's own claim ("tight for any combination of parameters").
The second sentence matches the abstract and p. 556. Two remarks:

- "optimal even in the static setting using randomization" can be read as
  "optimal by means of randomization"; what is meant (and true) is "even if
  randomization is allowed".
- The condition for polynomial universes is l = gamma lg n with gamma > 1 (so
  that l - lg n = Theta(l)): this is at line 419, fine.

Proposed replacement for lines 250-251 (wording only):

```latex
variants of the van Emde Boas tree~\cite{EKZDIEPQ} that answer in time $O(\lg\ell)$ are
optimal, even in the static setting and even if randomization is allowed (see Section~\ref{sec:related} for a precise statement). 
```

**Lines 417-420: correct, but incomplete with respect to line 251.** The
introduction promises "a precise statement" about randomization, and the
static statement of Section `sec:related` does not mention it (line 430 does,
for the dynamic bound only). The form "w = l = gamma lg n, gamma > 1" is that
of the STOC 2006 paper (as quoted in `theory.md`); the SODA paper writes "b =
l = Theta(lg n)". Proposed replacement for lines 417-418:

```latex
P\v{a}tra\c{s}cu and Thorup settled the static problem~\cite{Patrascu2006Timespace,PaTRDNHSP}, providing 
matching upper and lower bounds for every combination of $n$, $\ell$, $w$ and of the available space: the upper bounds are attained 
by deterministic query algorithms, and the lower bounds hold in the cell-probe model, even for randomized query algorithms 
with constant two-sided error probability~\cite{PaTRDNHSP}. 
```

### 3.4 BibTeX entry `PaTRDNHSP`

Verifiable from the PDF: authors, title, pages 555-564. Venue, year, location
and publisher are not printed (unverifiable here; the reference lists of the
other PDFs do not include this paper). No DOI is printed and the entry has
none. Two remarks:

- The entry spells the first author `P\v{a}tra\c{s}cu` (caron), while the
  other three entries of the same author (`Mortensen2005Dynamic`,
  `Patrascu2006Timespace`, `Patrascu2014Dynamic`) have
  `P{\u{a}}tra{\c{s}}cu` (breve). The first page of this PDF prints a breve
  (checked on a 500 dpi rendering). With two spellings BibTeX treats them as
  different names when sorting and building labels.
- The style differs from that of the other entries (`booktitle = {SODA '07:
  Proceedings of the eighteenth annual ACM-SIAM symposium on Discrete
  algorithms}`, `location`, `address`); see the last section for a
  harmonized entry.

### 3.5 Relevant and not yet in the Related work section

- **[I]** For polynomial universes the static bound gives more than line
  434-435 states: Omega(lg l) is a lower bound on the *query* time of every
  structure using near-linear space, randomized or not and whatever its
  update time, so the query time of the z-fast trie is optimal there by
  itself, not only "for the maximum between query and update time".
- "The case b > l models the external memory model": the STOC 2006 conclusion
  on external memory quoted in `theory.md` (B-trees or a RAM algorithm) holds
  for randomized query algorithms too. Relevant to the paragraph "Strings in
  external memory" only for keys of one word.
- p. 555: "integer predecessor search is asymptotically equivalent to the IP
  look-up problem for forwarding packets on the Internet [6]", a link with the
  paragraph on Waldvogel et al.

---

## 4. Beame, Fich, *Optimal bounds for the predecessor problem and related problems* (`OptimalBoundsPredecessor.pdf`)

**Which version.** The journal paper: "Journal of Computer and System
Sciences 65, 38-72 (2002), doi:10.1006/jcss.2002.1822", 35 pages, "Received
January 31, 2000; revised August 23, 2001".

### 4.1 Sentences in `dynzfast.tex`

One sentence (key `Beame2002Optimal`), lines 412-415:

> The other approach reduces the \emph{number} of keys: fusion
> trees~\cite{FrWSITBFT} answer queries in time $O(\lg n/\lg w)$, and the
> exponential search trees of Andersson and Thorup~\cite{AnTDOSEST} provide
> deterministic, worst-case bounds in linear space for queries and updates,
> such as $O\bigl(\sqrt{\lg n/\lg\lg n}\bigr)$, which is optimal as a function
> of $n$ alone~\cite{Beame2002Optimal}.

### 4.2 What the paper states

Notation: n elements, universe of size N (so l = log N), b bits per word.

- **Upper bound, static: Theorem 4.5 (p. 67).** "There is a static data
  structure for representing n integers from the universe [1, N] that uses
  O(n^2 log n / log log n) words of log N bits and answers predecessor queries
  in time O(min{log log N / log log log N, sqrt(log n / log log n)}).
  Moreover, this data structure can be constructed in O(n^{2+epsilon}) time
  for any epsilon > 0." Unit-cost word RAM with multiplication (abstract);
  the construction is deterministic (it uses the deterministic hash functions
  of Raman, p. 67).
- **Upper bound, dynamic: Corollary 4.6 (p. 69).** "Exponential search trees
  [7, 10] can be combined with our static data structure to give a linear
  size dynamic data structure. COROLLARY 4.6. The dynamic predecessor problem
  for a set of up to n integers from the universe [1, N] can be solved on a
  RAM with O(n) words of log N bits, in time O(min{(log log n) log log N / log
  log log N, sqrt(log n / log log n)})." The corollary does not say whether
  the bound is worst case or amortized; Section 2 (pp. 42-43) says that
  Andersson's exponential search trees [7] give "worst case search time and
  amortized update time" and that "Andersson and Thorup [10] have combined a
  variant of exponential search trees with eager partial rebuilding to
  improve the resulting dynamic data structure, achieving worst-case instead
  of amortized bounds for update time". ([7] = Andersson, FOCS 1996; [10] =
  Andersson and Thorup, STOC 2000.)
- **Before this paper** (p. 42) exponential search trees combined "with fusion
  trees, packed B-trees, and x-fast tries" gave O(min{sqrt(log n), log log N *
  log log n, log n / log b + log log n}): the bound sqrt(log n / log log n)
  needs the static structure of Beame and Fich.
- **Lower bounds, static (p. 57).** Proved for a class of "prefix problems"
  in the communication game model, hence in the cell-probe model.
  - Corollary 3.9: "Consider any cell probe data structure for the static
    predecessor problem that stores each set S from a universe of size N using
    |S|^{O(1)} memory cells of 2^{(log N)^{1-Omega(1)}} bits. Then, in the
    worst case, queries take Omega(log log N / log log log N) time."
  - Corollary 3.10: "Consider any cell probe data structure for the static
    predecessor problem. If each set of size n from a universe of size N is
    stored using n^{O(1)} memory cells of (log N)^{O(1)} bits. Then, in the
    worst case, queries take Omega(sqrt(log n / log log n)) time."
  - Both are existential in the other parameter: Theorem 3.7 has "there
    exists a function n(N) <= N", Theorem 3.8 "there is a function N(n)". In
    the proof of Theorem 3.8 the hard universe has N = n^{(ckt)^t} with t =
    Theta(sqrt(log n / log log n)).
  - The same bounds hold for predecessor parity, point separation, rank and
    exact range counting.
- **Lower bounds, dynamic (Section 3.5, pp. 57-60),** by a translation
  argument of Miltersen.
  - Theorem 3.12: "Consider any cell-probe data structure for the semidynamic
    predecessor problem (i.e., no delete operations are performed) on [1, N]
    restricted to sets of size at most n. Suppose there are b = (log N)^{O(1)}
    bits per memory cell and the data structure uses 2^{O(b)} memory cells. If
    the amortized time for inserts is n^{O(1)}, then the worst-case query time
    is not o(sqrt(log n / log log n))."
  - Theorem 3.14: same setting with "b = 2^{(log N)^{1-Omega(1)}} bits per
    memory cell" and "amortized time for inserts ... 2^{(log
    N)^{1-Omega(1)}}": "then the worst-case query time is Omega(log log N /
    log log log N)."
  - Theorems 3.15 and 3.16: the same two bounds without any restriction on
    the number of cells, given *worst-case* update time 2^{(log
    N)^{1-Omega(1)}} and 2^{O(sqrt(log n))} respectively (stated for the
    prefix problem; "There are analogous lower bounds for the semidynamic
    predecessor problem").
- **What the authors say on the dynamic case.** Abstract: "Our algorithms are
  for the unit-cost word RAM with multiplication and are extended to give
  dynamic algorithms. The lower bounds are proved for a large class of
  problems, including both static and dynamic predecessor problems". Section 1
  (p. 39): "Using recent generic transformations of Andersson and Thorup [7,
  10], the algorithm can be made dynamic and the space improved to O(n),
  although the time increases to O(min{(log log N / log log log N) * log log
  n, sqrt(log n / log log n)})." Conclusions (p. 69): "It would be nice to
  remove the log log n factor in the numerator in the first term of the
  minimum in Corollary 4.6 so that our upper and lower bounds would match for
  the dynamic versions of these problems." That is, in the dynamic case the
  bounds match in the term depending on n and do not match in the term
  depending on N.
- **Randomization.** The paper does not discuss randomized lower bounds; a
  cell-probe algorithm is "a decision tree" (p. 43). Patrascu and Thorup (SODA
  2007, p. 556) state that the round elimination lemma of Sen and Venkatesh
  "can reprove the lower bounds of Beame and Fich, but also for randomized
  algorithms".

### 4.3 Verdict on lines 412-415: correct; the attribution is imprecise

- **"optimal as a function of n alone" is confirmed.** Upper bound: Corollary
  4.6, linear space. Lower bound: Corollary 3.10 (static, polynomial space)
  and Theorem 3.12 (insertions only, any reasonable space, polynomial
  amortized insertion time). It is exactly the qualification suggested by the
  authors' conclusions. Precisely, what is proved is: for words of (log
  N)^{O(1)} = l^{O(1)} bits there are universe sizes for which no
  (deterministic) structure with update time n^{O(1)} has worst-case query
  time o(sqrt(lg n / lg lg n)). The bound is on the query time, and hence on
  the maximum between query and update time **[I]**.
- **Attribution.** The sentence credits the bound to exponential search trees
  and cites Beame and Fich for optimality only. By Corollary 4.6 and p. 42,
  the term sqrt(lg n / lg lg n) is obtained by plugging the static structure
  of Beame and Fich into exponential search trees; exponential search trees
  alone gave sqrt(lg n).
- **"deterministic, worst-case bounds ... for queries and updates"** is a
  statement about `AnTDOSEST` and is **unverifiable from this PDF** in its
  final form; it is consistent with p. 43 (worst-case updates in the STOC
  2000 version) and with `theory.md`, which read the arXiv version of the
  JACM paper.
- "fusion trees answer queries in time O(lg n / lg w)" agrees with Patrascu
  and Thorup, SODA 2007, p. 555 ("fusion trees, which requires linear space
  and can answer queries in O(log_l n) time", with b = l); Beame and Fich
  report the original O(log n / log log n) and Hagerup's O(1 + log n / log b)
  (pp. 41, 68).

Proposed replacement for lines 413-415 (minimal):

```latex
trees~\cite{FrWSITBFT} answer queries in time $O(\lg n/\lg w)$, and the exponential search trees of 
Andersson and Thorup~\cite{AnTDOSEST} provide deterministic, worst-case bounds in linear space for
queries and updates, such as $O\bigl(\sqrt{\lg n/\lg\lg n}\bigr)$ (using the static structure of Beame and Fich~\cite{Beame2002Optimal}, 
who also show that this bound is optimal as a function of $n$ alone).
```

### 4.4 BibTeX entry `Beame2002Optimal`

Authors, title, journal, volume 65, pages 38-72, year 2002 and DOI match the
first page. The issue number is not printed there; the reference lists of the
other two PDFs print "65(1):38-72, 2002". No correction.

### 4.5 Relevant and not yet in the Related work section

- **A lower bound in l alone [I].** By Theorem 3.14, for every word size up
  to 2^{l^{1-epsilon}} bits and update time up to 2^{l^{1-epsilon}} there are
  set sizes for which queries need Omega(lg l / lg lg l) time. So, as a
  function of the length of the keys alone, the O(lg l) bound of the z-fast
  trie is within a factor lg lg l of optimal whatever the space; this is the
  natural companion of "optimal as a function of n alone", and it is the
  relevant measure for a structure whose bounds do not depend on n. (The
  precise picture in linear space is that of `Patrascu2014Dynamic`, lines
  421-436.)
- Theorem 4.5 beats O(lg l) by a factor lg lg l, but in space O(n^2 lg n / lg
  lg n) words; with linear space (Corollary 4.6) the same term is multiplied
  by lg lg n. This is the separation between polynomial and near-linear space
  recalled in the abstract of the SODA 2007 paper.
- On Willard's tries (p. 41): "Both of Willard's data structures perform
  membership and predecessor queries in worst-case O(log log N) time, but use
  randomization (for rehashing) to achieve expected O(log log N) update time."
  Line 408 of `dynzfast.tex` says only "at the price of amortized bounds for
  updates" for y-fast tries; according to this description the update bounds
  are also expected. (A statement about Willard's paper, which is not among
  the PDFs read here.)
- On existential range queries (p. 43): "Another problem that can be solved
  using predecessor queries is one dimensional range search (i.e., find some
  element of S contained within a query interval, if one exists). However,
  Alstrup, Brodal, and Rauhe [3] have recently obtained a static data
  structure of size O(n) ... that supports range search in constant time."

---

## Proposed edits to dynzfast.tex

Ordered by line. E1 is wording only; E2-E5 change content; E6 and E7 are
optional.

**E1. Lines 250-251** (wording).

- Old: `optimal even in the static setting using randomization (see Section~\ref{sec:related} for a precise statement).`
- New: `optimal, even in the static setting and even if randomization is allowed (see Section~\ref{sec:related} for a precise statement).`
- Reason: "using randomization" is ambiguous; the result is a lower bound for
  randomized query algorithms (SODA 2007, abstract and Section 2).

**E2. Lines 413-415** (attribution of the upper bound).

- Old: `queries and updates, such as $O\bigl(\sqrt{\lg n/\lg\lg n}\bigr)$, which is optimal as a function of $n$ alone~\cite{Beame2002Optimal}.`
- New: `queries and updates, such as $O\bigl(\sqrt{\lg n/\lg\lg n}\bigr)$ (using the static structure of Beame and Fich~\cite{Beame2002Optimal}, who also show that this bound is optimal as a function of $n$ alone).`
- Reason: Beame-Fich, Corollary 4.6 and p. 42: the bound is obtained by
  combining exponential search trees with their static structure (Theorem
  4.5). The optimality claim is confirmed (Corollary 3.10, Theorem 3.12, and
  the conclusions).

**E3. Lines 417-418** (the precise statement promised at line 251).

- Old: `matching upper and lower bounds for every combination of $n$, $\ell$, $w$ and of the available space.`
- New: `matching upper and lower bounds for every combination of $n$, $\ell$, $w$ and of the available space: the upper bounds are attained by deterministic query algorithms, and the lower bounds hold in the cell-probe model, even for randomized query algorithms with constant two-sided error probability~\cite{PaTRDNHSP}.`
- Reason: SODA 2007, Sections 1.1, 2 and 2.1. Without it, Section
  `sec:related` says nothing on randomization in the static case.

**E4. Lines 447-449** (space and kind of bound).

- Old: `show that one can report an element of a query interval (i.e., answer existential range queries, and thus existential prefix queries) in time $O(\lg\lg w)$ with updates in time $O(\lg w)$.`
- New: `show that in linear space one can report an element of a query interval (i.e., answer existential range queries, and thus existential prefix queries) in time $O(\lg\lg w)$ with updates in time $O(\lg w)$, with high probability.`
- Reason: STOC 2005, p. 104 ("Las Vegas randomization ... the bounds stated
  will hold with high probability in n") and p. 105 ("O(n) words"). The
  sentence "and thus not optimally" at line 450 stands; note that the
  optimality of the bound of Mortensen et al. is only conjectured.

**E5. Lines 451-454** (definition of Delta; worst case and expectation).

- Old: `depend on the distance $\Delta$ between the query and the answer,` ...
  `and in the dynamic case, in linear space, by the $\Delta$-fast tries of Ehrhardt and Mulzer~\cite{Ehrhardt2017Delta}, which the survey~\cite{Navarro2020Predecessor} describes as a variant of the z-fast trie.`
- New: `depend on the distance $\Delta$ between the query and its nearest neighbor in the set (i.e., the closer of its predecessor and its successor),` ...
  `and in the dynamic case by the $\Delta$-fast tries of Ehrhardt and Mulzer~\cite{Ehrhardt2017Delta}, which use linear space, answer queries in time $O(\lg\lg\Delta)$ in the worst case and perform updates in expected time $O(\lg\lg\Delta)$. The $\Delta$-fast trie is built on a compacted trie on integers of $w$ bits with a z-map (the hash map of ``associated keys'' of~\cite{Ehrhardt2017Delta}), and adds hash tables that make it possible to start the fat binary search close to the bottom of the trie; indeed, the survey~\cite{Navarro2020Predecessor} describes it as a variant of the z-fast trie.`
- Reason: WADS 2017, abstract and Theorem 4.3 (Delta = min of the two
  distances; queries worst case, updates expected); Sections 2 and 3.1 for
  the relation with the z-fast trie. The clause on Bose et al. ("expected
  time, but not linear space") is left unchanged: the space is confirmed
  (O(n lg lg lg |U|) words), "expected" could not be checked.

**E6 (optional). After E5**, or where jump pointers are introduced (near line 1454):

- New: `To reach the leftmost and rightmost leaf under a node, instead of jump pointers they use a decomposition of the trie into paths, each represented by a sequence of balanced trees.`
- Reason: WADS 2017, Section 4.2; the only published alternative to jump
  pointers found so far.

**E7 (optional). Lines 417 and 447**, `P\v{a}tra\c{s}cu` to
`P\u{a}tra\c{s}cu`.

- Reason: the name has a breve, not a caron (the SODA 2007 first page prints
  a breve; three of the four BibTeX entries already use `\u{a}`). The STOC
  2005 PDF itself prints a caron, so the sources are not uniform.

Not edited, but depending on a paper outside this set: "expected" and the
definition of Delta for Bose et al. (line 452); "deterministic, worst-case"
for `AnTDOSEST` (line 414); "amortized" for y-fast tries (line 408, see
Section 4.5); lines 421-439 (`Patrascu2014Dynamic`).

## Corrections to biblio.bib

1. `Ehrhardt2017Delta`: add `volume = {10389}`. Everything else matches the
   first page.
2. `PaTRDNHSP`: spell the author as in the other entries,
   `P{\u{a}}tra{\c{s}}cu, Mihai` (now `P\v{a}tra\c{s}cu`): with two
   spellings BibTeX sees two different authors. Optionally harmonize the
   entry with the neighbouring ones (only authors, title and pages are
   verifiable from the PDF; venue and year are as in the current entry):

   ```bibtex
   @inproceedings{PaTRDNHSP,
     author    = {P{\u{a}}tra{\c{s}}cu, Mihai and Thorup, Mikkel},
     title     = {Randomization does not help searching predecessors},
     booktitle = {Proceedings of the 18th Annual {ACM}-{SIAM} Symposium on Discrete Algorithms (SODA 2007)},
     pages     = {555--564},
     publisher = {SIAM},
     year      = {2007}
   }
   ```

   No DOI is printed in the PDF; none was added.
3. `Mortensen2005Dynamic`: no correction (DOI not printed in the PDF, not
   checked here).
4. `Beame2002Optimal`: no correction.
5. If the static result on range reporting is mentioned (Sections 2.5 and
   4.5), copy `Alstrup2001Optimal` from `related-work/theory.bib`; that paper
   was not read.
