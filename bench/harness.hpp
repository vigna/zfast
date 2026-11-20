/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

/*
 * Code shared by the C++ drivers of the comparative benchmarks: loading the
 * workload files written by `zfast-bench gen` (sequences of records formed by
 * a 32-bit little-endian length followed by that many bytes), timing, and
 * printing results in the format of `zfast-bench run`.
 *
 * A driver defines METHOD (the name printed in the results) before including
 * this file.
 */

#pragma once

#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <vector>

/* A workload: the records, their overall size, their number and the length
 * of the longest one. */
struct Workload {
	std::vector<uint8_t> bytes;
	size_t size = 0;
	size_t n = 0;
	size_t max_len = 0;
};

/* Loads a workload. If terminate is true, a zero is appended to each key
 * (for structures that require a prefix-free set of keys). The vector is
 * followed by the given number of zeros, for structures that can read a few
 * bytes after the end of a key. */
static Workload load(const char *prefix, const char *ext, bool terminate, size_t padding) {
	char name[4096];
	snprintf(name, sizeof name, "%s.%s", prefix, ext);
	FILE *f = fopen(name, "rb");
	if (!f) {
		perror(name);
		exit(1);
	}
	fseek(f, 0, SEEK_END);
	const size_t size = (size_t)ftell(f);
	fseek(f, 0, SEEK_SET);
	std::vector<uint8_t> raw(size);
	if (fread(raw.data(), 1, size, f) != size) {
		perror(name);
		exit(1);
	}
	fclose(f);
	Workload w;
	w.bytes.reserve(size + (terminate ? size / 4 : 0) + padding);
	for (size_t pos = 0; pos < size;) {
		uint32_t len;
		memcpy(&len, raw.data() + pos, 4);
		const uint32_t new_len = len + terminate;
		const uint8_t *p = (const uint8_t *)&new_len;
		w.bytes.insert(w.bytes.end(), p, p + 4);
		w.bytes.insert(w.bytes.end(), raw.data() + pos + 4, raw.data() + pos + 4 + len);
		if (terminate) w.bytes.push_back(0);
		if (new_len > w.max_len) w.max_len = new_len;
		pos += 4 + len;
		w.n++;
	}
	w.size = w.bytes.size();
	w.bytes.resize(w.size + padding);
	return w;
}

static double now() {
	struct timespec ts;
	clock_gettime(CLOCK_MONOTONIC, &ts);
	return ts.tv_sec * 1E9 + ts.tv_nsec;
}

/* Returns the resident set size of the process in bytes. */
static size_t rss() {
	size_t size = 0, resident = 0;
	FILE *f = fopen("/proc/self/statm", "r");
	if (f) {
		if (fscanf(f, "%zu %zu", &size, &resident) != 2) resident = 0;
		fclose(f);
	}
	return resident * 4096;
}

static void result(const char *op, size_t n, double ns, size_t checksum) {
	printf("RESULT method=%s op=%s n=%zu ns=%.1f checksum=%zu\n", METHOD, op, n, ns, checksum);
	fflush(stdout);
}

static void space(size_t n, size_t bytes) {
	printf("SPACE method=%s n=%zu bytes_per_key=0.0 rss_per_key=%.1f\n", METHOD, n, (double)bytes / n);
	fflush(stdout);
}

/* Runs an operation on all records, printing the best time per operation
 * over the given number of repetitions. Each call returns a value that is
 * added to the checksum. */
static void run(const char *op_name, int reps, Workload &w, size_t (*op)(uint8_t *, uint32_t)) {
	double best = 1E300;
	size_t sum = 0;
	const size_t size = w.size;
	uint8_t *bytes = w.bytes.data();
	for (int r = 0; r < reps; r++) {
		sum = 0;
		const double start = now();
		for (size_t pos = 0; pos < size;) {
			uint32_t len;
			memcpy(&len, bytes + pos, 4);
			sum += op(bytes + pos + 4, len);
			pos += 4 + len;
		}
		const double t = (now() - start) / w.n;
		if (t < best) best = t;
	}
	result(op_name, w.n, best, sum);
}
