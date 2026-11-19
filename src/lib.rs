/*
 * SPDX-FileCopyrightText: 2026 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

#![doc = include_str!("../README.md")]

mod dynamic;
mod fixed;
mod keys;
pub mod utils;

pub use dynamic::{Iter, ZFastTrie};
pub use fixed::{BuildError, StaticZFastTrie};
pub use keys::{KeySeq, PackedKeys};
