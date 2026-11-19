# Referee pass on dynzfast.tex

Sub-review commissioned on 2026-11-16 during the z-fast trie project review; report delivered 2026-11-16/17.
It was produced by a separate reviewing agent that read the material in full. Claims marked as simulated were
checked by that agent with throwaway scripts (copied to `../scripts/` where they survived); they were not all
independently re-verified. See `../REVIEW.md` for which claims were spot-checked.

## Report

**Coverage.** I read all of `/Users/vigna/pub/zfast/dynzfast.tex` (lines 1–2031, commented blocks included), the `fig/*.tex` inputs, the `.mp`, `trie.rb` and `jumps.py` sources, both example sets, and every notebook code cell. Nothing in the project was modified or built. "Simulated" means checked on thousands of random tries with throwaway scripts in `/private/tmp/claude-501/-Users-vigna-pub-zfast/955cf1ac-f832-4902-86ae-acdd82779e8c/scratchpad`. Not done: literature search, viewing rendered PDFs, checking timings against benchmark logs.

## A. Technical correctness (ranked)

1. **1755–1790, Theorem `th:parent` / ParentStack.** `b ← |e_σ|−1` and the input `e_σ[0..|e_σ|−1)` are wrong.
   - Why: the interval then includes part of σ's own skip interval, so Z can return ⊥ or σ again, and "the z-map never returns ⊥" (1770) is false. Simulated: fails on 61% of nodes.
   - Fix: use `|n_σ|−1` and `e_σ[0..|n_σ|−1)`, as line 1752 already says; this never failed and never returned ⊥.
   - Also: `x` is undefined at 1782, and σ = root is unhandled.
   - The proof's formula for `|h_{α_{t−1}}|` is false: with handle lengths 20 and 19 and only the root above, it gives 16, the truth is 0. Prove via "m is a backjump of r iff m is 2-fattest in [m..r]".
   - Confidence: high.

2. **1567–1590, 1635–1643, 911–914: deletion.**
   - Algorithm `jumpupdd` contradicts itself: "λ used to be a left child" (1570), caption "parex node was a left child" (1588), then "if λ is a right child" (1576).
   - Its second loop uses `J^+` and `|e_α|'` where `J^-` and `|h_α|'` are needed; `β ← exit x` is dead code.
   - Read literally it leaves pointers to the deleted leaf (simulated: 34% of deletions).
   - Fix: assume the parent η was a left child; if λ was η's left child, re-target each `J^-_α = λ` by walking `J^-` from ξ until it intercepts `|h_α|'`. This passes all tests, provided ξ's own pointers are recomputed when its handle changes.
   - "x cuts low" (913, 1639) is vacuous for x∈S. The intended condition is that the 2-fattest number of the merged interval is `|h_parex|`.
   - No theorem or time bound is given.
   - Confidence: high.

3. **1539–1565, 1443–1457, 1598–1629: insertion.**
   - 1552: "α is internal" must be "β is internal" (the literal version dereferences a leaf in 17% of runs).
   - 1544: `< n_λ` should be `< |n_λ|`.
   - "exit x" and "parex x" mean the pre-insertion nodes; say so.
   - Missing step: when x cuts low at an internal exit node, that node gets a new handle and its jump pointers must be recomputed. Simulated: 852 of 4905 insertions wrong, all in this case; none with the fixes above.
   - The proof needs jumps to be non-decreasing up the stack. That is Lemma `lemma:updown`, which is never cited anywhere.
   - 1626: the bound is `|x^+|`, not `|x^-|`.
   - The commented-out argument at 1612–1622 is correct and should be restored.
   - Confidence: high.

4. **336, 206–209, 1794–1799: signed updates.**
   - Theorem `th:boundslong` gives updates no `|x|/w` term and says "in expectation"; the abstract says `|x|/w + …` w.h.p.
   - On a low cut, the old exit node's new handle is not a prefix of x. For a non-root exit its length is below 2|x| (simulated), so O(|x|/w) holds but must be argued.
   - When the root is split the new handle can be about |e_root| long regardless of |x|, which exceeds the bound.
   - Confidence: medium-high.

5. **1160–1198: signatures.**
   - O(1) prefix signatures after preprocessing are incompatible with full randomness. Name a hash family and redo the collision bound.
   - Duplicates are "somehow" recorded. Nothing is said on unmarking after deletion, and the text alternates between "explicit comparison" and "re-run".
   - Footnote 1120: the `|h_β|=f` filter is needed for termination, not a WLOG.
   - Proof of `th:errors` relies on an invariant that presumes earlier answers were correct; argue on the first false positive. "Never occurred in the steps preceding" (1132) is false for ⊥ steps.
   - Confidence: medium-high.

6. **1424–1429: enumerative range.**
   - "The entire subtree under exit x is part of the output" is false: S={000,001,010,011}, x=0, y=001.
   - It does hold for the node α where the emptiness walk stopped.
   - The remaining cases, and stopping without one string comparison per key, are untreated.
   - The body claims +O(k); the abstract (202–205) and intro (346–349) add lg max{|x^-|,|y^+|}.
   - Confidence: high on the counterexample, medium on the right bound.

7. **646–651: exit node.** No node satisfies the definition when x properly extends a key, yet 884 and the notebook take the leaf. The parex parenthetical and the proof of `lemma:descleaves` (1267) miss this case. The lemma statement is correct once the definition is fixed. Confidence: high.

8. **849, 1062, 1166, 1237, 1448.** "At most lg|x|" is false: up to ⌊lg|x|⌋+2 iterations (Table 1 has 6 for |x|=20) and ⌊lg|x|⌋+1 jumps. Confidence: high.

9. **784–829, `th:parex`.**
   - Case 3, "necessarily f=h_σ", is false: in Table 1, f=10 and f=9 lie in α_5's skip interval. The conclusion survives.
   - "β=α_{i+1} by Lemma `twofateq`" skips steps.
   - The statement is ill-formed for η=⊥ and never asserts the stack content.
   - Both search algorithms simulated correct. Confidence: high.

10. **320, 338, 1803–1819, Fig. `nodesbis`: space.** The theorems say 12n pointers + n integers. Section 9 gives 11p+2w (exact) and 12p+2w (signed). The figure shows 9 pointers + 2 integers before the dictionary. The comment at 217–220 gives a fourth count. Confidence: high.

11. **349–352, 1282, 865, 960.** Worst-case query bounds need a worst-case-lookup dictionary, yet linear probing is offered; updates are amortized. `th:bounds` has no membership bound although 865 and 960 invoke it; 960 should cite `th:boundslong`. Confidence: high.

12. **Smaller.**
    - 713–715: the "at least as fat" intuition is wrong (it differs from the definition on 43% of nodes). Correct: fatter than every length between it and |h_α|.
    - 519–536: "for every x" fails at 0; the proof skips a=1.
    - 1224–1230: existence and uniqueness need |h_α|'>j_α, which is unstated.
    - 1390: the Output line is inverted. 974: Output says a node, but a stack is returned. 1317, 1391: LocateExit arity.
    - 1656–1690: "i−1" should be i+1; "increment" should be decrement; ≫ must be arithmetic.
    - 1652: lg lg w needs a citation.

**Notebook.**
- It has no jump pointers, updates or ParentStack, so those algorithms had no reference check.
- `search(needsExit=False)` restarts with `a=len(extent)`; the message prints `(a,b]` but the callee uses `[a..b]`. It returns the wrong parex in 248 of 42,924 queries, and none with `+1`.
- `rangeExistential` is not Algorithm `algo:range` and is wrong: S={1000,11110}, [10000,1111) returns True.
- The paper's own range algorithm passed 53,542 tests.

## B. Consistency
- `fig/example-query-parex.tex:267`: last row has a, b swapped (8, 9 should be 9, 8).
- 902–903: β's handle is given as 011101011 (its name); it is 0111010111.
- 907: should reference Fig. `large-example-new`.
- `fig/example-query-exitorparex.tex:8`: seven column specs, six columns.
- 509: `\label` precedes `\caption`, so "Fig." resolves to the wrong counter.
- Names: the new parent is β (888, 1444) but η (1541); β is the exit node in `th:fatanc` and a cursor in `jumpupda`. λ is msb, leaf and predecessor. w is word size and |x| (762). ℓ is string length and binary lcp (436). Index j (793) becomes i (809).
- Two sections are titled "Space usage" (920, 1801). `fig:nodesbis`, `lemma:updown` and `sec:updates` are never referenced; the last is the target of "Section ??".
- 630: "(above)" is stale.
- 1066–1068 and 1314: the empty stack arises only for a single-leaf trie.
- 1130: σ's "definition" is in the proof, not the statement.
- 364 vs 1915: Sux4J (Java, old `dsi` domain) versus an implementation that calls `free`.
- 1922: "two times faster" is 1.6–2.05× in `tab:intb`.

## C. Completeness
**Stubs.** Deletions (1635–1643); Section 8.3 (1792–1799); enumerative range (1424–1429); reference-leaf removal (950); 1934–1935 ends mid-sentence. The SPIRE 2010 conference version (`BBVDZT` in `biblio.bib`) is never cited.

**Todos.**
- 214: reconcile items A4, A6, A10, A11.
- 306: multiplication is needed only for λ, which Section 8.1 removes; decide after fixing the hash family.
- 366: rewrite.
- 762: assume |x|=2^k and rename.
- 790: copy the phrase from 1009.
- 950: when the deleted leaf is referenced by a node other than its parent, give that node the parent's reference leaf.
- 1272: the statement is correct given A7.
- 1630: see A3.
- 1637: `\ref{sec:updates}`.
- 1651: rename the leaf.
- 1917, 1919: add links.
- 2025: write.

**Possibly outdated** (from memory, unverified).
- 236–241 and 366–372: Pătraşcu–Thorup's dynamic optimal structure (2014) postdates this.
- 256–258 and 357–359: "first" claims.
- 391: the string B-tree is not cache-oblivious (Bender et al. is).
- 1897: "presently".
- 1902: MurmurHash2.
- Follow-ups to consider: Δ-fast tries and Ružić (PDFs are in the directory), packed compact tries, c-trie++.

**Experiments need.**
- Hardware, compiler and flags; page settings identical across tables.
- A definition of Contains±, workload and insertion order, and what Add/Delete measure.
- Runs and variance.
- Measured space per key for every structure.
- B-tree node size and string representation.
- How URLs are made prefix-free.
- Fallback and duplicate-signature frequencies.
- Signed versus unsigned variants, and Section 8 ablations.
- Range and prefix queries.
- Digital competitors (ART, HOT, Judy, Masstree, y-fast).

## D. Language
- 228 "structure of choice are"; 234 "predecessor/successors problems"; 283 "Theorem … and"; 286 "make"; 319 missing `|` after `x^-`.
- 354 "reflects"; 369 "in term of"; 380, 383 missing O(; 391 "B-trees and its".
- 451 "brings in back"; 553 "Morevoer"; 662 "nodes contains"; 677 "is 2-fattest number"; 716 "exists"; 721 "that follows".
- 790, 804 "emtpy"; 818 missing "and"; 824 `f=h_σ`; 856 "wheter"; 906 "parex ,".
- 931 "how reduce"; 934 "both extremes skip intervals"; 949 "pointers assignment".
- 1170–1172 garbled; 1187 "signatures"; 1198 "duplicate keys".
- 1263, 1267 `e_x`; 1285 "otherwhise"; 1303 "if and we"; 1306 missing "Algorithm"; 1336 "left jump pointers" for both.
- 1360 "equal x"; 1361 "because is α is"; 1365 "equal o"; 1439 "section".
- 1535 "that update"; 1595 "needs"; 1625 "apart for"; 1637 "},removal"; 1639 "only if is … |x| cuts".
- 1653 "that 2-fattest numbers"; 1657–1666 broken sentence; 1688 `λ{…}`; 1749 "stack 2-fat"; 1754 "avoid to rerun".
- 1815 "preferrable"; 1864 "avoiding to store"; 1895 "collection"; 1896 "golden standard"; 1902 "which … it is".
- 1913 `''short''`; 1916 "can used"; 1918 "public available"; 1921, 1930 "Tables … shows/contains"; 1934 "twice faster".

## Brief given to the reviewer

You are acting as a careful technical referee and proofreader for an unfinished journal-length paper on a data structure. This is a read-and-report task: do NOT modify any file and do not run any build.

## The paper
`/Users/vigna/pub/zfast/dynzfast.tex` (single LaTeX file, ~2030 lines, title "Z-Fast Tries"). Figures and tables are `\input` from `/Users/vigna/pub/zfast/fig/*.tex` (read `example-query-parex.tex`, `example-query-exitorparex.tex`, `jump-line.tex`, `insertion-rightchild-*.tex` when checking the text that refers to them). The running example's string set is `/Users/vigna/pub/zfast/fig/large-example.txt` (and `large-example-new.txt` after one insertion). A reference Python implementation of the algorithms is in `/Users/vigna/pub/zfast/python/explicitZFast.ipynb` (JSON; read the code cells) — use it to cross-check pseudocode.

Read the ENTIRE .tex file, including commented-out passages only insofar as they reveal dropped claims.

## What to look for
A. **Technical correctness** (highest priority). For every definition, lemma, theorem, proof, and Algorithm float:
- Is the statement precise and are all symbols defined before use? (e.g. the notation |h|' and |h|_{(n)} for 2-fat jumps/backjumps, skip interval conventions [i..j] vs (i..j], $w$, $\lambda$.)
- Do the pseudocode loops agree with the definitions and with the Python reference (interval conventions, `a <= b` vs `a < b`, `|e|+1`, what is pushed on the stack, what is returned when x exits at the root)?
- Are there gaps or errors in proofs? Unproved claims presented as obvious that are not? Bounds stated in the abstract/introduction that the body does not establish (the abstract itself carries a "double-check the bounds" note)?
- Signatures section: is the handling of false positives and of duplicate signatures complete and consistent? Is the claimed error probability / expected time justified?
- Jump pointers: is the definition well-formed (existence and uniqueness of the intercepting descendant)? Are the update algorithms after insertion/deletion complete (all four symmetric cases)? Is the deletion section more than a stub?
- Space usage and "Improvements" sections: are the pointer/integer counts consistent with the node descriptions elsewhere in the paper?
Give for each issue: line number(s), what is wrong or unclear, why, and a suggested fix when you have one. Say how confident you are.

B. **Consistency**: variable names that change mid-algorithm or mid-proof, terms used with two meanings, inconsistent notation between sections, figures/tables whose content does not match the text or the example string set, references to sections/algorithms that are wrong or dangling ("Section ??"), duplicate `\label`s.

C. **Completeness**: sections that are stubs or missing (list them), every `\todo` with a one-line assessment of what it would take to resolve, places where the related-work discussion looks outdated (the paper was started in 2010; flag statements about "best known" results or "presently" that may no longer hold — you do not need to do a literature search, just flag them), and what the Experiments section would need to be convincing (what is measured, on what, against what, what is missing: space numbers, hardware description, methodology).

D. **Language**: typos, grammar, broken sentences, with line numbers. Batch these in a compact list at the end; do not let them crowd out A–C.

## Already known — do not spend words re-reporting these
- `\url` is redefined as `\cite{myurl}` (undefined citation at the footnote near line 1932), and that footnote's URL contains the typo "unini".
- Labels `def:twofattest` and `fig:nodesbis` are each defined twice.
- Algorithm LocateExit (near line 1074) takes node sigma as input but its body refers to eta.
- "Section~??" near line 1637; the Conclusions section is empty.

## Output format
At most ~1500 words. Section A as a ranked list (most serious first), each item: `line N` — issue — why — suggested fix — confidence. Then B, C, D as compact lists. Be concrete and specific; if you are unsure whether something is an error, say what you would need to check. Do not pad with praise or general advice.

### Follow-up message sent to the reviewer

The coordinator sent a message while you were working:
The usage limit that interrupted you has reset: please continue your referee pass on /Users/vigna/pub/zfast/dynzfast.tex from where you stopped and deliver the report in the format originally requested. Do not redo reading you have already done. If you had not finished reading the file, prioritise section A (technical correctness of definitions, lemmas, proofs and Algorithm floats, cross-checked against the notebook), then C (completeness), then B, then D. State clearly which line ranges you read carefully and which you did not reach. One addition to the "already known" list, so you need not re-derive it: in the Experiments section the two URL tables (tab:urlsm, tab:urlsb) are suspected of having the red-black and B-tree columns swapped relative to the stored benchmark outputs, and the sentence claiming the z-fast trie is "within 10%" of a B-tree at 32 billion integers is contradicted by the table next to it.

Address this before completing your current task.

### Follow-up message sent to the reviewer

The coordinator sent a message while you were working:
The usage limit that interrupted you has reset: please continue your referee pass on /Users/vigna/pub/zfast/dynzfast.tex from where you stopped and deliver the report in the format originally requested. Do not redo reading you have already done. If you had not finished reading the file, prioritise section A (technical correctness of definitions, lemmas, proofs and Algorithm floats, cross-checked against the notebook), then C (completeness), then B, then D. State clearly which line ranges you read carefully and which you did not reach. One addition to the "already known" list, so you need not re-derive it: in the Experiments section the two URL tables (tab:urlsm, tab:urlsb) are suspected of having the red-black and B-tree columns swapped relative to the stored benchmark outputs, and the sentence claiming the z-fast trie is "within 10%" of a B-tree at 32 billion integers is contradicted by the table next to it.

Address this before completing your current task.
