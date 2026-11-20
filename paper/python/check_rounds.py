# Randomized check of the probe-tree theorems: the answers of the z-map are arbitrary
# (any hit returns an extent length >= f), which covers every trie and every false positive.
import random, math
def fattest(a, b):
    if a == 0: return 0
    return (-1 << ((a - 1) ^ b).bit_length() - 1) & b
def tree(a, b, depth=0, out=None, parent=None, par=None):
    # returns dict node -> depth, and dict node -> parent, of T[a..b]
    if out is None: out, par = {}, {}
    if a > b: return out, par
    f = fattest(a, b)
    out[f] = depth; par[f] = parent
    tree(a, f - 1, depth + 1, out, f, par); tree(f + 1, b, depth + 1, out, f, par)
    return out, par
def is_proper_desc(par, v, u):
    v = par[v]
    while v is not None:
        if v == u: return True
        v = par[v]
    return False
random.seed(1)
worst = {}
for trial in range(20000):
    m = random.randint(1, 600); s = random.randint(1, 5); p = random.random()
    depth, par = tree(0, m)
    assert max(depth.values()) <= int(math.log2(m)) + 1
    answers = {}
    def Z(g):
        if g not in answers:
            answers[g] = random.randint(g, m + 3) if random.random() < p else None
        return answers[g]
    # sequential
    a, b, seq = 0, m, []
    while a <= b:
        f = fattest(a, b); seq.append(f); e = Z(f)
        if e is not None: a = e + 1
        else: b = f - 1
    for u, v in zip(seq, seq[1:]): assert is_proper_desc(par, v, u), (m, seq)
    assert seq[0] == 0 and len(seq) <= int(math.log2(m)) + 2
    # in rounds
    a, b, par_seq, rounds, maxq = 0, m, [], 0, 0
    while a <= b:
        d, _ = tree(a, b); P = {g for g in d if d[g] < s}; maxq = max(maxq, len(P)); rounds += 1
        f = fattest(a, b)
        while True:
            assert f in P
            par_seq.append(f); e = Z(f)
            if e is not None: a = e + 1
            else: b = f - 1
            if a > b: break
            f = fattest(a, b)
            if f not in P: break
    assert par_seq == seq
    bound = -(-(int(math.log2(m)) + 2) // s)
    assert rounds <= bound and maxq <= 2 ** s - 1, (m, s, rounds, bound)
    worst[s] = max(worst.get(s, 0), rounds / bound)
print("ok", {s: round(v, 2) for s, v in sorted(worst.items())})
