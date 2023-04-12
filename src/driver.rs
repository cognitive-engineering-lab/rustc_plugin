use std::{
  borrow::Cow,
  env, fs,
  ops::Deref,
  path::{Path, PathBuf},
  process::{exit, Command, Stdio},
};

use cargo_metadata::camino::Utf8Path;
use rustc_tools_util::VersionInfo;
use serde::{de::DeserializeOwned, Serialize};

use super::plugin::{RustcPlugin, PLUGIN_ARGS};

/// If a command-line option matches `find_arg`, then apply the predicate `pred` on its value. If
/// true, then return it. The parameter is assumed to be either `--arg=value` or `--arg value`.
fn arg_value<'a, T: Deref<Target = str>>(
  args: &'a [T],
  find_arg: &str,
  pred: impl Fn(&str) -> bool,
) -> Option<&'a str> {
  let mut args = args.iter().map(Deref::deref);
  while let Some(arg) = args.next() {
    let mut arg = arg.splitn(2, '=');
    if arg.next() != Some(find_arg) {
      continue;
    }

    match arg.next().or_else(|| args.next()) {
      Some(v) if pred(v) => return Some(v),
      _ => {}
    }
  }
  None
}

fn toolchain_path(home: Option<String>, toolchain: Option<String>) -> Option<PathBuf> {
  home.and_then(|home| {
    toolchain.map(|toolchain| {
      let mut path = PathBuf::from(home);
      path.push("toolchains");
      path.push(toolchain);
      path
    })
  })
}

fn get_sysroot(orig_args: &Vec<String>) -> (bool, String) {
  // Get the sysroot, looking from most specific to this invocation to the least:
  // - command line
  // - runtime environment
  //    - SYSROOT
  //    - RUSTUP_HOME, MULTIRUST_HOME, RUSTUP_TOOLCHAIN, MULTIRUST_TOOLCHAIN
  // - sysroot from rustc in the path
  // - compile-time environment
  //    - SYSROOT
  //    - RUSTUP_HOME, MULTIRUST_HOME, RUSTUP_TOOLCHAIN, MULTIRUST_TOOLCHAIN
  let sys_root_arg = arg_value(orig_args, "--sysroot", |_| true);
  let have_sys_root_arg = sys_root_arg.is_some();
  let sys_root = sys_root_arg
    .map(PathBuf::from)
    .or_else(|| std::env::var("MIRI_SYSROOT").ok().map(PathBuf::from))
    .or_else(|| std::env::var("SYSROOT").ok().map(PathBuf::from))
    .or_else(|| {
      let home = std::env::var("RUSTUP_HOME")
        .or_else(|_| std::env::var("MULTIRUST_HOME"))
        .ok();
      let toolchain = std::env::var("RUSTUP_TOOLCHAIN")
        .or_else(|_| std::env::var("MULTIRUST_TOOLCHAIN"))
        .ok();
      toolchain_path(home, toolchain)
    })
    .or_else(|| {
      Command::new("rustc")
        .arg("--print")
        .arg("sysroot")
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .map(|s| PathBuf::from(s.trim()))
    })
    .or_else(|| option_env!("SYSROOT").map(PathBuf::from))
    .or_else(|| {
      let home = option_env!("RUSTUP_HOME")
        .or(option_env!("MULTIRUST_HOME"))
        .map(ToString::to_string);
      let toolchain = option_env!("RUSTUP_TOOLCHAIN")
        .or(option_env!("MULTIRUST_TOOLCHAIN"))
        .map(ToString::to_string);
      toolchain_path(home, toolchain)
    })
    .map(|pb| pb.to_string_lossy().to_string())
    .expect(
      "need to specify SYSROOT env var during clippy compilation, or use rustup or multirust",
    );
  (have_sys_root_arg, sys_root)
}

struct DefaultCallbacks;
impl rustc_driver::Callbacks for DefaultCallbacks {}

/// The top-level function that should be called by your internal driver binary.
pub fn driver_main<T: RustcPlugin>(plugin: T) {
  rustc_driver::init_rustc_env_logger();

  exit(rustc_driver::catch_with_exit_code(move || {
    let mut orig_args: Vec<String> = env::args().collect();

    let (have_sys_root_arg, sys_root) = get_sysroot(&orig_args);

    if orig_args.iter().any(|a| a == "--version" || a == "-V") {
      let version_info = rustc_tools_util::get_version_info!();
      println!("{version_info}");
      exit(0);
    }

    // Setting RUSTC_WRAPPER causes Cargo to pass 'rustc' as the first argument.
    // We're invoking the compiler programmatically, so we ignore this
    let wrapper_mode =
      orig_args.get(1).map(Path::new).and_then(Path::file_stem) == Some("rustc".as_ref());

    if wrapper_mode {
      // we still want to be able to invoke it normally though
      orig_args.remove(1);
    }

    // this conditional check for the --sysroot flag is there so users can call
    // the driver directly without having to pass --sysroot or anything
    let mut args: Vec<String> = orig_args.clone();
    if !have_sys_root_arg {
      args.extend(["--sysroot".into(), sys_root]);
    };

    // On a given invocation of rustc, we have to decide whether to act as rustc,
    // or actually execute the plugin. There are three conditions for executing the plugin:
    // 1. CARGO_PRIMARY_PACKAGE must be set, as we don't run the plugin on dependencies.
    // 2. --print is NOT passed, since Cargo does that to get info about rustc.
    // 3. If rustc is running on src/lib.rs, then we only run the plugin if we're supposed to,
    //    i.e. RUSTC_PLUGIN_LIB_TARGET is set. If the plugin is supposed to run on a reverse-dep
    //    of the lib, then we need to let the lib be checked as normal to generate an rmeta.
    // TODO: document RUSTC_PLUGIN_ALL_TARGETS
    let primary_package = env::var("CARGO_PRIMARY_PACKAGE").is_ok();
    let normal_rustc = args.iter().any(|arg| arg.starts_with("--print"));
    let is_lib = args.iter().any(|arg| arg == "src/lib.rs");
    let is_build_script = args.iter().any(|arg| arg == "build.rs");
    let plugin_lib_target = env::var("RUSTC_PLUGIN_LIB_TARGET").is_ok();
    let plugin_all_targets = env::var("RUSTC_PLUGIN_ALL_TARGETS").is_ok();
    let run_plugin = primary_package
      && !normal_rustc
      && !is_build_script
      && (!is_lib || plugin_lib_target || plugin_all_targets);

    if run_plugin {
      log::debug!("Running plugin...");
      let plugin_args: T::Args = serde_json::from_str(&env::var(PLUGIN_ARGS).unwrap()).unwrap();
      plugin.run(args, plugin_args)
    } else {
      log::debug!(
        "Not running plugin. Relevant variables: \
primary_package={primary_package}, \
normal_rustc={normal_rustc}, \
is_build_script={is_build_script}, \
is_lib={is_lib}, \
plugin_lib_target={plugin_lib_target}, \
plugin_all_targets={plugin_all_targets}"
      );
      rustc_driver::RunCompiler::new(&args, &mut DefaultCallbacks).run()
    }
  }))
}
