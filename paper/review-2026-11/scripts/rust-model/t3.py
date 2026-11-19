import sys; sys.path.insert(0, "/private/tmp/claude-501/-Users-vigna-pub-zfast/955cf1ac-f832-4902-86ae-acdd82779e8c/scratchpad/review-model")
import random, time
from model import *

def rnd(rng, L):
    return format(rng.getrandbits(L), f'0{L}b') if L else ''

# C. the shape of lib.rs test_z_fast_fixed_small / bench *_fixed_small: random fixed-length keys, HashMap + RollingHash(37, 1e9+7)
for n, L, seeds in ((100000, 40, 3), (1000, 200, 3)):
    for seed in range(seeds):
        rng = random.Random(seed)
        keys = sorted({rnd(rng, L) for _ in range(n)})
        t = Trie(keys)
        tot = bad = 0
        for _ in range(100000 if n > 1000 else 5000):
            x = rnd(rng, L)
            tot += 1
            bad += t.query(x) != naive(t.keys, x)
        # queries aimed at the lost nodes' subtrees: all keys, as exact-match queries
        badk = sum(t.query(k) != naive(t.keys, k) for k in keys)
        print(f"hashmap n={len(keys)} bits={L} seed={seed}: overwritten z-map entries {t.lost}; wrong on random queries {bad}/{tot}; wrong on key-as-query {badk}/{len(keys)}")

# D. MPHF without signatures: how often does the first (unchecked) search have to be redone?
for n, L, var in ((1000, 200, False), (1000, 30, False), (1000, 200, True), (5000, 10000, False), (100000, 40, False)):
    rng = random.Random(3)
    if var:
        keys = []
        while len(keys) < n:
            s = rnd(rng, rng.randrange(L // 4, L))
            if not any(s.startswith(k) or k.startswith(s) for k in keys): keys.append(s)
    else:
        keys = list({rnd(rng, L) for _ in range(n)})
    t = Trie(keys, mph_model=True, rng=rng)
    q = 3000
    wrong = 0
    for _ in range(q):
        x = rnd(rng, rng.randrange(L // 4, L) if var else L)
        wrong += t.query(x) != naive(t.keys, x)
    print(f"mph-model n={len(keys)} bits={L} var={var}: fallback (second, exact search) in {100*t.fallbacks/q:.1f}% of queries; probes/query {t.probes/q:.1f}; wrong {wrong}")
