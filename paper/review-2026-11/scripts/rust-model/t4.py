import sys; sys.path.insert(0, "/private/tmp/claude-501/-Users-vigna-pub-zfast/955cf1ac-f832-4902-86ae-acdd82779e8c/scratchpad/review-model")
import random
from model import *
def rnd(rng, L): return format(rng.getrandbits(L), f'0{L}b')
# long handles: G random 100-bit group prefixes, two keys per group -> one internal node per group with a 64-bit handle
for G in (100000, 200000):
    rng = random.Random(G)
    keys = []
    for _ in range(G):
        p = rnd(rng, 100)
        keys += [p + '0' + rnd(rng, 8), p + '1' + rnd(rng, 8)]
    t = Trie(keys)
    h64 = sum(1 for h in t.handles if len(h) == 64)
    bad = sum(t.query(k) != naive(t.keys, k) for k in keys)
    print(f"G={G}: internal {len(t.internal)}, 64-bit handles {h64}, overwritten {t.lost} (birthday estimate {h64*h64/2/M_DEF:.1f}), wrong answers on key-as-query {bad}/{len(keys)}")
