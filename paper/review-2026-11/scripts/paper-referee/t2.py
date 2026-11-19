from zf import *
from gen import *
rng = random.Random(1)
maxratio = 0; worst=None
bad_parex = bad_eop = 0
maxit = {}
for trial in range(3000):
    strs = gen(rng, maxdepth=rng.randint(1,7), maxpath=rng.choice([3,9,20]))
    if len(strs) < 2: continue
    t = Trie(strs)
    for _ in range(40):
        x = randq(rng, strs)
        if x == '': continue
        e = t.exit(x); p = t.parent(e)
        st, it = t.locate_parex(x)
        exp = t.fat_anc(p) if p is not None else []
        if st != exp: bad_parex += 1
        lam = len(x).bit_length() - 1
        maxit[it - lam] = maxit.get(it - lam, 0) + 1
        # exit-or-parex
        cut = len(lcp(x, e.ext))
        sigma = p if (e.leaf() or cut < e.hl) else e
        st2, it2 = t.locate_eop(x)
        exp2 = t.fat_anc(sigma) if sigma is not None else []
        if st2 != exp2: bad_eop += 1
print('bad parex', bad_parex, 'bad eop', bad_eop)
print('iterations minus floor(lg|x|):', sorted(maxit.items()))
