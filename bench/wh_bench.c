/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

/*
 * Driver for Wormhole in the comparative benchmarks: it reads the workload
 * files written by `zfast-bench gen` (sequences of records formed by a 32-bit
 * little-endian length followed by that many bytes), runs the same phases as
 * `zfast-bench run`, and prints the same lines.
 *
 *   wh_bench PREFIX REPS
 *
 * We use the single-threaded ("unsafe") interface of Wormhole, as all
 * structures are benchmarked on a single thread.
 */

#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "lib.h"
#include "kv.h"
#include "wh.h"

/* A workload: the bytes of a file, and the number of records. */
struct workload {
	u8 *bytes;
	size_t size;
	size_t n;
};

static struct workload load(const char *prefix, const char *ext) {
	char name[4096];
	snprintf(name, sizeof name, "%s.%s", prefix, ext);
	FILE *f = fopen(name, "rb");
	if (!f) {
		perror(name);
		exit(1);
	}
	fseek(f, 0, SEEK_END);
	struct workload w = { .size = (size_t)ftell(f) };
	fseek(f, 0, SEEK_SET);
	w.bytes = malloc(w.size + 1);
	if (fread(w.bytes, 1, w.size, f) != w.size) {
		perror(name);
		exit(1);
	}
	fclose(f);
	for (size_t pos = 0; pos < w.size; w.n++) {
		u32 len;
		memcpy(&len, w.bytes + pos, 4);
		pos += 4 + len;
	}
	return w;
}

static double now(void) {
	struct timespec ts;
	clock_gettime(CLOCK_MONOTONIC, &ts);
	return ts.tv_sec * 1E9 + ts.tv_nsec;
}

/* Returns the resident set size of the process in bytes. */
static size_t rss(void) {
	size_t size = 0, resident = 0;
	FILE *f = fopen("/proc/self/statm", "r");
	if (f) {
		if (fscanf(f, "%zu %zu", &size, &resident) != 2) resident = 0;
		fclose(f);
	}
	return resident * 4096;
}

static void result(const char *op, size_t n, double ns, size_t checksum) {
	printf("RESULT method=wormhole op=%s n=%zu ns=%.1f checksum=%zu\n", op, n, ns, checksum);
	fflush(stdout);
}

/* The operations: each one returns a value that is added to the checksum. */

static struct wormhole *map;
static struct wormhole_iter *iter;
static struct kv *kvbuf;

static size_t op_insert(const u8 *key, u32 len) {
	kv_refill(kvbuf, key, len, NULL, 0);
	return whunsafe_put(map, kvbuf);
}

static size_t op_contains(const u8 *key, u32 len) {
	struct kref kref;
	kref_ref_hash32(&kref, key, len);
	return whunsafe_probe(map, &kref);
}

static size_t op_succ(const u8 *key, u32 len) {
	struct kref kref;
	kref_ref_hash32(&kref, key, len);
	whunsafe_iter_seek(iter, &kref);
	struct kref out;
	return wormhole_iter_kref(iter, &out) ? out.len : 0;
}

static size_t op_remove(const u8 *key, u32 len) {
	struct kref kref;
	kref_ref_hash32(&kref, key, len);
	return whunsafe_del(map, &kref);
}

/* Runs an operation on all records, printing the best time per operation
 * over the given number of repetitions. */
static void run(const char *op_name, int reps, const struct workload *w, size_t (*op)(const u8 *, u32)) {
	double best = 1E300;
	size_t sum = 0;
	for (int r = 0; r < reps; r++) {
		sum = 0;
		const double start = now();
		for (size_t pos = 0; pos < w->size;) {
			u32 len;
			memcpy(&len, w->bytes + pos, 4);
			sum += op(w->bytes + pos + 4, len);
			pos += 4 + len;
		}
		const double t = (now() - start) / w->n;
		if (t < best) best = t;
	}
	result(op_name, w->n, best, sum);
}

int main(int argc, char **argv) {
	if (argc != 3) {
		fprintf(stderr, "Usage: %s PREFIX REPS\n", argv[0]);
		return 1;
	}
	const int reps = atoi(argv[2]);
	const struct workload keys = load(argv[1], "keys"), hits = load(argv[1], "hits"), misses = load(argv[1], "misses");

	/* Room for the longest key */
	kvbuf = malloc(sizeof(struct kv) + (1 << 20));

	const size_t before = rss();
	map = whunsafe_create(NULL);
	run("insert", 1, &keys, op_insert);
	printf("SPACE method=wormhole n=%zu bytes_per_key=0.0 rss_per_key=%.1f\n", keys.n, (double)(rss() - before) / keys.n);

	iter = whunsafe_iter_create(map);
	run("contains-hit", reps, &hits, op_contains);
	run("contains-miss", reps, &misses, op_contains);
	run("succ-hit", reps, &hits, op_succ);
	run("succ-miss", reps, &misses, op_succ);

	/* Iteration over all keys */
	double start = now();
	size_t sum = 0;
	struct kref null = { 0 }, out;
	null.ptr = (const u8 *)"";
	whunsafe_iter_seek(iter, &null);
	while (wormhole_iter_kref(iter, &out)) {
		sum += out.len;
		whunsafe_iter_skip1(iter);
	}
	result("iterate", keys.n, (now() - start) / keys.n, sum);
	whunsafe_iter_destroy(iter);

	run("remove", 1, &hits, op_remove);
	return 0;
}
