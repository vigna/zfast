# C implementations (~/git/zfast, ~/svn/c/sux) and benchmarks

Sub-review commissioned on 2026-11-16 during the z-fast trie project review; report delivered 2026-11-16/17.
It was produced by a separate reviewing agent that read the material in full. Claims marked as simulated were
checked by that agent with throwaway scripts (copied to `../scripts/` where they survived); they were not all
independently re-verified. See `../REVIEW.md` for which claims were spot-checked.

## Report

# Review of the z-fast trie C code and its benchmarks

Nothing was built, run or modified in the three directories. `file:line` references are relative to `/Users/vigna/git/zfast/` unless marked 2017 (`/Users/vigna/svn/c/sux/`). Cache-miss counts in section 3 are inferences from the code and linear-probing formulas, not measurements.

## Headline findings

- **Two columns are swapped in both URL tables.** In `tab:urlsm` and `tab:urlsb`, the "red-black tree" column holds the `btree-*` file values and the "B-tree" column the `rbtree-*` values (section 6).
- **The text contradicts `tab:intbb`.** "Within 10% of a B-tree" (l. 1928) holds for one row of five; the ratios are 1.24, 1.08, 1.17, 1.16, 1.59.
- **"Single-shot allocation, no `free`" (l. 1914) is false for the z-fast programs.** The hash table starts at 64 slots and doubles with allocate-and-free, and the string version calls `calloc`/`free` on every operation.
- **The no-argument crash is not in the newer version.** All six drivers check `argc < 2`. It exists only in 2017 `zfasttrie.c:1729`, fixed by cd05aef.
- **The integer key shuffle is not uniform** (`zcommon.c:75-80`), so the "random" insertion order is skewed.
- **The data behind `tab:intbb` is not in git**, and its stderr format matches no committed `zcommon.c`.

## 1. Changes from 2017 to 2022

The first git commit (206c87f, 2021-03-04) is byte-identical to the 2017 snapshot for both files. The 114 commits up to 1a12fb5 (2022-04-12) are therefore the complete history.

- **Layout:** node sizes are unchanged. The integer field `key` became `extent` and moved first; globals were wrapped in `zfast_cstr`/`zfast_uint64` (0a149d5). Slot layout and growth rule are unchanged.
- **Hashing:** strings keep the Murmur2-derived prefix hash, now in `murmur.h`, with an optional XXH64 variant (c1dc417). Integers moved from the MurmurHash3 finaliser to Stafford's Mix13 (ae58005).
- **Search:** one `fat_binary_search` with `exact`/`stack`/`last_f` parameters became four specialised searches plus `complete_fat_binary_search_stack` (4368f22, c97925a, 12464e4, 905b606, 7cbbec9). `find()` now returns nodes and checks the handle length (245981a). The 2017 code special-cased the root; 2022 probes the table for it.
- **Key comparison:** 2017 validated a search with one lcp against the candidate child's key. 2022 computes lcp against the found node's extent, then `lcp_from` on the child (0b0d4ec, ea77647). `same_bits` became word-at-a-time (2e2db25, f950a5d). 976d887 stopped `lcp` reading the partial last word. Integers switched to direct key equality (bf7bccb).
- **Duplicates:** the duplicate-signature bit is unchanged. Duplicate generated integers were first patched by retrying `++x[i]` (57f9aee, bbce9e0), then excluded by construction (26b2f75, 9937aab).
- **Allocation:** `calloc` became `bigxcalloc`/`bigfree` with optional `mmap` + `MADV_HUGEPAGE` (de4ac5d, 9d81432). Baselines got pool and bump allocators (c6f62e1).
- **Driver:** 2017 strings used an unrelated mutated buffer for negatives and had no predecessor phase; integers were read from a file. 2022 has the same phases in all six programs (9829ee8). 27abb46 fixed a per-delete time divided by the wrong count.
- Two code commits (e0f9d12, 1a12fb5) postdate the benchmark commit 1e44375.

## 2. Layout and memory

| | Integers (`intzfasttrie.c:37-55`) | Strings (`zfasttrie.c:48-82`) |
|---|---|---|
| Internal node | 48 B: extent 8, two `int32_t` lengths 8, left/right 16, jumps 16 | 48 B: lengths 8, left/right 16, reference 8, jumps 16 |
| Leaf | 32 B, the struct up to `jump_left` (`:95-103`) | 48 B, same struct; union holds key pointer and an unused `key_length` |
| Slot | 8 B, bare `node *` | 16 B, 63-bit signature + duplicate bit, `node *` |
| Load factor | doubles when `length < 2*size` (`:277`), so 0.25–0.5 | same (`:368`) |
| Bytes per key | 80 + 16..32 = 96–112; 97.2 at n = 2^k·10^9 | 96 + 32..64 = 128–160; 130.4 at 10^9, keys excluded |

- **Confirmation:** the 32G stderr log reports 1,536,000,000,096 B internal, 1,024,000,000,064 B leaves and a final table of 549,755,813,888 B. That is 3.11 TB plus 256 GB for the key array.
- **Paper (l. 1801–1819):** (2k+2)w + 9p = 104 B for integers and (3c+2)w + 3 lg w + 12p ≈ 136 B for strings. Both assume load factor 2/3; the code's maximum is 1/2.
- **Unused field:** `key_length` (`zfasttrie.c:72`) is never read or written, so string nodes cost 96 B per key against the paper's 88.
- **Baselines:** red-black 40 B per key (`intprb.h:75-81`). B-tree about 11.4 B per key (inference: 256-byte nodes, 30 keys, cpp-btree's own 75%-fill estimate).

## 3. Hot paths

**Integers** (`zfast_uint64_contains` `:630`, `get_exit_node` `:566`, `fat_binary_search` `:526`, `find` `:215`)
- At most 7 probes (prefix lengths 0, 32, 16/48, …). No allocation; lcp is register arithmetic.
- The slot is a bare pointer, so every occupied slot inspected dereferences a node (`:225`). At load 0.47 that is about 1.4 nodes per hit and 1.3 per miss.
- Total about 7 slot lines, 9 node lines and 1 leaf, roughly 15 cold misses at n ≈ 2^33. A B-tree touches 7–8 nodes.
- Add (`:653`): rehash recomputes every hash by dereferencing every node (`:285`), about one extra random read and write per key amortised.
- Remove (`:754`): backward-shift deletion dereferences each following cluster entry (`:259`).

**Strings** (`zfast_cstr_contains`, `zfasttrie.c:863`)
- Hash state: `preprocess` (`murmur.h:15-46`) allocates ⌊bits/64⌋+1 words holding the running state after each 64-bit word; `hash` (`murmur.h:51-105`) finishes any prefix in O(1).
- That state is allocated and freed on every contains, pred, add and remove (`zfasttrie.c:869/872`, `882/885`, `916/928/990`, `1026/1035/1092`).
- `fat_binary_search` (`:650`) makes at most msb(len)+2 probes, about 11 for 78-byte URLs. Each costs a slot miss; a node is dereferenced only on signature match (`:273`, `:669`).
- Two lcp passes: one through node, reference leaf and key bytes (`:743`, three dependent misses), then `lcp_from` on the child's key (`:761`).
- Total roughly 20 cold misses and four passes over the key.
- Add and remove (`:892`, `:1007`): completion uses `find_exact` (`:641`), which reads node, reference leaf and key bytes per candidate. Rehash and deletion use stored signatures, so no dereferences (`:343`, `:376`).

**Main inefficiencies and remedies**
1. Store extent and handle lengths in the slot (for integers also the extent, or a tag), so the search needs no node dereference. That takes integers from about 15 misses to about 9 and saves about 5 for strings.
2. Validate against the child's key only, as the 2017 code did; this drops the reference-leaf and key-byte misses.
3. Put the hash state on the stack or in caller scratch, and fuse the `strlen`, hash and lcp passes.
4. Start from `z->root` instead of probing the table at f = 0.
5. Prefetch both possible next slots per level, or interleave a batch of queries.
6. Pre-size the table: the drivers pass 0 (`intzfasttrie.c:994`, `zfasttrie.c:1398`), and `create_map` sizes for 2/3 while growth enforces 1/2 (`:250` vs `:368`).
7. Half of the 48-byte internal nodes straddle a cache line; separate jump pointers from search fields.

## 4. Correctness risks

- **Over-read past the query's terminator** (high confidence): `lcp`/`lcp_from` compare whole words below `max_length>>6` (`zfasttrie.c:172-209`). `max_length` is the stored extent, so a shorter query is read up to 7 bytes past its NUL. This is harmless in the packed benchmark buffer except possibly for the last string.
- **Unaligned, type-punned `uint64_t` loads** (`:154`, `:174`, `:195`; high): formally UB, tolerated on x86-64 with `-fno-strict-aliasing`.
- **Left shift of negative values** (high): `-1LL <<` at `:136/141/147`, `intzfasttrie.c:123-134`, `murmur.h:77,156`. GCC defines it.
- **Shift by 64** (medium-high): none reachable that I can trace; all `-1ULL >> len` uses have len ≤ 63 (`intzfasttrie.c:144,188,223,700`), and `:647` short-circuits.
- **Uninitialised read** (high): `parex_is_right_child` (`zfasttrie.c:1044`) is unset when the parex is the root and read at `:1087`. Both branches then touch only the detached node, so it looks benign (medium). The integer version initialises it (`:786`).
- **Unchecked stacks** (medium): `stack[64]` (`:913`, `:1023`) and `stack[8]` (`intzfasttrie.c:673,769`) are safe only through the 2-fat-ancestor invariant (at most 32 and 7 entries). Completion dereferences `find` results without NULL checks (`:642`, `intzfasttrie.c:518`).
- **Length limit** (high): bit lengths are `int32_t` with the top bit as leaf flag (`:32-52`, `:901`); keys of 2^28 − 1 bytes or more wrap silently.
- **NUL reliance:** keys are stored by pointer, `strlen` runs on every call, and the terminator makes the set prefix-free. The driver needs a trailing newline and no empty lines (`:1394` would overwrite the terminator); `read` errors loop forever (`:1355`).
- **`get_random_ints` overflow abort** (`zcommon.c:66-73`): the margin is 1/2048 of the range, so small n can abort; my re-implementation aborts at n = 1000.
- **Endianness:** no dependence found; word loops test equality only and hashing is byte-wise.

## 5. Benchmark methodology

- **Integer phases** (`intzfasttrie.c:1000-1090`): add all n, contains on `x[i]` every `gap`, contains on `x[i]+1`, predecessor of `x[i]+1`, random contains, traversal, delete all n in insertion order.
- **B-tree "predecessor" is a successor query:** `lower_bound(x[i]-1)` (`intbtree.cpp:137-141`).
- **String phases** (`zfasttrie.c:1404-1470`): all keys in file order. Negatives are copies with the middle byte set to 0xFF (`:1394`); predecessor results are unchecked. The B-tree dereferences `lower_bound` even at `end()` (`btree.cpp:183`).
- **Timer** (`zcommon.c:58-62`): user CPU time only, truncated to 1 ms, single run. At 10^6 the query phases last 21–98 ms, so every query cell is a multiple of 10 ns.
- **Comparable:** compiler flags are identical (`comp.sh:3-8`), and integer keys are identical across programs (fixed seed).
- **Not comparable under `Z_MMAP`:** the string red-black pool uses plain `calloc` (`strprb.c:48-51`) and both baselines `malloc` the key buffer. The stored URL runs are standard pages, so they are unaffected.

**Weaknesses a referee would raise**
- The shuffle draws `p` from `[0, n-i)` instead of `[i, n)`. In my re-implementation at n = 10^6, the first and last 1,000 inserts have median rank 0.999n, and position-versus-rank correlation is −0.46 in the first half.
- Positive queries and deletes follow pool allocation order, which helps z-fast and red-black final-node access but not the B-tree (inference).
- Integer negatives are `x+1`, the same path as a positive.
- String positives pass the stored pointer itself, so the final comparison is cache-free for all three.
- `tab:intb` uses standard pages and `tab:intbb` huge pages on different hardware; the paper gives no hardware, compiler or kernel details.
- The command lines for the stored runs are not recorded: the committed scripts use gap 100, but the 8G outputs show gap 10.
- The string B-tree holds pointers and binary-searches with `strcmp`; no trie baseline (ART, HOT, Judy).
- Memory is never measured.

## 6. Paper tables against stored outputs

| Table | Files in `benchmarks/` | Result |
|---|---|---|
| `tab:intm` | `int{zfasttrie,rbtree,btree}-1000000-std.out` | 15 of 15 exact |
| `tab:intb` | `…-8000000000-std.out` | 15 of 15; decimals truncated, not rounded (1733.78 printed as 1733, 1699.89 as 1699, 6529.85 as 6529) |
| `tab:intbb` | `int{zfasttrie,btree}-32000000000-mmap.out` | 10 of 10, truncated (1295.93 as 1295, 1047.52 as 1047); files untracked |
| `tab:urlsm` | `{zfasttrie,rbtree,btree}-test-1000000.urls-std.out` | z-fast column exact; other two swapped |
| `tab:urlsb` | `…-test-1000000000.urls-std.out` | same swap; truncated |

| Cell | Paper "red-black" | Paper "B-tree" | `rbtree-*` file | `btree-*` file |
|---|---|---|---|---|
| urlsm Add | 1288 | 1780 | 1780.00 | 1288.00 |
| urlsm Contains+ | 1532 | 1437 | 1437.00 | 1532.00 |
| urlsm Contains− | 1579 | 1814 | 1814.00 | 1579.00 |
| urlsm Predecessor | 1547 | 1832 | 1832.00 | 1547.00 |
| urlsm Delete | 1255 | 1607 | 1607.00 | 1255.00 |
| urlsb Add | 6936 | 10044 | 10044.98 | 6936.78 |
| urlsb Contains+ | 7261 | 7686 | 7686.18 | 7261.46 |
| urlsb Contains− | 8017 | 8676 | 8676.59 | 8017.62 |
| urlsb Predecessor | 8120 | 8701 | 8701.09 | 8120.88 |
| urlsb Delete | 6798 | 11652 | 11652.18 | 6798.46 |

- **Which side is wrong (inference):** file names come from the binary name, and the `btree` files are faster at 10^9 as expected, so the paper's headers are the likely error.
- **Effect on the URL claims:** against the real B-tree at 10^9, z-fast is 1.56, 2.11, 2.36, 2.14 and 1.21 times faster, so "more than twice" (l. 1934) holds for queries only. At 10^6 it loses Add and Delete.
- **"Two times faster than red-black at 8G" (l. 1922):** the ratios are 1.60, 1.88, 1.98, 2.05, 1.68. With the unused huge-page 8G files they fall to 1.29, 1.30, 1.42, 1.44, 1.23.
- **Unused standard-page runs:** integers at 10^7, 10^8, 10^9, 2·10^9, 16·10^9, and 32·10^9 (no red-black).
- **Unused huge-page runs:** 10^9, 2·10^9, 8·10^9, 16·10^9, and 34·10^9 (no red-black).
- **Unused rows:** "Querying (random)" and "Traversing" everywhere.
- **Only replicate:** root-level `intbtree-32000000000-std.out` differs from the `benchmarks/` copy by 0.9–3.5%.

## 7. Other notes

- **TODO:** the only one is `intzfasttrie.c:571` ("length - 1?"); the string version passes `length` at `zfasttrie.c:734`.
- **Switches:**
  - `Z_MMAP`: mmap plus THP advice, result unchecked.
  - `Z_SIZE`: B-tree arena size.
  - `Z_XXH64`: alternative hash.
  - `SHORT_SIGNATURES`: 2-bit signatures to stress collisions.
  - `NO_DUPS`, `NO_EXACT`: abort if the slow path is taken.
  - `COUNT_SCANS`: probe statistics.
  - `ASSERTS`: full `assert_trie` after every update.
  - `DEBUG`, `DDEBUG`: tracing.
- **`NDEBUG`:** `comp.sh` does not set it; only the `large_*.sh` scripts do.
- **Dead code:** `prb.c` is not built and would not compile (`:51` calls `bigxcalloc` with two arguments). `XXH64()` in `xxh64.h:122-155` is unused.
- **B-tree allocator** (`intbtree.cpp:40-64`): bump pointer with no bounds check; never frees.
- **Paper loose ends:** `\todo{LINK}` at l. 1917 and 1919, a truncated sentence at l. 1934–1935, the URL typo `unini.it` at l. 1932, and a missing Conclusions section (l. 2025).

The shuffle simulation is at `/private/tmp/claude-501/-Users-vigna-pub-zfast/955cf1ac-f832-4902-86ae-acdd82779e8c/scratchpad/shuffle_sim.py` (with `shuffle_sim2.py` beside it).

## Brief given to the reviewer

You are helping with a technical review of C implementations of a data structure. Read-and-report only: do NOT modify, build, or run anything in the directories below, and do not run any git/svn command that changes state (read-only commands such as `git log`, `git show`, `git diff` are fine). Read whole source files.

## Background
The z-fast trie is a dynamic predecessor structure for strings: a compacted binary trie plus a hash map (the "z-map") from "handles" to internal nodes. The handle of a node is the prefix of its extent whose length is the "2-fattest" number (most trailing zeros) in the node's skip interval (name_length-1 .. extent_length]. The exit node of a query x is found by "fat binary search" over prefix lengths, probing the map with hashes of prefixes of x (O(log |x|) probes). The string version stores 63-bit signatures in the map instead of handles; a false positive is detected afterwards and the search is redone in "exact" mode. Internal nodes carry two "jump pointers" used to reach leftmost/rightmost descendant leaves quickly; leaves form a doubly linked list.

## Material
- NEWER version (git, last commits 2021–2022): `/Users/vigna/git/zfast/` — `zfasttrie.c` (string keys), `intzfasttrie.c` (64-bit integer keys), `zcommon.c`, `murmur.h`, baselines `rbtree.c`/`strprb.c`/`prb.c`, `intrbtree.c`/`intprb.c` (libavl red-black trees), `btree.cpp`/`intbtree.cpp` (Google cpp-btree in `cpp-btree/`), build script `comp.sh`, and `benchmarks/` (stdout/stderr of timing runs, plus `large_std.sh`, `large_mmap.sh`). Use `git log --stat` / `git show` there to understand the history. Ignore stray untracked junk (`*.i`, `a.out`, `err*`).
- OLDER snapshot (2017): `/Users/vigna/svn/c/sux/zfasttrie.c` and `/Users/vigna/svn/c/sux/intzfasttrie.c`.
- The paper that reports experiments from this code: `/Users/vigna/pub/zfast/dynzfast.tex`, section "Experiments" (roughly lines 1892–2021), with five timing tables (labels tab:intm, tab:intb, tab:intbb, tab:urlsm, tab:urlsb).

## What to report (with `file:line` references into the NEWER version unless stated)
1. **What changed between the 2017 snapshot and the 2022 version**, in substance (not a line diff): data layout, hashing, how keys are compared, duplicate handling, allocation, benchmark driver. Which commit messages matter.
2. **Data layout and memory**: exact node struct(s) and sizes in bytes for leaves and internal nodes in each of the two programs, hash-table slot layout and load factor, and from that an estimate of bytes per key for the integer and the string version (excluding the keys themselves for strings). Compare with what the paper's "Space usage" section claims if it gives figures.
3. **Hot-path costs**: walk through `zfast_contains` / `zfast_pred` / `zfast_add` / `zfast_remove` and count, for a typical query, hash-table probes, node dereferences (likely cache misses), key comparisons/lcp computations, and any allocation (`malloc`/`calloc`/`free`) on the query path. Note how the per-key hash "state" for O(1) prefix hashing is built and whether it is allocated per query. Identify the main inefficiencies and what could be done about them (e.g. storing extent length in the hash slot to avoid a dereference, prefetching, batching).
4. **Correctness risks**: undefined behaviour (shifts by 64, signed overflow, unaligned or aliasing reads, reading past the end of strings in word-at-a-time loops), fixed-size stacks (`node *stack[64]`), key-length limits (32-bit lengths), reliance on NUL terminators, the no-argument crash in `main`, endianness assumptions. Give the triggering condition and your confidence.
5. **Benchmark methodology**: what exactly each driver times (which operations, in which order, on which keys, wall vs user time, timer resolution), how negative queries are generated, whether the three structures (z-fast trie, red-black tree, B-tree) are measured comparably (same allocator policy? same key order? same compiler flags from `comp.sh`?), and weaknesses a referee would point out.
6. **Do the paper's tables match the stored outputs?** For each of the five tables, find the corresponding files in `/Users/vigna/git/zfast/benchmarks/` and report whether the numbers agree, listing any discrepancies (table cell, paper value, file value, file name). Also list which stored runs (sizes, std vs mmap) are NOT used in the paper.
7. **Anything else notable**: TODO/FIXME comments, dead code, compile-time switches and what they do.

## Output format
A structured report, at most ~1300 words plus small tables for (2) and (6). Verified facts first; label inferences as such. If a file or number is missing, say so explicitly instead of guessing.
