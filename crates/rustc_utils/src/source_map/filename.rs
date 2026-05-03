use std::{fmt, path::PathBuf};

use rustc_index::Idx;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Filename(pub PathBuf);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
pub struct FilenameIndex {
  private_use_as_methods_instead: usize,
}

impl fmt::Debug for FilenameIndex {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "f{}", self.private_use_as_methods_instead)
  }
}

impl Idx for FilenameIndex {
  fn new(idx: usize) -> Self {
    FilenameIndex {
      private_use_as_methods_instead: idx,
    }
  }

  fn index(self) -> usize {
    self.private_use_as_methods_instead
  }
}

// NOTE(nightly-2026-05-01): the newtype_index has been commented out below
// bc it uses some wacky `T is range` type that isn't implemented by serde or ts_rs.

// rustc_index::newtype_index! {
//   #[cfg_attr(feature = "serde", derive(serde::Serialize))]
//   #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
//   #[debug_format = "f{}"]
//   pub struct FilenameIndex {}
// }
