#!/usr/bin/env python3
"""Executable check of the algorithms of the paper "Z-Fast Tries".

This is an explicit, unoptimised model of the (unsigned) z-fast trie that
follows the pseudocode of the paper line by line, in the paper's notation:

    n_alpha, e_alpha   name and extent of a node (binary strings)
    [i..j]             its skip interval [|n_alpha|..|e_alpha|] (closed)
    h_alpha            its handle: the prefix of the extent whose length is
                       the 2-fattest number in the skip interval
    Z                  the z-map, from handles to internal nodes
    J-, J+             the jump pointers of an internal node

It implements LocateParex, LocateExitOrParex, LocateExit, ParentStack,
insertion and deletion with the update of the z-map, of the jump pointers,
of the list of leaves and of the reference leaves, predecessor/successor,
existential and enumerative range queries.

After EVERY update the whole structure is compared with its definition,
recomputed naively from the set of keys (`ZFastTrie.check`); queries are
compared with brute force. The script then reintroduces, one at a time,
errors that were present in earlier versions of the pseudocode and verifies
that each of them is detected.

Usage:  python3 check_updates.py [--quick]

Only the standard library is needed. Binary strings are Python strings over
'0' and '1'; their lexicographic order is the order of Python strings.
"""

import random
import sys
from math import inf

# Name of the error currently reintroduced (None: the algorithms of the paper).
MUTATION = None


# ----------------------------------------------------------------------
# 2-fattest numbers, jumps and backjumps (Section "Notation and tools")
# ----------------------------------------------------------------------

def two_fattest(a, b):
    """The 2-fattest number in the closed interval [a..b]."""
    assert 0 <= a <= b
    if a == 0:
        return 0
    return b & (-1 << (((a - 1) ^ b).bit_length() - 1))


def jump(x):
    """The 2-fat jump x' (infinite for x = 0)."""
    return inf if x == 0 else x + (x & -x)


def backjump(x):
    """The 2-fat backjump of x (0 for x = 0)."""
    return 0 if x == 0 else x - (x & -x)


def backjumps(x):
    """The set { x_(n) | n >= 0 }."""
    result = {x}
    while x > 0:
        x = backjump(x)
        result.add(x)
    return result


def lcp_length(x, y):
    """The length of the longest common prefix of x and y."""
    n = 0
    while n < len(x) and n < len(y) and x[n] == y[n]:
        n += 1
    return n


def is_proper_prefix(p, x):
    return len(p) < len(x) and x.startswith(p)


# ----------------------------------------------------------------------
# Nodes
# ----------------------------------------------------------------------

class Node:
    """A node of the compacted trie. Leaves have no children."""

    def __init__(self, name, extent):
        self.name = name          # n_alpha
        self.extent = extent      # e_alpha
        self.left = None          # 0-child
        self.right = None         # 1-child
        self.jminus = None        # J^-_alpha (internal nodes)
        self.jplus = None         # J^+_alpha (internal nodes)
        self.prev = None          # previous leaf (leaves)
        self.next = None          # next leaf (leaves)
        self.ref = None           # reference leaf (internal nodes), or the
                                  # internal node referring to this leaf

    @property
    def is_leaf(self):
        return self.left is None

    @property
    def i(self):
        return len(self.name)

    @property
    def j(self):
        return len(self.extent)

    @property
    def handle_length(self):
        return two_fattest(self.i, self.j)

    @property
    def handle(self):
        return self.extent[:self.handle_length]

    @property
    def jump_length(self):
        """|h_alpha|'."""
        return jump(self.handle_length)

    def intercepts(self, k):
        """An internal node intercepts k if k is in its skip interval; a
        leaf intercepts k if k >= i."""
        return self.i <= k and (self.is_leaf or k <= self.j)

    def child(self, x):
        """The child on the path of x; x must properly extend the extent."""
        return self.left if x[self.j] == '0' else self.right

    def __repr__(self):
        return '<%s|%s>' % (self.name, self.extent[self.i:])


def J(node, side):
    """The jump pointer of `node` on a side ('-' is left, '+' is right)."""
    return node.jminus if side == '-' else node.jplus


def set_J(node, side, target):
    if side == '-':
        node.jminus = target
    else:
        node.jplus = target


def child_on(node, side):
    return node.left if side == '-' else node.right


def extreme_leaf(node, side):
    """The leftmost ('-') or rightmost ('+') leaf under a node, reached by
    following jump pointers; also returns the number of jumps."""
    jumps = 0
    while not node.is_leaf:
        node = J(node, side)
        jumps += 1
    return node, jumps


def exits_on_the_left(x, exit_node):
    """Whether x exits on the left at its exit node."""
    e = exit_node.extent
    c = lcp_length(x, e)
    return c == len(x) or (c < len(e) and x[c] == '0')


# ----------------------------------------------------------------------
# The z-fast trie
# ----------------------------------------------------------------------

class ZFastTrie:

    def __init__(self):
        self.root = None
        self.Z = {}               # handle -> internal node
        self.iterations = 0       # iterations of the last fat binary search

    # ------------------------------------------------------------------
    # Fat binary search
    # ------------------------------------------------------------------

    def locate_parex(self, x):
        """Algorithm LocateParex: the stack of the 2-fat ancestors of
        parex(x), with parex(x) on top (empty if x exits at the root)."""
        stack = []
        a, b = 0, len(x)
        self.iterations = 0
        while a <= b:
            self.iterations += 1
            f = two_fattest(a, b)
            beta = self.Z.get(x[:f])
            if beta is not None and is_proper_prefix(beta.extent, x):
                a = beta.j + 1
                stack.append(beta)
            else:
                b = f - 1
        return stack

    def locate_exit_or_parex(self, x):
        """Algorithm LocateExitOrParex: no string comparison."""
        stack = []
        a, b = 0, len(x)
        self.iterations = 0
        while a <= b:
            self.iterations += 1
            f = two_fattest(a, b)
            beta = self.Z.get(x[:f])
            if beta is not None:
                a = beta.j + 1
                stack.append(beta)
            else:
                b = f - 1
        return stack

    def locate_exit(self, x):
        """Algorithm LocateExit."""
        stack = self.locate_exit_or_parex(x)
        if not stack:
            return self.root      # the trie is a single leaf
        sigma = stack[-1]
        if is_proper_prefix(sigma.extent, x):
            return sigma.child(x)
        return sigma

    def parent_stack(self, stack):
        """Algorithm ParentStack: turns the stack of the 2-fat ancestors of
        a node sigma (on top) into that of the parent of sigma."""
        sigma = stack.pop()
        if not stack:
            return stack          # sigma is the root
        a = stack[-1].j + 1
        if MUTATION == 'parent-stack-uses-extent':
            b = sigma.j - 1       # as printed before the correction
        else:
            b = sigma.i - 1
        while a <= b:
            f = two_fattest(a, b)
            beta = self.Z[sigma.extent[:f]]      # never undefined
            stack.append(beta)
            a = beta.j + 1
        return stack

    # ------------------------------------------------------------------
    # Jump pointers
    # ------------------------------------------------------------------

    def set_jump(self, alpha, side):
        """Sets a jump pointer of an internal node by descending from its
        child on that side along the jump pointers of its descendants.
        Returns the number of jumps."""
        k = alpha.jump_length
        delta = child_on(alpha, side)
        jumps = 0
        while not delta.intercepts(k):
            delta = J(delta, side)
            jumps += 1
        set_J(alpha, side, delta)
        return jumps

    def fix_jumps_after_insertion(self, stack, exit_node, nu, mu, side,
                                  mu_on_the_same_side):
        """Algorithm "jump pointers after an insertion".

        stack       the 2-fat ancestors of parex(x) (parex on top)
        exit_node   the node that was exit(x) before the insertion
        nu, mu      the new internal node and the new leaf
        side        '-' if the exit node was a left child (then the left
                    jump pointers are fixed), '+' if it was a right child
        mu_on_the_same_side
                    whether x exits on that same side
        """
        while (stack and J(stack[-1], side) is exit_node
               and stack[-1].jump_length < mu.i):
            alpha = stack.pop()
            set_J(alpha, side, nu)
        if mu_on_the_same_side:
            # The scan moves towards the leaf next to x (x^+ if side is
            # '-', x^- otherwise), so it follows O(lg) jump pointers.
            neighbour, max_jumps = extreme_leaf(exit_node, side)
            assert max_jumps <= neighbour.j.bit_length()
            delta = exit_node
            while stack:
                alpha = stack.pop()
                if MUTATION == 'insertion-guard-on-alpha':
                    # as printed before the correction ("alpha is internal")
                    while not alpha.is_leaf and J(alpha, side) is not delta:
                        delta = J(delta, side)
                else:
                    while not delta.is_leaf and J(alpha, side) is not delta:
                        delta = J(delta, side)
                if J(alpha, side) is not delta:
                    return
                set_J(alpha, side, mu)

    def fix_jumps_after_deletion(self, stack, pi, mu, xi, side,
                                 mu_on_the_same_side):
        """Algorithm "jump pointers after a deletion".

        stack       the 2-fat ancestors of the parent of pi (on top)
        pi, mu, xi  the removed internal node, the removed leaf and its
                    sibling, which has taken the place of pi
        side        '-' if pi was a left child, '+' if it was a right child
        mu_on_the_same_side
                    whether mu was the child of pi on that same side
        """
        while stack and J(stack[-1], side) is pi:
            alpha = stack.pop()
            set_J(alpha, side, xi)
        if MUTATION == 'deletion-as-printed':
            # The second loop as printed before the correction: it runs when
            # the leaf was on the OTHER side, on the other jump pointers,
            # and uses |e_alpha|' instead of |h_alpha|'.
            other = '+' if side == '-' else '-'
            if not mu_on_the_same_side:
                delta = xi
                while stack and J(stack[-1], other) is mu:
                    alpha = stack.pop()
                    while not delta.intercepts(jump(alpha.j)):
                        delta = J(delta, other)
                    set_J(alpha, other, delta)
            return
        if mu_on_the_same_side:
            # Again, the scan moves towards the leaf that was next to x.
            neighbour, max_jumps = extreme_leaf(xi, side)
            assert max_jumps <= neighbour.j.bit_length()
            delta = xi
            while stack and J(stack[-1], side) is mu:
                alpha = stack.pop()
                while not delta.intercepts(alpha.jump_length):
                    delta = J(delta, side)
                set_J(alpha, side, delta)

    # ------------------------------------------------------------------
    # Updates
    # ------------------------------------------------------------------

    def insert(self, x):
        """Inserts x; returns False if x is in the set or the set would not
        be prefix-free any longer."""
        if self.root is None:
            self.root = Node('', x)
            return True

        stack = self.locate_parex(x)
        gamma = stack[-1] if stack else None            # parex(x)
        exit_node = self.root if gamma is None else gamma.child(x)
        c = lcp_length(x, exit_node.extent)             # the cutpoint
        if c == len(x) or (exit_node.is_leaf and c == exit_node.j):
            return False
        cuts_low = c >= exit_node.handle_length
        x_exits_on_the_right = x[c] == '1'
        was_left_child = gamma is not None and gamma.left is exit_node
        old_handle = exit_node.handle

        # The new internal node and the new leaf.
        nu = Node(exit_node.name, x[:c])
        mu = Node(x[:c + 1], x)
        nu.ref, mu.ref = mu, nu
        if x_exits_on_the_right:
            nu.left, nu.right = exit_node, mu
        else:
            nu.left, nu.right = mu, exit_node

        # Jump pointers of the new internal node: one points to the new
        # leaf; the other one is the old jump pointer of the exit node if
        # the new node inherits its handle, the exit node itself otherwise.
        towards_exit = '-' if x_exits_on_the_right else '+'
        towards_leaf = '+' if x_exits_on_the_right else '-'
        set_J(nu, towards_leaf, mu)
        if cuts_low and not exit_node.is_leaf:
            set_J(nu, towards_exit, J(exit_node, towards_exit))
        else:
            set_J(nu, towards_exit, exit_node)

        # Trie structure.
        if gamma is None:
            self.root = nu
        elif was_left_child:
            gamma.left = nu
        else:
            gamma.right = nu

        # Jump pointers of the ancestors.
        if gamma is not None:
            side = '-' if was_left_child else '+'
            self.fix_jumps_after_insertion(
                stack, exit_node, nu, mu, side,
                x_exits_on_the_right == (side == '+'))

        # Names, z-map, and jump pointers of the old exit node.
        exit_node.name = exit_node.extent[:c + 1]
        if cuts_low and not exit_node.is_leaf:
            assert nu.handle == old_handle
            self.Z[old_handle] = nu
            assert exit_node.handle not in self.Z
            self.Z[exit_node.handle] = exit_node
            if MUTATION != 'no-recomputation-on-low-cut':
                for side in '-+':
                    jumps = self.set_jump(exit_node, side)
                    assert jumps <= exit_node.j.bit_length() + 1
            # The new handle of the exit node is not a prefix of x, but
            # unless the exit node was the root it is shorter than 2|x|.
            assert gamma is None or exit_node.handle_length < 2 * len(x)
        else:
            assert nu.handle not in self.Z
            self.Z[nu.handle] = nu

        # List of leaves: x exits on the left iff it is smaller than all
        # the keys under the old exit node.
        if x_exits_on_the_right:
            before, _ = extreme_leaf(exit_node, '+')
            after = before.next
        else:
            after, _ = extreme_leaf(exit_node, '-')
            before = after.prev
        mu.prev, mu.next = before, after
        if before is not None:
            before.next = mu
        if after is not None:
            after.prev = mu
        return True

    def delete(self, x):
        """Deletes x; returns False if x is not in the set."""
        if self.root is None:
            return False
        stack = self.locate_parex(x)
        if not stack:                       # x exits at the root
            if self.root.is_leaf and self.root.extent == x:
                self.root = None
                return True
            return False
        pi = stack[-1]                      # parex(x)
        mu = pi.child(x)                    # exit(x)
        if not (mu.is_leaf and mu.extent == x):
            return False
        mu_was_left_child = pi.left is mu
        xi = pi.right if mu_was_left_child else pi.left      # the sibling

        # Trie structure; the stack becomes that of the parent of pi.
        if pi is self.root:
            self.root = xi
            stack = []
            side = None
        else:
            stack = self.parent_stack(stack)
            grandparent = stack[-1]
            assert pi in (grandparent.left, grandparent.right)
            if grandparent.left is pi:
                grandparent.left = xi
                side = '-'
            else:
                grandparent.right = xi
                side = '+'

        # Whether the sibling takes over the handle of pi: this happens iff
        # the 2-fattest number of the merged skip interval is |h_pi|.
        takes_over = two_fattest(pi.i, xi.j) == pi.handle_length

        # Reference leaves: the reference leaf of pi becomes free; if the
        # deleted leaf was referred to by another node, that node takes it.
        rho = mu.ref
        if rho is None:
            pi.ref.ref = None
        elif rho is not pi:
            rho.ref = pi.ref
            rho.ref.ref = rho

        # List of leaves.
        if mu.prev is not None:
            mu.prev.next = mu.next
        if mu.next is not None:
            mu.next.prev = mu.prev

        # Names, z-map, and jump pointers of the sibling.
        if takes_over and not xi.is_leaf:
            # The old handle of the sibling is not a prefix of x, but unless
            # pi was the root it is shorter than 2|x|.
            assert pi.i == 0 or xi.handle_length < 2 * len(x)
            del self.Z[xi.handle]
            handle = pi.handle
            xi.name = pi.name
            assert xi.handle == handle
            self.Z[handle] = xi
            # The sibling inherits the jump length of pi: on the side
            # opposite to the deleted leaf it also inherits the jump
            # pointer; on the side of the deleted leaf the pointer is
            # recomputed, moving towards the leaf that was next to x.
            side_of_mu = '-' if mu_was_left_child else '+'
            opposite = '+' if mu_was_left_child else '-'
            if MUTATION != 'no-recomputation-of-sibling':
                set_J(xi, opposite, J(pi, opposite))
                jumps = self.set_jump(xi, side_of_mu)
                neighbour, _ = extreme_leaf(xi, side_of_mu)
                assert jumps <= neighbour.j.bit_length()
        else:
            del self.Z[pi.handle]
            xi.name = pi.name

        # Jump pointers of the ancestors.
        if side is not None:
            self.fix_jumps_after_deletion(
                stack, pi, mu, xi, side,
                mu_was_left_child == (side == '-'))
        return True

    # ------------------------------------------------------------------
    # Queries
    # ------------------------------------------------------------------

    def contains(self, x):
        if self.root is None:
            return False
        exit_node = self.locate_exit(x)
        return exit_node.is_leaf and exit_node.extent == x

    def pred_succ(self, x):
        """Algorithm Predecessor/Successor: (x^-, x^+), None if missing."""
        eta = self.locate_exit(x)
        if exits_on_the_left(x, eta):
            succ, _ = extreme_leaf(eta, '-')
            pred = succ.prev
        else:
            pred, _ = extreme_leaf(eta, '+')
            succ = pred.next
        return (None if pred is None else pred.extent,
                None if succ is None else succ.extent)

    def is_nonempty(self, x, y):
        """Algorithm IsNonempty: whether S intersects [x..y), for x < y."""
        alpha = self.locate_exit(x)
        if exits_on_the_left(x, alpha):
            while not alpha.is_leaf and alpha.j < len(y):
                alpha = alpha.jminus
            return alpha.extent < y
        beta = self.locate_exit(y)
        if not exits_on_the_left(y, beta):
            while not beta.is_leaf and beta.j < len(x):
                beta = beta.jplus
            return x <= beta.extent
        eta = self.locate_exit(x[:lcp_length(x, y)])
        alpha = eta.left
        while not alpha.is_leaf and alpha.j < len(x):
            alpha = alpha.jplus
        if x <= alpha.extent:
            return True
        beta = eta.right
        while not beta.is_leaf and beta.j < len(y):
            beta = beta.jminus
        return beta.extent < y

    def range(self, x, y):
        """Enumerative range query: the keys in [x..y), for x < y, without
        comparing the enumerated keys with y."""
        # The first leaf of the output.
        alpha = self.locate_exit(x)
        steps_x = 0
        if exits_on_the_left(x, alpha):
            while not alpha.is_leaf and alpha.j < len(y):
                alpha = alpha.jminus
            if not alpha.extent < y:
                return []
            # All the keys under alpha are in the output.
            first, steps_x = extreme_leaf(alpha, '-')
        else:
            pred, _ = extreme_leaf(alpha, '+')          # x^-
            first = pred.next
        # The first leaf after the output.
        beta = self.locate_exit(y)
        steps_y = 0
        if exits_on_the_left(y, beta):
            stop, _ = extreme_leaf(beta, '-')           # y^+
        else:
            while not beta.is_leaf and beta.j < len(x):
                beta = beta.jplus
            if not x <= beta.extent:
                return []
            # All the keys under beta are in the output.
            last, steps_y = extreme_leaf(beta, '+')
            stop = last.next
        result = []
        leaf = first
        while leaf is not stop:
            result.append(leaf.extent)
            leaf = leaf.next
        assert steps_x <= len(result) and steps_y <= len(result)
        return result

    # ------------------------------------------------------------------
    # Definitions, computed naively (used only for checking)
    # ------------------------------------------------------------------

    def nodes(self, node=None):
        node = node or self.root
        if node is None:
            return
        yield node
        if not node.is_leaf:
            yield from self.nodes(node.left)
            yield from self.nodes(node.right)

    def ancestors(self, node):
        """The path from the root to a node, the node included."""
        path = [self.root]
        while path[-1] is not node:
            path.append(path[-1].child(node.name))
        return path

    def two_fat_ancestors(self, node):
        """The 2-fat ancestors of a node, by increasing handle length."""
        lengths = backjumps(node.handle_length)
        return [a for a in self.ancestors(node) if a.handle_length in lengths]

    def naive_exit(self, x):
        """exit(x): the node alpha such that n_alpha is a prefix of x and
        either alpha is a leaf or e_alpha is not a proper prefix of x."""
        node = self.root
        while not node.is_leaf and is_proper_prefix(node.extent, x):
            node = node.child(x)
        return node

    def naive_parex(self, x):
        path = self.ancestors(self.naive_exit(x))
        return path[-2] if len(path) > 1 else None

    def check(self, keys):
        """Compares the whole structure with its definition."""
        keys = sorted(keys)
        if not keys:
            assert self.root is None and not self.Z
            return

        # The compacted trie.
        def expect(node, name, lo, hi):
            assert node.name == name, (node, name)
            extent = keys[lo][:lcp_length(keys[lo], keys[hi - 1])]
            assert node.extent == extent, (node, extent)
            if hi - lo == 1:
                assert node.is_leaf
                return
            assert not node.is_leaf
            mid = lo
            while keys[mid][len(extent)] == '0':
                mid += 1
            expect(node.left, extent + '0', lo, mid)
            expect(node.right, extent + '1', mid, hi)
        expect(self.root, '', 0, len(keys))

        internal = [n for n in self.nodes() if not n.is_leaf]
        leaves = [n for n in self.nodes() if n.is_leaf]

        # The z-map.
        assert len(self.Z) == len(internal)
        for node in internal:
            assert self.Z.get(node.handle) is node, node

        # Jump pointers.
        for node in internal:
            k = node.jump_length
            assert k > node.j
            for side in '-+':
                target = child_on(node, side)
                while not target.intercepts(k):
                    target = child_on(target, side)
                assert J(node, side) is target, (node, side, target)

        # The list of leaves (the trie visit lists leaves in order).
        assert [n.extent for n in leaves] == keys
        for k, leaf in enumerate(leaves):
            assert leaf.prev is (leaves[k - 1] if k > 0 else None)
            assert leaf.next is (leaves[k + 1] if k + 1 < len(leaves)
                                 else None)

        # Reference leaves: an injection from internal nodes to their
        # descendant leaves, with back pointers.
        for node in internal:
            assert node.ref.is_leaf and node.ref.ref is node
            assert node.ref.extent.startswith(node.extent)
        assert sum(leaf.ref is not None for leaf in leaves) == len(internal)

    def check_searches(self, queries):
        """Checks the search algorithms and the bounds stated in the paper
        on the current trie."""
        for x in queries:
            exit_node = self.naive_exit(x)
            parex = self.naive_parex(x)
            bound = (len(x).bit_length() - 1) + 2 if x else 1

            stack = self.locate_parex(x)
            assert self.iterations <= bound
            if parex is None:
                assert stack == []
            else:
                assert stack == self.two_fat_ancestors(parex)

            stack = self.locate_exit_or_parex(x)
            assert self.iterations <= bound
            if exit_node.is_leaf and parex is None:
                assert stack == []
            else:
                cuts_low = (lcp_length(x, exit_node.extent)
                            >= exit_node.handle_length)
                sigma = (exit_node if cuts_low and not exit_node.is_leaf
                         else parex)
                assert stack == self.two_fat_ancestors(sigma)
            assert self.locate_exit(x) is exit_node

        # ParentStack, and the number of jumps to the extreme leaves.
        for node in self.nodes():
            if node.is_leaf:
                continue
            stack = self.parent_stack(self.two_fat_ancestors(node))
            if node is self.root:
                assert stack == []
            else:
                parent = self.ancestors(node)[-2]
                assert stack == self.two_fat_ancestors(parent)
            for side in '-+':
                leaf, jumps = extreme_leaf(node, side)
                assert jumps <= (leaf.j.bit_length() - 1) + 1

    def check_queries(self, keys, queries):
        keys = sorted(keys)
        for x in queries:
            assert self.contains(x) == (x in keys)
            smaller = [k for k in keys if k < x]
            larger = [k for k in keys if k >= x]
            assert self.pred_succ(x) == (smaller[-1] if smaller else None,
                                         larger[0] if larger else None)
        for x in queries:
            for y in queries:
                if x < y:
                    expected = [k for k in keys if x <= k < y]
                    assert self.is_nonempty(x, y) == bool(expected)
                    assert self.range(x, y) == expected


# ----------------------------------------------------------------------
# Random workloads
# ----------------------------------------------------------------------

def random_string(rng, length):
    return ''.join(rng.choice('01') for _ in range(length))


def uniform(rng, max_length, keys):
    """Random strings of the same length."""
    return random_string(rng, max_length)


def variable(rng, max_length, keys):
    """Random strings of random length (most are rejected because the set
    must remain prefix-free)."""
    return random_string(rng, rng.randint(1, max_length))


def clustered(rng, max_length, keys):
    """A key of the set with a bit flipped, and possibly a new tail."""
    if not keys or rng.random() < 0.1:
        return random_string(rng, rng.randint(1, max_length))
    x = rng.choice(sorted(keys))
    p = rng.randrange(len(x))
    x = x[:p] + ('1' if x[p] == '0' else '0')
    if rng.random() < 0.5:
        tail_length = rng.randint(0, max(0, max_length - len(x)))
        return x + random_string(rng, tail_length)
    return x + rng.choice(sorted(keys))[p + 1:]


def caterpillar(rng, max_length, keys):
    """b^p (not b) b^r: the trie is a caterpillar with long extents."""
    b = rng.choice('01')
    nb = '1' if b == '0' else '0'
    p = rng.randrange(max_length)
    return b * p + nb + b * rng.choice([0, max_length - p - 1])


GENERATORS = [uniform, variable, clustered, caterpillar]


def query_strings(rng, max_length, keys):
    """Keys, prefixes and extensions of keys, and random strings."""
    queries = set()
    for x in rng.sample(sorted(keys), min(len(keys), 6)):
        queries.add(x)
        queries.add(x[:rng.randint(1, len(x))])
        queries.add(x + random_string(rng, rng.randint(1, 3)))
        p = rng.randrange(len(x))
        queries.add(x[:p] + ('1' if x[p] == '0' else '0') + x[p + 1:])
    for _ in range(4):
        queries.add(random_string(rng, rng.randint(1, max_length + 2)))
    return sorted(queries)


def run(seed, generator, max_length, max_size, updates):
    """One random sequence of insertions and deletions, checked after every
    update. Returns the number of successful updates."""
    rng = random.Random(seed)
    trie = ZFastTrie()
    keys = set()
    done = 0
    for step in range(updates):
        # The target size oscillates, so the trie is emptied and refilled.
        if not keys or (len(keys) < max_size and rng.random() < 0.55):
            x = generator(rng, max_length, keys)
            prefix_free = all(not k.startswith(x) and not x.startswith(k)
                              for k in keys)
            assert trie.insert(x) == prefix_free, x
            if prefix_free:
                keys.add(x)
                done += 1
        else:
            if rng.random() < 0.1:
                x = generator(rng, max_length, keys)
            else:
                x = rng.choice(sorted(keys))
            assert trie.delete(x) == (x in keys), x
            if x in keys:
                keys.remove(x)
                done += 1
        trie.check(keys)
        if keys and step % 8 == 0:
            queries = query_strings(rng, max_length, keys)
            trie.check_searches(queries)
            trie.check_queries(keys, queries)
    # Empty the trie.
    for x in rng.sample(sorted(keys), len(keys)):
        assert trie.delete(x)
        keys.remove(x)
        trie.check(keys)
        done += 1
    return done


def run_all(runs_per_configuration):
    tries = updates = 0
    for generator in GENERATORS:
        for max_length in (3, 5, 8, 16, 33, 64, 200):
            for max_size in (4, 16, 48):
                for k in range(runs_per_configuration):
                    seed = '%s-%d-%d-%d' % (generator.__name__, max_length,
                                            max_size, k)
                    updates += run(seed, generator, max_length, max_size,
                                   3 * max_size + 20)
                    tries += 1
    return tries, updates


MUTATIONS = {
    'parent-stack-uses-extent':
        'ParentStack with b = |e_sigma| - 1 instead of |n_sigma| - 1',
    'no-recomputation-on-low-cut':
        'insertion without recomputing the jump pointers of an internal '
        'exit node when x cuts low',
    'insertion-guard-on-alpha':
        'insertion with the inner loop guarded by "alpha is internal"',
    'deletion-as-printed':
        'deletion with the second loop on the wrong side and pointers',
    'no-recomputation-of-sibling':
        'deletion without updating the jump pointers of a sibling that '
        'takes over the handle of its parent',
}


def main():
    global MUTATION
    quick = '--quick' in sys.argv[1:]
    runs = 2 if quick else 12

    tries, updates = run_all(runs)
    print('paper algorithms: %d random tries, %d updates, every update and '
          'query checked: OK' % (tries, updates))

    failed = False
    for name, description in MUTATIONS.items():
        MUTATION = name
        try:
            run_all(1 if quick else 2)
        except (AssertionError, KeyError, AttributeError) as error:
            print('error reintroduced (%s): detected (%s)'
                  % (description, type(error).__name__))
        else:
            print('error reintroduced (%s): NOT DETECTED' % description)
            failed = True
        MUTATION = None
    sys.exit(1 if failed else 0)


if __name__ == '__main__':
    main()
