# Java: Sux4J ZFastTrie, ZFastTrieDistributor, zuffix prototype

Sub-review commissioned on 2026-11-16 during the z-fast trie project review; report delivered 2026-11-16/17.
It was produced by a separate reviewing agent that read the material in full. Claims marked as simulated were
checked by that agent with throwaway scripts (copied to `../scripts/` where they survived); they were not all
independently re-verified. See `../REVIEW.md` for which claims were spot-checked.

## Report

All ten files exist and were read in full. I also read `Hashes.java:745-937` and grepped the ranker's return paths and the distributor test to check claims. Nothing was built or run, so every bug below comes from hand-tracing.

Aliases:
- `ZFT` = `/Users/vigna/git/Sux4J/src/it/unimi/dsi/sux4j/util/ZFastTrie.java`
- `ZT` = `/Users/vigna/git/Sux4J/test/it/unimi/dsi/sux4j/util/ZFastTrieTest.java`
- `SP` = `/Users/vigna/git/Sux4J/src/it/unimi/dsi/sux4j/test/ZFastTrieSpeedTest.java`
- `D` = `/Users/vigna/git/Sux4J/src/it/unimi/dsi/sux4j/mph/ZFastTrieDistributor.java`
- `M` = `/Users/vigna/git/Sux4J/src/it/unimi/dsi/sux4j/mph/ZFastTrieDistributorMonotoneMinimalPerfectHashFunction.java`
- `H` = `/Users/vigna/git/Sux4J/src/it/unimi/dsi/sux4j/mph/Hashes.java`
- `DT` = `/Users/vigna/git/Sux4J/test/it/unimi/dsi/sux4j/mph/ZFastTrieDistributorMonotoneMinimalPerfectHashFunctionTest.java`
- `ZA` = `/Users/vigna/git/zuffix/src/ZuffixArray.java`
- `CH` = `/Users/vigna/git/zuffix/src/CyclicHash.java`
- `FS` = `/Users/vigna/git/zuffix/src/FatSearchSpeed.java`
- `FC` = `/Users/vigna/git/zuffix/fatsearch.c`
- `ZAT` = `/Users/vigna/git/zuffix/test/ZuffixArrayTest.java`

## 1. ZFastTrie (verified by reading)

**Nodes.** `Node` holds `nameLength` (ZFT:544). `InternalNode` adds `extentLength`, `left`, `right`, `jumpLeft`, `jumpRight` and `reference`, a descendant leaf whose key supplies the extent bits (ZFT:627-637). `Leaf` holds `prev`/`next` (sorted list with sentinels), `key`, and a back-pointer to the one internal node borrowing its key (ZFT:688-695). Leaf versus internal is by class. Handle and jump lengths are recomputed, never stored; the root handle is always empty.

**Z-map** (ZFT:115-539). Parallel `long[] signature` and `InternalNode[] node`; power-of-two size, linear probing, empty = null node, doubling above 2/3 load, no shrinking. A slot holds a 63-bit signature plus a "duplicate" bit. Inserting flags every equal-signature entry on the probe path, so only the last stays unflagged. `find` compares handle bits for flagged entries only and accepts an unflagged one on signature plus handle-length equality (ZFT:248-258, 476-479). A true handle is therefore never missed.

**Hashing.** Murmur64A-style over 64-bit words, fixed seed 42; per-word states are saved once, so any prefix hash is O(1) (H:818-841, 913-937).

**Searches.**
- `fatBinarySearch` (ZFT:1607) returns the last hit: the parex or the exit node, or null when the root is the exit.
- `fatBinarySearchStack` (ZFT:1485) pushes every hit whose extent is shorter than the query.
- Each has an `Exact` twin that compares handles (ZFT:1530, 1652).
- `getExitNode` (ZFT:1339) returns (exit, lcp); `getParentExitNode` (ZFT:1388) returns (parex, exit, lcp) and leaves the 2-fat ancestors on the stack.

False positives are checked once, after the search: if the query's lcp with the top node's extent is below that node's handle length, the exact twin is rerun (ZFT:1351-1354, 1402-1411). The first probe is always the empty root handle.

**Updates.** Insertion splits the exit node at the lcp. If the cut is at or above the exit node's handle length (`cutLow`, ZFT:1157) and that node is internal, the new node inherits its map slot and the exit node is re-inserted; otherwise the new node gets a fresh slot (ZFT:1194-1203). Ancestor jumps are repaired by popping the stack (ZFT:957-1027). Deletion mirrors this (ZFT:1276-1311).

**API.** `add`, `remove`, `contains`, four predecessor/successor variants with `NavigableSet`-style aliases (ZFT:1721-1857), `isNonempty(l, u)` on [l, u) (ZFT:1867), bidirectional iterators, `first`, `last`. There is no prefix enumeration, rank or value; subset views throw. Keys are any `T` mapped by a `TransformationStrategy` to prefix-free bit vectors; `add` throws on a prefix violation (ZFT:1148, 1153). Serialization is a preorder stream of skip lengths and keys (ZFT:1980-1989).

**Inferences.**
- Checking only the top looks sound. A false positive at f means f lies in the exit node's skip interval or beyond its extent, so no true handle can be hit afterwards, and a verified top implies a clean stack.
- Java-only: transform and `BitVector` indirection, per-query copies, class-based node typing, sentinel leaves, sorted-set plumbing, and shift counts that wrap silently (`checkMask(-1)` on an empty query, ZFT:817, 1491).
- A port also needs: a node arena with indices, an explicit empty-slot marker, a reseedable hash, reusable scratch for hash state and stack, non-recursive dump and validation, a policy for keys that are prefixes of other keys, and a 256-way replacement for `getBoolean(lcp)`, `left`/`right` and the two jump pointers.

## 2. ZFastTrieDistributor and its MMPHF (verified)

A distributor maps each key of a sorted set to its bucket index, that is, its rank among the delimiters. The MMPHF returns `bucket << log2BucketSize` plus a stored offset (M:259).

- **Per key:** one "behaviour" bit (left or right of the exit node's subtree) in a `GOV3Function` (D:343-347, 448), and a `log2BucketSize`-bit offset in a second `GOV3Function` on the same `BucketedHashStore` (M:214-224); optionally a signature (M:236-239).
- **Per bucket:** the delimiter is the last key of each full bucket (D:218); the delimiter trie exists only during construction. Each internal node contributes two things:
  - one `GOV3Function` entry, handle → (extent signature of ⌈log log w⌉ + log b bits, extent length) (D:163-174, 261-266, 529);
  - three "ranker" strings in a `TwoStepsLcpMonotoneMinimalPerfectHashFunction`, with a `Rank9` vector marking which rankers are delimiters (D:465-520).
- **Query:** fat binary search over prefix lengths, each probe evaluating the static function on the prefix's hash (D:626-680). A hit needs f ≤ g ≤ |x| and a matching signature of x[0..g). The parex extent, the next bit and the behaviour bit select the ranker string (D:698-726).
- **Construction-time errors:** the search is replayed for every key and compared with the true parex length (D:544-560). The 32-bit hashes of failing keys go into an explicit `IntOpenHashSet`. Every key hitting that set has its correct length stored in a `corrections` function, consulted first (D:572-587, 629-633).
- **Bucket size:** ⌈10.5 + 4.05 ln ℓ + 2.43 ln(ln n + 1) + 2.43 ln(ln ℓ + 1)⌉ rounded down to a power of two, ℓ being the average length (M:204-205).
- **Space:** stated only as logged forecasts (D:590-598, M:228-232); there is no documented bound.
- **Non-keys:** arbitrary result; `containsKey` is always true (D:741), and the MMPHF clamps out-of-range values to −1 (M:262).

## 3. Zuffix prototype (verified)

- **Trie:** a suffix array over n+1 suffixes, a naive LCP array and the `up`/`down`/`next` child table (ZA:80-126). A node is an lcp-interval packed in a long. Its extent length comes from the child table (ZA:195-201) and its name length is `1 + max(lcp[i], lcp[j+1])` (ZA:350), so nothing is stored per node.
- **Z-map:** `Long2LongOpenHashMap` from the 64-bit hash of the handle (length in symbols) to the interval, internal nodes only; a build-time collision throws (ZA:217).
- **Search:** the same mask loop over symbol counts, with the root preset and hits filtered by extent ≥ f (ZA:253-298).
- **Mistakes:** the pattern is compared with the text at the node; an lcp below the *name* length triggers `findExact`, a plain top-down walk (ZA:358, 300-339).
- **Child selection:** a linear scan of `getChildren` on the symbol after the extent, then a second comparison against the child's extent (ZA:366-394).
- **Prefix hashes:** none. Each probe rehashes from scratch (ZA:278), so a query costs O(m log m). Inference: `CyclicHash`'s XOR-of-rotations form would allow O(1) prefix hashes from a table, but that is not implemented.
- `FC` and `FS` only benchmark plain versus 2-fattest-midpoint binary search on sorted integers.

## 4. Markers and admitted limitations

There are only two markers:
- `// TODO: These maxes should not be necessary` (D:707).
- `// ALERT: this should return NULL in certain cases` (ZA:263).

Comments admitting limitations:
- `setJumps` recomputes both sides where one suffices (ZFT:1300).
- "we could even reuse part of the stack" (ZFT:1890).
- Non-keys "can generate bizarre 3-hyperedges" (M:261).
- Probe statistics are commented out (SP:111).

## 5. Suspected bugs and performance problems

1. **Pred/succ on non-prefix-free queries** (high confidence, traced). If the query properly extends a stored key and its next bit is 0, all four of ZFT:1710, 1750, 1790, 1830 are off by one leaf. If it is a proper prefix of an extent, `strictSuccessor` and `weakPredecessor` are wrong. This is outside the documented contract, and the tests use only prefix-free transforms (ZT:526, 589).
2. **`isNonempty` on an empty trie** dereferences a null root (ZFT:1873 → 1347). High.
3. **`removeExisting`/`replaceExisting`** loop forever on an absent node; the `IllegalStateException` at ZFT:427 is unreachable. High.
4. **Distributor, root exit** (medium; not run).
   - A key that does not share the delimiters' common prefix gets parex length 0. The ranker is then queried with "" or "1", which are ranker keys only by accident (D:347, 698-726 versus 465-476).
   - The `Math.max(0, pos)` rescues the left case when the ranker returns −1; a right exit whose first bit is 0 is not rescued.
   - Candidate reproducer, untested: {"aa","ab","ac","ad","b"} with bucket size 2.
   - `DT` never reaches this case, and the closing self-check is compiled out (D:61, 607-617).
5. **`CyclicHash`** rotates by `i` mod 64 (CH:19), so the hash is unchanged by swapping symbols 64 positions apart. High (algebraic).
6. **`FS`** calls the searches with `from = left` and `to ≤ left/2` (FS:57-59), so the loops almost never iterate. High.
7. **Minor, high confidence:**
   - The `size()` test is vacuous (ZFT:771).
   - The assert at ZFT:1308 is vacuous because `root` was reassigned at 1265.
   - `first()`/`last()` return null when empty.
   - Size-1 `remove` uses `key.equals` (ZFT:1231).
   - ZA:395-397 is unreachable.
   - The oracle loop at ZAT:38 skips the last alignment.
   - The space assert at DT:124 is dead (loop bound at DT:115).
8. **Recursion** in `writeNode`/`readNode`/`visit` (ZFT:893, 1980, 2013) and `buildMap` (ZA:207) is as deep as the trie. Medium; tests reach depth 100 only.
9. **Hot-path costs** (verified):
   - Allocations per call at ZFT:1110-1131, 1693-1694, 1364, D:639, 686, 698, ZA:174-192.
   - `transform.toBitVector(key)` on every extent access (ZFT:665-667, 728-730).
   - Virtual `isInternal()` in spine loops (ZFT:591-601).
   - Each map probe reads two arrays and, on a signature match, the node (ZFT:248-253).

## 6. Testing

Reusable strategies are in the table. Gaps:
- The invariant checkers are compiled out (`ASSERTS = false`, ZFT:91).
- No test forces signature collisions, so the duplicate-flag and exact-fallback paths are effectively unexercised.
- There is no interleaved add/remove against an oracle.
- Negative `contains` results are not asserted (ZT:470).

## Technique table

| Technique | Where | What it does |
|---|---|---|
| Implicit handles | ZFT:639-647, 786-788 | Handle length is the 2-fattest of (name−1..extent], jump length is h + lowbit(h); both recomputed on demand |
| Borrowed extents | ZFT:637, 695, 1166-1168, 1283-1289 | Internal nodes store no bits; each points to a descendant leaf, kept bijective in O(1) per update |
| Signature + duplicate bit | ZFT:96-99, 248-258, 428, 476-479 | 63-bit signature; only colliding entries pay a handle comparison |
| Backward-shift deletion | ZFT:430-445 | No tombstones in the linear-probing map |
| In-place owner swap | ZFT:400-407, 1195, 1299 | When a handle moves to another node, only the slot's pointer changes |
| O(1) prefix hashes | H:818-841, 913-937 | Hash state saved per 64-bit word; prefix hash = state + partial word + length |
| Hash resumed from a shared prefix | H:863-897; ZFT:1197, 1297 | Hashes another string's handle starting from the query's state at the lcp |
| Mask-shift 2-fattest loop | ZFT:804-818, 1491-1514; D:637-675; ZA:268-293 | `f = b & mask` is 2-fattest iff `(a & mask) != f`; one shift per iteration, no msb |
| Verify once, then exact rerun | ZFT:1351-1354, 1402-1411 | One lcp with the top node replaces per-probe verification |
| 2-fat ancestor stack | ZFT:1425-1430, 1464-1469, 1575-1594 | "Lucky" parent test on the stack, else a completion search in a known-good interval |
| `cutLow` tests | ZFT:1157, 1278-1280 | Decide which node changes handle on split or merge; deletion uses a lowest-set-bit comparison |
| Jump-pointer spine descent | ZFT:591-601, 933-945 | Leftmost/rightmost leaf by following jump pointers; `setJumps` rebuilds one node's jumps from its descendants' |
| Range emptiness without reaching leaves | ZFT:1867-1900 | Descends by jumps to the first spine node with extent ≥ the bound's length, then compares once |
| One-pass jump rebuild on load | ZFT:2013-2101 | Preorder read restores map, references, leaf list and all jumps with direction-run stacks |
| Invariant checkers | ZFT:131-170, 839-925, 1437-1444 | Probe-cluster and duplicate-flag invariants; names, references, handles and jumps recomputed naively |
| One-bit signature hook | ZFT:97 | Commented-out mask that would force collisions for testing |
| Static z-map as a static function | D:163-174, 529, 646-669 | Handle → (extent signature, extent length); verification hashes x[0..g) |
| Behaviour bit | D:343-347, 448, 693 | One bit per key replaces storing exit-node extents |
| Ranking by strings | D:465-520, 698-726 | Three strings per internal node in an MMPHF plus a rank vector give the bucket index |
| Mistake whitelist | D:544-587, 629-633 | 32-bit hash set of failing keys plus a correction function |
| Bucket-size formula | M:204-205 | Chosen from average key length and n |
| Node = lcp-interval | ZA:195-201, 350, 404-406 | Name and extent lengths derived from the LCP array and child table |
| Root preset | ZA:262-266 | Skips the empty-handle probe |
| Name-length acceptance test | ZA:358 | Accepts any hit whose name is a prefix of the pattern |
| Byte-width statistics | ZA:132-168 | Measures how often lcp and child-table deltas exceed one byte |
| Reduced-width hash trials | ZAT:136-153 | Builds a 1M-symbol text with 37-bit hashes to see whether handles collide |
| Brute-force pattern oracle | ZAT:26-127 | Every pattern over a small alphabet, sticky-symbol generator, constant and all-distinct texts, patterns longer than the text |
| `TreeSet` oracle | ZT:520-634 | All four pred/succ variants over an exhaustive small universe and random bit strings; range emptiness on adjacent keys and single-bit-flip pairs |
| Shape extremes | ZT:303-348, 411-447 | Complete tries and caterpillars, inserted and removed in both orders |
| Removal orders at a root split | ZT:250-300 | Three keys, three removal orders, both mirror images |
| White-box search checks | ZT:372-388, 637-804 | Expected stack sizes, including an iteration where the 2-fattest test fails |
| Serialization round trip | ZT:57-65 | Applied after nearly every scenario |

## Brief given to the reviewer

You are helping with a technical review. Do NOT modify, build, or run anything; this is a read-and-report task. Read whole files, not excerpts.

## Background
The z-fast trie is a predecessor/prefix-search data structure for strings: a compacted binary trie plus a hash map (the "z-map") from "handles" to internal nodes. The handle of a node is the prefix of its extent whose length is the "2-fattest" number (most trailing zeros) in the node's skip interval (name length .. extent length]. Queries locate the exit node of a string x by "fat binary search" over prefix lengths, probing the z-map with hashes of prefixes of x, in O(log |x|) probes. Variants use signatures (hashes) instead of full handles in the map, which introduces false positives that must be detected and repaired.

I am reviewing every existing implementation in order to design NEW Rust implementations: one fully dynamic, one static, both operating on BYTE strings (lengths and handles measured in bytes, alphabet 256) rather than bit strings. I need to extract the best ideas from the existing code.

## Your job
Read these Java sources and report the design ideas and engineering techniques they contain, with `file:line` references:

1. `/Users/vigna/git/Sux4J/src/it/unimi/dsi/sux4j/util/ZFastTrie.java` (dynamic z-fast trie, ~2170 lines) — the main target.
2. `/Users/vigna/git/Sux4J/test/it/unimi/dsi/sux4j/util/ZFastTrieTest.java` and `/Users/vigna/git/Sux4J/src/it/unimi/dsi/sux4j/test/ZFastTrieSpeedTest.java`.
3. `/Users/vigna/git/Sux4J/src/it/unimi/dsi/sux4j/mph/ZFastTrieDistributor.java` and `ZFastTrieDistributorMonotoneMinimalPerfectHashFunction.java` in the same directory (a STATIC, succinct, probabilistic relative of the structure).
4. `/Users/vigna/git/zuffix/src/ZuffixArray.java`, `/Users/vigna/git/zuffix/src/CyclicHash.java`, `/Users/vigna/git/zuffix/src/FatSearchSpeed.java`, `/Users/vigna/git/zuffix/fatsearch.c`, `/Users/vigna/git/zuffix/test/ZuffixArrayTest.java` (a 2018 prototype applying the z-fast idea to suffix arrays over non-binary alphabets).

## What to report
For ZFastTrie.java:
- Node layout (fields, how leaves vs internal nodes are distinguished, reference/key pointers, jump pointers) and the z-map layout (what a slot stores, collision policy, how duplicate signatures are handled, resizing).
- Hashing: which hash, how prefix hashing is made O(1) after preprocessing, signature width.
- The search routines: names, what each returns, how/where false positives are detected and what the fallback is. Any optimisations over a textbook fat binary search (e.g. avoiding 2-fattest computation, reusing a stack of 2-fat ancestors, restricting the initial interval).
- Updates: how insertion and deletion maintain handles and jump pointers; cost drivers.
- Public API: which queries exist (contains, pred/succ, prefix, range, iterators, etc.), key abstraction (TransformationStrategy / BitVector), serialization.
- Anything present in Java but NOT obviously needed in a C port, and vice versa.

For ZFastTrieDistributor (+ the MMPHF built on it):
- Exactly what is stored per key / per bucket, what static functions or MPHFs back it, bucket size, claimed space, what "distributor" means, behaviour on keys not in the set, and how errors/false positives are handled at construction time.

For the zuffix Java prototype:
- How the trie is represented (suffix array + LCP? intervals?), what the z-map maps to what, how fat binary search is done over symbols rather than bits, how a child is selected at the end, how prefix hashes are precomputed, and how mistakes are detected.

Across everything:
- All TODO/FIXME/XXX comments and any comment admitting a limitation.
- Anything that looks like a bug, an unhandled corner case, or an obvious performance problem (per-query allocation, virtual dispatch in hot loops, etc.). Say how confident you are and why.
- Testing strategies in the test files worth reusing (invariant checkers, randomized cross-checks, corner cases covered).

## Output format
A structured report, at most ~1100 words plus one table mapping "technique → file:line → one-line description". Put concrete facts first; clearly separate what you verified by reading code from what you infer. If a file listed above does not exist, say so rather than guessing.
