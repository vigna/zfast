# Independent re-implementation (for analysis only) of get_random_ints() from zcommon.c
# to characterise the permutation produced by its shuffle loop.
import sys
M = (1 << 64) - 1
s = [0xdf900294d8f554a5, 0x170865df4b3201fc]
def rotl(x, k): return ((x << k) | (x >> (64 - k))) & M
def nxt():
    s0, s1 = s
    r = (s0 + s1) & M
    s1 ^= s0
    s[0] = rotl(s0, 24) ^ s1 ^ ((s1 << 16) & M)
    s[1] = rotl(s1, 37)
    return r

def gen(n):
    gap = ((M // (n + 1)) // 1024) * 2047 - 2
    x = [0] * n
    x[0] = nxt() % gap
    overflow = False
    for i in range(1, n):
        v = x[i - 1] + nxt() % gap + 2
        if v > M: overflow = True
        x[i] = v & M
    # ranks: x is sorted here, so rank == index
    r = list(range(n))
    for i in range(n):
        p = nxt() % (n - i)
        r[i], r[p] = r[p], r[i]
    return r, overflow

for n in (1000, 10**6):
    s[0], s[1] = 0xdf900294d8f554a5, 0x170865df4b3201fc
    r, ov = gen(n)
    print("n =", n, "overflow(abort) =", ov)
    print("  first 12 ranks/n:", [round(v / n, 4) for v in r[:12]])
    h = n // 2
    import statistics
    fh, sh = r[:h], r[h:]
    print("  mean rank/n first half: %.3f  second half: %.3f" % (statistics.mean(fh) / n, statistics.mean(sh) / n))
    print("  frac of first-half positions holding a rank >= n/2: %.3f" % (sum(1 for v in fh if v >= h) / h))
    # Spearman-like: correlation between position and rank in the first half
    def corr(a):
        m = len(a); mx = (m - 1) / 2; my = statistics.mean(a)
        sxy = sum((i - mx) * (v - my) for i, v in enumerate(a)); sxx = sum((i - mx) ** 2 for i in range(m)); syy = sum((v - my) ** 2 for v in a)
        return sxy / (sxx * syy) ** 0.5
    print("  corr(position, rank): first half %.3f, second half %.3f, whole %.3f" % (corr(fh), corr(sh), corr(r)))
    # how many fixed points
    print("  fixed points:", sum(1 for i, v in enumerate(r) if i == v))
    # decile profile of where each position decile's values come from
    d = n // 10
    print("  mean rank/n by position decile:", [round(statistics.mean(r[k * d:(k + 1) * d]) / n, 3) for k in range(10)])
