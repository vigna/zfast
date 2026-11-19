import random
def gen(rng, depth=0, maxdepth=7, maxpath=9, prefix=''):
    k = rng.choice([0,0,0,1,1,2,3,rng.randint(0,maxpath)])
    p = prefix + ''.join(rng.choice('01') for _ in range(k))
    if depth >= maxdepth or (depth > 0 and rng.random() < 0.25):
        return [p]
    return gen(rng, depth+1, maxdepth, maxpath, p+'0') + gen(rng, depth+1, maxdepth, maxpath, p+'1')
def randq(rng, strs):
    # query: random prefix of a key, possibly mutated/extended
    s = rng.choice(strs)
    l = rng.randint(0, len(s))
    q = s[:l] + ''.join(rng.choice('01') for _ in range(rng.choice([0,0,1,2,5])))
    if rng.random() < 0.2: q = s + ''.join(rng.choice('01') for _ in range(rng.randint(0,3)))
    return q
