use std::{
  env,
  ops::Deref,
  path::{Path, PathBuf},
  process::{exit, Command},
};

use rustc_session::{config::ErrorOutputType, EarlyDiagCtxt};
use rustc_tools_util::VersionInfo;

use super::plugin::{RustcPlugin, PLUGIN_ARGS};
use crate::cli::{RUN_ON_ALL_CRATES, SPECIFIC_CRATE, SPECIFIC_TARGET};

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

fn get_sysroot(orig_args: &[String]) -> (bool, String) {
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
  let early_dcx = EarlyDiagCtxt::new(ErrorOutputType::default());
  rustc_driver::init_rustc_env_logger(&early_dcx);

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
    // or actually execute the plugin. There are two conditions for executing the plugin:
    // 1. Either we're supposed to run on all crates, or CARGO_PRIMARY_PACKAGE is set.
    // 2. --print is NOT passed, since Cargo does that to get info about rustc.
    let primary_package = env::var("CARGO_PRIMARY_PACKAGE").is_ok();
    let run_on_all_crates = env::var(RUN_ON_ALL_CRATES).is_ok();
    let normal_rustc = arg_value(&args, "--print", |_| true).is_some();
    let is_target_crate = match (env::var(SPECIFIC_CRATE), env::var(SPECIFIC_TARGET)) {
      (Ok(krate), Ok(target)) => {
        arg_value(&args, "--crate-name", |name| name == krate).is_some()
          && arg_value(&args, "--crate-type", |name| name == target).is_some()
      }
      _ => true,
    };
    let run_plugin =
      !normal_rustc && (run_on_all_crates || primary_package) && is_target_crate;

    if run_plugin {
      log::debug!("Running plugin...");
      let plugin_args: T::Args =
        serde_json::from_str(&env::var(PLUGIN_ARGS).unwrap()).unwrap();
      plugin.run(args, plugin_args)
    } else {
      log::debug!(
        "Running normal Rust. Relevant variables:\
normal_rustc={normal_rustc}, \
run_on_all_crates={run_on_all_crates}, \
primary_package={primary_package}, \
is_target_crate={is_target_crate}"
      );
      rustc_driver::RunCompiler::new(&args, &mut DefaultCallbacks).run()
    }
  }))
}
