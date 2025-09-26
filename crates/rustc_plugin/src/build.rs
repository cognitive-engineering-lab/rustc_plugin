use std::{
  path::{Path, PathBuf},
  process::Command,
};

fn rustc_path() -> PathBuf {
  let output = Command::new("rustup")
    .args(["which", "--toolchain", crate::CHANNEL, "rustc"])
    .output()
    .expect("failed to run rustup which");
  let rustc_path = String::from_utf8(output.stdout).unwrap();
  PathBuf::from(rustc_path.trim())
}

fn target_libdir(rustc: &Path) -> PathBuf {
  let output = Command::new(rustc)
    .args(["--print", "target-libdir"])
    .output()
    .expect("failed to run rustc --print target-libdir");
  let libdir = String::from_utf8(output.stdout).unwrap();
  PathBuf::from(libdir.trim())
}

pub fn build_main() {
  let rustc_path = rustc_path();
  let target_libdir = target_libdir(&rustc_path);
  println!(
    "cargo::rustc-link-arg=-Wl,-rpath,{}",
    target_libdir.display()
  );
}
