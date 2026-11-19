import sys; sys.path.insert(0, "/private/tmp/claude-501/-Users-vigna-pub-zfast/955cf1ac-f832-4902-86ae-acdd82779e8c/scratchpad/review-model")
import random
from model import *
import t1b_helpers as H

# A. hashmap model, mismatches per key distribution
for mode in ['fixed', 'var', 'deep', 'runs']:
    rng = random.Random(5)
    tot = bad = lost = nodes = 0
    for _ in range(150):
        keys = H.gen_keys(rng, rng.choice([5, 17, 100, 400]), mode)
        if len(keys) < 2: continue
        t = Trie(keys)
        lost += t.lost; nodes += len(t.internal)
        for _ in range(200):
            x = H.gen_query(rng, keys)
            tot += 1
            bad += t.query(x) != naive(t.keys, x)
    print(f"hashmap/{mode}: internal nodes {nodes}, overwritten z-map entries {lost}, wrong pred/succ {bad}/{tot}")

# B. the 4-key counterexample
keys = ["1", "01", "0" * 65, "0" * 64 + "1"]
t = Trie(keys)
print("4 keys: handles", sorted((len(h), slow_hash(h, len(h))) for h in t.handles), "zmap size", len(t.zmap), "lost", t.lost)
print("  query '00':", tuple(None if v is None else (v if len(v) < 8 else f"0^{v.count('0')}{'1' if v.endswith('1') else ''}") for v in t.query("00")),
      " expected:", naive(t.keys, "00")[0], "/ 0^65")
print("  ex_pref('00'):", t.pref("00"), "expected", naive_pref(t.keys, "00"))
# seed-independent: every base, the MPHF modulus n = 101
print("  hash('0')==hash(0^64) for every base mod 101:", all(slow_hash("0", 1, b, 101) == slow_hash("0" * 64, 64, b, 101) for b in range(1, 101)))
print("  ... and for 1000 random (base, modulus):", all(slow_hash("0", 1, b, m) == slow_hash("0" * 64, 64, b, m) for b, m in ((random.randrange(1, 10**9), random.randrange(2, 2**31)) for _ in range(1000))))
# all-ones word: digit 0 in release (wrap), overflow in debug
print("  release: hash(1^64)==hash('') :", slow_hash("1" * 64, 64) == slow_hash("", 0), "; hash(s.1^64)==hash(s):", slow_hash("01" * 32 + "1" * 64, 128) == slow_hash("01" * 32, 64))
try:
    slow_hash("1" * 64, 64, release=False); print("  debug: no overflow")
except Overflow:
    print("  debug: x + 1 overflows on an all-ones word")
keys = ["0", "1" * 64 + "0", "1" * 64 + "1"]
t = Trie(keys)
print("3 keys {0, 1^64 0, 1^64 1} (release): lost", t.lost, "; query '10' ->", tuple(None if v is None else len(v) for v in t.query("10")), "(lengths) expected", tuple(None if v is None else len(v) for v in naive(t.keys, "10")))
# congruent words mod n
n = 1230
w1 = random.getrandbits(64); w2 = (w1 + 7 * n) & MASK64
s1 = ''.join('1' if (w1 >> i) & 1 else '0' for i in range(64)); s2 = ''.join('1' if (w2 >> i) & 1 else '0' for i in range(64))
print("  two 64-bit handles with w1 = w2 mod n collide for every base mod n:", s1 != s2 and all(slow_hash(s1, 64, b, n) == slow_hash(s2, 64, b, n) for b in range(1, n)))
