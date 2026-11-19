# Corrections to `dynzfast.tex` (November 2026)

Changes made with respect to the imported text (commit `8a96615`, Subversion
r24015), following the review in `review-2026-11/`. Labels refer to
`dynzfast.tex`. Every algorithm that was changed is implemented literally in
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

## 5. Language

All items of section D of `review-2026-11/reports/A-paper-referee.md`.

## 6. Related work

- The SPIRE 2010 version is cited. One paragraph mentions the z-fast tries on
  arbitrary alphabets and zuffix arrays, c-trie++ and Wormhole; the four
  entries were added to `biblio.bib` from local sources.

## 7. Notebook

- `search(needsExit=False)` restarted from `|e|` instead of `|e| + 1`
  (wrong parex in 225 of 36,000 targeted queries; none after the fix).
- `rangeExistential` now implements `algo:range` (127 wrong answers in a
  randomized test before; none in the same test after). Stale outputs of the two
  cells that exercised it are cleared.
- Not changed: the successor check compares with the strict successor while
  `successor` returns the key itself for a key in the set; it passes only
  because random queries are almost never keys.

## 8. Open issues (each is a `\todo` in the text)

- Abstract: "with high probability" for updates on long strings; worst-case
  query bounds versus linear probing; root splits.
- Section 1, model: whether multiplications are needed (authors' note).
- Section 1, previous work: Pătraşcu–Thorup (2014) is not in the
  bibliography and the "first" claims predate it (authors' note kept, and a
  precise one added).
- Section 4.3: constant-time prefix signatures are incompatible with full
  randomness; a hash family must be named and the bound redone.
- Section 8.1: citation for the `lg lg w` cost of `λ`.
- Section 8.3: when an update splits or removes the root, the handle to be
  hashed can be as long as the extent of the root.
- Section 10: two `LINK`s; rerun the experiments (hardware, compiler, space,
  several runs, wall-clock time, uniform shuffle); placeholder for the Rust
  implementation.
- Section 11: conclusions are missing.
