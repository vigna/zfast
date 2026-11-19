/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
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
