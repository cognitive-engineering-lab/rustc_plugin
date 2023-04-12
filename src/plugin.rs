use std::{borrow::Cow, path::PathBuf};

use cargo_metadata::camino::Utf8Path;

use serde::{de::DeserializeOwned, Serialize};

/// Arguments from your plugin to the rustc-plugin framework.
pub struct RustcPluginArgs<Args> {
  /// Whatever CLI arguments you want to pass along.
  pub args: Args,

  /// Any compiler flags you want to add to the Cargo invocation.
  pub flags: Option<Vec<String>>,

  /// A specific file that you want to execute the plugin on.
  ///
  /// The framework will automatically detect the crate containing
  /// the file, and only run on that crate.
  pub file: Option<PathBuf>,
}

/// Interface between your plugin and the rustc-plugin framework.
pub trait RustcPlugin: Sized {
  /// Command-line arguments passed by the user.
  type Args: Serialize + DeserializeOwned;

  /// Returns the version of your plugin.
  ///
  /// A sensible default is your plugin's Cargo version:
  ///
  /// ```ignore
  /// env!("CARGO_PKG_VERSION").into()
  /// ```
  fn version(&self) -> Cow<'static, str>;

  /// Returns the name of your driver binary as it's installed in the filesystem.
  ///
  /// Should be just the filename, not the full path.
  fn driver_name(&self) -> Cow<'static, str>;

  /// Parses and returns the CLI arguments for the plugin.
  fn args(&self, target_dir: &Utf8Path) -> RustcPluginArgs<Self::Args>;

  /// Executes the plugin with a set of compiler and plugin args.
  fn run(
    self,
    compiler_args: Vec<String>,
    plugin_args: Self::Args,
  ) -> rustc_interface::interface::Result<()>;
}

/// The name of the environment variable shared between the CLI and the driver.
/// Must not conflict with any other env var used by Cargo.
pub const PLUGIN_ARGS: &str = "PLUGIN_ARGS";
