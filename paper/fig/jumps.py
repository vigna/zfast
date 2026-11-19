#!/usr/bin/env python3
# coding: utf-8

import random
import math
import sys


# Returns the binary representation of n; if t>0, padded with zeroes so that its length is not less than t.
def binary(n, t = -1):
    if t > 0:
        return '{0:b}'.format(n).rjust(t, '0')
    return '{0:b}'.format(n)

# Finds the 2-fatness of x, i.e., the largest k such that 2^k divides x (float('inf') if x is zero)
def fatness(x):
    if x == 0:
        return float('inf')
    k = 0
    while x % 2 == 0:
        k += 1
        x //= 2
    return k

# Finds the 2-fattest number in the interval [x..y]
def fattest(x, y):
    ftst = 0
    maxf = -1
    for k in range(x, y + 1):
        f = fatness(k)
        if f > maxf:
            maxf = f
            ftst = k
    return ftst

# Finds the 2-fat jump of x, i.e., the smallest value larger than x with larger fatness than x
def _fat_jump(x):
    f = fatness(x)
    y = x + 1
    while not fatness(y) > f:
        y += 1
    return y

# Finds the n-th iterated 2-fat jumpt of x, i.e., applies _fatJump n times to x. If x = 0, it returns None
def fat_jump(x, n = 1):
    if x == 0:
        return None
    for k in range(n):
        x = _fat_jump(x)
    return x

# Finds the 2-fat backjump of x, i.e., the largest value smaller than x with larger fatness than x. If x = 0, it returns None
def _fat_backjump(x):
    f = fatness(x)
    y = x - 1
    while not fatness(y) > f:
        y -= 1
    return y

# Finds the n-th iterated 2-fat bacljumpt of x, i.e., applies _fatBackJump n times to x. If x = 0, it returns None
def fat_backjump(x, n = 1):
    if x == 0:
        return 0
    for k in range(n):
        if x == 0:
            return 0
        x = _fat_backjump(x)
    return x

# Returns the set of all backjumps of x
def all_fat_backjumps(x):
    s = set()
    while x>0:
        s.add(x)
        x=fat_backjump(x)
    s.add(x)
    return s



if len(sys.argv) != 2:
    print("Usage: {} <number of intervals>".format(sys.argv[0]))
    sys.exit(1)

n=int(sys.argv[1])


ski = []
skj = []
ski.append(0)
for i in range(n):
    size = random.randint(0,10)
    if i == n - 1:
        size = 10
    skj.append(ski[-1] + size)
    if i < n - 1:
        ski.append(skj[-1] + 1)


mx = 0
h = []
jp = []
for i in range(n):
    h.append(fattest(ski[i], skj[i]))
    if h[-1]>0:
        jp.append(fat_jump(h[-1]))        
        mx = max(mx, jp[-1])
scale = 1
while mx / scale > 10:
    scale *= 2



def outTikz():
    print("\t\t\t\t% Main line")
    print("\t\t\t\t\\draw (0, 0) -- ({}, 0);".format(math.ceil(mx/scale)))
    print("\t\t\t\t% Ticks (skip intervals extreme)")
    print("\t\t\t\t\\foreach \\x in {{{},{}}}".format(",".join([str(x/scale) for x in ski]),skj[-1]/scale))
    print("\t\t\t\t\t\\draw (\\x, -0.03) -- (\\x, 0.03) node[below]{};")
    print("\t\t\t\t% Skip interval of beta")
    print("\t\t\t\t\\draw [thick, color=blue] ({}, 0) -- ({}, 0);".format(ski[-1]/scale, skj[-1]/scale))
    print("\t\t\t\t\\draw [thick, color=blue] ({}, -0.03) -- ({}, 0.03) node[below]{{}};".format(ski[-1]/scale,ski[-1]/scale))
    print("\t\t\t\t\\draw [thick, color=blue] ({}, -0.03) -- ({}, 0.03) node[below]{{}};".format(skj[-1]/scale,skj[-1]/scale))
    print("\t\t\t\t% Handles")
    print("\t\t\t\t\\foreach \\x in {{{}}}".format(",".join([str(x/scale) for x in h])))
    print("\t\t\t\t\t\\filldraw (\\x, 0) circle (0.01);")
    print("\t\t\t\t\\filldraw [color=blue] ({}, 0) circle (0.01);".format(h[-1]/scale));
    print("\t\t\t\t% Backjumps")
    start = h[-2]
    while start > 0:
        print("\t\t\t\t\\draw[->]({}, 0) arc (0:-180:{});".format(start/scale, (start-fat_backjump(start))/(2*scale)));
        start = fat_backjump(start)
    print("\t\t\t\t% Jumps")
    for i in range(1, n - 1):
        if fat_jump(h[i]) > ski[-1]:
            print("\t\t\t\t\\draw[->][color=blue]({}, 0) arc (180:0:{});".format(h[i]/scale, (fat_jump(h[i])-h[i])/(2*scale)));
        else:
            print("\t\t\t\t\\draw[->]({}, 0) arc (180:0:{});".format(h[i]/scale, (fat_jump(h[i])-h[i])/(2*scale)));


print("\\documentclass[border=3pt]{standalone}\n\t\\usepackage{tikz}\n\t\\usetikzlibrary{arrows, chains, fit, quotes}\n\n")
print("\t\\begin{document}\n\n\t\t\\begin{tikzpicture}\n\n")
outTikz()
print("\n\n\t\t\\end{tikzpicture}\n\n\\end{document}")





