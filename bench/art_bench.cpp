/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

/*
 * Driver for the adaptive radix tree (ART) in the comparative benchmarks. We
 * use the implementation with optimistic lock coupling of
 * <https://github.com/flode/ARTSynchronized> (Leis, Scheibner, Kemper,
 * Neumann, "The ART of practical synchronization", DaMoN 2016), which is the
 * one used by most recent comparisons, on a single thread: the
 * unsynchronized version in the same repository has no range scan.
 *
 * It reads the workload files written by `zfast-bench gen`, runs the same
 * phases as `zfast-bench run`, and prints the same lines.
 *
 *   art_bench PREFIX REPS
 *
 * ART is a secondary index: it maps keys to tuple identifiers, and retrieves
 * the key of a tuple through a callback. Our tuples are records containing
 * just the key, so that the space we measure includes the keys, as it
 * happens for the other structures. Moreover:
 *
 * - no key can be a prefix of another key, so we append a zero to all keys
 *   (the keys must not contain zeros);
 * - the range scan does not compare the first leaf it reaches with the start
 *   key, so for successor queries we ask for two results and skip the first
 *   one if it is smaller than the query;
 * - a deletion requires the tuple identifier, so it is preceded by a lookup.
 */

/*
 * If ART_UNSYNC is defined, the driver uses instead the unsynchronized
 * version of the same repository, which has no range scan: the phases based
 * on scans (successor queries and the iteration) are then skipped. We use it
 * to check how much the synchronization costs on a single thread.
 */

#ifdef ART_UNSYNC
#define METHOD "art-unsync"
#else
#define METHOD "art"
#endif
#include "harness.hpp"

#ifdef ART_UNSYNC
#include "ART/Tree.h"
/* The unsynchronized tree has the same methods, without the last argument */
struct Unsync {
	ART_unsynchronized::Tree tree;
	Unsync(ART_unsynchronized::Tree::LoadKeyFunction f) : tree(f) {}
	void insert(const Key &k, TID tid, int) { tree.insert(k, tid); }
	TID lookup(const Key &k, int) { return tree.lookup(k); }
	void remove(const Key &k, TID tid, int) { tree.remove(k, tid); }
};
typedef Unsync Tree;
typedef int Info;
#else
#include "OptimisticLockCoupling/Tree.h"
typedef ART_OLC::Tree Tree;
typedef ART::ThreadInfo Info;
#endif

/* A tuple: a key preceded by its length. */
struct Tuple {
	uint32_t len;
	uint8_t key[];
};

static void load_key(TID tid, Key &key) {
	const Tuple *t = reinterpret_cast<const Tuple *>(tid);
	key.set(reinterpret_cast<const char *>(t->key), t->len);
}

static Tree *tree;
static Info *info;

/* The operations: each one returns a value that is added to the checksum. */

static size_t op_insert(uint8_t *key, uint32_t len) {
	Tuple *t = static_cast<Tuple *>(malloc(sizeof(Tuple) + len));
	t->len = len;
	memcpy(t->key, key, len);
	Key k;
	k.set(reinterpret_cast<const char *>(key), len);
	tree->insert(k, reinterpret_cast<TID>(t), *info);
	return 1;
}

static size_t op_contains(uint8_t *key, uint32_t len) {
	Key k;
	k.set(reinterpret_cast<const char *>(key), len);
	return tree->lookup(k, *info) != 0;
}

/* Returns the length of the smallest key greater than or equal to the given
 * one (without the final zero), or zero if there is no such key. */
#ifndef ART_UNSYNC
static size_t op_succ(uint8_t *key, uint32_t len) {
	Key k;
	k.set(reinterpret_cast<const char *>(key), len);
	TID found[2];
	size_t count = 0;
	tree->lookupRange(k, found, 2, count, *info);
	for (size_t i = 0; i < count; i++) {
		const Tuple *t = reinterpret_cast<const Tuple *>(found[i]);
		const uint32_t min = t->len < len ? t->len : len;
		const int c = memcmp(t->key, key, min);
		if (c > 0 || (c == 0 && t->len >= len)) return t->len - 1;
	}
	return 0;
}

#endif

static size_t op_remove(uint8_t *key, uint32_t len) {
	Key k;
	k.set(reinterpret_cast<const char *>(key), len);
	const TID tid = tree->lookup(k, *info);
	if (tid == 0) return 0;
	tree->remove(k, tid, *info);
	free(reinterpret_cast<void *>(tid));
	return 1;
}

int main(int argc, char **argv) {
	if (argc != 3) {
		fprintf(stderr, "Usage: %s PREFIX REPS\n", argv[0]);
		return 1;
	}
	const int reps = atoi(argv[2]);
	Workload keys = load(argv[1], "keys", true, 0), hits = load(argv[1], "hits", true, 0), misses = load(argv[1], "misses", true, 0);
	/* The identifiers returned by the scan of all keys */
	std::vector<TID> all(keys.n + 1);

	const size_t before = rss();
	tree = new Tree(load_key);
#ifdef ART_UNSYNC
	Info thread_info = 0;
#else
	Info thread_info = tree->getThreadInfo();
#endif
	info = &thread_info;
	run("insert", 1, keys, op_insert);
	space(keys.n, rss() - before);

	run("contains-hit", reps, hits, op_contains);
	run("contains-miss", reps, misses, op_contains);
#ifndef ART_UNSYNC
	run("succ-hit", reps, hits, op_succ);
	run("succ-miss", reps, misses, op_succ);

	/* Iteration over all keys */
	const double start = now();
	size_t sum = 0, count = 0;
	Key first;
	first.set("", 0);
	tree->lookupRange(first, all.data(), all.size(), count, *info);
	for (size_t i = 0; i < count; i++) sum += reinterpret_cast<const Tuple *>(all[i])->len - 1;
	result("iterate", keys.n, (now() - start) / keys.n, sum);
#endif

	run("remove", 1, hits, op_remove);
	return 0;
}
