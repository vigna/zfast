# Corrections to `dynzfast.tex` (November 2026)

Changes made with respect to the imported text (commit `8a96615`, Subversion
r24015), following the review in `review-2026-11/`. Labels refer to
`dynzfast.tex`; section numbers are those of the text before the section on
related work was added (the sections after the introduction are now shifted by
one). Every algorithm that was changed is implemented literally in
`python/check_updates.py`, which compares the whole structure with its
definition after every update (`python3 python/check_updates.py`, about half
a minute; `--quick` for a few seconds).

## 1. Algorithms and proofs

- **Exit node** (Section 3.1). The definition had no solution when `x`
  properly extends a key. Now: `n_α ⪯ x` and either `α` is a leaf or `e_α` is
  not a proper prefix of `x`. "Exits on the right" covers that case
  explicitly; the parex is "the internal node with the longest extent that is
  a proper prefix of `x`".
- **2-fat ancestors** (after the definition). The intuition "handle lengths at
  least as fat as `|h_α|`" was wrong. Now: `|h_β|` is the 2-fattest number in
  `[|h_β|..|h_α|]`. A remark after `lemma:twofateq` proves that `m` is a
  backjump of `r` iff `m` is 2-fattest in `[m..r]`; it is used in three
  proofs.
- **`lemma:updown`**. Restricted to positive `x`; the case `a = 1` and the
  second inequality are now proved.
- **`th:parex`**. The statement fixes the order of the 2-fat ancestors,
  asserts the content of the stack and treats `parex(x) = ⊥` separately. In
  the proof, the first case no longer skips steps, and the third case no
  longer claims `f = |h_σ|` (false when `f` is in the skip interval of `σ`
  but is not its handle length).
- **Iteration counts**. "At most `lg|x|` iterations" was false: it is
  `⌊lg|x|⌋ + 2` for `algo:parex` and `algo:exitorparex` (with a proof), and
  `⌊lg|x|⌋ + 1` jumps to reach a leaf. Probe counts in Section 4.3 follow.
- **`algo:exit` (LocateExit)**. Took `σ` but used `η`; it is now a function
  of `x` alone that calls LocateExitOrParex, and `algo:predsucc` and
  `algo:range` call it consistently. The empty stack arises only for a
  single-leaf trie, and the remarks say so.
- **`th:errors`**. The proof now argues on the first false positive (the old
  one assumed earlier answers were correct). The footnote says that the
  handle-length filter is necessary for termination, not a WLOG.
- **Jump-pointer definition** (Section 5). Existence and uniqueness are now
  argued (`|h_α|' > j_α`).
- **`lemma:descleaves`**. Statement unchanged and correct; the proof wrote
  `e_x` for `e_{exit(x)}` and missed the case of `x` extending a leaf. The
  `\todo` asking to check the statement is removed.
- **Insertion, new node** (Section 7.1). The jump pointer towards the old
  exit node is now given in constant time: a copy of the pointer of the exit
  node on a low cut at an internal exit node, the exit node itself otherwise.
- **Insertion, old exit node** (new paragraph). The step was missing: on a
  low cut at an internal exit node its handle changes and both its jump
  pointers must be recomputed. The procedure (follow jump pointers from the
  child), its correctness and its `O(lg|e_{exit(x)}|)` cost are stated.
- **`algo:jumpupda`**. The inner loop tested "`α` is internal" instead of the
  cursor; `|h|' < n_λ` compared a number with a string. The algorithm now
  takes the stack as input and states that it uses the jump pointers of the
  exit node as they were before the insertion.
- **`th:correctjumpupd`**. Proof rewritten: it uses `lemma:updown` (jump
  lengths are nondecreasing down the stack), which was never cited; the
  commented-out parenthesis argument is restored; the bound is `|x^+|` for
  the left case, not `|x^-|`. The `\todo` asking to double-check is removed.
- **Deletion, z-map** (Section 3.4). "`x` cuts low" is vacuous for a key in
  the set. Now: the sibling takes over the handle of the parex iff the
  2-fattest number of the merged skip interval is `|h_parex|`.
- **Deletion, sibling** (Section 7.2, new). When the sibling takes over the
  handle of the parex, its jump pointer on the side opposite to the deleted
  leaf is a copy of that of the parex, and the other one is recomputed.
  Copying is necessary for the time bound (see section 8 below).
- **`algo:jumpupdd`**. It contradicted itself (assumption, test and caption
  named three different conditions), used `J^+` and `|e_α|'` for `J^-` and
  `|h_α|'`, and had a dead assignment. Rewritten after the Java
  `fixJumpsForLeftParexNode`.
- **`th:correctjumpupdd`** (new). The Deletions section was a stub: it now
  has a statement, a bound and a proof. `x^±` are taken in `S ∖ {x}`.
- **`th:parent` / `algo:parent` (ParentStack)**. `b` was `|e_σ| − 1`; it is
  `|n_σ| − 1`, and the input string is `e_σ[0..|n_σ|−1)`, the extent of the
  parent. The proof is new (the old formula for the previous handle length
  was false). The algorithm queried an undefined `x` and did not handle the
  root. The claim that the z-map never returns `⊥` is now proved.
- **Enumerative range queries** (Section 6.3). "The entire subtree under
  `exit(x)` is part of the output" was false (counterexample in the text).
  The paragraph now describes how to find the first leaf of the output and
  the first leaf after it without comparing the enumerated keys, at an
  additional cost `O(lg max{|x^-|,|y^+|} + k)`, which is what the abstract
  and the introduction state.
- **Reference leaves** (Section 3.5). The removal case, left as `\todo{???}`,
  is described.
- **Crossing** (Section 7.1). Defined for proper ancestors, as `th:fatanc`
  needs.
- **2-fattest numbers without `λ`** (Section 8.1). `i − 1` was `i + 1`,
  "increment" was "decrement", the broken sentence is repaired, and the shift
  is stated to be arithmetic.

## 2. Mechanical defects and consistency

- `\url` was `\cite{myurl}`: package `url` is loaded; `unini` is `unimi`;
  the Sux4J footnote uses the current host.
- Duplicate labels: the handle definition is `def:handles`; the stray
  `fig:nodesbis` is removed. "Section ??" is `\ref{sec:updates}`.
  `\label` now follows `\caption` in `fig:tablejumps`.
- `fig/example-query-parex.tex`: last row `8, 9` is `9, 8`.
  `fig/example-query-exitorparex.tex`: six column specifications.
- The handle of the new node in the insertion example is `0111010111`
  (was its name); the deletion example refers to `fig:large-example-new`.
- Names: the new parent is `ν` (the `β` of the figure), the inserted or
  deleted leaf is `μ` (so `λ` is only the most significant bit), the deleted
  internal node is `π`, cursors are `δ`. The intuition after `algo:parex`
  uses `m = |x|` instead of `w`. The common prefix in Section 2.1 is `p`.
- Section 3.5 is renamed "Avoiding the storage of strings" (there were two
  "Space usage"). `fig:nodesbis`, `lemma:updown` and `sec:updates` are now
  referenced.
- `algo:range` output line (was inverted); `algo:exitorparex` output line.

## 3. Experiments

- `tab:urlsm`, `tab:urlsb`: the red-black and B-tree columns were swapped
  with respect to the stored outputs; the headers now match the data.
- All table values are now rounded from the stored outputs (they were
  truncated): `tab:intb`, `tab:intbb` and `tab:urlsb` change in the last
  digit in fifteen cells.
- Claims rewritten to what the tables show: 1.6 to 2.05 times faster than a
  red-black tree at 8 billion integers (was "two times"); 8% to 59% slower
  than a B-tree at 32 billion (was "within 10%"); URLs at a million and at a
  billion stated per operation (was "more than twice faster").
- "Single-shot allocation, no `free`" is restricted to node pools. The text
  states the measurement protocol, that timings are user CPU time of a single
  run, and that the B-tree predecessor column measures `lower_bound`.
- The sentence cut off at the end of the section is completed.

- New Section "A byte-oriented implementation in Rust" (`sec:rust`),
  preliminary: it describes the Rust crate in `../crate` and reports the first
  measurements against Wormhole, the B-tree of the Rust standard library and
  binary search (`tab:rustdynb`, `tab:rustdynm`, `tab:ruststatb`,
  `tab:ruststatm`). The tables are generated by `../bench/run.py table` from
  the raw results stored in `results-epyc/`; the numbers quoted in the text
  were computed from the same files. The old text is now Section "The C
  implementation".

## 4. Bounds, space, signatures

- `th:bounds`, `th:boundslong`: membership queries added; `2n` integers (was
  `n`), in agreement with `fig:nodesbis` and Section 9; update bounds of
  `th:boundslong` now have the `|x|/w` term and exclude root splits.
- Section 9 derives the three counts in order (`(2k+2)w + 9p`,
  `(k+2)w + 11p`, `2w + 12p`) and links the last to the theorems and to the
  figure. The two stale comment blocks with other counts are deleted.
- Dictionary: "worst-case dictionary" removed; the text says that
  worst-case query bounds need worst-case lookups.
- Section 4.3: the garbled probability sentence is rewritten with the
  correct number of probes; duplicate signatures are handled by a duplicate
  mark, whose invariant, insertion and deletion rules are described (as in
  the Java code).
- Section 8.3: the new handle of the exit node after a low cut is shorter
  than `2|x|` unless the exit node is the root (proof included).

- **After reading the primary sources** (`related-work/verified-hashing.md`):
  - Signatures (Section 4.3). The analysis with polynomial hashing written
    from secondary sources had an error: the signature did not depend on the
    length, so strings differing by trailing zeroes (or by leading zero words)
    collided for every choice of the random point, and the bound on duplicate
    signatures was false (for `S = {0^k 1}` all handles had signature 0). The
    length is now the constant term of the polynomial; the collision bound
    holds for any two distinct strings; signatures have
    `max{w, c lg n + lg L} + O(1)` bits (`p > 2^w` is needed); false positives
    and duplicates are treated as one event ("bad" prefixes), giving
    `1 - 3/n^{c-1}`; the citation points to Section 5 of Dietzfelbinger et al.
  - Dictionary (after the theorems). The assumptions are stated at three
    levels: linear probing with 5-independence or simple tabulation (bounds in
    expectation; pairwise independence is not enough); worst-case lookups with
    expected amortized updates (dynamic perfect hashing, Cuckoo hashing);
    constant time with high probability (Arbitman, Naor, Segev, with capacity
    known in advance). The abstract and `th:bounds`/`th:boundslong` say
    "amortized" for updates and no longer claim "with high probability" for
    updates on long strings.
  - Model (Section 1). Multiplication is needed only for hashing; it cannot be
    avoided for a constant-time dictionary (Andersson et al.; Thorup), except
    when `w = O(lg n)`. The most significant bit is not needed at all: the
    loop of Section 8.1 can start from an upper bound found by doubling.

## 5. Language

All items of section D of `review-2026-11/reports/A-paper-referee.md`.

## 6. Related work

Rewritten from the three literature surveys in `related-work/` (which say, for
each statement, whether the primary source was read). The treatment is now a
section, "Related work" (`sec:related`), right after the introduction: it had
grown to seven strands of literature, and the introduction already contains
the statements of the theorems. The introduction keeps only what is needed to
state the contribution, with pointers to the section.

- **Organisation of the section.** Predecessor search on integers (length
  reduction, cardinality reduction, the static and dynamic optimal bounds of
  Pătraşcu and Thorup, range reporting, distance-sensitive search);
  binary search on prefix lengths (x-fast tries, Waldvogel et al., Ružić,
  and what a single handle per node changes); strings in the RAM model;
  strings in external memory; ordered indices for main memory (ART, Masstree,
  HOT, Cuckoo Trie, B-trees, learned indices, Wormhole); uses of z-fast tries
  since 2009, grouped by purpose; static dictionaries of strings.
- **Claims changed.**
  - "The static version ... has been completely solved" now cites the STOC
    2006 paper, too, and says that the dynamic version is solved in
    expectation (FOCS 2014).
  - "the first simple optimal digital data structure that is neither
    recursive nor needs to use bucketing" is now "the first simple dynamic
    digital data structure that attains the O(lg l) bound in linear space
    without being recursive and without using buckets", with the remark that
    the bound is optimal only for sets that are not too small; the section
    states when: for l = w, exactly when lg n = w^Omega(1) and
    lg(2^w - n) = w^Omega(1).
  - "The z-fast trie can answer optimally a number of other queries" is now
    "within the same kind of bounds": existential range and prefix queries
    on integers can be answered exponentially faster (Mortensen, Pagh,
    Pătraşcu).
  - "under a full randomness assumption" is removed: hashing is used for the
    dictionary and for signatures, and signatures are now computed by
    polynomial hashing (see section 4 below).
  - "the first paper to propose a digital data structure for variable length
    strings with optimal performance in the RAM model and good performance in
    the cache-oblivious model" is now "the first dynamic digital data
    structure for variable-length strings whose bounds ... depend only on the
    length of the strings involved, and not on n", crediting Ružić
    (SODA 2007, TALG 2009) for the same bounds in the static, deterministic
    case. Ružić was not cited before.
  - The bound of Andersson and Thorup for strings is stated as in their
    theorem (the common prefix in words, deterministic, worst case).
  - A sentence in the introduction and one in the section state that answers
    are always correct and only running times are probabilistic, as two
    papers describe the structure otherwise.
- **Signatures** (Section 4.3, `sec:dict`). The analysis assumed fully random
  signatures of c lg n + lg w bits, which is incompatible with constant-time
  signatures of prefixes. It now uses polynomial hashing over a prime field
  (Dietzfelbinger et al., as in the ESA 2010 paper), with signatures of
  c lg n + lg L + O(1) bits, L a bound on the length of the strings; the
  probability bounds are unchanged, signatures are (c+1)w + O(1) = O(w) bits
  when n and L are not known, and the space count of Section 9 becomes
  (3c+5)w + O(1) + 12p bits per key. The `\todo` asking for a hash family is
  replaced by one asking all authors to check the new argument.
- **Bibliography.** 63 entries added from the `.bib` files of the surveys
  (verified against Crossref, DataCite or arXiv; their verification notes are
  not printed); the entries of c-trie++ and Wormhole are now the published
  versions, under the same keys. `\dotaccent` preserves the dot accent, which
  the paper redefines.
- **Removed `\todo`s**: "This certainly needs to be updated" and the one on
  Pătraşcu and Thorup and the "first" claims.
- `related-work/TO-DOWNLOAD.md` lists the papers that the surveys could not
  access, in order of importance for the text.

## 7. Notebook

- `search(needsExit=False)` restarted from `|e|` instead of `|e| + 1`
  (wrong parex in 225 of 36,000 targeted queries; none after the fix).
- `rangeExistential` now implements `algo:range` (127 wrong answers in a
  randomized test before; none in the same test after). Stale outputs of the two
  cells that exercised it are cleared.
- Not changed: the successor check compares with the strict successor while
  `successor` returns the key itself for a key in the set; it passes only
  because random queries are almost never keys.

## 7b. Second batch of primary sources (2026-11-20)

Reports: `related-work/verified-indexes.md`, `verified-btrees-hashing.md`,
`verified-static-uses.md`. Nothing the text said about these papers was wrong;
the changes are precisions and additions.

- First use in compressed text indices: the citation is now the LATIN 2014
  paper of Gagie et al., which states that fat binary search with Karp-Rabin
  fingerprints is its contribution (the LATA 2012 paper attains `O(m^2)`).
- Strings in external memory: weak prefix search without z-fast tries
  (Ferragina 2013; Ferragina and Venturini 2016).
- Static dictionaries: the two-level dictionary of Ferragina, Rotundo and
  Vinciguerra; compressed double arrays.
- Ordered indices: B-trees for strings rewritten with attributions (prefix
  B-trees, poor man's normalized keys, the comparison of Gao et al.); HOT
  cited in its journal version, with the tree heights it reports; SIndex, the
  journal version of LITS, Hyperion.
- PIM-trie moved to the list of uses, with a description of what it uses.
- The dictionary: de-amortized cuckoo hashing; iceberg hashing removes the
  need for a bound on `n` known in advance and handles keys of `Theta(w)` bits.
- Signatures: a remark on Rabin's fingerprints (the collision bound is derived
  in the report, not printed in the sources: to be checked) and on CRCs; it
  replaces the note on the hash functions of the implementations.
- `sec:rust`: the text says that the B-tree baseline uses none of the
  techniques for strings, and that ART and HOT are not included yet.

## 8. Open issues (each is a `\todo` in the text)

- Abstract and `th:boundslong`: when an update splits or removes the root, the
  handle to be hashed can be as long as the extent of the root (Section 8.3).
- Section 4.3: the implementations use hash functions for which the bound on
  signatures is not available.
- Related work (`sec:related`): the statements about the papers of section 1
  of `related-work/TO-DOWNLOAD.md` have been checked on the papers
  (`related-work/verified-*.md`); those about the papers of sections 2 and 3
  still come from secondary sources.
- Experiments: the surveys point out that a B-tree storing pointers to the
  strings is a weak baseline for strings (current B-trees truncate prefixes
  and keep key heads in the nodes); the new Section `sec:rust` compares with
  Wormhole, but the old text has not been changed in this respect.
- Section 8.1: the citation for the `lg lg w` lower bound on `λ` without
  multiplication (Brodnik, Miltersen, Munro) is taken from Thorup's paper and
  has not been checked.
- Section 10: two `LINK`s (three with the Rust code); rerun the experiments of
  the C implementation (hardware, compiler, space, several runs, wall-clock
  time, uniform shuffle); the section on the Rust implementation is
  preliminary.
- Section 11: conclusions are missing.
