import random
import nbfixed as nb
from zf import Trie as T2
from gen import *
rng = random.Random(7); bad = tot = 0
for trial in range(1500):
    strs = sorted(gen(rng, maxdepth=rng.randint(1,7), maxpath=rng.choice([3,9,20])))
    if len(strs) < 2: continue
    t = nb.Trie(strs); t2 = T2(strs)
    for _ in range(30):
        x = randq(rng, strs)
        if x == '': continue
        tot += 1
        p = t2.parent(t2.exit(x))
        r = t.search(x, needsExit=False)[0]
        if (None if r is None else r.extent) != (None if p is None else p.ext): bad += 1
print('with a = |e|+1: mismatches', bad, 'of', tot)
