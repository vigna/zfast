#!/usr/bin/env python3
"""Throwaway measurement: how many hash entries would a pruned z-map need?

For a text, builds the suffix array and LCP array, enumerates the internal
nodes of the suffix tree (LCP intervals) and, for several thresholds t, counts
  heavy(t): internal nodes whose interval contains at least t suffixes,
  kids(t):  children (of any size, leaves included) of heavy nodes,
the two kinds of entries a z-map over a PLAIN suffix array would store
(handles of heavy nodes; names of their children, to step down without a
child table). Reported per text symbol.
"""
import sys
import numpy as np

THRESHOLDS = (2, 8, 16, 32, 64, 128, 256, 1024)


def suffix_array(b):
    n = len(b)
    rank = np.frombuffer(b, dtype=np.uint8).astype(np.int64)
    k = 1
    while True:
        r2 = np.zeros(n, dtype=np.int64)
        r2[: n - k] = rank[k:] + 1
        key = rank * (n + 2) + r2
        sa = np.argsort(key, kind="stable")
        sk = key[sa]
        new = np.empty(n, dtype=np.int64)
        new[sa] = np.concatenate(([0], np.cumsum(sk[1:] != sk[:-1])))
        rank = new
        if int(rank.max()) == n - 1:
            return sa, rank
        k *= 2


def kasai(b, sa, rank):
    n = len(b)
    sa = sa.tolist()
    rank = rank.tolist()
    lcp = [0] * n
    h = 0
    for i in range(n):
        r = rank[i]
        if r > 0:
            j = sa[r - 1]
            while i + h < n and j + h < n and b[i + h] == b[j + h]:
                h += 1
            lcp[r] = h
            if h > 0:
                h -= 1
        else:
            h = 0
    return lcp


def stats(lcp, n):
    heavy = dict.fromkeys(THRESHOLDS, 0)
    kids = dict.fromkeys(THRESHOLDS, 0)
    maxext = dict.fromkeys(THRESHOLDS, 0)
    internal = 0

    def close(ell, size, children):
        nonlocal internal
        internal += 1
        for t in THRESHOLDS:
            if size >= t:
                heavy[t] += 1
                kids[t] += children
                if ell > maxext[t]:
                    maxext[t] = ell

    stack = [[min(lcp[1:]), 0, 0, 0]]  # extent, left boundary, leaves under child intervals, child intervals
    for i in range(1, n + 1):
        cur = lcp[i] if i < n else -1
        lb = i - 1
        pending = None
        while stack and cur < stack[-1][0]:
            ell, l, cov, nci = stack.pop()
            size = i - l
            lb = l
            close(ell, size, nci + size - cov)
            if stack and cur <= stack[-1][0]:
                stack[-1][2] += size
                stack[-1][3] += 1
            elif stack:
                pending = size
        if stack and cur > stack[-1][0]:
            stack.append([cur, lb, 0, 0])
            if pending:
                stack[-1][2] += pending
                stack[-1][3] += 1
    return internal, heavy, kids, maxext


def run(name, data):
    data = data.replace(b"\x00", b" ") + b"\x00"
    n = len(data)
    sa, rank = suffix_array(data)
    lcp = kasai(data, sa, rank)
    internal, heavy, kids, maxext = stats(lcp, n)
    print(f"\n## {name}: n = {n}, alphabet {len(set(data))}, internal nodes {internal} ({internal / n:.3f} n), "
          f"mean LCP {sum(lcp) / n:.1f}, max LCP {max(lcp)}")
    print("   t | heavy/n | kids/n | entries/n (heavy+kids) | max heavy extent")
    for t in THRESHOLDS:
        print(f"{t:5d} | {heavy[t] / n:7.4f} | {kids[t] / n:6.4f} | {(heavy[t] + kids[t]) / n:8.4f}               | {maxext[t]}")


if __name__ == "__main__":
    limit = int(sys.argv[1])
    for path in sys.argv[2:]:
        with open(path, "rb") as f:
            run(path.rsplit("/", 1)[-1], f.read(limit))
