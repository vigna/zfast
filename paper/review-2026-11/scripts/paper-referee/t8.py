import random, collections
from zf import *
from gen import *
rng = random.Random(21)
c = collections.Counter(); worst_root = (0, None)
for trial in range(4000):
    strs = sorted(gen(rng, maxdepth=rng.randint(1,7), maxpath=rng.choice([3,9,20,60])))
    if len(strs) < 2: continue
    t = Trie(strs)
    for n in t.nodes():
        if n.leaf() or n is t.root: continue
        g = t.parent(n)
        fa = t.fat_anc(g)
        anc = t.ancestors(g)
        # crossing nodes == 2-fat ancestors of parent?
        cross = [a for a in anc if jump(a.hl) >= n.i]
        c['cross==fat' if cross == fa else 'cross!=fat'] += 1
        js = [jump(a.hl) for a in fa]
        c['jumps monotone' if all(js[i] >= js[i+1] for i in range(len(js)-1)) else 'jumps NOT monotone'] += 1
        # "at least as fat" intuition
        loose = [a for a in t.ancestors(n) if fatness(a.hl) >= fatness(n.hl)]
        c['intuition ok' if loose == t.fat_anc(n) else 'intuition differs'] += 1
    for _ in range(20):
        x = randq(rng, strs)
        if x == '' or x in strs: continue
        e = t.exit(x); cut = len(lcp(x, e.ext))
        if cut == len(x) or e.leaf() or cut < e.hl: continue
        f2 = fattest(cut + 1, e.j)      # new handle length of old exit node (cuts low, internal)
        if e is t.root:
            c['root cutlow'] += 1
            if f2 - 2 * len(x) > worst_root[0]: worst_root = (f2 - 2 * len(x), (len(x), f2, len(e.ext)))
            if f2 >= 2 * len(x): c['root: new handle >= 2|x|'] += 1
        else:
            c['nonroot cutlow'] += 1
            if f2 >= 2 * len(x): c['nonroot: new handle >= 2|x|'] += 1
print(dict(c)); print('worst root (|x|, new handle len, |e_root|):', worst_root)
