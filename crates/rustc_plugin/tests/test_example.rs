use std::{env, fs, path::Path, process::Command, sync::Once};

use anyhow::{Context, Result, ensure};

static SETUP: Once = Once::new();

fn run(dir: &str, f: impl FnOnce(&mut Command)) -> Result<String> {
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

  let _ = fs::remove_dir_all(ws.join("target"));

  let output = cmd.output().context("Process failed")?;
  ensure!(
    output.status.success(),
    "Process exited with non-zero exit code. Stderr:\n{}",
    String::from_utf8(output.stderr)?
  );

  Ok(String::from_utf8(output.stdout)?)
}

// TODO: why do these tests need to be run sequentially?

#[test]
fn basic() -> Result<()> {
  let output = run("workspaces/basic", |_cmd| {})?;
  assert!(output.contains(r#"There is an item "add" of type "function""#));
  Ok(())
}

#[test]
fn arg() -> Result<()> {
  let output = run("workspaces/basic", |cmd| {
    cmd.arg("-a");
  })?;
  assert!(output.contains(r#"THERE IS AN ITEM "ADD" OF TYPE "FUNCTION""#));
  Ok(())
}

#[test]
fn feature() -> Result<()> {
  let output = run("workspaces/basic", |cmd| {
    cmd.args(["--", "--features", "sub"]);
  })?;
  assert!(
    output.contains(r#"There is an item "sub" of type "function""#),
    "output:\n{output}"
  );
  Ok(())
}

#[test]
fn multi() -> Result<()> {
  run("workspaces/multi", |_cmd| {})?;
  Ok(())
}
