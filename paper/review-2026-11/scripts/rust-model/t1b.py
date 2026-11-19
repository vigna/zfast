import sys; sys.path.insert(0, "/private/tmp/claude-501/-Users-vigna-pub-zfast/955cf1ac-f832-4902-86ae-acdd82779e8c/scratchpad/review-model")
import random, sys
from model import *

# 1. get_fattest against the textbook formula
for low in range(1, 300):
    for x in range(low, 600):
        assert get_fattest(x, low) == fattest_ref(low - 1, x), (low, x)
print("get_fattest == 2-fattest of (low-1..x] for all 1<=low<=x<600; low==0 -> 0")
for i in range(0, 5000):
    assert calc(i) == (MASK64 << i.bit_length()) & MASK64
print("calc(i) == -1 << bitlen(i)")

# 2. slow vs fast prefix hash
rng = random.Random(7)
for _ in range(300):
    L = rng.randrange(0, 400)
    s = ''.join(rng.choice('01') for _ in range(L))
    for base, mod in ((B_DEF, M_DEF), (rng.randrange(1, 100), 101), (rng.randrange(1, 1229), 1230)):
        st = compute_state(s, base, mod)
        for ind in range(L + 1):
            assert slow_hash(s, ind, base, mod) == fast_hash(s, st, ind, base, mod)
print("slow_prefix_hash == fast_prefix_hash on random strings, all prefix lengths")

def gen_keys(rng, n, mode):
    keys = set()
    if mode == 'fixed':
        L = rng.choice([5, 8, 30, 70, 130, 200])
        while len(keys) < n and len(keys) < 2 ** L // 2:
            keys.add(''.join(rng.choice('01') for _ in range(L)))
        return sorted(keys)
    # variable length, prefix free, optionally with long shared prefixes
    out = []
    tries = 0
    while len(out) < n and tries < 20 * n:
        tries += 1
        if mode == 'deep' and out and rng.random() < 0.7:
            b = rng.choice(out)
            cut = rng.randrange(0, len(b))
            s = b[:cut] + ('1' if b[cut] == '0' else '0') + ''.join(rng.choice('01') for _ in range(rng.randrange(0, 150)))
        elif mode == 'runs':
            s = ''.join(rng.choice(['0' * rng.randrange(1, 90), '1' * rng.randrange(1, 90)]) for _ in range(rng.randrange(1, 5)))
        else:
            s = ''.join(rng.choice('01') for _ in range(rng.randrange(1, 200)))
        if any(s.startswith(k) or k.startswith(s) for k in out):
            continue
        out.append(s)
    return sorted(out)

def gen_query(rng, keys):
    r = rng.random()
    k = rng.choice(keys)
    if r < 0.15:
        return k
    if r < 0.35:
        return k[:rng.randrange(0, len(k) + 1)]
    if r < 0.5:
        return k + ''.join(rng.choice('01') for _ in range(rng.randrange(1, 80)))
    if r < 0.8:
        c = rng.randrange(0, len(k))
        return k[:c] + ('1' if k[c] == '0' else '0') + ''.join(rng.choice('01') for _ in range(rng.randrange(0, 80)))
    return ''.join(rng.choice('01') for _ in range(rng.randrange(0, 250)))

def run(mode_dict, rounds, seed):
    rng = random.Random(seed)
    bad = {'pred': 0, 'succ': 0, 'range': 0, 'pref': 0, 'exc': 0}
    total = 0
    fb = pr = q = 0
    for _ in range(rounds):
        n = rng.choice([2, 3, 5, 17, 100, 400])
        keys = gen_keys(rng, n, rng.choice(['fixed', 'var', 'deep', 'runs']))
        if len(keys) < 2:
            continue
        t = Trie(keys, mph_model=(mode_dict == 'mph'), rng=rng)
        if mode_dict == 'exact':
            t.zget = lambda x, st, f, t=t: t.handles.get(x[:f])
        for _ in range(200):
            x, y = gen_query(rng, keys), gen_query(rng, keys)
            if x > y:
                x, y = y, x
            total += 1
            try:
                p, s = t.query(x)
                np_, ns = naive(t.keys, x)
                bad['pred'] += p != np_
                bad['succ'] += s != ns
                bad['range'] += t.is_nonempty(x, y) != naive_range(t.keys, x, y)
                bad['pref'] += t.pref(x) != naive_pref(t.keys, x)
            except Exception as e:
                bad['exc'] += 1
        fb += t.fallbacks; pr += t.probes
    return total, bad, fb, pr

for md in ('exact', 'mph', 'hashmap'):
    total, bad, fb, pr = run(md, 400, 11)
    print(md, "queries", total, "mismatches", bad, "fallback searches", fb, "probes", pr)
