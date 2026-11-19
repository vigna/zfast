# Related work, theory: predecessor search and dynamic dictionaries/tries on integers and strings

Survey written on 2026-11-19 for the related-work section of *Z-Fast Tries*
(`dynzfast.tex`; line numbers refer to the file as of that date, 2287 lines).
BibTeX keys refer to `theory.bib` in this directory.

## 0. How to read this report

Every statement carries one of these markers.

- **[R]** read by me in the primary source; the version read is named (often
  the arXiv version, not the published one).
- **[A]** only the abstract of the primary source was read.
- **[S: X]** not read in the original; taken from secondary source X, which I
  did read. Treat as unverified against the original.
- **[I]** my inference or calculation, not stated in any source.

Notation: n keys; w word size in bits; l key length in bits for integers;
|x| length in bits of a query string; B block size; sigma alphabet size;
alpha = w / log sigma characters per word; "lg" and "log" are base 2.

Bibliographic data were checked against Crossref (publisher metadata), DataCite
(LIPIcs) or arXiv. DBLP could not be used: on the day of the survey its API
answered automated clients with a bot check. Entries for which no registry
record was found are flagged in `theory.bib`.

## 1. Main findings

1. **Patrascu-Thorup 2014** settles dynamic predecessor search for l <= w in
   expectation. Against it, the O(log w) bound of the z-fast trie is optimal
   exactly when log n = w^{Omega(1)} (and the set is not co-tiny), and
   suboptimal for small sets in large words, where the optimum is
   O(log n / log w), down to O(1) for n = w^{O(1)} (Section 2.8).
2. **Ruzic (SODA 2007, TALG 2009) is missing from the paper and is the closest
   prior work.** It gives a *static, deterministic* predecessor structure for
   variable-length binary strings with query time O(|s|/w + log |s|) on the
   word RAM and O(|s|/B + log |s|) I/Os in the cache-oblivious model: the
   bounds of Theorem `th:boundslong`, three years before SPIRE 2010. Both the
   Navarro-Rojas-Ledesma survey and Ehrhardt's thesis describe it as an
   independent discovery of the idea behind the z-fast trie (Sections 4, 6.1).
   The PDFs of both versions are already in the paper directory
   (`a26-ruzic.pdf`, `ruzic.pdf`) but the text does not cite them.
3. **"Optimally" at line 290 is false for existential range and prefix
   queries**: Mortensen, Pagh and Patrascu (STOC 2005) answer dynamic
   one-dimensional range reporting on w-bit integers in O(lg lg w) time with
   O(lg w) updates, exponentially faster than predecessor search (Section 5.2).
4. The Navarro-Rojas-Ledesma survey gives the z-fast trie its own subsection
   and figure, and presents it as the linear-space, non-bucketed successor of
   x-fast and y-fast tries (Section 4).
5. For strings, the bound O(|x|/w + log |x|) is matched or improved in various
   directions by later dynamic tries, all of which cite the SPIRE 2010 paper:
   packed c-tries (O(m/alpha + f(k, n)) expected, f any dynamic predecessor
   bound), c-trie++ (O(m/alpha + lg alpha) expected), zip-tries (O(k/alpha +
   log n), comparison based). The two hash-based ones claim only lookup,
   prefix search and updates, not predecessor or range queries; none gives
   bounds in terms of |x^+|, |x^-| (Section 6).
6. A hash family compatible with the probability analysis exists and is
   already cited by the authors' own ESA 2010 paper: polynomial hashing over a
   prime field evaluated by Horner's rule (Dietzfelbinger, Gil, Matias and
   Pippenger, ICALP 1992; textbook treatment in Thorup's notes, Section 5.3).
   With it the signature length becomes c lg n + lg L bits, L the maximum
   string length in bits, instead of c lg n + lg w (Section 8).

## 2. Integer predecessor search: upper bounds

All structures here store n integers of l <= w bits. Unless stated otherwise
the model is the word RAM with multiplication.

### 2.1 van Emde Boas trees

- Bib: `vanEmdeBoas1975Preserving`, `vanEmdeBoas1977Preserving`,
  `vanEmdeBoas1976Design`.
- Statement **[S: Navarro2020Predecessor, Sections 2.3, 3.1.1]**: predecessor
  queries and updates in O(log log u) = O(log l) time, deterministic, worst
  case; space Theta(u) words. The 1977 note reduces space; the original
  avoids multiplication by means of "canonical subtrees".
- Relation: same time as the z-fast trie on integers; space depends on the
  universe. The survey compares the jump pointers of the z-fast trie to these
  canonical subtrees.
- Cites z-fast tries: predates.

### 2.2 Willard's x-fast and y-fast tries

- Bib: `Willard1983Loglogarithmic`.
- Statement **[S: Navarro2020Predecessor, Section 3.1.2]**: x-fast trie:
  O(n log u) words, queries O(log log u) worst case (with perfect hashing),
  updates O(log u) expected. y-fast trie: O(n) words, queries O(log log u)
  worst case, updates O(log log u) *expected amortized*, by bucketing
  Theta(log u) keys per bucket.
- Relation: the z-fast trie has the y-fast bounds without bucketing and
  without amortisation other than that of the dictionary; this is the claim
  at lines 251-254 and it stands. It also handles variable-length strings.
- Cites z-fast tries: predates.

### 2.3 Mehlhorn and Naher

- Bib: `Mehlhorn1990Bounded`.
- Statement **[S: Navarro2020Predecessor, Section 3.1.2]**: van Emde Boas
  recursion with hashed clusters; O(n log log u) space without bucketing,
  O(n) with bucketing; static queries O(log log u) worst case; dynamic
  queries and updates O(log log u) expected (expected amortized with
  bucketing). The paper cites it at line 249 with dynamic perfect hashing.
- Relation: recursive; linear space only with buckets. Consistent with the
  claim at lines 264-266.

### 2.4 Fusion trees

- Bib: `Fredman1993Surpassing`.
- Statement **[S: Patrascu2014Dynamic, Section 1, footnote 2;
  Navarro2020Predecessor, Section 3.2.1]**: static queries O(log n / log w);
  dynamic O(log n / log b + log b) amortized for any b <= w^{1/6}, hence
  O(log n / log log n); linear space; deterministic; multiplication needed.
- Relation: incomparable (bounds in n, not in l). Better than the z-fast trie
  when log n / log w = o(log w); worse for large n.

### 2.5 Andersson; Andersson-Thorup exponential search trees

- Bib: `Andersson1995Sublogarithmic`, `Andersson1996Faster`,
  `Andersson2007Dynamic` (journal version, merging FOCS 1996 and STOC 2000 and
  including the string result of `Andersson2001Dynamic`).
- Statement **[R: arXiv cs/0210006v2, Theorem 1, Corollary 4]**: a general
  reduction from polynomial-space static structures to linear-space dynamic
  ones. Deterministic, linear space, *worst-case* time per search, insert and
  delete

      O( min{ sqrt(log n / log log n),
              log log n * log log U / log log log U,
              log log n + log n / log W } ),

  U < 2^W a bound on the keys, W the word length; O((log n)^{3/4+o(1)}) with
  AC0 operations only. The first branch is optimal as a function of n alone
  (Beame-Fich).
- Relation: incomparable. Deterministic and worst case, which the z-fast trie
  is not (updates are expected). As a function of l = log U alone the second
  branch is worse than O(log l) by the log log n factor divided by
  log log log U. The paper cites this at lines 383-385; the statement there
  is correct but now incomplete (Section 9).
- Cites z-fast tries: predates.

### 2.6 Beame-Fich

- Bib: `Beame2002Optimal`.
- Statement **[S: Andersson2007Dynamic, Theorem 3; Navarro2020Predecessor,
  Section 3.3.3]** (I read only the first page of the STOC 1999 version):
  static, polynomial space n^{O(1)}, deterministic queries in
  O(min{ log log U / log log log U, sqrt(log n / log log n) }); matching
  cell-probe lower bounds for polynomial space. With exponential search trees
  this gives the first two branches of Corollary 4 above.
- Relation: shows that O(log l) can be beaten by a log log l factor, but only
  with polynomial space (static) or at the price of a log log n factor
  (dynamic). In linear space O(log l) remains optimal in the regime of
  Section 2.8.

### 2.7 Patrascu-Thorup 2006 and 2007 (static problem)

- Bib: `Patrascu2006Timespace`, `Patrascu2007Randomization`.
- Statement **[R: arXiv cs/0603043v1, abstract and Section 1]**: for n
  integers of l bits, a RAM with word length w and S words of space, with
  a = lg(S/n) + lg w, the optimal search time is, up to constants,

      min{ log_w n,
           lg((l - lg n) / a),
           lg(l/a) / lg( (a / lg n) * lg(l/a) ),
           lg(l/a) / lg( lg(l/a) / lg(lg n / a) ) }.

  Upper bounds are deterministic RAM algorithms; the lower bound is in the
  cell-probe model. "In the important case of w = l = gamma lg n, for
  gamma > 1 (i.e. polynomial universes), and near linear space (such as S =
  n lg^{O(1)} n), the optimal search time is Theta(lg l). Thus, our lower
  bound implies the surprising conclusion that van Emde Boas' classic data
  structure ... is optimal in this case." Also: "In external memory (w > l),
  it follows that the optimal strategy is to use either standard B-trees, or
  a RAM algorithm ignoring the larger block size."
- SODA 2007 **[A: abstract on the first author's publication page]**: the
  lower bounds are extended "to give lower bounds for randomized query
  algorithms with constant error probability"; "our trade-off lower bounds
  are tight for any combination of parameters".
- Relation: this is the source of the paper's statement at lines 244-247.
  The paper cites only the SODA 2007 paper for "matching upper and lower
  bounds"; the upper bounds and the deterministic lower bound are in the STOC
  2006 paper, which is not in the bibliography. The external-memory sentence
  supports the cache-oblivious claims for one-word keys: no structure can
  beat min{B-tree, RAM structure} I/Os **[I]**.
- Cites z-fast tries: predates.

### 2.8 Patrascu-Thorup 2014 (dynamic problem)

- Bib: `Patrascu2014Dynamic`. FOCS 2014, pp. 166-175.
- Read: **[R: arXiv 1408.3045v2, whole paper; formulas checked on the rendered
  PDF]**.

**What it achieves.**

1. *Dynamic fusion tree* (Sections 3-4). A dynamic set of n integers of w
   bits, w >= log n, in O(n) words, supporting insert, delete, predecessor,
   successor, rank and select in O(log n / log w) time. "Our main result is a
   deterministic linear space data structure supporting all the above
   operations in O(log n / log w) time". Sets of size w^{O(1)} are handled in
   constant time per operation (dynamic fusion node, up to w^{1/4} keys per
   node; node rebuilding in the background, de-amortised as in
   Andersson-Thorup).
2. *Optimality of that bound.* Optimal for dynamic rank and select
   (Fredman-Saks lower bound, `Fredman1989Cell`). Optimal for dynamic
   predecessor "whenever log n / log w = O(log w / log log w)" (Beame-Fich
   lower bound, valid for randomised structures by `Sen2008Lower`).
3. *Complete randomised picture for predecessor alone* (Section 6, two pages
   that rely on Section 5 of the full version of the STOC 2006 paper). "With
   key length l <= w, we get that the optimal expected maximal operation time
   for dynamic predecessor (no rank or select) is"

       Theta( max{ 1, min{ A, B, C } } )
       A = log n / log w
       B = log(l / log w) / log( log(l / log w) / log(log n / log w) )
       C = log( log(2^l - n) / log w )

   in linear space. A is the dynamic fusion tree. C is "van Emde Boas' data
   structure with some simple tuning. This includes the space saving Y-fast
   tries of Willard, but using our dynamic fusion nodes in the bottom trees".
   B dynamises the small-space structure of the STOC 2006 paper with periodic
   rebuilding. B and C "are fairly easy to implement dynamically if we use
   randomized hash tables with constant expected update and query time", so
   they hold in expectation. "Maximal operation time" is the maximum of query
   and update time.
4. *Lower bound* (Sections 5-6): cell-probe, any space ("dynamic cell-probe
   lower bounds hold regardless of the available space" as long as updates
   are polylogarithmic), randomised. Branches A and B are the static bound at
   near-linear space; C strengthens the static branch
   log((l - lg n) / log w) to log(log(2^l - n) / log w).

**Model.** Word RAM with multiplication (which the paper notes is necessary
for constant-time fusion nodes with standard instructions, citing Thorup
2003). Fixed-length integer keys that fit in a word. Nothing on strings, on
variable length, or on keys longer than a word. Bounds: deterministic for A,
expected for B and C; none is "with high probability".

**Cites z-fast tries?** No. (The acknowledgement thanks Belazzougui "for
pointing out a bug in one of the bounds stated for predecessor search in an
earlier version of this paper".)

**Comparison with the z-fast trie [I]** (the case analysis for l = w in the
static setting is in Navarro2020Predecessor, Section 3.4.1; the dynamic one
below is mine). Take l = w, so that the z-fast trie takes O(log w).

| Regime (l = w) | Optimum (Patrascu-Thorup) | z-fast trie |
|---|---|---|
| log n <= log^2 w / log log w | Theta(max{1, log n / log w}); O(1) for n = w^{O(1)} | O(log w): not optimal |
| log^2 w / log log w <= log n = w^{o(1)} | Theta(log w / log(log w / log log n)) = o(log w) | not optimal, by a factor up to log log w |
| log n = w^{Omega(1)} and log(2^w - n) = w^{Omega(1)} | Theta(log w) | **optimal** (expected, max of query and update) |
| log(2^w - n) = w^{o(1)} (co-tiny set) | Theta(max{1, log(log(2^w - n) / log w)}) = o(log w) | not optimal |

Derivation: C = Theta(log w) unless the complement of the set is tiny.
Writing log n = w^eps log w gives B about log w / log(1/eps), which is
Theta(log w) iff eps = Omega(1). A >= log w as soon as log n >= log^2 w.

Remarks **[I]**.

- The optimal regime, "word length polynomial in log n", contains the
  transdichotomous setting w = Theta(log n) with universe n^c, c > 1, and
  the practical one (w = 64, n from 2^20 to 2^32). So the z-fast trie is
  optimal where it is used; it is not optimal "for all n and w".
- For general l <= w replace log w by log l in the z-fast bound and use the
  formula with l: the two differ only for l = O(polylog w), where the optimum
  is log(l / log w) against log l (constant against log log w when
  l = O(log w)).
- Statically, dense sets are easy (branch log((l - lg n)/a)); dynamically
  they are not: for n = 2^l / 2 the dynamic optimum is still Theta(log l).
  This is a point in favour of comparing with the dynamic bound only.
- What Patrascu-Thorup do not give and the z-fast trie does: strings and
  variable length; time depending on |x|, |x^+|, |x^-| rather than on w;
  worst-case queries when the dictionary has worst-case lookups; no
  bucketing in the branch that matters here (their branch C is a y-fast
  trie with fusion nodes in the buckets); explicit space constants.
- Priority: SPIRE 2010 predates FOCS 2014. The "first" at line 265 is not
  threatened by this paper; "optimal" is what needs a qualifier.

### 2.9 Engineering of the theoretical structures

- `Dinklage2021Engineering` **[R: abstract of arXiv 2104.06740, local copy]**:
  engineered dynamic predecessor structures for w = 32, 40, 64: universe
  reduction in the style of van Emde Boas, y-fast trie variants, and B-trees
  including "an implementation of dynamic fusion nodes [Patrascu and Thorup,
  FOCS'14]". Does not mention z-fast tries. It is the natural experimental
  comparison for the integer experiments of the paper, and it qualifies the
  remark of the survey (Section 5.6) that the Patrascu-Thorup structures "are
  not good candidates for a verbatim implementation".

## 3. Lower bounds that apply

- **Static, integers** `Patrascu2006Timespace`, `Patrascu2007Randomization`:
  the four-branch formula of Section 2.7, cell-probe model, tight, valid for
  randomised queries with constant error **[R/A as above]**. Earlier:
  Ajtai; Miltersen; Miltersen, Nisan, Safra, Wigderson; Beame-Fich;
  Sen-Venkatesh **[S: Navarro2020Predecessor, Section 4]**.
- **Dynamic, integers** `Patrascu2014Dynamic`: the formula of Section 2.8 for
  the maximum of query and update time, any space, randomised, expected
  **[R]**. Open according to the survey (Section 6): the deterministic
  complexity; whether the bounds can be matched "with high probability,
  instead of in expectation"; whether updates can be much faster than
  queries.
- **Rank/select** `Fredman1989Cell`: Omega(log n / log w) amortised for
  dynamic rank and select **[S: Patrascu2014Dynamic, Section 5]**. The z-fast
  trie does not support rank or select, so the bound is not violated; a
  related-work sentence could say that adding them costs at least this.
- **Strings.** I found no cell-probe lower bound specific to variable-length
  strings. What is available:
  - the trivial Omega(|x|/w) (or Omega(|x|/B) I/Os) to read the query;
  - Andersson-Thorup's padding argument **[R: Andersson2007Dynamic, Section
    1.9]**: give all keys a common prefix of l words; a search needs
    Omega(l + T(n)) where T is the one-word lower bound. With the 2014 bound
    this gives Omega(|x|/w + pt(n, w)), pt being the formula of Section 2.8
    with l = w **[I]**;
  - in external memory, O(log_B n + |P|/B) is "optimal for unbounded
    alphabets" (comparison-based) **[R: Brodal2006Cacheoblivious, abstract]**;
    this does not apply to the z-fast trie, which is not comparison based.
  Consequence **[I]**: for |x| >= w the bound O(|x|/w + lg |x|) equals
  O(|x|/w + lg w) (footnote at lines 393-396), so on long strings the z-fast
  trie is optimal in the same regime as on integers (log n = w^{Omega(1)})
  and not otherwise. Nothing is known about the optimality of the terms
  lg |x^+|, lg |x^-|.
- **Range reporting is easier than predecessor**: see Section 5.2.
- **Multiplication.** `Thorup2003AC0` proves multiplication necessary for
  constant-time fusion nodes and for constant-time membership on sets of
  size Omega(log n) with standard instructions **[S: Patrascu2014Dynamic,
  Section 1]**; `Andersson1996Static` shows that dictionaries on AC0 RAMs
  need Theta(sqrt(log n / log log n)) query time **[S: title of the paper and
  Ruzic2009Making, Section 2.2]**. Since the z-map is a constant-time
  dictionary, this answers the authors' note at line 315 ("Do we really need
  multiplications?"): some non-AC0 operation such as multiplication is needed
  for the dictionary, independently of the most-significant-bit computation
  **[I]**.

## 4. The Navarro-Rojas-Ledesma survey

- Bib: `Navarro2020Predecessor`. ACM Computing Surveys 53(5), article 105,
  35 pages, 2020. **[R: whole text, local copy
  `/Users/vigna/pub/zfast/related/3409371.pdf`]**.

What it says about z-fast tries:

- Section 3.1.2 ("Reducing the Space of van Emde Boas Trees") has four
  parts: x-fast tries, y-fast tries, Mehlhorn-Naher, and "*Z-fast tries:
  linear space, and fast updates (in expectation)*", with Figure 4 showing a
  z-fast trie. "In 2010, Belazzougui et al. introduced the dynamic z-fast
  trie, a version of Willard's x-fast tries that achieves linear space and
  O(log log u) query and update time with high probability. The first version
  of this data structure was actually introduced one year earlier ... but it
  was static and could only find the longest common prefix between q and its
  predecessor." It lists four changes with respect to the x-fast trie:
  compacted trie; a single hash table; handles (2-fattest numbers) and fat
  binary search; min/max pointers that "do not point directly to these nodes,
  but to some other descendant in the path ... (similar in essence to the
  canonical subtrees of van Emde Boas)", reached in O(log log u).
- On priority of the idea: "A very similar idea for searching longest common
  prefixes in a trie was introduced independently by Ruzic, although there,
  the keys associated with a node are stored in different hash tables
  depending on their size instead of storing them all in the same hash, and
  the data structure is static." (Reference: `Ruzic2009Making`.)
- Bounds attributed: "queries in O(log log u) worst-case time, and updates in
  O(log log u) expected time. As for other data structures, the only reason
  of having probabilities in the update bound is the use of hashing." The
  citation there is the ESA 2010 paper, apparently a slip for SPIRE 2010.
  Note the inconsistency inside the survey ("with high probability" a few
  lines earlier): it mirrors the unsettled wording of the paper's abstract.
- Section 5.1 (distance-sensitive search): see Section 5.1 below.
- Section 5.5 ("Prefix Search"): "The z-fast trie, for instance, was
  introduced in this context of prefix and predecessor search for strings.
  Given a set S of prefix-free strings ... of variable length within O(w),
  z-fast tries can answer prefix queries in O(log max(|p|, |p^+|, |p^-|))
  time". It then cites Fischer-Gawrychowski for general alphabets.
- The z-fast trie does not appear in the "short story" (Section 2), which
  goes from van Emde Boas and fusion trees to Patrascu-Thorup, nor in
  Section 5.6 on practical solutions.

Other points of the survey used in this report: the case analysis of the
static bound for l = w (Section 3.4.1); the statement that the dynamic
problem is solved only in expectation (Sections 2.2, 3.4.2, 4.2.2, 6); the
remark that all length-reduction structures have times independent of n.

## 5. Variants on integers

### 5.1 Distance-sensitive (local) searches

- `Bose2013Fast` **[R: abstract and introduction of the CCCG 2010 version,
  local copy `paper69.pdf`]**: successor searches in O(log log Delta)
  expected time and updates in O(log log Delta) expected amortized time,
  Delta the distance between the query and its successor; O(n log log log U)
  expected space. x-fast and y-fast tries plus skip lists. Does not cite
  z-fast tries (the CCCG version predates SPIRE 2010).
- `Belazzougui2012Predecessor` (arXiv only) **[R: abstract, introduction,
  theorem statements]**: static; O(log log d(x, S)) queries with O(n log w)
  bits in addition to the keys, by fat binary search with a restricted
  initial interval; also O(log(w - log D(x, S))) and finger variants.
- `Ehrhardt2017Delta` **[S: Navarro2020Predecessor, Section 5.1;
  Ehrhardt2015Indepth, Theorems 6.2, 8.1, 8.2, read]**: "another variant of
  the z-fast trie, which, using O(n) space, supports queries in
  O(log log Delta) time, and updates in O(log log Delta) expected worst-case
  time". The thesis also re-derives the dynamic z-fast trie on integers with
  a *different mechanism for the extreme leaves*: it calls jump pointers
  "complex techniques" and replaces them with a path decomposition
  maintained by modified tango trees, in O(log log |U|) per operation
  (Theorem 6.2). It states that the z-fast trie "was independently discovered
  by Ruzic and Belazzougui et al in 2009".
- Relation: the z-fast trie's bound O(lg max{|x|, |x^+|, |x^-|}) is a
  length-sensitive bound for variable-length strings, not a
  distance-sensitive one; on fixed-length integers it degenerates to
  O(log w). These papers are complementary. The alternative to jump pointers
  in Ehrhardt's thesis should be mentioned where jump pointers are
  introduced.

### 5.2 One-dimensional range reporting (existential range queries)

- `Mortensen2005Dynamic` **[A: arXiv cs/0502032 abstract]**: dynamic set of
  w-bit integers, "report a point (equivalently, all points) in a given
  interval": "updates in O(lg w) time and ... queries in O(lg lg w) time. The
  update time is identical to the van Emde Boas structure, but the query time
  is exponentially faster." Linear space, by "the first scheme for dynamic
  perfect hashing requiring sublinear space". Randomised (hashing).
- `Alstrup2001Optimal` **[S: Navarro2020Predecessor, Section 5.5]**: static,
  linear space, an element of the query interval in O(1), all k in O(k).
- Relation: the existential range query of the paper (Theorem `th:bounds`:
  O(lg max{|x|, |y|})) is exactly this problem on fixed-length integers, and
  an existential prefix query is the special case [x0...0 .. x1...1]. So the
  z-fast trie is exponentially slower than the best known dynamic structure
  on these two queries for l = Theta(w) **[I]**. I found no extension of the
  Mortensen-Pagh-Patrascu structure to variable-length strings.
- Cites z-fast tries: predates.

### 5.3 Indices with few probes to the keys

- `Cohen2015Minimal` (ESA 2013 as "Minimal Indices for Successor Search")
  **[R: abstract, Table 1 and related-work passages, local copy]**: static;
  an index of O(n log w) bits with an optimal number of probes and O(1)
  probes to the keys, at the price of O(log w) extra word operations. It
  describes itself as "matching the space complexity of the
  (probe-suboptimal) z-fast trie of Belazzougui et al.", and uses "a z-fast
  trie applied to w / log w keys" as its alpha-node. Cites the SODA 2009 and
  ESA 2010 papers, not the dynamic one. The survey notes that a dynamic
  index of this kind is open.

### 5.4 Other models

- `Bille2022Predecessor` (journal: `Bille2024Predecessor`) **[R: abstract and
  introduction of arXiv 2201.11550v2]**: on the ultra-wide word RAM (words of
  w^2 bits, scattered memory access), linear space, predecessor in constant
  time and updates in amortized expected constant time, via "a new
  implementation of the classic x-fast trie". It restates the
  Patrascu-Thorup bound as the tight word-RAM bound, and lists the SPIRE 2010
  paper among the upper bounds for predecessor and for string searching.

## 6. Strings on the word RAM

### 6.1 Ruzic: deterministic signatures and a static string predecessor structure

- Bib: `Ruzic2007Making` (SODA 2007, pp. 900-909), `Ruzic2009Making` (ACM
  TALG 5(3), article 26). **[R: both versions, local copies `ruzic.pdf` and
  `a26-ruzic.pdf`; Sections 1, 2, 6, 7 of the journal version]**.
- Statements (journal version). Theorem 2.3: "Let S be a set of n binary
  strings, and let N be the sum of their lengths in bits. There is a
  deterministic static predecessor structure that can store S using O(N) bits
  of space and answer queries in O(|s|/w + log |s|) time, with s being the
  query argument. On sorted input set, the structure can be constructed in
  time O(n + N/w)." Theorem 2.9: the same in the cache-oblivious model with
  O(|s|/B + log |s|) I/Os and O(N/B) construction I/Os. "In the special case
  when all keys have a fixed length of w bits ... Query time is then
  O(log w)", with the footnote that this "is not really a new thing".
  The conference version has weaker bounds: O(|s|/w + log |s| + log w +
  log log n) (Theorem 2.1) and O(|s|/B + log |s| + log log n) I/Os
  (Theorem 2.2).
- Technique (Section 7): a van Emde Boas recursion adapted to variable-length
  keys: for each power of two l there is a string dictionary; a query looks
  up the l-bit prefix of the remaining suffix, recursing on the lower part on
  a match and on the upper part otherwise; "each edge of the trie is cut at
  most once, and only one dictionary may hold an entry that refers to the
  edge". Dictionaries are built from deterministic signature functions,
  one-to-one on S, with O(log n)-bit values (Theorems 2.1, 2.7). A top/bottom
  decomposition with groups of polylogarithmic size absorbs the construction
  cost. The structure is recursive, uses bucketing, and is static.
- Relation to the z-fast trie:
  - same query bounds as Theorem `th:boundslong` in both models, obtained
    *deterministically* and without false positives, in the static case,
    earlier (2007 with extra additive terms; July 2009 in the form above);
  - space O(N) bits including the keys, against O(n) words plus the keys;
  - no updates; bounds in |s| only, not in |s^+|, |s^-| (predecessor queries
    are supported with the same bound, which is *better* than the paper's
    O(|x|/w + lg max{|x|, |x^+|, |x^-|}) when the neighbours are long);
  - several dictionaries indexed by length instead of one z-map, recursion
    and buckets instead of a single uniform structure.
- Cites z-fast tries: no. The journal version is dated July 2009, after SODA
  2009, and does not cite the monotone hashing paper (the authors' names do
  not occur in its text).
- This paper must be cited, and the "first" at line 373 must be restricted
  to dynamic structures (Section 9).

### 6.2 Andersson-Thorup string searching

- Bib: `Andersson2001Dynamic` (two-page SODA abstract), full version in
  `Andersson2007Dynamic` **[R: arXiv cs/0210006v2, Sections 1.9, 1.10.2, 6]**.
- Statement (Theorem 6): "For the dynamic string searching problem, if the
  longest common prefix between a key x and the stored keys has l words, we
  can insert, delete, and search x in O(l + sqrt(log n / log log n)) time,
  where n is the current number of keys. In addition to the stored keys
  themselves, our space bound is O(n)." Deterministic, worst case. Strings
  are sequences of one-word characters with a terminator (prefix-free).
  Compacted trie over one-word characters; each node has a dictionary for its
  heavy children and an exponential search tree for the light ones. Claimed
  optimal by the padding argument of Section 3.
- Relation: incomparable.
  - Better than the z-fast trie when sqrt(log n / log log n) = o(lg w) (for
    |x| >= w), i.e. for small sets; deterministic; worst case; charges the
    longest common prefix, not |x|.
  - Worse for short queries on large sets, and in the cache-oblivious model
    (one random access per word of the common prefix), as the paper argues
    at lines 397-405. That argument stands.
  - The paper quotes the bound as "O(|x|/w + sqrt(lg n / lg lg n))" (lines
    391-392); the theorem is stated with the common prefix in words.
- With the 2014 predecessor structure in the nodes one would expect
  O(l + log n / log w); the recursion T(m) <= O(S(m)) + T(m^{1-1/k}) of
  Section 6.2 of the paper sums to O(log m / log w) for S(m) = log m / log w
  **[I]**. I did not find this stated anywhere. In expectation it follows
  from packed c-tries (Section 6.5).
- Cites z-fast tries: predates.

### 6.3 Fischer-Gawrychowski: wexponential search trees

- Bib: `Fischer2015Alphabet` (CPM 2015). **[R: arXiv 1302.3347v1, abstract
  and Section 1; the published version was not accessible]**.
- Statements (arXiv v1). Theorem 1: static compacted trie, O(n) space in
  addition to the strings, prefix and predecessor queries in O(m +
  lg lg sigma) deterministic worst-case time for patterns of m characters.
  Theorem 3: dynamic, linear size, insertion of a string of length l in
  O(l + lg^2 lg sigma / lg lg lg sigma) deterministic worst-case time and
  queries in O(m + lg^2 lg sigma / lg lg lg sigma). Deletions are not
  mentioned. The survey reports the dynamic bound as O(|p| + log log sigma);
  I could not check which is right for the published version.
- Model: word RAM with Omega(lg n)-bit words; characters are processed one
  at a time (no packing), so the time is Theta(m) = Theta(|x| / log sigma),
  not O(|x|/w).
- Relation: incomparable. Deterministic and worst case; better for patterns
  of O(1) words over large alphabets (additive term in sigma rather than in
  |x|); worse by a factor alpha = w / log sigma on long strings over small
  alphabets; for the binary alphabet it is the plain O(|x|) trie search.
- Cites z-fast tries: no (arXiv v1).

### 6.4 Bille-Gortz-Skjoldjensen: deterministic indexing for packed strings

- Bib: `Bille2017Deterministic` (CPM 2017, LIPIcs 78, 6:1-6:11).
  **[R: arXiv 1612.01748v1, abstract and Sections 1-2]**.
- Statement (Theorem 1): for a packed string S of length n over an alphabet
  of size sigma, an index built in O(n) deterministic time and O(n) space
  supporting Count and Predecessor in O(m/alpha + log m + log log sigma) and
  Locate with an additional occ, alpha = w / log sigma, w = Theta(log n).
  Static, deterministic, a suffix-tree/suffix-array index (the keys are the
  suffixes of one string).
- Relation: the same shape of bound as the z-fast trie, O(|x|/w + log |x|)
  up to the alphabet term, obtained deterministically but statically, for
  w = Theta(log n), and by different means (it follows Fischer-Gawrychowski
  and a packed suffix array; no binary search on prefix lengths). The zuffix
  array (already cited in the paper) is the randomised counterpart based on
  the z-map.
- Cites z-fast tries: no. (It cites Belazzougui's packed string matching and
  Ruzic's ICALP 2008 dictionary.)

### 6.5 Takagi, Inenaga, Sadakane, Arimura: packed compact tries

- Bib: `Takagi2016Packed` (IWOCA 2016), `Takagi2017Packed` (IEICE Trans.
  Fundamentals E100-A(9)). **[R: arXiv 1602.00422v1, abstract, Section 1,
  related work]**.
- Statement (Theorem 1): let f(k, n) be the time of any dynamic
  predecessor/successor structure storing k integers from [1, n] in
  O(k log n) bits. A packed c-trie stores k strings of total length n in
  n log sigma + O(k log n) bits and "supports prefix search and
  insertion/deletion for a given string of length m in O((m/alpha) f(k, n))
  worst-case time or in O(m/alpha + f(k, n)) expected time", with
  alpha = log_sigma n (w = log n). Corollary 2 instantiates f with Beame-Fich
  or y-fast tries: O(m/alpha + log log n) expected.
- With `Patrascu2014Dynamic` as the predecessor structure, f is the optimal
  dynamic bound for universe n **[I]**; this is the cleanest way to obtain
  "O(|x|/w + optimal predecessor time)" for prefix search and updates, in
  expectation.
- What it says about the z-fast trie: "Belazzougui et al. proposed a
  randomized compact trie called the signed dynamic z-fast trie ... supports
  prefix search in O(m/alpha + log m) worst-case time only with high
  probability, and supports insert/delete operations in O(m/alpha + log m)
  expected time only with high probability. On the other hand, our packed
  c-trie always return the correct answer", with the footnote "The O(log m)
  expected bound for insertion/deletion stated in [SPIRE 2010] assumes that
  the prefix search for the string has already been performed." (The z-fast
  trie does always return the correct answer; what holds with high
  probability is the running time. The related-work text should make this
  hard to misread.)
- Relation: incomparable. log log n against log |x| in the additive term:
  better for long queries, worse for queries shorter than log n bits. No
  predecessor or range queries are claimed, only prefix search and updates.

### 6.6 Tsuruta et al.: c-trie++

- Bib: `Tsuruta2020Ctrie` (DCC 2020), `Tsuruta2022Ctrie` (Information and
  Computation 285, 104794). **[R: arXiv 1904.07467v3, Sections 1-2; extract in
  `/Users/vigna/pub/zfast/review-2026-11/literature-extracts/ctriepp.txt`]**.
  Already cited in the paper as `TKKctriepp` (arXiv only); the published
  versions should be cited instead.
- Statement: a keyword dictionary for k strings of total length n in
  n lg sigma + Theta(k lg n) bits, or |T| lg sigma + Theta(k w) bits with
  front coding, supporting lookup, prefix search, insert and delete "in
  O(m/alpha + lg alpha) expected time", alpha = w / lg sigma. A macro trie of
  micro tries of height alpha; the micro tries are "alphabet-aware z-fast
  tries" on strings that fit in a word.
- What it says about the z-fast trie: it "uses |T| lg sigma + Theta(kw) bits
  of space, and supports all operations in O(m/alpha + lg m + lg lg sigma)
  expected time", with the footnote "This time bound can be achieved by
  omitting the jump pointers ... since their maintenance needs additional
  time. The jump pointers are used to enable additional operations on the
  trie such as predecessor queries, which we omit in this article." And:
  "the z-fast trie becomes inferior for string lengths m > w / lg^2 sigma".
- Relation: in bits the bound is O(|x|/w + lg(w / lg sigma)). For a constant
  alphabet this is O(|x|/w + lg w), which is never better than the z-fast
  bound O(|x|/w + lg |x|) and is worse for |x| = w^{o(1)}; for large
  alphabets it removes the binary search inside a character **[I]**. No
  predecessor or range queries; expected time only. The authors' C++ port of
  Sux4J's `ZFastTrie` is the source of the only third-party measurements
  (see `REVIEW.md`, Section 4.2).

### 6.7 Eppstein, Gila, Goodrich, Kitagawa: zip-tries

- Bib: `Eppstein2025Ziptries` (ACDA 2025, pp. 130-143). **[R: arXiv
  2505.04953v2, abstract, introduction, related-work appendix]**.
- Statement: search and update for strings of k characters in O(k + log n)
  time in the RAM model and O(k/alpha + log n) in the word RAM, with
  O(log log n + log log(k/alpha)) bits of metadata per node w.h.p.;
  LCP-aware variants; parallel and external-memory variants (a parallel
  string B-tree). Comparison based on string comparisons with LCP
  bookkeeping, in the line of `Grossi1999Efficient`.
- What it says about the z-fast trie: "The dynamic z-fast tries by
  Belazzougui et al. achieve O(k/alpha + log m) time, returning a correct
  result w.h.p., but are complex and not easily parallelized." It also calls
  c-trie++ "an optimized implementation of the z-fast trie data structure"
  and reports that zip-tries outperform it on long strings. The phrase
  "returning a correct result w.h.p." is the same misreading as in
  Section 6.5.
- Relation: log n against log |x|: worse asymptotically whenever |x| is
  subpolynomial in n, but simple, comparison based and LCP-aware.

### 6.8 Other dynamic tries

- `Jansson2015Linked` **[S: Tsuruta2022Ctrie, Section 1.2]**: a dynamic trie
  in O(|T| lg sigma) bits; prefix search in O((m / log_sigma |T|)
  (lg lg |T|)^2 / lg lg lg |T|) time, or O(m / lg_sigma |T| + lg lg |T|)
  worst case with leaf insertions and deletions in O(lg lg |T|) expected
  amortized time.
- `Grossi1999Efficient` **[S: Eppstein2025Ziptries, Section 1]**: a framework
  turning linked comparison-based structures into string structures with
  search, insert and delete in O(k + log n). This is the comparison-based
  baseline (the "O(|s|/w + log n)" that Ruzic calls "the best alternatives").
- `Belazzougui2016Predecessor` (Encyclopedia of Algorithms entry "Predecessor
  Search, String Algorithms and Data Structures"): not accessible; the survey
  cites it for the length-reduction / cardinality-reduction terminology.

### 6.9 Summary table for strings (word RAM)

| Structure | Dynamic | Query time (m characters, |x| bits) | Guarantee | Queries | Cites z-fast |
|---|---|---|---|---|---|
| Ruzic 2009 | no | O(|x|/w + lg |x|) | deterministic | predecessor | predates |
| Andersson-Thorup 2007 | yes | O(lcp/w + sqrt(lg n / lg lg n)) | deterministic, worst case | predecessor | predates |
| **z-fast trie** | yes | O(|x|/w + lg max{|x|, |x^+|, |x^-|}) | w.h.p. (queries), expected (updates) | predecessor, prefix, range | - |
| Fischer-Gawrychowski 2015 | insertions | O(m + lg^2 lg sigma / lg lg lg sigma) | deterministic, worst case | prefix, predecessor | no |
| Bille et al. 2017 | no | O(m/alpha + lg m + lg lg sigma) | deterministic | count, locate, predecessor (suffixes) | no |
| Packed c-trie 2016 | yes | O(m/alpha + f(k, n)) | expected | prefix | yes |
| c-trie++ 2020 | yes | O(m/alpha + lg alpha) | expected | lookup, prefix | yes |
| Zip-trie 2025 | yes | O(m/alpha + lg n) | w.h.p. metadata; comparison based | predecessor, prefix, range | yes |

## 7. Strings in external memory and in the cache-oblivious model

- **String B-tree** `Ferragina1999String` **[S: Ruzic2009Making, Section 1.3;
  Brodal2006Cacheoblivious, Section 1.2; Eppstein2025Ziptries, Section 1]**:
  lookups and updates in O(log_B n + |s|/B) I/Os in the I/O model, "optimal
  for comparison-based structures and unbounded alphabets"; supports prefix,
  range and predecessor queries. "String B-trees crucially depend on knowing
  the value of B." Cited in the paper (lines 407-410); the description there
  agrees with these sources.
- **Cache-oblivious string dictionaries** `Brodal2006Cacheoblivious`
  **[R: abstract and Sections 1.1-1.2 of the authors' copy]**: static,
  cache-oblivious, linear space, "prefix queries in O(log_B n + |P|/B) I/Os,
  where n is the number of leaves in the trie, P is the query string ... This
  query cost is optimal for unbounded alphabets." According to Ruzic it needs
  a tall cache M >= B^{2+epsilon} and "is not simple".
- **Cache-oblivious string B-trees** `Bender2006Cacheoblivious` **[S: the
  authors' research abstract on the MIT CSAIL site; Ruzic2009Making, Section
  1.3; the PODS paper was not accessible]**: dynamic, randomised,
  cache-oblivious; membership in O(1 + log_B N + |k|/B) block transfers;
  insertions and deletions with an additional factor polylogarithmic in N on
  the |k|/B term (the formula is garbled in the HTML abstract); range queries
  in O(1 + log_B N + (|k| + |k'| + output)/B); "All bounds are with high
  probability"; a "provably good form of front compression"; supports
  predecessor searches; tall cache M >= B^2. The exact statements must be
  checked in the paper.
- **Ruzic** `Ruzic2009Making`, Theorem 2.9 **[R]**: static, deterministic,
  cache-oblivious, O(|s|/B + log |s|) I/Os, O(N) bits, construction in
  O(N/B) I/Os on sorted input.
- Relation to the z-fast trie (O(|x|/B + lg max{|x|, |x^+|, |x^-|}) I/Os,
  cache-oblivious, dynamic, w.h.p.):
  - against string B-trees and their cache-oblivious versions: incomparable,
    lg |x| against log_B n, as the paper says at line 409. The z-fast trie
    wins for short strings in large sets (lg |x| < log_B n) and loses for
    long strings in small sets or with large B;
  - cache-oblivious string B-trees are a *dynamic* cache-oblivious structure
    with predecessor search from 2006, so "good performance in the
    cache-oblivious model" (line 375) is not new in itself; what is new is a
    bound independent of n **[I]**;
  - Ruzic's structure has the same I/O bound, statically and
    deterministically.

## 8. Hashing assumptions

The open issue (line 1245, `CORRECTIONS.md` Section 8): the analysis of
Section 4.3 assumes fully random signatures and, at the same time,
constant-time signatures of arbitrary prefixes of x after O(|x|/w)
preprocessing.

### 8.1 Hash families with constant-time prefix signatures

- **Polynomial hashing over a prime field.** `Dietzfelbinger1992Polynomial`
  **[S: Thorup2015High, Section 5.3, read; the ICALP paper was not
  accessible]**. For a string x_0 ... x_d of characters in [u], a prime
  p >= u and a random a in [p], h_a(x) = sum_i x_i a^i mod p; for two
  different strings of length at most d + 1, "Pr[h_a(x) = h_a(y)] <= d/p",
  because a must be a root of a nonzero polynomial of degree at most d. With
  the coefficients reversed, Horner's rule H_0 = x_0, H_i = (a H_{i-1} + x_i)
  mod p computes the hash, and "we can easily update the hash value if [a]
  new character ... is added to the end of the string": the intermediate
  values H_i are the hashes of the prefixes. Thorup suggests Mersenne primes
  (2^61 - 1, 2^89 - 1) for speed.
  The authors' own ESA 2010 paper already uses this: "using incremental
  hashing [Dietzfelbinger et al., Section 5], we can preprocess p in time
  O(|p|/w) (and in |p|/B I/Os) so that hashing prefixes of p requires
  constant time afterwards" **[R: Belazzougui2010Fast, arXiv 1804.04720,
  Section 3.2]**. Citing the same family in `dynzfast.tex` resolves the
  naming part of the issue.
- **Karp-Rabin fingerprints** `Karp1987Efficient` (not accessible; described
  here from general knowledge, unverified): the string is read as an integer
  and reduced modulo a random prime; the fingerprint of an extension by one
  character is computed in O(1). The usual modern form is the polynomial
  hash above with a fixed prime and a random evaluation point.
- **Rabin fingerprints / CRC** (random irreducible polynomial over GF(2);
  `Broder1993Some` and Rabin's 1981 technical report, neither read): the
  family behind CRC-like hashes. A *fixed* polynomial such as CRC32C, which
  the C and C++ code uses, gives no collision bound against a worst-case
  input; the bound needs a random irreducible polynomial. Unverified, from
  general knowledge.
- **Deterministic signatures** `Ruzic2009Making`, Theorems 2.1 and 2.7
  **[R]**: functions that are one-to-one on S with O(log n)-bit values,
  evaluated in O(|s|/B) I/Os, built deterministically from the *sorted* set.
  They are static (the function depends on S), and the paper gives no
  constant-time evaluation on prefixes of a query; they serve a static
  deterministic variant, not the dynamic trie.

### 8.2 Redoing the bound with polynomial hashing [I]

This is my derivation; it should be checked by the authors.

- Characters are w-bit words; a prefix whose length is not a multiple of w
  needs one more multiply-add on the last partial word, together with its
  length. Since the z-map compares handle lengths as well as signatures (the
  "handle-length filter" of the footnote to `th:errors`), only strings of
  equal length must be distinguished, and for these the difference
  polynomial is nonzero. (Without the length, two strings that differ only
  by leading zero words have the same value under Horner's rule, so the d/p
  bound must be read for strings of equal length or with the length hashed
  in.)
- Let L be the maximum string length in bits, so that degrees are at most
  L/w. A fixed pair of distinct equal-length strings collides with
  probability at most L/(wp). A probe with a prefix that is not a handle is
  a false positive with probability at most n L/(wp), by a union bound over
  the at most n handles.
- Adaptivity. The prefixes probed depend on earlier answers, hence on the
  hash function. This does not matter: the signature-based execution
  coincides with the execution on exact handles until the first false
  positive, so the first false positive happens on a probe of the exact
  execution, whose at most floor(lg |x|) + 2 <= w + 2 <= 3w probes do not
  depend on the hash function. This is the argument already used in the
  proof of `th:errors`.
- Hence Pr[false positive in a query] <= 3w * n * L/(wp) = 3nL/p. Choosing a
  prime p >= L n^c gives 3/n^{c-1}, the bound of the paper, with signatures
  of **c lg n + lg L bits instead of c lg n + lg w**. For L <= 2^w and
  n <= 2^w this is still O(w) bits, i.e. O(1) words, as line 1254 says, and
  arithmetic modulo p costs O(1) per word.
- Duplicate signatures among handles: at most n^2 pairs, each colliding with
  probability at most L/(wp); the per-query bound of line 1278 follows in the
  same way.
- The hash function must be chosen independently of the keys and queries
  (oblivious adversary) and, since n is not known in advance, with the
  largest p from the start, as the paper already does for the signature
  length.

### 8.3 The dictionary itself

- With O(w)-bit signatures (long strings) or O(w)-bit (handle, length) pairs
  (short strings), the keys of the z-map are O(1) words. Full randomness is
  then unnecessary **[I]**:
  - `Dietzfelbinger1994Dynamic` (dynamic perfect hashing, already cited):
    worst-case constant lookups, constant amortized expected updates
    **[S: Patrascu2014Dynamic, Section 6]**;
  - `Pagh2004Cuckoo` (journal version of the cited ESA 2001 paper);
  - linear probing, which is what the implementations use: expected constant
    time with 5-independent hashing (`Pagh2009Linear`) or simple tabulation
    (`Patrascu2012Power`). Unverified, from general knowledge; neither paper
    was read for this survey.
- "With high probability using an appropriate dictionary" (abstract, lines
  191-193): `Arbitman2010Backyard` **[A: arXiv 0912.5424]** gives a dynamic
  dictionary with n elements in (1 + epsilon) n words and constant worst-case
  time per operation with high probability; `Arbitman2009Deamortized` and
  `Dietzfelbinger1990New` are the earlier results in this direction (not
  read); `Bender2023Iceberg` **[A: arXiv 2109.04548]** is the current
  state of the art for constant-time operations with low failure
  probability. One of these should be named where the abstract says
  "appropriate dictionary".

## 9. Claims in the paper that need revision

Line numbers refer to `dynzfast.tex`.

1. **Lines 244-245.** "The static version of this problem has been
   completely solved, with matching upper and lower
   bounds~\cite{PaTRDNHSP}."
   True, but the upper bounds and the deterministic lower bound are in
   `Patrascu2006Timespace`, which is not cited; `PaTRDNHSP` is the randomised
   lower bound. Add: the dynamic version is settled in expectation for
   l <= w by `Patrascu2014Dynamic`.
2. **Lines 245-247.** "If only linear space is available, in a significant
   number of cases variants of the van Emde Boas tree that answer in time
   O(lg l) are optimal even in the static setting using randomization."
   Correct. It can now be made precise: for l = w, exactly when
   log n = w^{Omega(1)} and (statically) w - log n = w^{Omega(1)} or
   (dynamically) log(2^w - n) = w^{Omega(1)}; in particular for polynomial
   universes with w = Theta(log n).
3. **Lines 264-266.** "the first simple optimal digital data structure that
   is neither recursive nor needs to use bucketing to achieve optimal time
   and linear space."
   - "optimal" is true only in the regime of item 2. For n = w^{O(1)} the
     optimum is O(1); for log n = w^{o(1)} it is o(log w).
   - "first ... neither recursive nor bucketing": I found nothing earlier
     with linear space, O(log l) time and neither device. Ruzic's structure
     has the same time but is static, recursive and bucketed; Mehlhorn-Naher
     is recursive; y-fast tries and branch C of Patrascu-Thorup use buckets.
     The claim stands with "dynamic" understood and with the SPIRE 2010 date.
   - Suggested form: "the first digital structure that attains the
     O(lg l) bound, which is optimal for sets of size 2^{l^{Omega(1)}}, in
     linear space without recursion or bucketing".
4. **Lines 290-291.** "The z-fast trie can answer optimally a number of other
   queries, such as predecessor/successor queries, prefix queries, and range
   queries."
   - Predecessor/successor: optimal only as in item 2, and nothing is known
     about the optimality of lg max{|x|, |x^+|, |x^-|} (for a query far
     shorter than its neighbours, Ruzic's static bound O(|x|/w + lg |x|) is
     better).
   - Existential range and prefix queries: not optimal on integers.
     `Mortensen2005Dynamic` has O(lg lg w) queries with O(lg w) updates;
     statically `Alstrup2001Optimal` has O(1). (The authors' own ESA 2010
     paper has *weak* prefix search, meaningful only when the prefix occurs,
     in O(|p|/w) with more space: `Belazzougui2010Fast`, Theorem 6.)
   - Remove "optimally" or restrict it to predecessor search in the stated
     regime.
5. **Lines 314-317.** "under a full randomness assumption" with the footnote
   that hashing is used only for the hash table and the signatures.
   The assumption can be removed: Section 8 above. The authors' question on
   multiplications in the same lines is answered in Section 3 (needed for a
   constant-time dictionary).
6. **Lines 373-375.** "To our knowledge, this is the first paper to propose a
   digital data structure for variable length strings with optimal
   performance in the RAM model and good performance in the cache-oblivious
   model."
   - Contradicted for the static case by `Ruzic2007Making` /
     `Ruzic2009Making`: same RAM and cache-oblivious bounds, deterministic,
     digital, variable-length and unbounded strings.
   - "optimal performance in the RAM model" is not established for strings:
     Andersson-Thorup's O(lcp/w + sqrt(lg n / lg lg n)) is better for small
     sets, packed c-tries give O(|x|/w + lg lg n) expected, and the only
     lower bound is Omega(|x|/w + pt(n, w)).
   - "good performance in the cache-oblivious model" for a dynamic string
     structure with predecessor search was already achieved, with a
     different bound, by `Bender2006Cacheoblivious`.
   - Suggested form: "the first *dynamic* digital data structure for
     variable-length strings whose RAM and cache-oblivious bounds depend on
     the length of the strings involved and not on n", crediting Ruzic for
     the static deterministic case.
7. **Lines 382-385.** "several structures already provide optimal bounds both
   in terms of n (e.g., Andersson and Thorup achieve O(sqrt(lg n / lg lg n))
   update/query time) and in terms of the string length for fixed-length
   strings (e.g., y-fast tries)."
   Correct as far as it goes; it should now say that
   `Patrascu2014Dynamic` gives the optimal expected bound for every
   combination of n, l and w, including O(log n / log w) deterministically,
   and that the z-fast trie matches it in the regime of item 2.
8. **Lines 390-396.** The Andersson-Thorup string bound is quoted as
   "O(|x|/w + sqrt(lg n / lg lg n))"; the theorem charges the longest common
   prefix with the stored keys, in words, and holds for updates as well,
   deterministically and in the worst case. The footnote ("it would be
   possible to achieve similarly O(|x|/w + lg w)") can now point to packed
   c-tries, which make this kind of reduction explicit in expectation.
9. **Lines 407-410** (string B-trees). Correct. Add Brodal-Fagerberg and the
   cache-oblivious string B-tree, since the paper's claim is about the
   cache-oblivious model, where the string B-tree itself does not apply.
10. **Lines 1237-1244** (signature length c lg n + lg w, probability
    3/n^{c-1}). With polynomial hashing the length must be c lg n + lg L,
    L the maximum string length in bits (Section 8.2); the O(w) conclusion
    of line 1254 is unaffected.
11. **Abstract, lines 191-193** ("or with high probability using an
    appropriate dictionary"): name the dictionary (Section 8.3).
12. **Lines 412-419** (work after the conference version). The paragraph
    mentions zuffix arrays, c-trie++ and Wormhole. Missing: the survey;
    packed c-tries; Ehrhardt-Mulzer and the distance-sensitive line; Cohen
    et al.; zip-tries. The c-trie++ entry in `biblio.bib` is the arXiv
    version; DCC 2020 and Information and Computation 2022 exist.
13. Two descriptions of the z-fast trie in the literature say that it returns
    "a correct result w.h.p." (zip-tries) or answers "only with high
    probability" (packed c-tries). The paper should state prominently that
    answers are always correct and only the running time is probabilistic.

## 10. Proposed outline for the related-work section

- **Integer predecessor search, the landscape.**
  - Length reduction: van Emde Boas; x-fast and y-fast tries; Mehlhorn-Naher.
    O(lg l) time; linear space needs buckets (amortisation) or recursion.
  - Cardinality reduction: fusion trees; exponential search trees
    (deterministic, worst case, three-branch bound).
  - Optimal bounds: static (Patrascu-Thorup 2006, 2007; Beame-Fich;
    Sen-Venkatesh); dynamic in expectation (Patrascu-Thorup 2014), with the
    formula.
  - One sentence placing the z-fast trie: matches the optimum iff
    log n = l^{Omega(1)}; simple; no buckets; worst-case queries; pointer to
    the survey of Navarro and Rojas-Ledesma, which presents it this way.
  - Open: deterministic and w.h.p. dynamic bounds.
- **Origin of the idea.**
  - Binary search on prefix lengths with one hash table per length (x-fast
    tries; Waldvogel et al., from the systems report).
  - One entry per compacted-trie node: Ruzic (static, deterministic,
    dictionaries per power of two) and the static z-fast trie of the SODA
    2009 and ESA 2010 papers, independently.
  - What the dynamic structure adds: a single dictionary, handles, updates,
    jump pointers.
- **Queries other than predecessor.**
  - Existential range and prefix queries are easier than predecessor on
    integers (Alstrup-Brodal-Rauhe; Mortensen-Pagh-Patrascu); the z-fast trie
    does not exploit this.
  - Distance-sensitive search: Bose et al.; Belazzougui-Boldi-Vigna 2012;
    Ehrhardt-Mulzer (also: tango-tree alternative to jump pointers).
  - Small indices: Cohen et al.
- **Strings on the word RAM.**
  - Comparison based: O(|x|/w + lg n) (Grossi-Italiano; zip-tries).
  - Andersson-Thorup: O(lcp/w + sqrt(lg n / lg lg n)), deterministic.
  - Alphabet-dependent: Fischer-Gawrychowski; Bille et al. (static, packed).
  - Hash-based packed tries after the conference version: packed c-tries,
    c-trie++; what they support (prefix search, not predecessor) and their
    bounds against O(|x|/w + lg |x|).
  - Lower bounds: only Omega(|x|/w) plus the integer bound.
- **Strings in external memory.**
  - String B-trees; cache-oblivious string B-trees; Brodal-Fagerberg;
    Ruzic. lg |x| against log_B n.
- **Hashing.**
  - Polynomial prefix hashing (Dietzfelbinger et al.; Karp-Rabin); what is
    assumed; dictionaries with worst-case or w.h.p. guarantees.
- **Engineering** (pointer to the systems part): Dinklage et al. for
  integers; c-trie++ and Wormhole for strings; zuffix arrays.

## 11. Papers I could not access

For each: what I used instead and what should be checked in the original.

1. Bender, Farach-Colton, Kuszmaul. *Cache-oblivious string B-trees.* PODS
   2006, pp. 233-242. DOI 10.1145/1142351.1142385. Needed: exact theorems
   (search, update and range bounds; whether w.h.p., amortized or expected;
   tall-cache assumption; predecessor support). Used the authors' research
   abstract and Ruzic's description.
2. Ferragina, Grossi. *The String B-tree: a new data structure for string
   search in external memory and its applications.* J. ACM 46(2):236-280,
   1999. DOI 10.1145/301970.301973. Needed: exact bounds for prefix, range
   and predecessor queries and for updates. Used three secondary
   descriptions.
3. Ehrhardt, Mulzer. *Delta-fast tries: local searches in bounded universes
   with linear space.* WADS 2017, LNCS, pp. 361-372. DOI
   10.1007/978-3-319-62127-2_31. Needed: the theorem as published and how it
   refers to the z-fast trie. Used Ehrhardt's master's thesis (local copy)
   and the survey.
4. Fischer, Gawrychowski. *Alphabet-dependent string searching with
   wexponential search trees.* CPM 2015, LNCS, pp. 160-171. DOI
   10.1007/978-3-319-19929-0_14. Needed: the dynamic bound of the published
   version (arXiv v1 says O(m + lg^2 lg sigma / lg lg lg sigma), the survey
   says O(|p| + log log sigma)) and whether deletions are supported.
5. Belazzougui. *Predecessor search, string algorithms and data structures.*
   Encyclopedia of Algorithms, 2nd ed., Springer 2016, pp. 1605-1611. DOI
   10.1007/978-1-4939-2864-4_632. Needed: what it says about z-fast tries
   and about strings.
6. Dietzfelbinger, Gil, Matias, Pippenger. *Polynomial hash functions are
   reliable.* ICALP 1992, LNCS, pp. 235-246. DOI 10.1007/3-540-55719-9_77.
   Needed: Section 5 (incremental hashing), the exact family and collision
   bound to be cited in Section 4.3 of the paper. Used Thorup's notes.
7. Karp, Rabin. *Efficient randomized pattern-matching algorithms.* IBM J.
   Res. Dev. 31(2):249-260, 1987. DOI 10.1147/rd.312.0249. Needed: the exact
   fingerprint family and error bound.
8. Patrascu, Thorup. *Randomization does not help searching predecessors.*
   SODA 2007, pp. 555-564. Needed: the exact statement (error probability,
   space range). Only the abstract was read.
9. Beame, Fich. *Optimal bounds for the predecessor problem and related
   problems.* J. Comput. System Sci. 65(1):38-72, 2002. DOI
   10.1006/jcss.2002.1822. Needed: Theorem 3.6 and the dynamic corollary.
   Only the first page of the STOC 1999 version was read.
10. Classical papers taken from the survey and not read in the original:
    van Emde Boas (FOCS 1975, DOI 10.1109/SFCS.1975.26; IPL 1977, DOI
    10.1016/0020-0190(77)90031-X); van Emde Boas, Kaas, Zijlstra (DOI
    10.1007/BF01683268); Willard (DOI 10.1016/0020-0190(83)90075-3);
    Mehlhorn, Naher (DOI 10.1016/0020-0190(90)90022-P); Fredman, Willard
    (DOI 10.1016/0022-0000(93)90040-4); Fredman, Saks (DOI
    10.1145/73007.73040); Sen, Venkatesh (DOI 10.1016/j.jcss.2007.06.016);
    Alstrup, Brodal, Rauhe (DOI 10.1145/380752.380842).
11. Mortensen, Pagh, Patrascu. *On dynamic range reporting in one dimension.*
    STOC 2005, pp. 104-111. DOI 10.1145/1060590.1060606. The arXiv version
    cs/0502032 is available; only its abstract was read. To check: whether
    the bounds are worst case or expected, and the space.
12. Dictionaries: Arbitman, Naor, Segev, *Backyard cuckoo hashing* (FOCS
    2010, DOI 10.1109/FOCS.2010.80, arXiv 0912.5424, abstract read) and
    *De-amortized cuckoo hashing* (ICALP 2009, DOI
    10.1007/978-3-642-02927-1_11); Dietzfelbinger, Meyer auf der Heide, *A
    new universal class of hash functions and dynamic hashing in real time*
    (ICALP 1990, DOI 10.1007/BFb0032018); Bender et al., *Iceberg hashing*
    (J. ACM 70(6), 2023, DOI 10.1145/3625817, abstract read); Pagh, Pagh,
    Ruzic, *Linear probing with constant independence* (SIAM J. Comput.
    39(3), 2009, DOI 10.1137/070702278); Patrascu, Thorup, *The power of
    simple tabulation hashing* (J. ACM 59(3), 2012, DOI
    10.1145/2220357.2220361). Needed: exact guarantees and the independence
    they require, to replace the full-randomness assumption.
13. Thorup, *On AC0 implementations of fusion trees and atomic heaps* (SODA
    2003, pp. 699-707) and Andersson, Miltersen, Riis, Thorup, *Static
    dictionaries on AC0 RAMs: query time Theta(sqrt(log n / log log n)) is
    necessary and sufficient* (FOCS 1996, DOI 10.1109/SFCS.1996.548503).
    Needed for the question on multiplications at line 315.
14. Published versions of papers read on arXiv, in case they differ: Takagi
    et al. (IWOCA 2016, DOI 10.1007/978-3-319-44543-4_17; IEICE 2017, DOI
    10.1587/transfun.E100.A.1785); Tsuruta et al. (DCC 2020, DOI
    10.1109/DCC47342.2020.00032; Inf. Comput. 2022, DOI
    10.1016/j.ic.2021.104794); Bose et al. (Comput. Geom. 2013, DOI
    10.1016/j.comgeo.2012.01.002); Jansson, Sadakane, Sung (Algorithmica
    71(4), DOI 10.1007/s00453-013-9836-6, not read at all).
15. Rabin, *Fingerprinting by random polynomials*, Technical Report
    TR-15-81, Harvard, 1981 (no registry record found; not in `theory.bib`),
    and Broder, *Some applications of Rabin's fingerprinting method*
    (Sequences II, 1993, DOI 10.1007/978-1-4613-9323-8_11). Needed if the
    paper wants to justify CRC-like signatures.
