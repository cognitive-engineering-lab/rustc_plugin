//! A framework for writing plugins that integrate with the Rust compiler.
//!
//! Much of this library is either directly copy/pasted, or otherwise generalized
//! from the Clippy driver: <https://github.com/rust-lang/rust-clippy/tree/master/src>

#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_session;

pub use build::build_main;
#[doc(hidden)]
pub use cargo_metadata::camino::Utf8Path;
pub use cli::cli_main;
pub use driver::driver_main;
pub use plugin::{CrateFilter, RustcPlugin, RustcPluginArgs};

/// The toolchain channel that this version of rustc_plugin was built with.
///
/// For example, `nightly-2025-08-20`
pub const CHANNEL: &str = env!("RUSTC_CHANNEL");

mod build;
mod cli;
mod driver;
mod plugin;
