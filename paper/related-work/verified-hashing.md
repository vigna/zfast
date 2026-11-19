# Hashing, dictionaries and the model: statements verified on the primary sources

Checked on 2026-11-20 against the PDFs in `../../related/`. Subject: the
analysis of signatures in `dynzfast.tex` (Section "Signatures", `sec:sign`,
subsection `sec:dict`) and the three open issues listed in `CORRECTIONS.md`,
section 8: (a) the hash family for signatures, (b) the assumptions on the
dictionary, (c) multiplication in the model.

**Line numbers** refer to `dynzfast.tex` as it was at 00:05 on 2026-11-20
(2566 lines). The file was being edited by someone else while this report
was prepared: eleven lines had been added between lines 381 and 1184 before
that snapshot, and six more were added in the same region one minute later
(2572 lines at 00:06: add 6 to every number above 381 given here; numbers up
to 381 were stable). Every proposed edit therefore quotes the old text:
locate edits by text, not by number. The text of `sec:dict` itself did not
change between the first reading and the last check.

**Marks.** [P] printed in the PDF (page numbers are those printed on the
pages). [I] my inference from printed statements. [D] my own derivation
(written so that it can be checked). [G] general knowledge, not verified on
any source read here. [S] taken from a secondary source, which is named.

**Which versions were read.** Three of the six PDFs are not the versions
cited in `theory.bib`:

| PDF | What it is |
|---|---|
| `PolynomialHashFunctions.pdf` | ICALP 1992 extended abstract, pp. 235-246 (complete) |
| `BackyardCuckoo.pdf` | FOCS 2010 version, pp. 787-796 (not the longer arXiv version, to which it defers all proofs) |
| `LinearProbing.pdf` | **STOC 2007** version, pp. 318-327 (not SIAM J. Comput. 2009) |
| `TabulationHashing.pdf` | **STOC 2011** extended abstract, pp. 1-10 (not JACM 2012); the analysis of linear probing is only in the full version |
| `AC0Implementation.pdf` | pp. 699-707; the venue is not printed on the pages |
| `StaticDictionaries.pdf` | FOCS 1996, pp. 441-450 |

All six were read in full, except the encoding argument for cuckoo hashing
(pp. 6-10 of the tabulation paper) and the lower-bound proofs of the two AC0
papers, which were read for their statements only.

## Summary

- **(a) The analysis has one real error, one wrong length and one loose
  citation.** The signature `H_d` does not depend on the length of the
  string, so handles of different lengths can have the same signature with
  probability 1 (every string of zeroes has signature 0). The length filter
  saves the bound on false positives, but the bound on *duplicate signatures*
  (line 1447) is false: for S = {0^k 1 : k < n} all n - 1 handles have the
  same signature. Appending the length with one more Horner step repairs
  everything. Signatures have max{w, c lg n + lg L} + O(1) bits, not
  c lg n + lg L + O(1), because p > 2^w. The rest (collision bound for equal
  lengths, constant-time prefix signatures, union bound, adaptivity argument)
  is correct. The cited paper contains the family and the collision bound
  (Section 5, Lemma 5.2), nothing about strings, prefixes or incremental
  computation.
- **(b)** Three explicit levels of assumption: expected (linear probing needs
  5-independence; pairwise independence gives logarithmic expected time),
  worst-case lookups with expected amortized updates (dynamic perfect
  hashing, cuckoo hashing), constant worst-case time w.h.p. (Arbitman, Naor
  and Segev: capacity fixed in advance, keys of one word, polynomially many
  operations).
- **(c)** Multiplication is needed for hashing only (dictionary and
  signatures), and there it cannot be avoided in general; it is not needed
  for the most significant bit, which the search can do without altogether.

---

## 1. Dietzfelbinger, Gil, Matias, Pippenger, "Polynomial hash functions are reliable"

**Bibliographic check** (`Dietzfelbinger1992Polynomial` in `biblio.bib`).
First page [P]: title "Polynomial Hash Functions Are Reliable (Extended
Abstract)"; authors M. Dietzfelbinger, J. Gil, Y. Matias, N. Pippenger; pages
235 (first page, unnumbered) to 246. Authors, title, pages agree with the
entry. Neither the volume (LNCS 623) nor the DOI is printed in the PDF; the
entry has no `volume` field (add `volume = {623}`, from Crossref according to
`TO-DOWNLOAD.md`, not verified here).

**What the paper is about.** Its "polynomial hash functions" are *not*
polynomials whose coefficients are the pieces of the key. Section 3.1, p. 238
[P]: "For U = F_q a finite field and d >= 1, let H^d := {g_alpha | alpha =
(alpha_0, ..., alpha_d) in U^{d+1}}, where for alpha in U^{d+1} we let
g_alpha(x) := sum_{i=0}^d alpha_i x^i for x in U." The key is the *point*, the
random coefficients are the function. The title and Sections 4, 6, 7 concern
this class (Fact 3.1, p. 238 [P]: "the classes H^d are (1, d+1)-independent";
Theorem 4.6, p. 240: cubic polynomials are reliable; Theorem 4.17, p. 242:
linear ones are not).

**Section 5, "A New Class of Hash Functions"** (p. 242) is what the paper
under review needs. Verbatim [P]:

- "Definition 5.1 Let the universe be U = F_p^l. Then the class Y of hash
  functions from U into F_p is defined by Y = {(1, t, t^2, ..., t^{l-1}) : t in
  F_p}. For t in Y, t = (1, t, ..., t^{l-1}), the operation of t on x in U,
  x = (x_0, x_1, ..., x_{l-1}), is defined as the inner product t . x."
- "The functions in Y map the universe U of size u = p^l into the range F_p
  of size p. The evaluation of a function from Y can be done in constant
  time. Each such function can be specified using lg p = lg u / l bits."
- "Lemma 5.2 For any x, y in U, x != y, and t chosen at random from Y:
  Prob(t(x) = t(y)) <= (l - 1)/p."
- "Lemma 5.3 Let S subset of U, |S| = s, be fixed. Then, (a) For a random t
  in Y, Prob(t is not injective over S) <= (s choose 2)(l - 1)/p. [...] (c)
  If p > s^{2+epsilon} l for some constant epsilon > 0, and t in Y is random
  then t is injective for S with s-dominant probability."

**What Section 5 does not contain** [P, by reading all twelve pages and by
searching the text layer for "incremental", "prefix", "string"]:

- nothing on strings, on keys of variable length, on prefixes, or on
  incremental evaluation. The universe is F_p^l for a fixed l;
- no cost model in which the length of the key matters: "constant time"
  rests on the convention of Section 3.1, p. 237 [P], "it is assumed that
  arithmetical or vector space operations such as addition, multiplication,
  or scalar multiplication can be performed in one unit of time".

So the citation supports exactly this: for vectors of the same number l of
coefficients in F_p, evaluation at a random point collides with probability
at most (l - 1)/p. That signatures of prefixes come for free is immediate
from the definition, but it is not stated there; "incremental hashing
[Dietzfelbinger et al., Section 5]", as our ESA 2010 paper puts it
([S: `theory.md`, section 8.1]), attributes to the paper a notion it does
not name.

**Other statements used below** [P]:

- p. 244: "In [4] a dictionary is described in which each instruction can be
  performed in constant expected amortized time, and lookups take constant
  time in the worst case. Afterwards, in [5], a real-time dictionary was
  described, i.e., a dictionary in which with high probability all
  instructions execute each in constant time." ([4] is dynamic perfect
  hashing, `DKMDPHULB`; [5] is Dietzfelbinger and Meyer auf der Heide, ICALP
  1990.)
- Theorem 7.1, p. 245: "The real-time dictionary can be implemented using
  lg lg u + lg^{(k)} u + O(lg s) random bits where k is an arbitrary
  constant." The construction first uses class Y to reduce the universe to
  size O(n^{2+epsilon}) (p. 245), and assumes "that the number n of
  instructions to be performed is known beforehand".

## 2. Arbitman, Naor, Segev, "Backyard cuckoo hashing"

**Bibliographic check** (`Arbitman2010Backyard`). First page [P]: "2010 IEEE
51st Annual Symposium on Foundations of Computer Science"; title "Backyard
Cuckoo Hashing: Constant Worst-Case Operations with a Succinct
Representation"; Yuriy Arbitman, Moni Naor, Gil Segev; pp. 787-796; "DOI
10.1109/FOCS.2010.80"; reference [2] of the paper is the longer version,
"arXiv report 0912.5424". The entry is correct in every field.

**Model** [P, p. 788]: "We consider the unit cost RAM model in which the
elements are taken from a universe of size u, and each element can be stored
in a single word of length w = ceil(log u) bits. Any operation in the standard
instruction set can be executed in constant time on w-bit operands. This
includes addition, subtraction, bitwise Boolean operations, left and right
bit shifts by an arbitrary number of positions, and multiplication."

**Guarantee** [P, p. 791]:

- "Theorem III.1. For any n and 0 < epsilon < 1 there exists a dynamic
  dictionary with the following properties: 1) The dictionary stores n
  elements using (1 + epsilon)n memory words. 2) For any polynomial p(n) and
  for any sequence of at most p(n) operations in which at any point in time
  at most n elements are stored in the dictionary, with probability at least
  1 - 1/p(n) over the randomness of the initialization phase, all insertions
  are performed in time O(log(1/epsilon)/epsilon^2) in the worst case.
  Deletions and lookups are always performed in time
  O(log(1/epsilon)/epsilon^2) in the worst case."
- "Theorem IV.1. For any integer n there exists a dynamic dictionary with
  the following properties: 1) The dictionary stores n elements using
  (1 + epsilon)n memory words, for epsilon = Theta((log log n / log n)^{1/2}).
  2) For any polynomial p(n) and for any sequence of at most p(n) operations
  in which at any point in time at most n elements are stored in the
  dictionary, with probability at least 1 - 1/p(n) over the randomness of
  the initialization phase, all operations are performed in constant time,
  independent of epsilon, in the worst case."
- Theorem V.1 (pp. 792-793): the same with (1 + epsilon)B bits, B =
  ceil(log (u choose n)), epsilon = Theta(log log n / (log n)^{1/3}).

**Restrictions that matter for the z-fast trie** [P]:

1. *The capacity n is fixed in advance.* P. 790: "Given an upper bound n on
   the number of elements stored at any point in time"; pp. 795-796,
   "Supporting dynamic resizing. In this paper we assumed that there is a
   pre-determined bound on the maximal number of stored elements. It would
   be interesting to construct a dynamic dictionary with constant worst-case
   operations and full memory utilization at any point in time." The space
   is (1 + epsilon) times the *capacity*, not the current size.
2. *Keys fit one word.* Larger universes are handled by hashing: p. 789-790,
   "U can always be assumed to be of sufficiently large polynomial size by
   using a pairwise (almost) independent function mapping U to a set of
   polynomial size without any collisions with high probability."
3. *The probability is polynomially small in n, and for polynomially many
   operations.* P. 795: "The worst-case performance guarantees of our
   constructions are satisfied with all but an arbitrary small polynomial
   probability over the randomness of their initialization phase", and
   reducing it "to sub-polynomial" is listed as open.
4. *The sequence of operations does not depend on the random choices*
   (footnote 1, p. 787: "for any sequence of operations, with high
   probability over the randomness of the initialization phase, each
   operation is performed in constant time").
5. Hash functions: n^alpha-wise independent functions of Siegel, or those of
   Dietzfelbinger and Woelfel (pp. 789-790), evaluated "in constant time in
   the unit cost RAM model".
6. The FOCS text speaks of elements and lookups only; data associated with a
   key are never mentioned (no occurrence of "satellite", "associated" or
   "key-value" in the text layer). That a pointer can be stored with each key is a
   routine extension [I], not a printed statement.

**Earlier results, as reported there** [P, p. 788-789]: "Dietzfelbinger and
Meyer auf der Heide [15] constructed the first dynamic dictionary with
constant-time operations in the worst case with high probability, using O(n)
memory words to store n elements"; "Arbitman et al. [1] presented a
de-amortization of cuckoo hashing that guarantees constant-time operations
in the worst case with high probability, and achieves memory utilization of
about 50%".

## 3. Pagh, Pagh, Ruzic, "Linear probing with constant independence"

**Bibliographic check.** No entry in `biblio.bib`; `Pagh2009Linear` in
`theory.bib` is the journal version (SIAM J. Comput. 39(3):1107-1120, 2009).
The PDF is the conference version [P, p. 318]: "Linear Probing with Constant
Independence", Anna Pagh, Rasmus Pagh, and Milan Ruzic (printed without
diacritics), "STOC'07, June 11-13, 2007, San Diego, California, USA.
Copyright 2007 ACM 978-1-59593-631-8/07/0006", pp. 318-327. The journal data
are confirmed by the reference list of the tabulation paper [P, its ref. 15]:
"A. Pagh, R. Pagh, and M. Ruzic. Linear probing with constant independence.
SIAM J. Comp., 39(3):1107-1120, 2009. See also STOC'07." Theorem numbers
below are those of the STOC version; the journal version was not seen.

**Positive result** [P, p. 322]: "Theorem 2. Consider any sequence of
operations (insertions, deletions, and lookups) in a linear probing hash
table where the hash function h used has been chosen uniformly at random
from a 5-wise independent family of functions H. Let n and alpha-bar < 1
denote, respectively, the maximum number of keys in the table during a
particular operation and the corresponding maximum load. Then the expected
cost of that operation is O(1 + (1 - alpha-bar)^{-3}). As a consequence, the
expected average cost of successful lookups is O(1 + (1 - alpha-bar)^{-2})."

- The table size r is fixed (p. 319: "S denotes a subset of some universe U,
  and h will denote a function from U to R = [r]"); resizing is not
  discussed.
- Deletions are those of the implementations (p. 318): "Deletions can be
  performed by moving elements back in the probe sequence in a greedy
  fashion".
- The family (Section 2.2, p. 320): "the family of degree k - 1 polynomials
  in any finite field is k-wise independent. [...] a function can be
  evaluated in time O(k) on a RAM, assuming that addition and multiplication
  modulo p can be performed in constant time. To obtain a smaller range
  R = [r] we may map integers in [p] down to R by a modulo r operation.
  [...] the maximum load alpha-bar for this family is in the range
  [alpha; (1 + n/p) alpha]."

**What fails with less** [P]:

- Abstract: "linear probing using a pairwise independent family may have
  expected logarithmic cost per operation".
- "Theorem 1. For r = ceil(p/2) there exists a set S subset of [p], |S| <=
  r/2, such that the expected cost of inserting the elements of S in a
  linear probing hash table of size r using a hash function chosen uniformly
  at random from H(p, r) is Omega(r log r)", where H(p, r) = {x -> ((ax + b)
  mod p) mod r} (p. 321). Corollary 1 (p. 321): "pairwise independence is
  not a sufficient condition for linear probing to have expected constant
  cost per operation." The insertion sequence is "chosen independently of a
  and b" (p. 319).
- The paper has no lower bound for 3- and 4-wise independence (its only
  result with 4-wise independence is positive: Theorem 5, p. 326, on the
  average cost of successful searches in blocked probing). That
  5-independence is necessary is a statement of the tabulation paper [P,
  its p. 1]: "It is known that 5-independence is sufficient [15] and
  necessary [17] for linear probing", [17] being Patrascu and Thorup, ICALP
  2010 (not read).

## 4. Patrascu, Thorup, "The power of simple tabulation hashing"

**Bibliographic check.** No entry in `biblio.bib`; `Patrascu2012Power` in
`theory.bib` is the JACM version, which cannot be checked on this PDF. The
PDF [P, p. 1]: "The Power of Simple Tabulation Hashing", Mihai Patrascu and
Mikkel Thorup, AT&T Labs; "STOC'11, June 6-8, 2011, San Jose, California,
USA. Copyright 2011 ACM 978-1-4503-0691-1/11/06"; pp. 1-10; footnote: "A
full version of this paper is available as arXiv:1011.5200". P. 4 [P]: "We
only have room to present the most essential results; namely the Chernoff
style concentration from Theorem 1 and the results for Cuckoo hashing from
Theorem 2." **The proofs for linear probing are not in this PDF.**

**The scheme** [P, p. 1]: "Keys are viewed as consisting of c characters. We
initialize c tables T_1, ..., T_c mapping characters to random hash codes. A
key x = (x_1, ..., x_c) is hashed to T_1[x_1] xor ... xor T_c[x_c]"; "the
scheme is only 3-independent"; "if the keys are drawn from a universe of
size u, and hash values are machine words, the space required is O(c
u^{1/c}) words"; p. 2: "In all our results, we assume the number of
characters is c = O(1)"; p. 4: "the number of bins is always understood to
be a power of two".

**Linear probing** [P, stated without proof, pp. 2-3]:

- "for linear probing with fill bounded below 1, (2) shows that the longest
  filled interval is of length O(log n) w.h.p."
- "we show that if the table size is m = (1 + epsilon)n, then the expected
  time per operation is O(1/epsilon^2), which asymptotically matches the
  bound of Knuth [12] for a truly random function. In particular, this
  compares positively with the O(1/epsilon^{13/6}) bound of [15] for
  5-independent hashing."
- "For simple tabulation, we get quite strong concentration results for the
  time per operation, e.g., constant variance for constant epsilon. For
  contrast, with 5-independent hashing, the variance is only known to be
  O(log n)."

Hence, with simple tabulation, an operation of linear probing takes expected
constant time, with constant variance, and O(log n) time w.h.p.; *not*
constant time w.h.p. [I].

**Cuckoo hashing** [P, p. 3]: "Theorem 2. Any set of n keys can be placed in
two table of size m = (1 + epsilon)n by cuckoo hashing and simple tabulation
with probability 1 - O(n^{-1/3}). There exist sets on which the failure
probability is Omega(n^{-1/3})." And: "The dictionary can be built (in
linear time) after trying O(1) independent hash functions w.h.p., and later
every query runs in constant worst-case time with two probes"; "In the
dynamic case, Theorem 2 implies that we expect Omega(n^{4/3}) updates between
failures requiring a complete rehash with new hash functions." For
k-independence (p. 2): "For cuckoo hashing, O(lg n)-independence suffices and
at least 6-independence is needed [3]."

**Keys longer than a word.** The extended abstract has no statement about
them: c is a constant and the tables have u^{1/c} entries, so the universe
must be polynomial in n for the tables to have size n^{O(1)} [I]. The only
relevant sentence [P, p. 3]: "In [21], linear probing is shown to have
constant expected performance if the hash function is a composition of
universal hashing down to a domain of size O(n), with a strong enough hash
function on this small domain (i.e. any hash function with a good 4th
moment bound)", [21] being Thorup, "String hashing for linear probing",
SODA 2009, pp. 655-664 (not read). On polynomial hashing as a
k-independent family (p. 2): "Competitive implementations of polynomial
hashing simulate arithmetic modulo Mersenne primes via bitwise operations."

## 5. Thorup, "On AC0 implementations of fusion trees and atomic heaps"

**Bibliographic check.** No entry in `biblio.bib`; `Thorup2003AC0` in
`theory.bib`. First page [P]: "On AC^0 Implementations of Fusion Trees and
Atomic Heaps", Mikkel Thorup, AT&T Labs-Research; pp. 699-707. Title, author
and pages agree. The venue is not printed on any page (the PDF was created in
March 2003, consistent with SODA 2003; the survey took venue and pages from
the reference list of Patrascu and Thorup, FOCS 2014).

**Instruction set** [P, p. 700]: "By standard AC0 operations, we mean
standard operations where all the computational instructions are in AC0.
[...] In AC0, this includes addition, subtraction, comparisons, shifts,
logical and bit-wise boolean operations, and conversions between
representations, the non-trivial ones being between integers and floating
point numbers. AC0 basically rules out multiplication and division." The
formal model (p. 703) has the instruction "A <- msb(A)".

**Results** [P, p. 700]:

- "Theorem 1.1. For any b < sqrt(W), b = omega(1), there is a concrete set X
  of size 2^b so that any representation of X using less than 2^{W/b} space
  spends at least Omega(log b) time on some look-ups if restricted to
  standard AC0 operations." ("a look-up of a value x in X returns a pointer
  to an element in X with value x, if any [...]. Clearly look-ups are no
  harder than searches".)
- "Corollary 1.1. With standard AC0 operations, constant time look-ups or
  searches in worst-case sets of any non-constant size require space
  2^{Omega(W)}. In fact, for any constant epsilon > 0, space 2^{epsilon W}
  suffices for sets of size W^{O(1)}."
- Abstract: "if we have that much space, i.e., 2^{epsilon W} for some
  constant epsilon > 0, then we can tabulate multiplication of (epsilon
  W/2)-bit numbers, and then we get constant time multiplication of words
  using additions and shifts."

The lower bound is for the worst look-up of a deterministic representation;
the paper says nothing about expected time [P, by reading Section 2].

**On the most significant bit** [P, p. 701]:

- "Brodnik et al. [16] have shown that if the computational instructions are
  limited to additions, shifts, and bit-wise boolean operations, then the msb
  operation of computing the position of the most significant set bit in a
  word takes Omega(log log W) time with space O(2^{W^{1-epsilon}}) for any
  fixed epsilon > 0." ([16]: A. Brodnik, P. B. Miltersen, and I. Munro,
  "Trans-dichotomous algorithms without multiplication - some upper and lower
  bounds", Proc. 5th WADS, LNCS 1272, pp. 426-439, 1997; not read.)
- "msb, which is proved non-constant time on the Practical RAM, is, in fact,
  computed in constant time with standard AC0 operations. As noted by
  Andersson and Thorup [10], the msb of x can be obtained by converting x to
  a floating point number and then extracting the exponent".
- "Since msb is one of the key operations coded by multiplication in fusion
  trees, it is clearly not reasonable to exclude it when proving
  lower-bounds for fusion trees."

**On hashing and van Emde Boas structures** [P]:

- p. 700: "However, to get linear, or just polynomial, space with van Emde
  Boas's data structure, we need multiplication based hashing [28]." ([28]
  is Mehlhorn and Naher, `MeNBOD`.)
- p. 702: "However, these results also involve hashing which, as proved by
  Andersson et al. [8], is impossible to implement efficiently with AC0
  operations, even if we are allowed arbitrary non-standard AC0 operations."
- p. 701: without shifts the picture is different ("this lower-bound is
  violated by shifts, with which we get constant time using linear space
  with two-level hashing of Fredman et al. [19]"; this refers to the model
  of Ben-Amram and of Fich and Miltersen, with arithmetic and Boolean
  instructions and no shifts).

The positive part of the paper (Section 3: fusion trees and atomic heaps in
AC0 with the `PackSignBits` and `Shuffle` instructions of the Pentium 4) is
not relevant to the z-fast trie.

## 6. Andersson, Miltersen, Riis, Thorup, "Static dictionaries on AC0 RAMs"

**Bibliographic check.** No entry in `biblio.bib`; `Andersson1996Static` in
`theory.bib`. First page [P]: "Static Dictionaries on AC^0 RAMs: Query time
Theta(sqrt(log n / log log n)) is necessary and sufficient"; Arne Andersson,
Peter Bro Miltersen, Soren Riis, Mikkel Thorup; "0272-5428/96 $05.00 (c)
1996 IEEE"; pp. 441-450. Authors, title, pages, year agree. The proceedings
title is not printed; the PDF metadata say "Foundations of Computer Science,
1996. Proceedings., 37th Annual Symposium on" (the entry has "37th Conference
on"). The DOI is not printed.

**Model** [P, p. 441]: "an AC0 RAM is a RAM with an instruction set
consisting of direct and indirect addressing, conditional jump, and a finite
number of computational instructions each mapping a constant number of words
to one word. All computational instructions belong to AC0. All instructions
are charged unit cost." The instruction set may be chosen freely within AC0
("The AC0 instruction set required is non-standard, but not unrealistic",
p. 442).

**Statements** [P, pp. 441-442]:

- "Theorem A. There is a solution to the static dictionary problem on an AC0
  RAM using O(n) space and with a worst case query time of O(sqrt(log n /
  log log n)). On the other hand, query time o(sqrt(log n / log log n)) is
  not possible on any AC0 RAM even if space 2^{polylog n} is allowed."
- "Theorem B. For any epsilon > 0, there is a solution to the static
  dictionary problem on an AC0 RAM with constant query time using space
  O(2^{w^epsilon}) for storing sets of size n = 2^{w^{o(1)}}. On the other
  hand, on no AC0 RAM there is a constant query time solution using space
  2^{w^{o(1)}} for storing sets of size n = 2^{(log w)^2}."
- "Theorem C. There is no Circuit RAM solution to the static dictionary
  problem using space 2^{polylog n} and with query time o(log w / log log
  w). In fact, for any word length w, there is a fixed set X subset of
  {0,1}^w of size n = 2^{Theta(log^2 w / log log w)} so that any Circuit RAM
  dictionary for X using space 2^{polylog n} has worst case query time
  Omega(sqrt(log n / log log n)) = Omega(log w / log log w). The result
  holds even for Monte Carlo schemes and with worst case time replaced with
  average case time (where the average is over the set X)."
- The precise upper bound (Theorem 21, p. 446): query time O(min{log w
  (log log log n - log log log w)/log log w, sqrt(log n / log log n)}) and
  space O(n).
- Dynamic version (p. 447): "Theorem 22. On an AC0 RAM with randomization, we
  can maintain a dynamic clustering structure supporting insertion,
  deletion, and search. The space is O(n). The worst case query time and the
  expected amortized insertion and deletion times are" the bound of
  Theorem 21. ("The proof of the following theorem is omitted.")

**Scope of the lower bound** [P]. It is a worst case over the relation
between n and w: the hard sets have n = 2^{Theta(log^2 w / log log w)}, that
is, w = 2^{Theta(sqrt(log n log log n))}, much larger than log n. For small
words the bound does not apply: "previous linear space solutions [...]
implementable on AC0 RAMs were constructed by Tarjan and Yao [19], achieving
constant query time if w = c log n for some constant c" (p. 442), and Lemma
10 (p. 444): "T(c log n) = O(c). [...] We use perfect hashing and simulate
multiplication by a precomputed multiplication table."

**On which operations hashing needs** [P]:

- p. 441, on the scheme of Fredman, Komlos and Szemeredi with h_k(x) = (kx
  mod p) mod s: "multiplication and integer division are used by the query
  program. [...] The family may be replaced by a family due to Knuth [14,
  p.509] using only multiplication, bitwise Boolean operations, and shifts
  (see [8] for details), so integer division is not essential."
- p. 442: "there is a result of Mansour, Nisan, and Tiwari [16] (building on
  [15]) that no AC0 circuit implements a family of universal hash functions
  {H_k}, where 'implements' means that the circuit on input (k, x) computes
  H_k(x)."
- p. 442, for constant query time on sets of size w << n << 2^w: "the
  following space bounds are necessary and sufficient [...]: On the
  Classical RAM epsilon 2^w, on the Practical RAM 2^{epsilon w}, and on the
  AC0 RAM 2^{w^epsilon}"; "The best current lower bound for the Practical
  RAM is the Omega(sqrt(log n / log log n)) one implied by this paper".

**Consequence for issue (c)** [I]: a dictionary with constant query time and
linear (even quasi-polynomial) space does not exist on a RAM without
multiplication or another instruction outside AC0, when w is not bounded by
O(log n). The z-map is such a dictionary.

---

## Issue (a): the analysis of signatures in `sec:dict`

The text checked is lines 1395-1425 (signatures, false positives) and
1434-1454 (duplicate signatures).

### (a.1) What is correct

1. **Collision probability for equal lengths** (lines 1403-1404). For
   strings of the same length l, the padded sequences of d = ceil(l/w) words
   differ in some word, words are smaller than 2^w < p, so H_d(x) - H_d(y) =
   sum_i (x_i - y_i) a^{d-1-i} is a nonzero polynomial of degree at most
   d - 1 over Z_p and has at most d - 1 roots [D]. This is Lemma 5.2 of
   Dietzfelbinger et al. with the coefficients in reverse order (which does
   not change the bound, as the class is the same on reversed vectors [I]).
   The text's ceil(l/w)/p is correct, if one unit loose, and
   ceil(l/w) <= 2L/w for L >= w.
2. **Constant-time prefix signatures** (lines 1398-1402). The first
   floor(f/w) words of x[0..f) are words of x, so H_{floor(f/w)} is the same
   for the prefix and for x; if f mod w != 0 one more step of the recurrence
   with the masked partial word gives the signature [D]. No table of powers
   of a is needed (it would be with the coefficient order of Definition
   5.1). Cost: one mask, one multiplication and one addition modulo p.
3. **Union bound** (lines 1411-1414). A probe with a non-handle of length f
   is a false positive only through a handle of length f with the same
   signature; there are fewer than n handles; 2nL/(wp) <= 1/(w n^{c-1}) for
   p >= 2L n^c; at most floor(lg |x|) + 2 <= w + 2 <= 3w probes. Correct.
4. **Adaptivity** (lines 1414-1416). Correct: the execution with signatures
   coincides with the execution with an exact z-map up to the first probe on
   which they differ, so the bad event is contained in a union of events
   indexed by the probes of the exact execution, which are a function of x
   and S only [D].
5. **Independence.** The text requires a to be independent of S and of the
   queries (line 1396). This is necessary and is the oblivious-adversary
   assumption of all the dictionaries of issue (b).

### (a.2) Error: signatures do not depend on the length, and the bound on duplicates is false

The signature of line 1398 is H_d with H_0 = 0 and zero padding. Hence [D]:

- strings that differ by zeroes at the end of the last word have the same
  signature for every a (for instance u and u0, when |u| is not a multiple of
  w);
- strings that differ by leading words equal to zero have the same signature
  for every a (0^w u and u);
- every string of zeroes has signature 0.

The text knows that equal lengths are needed and invokes the length filter
(lines 1404-1405). That is enough for *false positives*. It is not enough
for the next paragraph, which says (lines 1447-1449): "With the same analysis
as above, we obtain that the probability of hitting a duplicate signature
during the execution of the algorithm is at most 3/n^{c-1}." The z-map is
keyed by signatures, and "duplicate" is defined at line 1435 as "two existing
handles have the same signature", whatever their lengths.

**Counterexample** [D]. Let S = {0^k 1 : 0 <= k < n}. It is prefix-free; its
compacted trie is a comb whose internal nodes have name and extent 0^k,
0 <= k <= n - 2, so the skip interval of each is [k..k] and its handle is
0^k. All n - 1 handles have signature 0, for every a, p and w. With the
duplicate marks of lines 1438-1444 all entries but one are marked, and every
probe with a prefix 0^f scans them, comparing the prefix with handles: the
answers remain correct, but a query costs Omega(n) with probability 1.

The phenomenon is not confined to strings of zeroes: any two nodes on a path
whose handles have lengths in the same block of w bits and differ only by
zeroes (u and u0^j) have the same signature with probability 1.

It also breaks the provisional assumption of lines 1392-1393 ("let us assume
that there are no duplicate signatures"), on which the analysis of false
positives rests: that assumption fails with probability 1 on such sets.

The existing implementations do not have this problem: the Java code computes
`Hashes.murmur(v, handleLength, state)`, a hash of the prefix *and of its
length* (`ZFastTrie.java`, lines 224 and 277), and compares handle lengths
before anything else (lines 253 and 255).

**Repair** [D]: put the length into the signature with one more step of the
recurrence,

    sig(x) = (a H_d + |x|) mod p = (sum_{i<d} x_i a^{d-i} + |x|) mod p.

Claim: two distinct strings x, y of length at most L <= 2^w have the same
signature with probability at most ceil(L/w)/p <= 2L/(wp) (for L >= w).
Proof: both signatures are values at a of polynomials of degree at most
ceil(L/w) over Z_p. If |x| != |y|, the constant terms differ, as 0 <= |x|,
|y| <= 2^w < p. If |x| = |y|, then d is the same and the padded sequences
differ in some word x_i != y_i, both smaller than 2^w < p, so the
coefficients of a^{d-i} differ. In both cases the difference is a nonzero
polynomial of degree at most ceil(L/w), with at most that many roots.

The prefix of length f has signature (a H_{f/w} + f) mod p if w divides f,
and (a((a H_{floor(f/w)} + t) mod p) + f) mod p otherwise, t being the last
partial word: still constant time, two multiplications.

(Equivalently, the key of the z-map could be the pair (length, H_d); that
costs lg L more bits per slot. The handle-length filter of the footnote to
`th:errors` is still needed for termination, since a false positive can now
have a handle of a different length.)

Checked by exhaustive enumeration [D] (w = 3, p = 11, all 511 strings of
length at most 8, all 11 values of a; script run in the scratchpad, not
kept): with the definition of the text, pairs of equal length collide for at
most 2 values of a (bound ceil(l/w) - 1 never exceeded), pairs of different
length for up to 11 of 11; with the repaired definition no pair exceeds its
bound (maximum 3 = ceil(8/3)), and the constant-time formula for prefixes
agrees with the definition on every prefix. The handles 0^k of the
counterexample have one signature with the text's definition and pairwise
distinct signatures for every a with the repaired one.

### (a.3) Error: the length of the signatures

Line 1407: "Assuming p >= 2Ln^c, so that signatures have length c lg n + lg L
+ O(1)". But p > 2^w is required (line 1396), and needed: words must remain
distinct modulo p. So signatures have ceil(lg p) >= w + 1 bits, that is,

    max{w, c lg n + lg L} + O(1) bits

(a prime between max{2^w, 2Ln^c} and twice that exists by Bertrand's
postulate [G]). With w = 64, n = L = 2^20, c = 2 the text claims 60 + O(1)
bits, while at least 65 are needed. The conclusion of lines 1423-1425 is not
affected: with n and L unknown, p is a prime slightly larger than
2^{(c+1)w+1} and signatures have (c+1)w + O(1) bits.

Two remarks [D]:

- With that choice of p the failure probability of a query is at most
  3w . 2nL/(wp) <= 3n/2^{cw} <= 3/2^{(c-1)w}, which is stronger than
  3/n^{c-1} and does not degenerate for small n. It also gives the bound in
  expectation cleanly: the fallback costs O((|x|/w + 1) lg |x|) with lg |x|
  <= w, so its expected contribution is O((|x|/w + 1) w 2^{-(c-1)w}) =
  O(|x|/w + 1) for c >= 2. With 3/n^{c-1} alone the expected cost of the
  fallback is not bounded by the claimed bound when n is small with respect
  to lg |x|.
- If the authors want back the c lg n + lg w + O(1) *stored* bits of the
  old analysis: keep p >= 4Ln^c for the computation and store g(sig(x)),
  with g a universal function from Z_p to s bits chosen independently
  (collision probability at most 2/2^s, e.g. Carter-Wegman [G]). Two
  distinct strings then collide with probability at most 2L/(wp) + 2/2^s
  <= 1/(2wn^c) + 2/2^s, and s = c lg n + lg w + 2 gives at most 1/(wn^c):
  the same per-probe bound as line 1412. The stored signatures then take
  cw + lg w + O(1) bits when n is unknown, instead of (c+1)w + O(1).
  Dietzfelbinger et al. use the class Y in the same way, "applying a Y type
  function k times [...] and then a linear hash function" [P, p. 242]. This
  is optional and is not in the list of edits.

### (a.4) Gap: the cost of arithmetic

The text says "takes constant time" without saying what is multiplied. Each
step is a multiplication of two integers smaller than p and a reduction
modulo p, with p of up to (c+1)w + 2 bits, that is, c + 2 words. On a word RAM
with multiplication this is constant time for constant c (schoolbook
multiplication of (c+2)-word numbers; reduction by a precomputed reciprocal,
or with a prime of special form [G]; the tabulation paper mentions
"arithmetic modulo Mersenne primes via bitwise operations" [P, p. 2]). The
unit-cost claim cannot be taken from Dietzfelbinger et al., who charge one
unit to any field or vector-space operation. One sentence should be added.
The values H_i stored by the preprocessing are c + 2 words each, so
O(|x|/w) words and O(|x|/B) I/Os for constant c, as claimed.

### (a.5) The citation

- `\cite{Dietzfelbinger1992Polynomial}` is the right source for the family
  and for the collision bound *if it points to Section 5* (Definition 5.1,
  Lemma 5.2). Without the pointer it misleads: the paper's own "polynomial
  hash functions" are polynomials evaluated at the key (Section 3.1), a
  different family.
- The text must not suggest that prefixes or incremental computation come
  from that paper (see section 1 above).
- Karp and Rabin (1987) is the usual reference for fingerprints of strings
  that can be extended by a character in constant time; their family is
  different (fixed base, random prime) [G; the paper is in section 2 of
  `TO-DOWNLOAD.md` and was not read]. Citing it is optional and should not
  be done without reading it.

### (a.6) Smaller points

- **Duplicates and false positives are one event** [D]. Call a probed prefix
  q *bad* if some handle h != q has sig(h) = sig(q). A false positive needs
  a bad q (q is not a handle and an entry has its signature). A probe that
  meets a marked entry needs a bad q (two distinct handles have signature
  sig(q), and one of them is not q). If no probe of the exact execution is
  bad, the execution with signatures is identical to it, with no comparison
  of strings. Hence the probability of line 1452 is at least 1 - 3/n^{c-1};
  the 6 is correct but loose.
- **"With high probability" presupposes constant-time lookups.** Lines
  1421-1423 conclude time O(|x|/w + lg |x|) w.h.p. This holds only if each
  of the O(lg |x|) lookups takes constant time in the worst case, or w.h.p.
  (see issue (b)). With linear probing the conclusion is a bound in
  expectation.
- **Duplicate marks are specific to open addressing.** The mechanism of
  lines 1438-1444 follows a probe sequence. With the dictionaries that give
  bounds w.h.p. (issue (b), level 3) there is no probe sequence. In that
  regime duplicates can be treated as a failure [D]: with p >= 2Ln^c the
  probability that two of the fewer than n handles collide is at most
  (n^2/2) . 2L/(wp) <= 1/(2w n^{c-2}), so for c >= 3 there are no duplicates
  at all w.h.p., and when one appears any slow method will do.

### (a.7) Proposed text (replaces lines 1395-1416, the `\todo` at 1417-1419, 1421-1425, and the last sentence of the paragraph on duplicates)

```latex
Signatures are computed by evaluating at a random point a polynomial whose coefficients are the words of the
string: this is the class of hash functions of~\cite[Section~5]{Dietzfelbinger1992Polynomial}, which we used
also in~\cite{Belazzougui2010Fast}. Let $L$, with $w\leq L\leq 2^w$, be a bound on the length of the strings (keys and queries),
$p>2^w$ a prime, and $a$ an element of $\mathbf Z_p$ chosen uniformly at random, independently of the strings in $S$ and of the queries. We see a string $x$
as a sequence of words $x_0$, $x_1$, $\dots\,$,~$x_{d-1}$ of $w$ bits, where $d=\lceil |x|/w\rceil$ and the last word is padded with zeroes,
we let $H_0=0$ and $H_{i+1}=(aH_i+x_i)\bmod p$, and we define the signature of $x$ as
\[
	(aH_d+|x|)\bmod p=\biggl(\,\sum_{i=0}^{d-1}x_ia^{d-i}+|x|\biggr)\bmod p.
\]
The length must be part of the signature, because $H_d$ alone does not tell apart strings that differ by zeroes at the end of the last word, or by
leading words equal to zero: for instance, $H_d=0$ for every string of zeroes, whatever $a$.

Two distinct strings $x$ and $y$ of length at most $L$ have the same signature with probability at most $\lceil L/w\rceil/p\leq 2L/(wp)$. Indeed,
the two signatures are the values in $a$ of two polynomials of degree at most $\lceil L/w\rceil$ with coefficients in $\mathbf Z_p$, and
the two polynomials are distinct: if $|x|\neq|y|$ their constant terms are different, as lengths are at most $2^w<p$; if $|x|=|y|$ the
two strings differ in some word, and words are smaller than $2^w<p$. A nonzero polynomial of degree $D$ over a field has at most $D$ roots.

Signatures of handles are computed at construction/insertion time,
whereas signatures for prefixes of the query string $x$ can be computed incrementally: after a
preprocessing using time $O(|x|/w)$ and $O(|x|/B)$ I/Os, in which we compute and store the values $H_i$, the signature of the
prefix of length $f$ is obtained by applying the recurrence to $H_{\lfloor f/w\rfloor}$ and to the last $f\bmod w$ bits of the prefix
(if $w$ does not divide $f$), and then once more to the result and to $f$. This requires a constant number of additions and multiplications
modulo $p$, that is, constant time, as long as $p$ fits a constant number of words.

Assume now $p\geq 2Ln^c$ for some $c>1$: since we need also $p>2^w$, signatures have $\max\{\?w, c\lg n + \lg L\?\}+O(1)$ bits.
Consider the execution of Algorithm~\ref{algo:exitorparex} on input $x$ with an exact z-map: it probes at most
$\lfloor\lg|x|\rfloor+2\leq w+2\leq 3w$ prefixes of $x$, and these prefixes depend only on $x$ and $S$, and thus not on $a$.
We say that one of these prefixes is \emph{bad} if its signature is the signature of a handle different from the prefix itself.
By a union bound on the fewer than $n$ handles, a given prefix is bad with probability at most $2nL/(wp)\leq 1/(wn^{c-1})$, so
with probability at least $1-3/n^{c-1}$ no probed prefix is bad. In that case, the execution of the algorithm with signatures
is identical to the execution with an exact z-map: by induction on the probes, a prefix that is a handle is found, as no other handle
has its signature, and a prefix that is not a handle is not, as no handle has its signature. In particular, there are no false positives.

Thus,
Algorithm~\ref{algo:exitorparex} will perform at most $w+2$ accesses to the z-map and end with the correct output with
high probability; if accesses to the z-map take constant time, the time required is $O(|x|/w +\lg|x|)$.
Note that since $n$ and $L$ are not known in advance, we must actually choose $p\geq 2^{(c+1)w+1}$, so signatures
are $(c+1)w+O(1)=O(w)$ bits long (we assume, as usual, that the set
size is at most $2^w$, so $c\lg n\leq cw$, and $\lg L\leq w$); in this case no probed prefix is bad with probability
at least $1-3n/2^{cw}\geq 1-3/2^{(c-1)w}$.
```

and, for lines 1446-1449 and 1451-1454:

```latex
This means that if during the execution of Algorithm~\ref{algo:exitorparex} we try to query the map with an
existing but duplicate signature, an explicit comparison, requiring time $O(|x|/w)$, will be needed. A query meets
a duplicate signature only if it probes a bad prefix, as two distinct handles have the signature of the prefix, and
one of them is not the prefix.

\smallskip
Summarizing, with probability at least $1-3/n^{c-1}$ a query will neither perform explicit comparisons
during the search nor need to re-run the algorithm on actual handles,
because neither false positives nor duplicate signatures will be met.
```

The `\todo[inline]` at lines 1417-1419 can then be reduced to its last
sentence ("the implementations use different hash functions, for which no
such bound is available"), which remains true.

---

## Issue (b): the dictionary

### (b.1) What the z-map asks of a dictionary

Keys of a constant number of words (a handle of O(w) bits with its length,
or a signature of (c+1)w + O(1) bits), a pointer associated with each key,
fewer than n keys, lookups, insertions and deletions. A query of the z-fast
trie performs O(lg |x|) lookups (plus O(lg max{...}) pointer jumps); an
update performs O(lg max{|x|, |x^+|, |x^-|}) lookups and O(1) insertions or
deletions. The operations requested depend on x and S, and in the signed case
on a; they do not depend on the random choices of the dictionary as long as
these are independent of a [D]. So every bound of the paper is "number of
dictionary operations times the cost of one", and inherits the kind of
guarantee the dictionary offers.

### (b.2) Three levels

1. **Expectation.** Lookups in expected constant time, updates in expected
   amortized constant time. *Linear probing with a 5-independent hash
   function* at load bounded away from 1 (Pagh, Pagh, Ruzic, Theorem 2:
   expected O(1 + (1 - alpha)^{-3}) per operation, for any sequence of
   operations), or with simple tabulation (Patrascu and Thorup, stated in
   the extended abstract, proved in the full version). A pairwise
   independent function is *not* enough (Theorem 1 and Corollary 1 of Pagh,
   Pagh, Ruzic). By linearity of expectation the bounds of the theorems hold
   in expectation. "Amortized" is needed because the table is resized: the
   theorem of Pagh, Pagh, Ruzic is for a table of fixed size [I].
2. **Worst-case lookups, expected amortized updates.** Dynamic perfect
   hashing (`DKMDPHULB`; as described by Dietzfelbinger et al. 1992, p. 244:
   "each instruction can be performed in constant expected amortized time,
   and lookups take constant time in the worst case") or cuckoo hashing
   (`PaRCH`; "every query runs in constant worst-case time with two probes",
   Patrascu and Thorup, p. 3). Query bounds become worst-case for short
   strings, and w.h.p. (over the choice of a alone) for long strings.
   Neither primary source was read here: the statements are as reported by
   the two papers quoted.
3. **High probability.** A dictionary in which every operation takes
   constant time in the worst case with high probability: Arbitman, Naor,
   Segev, Theorems III.1 and IV.1, under the restrictions of section 2 above
   (capacity N fixed in advance, space (1 + epsilon)N words, keys of one
   word, sequences of N^{O(1)} operations, failure probability 1/N^{O(1)});
   earlier, Dietzfelbinger and Meyer auf der Heide (ICALP 1990, as reported
   by both Arbitman et al. and Dietzfelbinger et al. 1992; not read). With
   it, all bounds of both theorems hold w.h.p., including updates.

Keys of more than one word [I]: for level 1, a polynomial of degree 4 over
Z_q with q larger than the universe is 5-independent and takes constant time
for keys of O(1) words (Pagh, Pagh, Ruzic, Section 2.2); for level 3, the
keys are first mapped to one word by a pairwise independent function that is
injective on the stored keys w.h.p. (Arbitman et al., pp. 789-790), and a
collision counts as a failure.

### (b.3) Sentences of the paper that are wrong or vague

| Lines | Text | Problem |
|---|---|---|
| 191-194 | "can perform the lookup of x in time O(lg\|x\|), while updates are performed within the same bounds in expectation (or with high probability using an appropriate dictionary)" | The lookup bound is unqualified: it is worst-case only at level 2, expected at level 1. "Appropriate dictionary" is not named. |
| 195-198 | "lookups and updates are supported in O(\|x\|/w + lg\|x\|) time [...] with high probability" | For lookups, true only with worst-case (or w.h.p.) dictionary lookups; for updates, true only at level 3, and not for updates that split or remove the root (`\todo` at 215). Contradicts `th:boundslong`, which says "in expectation". |
| 206-210 | "predecessor/successor queries and updates are supported [...] with high probability" | Same. |
| 322-323 | "for the former we make the assumptions required by the dictionary of choice" | Says nothing; this is where the assumption should be stated. |
| 336-337 | "The expected time required for insertions and deletions" | Should be "expected amortized" (resizing; rehashing in cuckoo hashing) [I]. The query bounds of the same theorem carry no qualification at all. |
| 348-355 | "with high probability [...] always w.h.p." | Holds at levels 2 and 3 only; the theorem does not say so. |
| 371-374 | "a dictionary using at most 3n buckets (e.g., a linear-probing hash table with load factor at most 2/3 [...] or a variant of Cuckoo hashing)" | Correct for space, but gives no assumption on the hash function. |
| 374-376 | "worst-case bounds only if the dictionary provides worst-case constant-time lookups, as Cuckoo hashing does; with linear probing they hold in expectation" | Correct, but the second half needs "with a 5-independent hash function". |
| 377-379 | "the only reason of having expectations in the update bounds is the presence of a dictionary" | True for `th:bounds`; for `th:boundslong` the signatures are a second source of randomness. |
| 380-381 | "there are dictionaries using linear space in which all operations take constant time with high probability" | Imprecise: linear in a capacity fixed in advance, keys of one word, polynomially many operations, probability over the initialization. The space bound "3n buckets" of the theorems does not apply to it. |
| 1386 | "if the signatures are sufficiently short, accessing the map requires constant time" | Constant in which sense is left open. |
| 1421-1423 | "will end with the correct output in time O(\|x\|/w + lg\|x\|) with high probability" | Needs constant-time lookups (see (a.6)). |
| 1545-1546 | "all time bounds hold only with high probability (whereas the corresponding bounds for short strings are worst-case bounds, provided that the dictionary has worst-case lookups)" | The proviso applies to both halves of the sentence. |

### (b.4) Proposed text

After the model paragraph (in place of "for the former we make the
assumptions required by the dictionary of choice", line 323, and of lines
371-381):

```latex
\noindent\textbf{The dictionary.} The z-map is a dynamic dictionary with fewer than $n$ keys, each taking a constant number
of words and having an associated pointer. Our algorithms access it $O(\lg|x|)$ times to locate the exit node of $x$, and each of our time bounds
is the number of such accesses, plus the time spent outside the dictionary: thus, the bounds hold in the same sense in which the dictionary provides
constant-time operations. We always assume that the random choices of the dictionary are independent of the keys, of the queries, and
of the function computing the signatures. In particular:
\begin{itemize}
  \item With \emph{linear probing} and a hash function chosen from a $5$-independent family, each operation on a table with load factor
  bounded away from one takes constant expected time~\cite{Pagh2009Linear}, so all our bounds hold in expectation (amortized,
  for updates, because of resizing); the same is true of simple tabulation hashing~\cite{Patrascu2012Power}. Note that pairwise
  independence is not sufficient~\cite{Pagh2009Linear}.
  \item With a dictionary providing lookups in constant worst-case time and updates in constant expected amortized time, such as
  dynamic perfect hashing~\cite{DKMDPHULB} or Cuckoo hashing~\cite{PaRCH}, the query bounds of Theorem~\ref{th:bounds} are worst-case bounds,
  the query bounds of Theorem~\ref{th:boundslong} hold with high probability, and the update bounds hold in expectation (amortized).
  \item If an upper bound $N$ on $n$ is known in advance, there are dictionaries using $O(N)$ words in which, for every
  sequence of $N^{O(1)}$ operations, with probability $1-1/N^{O(1)}$ every operation takes constant time in the worst
  case~\cite{Arbitman2010Backyard}: with such a dictionary all bounds of Theorems~\ref{th:bounds} and~\ref{th:boundslong}, including those for
  updates, hold with high probability.
\end{itemize}
The space bounds of Theorems~\ref{th:bounds} and~\ref{th:boundslong} assume a dictionary using at most $3n$ buckets
(e.g., a linear-probing hash table with load factor at most $2/3$ and
backing arrays whose length is a power of two, or Cuckoo hashing with two tables of $3n/2$ buckets).
```

(For keys of one word the third item is exactly Theorem IV.1 of Arbitman et
al.; that keys of O(1) words with an associated pointer are covered is an
inference, see (b.2). If the authors do not want to rely on it, add "for keys
of one word".)

In `th:bounds` (line 336): "The expected amortized time required for
insertions and deletions is". Before both theorems, or in their statements:
"Query bounds are in the worst case if the dictionary has worst-case
lookups, and in expectation otherwise (see below)".

Abstract, lines 190-198:

```latex
size $n-1$. With this simple setup, the z-fast trie can enumerate keys
in sorted order and can perform the lookup of $x$
using $O(\lg|x|)$ accesses to the dictionary; updates are performed within the
same bounds. Thus, lookups and updates require time $O(\lg|x|)$ in expectation using
linear probing, and with high probability using a dictionary
with constant-time operations with high probability. We also show a simple modification that makes it
possible to handle strings of length at most $2^w$; in this case, lookups
are supported in $O(|x|/w + \lg|x|)$ time
(and $O(|x|/B + \lg|x|)$ I/Os in the cache-oblivious
model) with high probability, given a dictionary with worst-case lookups, and updates
within the same bounds in expectation. Finally, just adding two suitable pointers
```

and at lines 207-210 "queries are supported in [...] with high probability,
and updates within the same bounds in expectation". Item (3) of the `\todo`
at line 215 (root splits) is not touched by any of this.

Lines 1545-1546:

```latex
all time bounds hold with high probability if the dictionary has worst-case lookups, and in expectation otherwise
(the corresponding bounds for short strings are worst-case bounds in the first case).
```

---

## Issue (c): multiplication in the model

### (c.1) Where multiplication is used [D]

- **Hash functions of the dictionary.** Every dictionary of issue (b) uses
  multiplication: polynomials over a prime field for k-independence,
  multiplicative schemes for perfect hashing. The model of Arbitman et al.
  includes it explicitly.
- **Signatures.** Each step of the recurrence is a multiplication modulo p.
- **Most significant bit** lambda, used to compute 2-fattest numbers. This
  is the only use named in the authors' note, and it is the one that is not
  needed. Inside a search, the loop of `tab:compalgos` computes lambda once,
  to initialize the mask, and the text already allows a larger initial value
  ("so that i+1 is the length [...], or larger", lines 2021-2022).
  An upper bound s on lambda(|x|) + 1 with s <= 2 lambda(|x|) + 2 is found by
  doubling s while (|x| >> s) != 0, in O(lg lg |x|) steps with shifts and
  comparisons only; the loop then performs at most s + 1 = O(lg |x|)
  iterations. So a search needs no computation of lambda, and no
  multiplication, outside hashing. Checked by simulation: the loop started
  from such a mask returns the 2-fattest number at every step (20,000 random
  runs against brute force; script in the scratchpad, not kept). In updates,
  lambda is needed a constant number of times (handle lengths of the nodes
  that change); each can be computed by doubling and bisection in O(lg lg m)
  steps, m being the length involved, which is within the update bounds
  provided that m is one of the lengths appearing in them. The 2-fat jump
  needs only the *lowest* bit set, x & -x, which involves no multiplication.

### (c.2) Is multiplication necessary for the dictionary? [P, with the scope given in sections 5 and 6]

- On a RAM whose instructions are all in AC0, whichever they are, a static
  dictionary using space 2^{polylog n} has worst-case query time
  Omega(sqrt(lg n / lg lg n)) for suitable n and w, even for Monte Carlo
  schemes and on average over the keys (Andersson et al., Theorems A and C);
  O(sqrt(lg n / lg lg n)) is attainable in linear space, also dynamically
  with expected amortized updates (Theorems 21 and 22).
- With the standard AC0 instructions *including the most significant bit*,
  "constant time look-ups or searches in worst-case sets of any non-constant
  size require space 2^{Omega(W)}" (Thorup, Corollary 1.1). This answers the
  note directly: giving msb for free does not make multiplication
  dispensable.
- Division is not needed (Andersson et al., p. 441).
- When w = O(lg n) multiplication can be tabulated in o(n) words: "we can
  tabulate multiplication of (epsilon W/2)-bit numbers, and then we get
  constant time multiplication of words using additions and shifts" (Thorup,
  abstract); Lemma 10 of Andersson et al. So for universes polynomial in n
  the model without multiplication loses nothing [I].
- "to get linear, or just polynomial, space with van Emde Boas's data
  structure, we need multiplication based hashing" (Thorup, p. 700): the
  z-fast trie is in the same position as the other descendants of van Emde
  Boas trees.

Caveat [I]: the lower bounds are for worst-case query time (and, in
Andersson et al., for bounded-error or average-case variants). That a
dictionary with *expected* constant query time is impossible follows from
the Monte Carlo statement by stopping the query after three times its
expected time; this step is mine, not printed.

### (c.3) The `\todo` at line 2017 ("Citation needed for the lg lg w bound")

The statement "unless multiplication is allowed in the model, the cost of
computing lambda is lg lg w" is right for a RAM with additions, shifts and
bitwise Boolean operations: lower bound Omega(lg lg w) unless the space is
exponential in w^{1-epsilon}, by Brodnik, Miltersen and Munro (WADS 1997), as
reported by Thorup, p. 701 [S: not read]; upper bound by bisection [G]. It
is wrong for "standard AC0" in Thorup's sense, where conversion to floating
point gives lambda in constant time. Constant time with multiplication is
due to Fredman and Willard (`FrWSITBFT`) [S: Thorup, p. 701, "msb is one of
the key operations coded by multiplication in fusion trees"].

### (c.4) Proposed text

Lines 320-321:

```latex
We work in the standard RAM model with a word of length $w$, whose instructions include addition, subtraction,
bitwise Boolean operations, shifts and multiplication. Multiplication is used only to compute hash functions, both for the
dictionary and for signatures: computing the most significant bit of a word, which requires constant time with
multiplication~\cite{FrWSITBFT}, can be avoided (see Section~\ref{sec:improvements}). On the other hand, multiplication cannot
be dispensed with in general, as on a RAM whose instructions are all in $\mathrm{AC}^0$ a dictionary using linear
space cannot answer queries in constant time~\cite{Andersson1996Static}, even if the instruction set contains the computation of the
most significant bit~\cite{Thorup2003AC0}; it can be replaced by a table using $o(n)$ words when $w=O(\lg n)$~\cite{Thorup2003AC0}.
```

Lines 2016-2019:

```latex
Algorithm~\ref{algo:parex} requires the computation of a 2-fattest number, and thus of $\lambda$ (the index of the highest bit set), at each
iteration. With multiplication, $\lambda$ can be computed in constant time~\cite{FrWSITBFT}; however, on a RAM providing just additions,
shifts and bitwise Boolean operations its computation requires time $\Omega(\lg \lg w)$, unless the space used is exponential in
$w^{1-\epsilon}$~\cite{Brodnik1997Transdichotomous}.
In this section we show how to compute the 2-fattest numbers required by Algorithm~\ref{algo:parex} using
a single computation of $\lambda$ at the start of the algorithm, or even none: it is sufficient to start from an upper bound
$s$ on $\lambda(b)+1$ such as the first power of two for which $b\gg s=0$, which can be found by doubling in $O(\lg\lg b)$ steps
and adds at most $\lambda(b)+2$ iterations to the loop of Table~\ref{tab:compalgos}.
```

(The citation of Brodnik et al. should be checked on the paper before it
goes in.)

---

## Proposed edits to `dynzfast.tex`

Line numbers as of 00:05 on 2026-11-20 (2566 lines); new text in full in the
sections named.

| # | Lines | Old text (start ... end) | New text | Reason |
|---|---|---|---|---|
| 1 | 1395-1402 | "Signatures are computed by polynomial hashing~\cite{Dietzfelbinger1992Polynomial} ... last $f\bmod w$ bits of a prefix of length $f$." | (a.7), first and third paragraph | The signature must contain the length (a.2); cite Section 5 (a.5); state the cost of arithmetic (a.4). |
| 2 | 1403-1405 | "The signatures of two distinct strings \emph{of the same length} $l$ ... the length of the prefix we are looking for." | (a.7), second paragraph | Collision bound for any two distinct strings; the appeal to the length filter is no longer needed. |
| 3 | 1407-1416 | "Assuming $p\geq 2Ln^c$, so that signatures have length $c\lg n + \lg L+O(1)$ ... which do not depend on $a$." | (a.7), fourth paragraph | Length is max{w, c lg n + lg L} + O(1) (a.3); one bad event for false positives and duplicates (a.6). |
| 4 | 1417-1419 | "\todo[inline]{This analysis replaces ... no such bound is available.}" | Keep only: "\todo[inline]{The implementations use different hash functions, for which no such bound is available.}" | The analysis has been checked; the remark on implementations stands. |
| 5 | 1421-1425 | "Thus, Algorithm~\ref{algo:exitorparex} will end with the correct output in time $O(\|x\|/w +\lg\|x\|)$ with high probability ... and $\lg L\leq w$)." | (a.7), fifth paragraph | Separate the number of accesses from their cost; give the bound 3n/2^{cw}. |
| 6 | 1447-1449 | "With the same analysis as above, we obtain that the probability of hitting a duplicate signature during the execution of the algorithm is at most $3/n^{c-1}$." | "A query meets a duplicate signature only if it probes a bad prefix, as two distinct handles have the signature of the prefix, and one of them is not the prefix." | False as written (counterexample in (a.2)); true after edit 1. |
| 7 | 1452 | "with probability at least $1-6/n^{c-1}$" | "with probability at least $1-3/n^{c-1}$" | (a.6); optional, 6 is not wrong. |
| 8 | 320-321 | "We work in the standard RAM model with a word of length $w$, allowing multiplications\todo{Seba: ...}." | (c.4), first block | Answers the note. |
| 9 | 322-324 | "Hash functions are used only to implement a dictionary and ... for the former we make the assumptions required by the dictionary of choice, and for the latter we use polynomial hashing (see Section~\ref{sec:dict})." | "Hash functions are used only to implement a dictionary and, in the ``long string'' case, to compute signatures by polynomial hashing (see Section~\ref{sec:dict}); the assumptions on the dictionary are discussed after the statements of the theorems." | The assumption is stated in one place. |
| 10 | 371-381 | "Note that the space bounds assume a dictionary using at most $3n$ buckets ... with high probability~\cite{Arbitman2010Backyard}." | (b.4), first block | Precise assumptions, three levels. |
| 11 | 336 | "The expected time required for  insertions and deletions is" | "The expected amortized time required for insertions and deletions is" | Resizing and rehashing (b.3). |
| 12 | 355 | "I/Os, in expectation, provided that" | "I/Os, in expectation (amortized), provided that" | Same. |
| 13 | 191-198, 206-210 | abstract: "can perform the lookup ... model) with high probability." | (b.4), abstract block | Consistency with the theorems; resolves items (1) and (2) of the `\todo` at 215. |
| 14 | 1545-1546 | "all time bounds hold only with high probability (whereas ... provided that the dictionary has worst-case lookups)." | (b.4), last block | The proviso applies to long strings, too. |
| 15 | 2016-2019 | "However, unless multiplication is allowed in the model, the cost of computing $\lambda$ is $\lg \lg w$\todo{Citation needed for the $\lg\lg w$ bound.}. In this section ... at the start of the algorithm." | (c.4), second block | Citation (secondary: to be checked on the paper), and the remark that lambda is not needed at all. |
| 16 | 2222-2223 | "a signature of $(c+1)w + O(1)$ bits in each slot" | no change | Still correct after edits 1-5. |

Not proposed, for the authors to decide: storing a reduced signature of
cw + lg w + O(1) bits (a.3); treating duplicates as failures when the
dictionary is not an open-addressing table (a.6).

## Corrections to `biblio.bib`

Entries present:

- `Dietzfelbinger1992Polynomial`: authors, title, pages 235-246 verified.
  Add `volume = {623}` (not printed in the PDF; from Crossref according to
  `TO-DOWNLOAD.md`). Cite as `\cite[Section~5]{...}`.
- `Arbitman2010Backyard`: every field verified on the first page (authors,
  title, "51st Annual IEEE Symposium on Foundations of Computer Science",
  pp. 787-796, DOI 10.1109/FOCS.2010.80, arXiv 0912.5424). No correction.
- `DKMDPHULB`: agrees with reference [14] of Arbitman et al. ("SIAM J.
  Comput., vol. 23, no. 4, pp. 738-761, 1994"). No correction.
- `PaRCH` (ESA 2001): the journal version is "R. Pagh and F. F. Rodler.
  Cuckoo hashing. J. Algorithms, 51(2):122-144, 2004" (reference [44] of
  Arbitman et al. and [16] of Patrascu and Thorup). Optional replacement.

Entries to add (the proposed text cites them); the first four are in
`theory.bib`:

- `Pagh2009Linear`: journal data confirmed by the reference list of
  Patrascu and Thorup. The version read is STOC 2007, pp. 318-327 (ACM ISBN
  978-1-59593-631-8); add `note = {Preliminary version in STOC 2007}` or
  check the theorem numbers on the journal version before quoting them.
- `Patrascu2012Power`: the JACM data (volume 59, number 3, 2012) could not
  be checked on the PDF, which is the STOC 2011 version (pp. 1-10, ACM ISBN
  978-1-4503-0691-1, full version arXiv:1011.5200). The field `pages =
  {1--50}` and the article number remain unverified, as the note in
  `theory.bib` says. The statements on linear probing quoted in the paper
  under review are proved only in the full version.
- `Thorup2003AC0`: title, author and pp. 699-707 verified; venue not printed
  in the PDF.
- `Andersson1996Static`: authors, title, pp. 441-450, IEEE 1996 verified.
  Suggested `booktitle = {37th Annual Symposium on Foundations of Computer
  Science (FOCS 1996)}`, after the PDF metadata.
- New, from the reference list of Thorup (his [16]); not read, DOI not
  available from the sources at hand:

```bibtex
@inproceedings{Brodnik1997Transdichotomous,
  author    = {Brodnik, Andrej and Miltersen, Peter Bro and Munro, J. Ian},
  title     = {Trans-dichotomous algorithms without multiplication---some upper and lower bounds},
  booktitle = {Algorithms and Data Structures, 5th International Workshop, WADS '97},
  series    = {Lecture Notes in Computer Science},
  volume    = {1272},
  pages     = {426--439},
  publisher = {Springer},
  year      = {1997}
}
```

  (First names are from general knowledge: Thorup prints "A. Brodnik, P. B.
  Miltersen, and I. Munro".)

Corrections to the surveys in this directory:

- `theory.md`, section 3, "Multiplication": Thorup's lower bound is for sets
  of *any non-constant size* in space less than 2^{Omega(W)}, not for "sets
  of size Omega(log n)"; the bound of Andersson et al. holds for suitable w
  (much larger than lg n) and not when w = O(lg n).
- `theory.md`, section 8.1: Section 5 of Dietzfelbinger et al. has the
  family and the bound "(l - 1)/p" for vectors of l coefficients; it says
  nothing about updating the hash when a character is appended (that
  sentence is from Thorup's notes).
- `theory.md`, section 8.2: the derivation there is the one now in the
  paper; it has the two errors of (a.2) and (a.3) ("signatures of c lg n +
  lg L bits", and the claim on duplicates "in the same way").
- `TO-DOWNLOAD.md`: the linear probing and tabulation papers were downloaded
  in their conference versions; the journal versions are still unread.
