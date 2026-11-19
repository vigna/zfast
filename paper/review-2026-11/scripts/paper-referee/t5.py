import random
from zf import *
from gen import *

def parent_stack(t, st, bmode):
    st = list(st)
    sigma = st.pop()
    if not st: return None, 'root'
    a = len(st[-1].ext) + 1
    b = len(sigma.ext) - 1 if bmode == 'paper' else len(sigma.name) - 1
    x = sigma.ext
    while a <= b:
        f = fattest(a, b)
        be = t.z.get(x[:f])
        if be is None: return None, 'bot'
        st.append(be); a = len(be.ext) + 1
    return st, 'ok'

rng = random.Random(5)
res = {'paper': {}, 'fixed': {}}
proof_bad = proof_tot = 0; ex = None; exb = None
for trial in range(3000):
    strs = sorted(gen(rng, maxdepth=rng.randint(1,7), maxpath=rng.choice([3,9,20])))
    if len(strs) < 2: continue
    t = Trie(strs)
    for n in t.nodes():
        if n.leaf() or n is t.root: continue
        st = t.fat_anc(n); tau = t.parent(n); exp = t.fat_anc(tau)
        for mode in ('paper', 'fixed'):
            got, why = parent_stack(t, st, mode)
            k = why if got is None else ('correct' if got == exp else 'wrong')
            res[mode][k] = res[mode].get(k, 0) + 1
            if mode == 'paper' and k != 'correct' and (exb is None or len(strs) < len(exb[0])):
                exb = (strs, n, k, got, exp)
        # proof claim: |h_{alpha_{t-1}}| == (-1 << msb(|h_s| ^ |h_tau|)) & |h_tau|
        s_, t_ = n.hl, tau.hl
        g = (-1 << ((s_ ^ t_).bit_length() - 1)) & t_
        proof_tot += 1
        if st[-2].hl != g:
            proof_bad += 1
            if ex is None or len(strs) < len(ex[0]): ex = (strs, n, tau, st[-2], st[-2].hl, g)
print(res)
print('example paper-b failure', exb)
print('proof formula mismatches', proof_bad, 'of', proof_tot)
print('example', ex)
