# B-trees for strings (second batch) and three leads on uses of z-fast tries: statements verified on the primary sources

Checked on 2026-11-20 against ten PDFs in `../../related/`. Subject: the
sentences on B-trees for strings in the paragraph "Ordered indices for main
memory" of `sec:related` (lines 570-577), the first item of the list "Uses of
z-fast tries" (lines 598-601), and the attribution of the ideas used by the
structure under development (a z-fast trie on segments of 4 KiB pages; pages
without the common prefix, four-byte heads in sorted slots; per-segment hash
table; blind directory of page separators).

**Line numbers** refer to `dynzfast.tex` as it was when this report was
written (2637 lines). Every proposed edit quotes the old text: locate edits
by text. No existing file was modified.

**Marks** (as in `verified-btrees-hashing.md`). [P] printed in the PDF (page
numbers are those printed on the pages). [I] my inference from printed
statements. [G] general knowledge, not verified on any source read here.
[S] taken from a secondary source, which is named. "Plot" means a value read
from a bar chart, accurate to about 0.05 at best.

**Which versions were read.**

| # | PDF | What it is | Read |
|---|---|---|---|
| 1 | `PrefixBTrees.pdf` | ACM TODS 2(1), March 1977, pp. 11-26 (scan with a text layer) | In full |
| 2 | `MoreModernBTree.pdf` | Foundations and Trends in Databases 13(3), pp. 169-249, 2024 (publisher's version, 84 PDF pages) | Contents, abstract, Sections 2, 3.1, 5.5, 7.2, the passage on suffix truncation in Section 3.3 (p. 185), reference list; text search of the whole file |
| 3 | `MainMemoryIndex.pdf` | SIGMOD 2001, pp. 163-174 | In full |
| 4 | `B2Trees.pdf` | **The journal version**: Datenbank-Spektrum 22:11-22, 2022, "B2-Tree: Page-Based String Indexing in Concurrent Environments". Not the BTW 2021 paper | In full |
| 5 | `EvaluationBTrees.pdf` | The VLDB Journal 35:4, 2026 (published online 28 November 2025), 24 pages | pp. 1-8 and 12-24 as pages; pp. 9-11 (synthetic data, parameter sweeps) by text search only |
| 6 | `HatTrieVLDB.pdf` | The VLDB Journal 19:633-660, 2010, "Engineering scalable, cache and space efficient tries for strings" | pp. 633-636 as pages; experimental design and section list as text; text search for the rest |
| 7 | `FasterGrammar.pdf` | **The LATA 2012 proceedings paper**, LNCS 7183, pp. 240-251 | In full |
| 8 | `ConciseForwarding.pdf` | ICNP 2017 (IEEE Xplore copy, 10 unnumbered pages) | pp. 1-5 and 9-10 as pages; text search of the whole file |
| 9 | `RealTimeStreaming.pdf` | ESA 2017, LIPIcs, article 41, pp. 41:1-41:15 | pp. 41:1-41:8 as pages; the rest as text |
| 10 | `SubstringOccurrence.pdf` | Algorithmica 74:65-90, 2016 (extended version of a PODS 2011 paper) | pp. 65-67 and 73 as pages; experiments and references as text; text search of the whole file |

All ten PDFs are the papers expected and all have a usable text layer.

## Summary

- **Bayer and Unterauer.** They define two structures: *simple prefix
  B-trees* (shortest separators in the index, with a *split interval* in which
  the split point is chosen so as to shorten the separator) and *prefix
  B-trees* (in addition, the common prefix of a page is pruned from its
  separators). The pruned prefix is **derived from the lower and upper bound
  of the page**, i.e. from separators in the ancestors, and is reconstructed
  while descending; it is **not** the prefix of the keys actually in the
  page, and it is not stored in the page. The variant based on the actual
  keys is mentioned in a Note (p. 20). The sentence at lines 570-572 is
  imprecise on this point (edit E1).
- **Key heads.** Three papers of 2001 put a fixed-size piece of the key in
  the slot: Graefe and Larson (poor man's normalized keys), Lomet (the
  "prefix vector representation", as attributed by Sun et al.; not read) and
  Bohannon et al. (partial keys, which are relative to the *preceding key*).
  The first measurement in a prefix B-tree that I have seen is in Sun et al.
  (2026): four bytes per key give +26% on point queries (synthetic keys).
- **Blind search in B-tree nodes.** The closest relatives of a blind
  directory in the database literature are Ferguson's *Bit Trees* (CACM 1992;
  known here only through Bohannon et al.), which store for each key of a
  leaf only the position of the first bit that differs from the preceding
  key and locate a key with exactly one access to a key, and the partial
  keys of Bohannon et al., which add a few bits of the key after that
  position and need at most one access per node. Offset-value coding
  (Conner, 1977; known through Graefe) is the same pair (offset of the first
  difference from the predecessor, value at that offset) used for sequential
  search and merging. None of papers 1, 2, 3, 5 mentions string B-trees.
- **Hash tags, fingerprints, hash-organized leaves**: in none of papers 1-5.
- **B2-tree**: a trie on single bytes embedded in each 64 KiB page narrows
  the binary search to about 32 slots; about two times faster on lookups
  than Umbra's B+-tree, a baseline with short separators and key bytes in the
  slots but, as far as the paper says, without page-wide prefix truncation.
- **LATA 2012**: the proceedings paper contains no fat binary search, no
  z-fast trie, no Karp-Rabin fingerprint, and does not cite any paper of
  Belazzougui et al. The citation of the LATIN 2014 paper at line 600 is
  correct; the inference recorded in `verified-static-uses.md` is confirmed.
- **Papers 8-10**: none uses a z-fast trie. Yu et al. merely cite the SODA
  2009 paper as theory of minimal perfect hashing; Golan and Porat build a
  structure of their own "similarly to the fat binary search of Belazzougui
  et al."; Orlandi and Venturini describe weak prefix search as an
  alternative that does not solve their problem.

---

## 1. Bayer, Unterauer, "Prefix B-trees" (`PrefixBTrees.pdf`)

**Version** [P, first page]: "Prefix B-Trees", Rudolf Bayer and Karl
Unterauer, Technische Universitaet Muenchen; "ACM Transactions on Database
Systems, Vol. 2, No. 1, March 1977, Pages 11-26"; "Received June 1976;
revised November 1976". An IBM Research Report with the same title (RJ 1796,
June 1976) is its reference [7].

**Entry.** `Bayer1977Prefix` in `biblio.bib` is correct in every field (the
DOI is not printed in the scan). `month = mar` can be added.

**Sentences of `dynzfast.tex`.**

- Lines 570-572: "B-trees for strings are still based on the \emph{prefix
  B-trees} of Bayer and Unterauer~\cite{Bayer1977Prefix}, which store only
  once the prefix common to all keys of a node, and use as separators the
  shortest strings separating the keys of adjacent leaves
  (see~\cite[Section~3.5]{Graefe2011Modern})". Verdict: **the second half is
  correct; the first half is imprecise** (see below). Replacement: edit E1.
- Line 2470: "it uses none of the techniques for strings discussed in
  Section~\ref{sec:related}". Consistent; no change.

**What the paper says.**

- Abstract [P]: "Two modifications of B-trees are described, simple prefix
  B-trees and prefix B-trees. Both store only parts of keys, namely prefixes,
  in the index part of a B*-tree. In simple prefix B-trees those prefixes are
  selected carefully to minimize their length. In prefix B-trees the prefixes
  need not be fully stored, but are reconstructed as the tree is searched."
  (B*-tree is their name for what is now called a B+-tree: records only in
  the leaves, linked leaves.)
- **Simple prefix B-trees** (Section 1, pp. 11-13) [P]. When a leaf is split
  between keys x and y, any string s with x < s <= y can be posted to the
  parent: "We call such a string s a separator [...] It seems prudent to
  choose one of the shortest separators" (p. 12); with the prefix property
  the shortest separator is a prefix of y. "Definition. A simple prefix
  B-tree is a B*-tree in which the B*-index is replaced by a B-tree of
  (variable length) separators." The technique applies to leaves only: "When
  a branch node is being split, one of the separators on that node must be
  moved up one level in the tree" (p. 12).
- **Split interval** (p. 13) [P]: "we could allow a certain split interval
  around the middle (or the median key) of a splitting leaf within which a
  split point [...] should be chosen so as to minimize the length of the
  shortest separator"; the same for branch nodes, where "a shortest separator
  within this interval is moved to the father page" (parameters sigma_l and
  sigma_b). No analysis: "We are quite confident, however, that small split
  intervals improve performance considerably." Example: among "On, Part,
  Problem, Problems, Solution, Solutions" the middle split gives "Problems",
  the best one gives "S".
- Predecessor acknowledged (p. 15) [P]: "Rear compression of keys, described
  in [8] and [9], is a technique similar to using shortest prefixes. Rear
  compression, however, does not use the important device [...] of the split
  interval" ([8] is Knuth, vol. 3; [9] is Wagner, IBM Syst. J. 1973).
- **Prefix B-trees** (Section 4, pp. 16-17) [P]. Each page P has "a largest
  lower bound lambda(P) and a smallest upper bound mu(P)" determined by the
  separators met on the path from the root (for the i-th child, the
  separators s_i and s_{i+1} of the parent, or the bounds of the parent at
  the two ends). All keys and separators in the subtree have the common
  prefix kappa(P), defined as the longest common prefix of lambda(P) and
  mu(P), extended by one letter when the two bounds continue with adjacent
  letters and the upper bound ends there. "lambda(p_i) and mu(p_i), and
  therefore also kappa(p_i), can be derived by traversing the tree from the
  root to the node P(p_i). Therefore there is no need to repeatedly store
  the common prefix kappa(p_i) in P(p_i); it suffices to store it once and
  to store the rest of the separators on P(p_i)." On the example: "in a
  prefix B-tree we would not store the common prefix T with the separators
  in the subtree."
- The prefix is recomputed during the descent, not read from the page [P]:
  search step (1) is "Determine kappa(p) according to its definition in
  Section 4", step (2) "Remove kappa(p) from x" (p. 17); "When proceeding
  from P to a son page P', it is easy to construct lambda(P'), mu(P'), and
  kappa(P') and to iterate the search process" (p. 18). The formal
  definition of a node (p. 19) contains only pointers and partial
  separators. [I] The words "store it once" on p. 17 therefore mean "once in
  the tree" (in the separators of the ancestors), not "once in the page".
- **The variant with the actual keys** is a Note on p. 20 [P]: "In certain
  cases, especially when insertions and deletions are fairly rare, it may be
  desirable to factor out the largest common prefix of the keys or
  separators actually stored in a node P instead of pruning off only
  kappa(P). Binary search of a page would still be possible. The advantage
  is additional storage saving; the disadvantage is additional processing
  required for some insertions or deletions which may alter the longest
  common prefix and therefore also the partial separators on a page."
- Leaves [P]: the pruning "holds even for the keys on leaves, although
  storage of full keys on leaves or at least the repetition of the common
  prefix on the leaves is desirable for sequential processing of the file
  without the use of the index part" (p. 17).
- In-page search (p. 18) [P]: no slot array. Partial separators of variable
  length are delimited by a special symbol; binary search starts "in the
  middle of the used part of the page" and "with a short sequential
  character scan" locates two neighbouring delimiters.
- The two techniques are independent (p. 20) [P]: "The technique of choosing
  shortest separators and pruning off the common prefixes kappa(P) allows
  storing only partial separators in the index part of the tree. The two
  techniques could be applied independently to a B*-tree."
- **Dense indexes** (Section 9, pp. 23-24) [P]: in a secondary index "it
  suffices to always store separators of two successive secondary keys in
  the index instead of full keys", also in the leaf pages; a search follows
  the pointer and "retrieve[s] the record referenced by q_i and compare[s]
  its secondary key with y". [I] This is a 1977 instance of storing only a
  distinguishing piece of each key in the leaf and verifying on the record.
- **Experiments** (Section 10 and Table V, pp. 24-26) [P]: trees built by
  inserting n = 1000, 5000, 10000 keys of maximal length 9 or 15 over an
  alphabet of 13 symbols. "The time to perform the algorithms for simple
  prefix B-trees is nearly identical to the time for B*-trees. Prefix B-trees
  need 50-100 percent more time. Considering the algorithms, this is
  unexpected, and we have, at the moment, no satisfactory explanation for
  this phenomenon." "For trees having between 400 and 800 pages, simple
  prefix B-trees require 20-25 percent fewer disk accesses than B*-trees.
  Compared to simple prefix B-trees, prefix B-trees need about 2 percent
  fewer disk accesses." Average separator length in simple prefix B-trees:
  2.62 to 3.50 characters.
- Analysis (Sections 6-8) [P]: expected length of a shortest separator for
  random keys (Table I: 3.845 bytes for 10^8 keys on an alphabet of 256
  symbols); with separators of 4 bytes instead of keys of 10 bytes, a tree
  of height 3 on pages of 1684 bytes indexes on average 3,944,311 entries
  instead of 753,570 (Table II).
- Not in the paper [P, read in full]: slot arrays, key bytes in fixed-size
  entries, hashing, tries inside pages, blind search.

## 2. Graefe, "More modern B-tree techniques" (`MoreModernBTree.pdf`)

**Version** [P]: "Goetz Graefe (2024), 'More Modern B-Tree Techniques',
Foundations and Trends in Databases: Vol. 13, No. 3, pp 169-249. DOI:
10.1561/1900000070"; Google Inc. It "is intended to complement" the survey
of 2011 (its reference [29]), not to replace it: Section 2 (pp. 172-178) is
a short reminder of the 2011 survey.

**Entry.** `Graefe2024More` in `practical-indexes.bib` is correct in every
field (the comment "Full text not read" can be updated). Not in `biblio.bib`.

**Sentences of `dynzfast.tex`.** None cites it. It bears on lines 570-577;
it contradicts nothing there, and adds nothing that must be cited: for
prefix and suffix truncation the right reference remains Section 3.5 of the
2011 survey.

**What the survey says that matters** (no measurement on strings anywhere).

- In-page layout (Section 3.1, p. 180) [P]: "The traditional layout uses an
  array of pointers (byte offsets within the page), possibly an array of
  pairs of pointer and key prefix after page-wide prefix truncation [5],
  [55]. Alternative approaches employ tries [21] rather than binary search or
  interpolation search [27], [65]." [5] is Bayer and Unterauer; [55] is
  Lomet, "The evolution of effective b-tree: Page organization and
  techniques: A personal account", SIGMOD Record 30(3), 2001; [21] is
  Fredkin's "Trie memory".
- Key heads (p. 181) [P]: "Poor man's normalized keys encode only the first
  few bytes of each key and are suitable for binary search; for sequential
  search, offset-value coding [12] captures the first difference between
  neighboring key values in a fixed-size code." [12] is W. M. Conner,
  "Offset value coding", IBM Technical Disclosure Bulletin 20(7), 1977,
  pp. 2832-2837.
- Offset-value coding (Section 5.5, p. 216) [P]: "Offset-value coding
  combines the size of the truncated prefix with the column value or the
  bytes following that prefix"; "Row-by-row prefix truncation renders binary
  search in b-tree nodes rather awkward but a variant of prefix truncation,
  offset-value coding, enables very efficient sequential search". Section
  7.2 (p. 232): "instead of poor man's normalized keys extracted from each
  key value after page-wide prefix truncation, the search may use
  offset-value coding [12], i.e., encoding each key value relative to its
  immediate predecessor within the b-tree's sort order. In this case, the
  search for one or multiple key values within a sorted array is more akin
  to a merge than a search."
- Summary of Section 7.2 (p. 234) [P]: "Page-wide prefix truncation
  compresses less than key-to-key prefix truncation. The former permits
  binary search and interpolation search but the latter does not."
- In-memory nodes (p. 232) [P]: "variable-size b-tree entries are usually
  managed with an array of offset-length pairs next to a node header; for
  cache efficient search, pairs of offset and key prefix might be better";
  for sequential search "the array to search can be dense - offsets or
  pointers to full index entries should be relegated to a parallel array".
  Out-of-node strings are discouraged (p. 231): "Out-of-node strings and
  memory management seem counter-intuitive to the idea of b-trees".
- Suffix truncation (Section 3.3, p. 185) [P]: "Instead of splitting a full
  leaf node at 50%, the shortest key that separates the key values at, say,
  40% and 60% is posted in the parent node [...] [5]. Note that suffix
  truncation can be applied only when splitting leaf nodes; applying it
  again when splitting branch nodes might guide future root-to-leaf
  traversals to the wrong cousin."
- Fence keys (p. 176) [P]: "each node has fence keys, which are simply
  copies of branch keys in the parent node". This survey uses them for
  consistency checks and foster B-trees; the statement that prefix
  truncation can be defined by the fence keys is in the 2011 survey, p. 240
  [S: `verified-btrees-hashing.md`].
- Masstree (p. 180) [P]: "It is not directly clear how the masstree
  structure compares to a b-tree with prefix- and suffix-truncation [5]
  after order-preserving compression [3], [73]."
- Not in the survey [P, text search of the whole file]: the words
  "fingerprint", "partial key", "Patricia", "string B-tree" do not occur;
  Bohannon et al., Ferragina and Grossi, and Bit Trees are not in the
  reference list (73 items); "blind" occurs only in "blind insertions";
  hashing occurs only as "b-trees on hash values" (an index whose keys are
  hash values, searched by interpolation) and lock-manager hash tables.

## 3. Bohannon, McIlroy, Rastogi, "Main-memory index structures with fixed-size partial keys" (`MainMemoryIndex.pdf`)

**Version** [P, first page]: Philip Bohannon, Peter McIlroy, Rajeev Rastogi
(Lucent Technologies, Bell Laboratories); "ACM SIGMOD 2001 May 21-24, Santa
Barbara, California, USA"; pages numbered 163-174.

**Entry.** `Bohannon2001Main` in `practical-indexes.bib` is correct; the
spelling "McIlroy" is confirmed by the first page (the comment about
Crossref's "Mcllroy" can stay; "Full text not read" can be updated). Not in
`biblio.bib`.

**Sentences of `dynzfast.tex`.** None cites it. It bears on lines 574-576
(key bytes in the entries of a node). Optional addition: edit E4.

**Mechanism, precisely.**

- Setting [P, pp. 163-164]: main-memory OLTP indexes in which keys are
  stored *indirectly* (a pointer to the record), "as is standard in
  main-memory databases"; the cost of fetching the key "can dominate the
  cost of an index traversal", while storing keys in the node is
  "inefficient in both time and space".
- **Partial key** (Section 3.1 and Figure 1, p. 165) [P]: "each key is
  represented in the index by three items: (1) a pointer to the record
  containing the key, (2) the offset of the first bit at which the index key
  differs from the base key, and (3) the first l bits of the index key
  following this offset." The fields are called `pkOffset`, `partKey`,
  `pkLength` (Table 1). The difference bit itself is not stored (p. 166:
  "the difference bit itself in not included in the l bits"): its value
  follows from the order.
- **Base key** [P]: "The base key for each key in N is simply the key
  immediately preceding it in N" (Section 3.3, p. 167). For the first key of
  a node, in the pkB-tree it is "the largest key contained in an ancestor of
  N that is less than the leftmost key" (Section 4.2, p. 170) [I: the lower
  fence of the node]; in the pkT-tree it is the first key of the parent.
- **Comparison** (Theorem 3.1 and Figure 3, p. 166) [P]: with d(.,.) the
  offset of the first differing bit, if the search key k_i and the index key
  k_j are on the same side of the base key k_b and d(k_i,k_b) is different
  from d(k_j,k_b), then d(k_i,k_j) is the minimum of the two and the order
  is decided by which one is larger; no key bit is read. If the offsets are
  equal, the l stored bits are compared with the search key; if they agree
  too, the comparison is unresolved. The search carries a pair (comparison,
  offset) from one key to the next, and from a node to its child.
- **Search in a node** (Figure 5, p. 168) [P]: a *linear* sweep of the
  entries of the node ("Linear Search Algorithm Requiring at most One Key
  Dereference"), which maintains the interval [low, high] of positions
  compatible with what is known. If at the end high - low > 1, the
  procedure FindBitTree is called: "FindBitTree employs the search algorithm
  for Bit Trees described in [11] and requires exactly one key to be
  dereferenced. In a nutshell, the algorithm performs a sequential scan of
  keys in N between offsets low and high. It maintains a variable pos [...]
  For each key examined, if for the difference bit offset in its partial
  key, the bit value in the search key is 1, then variable pos is set to the
  position of the key in N. On the other hand, if the bit value in the
  search key is 0, then keys for which the difference bit offset is greater
  than the current key's, are skipped [...] Once all the keys have been
  examined, N.key[pos] is dereferenced and is compared with the search
  key", and the offset of the mismatch then fixes the final position
  (pp. 168-169). [I] This is the blind search of a Patricia trie followed by
  one key access and a correction, performed on the array of the
  distinguishing-bit positions of adjacent keys instead of on a trie: the
  same two phases as the search in a node of a string B-tree.
- **Bit Trees** [P, p. 165; reference [11] is "D. Ferguson. Bit-tree a data
  structure for fast file processing. Communications of the ACM,
  35(6):114-120, June 1992"; not read]: "Bit Trees extend B+-trees by
  storing partial keys instead of full key values for (only) those keys
  contained in leaf nodes. The partial key in a Bit Tree consists of only
  the offset of the difference bit relative to the previous key in the node.
  [...] the precise position of a search key in a leaf node can be
  determined by performing exactly one pointer dereference to retrieve an
  indirect key." Partial-key trees differ in that "(1) partial keys are
  stored in both internal nodes and leaf nodes, (2) partial keys contain l
  bits of the key value following the difference bit in addition to the
  difference bit offset, and (3) searching for a key in a node in a
  partial-key tree requires at most one pointer to be dereferenced, and
  frequently requires no pointer dereferences". Later (p. 173): "Storing
  zero bytes of key information is a special case which reduces to an
  algorithm similar to the Bit Tree [11], but generalized to handle internal
  levels of the tree".
- Against prefix B-trees (p. 165) [P]: "(1) while prefix trees factor out
  the portion common to all keys in a node, partial-key trees factor out
  information in common between pairs of adjacent keys within the node,
  typically a longer prefix than is common to the whole node, (2) [...] in
  partial-key trees, only the first l bits of the suffix is stored - thus,
  partial-key trees may lose key value information while the prefix B+-tree
  does not, (3) in the prefix B+-tree no pointer dereferences are needed
  [...] and (4) the partial keys stored in a prefix B+tree are variable
  sized and this complicates implementation."
- **Byte granularity** (Section 5.2, p. 171) [P]: "it may be more convenient
  in an implementation to store difference information at a larger
  granularity [...] when byte offsets are equal, it may still be the case
  that the bit offsets would differ. In this case, one simply stores all the
  bits at which the difference could occur, in other words, the entire
  byte." The experiments use "partial keys stored at 2 bytes and stored
  offsets in a byte granularity" (p. 172).
- Updates (p. 169) [P]: "Insertion of a new key in the node requires the
  partial keys of the inserted key and the key following it to be
  recomputed, while deletion requires only the partial key for the key
  following the deleted key to be recomputed"; when the leftmost key of a
  node changes, partial keys in other nodes change too (Sections 4.1-4.2).

**How close it is to a blind directory** [I]. An entry of a byte-granular
pkB-tree with l = 1 byte is (pointer, length in bytes of the longest common
prefix with the preceding key, first differing byte of the key). A blind
directory entry as described in the brief (longest common prefix of adjacent
separators plus the *two* branching bytes) is that entry plus the branching
byte of the preceding key, minus the pointer. The differences are in the
search, not in the data: Bohannon et al. scan the node linearly and need the
stored bytes only to avoid the one key access of a Bit Tree; a string B-tree
node stores the same information as a Patricia trie and descends it. Hence:
the idea of a sorted array of (lcp with predecessor, branching symbol)
searched without reading keys and verified with one key access is Ferguson's
(with bits and no symbol) and Bohannon et al.'s (with l extra bits) in the
B-tree literature, and Ferragina and Grossi's in the trie form; storing both
branching bytes is not in any paper read here.

**Experiments** (Section 5, pp. 170-173) [P]. **No strings**: "We model keys
as unique, fixed-length sequences of unsigned bytes"; each byte is drawn
uniformly from an alphabet of 12 or 220 symbols (3.6 or 7.8 bits of
entropy); "partial-key trees may be used with multi-part and variable-length
keys, we did not implement these options in our tests". Sun Ultra 30 (296
MHz UltraSPARC II, 256 MB, 2 MB L2 cache with 64-byte blocks); nodes of 192
bytes (three L2 blocks); figures "for a tree with 1.5 million elements";
100,000 successful lookups per run; key sizes 8 to 36 bytes; lookups take 6
to 14 microseconds (Figure 9, plot). Conclusions (p. 173): "partial-key
trees perform better then B-Trees (in which keys are stored directly in the
node) for keys larger than 12-20 bytes, depending on key distribution";
pkB-trees have the fewest L2 misses for all key sizes; "direct B-trees
outperform the other algorithms in time for small key sizes"; pkT- and
pkB-trees take "approximately twice the space of indirect storage for all
key sizes, but less space than direct-storage B-trees for all key sizes
greater than 4"; "performance is almost always optimal with small values of
l - 2 or 4 bytes".

The only measurements of pkB-trees on real strings known to me are those of
Gao et al. and Sun et al. (section 5 below): with l = 2 bytes PkB is slower
than Head+Tail on all four datasets and has a compression ratio below 1.

## 4. Schmeisser, Schuele, Leis, Neumann, Kemper, B2-tree (`B2Trees.pdf`)

**Version** [P, first page]: "Datenbank Spektrum (2022) 22:11-22,
https://doi.org/10.1007/s13222-022-00409-y"; title "B2-Tree: Page-Based
String Indexing in Concurrent Environments"; "Received: 30 January 2021 /
Accepted: 5 January 2022 / Published online: 21 February 2022"; open access
(CC BY 4.0). It is the **journal version**. The conference paper is its
reference [10]: "Schmeisser J, Schuele ME, Leis V, Neumann T, Kemper A
(2021) B2-Tree: Cache-Friendly String Indexing within B-Trees. In: Sattler
K-U, Herschel M, Lehner W (eds) BTW 2021, pp 39-58.
https://doi.org/10.18420/btw2021-02" [P]. The journal version adds "a novel
analysis of the B2-tree in concurrent usage scenarios" (p. 12).

**Entries.** `Schmeisser2022B2` in `practical-indexes.bib` is correct (the
issue number is not printed). `Schmeisser2021B2` can now be completed with
editors, pages 39-58 and DOI as printed in reference [10] (not verified on
the conference paper itself). Neither is in `biblio.bib`.

**Sentences of `dynzfast.tex`.** None cites it. Optional addition: edit E5.

**Mechanism** (Section 3, pp. 13-17) [P].

- The outer structure is a B+-tree on Umbra's buffer manager, with pages of
  64 KiB ("the smallest page size Umbra's buffer manager supports"). Keys
  are stored in full, in sorted order, behind an indirection vector (slot
  array). "Our implementation also features some commonly known optimization
  techniques like the derivation of a shortened separator during a split".
- Each page contains an *embedded tree* that maps a key to a *search range*
  of slots. Two kinds of nodes (Figure 3): *decision nodes* (a one-byte
  header, a few key bytes used as separators, and either child pointers or,
  in leaf decision nodes, two-byte range indices); they "operate on a fixed
  size decision boundary represented by a single byte" and "direct the
  search to the first child where the currently investigated byte of the
  search key is less or equal to the separator", so, unlike the branch nodes
  of a Patricia trie, they "use the range of bytes" and not equality. *Span
  nodes* "store the byte sequence which forms the longest common prefix
  found in the subtree rooted at the current node" (a two-byte length and
  the bytes); the child follows the span node physically, and a mismatch is
  handled by two *virtual edges* (to the minimum and to the maximum of the
  subtree). [I] It is not a blind trie: the skipped bytes are stored and
  compared.
- A *range array* translates the indices stored in the leaf decision nodes
  into slot positions (exclusive upper bounds); an insertion or deletion
  only increments or decrements the entries of the range array from the
  affected range on, without touching the tree (p. 15).
- The tree is rebuilt "each time a page is split or merged and also
  periodically depending on the number of changes since the last
  invocation": starting from the common prefix of the page, it recursively
  creates span nodes and decision nodes "until each final subrange is at
  most as large as our threshold value"; "Our choice of roughly 32 elements
  per search range yielded the optimal result on all tested datasets"
  (p. 17).
- Lookup (Section 3.2): traverse the embedded tree, obtain the range and
  `skip` ("the length of the largest common prefix within the returned
  search range"), then "a regular binary search on the limited range of
  entries", comparing suffixes only. Each slot of the indirection vector
  occupies 12 bytes and contains a piece of the key: Umbra's B+-tree stores
  there "the prefix of a key [...] as unsigned integer", the B2-tree "a
  substring of each key since the prefixes are already part of the embedded
  tree", called the *infix* (p. 18). A range of 32 slots is six cache lines.
- Space (Section 3.5 and Table 3): the embedded tree takes 0.48% (Wikipedia
  titles), 0.49% (random strings), 0.52% (URLs) of the page. "the prefix of
  each key within the same search range does not have to be stored, the
  B2-tree may therefore also be used to compress the stored keys" is stated
  as a possibility (p. 17); [I] the evaluated implementation stores whole
  keys.
- The paper describes the string B-tree in its related work (p. 13) [P]:
  "composed of Patricia tries [21] as internal nodes where each Patricia
  trie node stores only the branching character [...] searches within the
  String B-Tree have to progress optimistically". It is not measured.

**Experiments** (Section 4, pp. 18-21) [P]. Intel Core i9 7900X, 128 GB,
GCC 9.3, everything in memory, 64 KiB pages. **The only competitor is
Umbra's B+-tree** ("our only reference due to the lack of any other page
based index structure capable of running on top of Umbra's buffer manager"),
which "uses some commonly known optimizations like the choice of the
smallest possible separator within the neighborhood of separators around the
middle of each page, and a data locality optimization where the first bytes
of each key are stored within its corresponding entry in the indirection
vector [27-29]" (the references are Graefe 2011, Graefe and Larson 2001,
Graefe 2006). **It is therefore not a comparison with a prefix B-tree in the
sense of Bayer and Unterauer**: the word "truncation" does not occur in the
paper [P, text search], and the explanation of the results, "Keys in the URL
and Wikipedia datasets often share large common prefixes, discriminative
bits are therefore often not part of the integer field within the
indirection vector of Umbra's B+-tree" (p. 19), implies [I] that the
baseline takes the key bytes of the slot from the beginning of the key, not
after a page-wide common prefix.

- Datasets (Table 1): Wikipedia titles (48,454,094 strings, average length
  21.82, median 17), random strings (30,000,000, average 68.00), URLs
  (6,393,703, average 62.14, median 59, maximum 343).
- Lookups (Figure 4a; plot): about 0.98 against 0.57 million per second on
  Wikipedia titles, 1.15 against 0.88 on random strings, 1.12 against 0.63
  on URLs. Text: "B2-tree's lookup throughput is roughly twice as high as
  that of its direct competitor"; the gap is smaller on random strings.
- Inserts (Figure 4b; plot): about 0.68 against 0.35 (Wikipedia), 0.85
  against 0.53 (random), 0.83 against 0.42 (URLs) million per second.
- Counters per lookup (Table 2) [P]: URLs, 1839 against 3382 instructions,
  45.69 against 79.58 L1 data misses, 11.35 against 28.88 last-level cache
  misses, 22.74 against 16.15 branch misses; Wikipedia, 1593 against 3147
  instructions and 13.84 against 30.82 last-level misses; random, 1402
  against 2519 and 10.17 against 20.02.
- Concurrency (Figure 5): 95% lookups and 5% inserts with up to 20 threads
  under optimistic lock coupling; the B2-tree is ahead by a factor between
  about 1.2 and 1.7 (plot). With part of the pages swapped out the two
  structures converge (Figure 6).
- No hashing or fingerprints in the pages [P, text search: "hash" occurs
  once, about buffer managers].

## 5. Sun, Gao, Ballijepalli, Wang, "An evaluation of B-tree compression techniques" (`EvaluationBTrees.pdf`)

**Version** [P, first page]: "The VLDB Journal (2026) 35:4,
https://doi.org/10.1007/s00778-025-00950-8"; Sikang Sun, Chuqing Gao, Shreya
Ballijepalli, Jianguo Wang (Purdue University); "Received: 12 November 2024
/ Revised: 24 October 2025 / Accepted: 27 October 2025 / Published online:
28 November 2025"; 24 pages; open access (CC BY 4.0). "This article is an
extended version of the conference paper presented at SIGMOD'24 [30] [...]
In this version, we present new contributions to explore techniques
(including prefix vector representation, unrolled binary search, and key
normalization) for further improving Head+Tail compression (in Section 6).
We estimate the new contribution to be at least 30%" (p. 2).

**Entry.** `Sun2026Evaluation` in `practical-indexes.bib` agrees with the
PDF (volume 35, article 4, year 2026). The issue number is not printed;
`pages = {4:1--4:24}` would match the style of `Gao2024Revisiting`. Not in
`biblio.bib`.

**Sentences of `dynzfast.tex`.**

- Lines 572-574: "in a recent comparison of seven techniques for compressing
  the keys of a B-tree, the combination of these two is reported to be the
  fastest, answering point queries from $25$\% to $120$\% faster than a
  B-tree storing whole keys~\cite{Gao2024Revisiting}." Verdict: **correct**;
  the journal version repeats the statement verbatim (Section 8.1, p. 21:
  "It achieves a 25% ~ 120% performance improvement over the uncompressed
  B-tree across different datasets"). The journal version can be cited too
  (edit E2).
- Lines 574-576 (key bytes in the entries, "poor man's normalized keys" of
  Graefe and Larson). Verdict: **correct**; this paper provides the
  measurement that the 2001 papers lack (edit E3).

**What the paper says.**

- Schemes (Section 2) [P]: the same seven as the SIGMOD paper: Tail, Head,
  Head+Tail (all attributed to Bayer and Unterauer, its [18]), IBM DB2
  (several prefixes per page), MongoDB WiredTiger and MySQL MyISAM (each key
  stores "only the suffix key bytes and the number of prefix bytes common
  with the previous key"; WiredTiger searches by binary search with key
  instantiation, MyISAM by sequential scan), and PkB.
- Head compression **as implemented here uses the bounds** (Section 2.2,
  p. 3) [P]: "for each node, we identify the largest lower bound and
  smallest upper bound from the parent nodes, and the common prefix between
  these bounds determines the prefix of the key. [...] Note that we do not
  calculate the prefix solely based on the current keys in the node. This
  ensures that adding new keys to a B-tree node does not need the
  re-computation of the prefix. This prefix is stored once per node, and all
  the keys in the node store only the suffix bytes."
- Tail compression (Section 2.1) [P]: the separator is the longest common
  prefix of the two keys at the split point plus one character; the split
  point is chosen in a range around the middle, here of radius 1/6 of the
  node (Section 3.3 and Figure 8).
- PkB at byte level (Section 2.7 and Figure 7, p. 5) [P]: for the keys abcd,
  abefg, abexyz, bcdmn, bcdxyz the entries are "0: ab | 2: ef | 3: xy | 0:
  bc | 3: xy", each with a pointer to the full key: offset of the first
  differing byte from the preceding key and l = 2 bytes from there. "The
  search algorithm is designed such that it requires at most one pointer
  dereference per node." SAP HANA's CPB+-tree is said to be a variant
  (reference [22] is a blog post).
- Setup [P]: Xeon Gold 6330 at 2.00 GHz, 128 GB, GCC 11.4.0 with O3; no
  values stored; defaults (Table 2): 512-byte pages in memory, 4096-byte
  pages on disk, 100M random keys of 32 bytes over 10 symbols; 1 KB pages
  for TPC-H and 4 KB for WEBSPAM.
- Real datasets (Table 1) [P]: TPC-H LINEITEM as one string (60M keys, 96 to
  156 bytes, average 129.58), WEBSPAM-UK2007 URLs (25M, 16 to 2047, average
  112.33), WikiTitles (25M, 1 to 255, average 19.34), MemeTracker URLs
  (5.5M, 8 bytes to 21.2 KB, average 75.90). The same as in the SIGMOD
  paper.
- Headline results (Section 8.1, pp. 21-22) [P]. Lookups: Head+Tail is 25%
  to 120% faster than the uncompressed B-tree, "21.7% ~ 90.2% faster than
  DB2", "129% ~ 684% faster than MongoDB WiredTiger", "7.3% ~ 139% faster
  than MySQL MyISAM". Inserts: "a 10% ~ 30% performance improvement over the
  uncompressed B-tree"; "only the Head, Tail, and Head+Tail Compression
  techniques outperform uncompressed B-tree in insertion". Space: "Head+Tail
  and WiredTiger are the top two candidates, achieving a compression ratio
  ranging from 1.2X ~ 3.5X"; "for real-world datasets where keys have
  certain semantics, such as names, sentences, and URLs, WiredTiger and
  MyISAM are the most space-efficient". Recommendation (Section 8.2):
  Head+Tail, plus the prefix vector representation.
- Figures 19-22 (real datasets) look identical to those of the SIGMOD paper
  [I]; the readings recorded in `verified-btrees-hashing.md` (section 3)
  apply. Additional plot readings: compression ratio on WEBSPAM about 2.4
  (Head), 2.6 (Head+Tail), 3.5 (WiredTiger), 3.3 (MyISAM), 2.0 (DB2), below
  1 (PkB); inserts on WEBSPAM and WikiTitles show the uncompressed B-tree at
  the level of Head+Tail or slightly above (about 0.32 against 0.30, and
  1.45 against 1.38, million per second), which does not agree with "10% ~
  30%" on those two datasets [I, from the plots].
- On the default synthetic set the figures differ from the SIGMOD paper:
  Head+Tail "surpassing the uncompressed B-tree by 48.2%", Head +2%, Tail
  +23.5% (p. 7); the SIGMOD paper has 40.7%, 5.7%, 27.2% [S:
  `verified-btrees-hashing.md`]. The experiments were rerun.
- Why URLs are slow in a plain B-tree (Section 5.4 and Table 6) [P]: "From
  TPC-H to MemeTracker, the average comparison length changed from 8.92 to
  25.67, resulting in an increase in proportion of comparison time in the
  total query time from 62.5% to 71.2%".

**What is new with respect to `Gao2024Revisiting`: Section 6** (pp. 14-21).

- Source [P]: "Lomet documented three specific optimization techniques for
  B-tree compression in [46]: prefix vector representation, unrolled binary
  search, and key normalization [...] that paper is a
  personal-experience-sharing style article that does not provide
  experimental results". [46] is Lomet, SIGMOD Record 30(3):64-69, 2001 (not
  read). Graefe and Larson are cited only in the introduction.
- **Prefix vector representation** (Section 6.1) [P]: "partition each key
  into two parts [...]: a fixed-length part and a variable-length part. All
  fixed-length parts go into the key prefix vector representation [...] The
  length of the fixed-length key can be configured to two, four, or eight
  bytes. Binary search is applied on the fixed-length vector first, and only
  when the comparison result is equal do we need to traverse the suffix".
  Figure 26 shows the head stored next to the key pointer in the metadata
  (slot). [I] This is the poor man's normalized key of Graefe and Larson and
  the "heads" of Mueller et al.
- Results with 4 bytes and 1 KB pages (Table 7) [P]: search throughput, in
  million operations per second and with the gain over the same scheme
  without heads: no compression 0.729 (+9.5%), Head 1.008 (+36.8%), Tail
  0.872 (+7.3%), Head+Tail 1.139 (+26.1%); inserts: 0.745 (+13.7%), 1.026
  (+37.2%), 0.841 (+6.5%), 1.137 (+27.6%). "The most significant performance
  improvement is seen in Head Compression [...] Head Compression removes the
  common prefixes within a node, thus giving the prefix vector
  representation a greater chance to exit the search during prefix
  comparisons." Head size (Table 8, Head+Tail): search 0.903 without heads,
  1.0632 with 2 bytes, 1.139 with 4, 1.090 with 8. The heads help "for all
  page sizes up until 16KB" (p. 16).
- [I] These measurements are on the synthetic keys: the section refers to
  the "experimental setting in Section 4.1" and its figures vary the number
  of keys and the key length; the real string datasets are not used in
  Section 6.
- Unrolled binary search (Shar's algorithm, Section 6.2): slower, -5.92% on
  search and -10.3% on inserts (Table 9). Key normalization (word-wise
  comparison with BSWAP, Section 6.3): at most +4.6% and +2.9%; full
  normalization increases the number of nodes by 7.3% (Table 11).
- Related work (p. 21) [P]: "Bumbulis and Bowman developed a compact B-tree
  [23], which uses a local Patricia tree (a variant of trie) to represent
  the keys inside each node instead of using an array" ([23]: "A Compact
  B-tree", SIGMOD 2002, pp. 533-541; not read). This is a second lead, after
  Bit Trees, to Patricia-style nodes in a database B-tree. The B2-tree is
  cited as a B-tree using "certain forms of head and tail compression".
- Not in the paper [P, text search]: fingerprints, hashing, string B-trees.

## 6. Askitis, Sinha, "Engineering scalable, cache and space efficient tries for strings" (`HatTrieVLDB.pdf`)

**Version** [P, first page]: "The VLDB Journal (2010) 19:633-660, DOI
10.1007/s00778-010-0183-9"; "Received: 4 April 2008 / Revised: 22 March 2010
/ Accepted: 27 March 2010 / Published online: 4 May 2010"; footnote: "A
preliminary version of this paper appeared in 'HAT-trie: A Cache-conscious
Trie-based Data Structure for Strings', Askitis and Sinha, Proceedings of
ACSC, Ballarat Australia, pg. 97-105, 2007".

**Entry.** `Askitis2010Engineering` in `practical-indexes.bib` is correct
(the issue number 5 is not printed); the comment "Full text not read" can be
replaced by "first pages and experimental design read". Not in `biblio.bib`.

**Sentences of `dynzfast.tex`.** None: neither the HAT-trie nor burst tries
are cited or mentioned. No edit proposed.

**What the paper says** (it changes nothing in the account of the ACSC paper
in `verified-indexes.md`, section 7).

- Abstract [P]: the HAT-trie "carefully combines a trie with a hash table,
  creating a variant of burst trie"; "the HAT-trie - and two novel variants
  engineered to achieve further space-efficiency - is currently the leading
  in-memory trie-based data structure offering rapid, compact, and scalable
  storage and retrieval of variable-length strings."
- New with respect to the conference paper (p. 634) [P]: three variants, the
  pure HAT-trie, the hybrid HAT-trie (B-trie splitting) and the HAT+-trie,
  "which explores an adaptive strategy that converts containers from single
  dynamic arrays to array hash tables based on load"; an analysis of the
  expected cache misses; more machines and competitors (Judy trie,
  red-black and splay trees).
- Order [P]: "Containers are internally unsorted, but they can be accessed
  in lexicographic order [...] containers can be rapidly sorted on demand"
  (p. 633). The experiments measure "the amount of time and space required
  for construction and self-search" (Section 8); [I] no ordered operation
  (successor, range) is measured.
- Datasets (Table 6) [P]: `distinct` (28,772,169 strings, average length
  9.59), `trec` (612,219 distinct, 177,999,203 occurrences), `urls`
  (1,289,459 distinct, 9,999,425 occurrences, average length 30.93, 46.62 MB
  of distinct strings), `random` (50,000,000 strings of length 10). Primary
  machine: 2.8 GHz Pentium IV. The counts of `urls` differ slightly from
  those of the conference paper.
- Claims (p. 635) [P]: "the HAT-tries were up to 80% faster than the
  standard-chain burst trie while simultaneously requiring around 70% less
  space"; "Compared to the array burst trie, the HAT-tries were up to 50%
  faster to build and search but were not as space-efficient"; on 50
  million random strings "10-60 times faster to build and search than the
  array burst trie". Containers hold 8,192 to 16,384 strings.
- No x-fast, y-fast or z-fast tries, no van Emde Boas trees [P, text search:
  none of these strings occurs].

## 7. Gagie, Gawrychowski, Karkkainen, Nekrich, Puglisi, "A faster grammar-based self-index" (`FasterGrammar.pdf`)

**Version** [P, first page]: "A.-H. Dediu and C. Martin-Vide (Eds.): LATA
2012, LNCS 7183, pp. 240-251, 2012. (c) Springer-Verlag Berlin Heidelberg
2012". It is the proceedings paper, 12 pages, with a reference list of 23
items.

**Entry.** `Gagie2012Faster` in `biblio.bib` describes this paper correctly
but lacks the volume (7183) and the editors; a completed entry is given at
the end. **It is not cited anywhere in `dynzfast.tex`** [P: text search for
the key], so the entry is at present unused.

**Sentences of `dynzfast.tex`.**

- Lines 598-601: "in text indices based on Lempel--Ziv parsings, grammars,
  and other forms of dictionary compression, to locate in time logarithmic
  in the length of a pattern the node of a compacted trie of phrase
  boundaries associated with each way of splitting the
  pattern~\cite{Gagie2014LZ77,Bille2018Time,...}". Verdict: **correct**; the
  first citation must remain the LATIN 2014 paper.

**Answer to the key question: the LATA 2012 proceedings paper does not use
fat binary search, z-fast tries or Karp-Rabin fingerprints, and does not
cite any paper of Belazzougui et al.**

- [P, text search of the whole file] The strings "fingerprint", "Karp",
  "Rabin", "Belazzougui", "Vigna", "z-fast", "binary search", "hash", "weak
  prefix" do not occur.
- [P, reference list, pp. 250-251] The 23 references do not include the SODA
  2009, ESA 2010 or SPIRE 2010 papers, nor Karp and Rabin, nor Ferragina's
  weak prefix search.
- [P, abstract and Theorem 4, p. 247] The result is "given a balanced
  straight-line program for a string S[1..n] whose LZ77 parse consists of z
  phrases, we can add O(z log log z) words and obtain a compressed
  self-index for S such that, given a pattern P[1..m], we can list the occ
  occurrences of P in S in O(m^2 + (m + occ) log log n) time."
- [P, Section 3, pp. 244-246] The m^2 term comes from searching two plain
  Patricia trees (reversed phrases, suffixes at phrase boundaries) from the
  root for each of the m splits of the pattern: "we first search for the
  reverse of P[1..i] in the Patricia tree for the reversed phrases and search
  for P[i + 1..m] in the Patricia tree for the suffixes, then use our data
  structure from Corollary 3 to verify that the characters omitted from the
  edge labels match those in the corresponding positions in P. This takes
  O(m + log log n) time for each choice of i, or O(m^2 + m log log n) time
  in total." The verification extracts text through bookmarks on the
  grammar (Theorem 2 and Corollary 3), deterministically.
- The rest of the paper (Section 4, "Postscript") is a self-index for
  relative Lempel-Ziv based on FM-indexes; nothing related.

So the statement of the LATIN 2014 paper (p. 732: the previous paper attains
"O(m^2 + (m + occ) log log n) time", and "Karp-Rabin fingerprinting, fat
binary search and bookmarked fingerprinting" are "the contribution of this
paper"; quoted in `verified-static-uses.md`) is accurate, and the note in
`static-and-uses.md` that describes `Gagie2012Faster` as the first use was
based on the later arXiv version (1109.3954v6), not on this paper. What was
marked [I] in `verified-static-uses.md` is now [P].

## 8. Yu, Belazzougui, Qian, Zhang, "A Concise forwarding information base for scalable and fast name lookups" (`ConciseForwarding.pdf`)

**Version** [P]: title "A Concise Forwarding Information Base for Scalable
and Fast Name Lookups"; Ye Yu (University of Kentucky), Djamal Belazzougui
(CERIST), Chen Qian (University of California, Santa Cruz), Qin Zhang
(Indiana University Bloomington); footer "978-1-5090-6501-1/17/$31.00 (c)
2017 IEEE"; the acknowledgement thanks "the anonymous ICNP reviewers". The
conference name, the page numbers and the DOI are not printed in this copy;
the DOI 10.1109/ICNP.2017.8117530 is the one in `TO-DOWNLOAD.md`.

**Entry.** New: `Yu2017Concise` (at the end). The journal version is
`Yu2018Memory` in `practical-indexes.bib` (not read).

**Sentences of `dynzfast.tex`.** None.

**Does it use z-fast tries? No: it merely cites the SODA 2009 paper, and
not for the z-fast trie.**

- The only citation is in the introduction (p. 2) [P]: "Concise is built on
  a new network algorithm named Othello Hashing. Othello applies the
  theoretical studies on minimal perfect hashing [17], [6]", where [6] is
  "D. Belazzougui, P. Boldi, R. Pagh, and S. Vigna. Monotone minimal perfect
  hashing: searching a sorted table with O (1) accesses. In Proc. of ACM
  SODA [...] 2009" and [17] is Majewski, Wormald, Havas and Czech.
- The structure (Section IV) [P]: Othello is a static function in the style
  of MWHC and Bloomier filters: two hash functions, a bipartite acyclic
  graph, two bitmaps, tau(k) = a[h_a(k)] xor b[h_b(k)]; it classifies flat
  names into forwarding actions with two memory reads, and "does not support
  IP prefix matching". No trie, no prefix search, no order [P, text search:
  "trie" occurs only in the title of the Poptrie reference].

No item for the list "Uses of z-fast tries".

## 9. Golan, Porat, "Real-time streaming multi-pattern search for constant alphabet" (`RealTimeStreaming.pdf`)

**Version** [P, first page]: Shay Golan and Ely Porat (Bar Ilan University);
"25th Annual European Symposium on Algorithms (ESA 2017). Editors: Kirk
Pruhs and Christian Sohler; Article No. 41; pp. 41:1-41:15. Leibniz
International Proceedings in Informatics"; DOI 10.4230/LIPIcs.ESA.2017.41.
The volume number of LIPIcs (87) is not printed on the page [G]. Proofs are
deferred: "The proof of this lemma and other lemmas will appear in the final
version of this paper".

**Entry.** New: `Golan2017Real` (at the end).

**Sentences of `dynzfast.tex`.** None cites it. It bears on line 608: "in
streaming algorithms for dictionary
matching~\cite{Clifford2015Dictionary,Gawrychowski2022Streaming}", which
remains correct.

**Does it use z-fast tries? Not as such: it describes a structure of its own
"similarly to" fat binary search, citing the SODA 2009 paper.** The
expression "z-fast" does not occur [P, text search].

- The only citation (Section 4, "Short Patterns", p. 41:7) [P]:
  "Intuitively, at each position that is a multiple of log m the algorithm
  performs kind of a binary search for the longest text suffix which is a
  string in Heads_[log m,2 log m)(D_1), similarly to the fat binary search of
  Belazzougui et al. [8]", where [8] is the SODA 2009 paper.
- What they build (pp. 41:7-41:8) [P]: a perfect hash table on the
  fingerprints of selected *suffixes* of the heads. For a head P the suffix
  lengths stored are "exactly the lengths whose binary representation is the
  same as the binary representation of |P|, except for some suffix of the
  representation that is replaced by zeros" (the set Delta_|P|); this gives
  O(d log^2 m) strings, "which is too much". They then define a *suffixes
  tree* by the parent relation of the binary search, and compress it: "For
  any string S which has only one child, the algorithm shrinks the path from
  S to its first descendant S' that has at least two children", storing with
  S the length |child(S)|. The query starts from length 0 and adds
  decreasing powers of two, probing the hash table with the fingerprint of
  the text suffix of that length; at a node with a single child it probes
  the length of the child instead. Space O(d log m) words, O(log m) time per
  search.
- [I] In our terms: the lengths in Delta_|P| are the marker positions of
  Waldvogel et al. (clearing the lowest bits of the length, as recalled at
  lines 488-493 of `dynzfast.tex`), on the trie of the reversed heads; the
  compression of unary paths brings the number of entries down to one per
  branching node or head, which is what handles achieve in a z-map. The
  result is a relative of the z-fast trie obtained independently from the
  description of fat binary search, with Karp-Rabin-style fingerprints as
  keys of the hash table and Monte Carlo correctness ("we assume that
  fingerprints never give false positives").
- The rest of the paper (long patterns, Section 5) uses periodicity and AVL
  trees of expected fingerprints, not binary search on lengths.

By the criterion of the brief (an item only if it really uses a z-fast
trie): **no new item**. If the authors want completeness for the streaming
line, a qualified mention can be attached to the existing item (edit E7,
optional).

## 10. Orlandi, Venturini, "Space-efficient substring occurrence estimation" (`SubstringOccurrence.pdf`)

**Version** [P, first page]: "Algorithmica (2016) 74:65-90, DOI
10.1007/s00453-014-9936-y"; "Received: 21 November 2013 / Accepted: 26
August 2014 / Published online: 10 September 2014"; "The work is an extended
version of the paper appeared in the Proceedings of the 30th ACM
SIGMOD-SIGACT-SIGART Symposium on Principles of Database Systems (PODS)
2011". The issue number (1) is not printed.

**Entry.** New: `Orlandi2016Space` (at the end).

**Sentences of `dynzfast.tex`.** None. It bears on lines 605-607 (uses
through weak prefix search), which need no change.

**Does it use z-fast tries or weak prefix search? No: it describes weak
prefix search as an existing alternative that does not solve its problem.**
"z-fast" does not occur [P, text search].

- The only passage (Section 2.5, p. 73) [P], after a pruned Patricia trie
  searched by blind search: "A similar solution resorts to a data structure
  presented by Belazzougui et al. [3]. Their solution solves via (perfect)
  hash functions a problem somehow related to ours, called Weak Prefix
  Search. [...] Their main solution needs just O(p log(sigma)/w + log p +
  log log sigma) time and O(v log(L log sigma)) bits of space [...] We can
  use their data structure to index the set of suffixes S, so that we can
  search P[1, p] and report its number of occurrences multiplied by l. Since
  in our case L = Theta(n), the index requires [...] O(n log n / l) bits of
  space. As in the case of pruned Patricia trie, the answer is arbitrary
  when P is not prefix of any suffix in S (i.e., it occurs less that l
  times). Hence, this solution improves the time complexity of the previous
  approach but retains the same important drawback." [3] is the ESA 2010
  paper; the next sentence mentions the cache-oblivious solution of
  Ferragina and Venturini ([13]).
- Their own indexes (APX_l in Section 4, CPST_l in Section 5) do not use it,
  and the experiments (Section 6) compare FM-index, APPROX-l, PST-l and
  CPST-l only [P].

No item for the list: the existing clause "as a compact index of the factors
of a text~\cite{Belazzougui2015Improved}" is not affected.

---

## Attribution table

"B&U" is Bayer and Unterauer 1977; "Graefe 2011" is known through
`verified-btrees-hashing.md` and is marked [S] when not re-read here.
Performance is on strings unless stated otherwise.

| Idea | Who and where | Performance reported | Mark |
|---|---|---|---|
| Short separators (suffix truncation, "tail compression") | B&U, Section 1: *simple prefix B-trees*. Predecessor acknowledged: rear compression (Knuth; Wagner 1973). Graefe 2024, p. 185: only at leaf splits. | B&U, Table V: CPU time as a B*-tree, 20-25% fewer disk accesses for trees of 400-800 pages (words over 13 symbols, n <= 10000). Sun et al.: Tail alone +23.5% lookups on synthetic keys, no gain on URLs (plot). | [P] |
| Choice of the split point to shorten the separator | B&U, p. 13: *split interval*, for leaves and (choosing among existing separators) for branch nodes. Graefe 2024, p. 185: 40%-60%. Sun et al.: radius 1/6. | Not analysed in B&U ("an open problem", p. 16). Sun et al., Figure 8: little sensitivity. | [P] |
| Prefix truncation derived from the bounds of the page (fence keys) | B&U, Section 4: *prefix B-trees*; prefix kappa(P) computed from the separators of the ancestors during the descent, **not stored in the page**. Stored once per node and defined by the fence keys: Graefe 2011, p. 240 [S]; implemented so by Gao et al. and Sun et al. (Section 2.2). | B&U: 50-100% more CPU time than simple prefix B-trees, about 2% fewer disk accesses. Sun et al.: Head alone roughly doubles lookups on WEBSPAM URLs (0.4 to 0.9 Mops/s, plot) and gives +2% on random keys; Head+Tail 25%-120% over uncompressed. | [P] |
| Prefix truncation derived from the keys actually in the page | B&U, Note on p. 20 ("factor out the largest common prefix of the keys or separators actually stored in a node"), with its cost on updates. Graefe 2011, Section 3.5 ("analyzes the keys in a B-tree node") [S]. Mueller et al. 2025 [S: `practical-indexes.md`, 4.3]. | Not measured separately from the previous row in the papers read here. | [P] for B&U |
| Key-to-key prefix truncation (delta coding in the page) | WiredTiger, MyISAM, as described by Sun et al., Sections 2.5-2.6; Graefe 2024, p. 234: compresses more than page-wide truncation but prevents binary search. | Sun et al.: best compression on real strings (up to about 3.5 on URLs, plot); WiredTiger 129%-684% and MyISAM 7.3%-139% slower than Head+Tail on lookups. | [P] |
| Key heads in the slot (poor man's normalized keys, prefix vector, heads) | From sorting (AlphaSort 1995) to B-trees: Graefe and Larson 2001 [S: `verified-btrees-hashing.md`] and Lomet 2001 (as attributed by Sun et al., p. 15, and cited by Graefe 2024, p. 180; not read). Graefe 2024, pp. 180-181 and 232. Umbra's B+-tree and the "infix" of the B2-tree (Schmeisser et al., pp. 18-19). "Heads" of 4 bytes: Mueller et al. [S]. | Sun et al., Tables 7-8 (synthetic keys, 1 KB pages): 4-byte heads give +26.1% lookups and +27.6% inserts on Head+Tail, +36.8% on Head alone, +9.5% without compression; 2 and 8 bytes are worse than 4. B2-tree paper: heads taken from the start of the key are ineffective on URLs. | [P] |
| Partial keys relative to the preceding key (offset of first difference + l bits + pointer) | Bohannon, McIlroy, Rastogi 2001 (pkT- and pkB-trees), Section 3. | Their own tests are on random fixed-length byte strings only: better than B-trees with keys in the nodes for keys longer than 12-20 bytes (1.5M keys, a 296 MHz UltraSPARC II). On real strings (Sun et al., plots): slower than Head+Tail everywhere, compression ratio below 1. | [P] |
| Blind search on stored distinguishing positions, one key access per node | Ferguson, Bit Trees, CACM 1992, as described by Bohannon et al., p. 165 (leaves only, bit offsets only, "exactly one pointer dereference"); generalized by Bohannon et al. (procedure FindBitTree; "at most one" access). Offset-value coding (Conner 1977) for sequential search and merge: Graefe 2024, pp. 181, 216, 232. | None on strings in the papers read. | [P] for the descriptions; Ferguson and Conner not read |
| String B-tree style nodes (Patricia trie in the node) | Ferragina and Grossi 1999 (described, not measured, by Schmeisser et al., p. 13). In a database B-tree: Bumbulis and Bowman, "A compact B-tree", SIGMOD 2002, as described by Sun et al., p. 21 (not read). Not mentioned by B&U, Graefe 2024, Bohannon et al. | None in the papers read. | [P] for the mentions |
| Trie embedded in a page to narrow the search | B2-tree (Schmeisser et al. 2021/2022): decision nodes on single bytes, span nodes with stored bytes, range array. "Alternative approaches employ tries" (Graefe 2024, p. 180, citing only Fredkin). | About 2 times the lookups of Umbra's B+-tree on URLs and Wikipedia titles; 0.5% of the page. | [P] |
| Hash tags or fingerprints inside leaves | **In none of papers 1-5** (the word "fingerprint" occurs in none of them). Wormhole, Mueller et al., FB+-tree [S: `practical-indexes.md`, 1 and 4.3-4.4]. | - | [P] for the absence |
| Hash-organized leaves, hash table from keys to positions | **In none of papers 1-5.** Graefe 2024 has only "b-trees on hash values". The HAT-trie (paper 6) has hash tables as containers of a burst trie, unsorted. | HAT-trie: construction and self-search only. | [P] for the absence |

**Consequences for the attribution of the new structure** [I].

- *Pages without the common prefix*: Bayer and Unterauer, with the caveat
  that their prefix is that of the bounds; the prefix of the actual keys is
  their Note of p. 20 and Graefe's formulation.
- *Four-byte heads in sorted slots*: Graefe and Larson 2001 and Lomet 2001
  (both from AlphaSort); the measurement to cite is Sun et al. 2026, Section
  6.1, together with Mueller et al. 2025.
- *Blind directory of separators (lcp of adjacent separators and branching
  bytes)*: the string B-tree for the trie form; in array form, Ferguson's
  Bit Trees and the partial keys of Bohannon et al. Both should be cited;
  Ferguson's paper must be read first, since it is known here only through
  two paragraphs of Bohannon et al.
- *Hash table from keys to positions in a segment*: nothing in the B-tree
  papers read here; the attribution stays with Wormhole and the
  fingerprinting leaves of Mueller et al., which earlier reports cover.

---

## Proposed edits to dynzfast.tex

All new text is ASCII and uses the macros of the file. E1 corrects an
imprecision; E2 and E3 add citations; E4, E5 and E7 are optional; E6 is the
confirmation that nothing changes.

### E1. Lines 570-572: what Bayer and Unterauer introduce

Old:

```latex
B-trees for strings are still based on the \emph{prefix B-trees} of Bayer and Unterauer~\cite{Bayer1977Prefix}, which store 
only once the prefix common to all keys of a node, and use as separators the shortest strings separating the keys of adjacent
leaves (see~\cite[Section~3.5]{Graefe2011Modern}): in a recent comparison of seven techniques for compressing the keys of a B-tree, 
```

New:

```latex
B-trees for strings are still based on the two ideas of the \emph{prefix B-trees} of Bayer and Unterauer~\cite{Bayer1977Prefix}: the 
separators are the shortest strings separating the keys of adjacent leaves, and the point at which a leaf is split can be moved 
so as to shorten them; moreover, the prefix common to all the keys that a node may contain, which is determined by the separators 
delimiting the node in its ancestors, is not stored with the keys (current implementations store it once per 
node; see~\cite[Section~3.5]{Graefe2011Modern}): in a recent comparison of seven techniques for compressing the keys of a B-tree, 
```

Reason: in the paper the prefix kappa(P) is that of the lower and upper
bound of the page, is computed during the descent (abstract: "the prefixes
need not be fully stored, but are reconstructed as the tree is searched";
search step (1), p. 17) and is absent from the page; the prefix of the keys
actually stored is only a Note (p. 20). The old wording matches the phrase
"it suffices to store it once" (p. 17) but suggests a prefix stored in the
node and computed from its keys, which is the later formulation. The split
interval is their "important device" (p. 15) and was missing. The rest of
the sentence ("the combination of these two") still reads correctly.

### E2. Line 574: cite the journal version too

Old:

```latex
than a B-tree storing whole keys~\cite{Gao2024Revisiting}. Moreover, a few bytes of each key following the common prefix can be stored in the 
```

New:

```latex
than a B-tree storing whole keys~\cite{Gao2024Revisiting,Sun2026Evaluation}. Moreover, a few bytes of each key following the common prefix can be stored in the 
```

Reason: the journal version repeats "25% ~ 120%" (Section 8.1) and is the
source of E3.

### E3. Lines 576-577: the measurement of key heads

Old:

```latex
of Graefe and Larson~\cite{Graefe2001Btree}, who trace the idea back to cache-conscious sorting. Recent designs for main memory
add hash-based fingerprints, which speed up point queries~\cite{Muller2025BTrees,Chen2025FB}. 
```

New:

```latex
of Graefe and Larson~\cite{Graefe2001Btree}, who trace the idea back to cache-conscious sorting; with four bytes per key, they are 
reported to make a prefix B-tree a further $26$\% faster on point queries~\cite{Sun2026Evaluation}. Recent designs for main memory
add hash-based fingerprints, which speed up point queries~\cite{Muller2025BTrees,Chen2025FB}. 
```

Reason: Table 7 of Sun et al. (Head+Tail, search: 1.139 Mops/s, +26.1%; 1 KB
pages). The measurement is on synthetic keys [I]; if the authors prefer to
say so, write "on point queries on synthetic keys". Sun et al. call the
technique "prefix vector representation" and attribute it to Lomet (SIGMOD
Record 2001), not to Graefe and Larson: the two papers are of the same year
and both refer to AlphaSort, so the present attribution can stay; Lomet's
account has not been read.

### E4 (optional). After the sentence modified by E3: partial keys

Add, before "Recent designs for main memory":

```latex
The \emph{partial keys} of Bohannon, McIlroy and Rastogi~\cite{Bohannon2001Main} are instead relative to the preceding key of the node: an entry contains 
a pointer to the key, the position of the first bit (or byte) in which the key differs from the preceding one, and a few of the following bits, 
and a node is searched accessing at most one key. 
```

Reason: Section 3 of the paper (definition on p. 165; "Linear Search
Algorithm Requiring at most One Key Dereference", p. 168). Useful only if
the paper is going to mention blind directories; a comparison with the
nodes of a string B-tree ("as in the blind search of string
B-trees~\cite{FeGSBT}") would be my inference, not a statement of the
paper, which does not cite Ferragina and Grossi. The ancestor of the idea,
Ferguson's Bit Trees, should be cited only after reading it.

### E5 (optional). After E4 or E3: the B2-tree

```latex
The B$^2$-tree~\cite{Schmeisser2022B2} embeds in each page a trie on single bytes that restricts the binary search to about $32$ keys, and 
is reported to answer point queries on URLs about two times faster than a B$^+$-tree with short separators and key bytes in the entries of the nodes. 
```

Reason: pp. 17 and 19 of the journal version. The baseline is described on
p. 19; page-wide prefix truncation is not mentioned in the paper, which is
why the sentence does not say "prefix B-tree".

### E6. Line 600: no change

`\cite{Gagie2014LZ77,Bille2018Time,...}` is correct: the LATA 2012
proceedings paper contains no fat binary search (section 7). Do not add
`Gagie2012Faster`. The entry is unused: either delete it from `biblio.bib`
or complete it (below).

### E7 (optional, not recommended as a "use"). Line 608: Golan and Porat

Old:

```latex
\item in streaming algorithms for dictionary matching~\cite{Clifford2015Dictionary,Gawrychowski2022Streaming};
```

New:

```latex
\item in streaming algorithms for dictionary matching~\cite{Clifford2015Dictionary,Gawrychowski2022Streaming} (the algorithm of~\cite{Golan2017Real} 
uses instead a hash table of fingerprints probed by a binary search modelled on fat binary search);
```

Reason: Section 4 of the paper, "similarly to the fat binary search of
Belazzougui et al. [8]". It is a structure of its own, not a z-fast trie, so
the default is to leave line 608 as it is.

### No edit

- Yu et al. (ICNP 2017) and Orlandi and Venturini (Algorithmica 2016): no
  item, no citation.
- The HAT-trie and Graefe's 2024 survey: nothing in the text depends on
  them.
- Line 2470 ("it uses none of the techniques for strings discussed in
  Section~\ref{sec:related}"): still true after E1-E5.

---

## Entries for biblio.bib

Already in `biblio.bib` and correct: `Bayer1977Prefix`, `Gagie2014LZ77`,
`Graefe2011Modern`, `Graefe2001Btree`, `Gao2024Revisiting`.

Needed by E2 and E3 (from `practical-indexes.bib`, checked on the PDF; the
issue number is not printed):

```bibtex
@article{Sun2026Evaluation,
  author  = {Sikang Sun and Chuqing Gao and Shreya Ballijepalli and Jianguo Wang},
  title   = {An Evaluation of {B}-tree Compression Techniques},
  journal = {The VLDB Journal},
  volume  = {35},
  number  = {1},
  pages   = {4:1--4:24},
  year    = {2026},
  doi     = {10.1007/s00778-025-00950-8}
}
```

Needed by E4 (checked on the PDF: authors, title, venue, pages):

```bibtex
@inproceedings{Bohannon2001Main,
  author    = {Philip Bohannon and Peter McIlroy and Rajeev Rastogi},
  title     = {Main-Memory Index Structures with Fixed-Size Partial Keys},
  booktitle = {Proceedings of the 2001 {ACM} {SIGMOD} International Conference on Management of Data},
  year      = {2001},
  pages     = {163--174},
  publisher = {ACM},
  address   = {Santa Barbara, California, USA},
  doi       = {10.1145/375663.375681}
}
```

Needed by E5 (checked on the PDF; the issue number is not printed). The
conference version is completed from reference [10] of the journal paper and
has not been seen:

```bibtex
@article{Schmeisser2022B2,
  author  = {Josef Schmei{\ss}er and Maximilian E. Sch{\"u}le and Viktor Leis and Thomas Neumann and Alfons Kemper},
  title   = {{B$^2$}-Tree: Page-Based String Indexing in Concurrent Environments},
  journal = {Datenbank-Spektrum},
  volume  = {22},
  number  = {1},
  pages   = {11--22},
  year    = {2022},
  doi     = {10.1007/s13222-022-00409-y}
}

@inproceedings{Schmeisser2021B2,
  author    = {Josef Schmei{\ss}er and Maximilian E. Sch{\"u}le and Viktor Leis and Thomas Neumann and Alfons Kemper},
  title     = {{B$^2$}-Tree: Cache-Friendly String Indexing within {B}-Trees},
  booktitle = {Datenbanksysteme f{\"u}r Business, Technologie und Web ({BTW}~2021)},
  editor    = {Kai-Uwe Sattler and Melanie Herschel and Wolfgang Lehner},
  series    = {Lecture Notes in Informatics},
  pages     = {39--58},
  year      = {2021},
  publisher = {Gesellschaft f{\"u}r Informatik},
  doi       = {10.18420/btw2021-02}
}
```

(The first names of the editors are not printed in reference [10], which has
"Sattler K-U, Herschel M, Lehner W": they are from general knowledge.)

Needed by E7 only (first page of the PDF; the LIPIcs volume number is from
general knowledge):

```bibtex
@inproceedings{Golan2017Real,
  author    = {Shay Golan and Ely Porat},
  title     = {Real-Time Streaming Multi-Pattern Search for Constant Alphabet},
  booktitle = {25th Annual European Symposium on Algorithms ({ESA} 2017)},
  editor    = {Kirk Pruhs and Christian Sohler},
  series    = {Leibniz International Proceedings in Informatics ({LIPIcs})},
  volume    = {87},
  pages     = {41:1--41:15},
  publisher = {Schloss Dagstuhl -- Leibniz-Zentrum f{\"u}r Informatik},
  year      = {2017},
  doi       = {10.4230/LIPIcs.ESA.2017.41}
}
```

Completed version of the entry already in `biblio.bib` (volume and editors
from the first page), if it is kept:

```bibtex
@inproceedings{Gagie2012Faster,
  author    = {Travis Gagie and Pawe{\l} Gawrychowski and Juha K{\"a}rkk{\"a}inen and Yakov Nekrich and Simon J. Puglisi},
  title     = {A Faster Grammar-Based Self-index},
  booktitle = {Language and Automata Theory and Applications -- 6th International Conference, LATA 2012},
  editor    = {Adrian-Horia Dediu and Carlos Mart{\'\i}n-Vide},
  series    = {Lecture Notes in Computer Science},
  volume    = {7183},
  pages     = {240--251},
  publisher = {Springer},
  year      = {2012},
  doi       = {10.1007/978-3-642-28332-1_21}
}
```

(The PDF prints "A.-H. Dediu and C. Martin-Vide (Eds.)"; the first names are
from general knowledge.)

Not needed by any edit, verified for the record (they can stay in
`practical-indexes.bib`): `Graefe2024More` and `Askitis2010Engineering` are
correct as they are. New entries for papers 8 and 10, which are not to be
cited:

```bibtex
@inproceedings{Yu2017Concise,
  author    = {Ye Yu and Djamal Belazzougui and Chen Qian and Qin Zhang},
  title     = {A Concise Forwarding Information Base for Scalable and Fast Name Lookups},
  booktitle = {2017 {IEEE} 25th International Conference on Network Protocols ({ICNP})},
  pages     = {1--10},
  publisher = {IEEE},
  year      = {2017},
  doi       = {10.1109/ICNP.2017.8117530}
}

@article{Orlandi2016Space,
  author  = {Alessio Orlandi and Rossano Venturini},
  title   = {Space-Efficient Substring Occurrence Estimation},
  journal = {Algorithmica},
  volume  = {74},
  number  = {1},
  pages   = {65--90},
  year    = {2016},
  doi     = {10.1007/s00453-014-9936-y}
}
```

(`Yu2017Concise`: the conference name, the pages and the DOI are not printed
in the PDF; the DOI is that of `TO-DOWNLOAD.md`, the rest is unverified.
`Orlandi2016Space`: the issue number is not printed.)

Leads found in the reference lists, **not read**; data exactly as printed by
the citing paper, to be checked before use:

```bibtex
% As printed in Bohannon et al., reference [11].
@article{Ferguson1992Bit,
  author  = {D. Ferguson},
  title   = {Bit-tree: A Data Structure for Fast File Processing},
  journal = {Communications of the ACM},
  volume  = {35},
  number  = {6},
  pages   = {114--120},
  year    = {1992}
}

% As printed in Sun et al., reference [46], and Graefe 2024, reference [55].
@article{Lomet2001Evolution,
  author  = {David B. Lomet},
  title   = {The Evolution of Effective {B}-tree: Page Organization and Techniques: A Personal Account},
  journal = {SIGMOD Record},
  volume  = {30},
  number  = {3},
  pages   = {64--69},
  year    = {2001}
}

% As printed in Sun et al., reference [23].
@inproceedings{Bumbulis2002Compact,
  author    = {Peter Bumbulis and Ivan T. Bowman},
  title     = {A Compact {B}-tree},
  booktitle = {Proceedings of the 2002 {ACM} {SIGMOD} International Conference on Management of Data},
  pages     = {533--541},
  year      = {2002}
}

% As printed in Graefe 2024, reference [12].
@article{Conner1977Offset,
  author  = {W. M. Conner},
  title   = {Offset Value Coding},
  journal = {IBM Technical Disclosure Bulletin},
  volume  = {20},
  number  = {7},
  pages   = {2832--2837},
  year    = {1977}
}
```

(Ferguson's title is printed "Bit-tree a data structure for fast file
processing", without punctuation; the first names of Bumbulis and Bowman are
printed as initials, "Bumbulis, P., Bowman, I.T.".)

## What remains to download

- D. Ferguson, "Bit-tree: a data structure for fast file processing", CACM
  35(6), 1992: the array form of blind search, needed to attribute the blind
  directory.
- P. Bumbulis, I. T. Bowman, "A compact B-tree", SIGMOD 2002: Patricia trees
  inside B-tree nodes in a database system.
- D. B. Lomet, "The evolution of effective B-tree: page organization and
  techniques: a personal account", SIGMOD Record 30(3), 2001: the origin of
  key heads in slots according to Sun et al.
- The BTW 2021 version of the B2-tree only if it is to be cited instead of
  the journal version.
