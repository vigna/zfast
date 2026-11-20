/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! Building blocks shared by the dynamic and the static structure: 2-fattest
//! numbers, longest common prefixes, and prefix hashing.

mod hash;
pub use hash::{Chain, ChainHasher, HashPrefixes, PrefixHasher, Xxh3, Xxh3Hasher};

/// Returns the 2-fattest number in the interval (`a`..`b`], that is, the
/// number in the interval with the largest number of trailing zeros.
///
/// The 2-fattest number of an interval is unique. Note that the interval is
/// open on the left: this is the convention used throughout the code, whereas
/// the paper uses closed intervals.
///
/// # Examples
///
/// ```
/// # use zfast::utils::two_fattest;
/// assert_eq!(two_fattest(0, 1), 1);
/// assert_eq!(two_fattest(4, 7), 6);
/// assert_eq!(two_fattest(3, 4), 4);
/// assert_eq!(two_fattest(8, 100), 64);
/// ```
#[inline(always)]
pub const fn two_fattest(a: usize, b: usize) -> usize {
    debug_assert!(a < b);
    b & (usize::MAX << (usize::BITS - 1 - (a ^ b).leading_zeros()))
}

/// Returns the 2-fat jump of `h`, that is, the smallest number larger than `h`
/// that is 2-fatter than `h` (i.e., that has more trailing zeros).
///
/// The 2-fat jump of zero is infinite, and it is represented by
/// [`usize::MAX`].
///
/// # Examples
///
/// ```
/// # use zfast::utils::jump;
/// assert_eq!(jump(1), 2);
/// assert_eq!(jump(6), 8);
/// assert_eq!(jump(8), 16);
/// assert_eq!(jump(0), usize::MAX);
/// ```
#[inline(always)]
pub const fn jump(h: usize) -> usize {
    if h == 0 {
        usize::MAX
    } else {
        h + (h & h.wrapping_neg())
    }
}

/// Prefetches the cache line containing the given address.
#[inline(always)]
pub(crate) fn prefetch<T>(ptr: *const T) {
    #[cfg(all(target_arch = "x86_64", not(miri)))]
    // SAFETY: prefetching has no architectural effect, even on invalid
    // addresses
    unsafe {
        use core::arch::x86_64::{_MM_HINT_T0, _mm_prefetch};
        _mm_prefetch::<_MM_HINT_T0>(ptr as *const i8);
    }
    #[cfg(all(target_arch = "aarch64", not(miri)))]
    // SAFETY: as above
    unsafe {
        core::arch::asm!(
            "prfm pldl1keep, [{ptr}]",
            ptr = in(reg) ptr,
            options(nostack, preserves_flags, readonly)
        );
    }
    // Miri does not support inline assembly
    #[cfg(any(miri, not(any(target_arch = "x86_64", target_arch = "aarch64"))))]
    let _ = ptr;
}

pub(crate) const fn parse_usize(s: &str) -> usize {
    let s = s.as_bytes();
    let mut n = 0;
    let mut i = 0;
    while i < s.len() {
        assert!(s[i].is_ascii_digit());
        n = n * 10 + (s[i] - b'0') as usize;
        i += 1;
    }
    n
}

/// The number of probes that are hashed and prefetched ahead of the current
/// one by [`fat_binary_search`].
pub(crate) const AHEAD: usize = match option_env!("ZFAST_AHEAD") {
    Some(s) => parse_usize(s),
    None => 2,
};

/// Performs a fat binary search on the interval (`a`..`b`].
///
/// At each step, the hash `h` of the prefix whose length `f` is the
/// 2-fattest number in the current interval is passed to `probe`, together
/// with `f`: if `probe` returns the length of an extent, which must be at
/// least `f`, the left extreme of the interval is set to that length;
/// otherwise, the right extreme is set to `f` − 1.
///
/// The search is inherently sequential, as each step depends on the result of
/// the previous one, and in a large structure each probe is a cache miss.
/// However, the sequence of probes that will be performed if the next probes
/// are all unsuccessful is known in advance, and unsuccessful probes are
/// common: thus, we hash the next [`AHEAD`] probes of that sequence, and pass
/// each hash to `prefetch`, before performing a probe. The speculation is
/// discarded after a successful probe.
#[inline(always)]
pub(crate) fn fat_binary_search(
    hasher: &impl PrefixHasher,
    mut a: usize,
    mut b: usize,
    prefetch: impl Fn(u64),
    mut probe: impl FnMut(usize, u64) -> Option<usize>,
) {
    // The probes that will be performed, in order, if they are all
    // unsuccessful
    let mut pending = [(0, 0); AHEAD + 1];
    let mut len = 0;
    // The right extreme of the interval after the pending probes
    let mut spec_b = b;
    loop {
        while len <= AHEAD && a < spec_b {
            let f = two_fattest(a, spec_b);
            let h = hasher.hash(f);
            prefetch(h);
            pending[len] = (f, h);
            len += 1;
            spec_b = f - 1;
        }
        if len == 0 {
            break;
        }
        let (f, h) = pending[0];
        // A shift of constant length, which the compiler turns into a few
        // moves (copy_within would call memmove)
        for i in 0..AHEAD {
            pending[i] = pending[i + 1];
        }
        len -= 1;
        match probe(f, h) {
            Some(extent) => {
                debug_assert!(extent >= f);
                a = extent;
                len = 0;
                spec_b = b;
            }
            None => b = f - 1,
        }
    }
}

#[inline(always)]
fn read_u64(s: &[u8], i: usize) -> u64 {
    u64::from_le_bytes(s[i..i + 8].try_into().unwrap())
}

/// Returns the length of the longest common prefix of `a` and `b`, given that
/// they are known to coincide on their first `from` bytes.
#[inline]
pub fn lcp_from(a: &[u8], b: &[u8], from: usize) -> usize {
    let n = a.len().min(b.len());
    debug_assert!(from <= n);
    debug_assert!(a[..from] == b[..from]);
    let (a, b) = (&a[..n], &b[..n]);
    let mut i = from;
    while i + 8 <= n {
        let x = read_u64(a, i) ^ read_u64(b, i);
        if x != 0 {
            return i + (x.trailing_zeros() / 8) as usize;
        }
        i += 8;
    }
    while i < n && a[i] == b[i] {
        i += 1;
    }
    i
}

/// Returns the length of the longest common prefix of `a` and `b`.
///
/// # Examples
///
/// ```
/// # use zfast::utils::lcp;
/// assert_eq!(lcp(b"abcdefghijk", b"abcdefghijx"), 10);
/// assert_eq!(lcp(b"abc", b"abcd"), 3);
/// assert_eq!(lcp(b"", b"abcd"), 0);
/// ```
#[inline]
pub fn lcp(a: &[u8], b: &[u8]) -> usize {
    lcp_from(a, b, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slow_two_fattest(a: usize, b: usize) -> usize {
        (a + 1..=b).max_by_key(|x| x.trailing_zeros()).unwrap()
    }

    #[test]
    fn test_two_fattest() {
        for a in 0..300 {
            for b in a + 1..300 {
                assert_eq!(two_fattest(a, b), slow_two_fattest(a, b), "({a}..{b}]");
            }
        }
    }

    #[test]
    fn test_jump() {
        for h in 1..1000_usize {
            let j = jump(h);
            assert!(j > h);
            assert!(j.trailing_zeros() > h.trailing_zeros());
            for x in h + 1..j {
                assert!(x.trailing_zeros() < h.trailing_zeros());
            }
        }
    }

    #[test]
    fn test_lcp() {
        let a: Vec<u8> = (0..100).collect();
        for l in 0..100 {
            for d in 0..l {
                let mut b = a[..l].to_vec();
                b[d] ^= 0x80;
                assert_eq!(lcp(&a, &b), d);
                assert_eq!(lcp(&b, &a), d);
                for from in 0..=d {
                    assert_eq!(lcp_from(&a, &b, from), d);
                }
            }
            assert_eq!(lcp(&a, &a[..l]), l);
        }
    }
}
