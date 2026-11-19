import random, collections
from zf import *
from gen import *
from t6 import search_spine, compare

def delete(t, x, variant, recompute_sib):
    lam = t.exit(x); assert lam.leaf() and lam.ext == x
    eta = t.parent(lam)
    xi = eta.right if eta.left is lam else eta.left
    lam_left = eta.left is lam
    g = t.parent(eta)
    S = t.fat_anc(g) if g is not None else []
    eta_left = g is not None and g.left is eta
    old_hl = xi.hl
    # structure
    xi.name = eta.name
    if g is None: t.root = xi
    elif eta_left: g.left = xi
    else: g.right = xi
    changed = (not xi.leaf()) and xi.hl != old_hl
    if recompute_sib and changed:
        k = jump(xi.hl)
        xi.jl = search_spine(xi.left, k, 'jl'); xi.jr = search_spine(xi.right, k, 'jr')
    if g is not None:
        J = 'jl' if eta_left else 'jr'
        while S and getattr(S[-1], J) is eta:
            a = S.pop(); setattr(a, J, xi)
        if variant == 'corrected':
            if lam_left == eta_left:        # deleted leaf was on the same side
                cur = xi
                while S and getattr(S[-1], J) is lam:
                    a = S.pop()
                    while not cur.intercepts(jump(a.hl)): cur = getattr(cur, J)
                    setattr(a, J, cur)
        else:  # literal: "if lambda is a right child" + J^+ pointers, in the left-child case (mirrored otherwise)
            O = 'jr' if eta_left else 'jl'
            if lam_left != eta_left:
                cur = xi
                while S and getattr(S[-1], O) is lam:
                    a = S.pop()
                    while not cur.intercepts(jump(len(a.ext))): cur = getattr(cur, O)
                    setattr(a, O, cur)
    return changed, lam

for variant, rec in (('corrected', True), ('corrected', False), ('literal', True)):
    rng = random.Random(13); res = collections.Counter(); ex = None
    for trial in range(4000):
        strs = sorted(gen(rng, maxdepth=rng.randint(1,7), maxpath=rng.choice([3,9,20])))
        if len(strs) < 3: continue
        for _ in range(4):
            x = rng.choice(strs)
            t = Trie(strs); t.set_jumps()
            changed, lam = delete(t, x, variant, rec)
            rest = [s for s in strs if s != x]
            # dangling pointers to the deleted leaf?
            dang = [n for n in t.nodes() if not n.leaf() and (n.jl is lam or n.jr is lam)]
            bad = compare_safe = None
            ref = Trie(rest); ref.set_jumps(); byname = {n.name: n for n in ref.nodes()}
            bad = [n for n in t.nodes() if not n.leaf() and (n.jl.name != byname[n.name].jl.name or n.jr.name != byname[n.name].jr.name or n.jl is lam or n.jr is lam)]
            if not bad: res['ok'] += 1
            else:
                res['bad' + (':dangling' if dang else '') + (':sib-handle-changed' if changed else '')] += 1
                if ex is None or len(strs) < len(ex[0]): ex = (strs, x, bad)
    print(variant, 'recompute_sibling=%s' % rec, dict(res))
    if ex: print('   example', ex)
