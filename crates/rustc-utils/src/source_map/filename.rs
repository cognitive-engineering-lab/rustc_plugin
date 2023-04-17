use std::path::PathBuf;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Filename(pub PathBuf);

rustc_index::newtype_index! {
  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[debug_format = "f{}"]
  pub struct FilenameIndex {}
}

// Filenames are interned at the thread-level, so they should only be
// used within a given thread. Generally sending an index across a thread
// boundary is a logical error.
impl !Send for FilenameIndex {}
