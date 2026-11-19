import sys, random
def fatness(x):
    if x == 0: return 10**9
    return (x & -x).bit_length() - 1
def fattest(a, b):
    assert a <= b
    if a == 0: return 0
    return (-1 << ((a - 1) ^ b).bit_length() - 1) & b
def jump(x):
    if x == 0: return float('inf')
    return x + (x & -x)
def backjump(x):
    if x == 0: return 0
    return x - (x & -x)
def backjumps(x):
    s = [x]
    while x > 0:
        x = backjump(x); s.append(x)
    return s
def lcp(a, b):
    i = 0
    while i < len(a) and i < len(b) and a[i] == b[i]: i += 1
    return a[:i]

class Node:
    def __init__(s, name, ext):
        s.name = name; s.ext = ext; s.left = s.right = None
        s.jl = s.jr = None; s.prev = s.next = None; s.label = None
    @property
    def i(s): return len(s.name)
    @property
    def j(s): return len(s.ext)
    @property
    def hl(s): return fattest(s.i, s.j)
    @property
    def handle(s): return s.ext[:s.hl]
    def leaf(s): return s.left is None
    def intercepts(s, k):
        return k >= s.i and (s.leaf() or k <= s.j)
    def __repr__(s): return (s.label or '') + '<%s|%s>' % (s.name, s.ext[len(s.name):])

def build(strs, name=''):
    strs = sorted(strs)
    p = strs[0]
    for x in strs: p = lcp(p, x)
    n = Node(name, p)
    if len(strs) == 1:
        return n
    n.left = build([x for x in strs if x[len(p)] == '0'], p + '0')
    n.right = build([x for x in strs if x[len(p)] == '1'], p + '1')
    return n

class Trie:
    def __init__(s, strs):
        s.root = build(strs)
        s.refresh()
    def nodes(s, n=None):
        n = n or s.root
        yield n
        if not n.leaf():
            yield from s.nodes(n.left); yield from s.nodes(n.right)
    def refresh(s):
        s.z = {}
        k = 0
        for n in s.nodes():
            if not n.leaf():
                assert n.handle not in s.z
                s.z[n.handle] = n
                n.label = 'a%d' % k; k += 1
    def set_jumps(s):
        for n in s.nodes():
            if n.leaf(): continue
            k = jump(n.hl)
            t = n.left
            while not t.intercepts(k): t = t.left
            n.jl = t
            t = n.right
            while not t.intercepts(k): t = t.right
            n.jr = t
    def exit(s, x):
        n = s.root
        while True:
            if n.leaf(): return n
            if x.startswith(n.ext) and x != n.ext:
                n = n.left if x[len(n.ext)] == '0' else n.right
            else: return n
    def parent(s, m):
        n = s.root; p = None
        while n is not m:
            p = n
            n = n.left if m.name[len(n.ext)] == '0' else n.right
        return p
    def ancestors(s, m):   # proper ancestors+self from root
        n = s.root; r = [n]
        while n is not m:
            n = n.left if m.name[len(n.ext)] == '0' else n.right
            r.append(n)
        return r
    def fat_anc(s, m):
        bj = set(backjumps(m.hl))
        return [a for a in s.ancestors(m) if a.hl in bj]
    def locate_parex(s, x, trace=None):
        st = []; a, b = 0, len(x); it = 0
        while a <= b:
            it += 1
            f = fattest(a, b)
            be = s.z.get(x[:f])
            ok = be is not None and x.startswith(be.ext) and x != be.ext
            if trace is not None: trace.append((a, b, f, be, ok))
            if ok:
                a = len(be.ext) + 1; st.append(be)
            else: b = f - 1
        if trace is not None: trace.append((a, b, None, None, None))
        return st, it
    def locate_eop(s, x, trace=None, a=0, b=None, st=None):
        st = list(st or []); 
        if b is None: b = len(x)
        it = 0
        while a <= b:
            it += 1
            f = fattest(a, b)
            be = s.z.get(x[:f])
            if trace is not None: trace.append((a, b, f, be))
            if be is not None:
                a = len(be.ext) + 1; st.append(be)
            else: b = f - 1
        if trace is not None: trace.append((a, b, None, None))
        return st, it
