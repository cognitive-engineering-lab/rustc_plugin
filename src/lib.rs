//! A framework for writing plugins that integrate with the Rust compiler.
//!
//! Much of this library is either directly copy/pasted, or otherwise generalized
//! from the Clippy driver: <https://github.com/rust-lang/rust-clippy/tree/master/src>

#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;

use std::{
  borrow::Cow,
  env, fs,
  ops::Deref,
  path::{Path, PathBuf},
  process::{exit, Command, Stdio},
};

#[doc(hidden)]
pub use cargo_metadata::camino::Utf8Path;

use rustc_tools_util::VersionInfo;
use serde::{de::DeserializeOwned, Serialize};

pub use cli::cli_main;
pub use driver::driver_main;
pub use plugin::{RustcPlugin, RustcPluginArgs};

mod cli;
mod driver;
mod plugin;
