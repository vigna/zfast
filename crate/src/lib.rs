/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

#![doc = include_str!("../README.md")]

mod bucketed;
mod dynamic;
mod fixed;
mod keys;
mod paged;
pub mod utils;

pub use bucketed::{Iter as SetIter, ZFastSet};
pub use dynamic::{Iter, ZFastTrie};
pub use fixed::{
    BuildError, DirNode, Dirs, EfDirs, PackedDirs, PackedNode, SeqDirs, SeqNode, Slot,
    StaticZFastTrie,
};
pub use keys::{KeySeq, PackedKeys};
pub use paged::{Iter as PagedIter, PagedKey, ZFastPagedSet};
