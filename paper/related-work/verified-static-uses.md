# Verification against primary sources: static dictionaries and uses of z-fast tries

Date: 2026-11-20. Sources: the PDFs in `/Users/vigna/git/zfast/related/`.
Paper checked: `/Users/vigna/git/zfast/paper/dynzfast.tex` as of 2026-11-20
00:13 (2596 lines); all line numbers refer to that state.

Conventions. **[P]** = printed in the PDF (quotes are verbatim, formulas
transliterated in ASCII); **[I]** = my inference; **[W]** = obtained from the
web (Crossref or arXiv metadata), not from the PDF. Each PDF was converted with
`pdftotext -layout` and read in full as text, except the encyclopedia volume
(only the two entries on predecessor search) and the papers 6-8, for which the
whole text was searched and the relevant sections read; first pages, tables and
figures were also inspected as page images.

Summary of the findings:

| # | PDF | What it is | z-fast trie: used / cited / measured |
|---|---|---|---|
| 1 | `PredecessorSearch.pdf` | Letter "P" of the *Encyclopedia of Algorithms* (146 pages); contains Belazzougui's entry (pp. 1605-1611) and Patrascu's entry (pp. 1601-1605). **Not** the Navarro-Rojas-Ledesma survey | neither (never mentioned) |
| 2 | `WeakPrefixSearch.pdf` | Ferragina, TCS 483 (2013) 75-84 | **replaces** it (Patricia tries + Karp-Rabin); cites ESA 2010 only |
| 3 | `CompressedCacheObliviousStringBTrees.pdf` | Ferragina, Venturini, ACM TALG 12(4), Article 52, 2016 | **replaces** it (Patricia tries + injective Karp-Rabin); cites ESA 2010 only |
| 4 | `LZ77.pdf` | Gagie et al., LATIN 2014, LNCS 8392, 731-742 | **uses** fat binary search on Patricia trees with Karp-Rabin fingerprints; cites SODA 2009 only; the words "z-fast trie" never appear |
| 5 | `MassiveStringDictionaries.pdf` | Ferragina, Rotundo, Vinciguerra, Information Systems 128 (2025) 102490 (the journal version) | neither used nor cited nor measured |
| 6 | `CompressedDoubleArrayTries.pdf` | Kanda, Morita, Fuketa: a **preprint** ("Under consideration for publication in Knowledge and Information Systems") | neither |
| 7 | `LempelZivTries.pdf` | Arroyuelo et al., ACM JEA 26(1), Article 1.14, 2021 | **cites** SPIRE 2010 once, in passing; does not use or measure |
| 8 | `SpaceEfficient.pdf` | Hsu, Ottaviano, WWW 2013 | neither; no weak prefix search |

None of the eight papers is currently cited in `dynzfast.tex`, and none of
their keys is in `biblio.bib` (checked with `grep`).

---

## 1. `PredecessorSearch.pdf`

### What the file is

**[P]** It is not a single paper: it is a 146-page extract of M.-Y. Kao (ed.),
*Encyclopedia of Algorithms*, Springer 2016 (footer of the first page: "(c)
Springer Science+Business Media New York 2016, M.-Y. Kao (ed.), Encyclopedia of
Algorithms, DOI 10.1007/978-1-4939-2864-4"), containing the entries under the
letter P, from "P2P" (p. 1491) onwards. It contains two entries on predecessor
search:

- "Predecessor Search", Mihai Patrascu, pp. 1601-1605 (PDF pages 111-115);
- "Predecessor Search, String Algorithms and Data Structures", Djamal
  Belazzougui (University of Helsinki), pp. 1605-1611 (PDF pages 115-121).

It is **not** the survey of Navarro and Rojas-Ledesma. The chapter DOI
10.1007/978-1-4939-2864-4_632 given in `TO-DOWNLOAD.md` is not printed on the
pages I read (only the DOI of the book is).

### What Belazzougui's entry says about z-fast tries, Ruzic, strings

**[P]** Nothing. The text of the whole 146-page file contains no occurrence of
"z-fast", "x-fast", "y-fast", "fat binary search", "Ruzic" or "Vigna"/"Boldi".
Despite its title, the entry is entirely about integers of l bits: the only
strings are the binary labels of trie nodes ("A node of the trie at depth d ...
will be labeled by a binary string of length d"). "String Algorithms and Data
Structures" appears to be a qualifier in the title, not a topic of the entry
**[I]**. Contents: problem definition and models (comparison, external memory,
RAM, AC0 RAM, cell probe); tries and compacted tries; van Emde Boas; fusion
trees; Beame-Fich; exponential search trees; deterministic dynamic bounds;
optimal static bounds; optimal randomized dynamic bounds; applications; open
problems. The closest it gets to our topic:

- p. 1607: "The Van Emde Boas tree [17-19] is a compacted trie representation
  supporting predecessor search by doing a binary search on the trie levels.
  ... Later, Willard [20] showed how to use hashing to reduce the space usage
  to O(n) while keeping the search time bounded by O(log l)." (No name is given
  to Willard's structures.)
- p. 1610, Cross-References: "Monotone Minimal Perfect Hash Functions" (the
  only link, by title, to the line of work in which the z-fast trie was
  introduced).
- Recommended reading (20 items): no paper of Belazzougui, Boldi, Pagh, Vigna,
  and no paper of Ruzic.

Patrascu's entry (pp. 1601-1605) does not mention them either.

### What the two entries say that corroborates the text of `dynzfast.tex`

- **[P]** p. 1609, "Optimal Randomized Dynamic Bounds": "When allowing
  randomization, optimal bounds can be achieved [15]. Again, the bounds are for
  the maximum of query and update times", followed by
  Theta(1 + min{ log n/log w, log(log(2^l - n)/log w), log(l/log w) /
  log(log(l/log w) / log(log n/log w)) }). This is the formula displayed at
  lines 436-440 (same three terms, in a different order), and "maximum between
  query and update time" is what lines 434-435 say. **Correct.**
- **[P]** p. 1609: "They later show that their lower bound holds even if
  randomization is allowed [14]" (static case): agrees with line 429.
- **[P]** p. 1610, "Open Problems": "The deterministic complexity of the
  dynamic predecessor search is still open. Another open problem is whether
  updates can be supported faster than searches when the search time is optimal
  or near optimal". Patrascu's entry, p. 1604: "For the dynamic problem, can
  the update times be made deterministic? In particular, can van Emde Boas
  search be implemented with fast deterministic updates?". Both agree with the
  second half of line 450 (see below).

### The statements attributed to `Navarro2020Predecessor`

Since the downloaded file is not the survey, they cannot be checked against it.
A copy of the survey, however, is on this machine at
`/Users/vigna/pub/zfast/related/3409371.pdf` (first page: "Predecessor Search",
Gonzalo Navarro and Javiel Rojas-Ledesma; running head "ACM Computing Surveys,
Vol. 53, No. 5, Article 105. Publication date: September 2020"); I checked the
four sentences against its text.

| Line | Sentence of `dynzfast.tex` | Survey **[P]** | Verdict |
|---|---|---|---|
| 450 | "whether the dynamic bounds can be attained with high probability, or deterministically, is open~\cite{Navarro2020Predecessor}." | Section 6, "Dynamic predecessor bounds": "The most fundamental of those unsolved questions is the deterministic complexity of the dynamic predecessor problem: the optimal dynamic data structure of Patrascu and Thorup [83] heavily relies on randomization. It is even open whether the running time of lower bounds of Patrascu and Thorup [83] can be matched by data structures with upper bounds that apply with high probability, instead of in expectation." | correct |
| 452-454 | "The survey by Navarro and Rojas-Ledesma ... presents the z-fast trie as the version of the x-fast trie that attains linear space without buckets, using a compacted trie, a single hash table, and one prefix per node." | Section 3.1.2, paragraph "Z-fast tries: linear space, and fast updates (in expectation)": "Belazzougui et al. [19] introduced the dynamic z-fast trie, a version of Willard's x-fast tries [102] that achieves linear space and O(log log u) query and update time with high probability"; "the elements are stored in a compact trie, instead of a complete trie"; "Only one hash table is used, instead of one per level of the tree"; "The key associated with each node alpha is the prefix of length f of l(alpha), where f is the 2-fattest number in the interval ...". The preceding paragraph says of Mehlhorn-Naher: "improved to linear by using the same bucketing trick of y-fast tries". | correct ("without buckets" is a fair paraphrase of the contrast, not a quotation) |
| 469 | "the survey~\cite{Navarro2020Predecessor} describes it as a variant of the z-fast trie." | Section 5.1: "Ehrhardt and Mulzer [45] remedied this by presenting another variant of the z-fast trie, which, using O(n) space, supports queries in O(log log Delta) time, and updates in O(log log Delta) expected worst-case time." | correct |
| 497 | "The static z-fast trie~\cite{BBPMMPH,Belazzougui2010Fast}, which was developed independently~\cite{Navarro2020Predecessor}, uses instead a single dictionary indexed by handles." | Section 3.1.2: "A very similar idea for searching longest common prefixes in a trie was introduced independently by Ruzic [92], although there, the keys associated with a node are stored in different hash tables depending on their size instead of storing them all in the same hash, and the data structure is static." | correct |

No replacement is needed for any of the four.

### Use in the paper

The encyclopedia entry adds nothing on z-fast tries, strings or Ruzic: it need
not be cited, and the item can be struck from section 3 of `TO-DOWNLOAD.md`.
If a second reference for the open problem at line 450 is wanted, the entry
supports only the "deterministically" half.

**BibTeX**: the entry `Belazzougui2016Predecessor` in `theory.bib` is correct
as to author, title and pages (1605-1611, verified); it lacks the editor. See
the last section.

---

## 2. `WeakPrefixSearch.pdf`

**Identification [P].** Paolo Ferragina, "On the weak prefix-search problem",
Theoretical Computer Science 483 (2013) 75-84, doi:10.1016/j.tcs.2012.06.011,
"(c) 2012 Elsevier B.V.". The acknowledgments ("the anonymous referees of CPM
'11") confirm that a preliminary version was at CPM 2011. The entry
`Ferragina2013Weak` in `static-and-uses.bib` is correct (drop "Not read").

**Sentences of `dynzfast.tex`.** None cites it or makes a claim about it. Weak
prefix search appears only at line 588, with `Belazzougui2010Fast`.

**What it says [P].**

- Problem (p. 75, "Problem 1 (Belazzougui et al. 2010)"): a prefix-free set S
  of n binary strings of average length l, sorted; given P[1,p], return the
  range of strings prefixed by P; "In the case that P does not prefix any
  string of S, the returned range can be any pair."
- How it reports the ESA 2010 result (p. 75): "The weak-feature allowed the
  authors of [2] to reduce the space occupancy from O(n l) to O(n log l) bits,
  and indeed they proved that this is a space lower-bound. Their solution takes
  O(p/w + log_2 p) memory transfers for searching in the RAM model with a
  memory word of w bits. ... In this context the Belazzougui's result can be
  rephrased as O(p/B + log_2 p) I/Os, by setting w = B, still within the
  optimal O(n log l) bits of space occupancy. Noteworthy the authors of [2]
  showed that this I/O-bound holds also in the more powerful Cache-Oblivious
  model".
- Comparison (p. 76): classic prefix search "uses O(n l) bits of space and
  O(p/B + log_B n) I/Os. This is always worse in terms of space occupancy than
  what is obtained for the weak prefix-search, but it is faster whenever log_B
  n < log_2 p. This latter inequality is realistic in practice even for
  moderate pattern's lengths, given that it corresponds to n < p^(log_2 B)
  ~ p^15."
- Result: Theorem 3 (external memory) and Theorem 4 (cache-oblivious): "The
  weak-prefix search problem can be solved in the cache-oblivious model by
  using O(n log l) bits of space and O(p/B + log_B n) I/Os. The correctness
  holds with high probability." Theorem 2 (RAM): O(p) time, O(n log l) bits.
  Theorem 5: O(p/B + log_B min{n, 1/P(P)}) I/Os when a distribution on the
  queries is known. The cache-oblivious result requires p <= M (p. 80).
- Technique (Sections 3-4): a two-level scheme. S is split into groups of log n
  consecutive strings; a Patricia trie PT' on the first and last string of each
  group stores in each node a Karp-Rabin fingerprint of O(log n) bits of the
  string spelled by the node, and one small Patricia trie per edge of PT'
  indexes the two groups at the ends of the range below the edge. The search is
  a blind search on PT' checked with fingerprints, then a blind search on a
  small trie; the tries are laid out on disk by the skeleton trees of Ko and
  Aluru (external memory) or by the centroid decomposition of Bender et al.
  (cache-oblivious).
- p. 76: "Our solution has the nice feature of being algorithmically simpler
  than the one proposed in [2], and thus worth to be implemented."
- p. 83: the algorithms become deterministic if the sampled strings are stored;
  "Compared to the String B-tree [9], we are performing the same number of I/Os
  but with a smaller storage cost of o(n log n) bits in addition to the indexed
  strings".

**Bounds against ESA 2010.** Same space, O(n log l) bits (optimal by the lower
bound of ESA 2010). I/Os: O(p/B + log_B n) instead of O(p/B + log p), i.e., the
logarithm of the length is replaced by log_B of the number of strings; better
when n < p^(log_2 B). The price: the answer is Monte Carlo ("its answer is
correct with high probability") also when P is a prefix of some string, because
a pattern prefix and a node of PT' may have the same fingerprint (this is
spelled out in the later paper of Ferragina and Venturini, see below). **[I]**
The comparison is only with the O(n log l)-bit, O(log p)-probe structure of
ESA 2010; the variant of ESA 2010 with O(|p|/w) time and more space (Theorem
6 there, according to `static-and-uses.md`) is not discussed.

**Does it use a z-fast trie?** No: it **replaces** it. Neither "z-fast trie"
nor "fat binary search" appears in the text. The only z-fast paper cited is
ESA 2010 (reference [2]), as the source of the problem, of the lower bound and
of the bound to beat. It measures nothing (no experiments).

**Relevance.** (a) It is the natural reference for the trade-off stated at
line 549 ("we replace lg_B n with the logarithm of the length of the strings
involved"): Ferragina's remark quantifies when lg_B n is the smaller term.
(b) Its layout (sampled strings in a Patricia trie with fingerprints, groups of
consecutive strings below) is the theoretical ancestor of paper 5.

**Proposal.** Not an item of "Uses of z-fast tries". One sentence in "Strings
in external memory" (see edit E2).

---

## 3. `CompressedCacheObliviousStringBTrees.pdf`

**Identification [P].** Paolo Ferragina and Rossano Venturini, "Compressed
Cache-Oblivious String B-Tree", ACM Transactions on Algorithms 12(4), Article
52 (August 2016), 17 pages, DOI 10.1145/2903141. Footnote: "A preliminary
version of this paper was published in the Proceedings of the 21st Annual
European Symposium on Algorithms (ESA) 2013". The entry
`Ferragina2016Compressed` is correct; `pages = {52:1--52:17}` is preferable to
`1--17`.

**Sentences of `dynzfast.tex`.** None cites it. It bears on lines 543-549
(Brodal-Fagerberg and Bender et al.), which it describes consistently with our
text: Brodal and Fagerberg give "a static trie-like data structure in the
cache-oblivious model" with "the same I/O- and space-bounds of the string
B-tree"; the cache-oblivious string B-tree (COSB) of Bender et al. is
"(randomized)", uses "(1 + eps)FC(S) + O(K log N) bits" and "supports searches
in O(log_B K + (1 + 1/eps)(Scan(P) + Scan(succ(P)))) I/Os ... The solution is
randomized, so the I/O bounds hold with high probability" (p. 52:2). This
agrees with lines 545-547 (membership O(|x|/B + lg_B n), predecessor and
successor O((|x| + |x^+-|)/B + lg_B n), w.h.p.). No correction.

**What it says [P].** Notation: K strings, total length N bits, Scan(P) =
Theta(1 + |P|/(B log N)), B in words; Trie(S) = sum of the edge-label lengths
of the compacted trie (t nodes); LB(S) = Trie(S) + log binom(Trie(S), t-1).

- Table I (p. 52:4), weak prefix search: O(log_2 |P| + Scan(P)) I/Os and O(K
  log(N/K)) bits [Belazzougui et al. 2010]; O(log_B N + Scan(P)) I/Os, same
  space, correct w.h.p. [Ferragina 2013]; O(log_B K + Scan(P)) I/Os and log
  binom(Trie(S), t-1) + O(K) bits (Theorem 4.1).
- Theorem 4.1: "there exists a deterministic data structure requiring log
  binom(Trie(S), t-1) + O(K) bits of space that solves the Weak-Prefix Search
  Problem for any pattern P with O(log_B K + Scan(P)) I/Os." "The lower bound
  in Belazzougui et al. [2010] states that Omega(K log(N/K)) bits are necessary
  regardless of the query time"; "our refined estimate of the space usage ...
  may go below the general lower bound in Belazzougui et al. [2010], which
  depends only on N and K. Our improvement may be up to a factor Theta(log K)".
  "In short, our data structure is deterministic, smaller, and faster than
  previously known solutions."
- Theorems 4.3 and 4.4: full prefix search and longest-prefix search (the range
  of strings sharing the longest common prefix with P) in O(log_B K + (1 +
  1/eps) Scan(P)) I/Os with (1 + eps)LB(S) + O(K) bits, for any constant eps >
  0, deterministically, in the cache-oblivious model.
- Theorem 3.1: a storage scheme of (1 + eps)LB(S) + O(K) bits (locality-
  preserving *rear* coding of the strings spelled by all trie nodes) from which
  any prefix of any string can be retrieved in an optimal number of I/Os.
- Relation to cache-oblivious string B-trees (p. 52:2-3): the term O((1 +
  1/eps)Scan(succ(P))) of COSB "may degenerate into Theta((1 + 1/eps)
  sqrt(N)/B) for some sets of strings"; the structure of Ferragina et al.
  (PODS 2008) "still has the dependency on Scan(succ(P)), which might be not
  bounded in terms of |P|". The new results remove this dependency: "we improve
  on both Bender et al. [2006] and Ferragina et al. [2008] via a space-I/O
  optimal deterministic solution". The structure is static: "We foresee two
  main open questions. The first one concerns the design of a dynamic solution
  for these problems in compressed space" (p. 52:16).
- Technique for weak prefix search (Section 4): "we follow the solution in
  Ferragina [2013] by using two-level indexing"; a Patricia trie on the first
  and last string of groups of log N strings, laid out by centroid
  decomposition, with "a (deterministic) injective instance of Karp-Rabin that
  maps any prefix of any string in S into a distinct value"; "the algorithm
  proposed in Ferragina [2013] identifies the edge e only with high
  probability. The reason is that a prefix of P and a prefix of a string in S
  may have the same fingerprint even if they are different."

**Discrepancy between papers 2 and 3 [P].** Paper 3 reports the bound of paper
2 as O(log_B N + Scan(P)), with N the total length; paper 2 states O(p/B +
log_B n), with n the number of strings.

**Does it use a z-fast trie?** No: it **replaces** it, like paper 2. No
occurrence of "z-fast" or "fat binary search"; ESA 2010 is cited for the
definition of weak prefix search, for the bound O(log_2 |P| + Scan(P)) and for
the space lower bound. No experiments.

**Consequence for the list of uses.** `static-and-uses.md` (section 8.1, group
C) lists this paper among the works using weak prefix search "(problem
formulation and lower bound)". It must **not** enter the item on weak prefix
search at lines 588-590: it solves the same problem without z-fast tries.
(`dynzfast.tex` does not cite it there: nothing to remove.)

**Relevance.** Its bounds for longest-prefix search are of the form of string
B-trees, O(|x|/B + lg_B n), with no dependency on the neighbours of the query;
ours are O(|x|/B + lg max{|x|, |x^+|, |x^-|}) (line 512). The remark on
Scan(succ(P)) is the same phenomenon as our dependency on |x^+-|, which for us
is under a logarithm.

**Proposal.** One or two sentences in "Strings in external memory" (edit E2).

---

## 4. `LZ77.pdf`

**Identification [P].** Travis Gagie, Pawel Gawrychowski, Juha Karkkainen,
Yakov Nekrich, Simon J. Puglisi, "LZ77-Based Self-indexing with Faster Pattern
Matching", in A. Pardo and A. Viola (Eds.), LATIN 2014, LNCS 8392, pp. 731-742,
Springer 2014. The entry `Gagie2014LZ77` in `static-and-uses.bib` is correct
but lacks volume and editors (see the last section).

**There is no lemma.** `TO-DOWNLOAD.md` asks for "the original statement of the
z-fast trie plus Karp-Rabin lemma". **[P]** The paper contains one lemma (Lemma
1, on balanced SLPs, from the authors' previous paper) and four theorems, none
about tries. The combination is described in prose, in Section 4 (p. 737):

> "Next, we augment the Patricia trees to support the fat binary search method
> of Belazzougui, Boldi, Pagh, and Vigna [4]. We also augment the SLP to
> support fingerprinting substrings in O(log n) time, as described in
> Subsection 2.4. Then, given P, we first compute the Karp-Rabin fingerprints
> phi(P[1]), ..., phi(P[1..m]) in O(m) time. After this, for 1 <= i <= m, in
> O(log m) time we can find in the Patricia tree T1 for the reversed phrases a
> node w1 such that, with high probability, either the reversed phrases
> prefixed by (P[1..i])^rev are precisely those identified by w1's leaf
> descendants, or there are no such reversed phrases. Similarly, in O(log m)
> time we can find in the Patricia tree T2 for the suffixes starting at phrase
> boundaries, a node w1 [sic] such that, with high probability, either the
> suffixes that start at phrase boundaries and are prefixed by P[i+1..m] are
> precisely those identified by w2's leaf descendants, or there are no such
> suffixes."

Section 4.3 (p. 739) on failures: "if two fingerprints are the same in one of
our two Patricia trees, then a fat binary search can fail to return the correct
node and we may miss some occurrences"; this is prevented "by testing
fingerprinting functions until we find one such that all the fingerprints in
each Patricia tree are distinct"; false positives are removed by "bookmarked
fingerprinting and a single bookmarked extraction to check all the results of
the fat binary searches in O(m log m) time". Result (Theorems 3 and 4): O(z
log(n/z)) space, pattern matching in O(m log m + occ log log n) time; "Our
construction is randomized and takes O(n log n) time with high probability, but
the resulting index does not use randomization and cannot fail."

**Which z-fast paper it cites [P].** Only reference [4]: "Belazzougui, D.,
Boldi, P., Pagh, R., Vigna, S.: Monotone minimal perfect hashing: searching a
sorted table with O(1) accesses. In: Proc. SODA, pp. 785-794 (2009)". Neither
ESA 2010 nor SPIRE 2010 is in the reference list (30 items). The expression
"z-fast trie" does not occur anywhere: the paper speaks of "fat binary search"
on "augmented Patricia trees".

**Does it use a z-fast trie?** Yes, in substance: the static structure (a
dictionary on handles probed by fat binary search), with Karp-Rabin
fingerprints as signatures and a final verification, on two Patricia trees
(reversed phrases, suffixes at phrase boundaries), to find in O(log m) time the
locus of each of the m splits of the pattern. This is exactly the first item of
the list at lines 581-584, and the description at lines 578-579 ("often with
signatures given by Karp--Rabin fingerprints and a final verification") fits it
exactly. No measurements (theory paper).

**A problem with the citation at line 583.** The first item cites
`Gagie2012Faster` (LATA 2012) and not this paper. But:

- **[P]** p. 732: "In a previous paper [13] we combined their index with a
  bookmarked straight-line program for S to obtain a self-index that takes O(z
  log(n/z)) space and supports extraction in O(l + log n) time and pattern
  matching in O(m^2 + (m + occ) log log n) time. In Section 4 we use Karp-Rabin
  fingerprinting, fat binary search and bookmarked fingerprinting to reduce
  pattern-matching time to O(m log m + occ log log n), which is the
  contribution of this paper." Here [13] is the LATA 2012 paper.
- **[P]** p. 734: "In an unpublished extension [14] of our previous paper we
  observed that ... we can compute the fingerprint of any substring S[i..j] of
  S in O(log n) time", where [14] is "A faster grammar-based self-index.
  Technical Report 1109.3954v6, arxiv.org (2012)".
- **[W]** arXiv 1109.3954: the abstract of v2 (28 October 2011) gives only
  "O(m^2 + (m + occ) log log n) time"; the abstract of v6 (26 September 2012,
  comment "journal version of LATA '12 paper") adds "If the straight-line
  program is balanced and we accept a small probability of building a faulty
  index, then we can reduce the O(m^2) term to O(m log m)."
- `static-and-uses.md` (line 515) records that `Gagie2012Faster` was read as
  "arXiv 1109.3954", i.e., in its last version.

**[I]** Hence the LATA 2012 proceedings paper, which is what the entry
`Gagie2012Faster` of `biblio.bib` describes (LNCS, pp. 240-251), most likely
contains no fat binary search: the z-fast machinery entered in the later arXiv
versions (v6 for certain; v3-v5 not checked) and was published in LATIN 2014.
I have not read the LATA proceedings paper itself, so this remains an inference
from the two sources above. **Verdict on line 583: imprecise.** The safe
citation for the first use in this line of work is `Gagie2014LZ77`, possibly
together with the arXiv report (edit E1).

**Proposal.** Edit E1; no new item in the list.

---

## 5. `MassiveStringDictionaries.pdf`

**Identification [P].** Paolo Ferragina, Mariagiovanna Rotundo, Giorgio
Vinciguerra, "Two-level massive string dictionaries", Information Systems 128
(2025) 102490, doi 10.1016/j.is.2024.102490, open access (CC BY), "Available
online 8 November 2024". It is the **journal version**: "This is an extended
version of: ... Engineering a Textbook Approach to Index Massive String
Dictionaries. SPIRE 2023. DOI:10.1007/978-3-031-43980-3_16." New with respect
to SPIRE (p. 2): the comparison of compressors for the storage level, and the
edge cache (Sections 3.3 and 4.4). The entries `Ferragina2025Two` and
`Ferragina2023Engineering` are correct.

**Sentences of `dynzfast.tex`.** None cites it. It bears on lines 601-602 ("the
binary search of a front-coded list is the kind of search that a z-fast trie on
the sampled strings can replace").

**Layout [P]** (Section 3).

- *Storage level*, on disk, memory-mapped: the sorted strings in blocks of
  fixed size (4, 8, 16, 32 KiB); "The first string of each block is stored
  explicitly (i.e., not compressed), whereas the subsequent strings are
  compressed with rear coding until the block is (almost) full"; lengths in
  variable-byte code. Blocks have a fixed size, not a fixed number of strings:
  "compared to the approach of creating variable-sized blocks with a fixed
  number of (front- or rear-coded) strings [41,46], our use of fixed-size
  blocks allows for better compression".
- *Indexing level*, in RAM: "a succinctly-encoded Patricia trie (PT) that
  indexes the first string of every block", with LOUDS or DFUDS topology, one
  byte per edge (branching character), the length of each edge, for LOUDS the
  rank of each leaf, plus a packed array c(b) of the number of strings
  preceding each block (needed for rank queries). In the experiments the trie
  is built on "the first string of every block truncated at its minimum
  distinguishing prefix".
- *Space bound*: for s indexed strings and m <= 2s - 1 nodes, "the overall
  space is at most 10m + (m - s) log l <= (20 + log l)s bits", plus s log s
  bits with LOUDS, l being the maximum edge length.
- *Query*: blind search in three stages (downward traversal; "Compare q against
  the string s pointed to by leaf l", one string access; upward traversal),
  then a scan of one block. Theorem 1: "If the indexing level fits in internal
  memory, then the rank of a query string q can be determined in O(|q|) time
  and at most 2 random I/Os; otherwise, its downward traversal might incur in
  up to |q| I/Os."
- *Cache* (Section 3.3): the labels of the most frequently traversed edges are
  stored in full ("a mix of a compacted trie and a Patricia trie"), so that the
  string access of the blind search can be avoided.

**Datasets, space, times [P].** Machine: Xeon E5-2650L v3 at 1.80 GHz, 32 GB of
RAM per NUMA node, a KIOXIA SSD, GCC 9.4.0.

| Dataset | Strings | Size | Avg. length | Avg. LCP | Alphabet |
|---|---|---|---|---|---|
| URLs (crawls eu-2015, gsh-2015, clueweb12, uk-2014 from law.di.unimi.it) | 3654.1 M | 272.7 GB | 73.6 | 53.7 | 88 |
| Filenames (Software Heritage) | 2294.3 M | 68.9 GB | 29.1 | 15.4 | 241 |

Table 2 (index in MB, storage in GB):

| Block | URLs Array | URLs PT | URLs storage | Filenames Array | Filenames PT | Filenames storage |
|---|---|---|---|---|---|---|
| 4K | 1075.3 | 195.2 | 82.2 | 190.7 | 80.9 | 36.1 |
| 8K | 492.5 | 95.0 | 81.2 | 91.3 | 39.0 | 36.0 |
| 16K | 226.5 | 46.4 | 80.7 | 44.1 | 19.2 | 35.9 |
| 32K | 104.3 | 22.8 | 80.5 | 21.3 | 9.5 | 35.9 |

("Array" is the in-memory array of the block heads searched by binary search,
"as attested by its use on some real data systems like RocksDB".)

*Bits per string.* The paper gives no figure in bits per string. **[I]**
Computed from Table 2, assuming MB = 10^6 bytes and GB = 10^9 bytes (the paper
does not say): with 4-KiB blocks on URLs there are about 20.1 M blocks, i.e.,
about 182 strings per block; the Patricia trie uses 0.43 bits per string of
the dictionary, that is, about 78 bits per indexed string (c(b) included),
against about 429 bits per indexed string for Array; the storage level uses
180 bits (22.5 bytes) per string. On Filenames: 8.8 M blocks, about 260 strings
per block, 0.28 bits per string, about 73 bits per indexed string (Array: 173),
storage 126 bits per string.

*Query times.* They are given only in plots. **[P, read off Figures 7 and 8,
approximate]** Indexing level alone (Fig. 7, membership queries on the indexed
prefixes, no disk access, PT with a full upward traversal): on URLs, Array
about 1.5-2.5 microseconds, PT-LOUDS about 2.5-3.5, PT-DFUDS about 7-10; FST,
PDT and CoCo-trie in between. Whole dictionary (Fig. 8, "The query time
includes both the index and the disk access time"): about 93-110 microseconds
on URLs and about 31-46 on Filenames, i.e., the disk access dominates by more
than an order of magnitude. Text: "on URLs, PT with 8-KiB blocks is the
fastest solution and is closely followed by Array with 8-KiB blocks, which
requires 5.2x more memory"; "under the same block size, on average PT is 5.0x
more compressed than Array on URLs, and 2.3x ... on Filenames"; with the edge
cache, on an adversarial workload forcing two I/Os, query time decreases "by a
factor between 2.8 and 8.1x" on URLs.

Stated conclusions (pp. 2, 12): "sophisticated string dictionaries like FST,
PDT, and CoCo-trie are unnecessarily complex for this purpose"; the fastest
in-memory index is not the best overall because "using more internal memory for
a faster index actually steals space available for caching disk pages"; "All
solutions, except our Patricia trie, use a space that is proportional to the
indexed distinguishing prefixes. ... the Patricia trie ... occupying a space
that is independent of the string lengths."

**Does it use or cite z-fast tries?** Neither. No occurrence of "z-fast", "fat
binary search", "weak prefix" in the text; none of SODA 2009, ESA 2010, SPIRE
2010 among the 54 references. Related items that *are* cited: c-trie++ [11]
(dismissed with ART and CART as "space-inefficient" on the basis of the
experiments of the CoCo-trie paper), Ferragina-Venturini 2016 [17] (as
inspiration, together with the string B-tree and Clark's PATRICIA-II),
BUbiNG [27] for the URLs, and Vigna's broadword select [49] ("we will implement
the former with the sux library"). Ferragina's own paper on weak prefix search
(paper 2) is not cited. No z-fast trie is measured.

**Relevance to a z-fast trie indexing one key every k of a sorted, front-coded
list.** **[I]** This is the same architecture with a succinct Patricia trie in
place of the z-fast trie, and the closest published baseline:

- the blind search costs O(|q|) node-by-node steps (one `select` per level on
  LOUDS) plus one access to an indexed string and an upward traversal; a z-fast
  trie on the block heads would use O(lg |q|) probes of a hash table on
  signatures, and the same single string access to validate the result, as in a
  weak prefix search; like the Patricia trie, its size does not depend on the
  length of the strings;
- the reported numbers bound what can be gained: when the blocks are on disk,
  the index takes 2-10 microseconds out of 30-110, and what matters is its
  size (page cache) and the number of block accesses (one or two); in RAM, the
  indexing-level times of Fig. 7 are the figures to beat, at about 75 bits per
  indexed string;
- they sample by block size (about 180-260 strings per 4-KiB block), not every
  k-th string; and their datasets (URLs from the LAW crawls) are available.

**Proposal.** One sentence in "Static dictionaries of strings" (edit E3). Not
an item of the list of uses.

---

## 6. `CompressedDoubleArrayTries.pdf`

**Identification [P].** Shunsuke Kanda, Kazuhiro Morita, Masao Fuketa,
"Compressed double-array tries for string dictionaries supporting fast lookup".
The PDF is a **preprint**: its first page reads "Under consideration for
publication in Knowledge and Information Systems", with "Received xxx / Revised
xxx / Accepted xxx" (22 pages, produced on 26 September 2016). **[W]** Crossref
confirms the published data of the entry `Kanda2017Compressed`: Knowledge and
Information Systems 51(3):1023-1042, online 4 October 2016, print June 2017.
Differences between preprint and published text were not checked. The code URL
printed in the preprint is `https://github.com/kamp78/cda-tries`, not the
`xcdat` repository named in the note of the BibTeX entry.

**Sentences of `dynzfast.tex`.** None.

**Z-fast tries:** neither used, cited nor measured (no occurrence; the only
reference to our group is UbiCrawler, for the dataset uk-2005).

**What matters as a static competitor [P].** XCDA/FXCDA are static double-array
tries (minimal-prefix trie plus a TAIL array) whose BASE/CHECK arrays are
compressed with directly addressable codes; they support `lookup` (string to
ID) and `access` (ID to string), not rank or predecessor queries on absent
strings **[I]**. Machine: "Quad-Core Intel Xeon 2 x 2.4 GHz, 16 GB RAM", clang
7.0.2, -O3. Queries: "1 million random strings from each dataset". Datasets
(Table 1): geonames (106.1 MB, 6.78 M strings, average length 15.6), nwc-2010
(460.8 MB, 20.7 M, 22.2), jawiki-titles (33.9 MB, 1.52 M, 22.3), enwiki-titles
(238.2 MB, 11.5 M, 20.7), uk-2005 (2855.5 MB, 39.5 M URLs, 72.4), gene-DNA
(198.5 MB, 15.3 M 12-mers, 13.0). Competitors: plain and compressed double
arrays (DA, CDA, DALF), the centroid path-decomposed trie of Grossi and
Ottaviano (Cent, Cent-rp), and PFC, HTFC-rp, HashDAC-rp of Martinez-Prieto et
al.

| Dataset | Structure | Size (% of raw) | lookup (microseconds) | access |
|---|---|---|---|---|
| uk-2005 | FXCDA-y | 25.2 | 2.70 | 3.54 |
| uk-2005 | DA (uncompressed) | 43.8 | 1.95 | 2.93 |
| uk-2005 | Cent / Cent-rp | 27.7 / 17.5 | 3.59 / 4.02 | 4.14 / 4.47 |
| uk-2005 | PFC (bucket 8) | 37.3 | 3.04 | 0.67 |
| uk-2005 | HTFC-rp (8) | 18.3 | 7.96 | 4.41 |
| enwiki-titles | FXCDA-y | 51.1 | 1.31 | 1.82 |
| enwiki-titles | Cent / PFC | 52.4 / 59.6 | 2.40 / 1.97 | 2.48 / 0.62 |
| geonames | FXCDA-y | 52.8 | 0.93 | 1.29 |
| geonames | Cent / PFC | 51.5 / 60.5 | 2.01 / 1.61 | 2.13 / 0.47 |

Claim: "our dictionaries can support the fastest lookup compared to the
state-of-the-art"; "FXCDA-y is up to 2.6, 1.8 and 1.5 times larger than
Cent-rp, HTFC-rp and HashDAC-rp". Plain front coding with buckets of 8 strings
answers lookups in 1.6-3.0 microseconds on these data: this is the search that
a z-fast trie on the sampled strings would replace (line 601).

**Proposal.** At most the addition of the citation to the enumeration at lines
599-600 (edit E4, optional). Otherwise, a baseline for the experiments, not for
the related work.

---

## 7. `LempelZivTries.pdf`

**Identification [P].** Diego Arroyuelo, Rodrigo Canovas, Johannes Fischer,
Dominik Koppl, Marvin Lobel, Gonzalo Navarro, Rajeev Raman, "Engineering
Practical Lempel-Ziv Tries", ACM Journal of Experimental Algorithmics 26(1),
Article 1.14 (October 2021), 47 pages, https://doi.org/10.1145/3481638;
"Received September 2020; revised March 2021; accepted August 2021". The entry
`Arroyuelo2021Engineering` is correct; better `number = {1}`, `pages =
{1.14:1--1.14:47}`, and no "Not read".

**Sentences of `dynzfast.tex`.** None.

**Z-fast tries [P].** *Cited once, not used, not measured.* The whole text
contains one reference to our work, [11] ("Djamal Belazzougui, Paolo Boldi, and
Sebastiano Vigna. 2010. Dynamic Z-fast tries. In Proceedings of the 17th SPIRE
(LNCS), Vol. 6393. 159-172"), in Section 4.3.1 "Jump Pointers" (p. 1.14:23):

> "Similarly to the word-packed lookup techniques of compacted tries [11, 69],
> we can speed up the lz trie traversal by augmenting certain nodes with
> dictionaries to jump over multiple heights: Given an integer parameter delta,
> we augment each node u having a depth of the form delta d with a dictionary
> D_u mapping strings of length delta to its descendants at depth delta(d + 1)
> whose heights are at least delta."

([69] is the packed compact trie of Takagi et al.) The device is a per-node
hash table on the next delta characters at depths multiple of delta: a
fixed-stride skip, with no handles, no 2-fattest numbers and no binary search
on the length **[I]**. The paper is about LZ78/LZW factorization in small
space with hash-based and other representations of the (non-compacted) LZ
trie; no z-fast trie appears among the structures implemented or measured.

**Proposal.** None: it is not a use, and the citing context is generic. It must
not enter the list at lines 580-596.

---

## 8. `SpaceEfficient.pdf`

**Identification [P].** Bo-June (Paul) Hsu (Microsoft Research) and Giuseppe
Ottaviano (Universita di Pisa), "Space-Efficient Data Structures for Top-k
Completion", "WWW 2013, May 13-17, 2013, Rio de Janeiro, Brazil. ACM
978-1-4503-2035-1/13/05". The PDF has 11 pages and its last page is numbered
593; **[W]** Crossref gives pages 583-594, as in the entry `Hsu2013Space`,
which is otherwise correct.

**Sentences of `dynzfast.tex`.** None.

**Z-fast tries / weak prefix search [P].** Neither used nor cited: no
occurrence of "z-fast", "weak", "Belazzougui", "monotone" or "perfect hash" in
the text; the only references to our group are "Codes for the World Wide Web"
[5] and the broadword rank/select paper [37].

**What it does [P].** Three static structures for a scored string set: the
Completion Trie (a compacted trie with the maximum score of each subtree, nodes
in a cache-friendly byte encoding), the RMQ Trie ("a simple scheme to augment
any sorted string dictionary data structure with an RMQ data structure": the
operation "PrefixRange(p) ... returns the pair (a, b), or [bottom] if no string
matches the prefix", then range-minimum queries on the scores in lexicographic
order; "in our experiments we use the lexicographic compressed trie of [17]",
i.e., the path-decomposed trie), and the Score-Decomposed Trie (a
path-decomposed trie whose paths follow the highest scores). On the AOL log
(10M queries): 120 bits/query and 3.7 microseconds for the top-10 completions
(Completion Trie), 62 bits and 8.0 microseconds (Score-Decomposed), 65 bits and
33.9 microseconds (RMQ Trie). Table 1, bits per string including scores: CT
49.3-130.9, SDT 39.8-62.4, RT 42.1-65.5 over QueriesA, QueriesB, URLs,
Unigrams.

**Relevance [I].** Only the RMQ Trie is a consumer of a prefix-range primitive,
the one a z-fast trie on a sorted list provides; note that it requires an
*exact* answer ("or [bottom]"), so a weak prefix search would need a
verification. The statement in `static-and-uses.md` (section 2.11), written
from memory, is confirmed in this limited sense.

**Proposal.** None.

---

## Proposed edits to dynzfast.tex

No sentence of the paper about these eight papers is wrong, because none is
cited. One existing citation is imprecise (E1), and three additions are
justified (E2, E3; E4 is optional). All new text is pure ASCII.

### E1. Line 583: the first use in compressed text indices

- **Old** (line 583):
  `pattern~\cite{Gagie2012Faster,Bille2018Time,Christiansen2018Compressed,Navarro2019Universal,Christiansen2021Optimal,Tsuruta2020Grammar,Gagie2024Space,Kosolobov2026Compressed} (see `
- **New**:
  `pattern~\cite{Gagie2014LZ77,Bille2018Time,Christiansen2018Compressed,Navarro2019Universal,Christiansen2021Optimal,Tsuruta2020Grammar,Gagie2024Space,Kosolobov2026Compressed} (see `
- **Reason.** The LATIN 2014 paper states that the LATA 2012 paper
  (`Gagie2012Faster`) attains O(m^2 + (m + occ) lg lg n) and that "Karp-Rabin
  fingerprinting, fat binary search and bookmarked fingerprinting" are "the
  contribution of this paper"; the O(m lg m) variant of the grammar-based index
  is in "an unpublished extension" (arXiv 1109.3954v6), which is the text that
  the survey read under the key `Gagie2012Faster`. The LATA proceedings paper
  was not read: if the authors prefer to keep the 2012 date, cite the arXiv
  report explicitly (entry `Gagie2012FasterArxiv` below) next to
  `Gagie2014LZ77`, i.e. `\cite{Gagie2012FasterArxiv,Gagie2014LZ77,Bille2018Time,...}`,
  rather than the LATA paper.
- Note for the same item: this paper cites only SODA 2009 (`BBPMMPH`) and calls
  the technique "fat binary search", never "z-fast trie"; the opening sentence
  at lines 578-579 already covers this ("After~\cite{BBPMMPH,...}").

### E2. After line 550: weak prefix search without z-fast tries (papers 2 and 3)

- **Where.** In "Strings in external memory", after the sentence ending at line
  550 (`... has the same I/O bounds as ours for queries in the static case.`)
  and before `Finally, note` (line 551).
- **Old**: nothing (insertion).
- **New**:

```latex
The same alternative appears in the static, compressed setting: weak prefix search, 
which a static z-fast trie solves in $O(|p|/B+\lg|p|)$ I/Os using $O(n\lg\ell)$ bits for $n$ strings of 
average length $\ell$~\cite{Belazzougui2010Fast}, can be solved within the same space in $O(|p|/B+\lg_B n)$ I/Os 
by replacing the z-fast trie with Patricia tries endowed with Karp--Rabin fingerprints and laid out by centroid decomposition, 
with high probability~\cite{Ferragina2013Weak} or deterministically~\cite{Ferragina2016Compressed}. 
Ferragina and Venturini~\cite{Ferragina2016Compressed} also give a static, deterministic cache-oblivious structure that finds the strings sharing the 
longest common prefix with $x$ in $O(|x|/B+\lg_B n)$ I/Os using space within a factor $1+\epsilon$ of a lower bound for the 
set of strings, plus $O(n)$ bits.
```

- **Reason.** These are the two follow-ups of the ESA 2010 paper on its own
  problem; they do not use z-fast tries, so they do not belong to the list of
  uses, but they are the exact static counterpart of the comparison made at
  line 549 (lg_B n against the logarithm of the length). Checked facts:
  Theorems 3 and 4 of paper 2; Table I and Theorems 4.1, 4.3, 4.4 of paper 3
  (with the factor (1 + 1/eps) on the scanning term absorbed in the O for
  constant eps, and B measured in bits as in the rest of our paragraph). The
  bound attributed to ESA 2010 is the one both papers attribute to it. If only
  one sentence is wanted, keep the first.

### E3. Line 602: the two-level dictionary of Ferragina, Rotundo and Vinciguerra

- **Old** (lines 601-602):
  `and are out of the scope of this paper. We just note that the binary search of a front-coded list is the kind of search that a z-fast trie on `
  `the sampled strings can replace. The measurements of a z-fast trie ...`
- **New** (insert after `the sampled strings can replace.`):

```latex
The same two-level organization is engineered in~\cite{Ferragina2025Two} for dictionaries of billions of strings, 
with a succinct Patricia trie on the first string of each block of a rear-coded list kept on disk: the trie 
is descended node by node, and the search is completed by accessing one of the indexed strings.
```

- **Reason.** It is the closest published relative of a z-fast trie on the
  sampled strings of a front-coded list (same architecture, a Patricia trie
  with blind search in place of fat binary search), it reports that this
  indexing level takes less than 200 MB for 3.65 billion URLs, and it neither
  uses nor cites z-fast tries. Everything in the sentence is printed in the
  paper (Sections 2-3, Theorem 1). Cite the journal version only.

### E4 (optional). Lines 599-600: compressed double arrays

- **Old**:
  `path-decomposed tries~\cite{Grossi2015Fast}, and succinct tries~\cite{Zhang2020Succinct,Boffa2024CoCo}; these structures use a fraction of the space of the strings, `
- **New**:
  `path-decomposed tries~\cite{Grossi2015Fast}, compressed double-array tries~\cite{Kanda2017Compressed}, and succinct tries~\cite{Zhang2020Succinct,Boffa2024CoCo}; these structures use a fraction of the space of the strings, `
- **Reason.** Completeness only: on the data sets of the paper it is the
  fastest of the compressed dictionaries for lookups (0.9-2.7 microseconds at
  25-55% of the raw size). It has no relation with z-fast tries; omit it if the
  enumeration is meant to be short.

### No edit

- Lines 450, 452-454, 469, 497 (`Navarro2020Predecessor`): correct, verified
  on the local copy of the survey (not on the downloaded file, which is not the
  survey).
- Lines 588-590 (weak prefix search item): do not add `Ferragina2013Weak` or
  `Ferragina2016Compressed` there.
- Lines 580-596: do not add `Arroyuelo2021Engineering` (one generic citation),
  `Hsu2013Space`, `Kanda2017Compressed`, `Ferragina2025Two` (no z-fast trie).
- `Belazzougui2016Predecessor`: nothing to cite.

---

## Entries for biblio.bib

Needed by the edits above (E1-E4). ASCII, accents as TeX macros; keys as in
`related-work/*.bib`.

```bibtex
@inproceedings{Gagie2014LZ77,
  author    = {Travis Gagie and Pawe{\l} Gawrychowski and Juha K{\"a}rkk{\"a}inen and Yakov Nekrich and Simon J. Puglisi},
  title     = {{LZ77}-Based Self-indexing with Faster Pattern Matching},
  booktitle = {LATIN 2014: Theoretical Informatics -- 11th Latin American Symposium},
  editor    = {A. Pardo and A. Viola},
  series    = {Lecture Notes in Computer Science},
  volume    = {8392},
  pages     = {731--742},
  publisher = {Springer},
  year      = {2014},
  doi       = {10.1007/978-3-642-54423-1_63}
}

@article{Ferragina2013Weak,
  author  = {Paolo Ferragina},
  title   = {On the weak prefix-search problem},
  journal = {Theoretical Computer Science},
  volume  = {483},
  pages   = {75--84},
  year    = {2013},
  doi     = {10.1016/j.tcs.2012.06.011}
}

@article{Ferragina2016Compressed,
  author  = {Paolo Ferragina and Rossano Venturini},
  title   = {Compressed Cache-Oblivious String {B}-Tree},
  journal = {ACM Transactions on Algorithms},
  volume  = {12},
  number  = {4},
  pages   = {52:1--52:17},
  year    = {2016},
  doi     = {10.1145/2903141}
}

@article{Ferragina2025Two,
  author  = {Paolo Ferragina and Mariagiovanna Rotundo and Giorgio Vinciguerra},
  title   = {Two-level massive string dictionaries},
  journal = {Information Systems},
  volume  = {128},
  pages   = {102490},
  year    = {2025},
  doi     = {10.1016/j.is.2024.102490}
}

@article{Kanda2017Compressed,
  author  = {Shunsuke Kanda and Kazuhiro Morita and Masao Fuketa},
  title   = {Compressed double-array tries for string dictionaries supporting fast lookup},
  journal = {Knowledge and Information Systems},
  volume  = {51},
  number  = {3},
  pages   = {1023--1042},
  year    = {2017},
  doi     = {10.1007/s10115-016-0999-8}
}
```

Only if the alternative form of E1 is chosen (data from arXiv, not from a PDF):

```bibtex
@misc{Gagie2012FasterArxiv,
  author       = {Travis Gagie and Pawe{\l} Gawrychowski and Juha K{\"a}rkk{\"a}inen and Yakov Nekrich and Simon J. Puglisi},
  title        = {A Faster Grammar-Based Self-Index},
  howpublished = {arXiv:1109.3954v6},
  year         = {2012},
  note         = {Extended version of the paper presented at LATA 2012}
}
```

Not needed by any edit, corrected for the record (they stay in
`related-work/*.bib`):

```bibtex
@incollection{Belazzougui2016Predecessor,
  author    = {Djamal Belazzougui},
  title     = {Predecessor Search, String Algorithms and Data Structures},
  booktitle = {Encyclopedia of Algorithms},
  editor    = {Ming-Yang Kao},
  pages     = {1605--1611},
  publisher = {Springer},
  year      = {2016},
  doi       = {10.1007/978-1-4939-2864-4_632}
}

@inproceedings{Ferragina2023Engineering,
  author    = {Paolo Ferragina and Mariagiovanna Rotundo and Giorgio Vinciguerra},
  title     = {Engineering a Textbook Approach to Index Massive String Dictionaries},
  booktitle = {String Processing and Information Retrieval -- 30th International Symposium, SPIRE 2023},
  series    = {Lecture Notes in Computer Science},
  pages     = {203--217},
  publisher = {Springer},
  year      = {2023},
  doi       = {10.1007/978-3-031-43980-3_16}
}

@article{Arroyuelo2021Engineering,
  author  = {Diego Arroyuelo and Rodrigo C{\'a}novas and Johannes Fischer and Dominik K{\"o}ppl and Marvin L{\"o}bel and Gonzalo Navarro and Rajeev Raman},
  title   = {Engineering Practical {Lempel--Ziv} Tries},
  journal = {ACM Journal of Experimental Algorithmics},
  volume  = {26},
  number  = {1},
  pages   = {1.14:1--1.14:47},
  year    = {2021},
  doi     = {10.1145/3481638}
}

@inproceedings{Hsu2013Space,
  author    = {Bo-June (Paul) Hsu and Giuseppe Ottaviano},
  title     = {Space-efficient data structures for Top-$k$ completion},
  booktitle = {Proceedings of the 22nd International Conference on World Wide Web (WWW 2013)},
  pages     = {583--594},
  publisher = {ACM},
  year      = {2013},
  doi       = {10.1145/2488388.2488440}
}
```

Unverified details in these entries: the chapter DOI of
`Belazzougui2016Predecessor` (from `TO-DOWNLOAD.md`; not printed on the pages
read) and the full name of the editor (printed as "M.-Y. Kao"); the DOI of the
SPIRE 2023 paper is the one printed in the journal version; the pages of
`Hsu2013Space` are those of Crossref (the PDF ends at page 593); the pages of
`Kanda2017Compressed` are those of Crossref (the PDF is a preprint); the DOI of
`Gagie2014LZ77` is not printed in the PDF (from `TO-DOWNLOAD.md`).
