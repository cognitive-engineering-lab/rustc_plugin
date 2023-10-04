use std::{
  env, fs,
  path::Path,
  process::{Command, Output},
  sync::Once,
};

use anyhow::{ensure, Context, Result};

static SETUP: Once = Once::new();

fn run(dir: &str, f: impl FnOnce(&mut Command)) -> Result<()> {
  let output = run_configures(dir, true, f)?;

  ensure!(
    output.status.success(),
    "Process exited with non-zero exit code"
  );

  Ok(())
}

fn run_configures(
  dir: &str,
  remove_target: bool,
  f: impl FnOnce(&mut Command),
) -> Result<Output> {
  let root = env::temp_dir().join("rustc_plugin");

  let heredir = Path::new(".").canonicalize()?;

  SETUP.call_once(|| {
    let mut cmd = Command::new("cargo");
    cmd.args([
      "install",
      "--path",
      "examples/print-all-items",
      "--debug",
      "--locked",
      "--root",
    ]);
    cmd.arg(&root);
    cmd.current_dir(&heredir);
    let status = cmd.status().unwrap();
    if !status.success() {
      panic!("installing example failed")
    }
  });

  let mut cmd = Command::new("cargo");
  cmd.arg("print-all-items");

  let path = format!(
    "{}:{}",
    root.join("bin").display(),
    env::var("PATH").unwrap_or_else(|_| "".into())
  );
  cmd.env("PATH", path);

  let ws = heredir.join("tests").join(dir);
  cmd.current_dir(&ws);

  f(&mut cmd);

  if remove_target {
    let _ = fs::remove_dir_all(ws.join("target"));
  }

  cmd.output().context("Process failed")
}

#[test]
fn basic() -> Result<()> {
  run("workspaces/basic", |_cmd| {})
}

#[test]
fn basic_with_arg() -> Result<()> {
  run("workspaces/basic", |cmd| {
    cmd.arg("-a");
  })
}

#[test]
fn multi() -> Result<()> {
  run("workspaces/multi", |_cmd| {})
}

#[test]
fn caching() -> Result<()> {
  let workspace = "workspaces/basic";
  let first_run = run_configures(workspace, false, |_| {})?;

  let second_run = run_configures(workspace, true, |_| {})?;
  ensure!(first_run == second_run);
  Ok(())
}
