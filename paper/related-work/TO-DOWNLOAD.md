# Papers to download

Consolidated from the three surveys in this directory (`theory.md`,
`practical-indexes.md`, `static-and-uses.md`), which list for each paper what
was read instead. The papers are grouped by how much the text of
`dynzfast.tex` depends on them.

## 1. The new text relies on these through secondary sources, or they settle an open issue

| Paper | Venue, DOI | What is needed |
|---|---|---|
| Bender, Farach-Colton, Kuszmaul, *Cache-oblivious string B-trees* | PODS 2006, 233-242, 10.1145/1142351.1142385 | Exact theorems (search, update, range bounds; w.h.p., amortized or expected; tall cache). The Related work section says only that it is dynamic and has bounds "of the same form" as string B-trees, from the authors' abstract. |
| Ehrhardt, Mulzer, *Delta-fast tries: local searches in bounded universes with linear space* | WADS 2017, LNCS 10389, 361-372, 10.1007/978-3-319-62127-2_31 | The theorem as published, and how the paper refers to the z-fast trie. Cited through the survey of Navarro and Rojas-Ledesma and Ehrhardt's thesis. |
| Dietzfelbinger, Gil, Matias, Pippenger, *Polynomial hash functions are reliable* | ICALP 1992, LNCS 623, 235-246, 10.1007/3-540-55719-9_77 | Section 5 (incremental hashing): the exact family and collision bound cited in the new analysis of signatures (Section 4.3 of the paper). Known through Thorup's notes and our ESA 2010 paper. |
| Fischer, Gawrychowski, *Alphabet-dependent string searching with wexponential search trees* | CPM 2015, LNCS 9133, 160-171, 10.1007/978-3-319-19929-0_14 | The dynamic bound of the published version (arXiv v1: O(m + lg^2 lg sigma / lg lg lg sigma); the survey: O(m + lg lg sigma)) and whether deletions are supported. The text says "a slightly larger additive term". |
| Mortensen, Pagh, Patrascu, *On dynamic range reporting in one dimension* | STOC 2005, 104-111, 10.1145/1060590.1060606 (arXiv cs/0502032) | Whether the O(lg w) / O(lg lg w) bounds are worst case or expected, and the space. Only the abstract was read. |
| Patrascu, Thorup, *Randomization does not help searching predecessors* | SODA 2007, 555-564 | Exact statement (error probability, range of space). Only the abstract was read. |
| Beame, Fich, *Optimal bounds for the predecessor problem and related problems* | JCSS 65(1):38-72, 2002, 10.1006/jcss.2002.1822 | Theorem 3.6 and the dynamic corollary; cited for the optimality of sqrt(lg n / lg lg n). |
| Arbitman, Naor, Segev, *Backyard cuckoo hashing* | FOCS 2010, 10.1109/FOCS.2010.80 (arXiv 0912.5424) | Exact guarantee (constant worst-case time w.h.p., space), now cited in the introduction as the "appropriate dictionary". Abstract read. |
| Ferragina, Grossi, *The string B-tree* | JACM 46(2):236-280, 1999, 10.1145/301970.301973 | Exact bounds for prefix, range and predecessor queries and updates (three secondary descriptions were used). |
| Wu, Ni, Jiang, *Wormhole* (publisher's PDF) | EuroSys 2019, 10.1145/3302424.3303955 | The arXiv v2 text was read and the reference list deposited at Crossref compared; article number and any change in the body are unchecked. |
| Waldvogel, Varghese, Turner, Plattner, *Scalable high-speed prefix matching* | ACM TOCS 19(4):440-482, 2001, 10.1145/502912.502914 | Whether the journal version cites van Emde Boas or Willard; refined marker bounds. |
| Thorup, *On AC0 implementations of fusion trees and atomic heaps*; Andersson, Miltersen, Riis, Thorup, *Static dictionaries on AC0 RAMs* | SODA 2003, 699-707; FOCS 1996, 10.1109/SFCS.1996.548503 | They answer the open `\todo` on multiplications in the model (a constant-time dictionary needs a non-AC0 operation). |
| Pagh, Pagh, Ruzic, *Linear probing with constant independence*; Patrascu, Thorup, *The power of simple tabulation hashing* | SIAM J. Comput. 39(3), 2009, 10.1137/070702278; JACM 59(3), 2012, 10.1145/2220357.2220361 | The independence needed by linear probing, to state the assumptions on the dictionary precisely (open issue of the abstract). |

## 2. Classical papers cited from the literature, never read for this work

van Emde Boas (FOCS 1975, 10.1109/SFCS.1975.26; IPL 1977,
10.1016/0020-0190(77)90031-X); van Emde Boas, Kaas, Zijlstra
(10.1007/BF01683268); Willard, x-fast and y-fast tries (IPL 17(2), 1983,
10.1016/0020-0190(83)90075-3); Mehlhorn, Naher (10.1016/0020-0190(90)90022-P);
Fredman, Willard (10.1016/0022-0000(93)90040-4); Fredman, Saks
(10.1145/73007.73040); Sen, Venkatesh (10.1016/j.jcss.2007.06.016); Alstrup,
Brodal, Rauhe (10.1145/380752.380842); Bayer, Unterauer, *Prefix B-trees* (ACM
TODS 2(1), 1977, 10.1145/320521.320530); Karp, Rabin (IBM J. Res. Dev. 31(2),
1987, 10.1147/rd.312.0249).

## 3. Useful to extend the related work or the experiments

| Paper | Venue, DOI | What is needed |
|---|---|---|
| Belazzougui, *Predecessor search, string algorithms and data structures* | Encyclopedia of Algorithms, 2016, 10.1007/978-1-4939-2864-4_632 | What it says about z-fast tries and strings. |
| Yang, Chen, *Learned indices for string keys with optimized performance and O(N) space cost* | ACM TODS, 2026, 10.1145/3841175 | The context in which it cites *Dynamic z-fast tries*; new baselines. |
| Binna et al., *Height optimized tries* | ACM TODS 47(1), 2022, 10.1145/3506692 | New experiments of the journal version of HOT. |
| Masker et al., *Hyperion* | SIGMOD 2019, 10.1145/3299869.3319870 | Mechanism, datasets, space on strings. |
| Wang et al., *SIndex* | APSys 2020, 10.1145/3409963.3410496 | Mechanism and measurements. |
| Graefe, *Modern B-tree techniques*; *More modern B-tree techniques*; Graefe, Larson, *B-tree indexes and CPU caches* | FnT Databases 3(4), 2011, 10.1561/1900000028; 13(3), 2024, 10.1561/1900000070; ICDE 2001, 10.1109/ICDE.2001.914847 | Normalised keys, prefix and suffix truncation: the right sections to cite if B-trees are discussed in more detail. |
| Gao, Ballijepalli, Wang, *Revisiting B-tree compression* | SIGMOD 2024, 10.1145/3654972 | Only the first page was read. |
| Ferragina, Rotundo, Vinciguerra, *Two-level massive string dictionaries* (and the SPIRE 2023 version) | Inf. Syst. 128, 2025, 10.1016/j.is.2024.102490; 10.1007/978-3-031-43980-3_16 | Layout, query times, bits per string; whether z-fast tries are used. Relevant to the sampled static trie. |
| Ferragina, *On the weak prefix-search problem* | TCS 483:75-84, 2013, 10.1016/j.tcs.2012.06.011 | Its bounds against those of our ESA 2010 paper. |
| Ferragina, Venturini, *Compressed cache-oblivious string B-tree* | ACM TALG 12(4), 2016, 10.1145/2903141 | How it improves on the weak prefix search bound. |
| Gagie, Gawrychowski, Karkkainen, Nekrich, Puglisi, *LZ77-based self-indexing with faster pattern matching* | LATIN 2014, LNCS 8392, 731-742, 10.1007/978-3-642-54423-1_63 | The original statement of the z-fast trie plus Karp-Rabin lemma (not cited in the text for this reason). |
| Kang et al., *PIM-trie* | SPAA 2023, 10.1145/3558481.3591070 | Read only in part. |
| Kanda, Morita, Fuketa, *Compressed double-array tries* ; Yata, *Dictionary compression by nesting prefix/patricia tries* (Marisa) | KAIS 51(3), 2017, 10.1007/s10115-016-0999-8; NLP (Japan) 2011 | Static competitors; the second reference is unverified. |
| Hsu, Ottaviano, *Space-efficient data structures for top-k completion* | WWW 2013, 10.1145/2488388.2488440 | Whether it uses z-fast tries. |
| Arroyuelo et al., *Engineering practical Lempel-Ziv tries* | ACM JEA 26, 2021, 10.1145/3481638 | Full text. |
| Heinz, Zobel, Williams, *Burst tries*; Askitis, Sinha, *HAT-trie* (both papers) | ACM TOIS 20(2), 2002, 10.1145/506309.506312; VLDB J. 19(5), 2010, 10.1007/s00778-010-0183-9 | Only if tries with containers are discussed. |
| Dharmapurikar et al.; Yuan, Crowley; Jiang et al. (two versions) | 10.1145/863955.863979; 10.1109/ANCS.2015.7110125; 10.1145/3600061.3600077, 10.1109/TNET.2024.3504244 | Only if IP and name lookup are discussed in more than a sentence. |
| Dictionaries: Arbitman, Naor, Segev, *De-amortized cuckoo hashing*; Dietzfelbinger, Meyer auf der Heide, ICALP 1990; Bender et al., *Iceberg hashing* | 10.1007/978-3-642-02927-1_11; 10.1007/BFb0032018; JACM 70(6), 2023, 10.1145/3625817 | Alternatives for the high-probability dictionary. |
| Range filters: Rosetta, REncoder, Memento (full texts); Aeris and Hourglass (SIGMOD 2026, no URL obtained) | 10.1145/3318464.3389731; 10.1109/ICDE55515.2023.00158; 10.1145/3698820 | Only if range filters enter the related work. |
| Rabin, *Fingerprinting by random polynomials* (TR-15-81, Harvard, 1981); Broder, *Some applications of Rabin's fingerprinting method* | 10.1007/978-1-4613-9323-8_11 | Only to justify CRC-like signatures. |

## 4. Not papers

- DBLP was unreachable for all three surveys: bibliographic data were checked
  against Crossref, DataCite or arXiv. DBLP-specific data (article numbers of
  ACM proceedings, for example) are unchecked.
- Google Scholar "cited by" lists for the six z-fast papers were not
  reachable: the list of works using z-fast tries is a lower bound.
- Leads not followed in the citing lists: Yu, Belazzougui, Qian, Zhang (ICNP
  2017, 10.1109/ICNP.2017.8117530); Golan, Porat (ESA 2017,
  10.4230/LIPIcs.ESA.2017.41); Orlandi, Venturini (Algorithmica 74(1), 2016,
  10.1007/s00453-014-9936-y); Takagi et al., *Applications of succinct dynamic
  compact tries to some string problems* (2013).
