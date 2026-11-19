import statistics, random
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
n = 10**6
for _ in range(n): nxt()          # gaps consumed first, as in get_random_ints
r = list(range(n))
for i in range(n):
    p = nxt() % (n - i)
    r[i], r[p] = r[p], r[i]
for k in (100, 1000, 10000, 100000):
    w = sorted(r[:k])
    print("first %6d inserted keys: median rank/n %.4f, 10th pct %.4f, min %.4f  (uniform shuffle would give ~0.5, ~0.1, ~0)" % (k, w[k // 2] / n, w[k // 10] / n, w[0] / n))
    w = sorted(r[n - k:])
    print("last  %6d inserted keys: median rank/n %.4f, 10th pct %.4f" % (k, w[k // 2] / n, w[k // 10] / n))
# with gap=10 queries (i = 0,10,20,...) the same positions are used
# mean |rank(x[i+1]) - rank(x[i])| / n vs uniform (1/3)
print("mean |rank diff| of consecutive inserts / n: %.3f (uniform shuffle: 0.333)" % (statistics.mean(abs(r[i + 1] - r[i]) for i in range(n - 1)) / n))
h = n // 2
print("  first half: %.3f  second half: %.3f" % (statistics.mean(abs(r[i + 1] - r[i]) for i in range(h - 1)) / n, statistics.mean(abs(r[i + 1] - r[i]) for i in range(h, n - 1)) / n))
