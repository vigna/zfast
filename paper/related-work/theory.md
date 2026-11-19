# Related work, theory: predecessor search and dynamic dictionaries/tries on integers and strings

PRELIMINARY VERSION (first pass, 2026-11-19). Written from the material read so
far; sections marked *[pending]* are being extended. Everything stated here
without a marker was read in the primary source named.

Sources read in full or in the relevant parts so far:

- Navarro and Rojas-Ledesma, *Predecessor Search*, ACM Computing Surveys 53(5),
  article 105, 2020, DOI 10.1145/3409371 (local copy
  `/Users/vigna/pub/zfast/related/3409371.pdf`).
- Patrascu and Thorup, *Dynamic Integer Sets with Optimal Rank, Select, and
  Predecessor Search*, FOCS 2014, pp. 166-175; read in the arXiv version
  1408.3045v2.
- Andersson and Thorup, *Dynamic ordered sets with exponential search trees*,
  J. ACM 54(3), article 13, 2007; read in the arXiv version cs/0210006v2.

## 1. Patrascu-Thorup 2014 (the result the paper must be compared with)

**What it achieves** (read in arXiv 1408.3045v2, abstract, Sections 1, 4, 5, 6).

1. *Dynamic fusion tree.* A dynamic set of n integers of w bits on a w-bit word
   RAM (with multiplication), w >= log n, in O(n) words, with insert, delete,
   predecessor, successor, rank and select in O(log n / log w) time each. The
   paper calls this "a deterministic linear space data structure"; the bound is
   stated per operation, with background rebuilding of nodes de-amortised as in
   Andersson-Thorup. Sets of size w^{O(1)} are handled in constant time per
   operation (the *dynamic fusion node*, for up to w^{1/4} keys).
2. *Optimality of that bound.* Optimal for dynamic rank and select (cell-probe
   lower bound of Fredman and Saks, STOC 1989), and optimal for dynamic
   predecessor whenever log n / log w = O(log w / log log w) (lower bound of
   Beame and Fich, extended to randomisation by Sen and Venkatesh).
3. *Complete randomised picture for predecessor only* (Section 6, a sketch of
   two pages relying on Section 5 of the full version of their STOC 2006 paper).
   For keys of l <= w bits the optimal *expected maximal operation time* (the
   maximum of query and update time) is

       Theta( max{ 1, min{ A, B, C } } ),
       A = log n / log w
       B = log(l / log w) / log( log(l / log w) / log(log n / log w) )
       C = log( log(2^l - n) / log w )

   in linear space. A is the dynamic fusion tree (deterministic). C is "van
   Emde Boas' data structure with some simple tuning", including "the space
   saving Y-fast tries of Willard, but using our dynamic fusion nodes in the
   bottom trees"; B is a dynamisation of the small-space Beame-Fich-style
   structure of the STOC 2006 paper. B and C use "randomized hash tables with
   constant expected update and query time", so they are bounds in expectation.
   The lower bound is in the cell-probe model, holds for any space as long as
   updates are polylogarithmic, and holds for randomised structures.

**Model**: word RAM with multiplication (the paper notes that multiplication is
necessary for constant-time fusion nodes with standard instructions). Keys are
fixed-length integers that fit in one word. No strings, no variable length.

**Does it cite z-fast tries?** No (the only mention of an author of this paper is
the acknowledgement to Belazzougui "for pointing out a bug in one of the bounds
stated for predecessor search in an earlier version").

**Comparison with the z-fast trie on integers** (inference from the formula
above; the case analysis for l = w is also given, for the static bound, in
Section 3.4.1 of the Navarro-Rojas-Ledesma survey). Take l = w, so that the
z-fast trie runs in O(log w).

- C = Theta(log w) unless the complement of the set is tiny (2^l - n =
  2^{w^{o(1)}}).
- B = Theta(log w) if and only if log n = w^{Omega(1)}: writing log n =
  w^eps log w, B is about log w / log(1/eps).
- A >= log w as soon as log n >= log^2 w.

Hence **O(log w) is optimal (in the Patrascu-Thorup sense: expected, maximum of
query and update, any polynomial space) exactly when log n = w^{Omega(1)}**, i.e.
when the word length is polynomial in log n, and the set is not co-tiny. This
includes the usual regime w = Theta(log n) with universe n^c, c > 1 (where the
optimality already follows from the static bound of 2006/2007), and the
practical one (w = 64, n between 2^20 and 2^32).

**The z-fast trie is not optimal** in these ranges:

- *Small sets, large words*: log n = w^{o(1)}. For n = w^{O(1)} Patrascu-Thorup
  need O(1), the z-fast trie Theta(log w). For log n = polylog(w) the optimum is
  Theta(min{log n / log w, log w / log log w}) = o(log w).
- *Co-tiny sets* (branch C): n so close to 2^l that log(2^l - n) = w^{o(1)}.
  Statically the corresponding branch is log((l - log n) / log w), which is
  o(log l) for dense sets (l - log n = l^{o(1)}).
- *Very short keys*: for l = O(log w) the optimum is O(1) (the set has size
  w^{O(1)}), against O(log l) = O(log log w).

What Patrascu-Thorup do *not* give and the z-fast trie does: variable-length
and long strings; bounds depending on the length of the query and of its
neighbours rather than on w; worst-case (rather than expected) query time once
the dictionary has worst-case lookups; no bucketing in the branch that matters
(their O(log w)-type branch is a y-fast trie with fusion nodes in the buckets);
explicit, small space constants.

## 2. What the Navarro-Rojas-Ledesma survey says about z-fast tries

Read in full. The z-fast trie has its own paragraph and figure in Section 3.1.2
("Reducing the space of van Emde Boas trees"), after x-fast tries, y-fast tries
and Mehlhorn-Naher: "*Z-fast tries: linear space, and fast updates (in
expectation).* In 2010, Belazzougui et al. [SPIRE 2010] introduced the dynamic
z-fast trie, a version of Willard's x-fast tries that achieves linear space and
O(log log u) query and update time with high probability. The first version of
this data structure was actually introduced one year earlier [SODA 2009], but
it was static and could only find the longest common prefix between q and its
predecessor." It lists the four changes with respect to the x-fast trie
(compacted trie; one hash table; handles and fat binary search; jump pointers,
"similar in essence to the canonical subtrees of van Emde Boas", reaching the
extreme leaves in O(log log u) rather than O(1)). Points to note:

- **Ruzic.** "A very similar idea for searching longest common prefixes in a
  trie was introduced independently by Ruzic [the survey's ref. 92], although
  there, the keys associated with a node are stored in different hash tables
  depending on their size instead of storing them all in the same hash, and
  the data structure is static." The paper does not cite Ruzic. *[pending:
  exact reference and statement]*
- The survey attributes "queries in O(log log u) worst-case time, and updates
  in O(log log u) expected time" to its reference [18], which is the ESA 2010
  paper (static); this looks like a slip for the SPIRE 2010 paper. It repeats
  the remark that "improvements in dynamic hashing immediately translate into
  better time bounds for the z-fast trie".
- Section 5.1 (distance-sensitive predecessor search): Bose et al.,
  O(log log Delta) with O(n log log log u) expected space; Belazzougui, Boldi
  and Vigna 2012, O(log log Delta) queries in O(n) space "using a z-fast trie
  instead of an x-fast trie", but updates not distance-sensitive; Ehrhardt and
  Mulzer, "another variant of the z-fast trie" with O(log log Delta) queries and
  expected updates in O(n) space.
- Section 5.5 (prefix search): "The z-fast trie, for instance, was introduced
  in this context of prefix and predecessor search for strings ... of variable
  length within O(w), z-fast tries can answer prefix queries in
  O(log max(|p|, |p^+|, |p^-|)) time", followed by Fischer-Gawrychowski
  (O(|p| + log log sigma) dynamic, linear space).
- Sections 2.2, 3.4.2, 4.2.2, 6: the dynamic problem is settled by
  Patrascu-Thorup 2014 only in expectation; "the optimal dynamic data structure
  of Patrascu and Thorup heavily relies on randomization. It is even open
  whether the running time of lower bounds ... can be matched by data
  structures with upper bounds that apply with high probability, instead of in
  expectation", and whether updates can be much faster than queries.

## 3. Andersson-Thorup: exponential search trees and string searching

Read in arXiv cs/0210006v2 (the J. ACM 2007 paper; it merges FOCS 1996 and STOC
2000 and includes the SODA 2001 paper "Dynamic string searching").

- Corollary 4: deterministic, linear space, worst-case time per insert, delete
  and search

      O( min{ sqrt(log n / log log n),
              log log n * log log U / log log log U,
              log log n + log n / log W } ),

  W the word length, U < 2^W a bound on the keys.
- Theorem 6 (string searching): "if the longest common prefix between a key x
  and the stored keys has l words, we can insert, delete, and search x in
  O(l + sqrt(log n / log log n)) time ... In addition to the stored keys
  themselves, our space bound is O(n)." Deterministic, worst case. Strings are
  sequences of one-word characters, made prefix-free by a terminator. The
  structure is a compacted trie over one-word characters with, in each node, a
  dictionary for the heavy children and an exponential search tree for the
  light ones; the paper presents it as "a general reduction from string
  searching to 1-word searching", and claims optimality by the Beame-Fich lower
  bound.
- Relation to the z-fast trie: incomparable. Andersson-Thorup is better when
  sqrt(log n / log log n) = o(log |x|) and is deterministic; it charges the lcp
  rather than |x| *[this needs the query to be read only up to the lcp]*. The
  z-fast trie is better for short queries in large sets and in the
  cache-oblivious model (the paper's existing argument, lines 390-405).
  Note that the paper says the bound is "O(|x|/w + sqrt(lg n / lg lg n))";
  the theorem is stated with the lcp in words.

## 4. Claims in the paper that need revision *[pending: to be completed]*

- Lines 244-247: "The static version of this problem has been completely
  solved, with matching upper and lower bounds [PaTRDNHSP]." The upper bound
  and the deterministic lower bound are in the STOC 2006 paper, which is not
  cited; SODA 2007 is the randomised lower bound. Add that the dynamic problem
  is settled in expectation by Patrascu-Thorup 2014 for l <= w.
- Lines 264-266: "the first simple optimal digital data structure that is
  neither recursive nor needs to use bucketing to achieve optimal time and
  linear space." "Optimal" must be qualified (Section 1 above).
- Lines 373-375: "the first paper to propose a digital data structure for
  variable length strings with optimal performance in the RAM model and good
  performance in the cache-oblivious model."
- Lines 382-385: "several structures already provide optimal bounds both in
  terms of n (e.g., Andersson and Thorup achieve O(sqrt(lg n / lg lg n))
  update/query time)".

## 5. Proposed outline *[pending]*

## 6. Papers I could not access *[pending]*
