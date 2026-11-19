import random, collections
from zf import *
from gen import *

def exits_left(x, e):
    if e.ext.startswith(x): return True
    w = lcp(x, e.ext)
    return len(w) < len(x) and len(w) < len(e.ext) and x[len(w)] == '0'

def search_spine(start, k, side, cnt=None):
    # follow jump pointers from 'start' down its side-spine until a node intercepting k
    m = start; j = 0
    while not m.intercepts(k):
        m = getattr(m, side); j += 1
    if cnt is not None: cnt.append(j)
    return m

def insert(t, x, recompute_old, literal_alpha, stats):
    eps = t.exit(x); gamma = t.parent(eps)
    c = len(lcp(x, eps.ext))
    if c == len(x) or (eps.leaf() and c == len(eps.ext)): return None   # not insertable
    sigma, _ = t.locate_parex(x)            # 2-fat ancestors of old parex
    assert (sigma[-1] if sigma else None) is gamma
    xleft = exits_left(x, eps)
    was_left = gamma is not None and gamma.left is eps
    cutlow = c >= eps.hl
    old_hl = eps.hl
    # --- structure
    eta = Node(eps.name, eps.ext[:c]); lam = Node(x[:c+1], x)
    eps.name = eps.ext[:c+1]
    if xleft: eta.left, eta.right = lam, eps
    else: eta.left, eta.right = eps, lam
    if gamma is None: t.root = eta
    elif was_left: gamma.left = eta
    else: gamma.right = eta
    # --- jump pointers of the new node
    k = jump(eta.hl)
    cnt = []
    if xleft:
        eta.jl = lam; eta.jr = search_spine(eps, k, 'jr', cnt)
    else:
        eta.jr = lam; eta.jl = search_spine(eps, k, 'jl', cnt)
    stats['newnode_jumps_minus_lgx'][cnt[0] - (len(x).bit_length() - 1)] += 1
    if gamma is None: stats['newroot_jumps'].append((cnt[0], len(x)))
    # --- old exit node (NOT in the paper)
    if recompute_old and not eps.leaf() and eps.hl != old_hl:
        k = jump(eps.hl)
        eps.jl = search_spine(eps.left, k, 'jl'); eps.jr = search_spine(eps.right, k, 'jr')
    # --- Algorithm jumpupda (and its mirror)
    if gamma is not None:
        J = 'jl' if was_left else 'jr'
        toward_new_leaf = (xleft == was_left)     # "x exits on the left" in the left-child case
        S = list(sigma)
        while S and getattr(S[-1], J) is eps and jump(S[-1].hl) < len(lam.name):
            a = S.pop(); setattr(a, J, eta)
        if toward_new_leaf:
            b = eps
            done = False
            while S and not done:
                a = S.pop()
                if literal_alpha:
                    while (not a.leaf()) and getattr(a, J) is not b:
                        b = getattr(b, J)        # crashes when b is a leaf
                else:
                    while (not b.leaf()) and getattr(a, J) is not b:
                        b = getattr(b, J)
                if getattr(a, J) is not b: done = True
                else: setattr(a, J, lam)
    return eps, eta, lam, cutlow

def compare(t, strs):
    ref = Trie(strs); ref.set_jumps()
    byname = {n.name: n for n in ref.nodes()}
    bad = []
    for n in t.nodes():
        r = byname[n.name]
        assert r.ext == n.ext
        if n.leaf(): continue
        if n.jl.name != r.jl.name or n.jr.name != r.jr.name: bad.append(n)
    return bad

for recompute_old, literal in ((True, False), (False, False), (True, True)):
    rng = random.Random(11)
    stats = collections.defaultdict(collections.Counter); stats['newroot_jumps'] = []
    res = collections.Counter(); ex = None
    for trial in range(4000):
        strs = sorted(gen(rng, maxdepth=rng.randint(1,7), maxpath=rng.choice([3,9,20])))
        if len(strs) < 2: continue
        for _ in range(6):
            x = randq(rng, strs)
            if x == '' or x in strs: continue
            t = Trie(strs); t.set_jumps()
            try:
                r = insert(t, x, recompute_old, literal, stats)
            except AttributeError as e:
                res['CRASH'] += 1; continue
            if r is None: continue
            eps, eta, lam, cutlow = r
            bad = compare(t, strs + [x])
            if not bad: res['ok'] += 1
            else:
                kinds = set()
                for n in bad:
                    kinds.add('old-exit' if n is eps else 'new' if n is eta else 'other')
                res['bad:' + '+'.join(sorted(kinds)) + (':cutlow' if cutlow else ':cuthigh')] += 1
                if ex is None or len(strs) < len(ex[0]): ex = (strs, x, bad)
    print('recompute_old=%s literal_alpha=%s' % (recompute_old, literal), dict(res))
    if ex: print('   example', ex)
    if recompute_old and not literal:
        print('   new-node search jumps minus floor(lg|x|):', sorted(stats['newnode_jumps_minus_lgx'].items()))
        nr = stats['newroot_jumps']
        print('   new root cases:', len(nr), 'max jumps', max(j for j, l in nr), 'worst (jumps,|x|)', max(nr, key=lambda p: p[0] - p[1].bit_length()))
