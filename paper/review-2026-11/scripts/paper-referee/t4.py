import random
from zf import *
from gen import *

def exits_left(x, e):
    if e.ext.startswith(x): return True
    w = lcp(x, e.ext)
    return len(w) < len(x) and len(w) < len(e.ext) and x[len(w)] == '0'

def is_nonempty(t, x, y, stats):
    a = t.exit(x)
    if exits_left(x, a):
        j = 0
        while not a.leaf() and len(a.ext) < len(y): a = a.jl; j += 1
        stats.append((j, len(y)))
        return a.ext < y
    b = t.exit(y)
    if not exits_left(y, b):
        j = 0
        while not b.leaf() and len(b.ext) < len(x): b = b.jr; j += 1
        stats.append((j, len(x)))
        return x <= b.ext
    p = lcp(x, y)
    st, _ = t.locate_eop(p)
    eta = st[-1]
    assert eta.ext == p and not eta.leaf(), (eta, p)
    a = eta.left
    while not a.leaf() and len(a.ext) < len(x): a = a.jr
    if x <= a.ext: return True
    b = eta.right
    while not b.leaf() and len(b.ext) < len(y): b = b.jl
    return b.ext < y

rng = random.Random(3)
bad = tot = 0; stats = []
maxj = {}
for trial in range(3000):
    strs = sorted(gen(rng, maxdepth=rng.randint(1,7), maxpath=rng.choice([3,9,20])))
    if len(strs) < 2: continue
    t = Trie(strs); t.set_jumps()
    # jumps to reach leftmost/rightmost leaf from every internal node
    for n in t.nodes():
        if n.leaf(): continue
        for side in ('jl', 'jr'):
            m = n; j = 0
            while not m.leaf(): m = getattr(m, side); j += 1
            L = len(m.ext)
            d = j - (L.bit_length() - 1) if L > 0 else j
            maxj[d] = maxj.get(d, 0) + 1
    for _ in range(40):
        x = randq(rng, strs); y = randq(rng, strs)
        if x == '' or y == '' or not x < y: continue
        tot += 1
        if is_nonempty(t, x, y, stats) != any(x <= s < y for s in strs): bad += 1
print('Algorithm range mismatches', bad, 'of', tot)
print('jumps to leaf minus floor(lg|leaf|):', sorted(maxj.items()))
print('max (jumps - floor lg len) in range walks', max(j - (l.bit_length()-1) for j, l in stats))
