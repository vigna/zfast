#!/usr/bin/env python3
# Independent re-implementation (NOT the repository's code) of the zuffix
# fat-binary-search control flow with exact (collision-free) keys, to measure
# the effect of computing hlen as twoFattestLR(nlen, e + 1) at query time
# (as MemcmpZuffixArray.hpp:207-209 does) versus twoFattestLR(nlen, e)
# (as the construction, ZFillByDFS, does).
import random
import sys

MASK = (1 << 64) - 1


def lam(x):
    return x.bit_length() - 1


def two_fattest_lr(a, b):
    # 2-fattest number in [a, b]; mirrors zuffix/util/common.hpp:47
    if a == 0:
        return 0
    x = (a - 1) ^ b
    return b & ~((1 << lam(x)) - 1)


def two_fattest_r(a, b):
    # 2-fattest number in (a, b]
    return b & ~((1 << lam(a ^ b)) - 1)


def build(text):
    n = len(text)
    K = 512
    sa = sorted(range(n), key=lambda i: text[i:i + K])
    for k in range(1, n):
        assert text[sa[k - 1]:sa[k - 1] + K] != text[sa[k]:sa[k] + K]
    lcp = [-1] * (n + 1)
    for k in range(1, n):
        a, b = sa[k - 1], sa[k]
        l = 0
        while text[a + l] == text[b + l]:
            l += 1
        lcp[k] = l
    # enumerate internal lcp-intervals [i, j) bottom-up
    nodes = []
    stack = [(0, 0)]  # (lcp value, left boundary)
    for k in range(1, n + 1):
        lb = k - 1
        while stack and lcp[k] < stack[-1][0]:
            e, i = stack.pop()
            nodes.append((i, k, e))
            lb = i
        if stack and lcp[k] > stack[-1][0]:
            stack.append((lcp[k], lb))
    assert not stack and nodes[-1] == (0, n, 0)  # the root is popped last
    zmap = {}
    extents = set()
    maxnlen = 0
    for (i, j, e) in nodes:
        p = max(lcp[i], lcp[j])
        nlen = p + 1
        h = two_fattest_lr(nlen, e)
        maxnlen = max(maxnlen, nlen)
        key = text[sa[i]:sa[i] + h]
        assert key not in zmap
        zmap[key] = (i, j, e, nlen)
        extents.add(text[sa[i]:sa[i] + e])
    return sa, lcp, zmap, extents, maxnlen, len(nodes)


def fbs(P, zmap, n, maxnlen, plus_one):
    alpha = (0, n)
    alpha_e = 0
    l, r = 0, min(len(P), maxnlen)
    probes = hits = rejected = 0
    while l < r:
        f = two_fattest_r(l, r)
        probes += 1
        beta = zmap.get(P[:f])
        if beta is None:
            r = f - 1
            continue
        i, j, e, nlen = beta
        hits += 1
        hlen = two_fattest_lr(nlen, e + 1 if plus_one else e)
        if hlen != f:
            rejected += 1
            r = f - 1
        elif not (alpha[0] <= i and j <= alpha[1] and alpha != (i, j)):
            l = e + 1
        else:
            l = e
            alpha = (i, j)
            alpha_e = e
    return alpha_e, probes, hits, rejected


def run(name, text, lengths, trials=20000):
    n = len(text)
    sa, lcp, zmap, extents, maxnlen, nnodes = build(text)
    print(f"== {name}: n={n} internal nodes={nnodes} maxnlen={maxnlen}")
    print(f"{'m':>6} {'probes(+1)':>10} {'probes(fix)':>11} {'rej/query':>10} {'q.w/rej%':>9} {'extra child steps':>18}")
    rng = random.Random(1)
    for m in lengths:
        tp = tf = trej = qrej = textra = 0
        for _ in range(trials):
            s = rng.randrange(0, n - m - 1)
            P = text[s:s + m]
            e1, p1, h1, r1 = fbs(P, zmap, n, maxnlen, True)
            e2, p2, h2, r2 = fbs(P, zmap, n, maxnlen, False)
            assert r2 == 0
            assert e1 <= e2
            tp += p1
            tf += p2
            trej += r1
            qrej += r1 > 0
            # internal nodes on the path strictly below alpha(+1) down to alpha(fix)
            textra += sum(1 for k in range(e1 + 1, e2 + 1) if P[:k] in extents)
        print(f"{m:>6} {tp / trials:>10.3f} {tf / trials:>11.3f} {trej / trials:>10.3f} {100 * qrej / trials:>9.1f} {textra / trials:>18.3f}")


def main():
    rng = random.Random(2023)
    n = 200000
    dna = bytes(rng.choice(b"ACGT") for _ in range(n - 1)) + b"\xff"
    run("random ACGT", dna, [1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 16, 20, 50, 100, 1000])
    if len(sys.argv) > 1:
        with open(sys.argv[1], "rb") as fh:
            w = fh.read(n - 1).replace(b"\xff", b"?") + b"\xff"
        run("natural-language word list prefix", w, [1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 16, 20, 50, 100, 1000])


main()
