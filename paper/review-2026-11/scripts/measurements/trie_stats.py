#!/usr/bin/env python3
"""Throwaway measurement: byte-level vs bit-level compacted tries on a key set.

For a sorted set of byte strings (made prefix-free with a 0x00 terminator) it
reports, for the 256-ary (byte) trie and for the binary (bit) trie:
  - number of internal nodes (= z-map entries),
  - how many bit-level handles are byte-aligned,
  - fan-out distribution of byte-level internal nodes,
  - number of z-map probes of a fat binary search for member keys,
and the same when only every k-th key is indexed (bucketing).
"""
import sys
from collections import Counter


def lcp_bytes(a, b):
    lo, hi = 0, min(len(a), len(b))
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if a[:mid] == b[:mid]:
            lo = mid
        else:
            hi = mid - 1
    return lo


def two_fattest(a, b):
    """2-fattest number in (a..b], for -1 <= a < b."""
    if a < 0:
        return 0
    return (-1 << ((a ^ b).bit_length() - 1)) & b


def probes(path, length):
    """Number of z-map probes of a fat binary search over (ell_root..length].

    path is the list of (name_len, extent_len) of the internal ancestors of the
    key, root first. The root is special-cased (no probe), as in the C code.
    """
    handles = {}
    for name, ext in path:
        handles[two_fattest(name - 1, ext)] = ext
    a, b, count = path[0][1], length, 0
    while b - a > 0:
        f = two_fattest(a, b)
        count += 1
        if f in handles and handles[f] > a:
            a = handles[f]
        else:
            b = f - 1
    return count


def analyse(keys, lcps, lengths, unit, sample):
    """keys sorted; lcps[i] = lcp(keys[i-1], keys[i]) in `unit`s; lengths in `unit`s."""
    n = len(keys)
    root_ell = min(lcps[1:]) if n > 1 else lengths[0]
    stack = [[root_ell, 0, 0, 0]]  # extent len, left boundary, leaves covered by child intervals, child intervals
    internal = 0
    degree = Counter()
    aligned = 0
    handle_sum = 0
    skip_sum = 0
    probe_sum = probe_max = sampled = 0
    depth_sum = 0

    def close(ell, lb, cov, nci, rb, parent_ell):
        nonlocal internal, aligned, handle_sum, skip_sum
        children = nci + (rb - lb + 1 - cov)
        internal += 1
        degree[children] += 1
        name = 0 if parent_ell is None else parent_ell + 1
        h = two_fattest(name - 1, ell)
        handle_sum += h
        skip_sum += ell - name + 1
        if unit == 1 and h % 8 == 0:
            aligned += 1

    for i in range(1, n + 1):
        cur = lcps[i] if i < n else -1
        # Ancestors of leaf i-1: the stack, plus a new node opened by lcps[i].
        if sample and (i - 1) % sample == 0:
            ells = [e[0] for e in stack]
            if cur > ells[-1]:
                ells.append(cur)
            path = [(0, ells[0])] + [(ells[k - 1] + 1, ells[k]) for k in range(1, len(ells))]
            p = probes(path, lengths[i - 1])
            probe_sum += p
            probe_max = max(probe_max, p)
            depth_sum += len(path)
            sampled += 1
        lb = i - 1
        pending = None
        while stack and cur < stack[-1][0]:
            ell, l, cov, nci = stack.pop()
            size = i - l
            lb = l
            if stack and cur <= stack[-1][0]:
                stack[-1][2] += size
                stack[-1][3] += 1
                close(ell, l, cov, nci, i - 1, stack[-1][0])
            elif stack:
                pending = (ell, l, cov, nci, size)
            else:
                close(ell, l, cov, nci, i - 1, None)  # the root
        if stack and cur > stack[-1][0]:
            stack.append([cur, lb, 0, 0])
            if pending:
                ell, l, cov, nci, size = pending
                stack[-1][2] += size
                stack[-1][3] += 1
                close(ell, l, cov, nci, i - 1, cur)
    return dict(internal=internal, degree=degree, aligned=aligned, handle_avg=handle_sum / max(internal, 1),
                skip_avg=skip_sum / max(internal, 1), probe_avg=probe_sum / max(sampled, 1), probe_max=probe_max,
                depth_avg=depth_sum / max(sampled, 1))


def run(name, raw):
    keys = sorted(set(raw))
    keys = [k + b"\x00" for k in keys if b"\x00" not in k]
    n = len(keys)
    tot = sum(len(k) for k in keys)
    print(f"\n## {name}: {n} keys, avg length {tot / n:.1f} bytes (terminator included)")
    for step in (1, 16, 64):
        ks = keys[::step]
        m = len(ks)
        lb = [0] * m
        lbit = [0] * m
        for i in range(1, m):
            a, b = ks[i - 1], ks[i]
            l = lcp_bytes(a, b)
            lb[i] = l
            lbit[i] = 8 * l + (8 - (a[l] ^ b[l]).bit_length())
        sample = max(1, m // 20000)
        by = analyse(ks, lb, [len(k) for k in ks], 8, sample)
        bi = analyse(ks, lbit, [8 * len(k) for k in ks], 1, sample)
        d = by["degree"]
        tot_int = by["internal"]
        # Sanity checks: leaves = 1 + sum(fan-out - 1); a binary trie has m - 1 binary internal nodes.
        assert sum((k - 1) * c for k, c in d.items()) == m - 1, "byte trie inconsistent"
        assert bi["internal"] == m - 1 and set(bi["degree"]) == {2}, "bit trie inconsistent"
        small = sum(c for k, c in d.items() if k <= 4) / tot_int
        mid = sum(c for k, c in d.items() if 4 < k <= 16) / tot_int
        big = sum(c for k, c in d.items() if k > 16) / tot_int
        print(f"- indexing every {step}-th key ({m} keys):")
        print(f"    byte trie: {tot_int} internal nodes ({tot_int / m:.3f} per key); "
              f"fan-out 2: {d[2] / tot_int:.1%}, <=4: {small:.1%}, 5-16: {mid:.1%}, >16: {big:.1%}, max {max(d)}; "
              f"avg fan-out {sum(k * c for k, c in d.items()) / tot_int:.2f}")
        print(f"               avg internal ancestors {by['depth_avg']:.1f}, avg handle length {by['handle_avg']:.1f} bytes, "
              f"probes/search avg {by['probe_avg']:.2f} max {by['probe_max']}  (+1 child lookup)")
        print(f"    bit trie:  {bi['internal']} internal nodes ({bi['internal'] / m:.3f} per key); "
              f"byte-aligned handles {bi['aligned'] / bi['internal']:.1%}; "
              f"avg internal ancestors {bi['depth_avg']:.1f}, probes/search avg {bi['probe_avg']:.2f} max {bi['probe_max']}")


if __name__ == "__main__":
    for path in sys.argv[1:]:
        with open(path, "rb") as f:
            run(path.rsplit("/", 1)[-1], f.read().split(b"\n")[:-1])
