import random
def gen_keys(rng, n, mode):
    keys = set()
    if mode == 'fixed':
        L = rng.choice([5, 8, 30, 70, 130, 200])
        while len(keys) < n and len(keys) < 2 ** L // 2:
            keys.add(''.join(rng.choice('01') for _ in range(L)))
        return sorted(keys)
    # variable length, prefix free, optionally with long shared prefixes
    out = []
    tries = 0
    while len(out) < n and tries < 20 * n:
        tries += 1
        if mode == 'deep' and out and rng.random() < 0.7:
            b = rng.choice(out)
            cut = rng.randrange(0, len(b))
            s = b[:cut] + ('1' if b[cut] == '0' else '0') + ''.join(rng.choice('01') for _ in range(rng.randrange(0, 150)))
        elif mode == 'runs':
            s = ''.join(rng.choice(['0' * rng.randrange(1, 90), '1' * rng.randrange(1, 90)]) for _ in range(rng.randrange(1, 5)))
        else:
            s = ''.join(rng.choice('01') for _ in range(rng.randrange(1, 200)))
        if any(s.startswith(k) or k.startswith(s) for k in out):
            continue
        out.append(s)
    return sorted(out)

def gen_query(rng, keys):
    r = rng.random()
    k = rng.choice(keys)
    if r < 0.15:
        return k
    if r < 0.35:
        return k[:rng.randrange(0, len(k) + 1)]
    if r < 0.5:
        return k + ''.join(rng.choice('01') for _ in range(rng.randrange(1, 80)))
    if r < 0.8:
        c = rng.randrange(0, len(k))
        return k[:c] + ('1' if k[c] == '0' else '0') + ''.join(rng.choice('01') for _ in range(rng.randrange(0, 80)))
    return ''.join(rng.choice('01') for _ in range(rng.randrange(0, 250)))

