use std::{
  env, fs,
  path::PathBuf,
  process::{exit, Command, Stdio},
};

use cargo_metadata::camino::Utf8Path;

use super::plugin::{RustcPlugin, PLUGIN_ARGS};
use crate::CrateFilter;

pub const RUN_ON_ALL_CRATES: &str = "RUSTC_PLUGIN_ALL_TARGETS";
pub const SPECIFIC_CRATE: &str = "SPECIFIC_CRATE";
pub const SPECIFIC_TARGET: &str = "SPECIFIC_TARGET";
pub const CARGO_VERBOSE: &str = "CARGO_VERBOSE";

/// The top-level function that should be called in your user-facing binary.
pub fn cli_main<T: RustcPlugin>(plugin: T) {
  if env::args().any(|arg| arg == "-V") {
    println!("{}", plugin.version());
    return;
  }

  let metadata = cargo_metadata::MetadataCommand::new()
    .no_deps()
    .other_options(["--all-features".to_string(), "--offline".to_string()])
    .exec()
    .unwrap();
  let plugin_subdir = format!("plugin-{}", env!("RUSTC_CHANNEL"));
  let target_dir = metadata.target_directory.join(plugin_subdir);

  let args = plugin.args(&target_dir);

  let mut cmd = Command::new("cargo");
  cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

  let mut path = env::current_exe()
    .expect("current executable path invalid")
    .with_file_name(plugin.driver_name().as_ref());

  if cfg!(windows) {
    path.set_extension("exe");
  }

  cmd
    .env("RUSTC_WORKSPACE_WRAPPER", path)
    .args(["check", "--target-dir"])
    .arg(&target_dir);

  if env::var(CARGO_VERBOSE).is_ok() {
    cmd.arg("-vv");
  } else {
    cmd.arg("-q");
  }

  let workspace_members = metadata
    .workspace_members
    .iter()
    .map(|pkg_id| {
      metadata
        .packages
        .iter()
        .find(|pkg| &pkg.id == pkg_id)
        .unwrap()
    })
    .collect::<Vec<_>>();

  match args.filter {
    CrateFilter::CrateContainingFile(file_path) => {
      only_run_on_file(&mut cmd, file_path, &workspace_members, &target_dir);
    }
    CrateFilter::AllCrates | CrateFilter::OnlyWorkspace => {
      cmd.arg("--all");
      match args.filter {
        CrateFilter::AllCrates => {
          cmd.env(RUN_ON_ALL_CRATES, "");
        }
        CrateFilter::OnlyWorkspace => {}
        CrateFilter::CrateContainingFile(_) => unreachable!(),
      }
    }
  }

  let args_str = serde_json::to_string(&args.args).unwrap();
  log::debug!("{PLUGIN_ARGS}={args_str}");
  cmd.env(PLUGIN_ARGS, args_str);

  // HACK: if running on the rustc codebase, this env var needs to exist
  // for the code to compile
  if workspace_members.iter().any(|pkg| pkg.name == "rustc-main") {
    cmd.env("CFG_RELEASE", "");
  }

  plugin.modify_cargo(&mut cmd, &args.args);

  let exit_status = cmd.status().expect("failed to wait for cargo?");

  exit(exit_status.code().unwrap_or(-1));
}

fn only_run_on_file(
  cmd: &mut Command,
  file_path: PathBuf,
  workspace_members: &[&cargo_metadata::Package],
  target_dir: &Utf8Path,
) {
  // We compare this against canonicalized paths, so it must be canonicalized too
  let file_path = file_path.canonicalize().unwrap();

  // Find the package and target that corresponds to a given file path
  let mut matching = workspace_members
    .iter()
    .filter_map(|pkg| {
      let targets = pkg
        .targets
        .iter()
        .filter(|target| {
          let src_path = target.src_path.canonicalize().unwrap();
          log::trace!("Package {} has src path {}", pkg.name, src_path.display());
          file_path.starts_with(src_path.parent().unwrap())
        })
        .collect::<Vec<_>>();

      let target = (match targets.len() {
        0 => None,
        1 => Some(targets[0]),
        _ => {
          // If there are multiple targets that match a given directory, e.g. `examples/whatever.rs`, then
          // find the target whose name matches the file stem
          let stem = file_path.file_stem().unwrap().to_string_lossy();
          let name_matches_stem = targets
            .clone()
            .into_iter()
            .find(|target| target.name == stem);

          // Otherwise we're in a special case, e.g. "main.rs" corresponds to the bin target.
          name_matches_stem.or_else(|| {
            let only_bin = targets
              .iter()
              .all(|target| !target.kind.contains(&"lib".into()));
            // TODO: this is a pile of hacks, and it seems like there is no reliable way to say
            // which target a file will correspond to given only its filename. For example,
            // if you have src/foo.rs it could either be imported by src/main.rs, or src/lib.rs, or
            // even both!
            if only_bin {
              targets
                .into_iter()
                .find(|target| target.kind.contains(&"bin".into()))
            } else {
              let kind = (if stem == "main" { "bin" } else { "lib" }).to_string();
              targets
                .into_iter()
                .find(|target| target.kind.contains(&kind))
            }
          })
        }
      })?;

      Some((pkg, target))
    })
    .collect::<Vec<_>>();
  let (pkg, target) = match matching.len() {
    0 => panic!("Could not find target for path: {}", file_path.display()),
    1 => matching.remove(0),
    _ => panic!("Too many matching targets: {matching:?}"),
  };

  // Add compile filter to specify the target corresponding to the given file
  cmd.arg("-p").arg(format!("{}:{}", pkg.name, pkg.version));

  enum CompileKind {
    Lib,
    Bin,
    ProcMacro,
  }

  // kind string should be one of the ones listed here:
  // https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field
  let kind_str = &target.kind[0];
  let kind = match kind_str.as_str() {
    "lib" | "rlib" | "dylib" | "staticlib" | "cdylib" => CompileKind::Lib,
    "bin" => CompileKind::Bin,
    "proc-macro" => CompileKind::ProcMacro,
    _ => unreachable!("unexpected cargo crate type: {kind_str}"),
  };

  match kind {
    CompileKind::Lib => {
      // If the rmeta files were previously generated for the lib (e.g. by running the plugin
      // on a reverse-dep), then we have to remove them or else Cargo will memoize the plugin.
      let deps_dir = target_dir.join("debug").join("deps");
      if let Ok(entries) = fs::read_dir(deps_dir) {
        let prefix = format!("lib{}", pkg.name.replace('-', "_"));
        for entry in entries {
          let path = entry.unwrap().path();
          if let Some(file_name) = path.file_name() {
            if file_name.to_string_lossy().starts_with(&prefix) {
              fs::remove_file(path).unwrap();
            }
          }
        }
      }

      cmd.arg("--lib");
    }
    CompileKind::Bin => {
      cmd.args(["--bin", &target.name]);
    }
    CompileKind::ProcMacro => {}
  }

  cmd.env(SPECIFIC_CRATE, &pkg.name.replace('-', "_"));
  cmd.env(SPECIFIC_TARGET, kind_str);

  log::debug!(
    "Package: {}, target kind {}, target name {}",
    pkg.name,
    kind_str,
    target.name
  );
}
