import io, contextlib, random
import nbcode as nb
from zf import Trie as T2, lcp as lcp2
from gen import *
rng = random.Random(7)
bad_search = bad_pred = bad_succ = bad_range = bad_prefix = 0; tot = 0; totr = 0
ex_search = ex_range = None
for trial in range(1500):
    strs = sorted(gen(rng, maxdepth=rng.randint(1,7), maxpath=rng.choice([3,9,20])))
    if len(strs) < 2: continue
    t = nb.Trie(strs); t2 = T2(strs)
    for _ in range(30):
        x = randq(rng, strs)
        if x == '': continue
        tot += 1
        e = t2.exit(x); p = t2.parent(e)
        try:
            r = t.search(x, needsExit=False)[0]
        except Exception as ex:
            r = 'EXC ' + repr(ex)
        got = None if r is None else (r if isinstance(r, str) else r.extent)
        exp = None if p is None else p.ext
        if got != exp:
            bad_search += 1
            if ex_search is None or len(strs) < len(ex_search[0]): ex_search = (strs, x, got, exp)
        # range
        y = randq(rng, strs)
        if y == '' or not x < y: continue
        totr += 1
        with contextlib.redirect_stdout(io.StringIO()):
            try: a = nb.rangeExistential(t, x, y, False)
            except Exception as ex: a = 'EXC'
        expd = any(x <= s < y for s in strs)
        if a != expd:
            bad_range += 1
            if ex_range is None or len(strs) < len(ex_range[0]): ex_range = (strs, x, y, a, expd)
print('search(needsExit=False) mismatches', bad_search, 'of', tot)
print('example', ex_search)
print('rangeExistential mismatches', bad_range, 'of', totr)
print('example', ex_range)
