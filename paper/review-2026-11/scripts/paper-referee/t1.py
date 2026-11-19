from zf import *
strs = [l.strip() for l in open('/Users/vigna/pub/zfast/fig/large-example.txt') if l.strip()]
t = Trie(strs)
for n in t.nodes():
    if not n.leaf(): print(n.label, 'name', n.name, 'ext', n.ext, 'skip [%d..%d]' % (n.i, n.j), 'handle', n.handle, n.hl, 'L' if True else '')
x = '01110101111100001100'
e = t.exit(x); print('exit', e, 'parex', t.parent(e), 'cut', len(lcp(x, e.ext)), 'hl', e.hl)
tr = []; st, it = t.locate_parex(x, tr)
for r in tr: print(r)
print('iters', it, st)
tr = []; st, it = t.locate_eop(x, tr)
for r in tr: print(r)
print('iters', it, st)
y = '0111010111100101110'
e = t.exit(y); print('INSERT exit', e, 'parex', t.parent(e), 'cut', len(lcp(y, e.ext)), 'hl', e.hl, 'is left child', t.parent(e).left is e)
t2 = Trie(strs + [y])
for n in t2.nodes():
    if not n.leaf(): print(n.label, 'name', n.name, 'ext', n.ext, 'skip [%d..%d]' % (n.i, n.j), 'handle', n.handle, n.hl)
