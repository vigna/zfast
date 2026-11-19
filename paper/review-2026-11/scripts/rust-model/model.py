# Independent Python model of the reviewed crate's logic (written from reading the
# source; does not import or execute anything from the checkout).
import random, sys, bisect
sys.setrecursionlimit(100000)
MASK64 = (1 << 64) - 1
M_DEF, B_DEF = 10**9 + 7, 37

def word(s, j):
    w = 0
    for i, c in enumerate(s[64 * j:64 * j + 64]):
        if c == '1':
            w |= 1 << i
    return w

class Overflow(Exception):
    pass

def slow_hash(s, ind, base=B_DEF, mod=M_DEF, release=True):
    res = base
    pot = base * base % mod
    cur = 0
    for j in range((len(s) + 63) // 64):
        if cur + 64 > ind:
            break
        cur += 64
        x = word(s, j)
        if x == MASK64 and not release:
            raise Overflow()
        res += (((x + 1) & MASK64) % mod) * pot % mod
        res %= mod
        pot = pot * base % mod
    while cur < ind:
        res = (res + pot) % mod
        if s[cur] == '1':
            res = (res + pot) % mod
        pot = pot * base % mod
        cur += 1
    return res

def compute_state(s, base=B_DEF, mod=M_DEF):
    res = base
    pot = base * base % mod
    v, pots = [res], [pot]
    for j in range((len(s) + 63) // 64):
        x = word(s, j)
        res = (res + (((x + 1) & MASK64) % mod) * pot % mod) % mod
        v.append(res)
        pot = pot * base % mod
        pots.append(pot)
    return v, pots

def fast_hash(s, state, ind, base=B_DEF, mod=M_DEF):
    res = state[0][ind // 64]
    pot = state[1][ind // 64]
    cur = (ind // 64) * 64
    while cur < ind:
        res += pot
        if s[cur] == '1':
            res += pot
        pot = pot * base % mod
        cur += 1
    return res % mod

def npot(x):
    return 1 if x <= 1 else 1 << (x - 1).bit_length()

def get_fattest(x, low):
    assert x >= low
    if low == 0:
        return 0
    i = (low - 1) ^ x
    m = ~((npot(i + 1) >> 1) - 1) & MASK64
    return m & x

def calc(i):
    return ~(npot(i + 1) - 1) & MASK64

def fattest_ref(a, b):  # 2-fattest in (a..b]
    return b & (MASK64 << ((a ^ b).bit_length() - 1)) & MASK64

class Node:
    __slots__ = ('left', 'right', 'lind', 'rind', 'jl', 'jr', 'to_leaf', 'key', 'prev', 'next', 'id')
    def __init__(self):
        self.left = self.right = self.jl = self.jr = self.to_leaf = self.key = self.prev = self.next = None

def lcp_len(a, b):
    n = min(len(a), len(b))
    i = 0
    while i < n and a[i] == b[i]:
        i += 1
    return i

class Trie:
    def __init__(self, keys, mph_model=False, rng=None):
        self.keys = sorted(keys)
        self.leaves = []
        self.internal = []
        self.mph_model = mph_model
        self.rng = rng or random.Random(1)
        self.root = self._build(0, len(self.keys), 0)
        for a, b in zip(self.leaves, self.leaves[1:]):
            a.next, b.prev = b, a
        self.zmap = {}
        self.lost = 0
        self.handles = {}
        self._zmap(self.root)
        self._jumps(self.root)
        self.fallbacks = 0
        self.probes = 0

    def _build(self, l, r, lind):
        nd = Node()
        nd.lind = lind
        if l + 1 == r:
            nd.key = self.keys[l]
            nd.rind = len(nd.key)
            self.leaves.append(nd)
            return nd
        e = lcp_len(self.keys[l], self.keys[r - 1])
        assert e < len(self.keys[l]), "not prefix free"
        nd.rind = e
        mid = l
        while self.keys[mid][e] == '0':
            mid += 1
        nd.left = self._build(l, mid, e + 1)
        nd.to_leaf = self.leaves[-1]      # rightmost leaf of left subtree
        nd.right = self._build(mid, r, e + 1)
        self.internal.append(nd)
        return nd

    def extent(self, nd):
        return nd.key if nd.key is not None else nd.to_leaf.key[:nd.rind]

    def handle(self, nd):
        return nd.to_leaf.key[:get_fattest(nd.rind, nd.lind)]

    def _zmap(self, nd):
        if nd.key is not None:
            return
        h = self.handle(nd)
        hv = slow_hash(h, len(h))
        self.handles[h] = nd
        if hv in self.zmap:
            self.lost += 1
        self.zmap[hv] = nd
        self._zmap(nd.left)
        self._zmap(nd.right)

    def _kth(self, nd, k, left):
        while True:
            if nd.lind <= k <= nd.rind or nd.key is not None:
                return nd
            nd = nd.left if left else nd.right

    def _jumps(self, nd):
        if nd.key is not None:
            return
        k = get_fattest(nd.rind, nd.lind)
        kk = MASK64 if k == 0 else k + (k & -k)
        nd.jl = self._kth(nd, kk, True)
        nd.jr = self._kth(nd, kk, False)
        assert nd.jl is not nd and nd.jr is not nd
        self._jumps(nd.left)
        self._jumps(nd.right)

    def zget(self, x, state, f):
        self.probes += 1
        if self.mph_model:  # MPHF: true handles map to their node, anything else to an arbitrary node
            p = x[:f]
            nd = self.handles.get(p)
            return nd if nd is not None else self.rng.choice(self.internal)
        return self.zmap.get(fast_hash(x, state, f))

    def _search(self, x, exact):
        res = None
        a, b = 0, len(x)
        state = compute_state(x)
        m = calc(b)
        it = 0
        while a <= b:
            it += 1
            assert it <= 130, "no termination"
            if a == 0 or (m & (a - 1)) != (m & b):
                f = 0 if a == 0 else m & b
                nd = self.zget(x, state, f)
                ok = False
                if nd is not None:
                    if exact:
                        e = self.extent(nd)
                        ok = len(e) < len(x) and get_fattest(nd.rind, nd.lind) == f and e == x[:len(e)]
                    else:
                        ok = get_fattest(nd.rind, nd.lind) == f
                if ok:
                    a = nd.rind + 1
                    res = nd
                else:
                    if f == 0:
                        break
                    b = f - 1
            m >>= 1
        return res

    def locate_exit(self, x):
        res = self._search(x, False)
        h = self.handle(res) if res is not None else ''
        if h != x[:min(len(x), len(h))]:
            self.fallbacks += 1
            res = self._search(x, True)
        if res is None:
            return self.root
        e = self.extent(res)
        if len(e) < len(x) and e == x[:len(e)]:
            return res.left if x[len(e)] == '0' else res.right
        return res

    def query(self, x):
        eta = self.locate_exit(x)
        if not (x > self.extent(eta)):
            lf = eta
            while lf.key is None:
                lf = lf.jl
            return (lf.prev.key if lf.prev else None, lf.key)
        lf = eta
        while lf.key is None:
            lf = lf.jr
        return (lf.key, lf.next.key if lf.next else None)

    def is_nonempty(self, x, y):
        if x == y:
            return False
        al = self.locate_exit(x)
        if not (x > self.extent(al)):
            while al.key is None and len(self.extent(al)) < len(y):
                al = al.jl
            return self.extent(al) < y
        be = self.locate_exit(y)
        if x > self.extent(be):
            while be.key is None and len(self.extent(be)) < len(x):
                be = be.jr
            return not (x > self.extent(be))
        z = x[:lcp_len(x, y)]
        eta = self.locate_exit(z)
        al = eta.left      # None for a leaf -> the crate's unwrap would panic
        if al is None:
            raise RuntimeError("unwrap on leaf in case 2b")
        while al.key is None and len(self.extent(al)) < len(x):
            al = al.jr
        if not (x > self.extent(al)):
            return True
        be = eta.right
        while be.key is None and len(self.extent(be)) < len(y):
            be = be.jl
        return self.extent(be) < y

    def pref(self, x):
        eta = self.locate_exit(x)
        e = self.extent(eta)[:len(x)]
        return len(e) == len(x) and e == x

def naive(keys_sorted, x):
    i = bisect.bisect_left(keys_sorted, x)
    return (keys_sorted[i - 1] if i > 0 else None, keys_sorted[i] if i < len(keys_sorted) else None)

def naive_range(keys_sorted, x, y):
    i = bisect.bisect_left(keys_sorted, x)
    return i < len(keys_sorted) and keys_sorted[i] < y

def naive_pref(keys_sorted, x):
    i = bisect.bisect_left(keys_sorted, x)
    return i < len(keys_sorted) and keys_sorted[i].startswith(x)
