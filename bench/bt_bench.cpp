/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

/*
 * Driver for the structures of the repository accompanying "B-Trees Are
 * Back: Engineering Fast and Pageable Node Layouts" (Mueller, Benson, Leis,
 * SIGMOD 2025), <https://github.com/m-mueller678/btree-cpp>: their B-tree in
 * one of its configurations, and the version of HOT they compare with, behind
 * the common interface DataStructureWrapper. The structure is chosen at
 * compile time by including one of the files in named-configs (see the
 * Makefile), and METHOD is the name printed in the results.
 *
 * It reads the workload files written by `zfast-bench gen`, runs the same
 * phases as `zfast-bench run`, and prints the same lines.
 *
 *   bt_bench PREFIX REPS
 *
 * Limitations of the structures, not of the driver: keys must be shorter
 * than BTreeNode::maxKVSize (about a third of a page) for the B-tree and
 * than 256 bytes for HOT, and they must not end with a zero for HOT; the
 * adapter of HOT does not implement deletions, so the remove phase is skipped
 * for it.
 *
 * The repository contains also an adapter for ART, which we do not use: on
 * our workloads it returns a few wrong answers for keys not in the set (its
 * lookup function does not check the bytes of a compressed path when the
 * search ends exactly at the end of the key, and the range scan can start
 * from the wrong key). See art_bench.cpp for ART.
 */

#include "btree/btree2020.hpp"

#ifndef METHOD
#error "METHOD must be defined"
#endif

#include "harness.hpp"

#ifdef USE_STRUCTURE_HOT
static const bool HAS_REMOVE = false;
static const size_t MAX_LEN = 255;
#else
static const bool HAS_REMOVE = true;
static const size_t MAX_LEN = BTreeNode::maxKVSize;
#endif

/* The operations: each one returns a value that is added to the checksum. */

static DataStructureWrapper *map;
static uint8_t key_out[1 << 16];
static uint8_t no_payload[1];

static size_t op_insert(uint8_t *key, uint32_t len) {
	map->insert(key, len, no_payload, 0);
	return 1;
}

static size_t op_contains(uint8_t *key, uint32_t len) {
	return map->lookup(key, len);
}

/* Returns the length of the smallest key greater than or equal to the
 * given one, or zero if there is no such key. */
static size_t op_succ(uint8_t *key, uint32_t len) {
	size_t found = 0;
	map->range_lookup(key, len, key_out, [&found](unsigned key_len, uint8_t *, unsigned) {
		found = key_len;
		return false;
	});
	return found;
}

static size_t op_remove(uint8_t *key, uint32_t len) {
	return map->remove(key, len);
}

int main(int argc, char **argv) {
	if (argc != 3) {
		fprintf(stderr, "Usage: %s PREFIX REPS\n", argv[0]);
		return 1;
	}
	const int reps = atoi(argv[2]);
	Workload keys = load(argv[1], "keys", false, 0), hits = load(argv[1], "hits", false, 0), misses = load(argv[1], "misses", false, 0);

	const size_t max_len = keys.max_len > misses.max_len ? keys.max_len : misses.max_len;
	if (max_len > MAX_LEN) {
		fprintf(stderr, "%s: the longest key has %zu bytes, but the limit is %zu\n", METHOD, max_len, MAX_LEN);
		return 1;
	}

	const size_t before = rss();
	map = new DataStructureWrapper(false);
	run("insert", 1, keys, op_insert);
	space(keys.n, rss() - before);

	run("contains-hit", reps, hits, op_contains);
	run("contains-miss", reps, misses, op_contains);
	run("succ-hit", reps, hits, op_succ);
	run("succ-miss", reps, misses, op_succ);

	/* Iteration over all keys */
	const double start = now();
	size_t sum = 0;
	uint8_t first[1] = { 0 };
	map->range_lookup(first, 0, key_out, [&sum](unsigned key_len, uint8_t *, unsigned) {
		sum += key_len;
		return true;
	});
	result("iterate", keys.n, (now() - start) / keys.n, sum);

	if (HAS_REMOVE) run("remove", 1, hits, op_remove);
	return 0;
}
