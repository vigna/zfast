/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//! Invariant checking for the bucketed set.

use super::*;

impl<H: HashPrefixes> ZFastSet<H> {
    /// Checks all the invariants of the structure.
    ///
    /// This method is very slow, and it is meant to be used in tests.
    ///
    /// # Panics
    ///
    /// If some invariant is violated.
    #[doc(hidden)]
    pub fn check(&self) {
        self.trie.check();
        // The anchors, in order, with their buckets
        let anchors: Vec<(Vec<u8>, u32)> = {
            let mut anchors = Vec::new();
            let mut entry = self.trie.pred_entry(&[]).map(|(a, b)| (a.to_vec(), b));
            while let Some((anchor, b)) = entry {
                entry = self.trie.next_entry(&anchor).map(|(a, b)| (a.to_vec(), b));
                anchors.push((anchor, b));
            }
            anchors
        };
        assert_eq!(anchors.len(), self.trie.len());
        assert_eq!(anchors.len(), self.num_buckets());
        assert_eq!(anchors[0], (vec![], self.head));
        assert_eq!(anchors[anchors.len() - 1].1, self.tail);

        let mut len = 0;
        let mut prev_key: Option<&[u8]> = None;
        for (j, (anchor, b)) in anchors.iter().enumerate() {
            let bucket = &self.buckets[*b as usize];
            let n = bucket.len();
            len += n;
            assert!(n <= CAP);
            assert!(n > 0 || j == 0, "empty bucket {b}");
            assert_eq!(bucket.prev, if j == 0 { NONE } else { anchors[j - 1].1 });
            assert_eq!(bucket.next, anchors.get(j + 1).map_or(NONE, |a| a.1));
            let p = bucket.prefix_len as usize;
            for i in 0..CAP {
                assert_eq!(bucket.keys[i].is_some(), i < n);
                if i >= n {
                    assert_eq!(bucket.heads[i], 0);
                    continue;
                }
                let key = bucket.key(i);
                // Keys are in order, also across buckets, and each key lies
                // between the anchor of its bucket and the next one
                assert!(prev_key.is_none_or(|prev| prev < key), "keys out of order");
                prev_key = Some(key);
                assert!(anchor.as_slice() <= key);
                assert!(
                    anchors
                        .get(j + 1)
                        .is_none_or(|next| key < next.0.as_slice())
                );
                // Summaries
                assert_eq!(bucket.tags[i], tag(key));
                assert!(key.len() >= p);
                assert_eq!(&key[..p], &bucket.key(0)[..p]);
                assert_eq!(bucket.heads[i], head(key, p));
                assert_eq!(bucket.find(key), Some(i));
                assert_eq!(bucket.search(key), (i, true));
            }
            if n > 0 {
                assert_eq!(bucket.prefix_hash, xxh3_64(&bucket.key(0)[..p]));
            }
        }
        assert_eq!(len, self.len);
        for &b in &self.free {
            assert_eq!(self.buckets[b as usize].len(), 0);
        }
    }
}
