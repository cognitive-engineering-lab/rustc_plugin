use std::path::PathBuf;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Filename(pub PathBuf);

rustc_index::newtype_index! {
  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
  #[debug_format = "f{}"]
  pub struct FilenameIndex {}
}
