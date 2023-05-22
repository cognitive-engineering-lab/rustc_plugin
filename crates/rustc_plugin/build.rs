fn main() {
  let toolchain_toml = include_str!("rust-toolchain.toml");
  let toolchain_table = toolchain_toml.parse::<toml::Table>().unwrap();
  let toolchain = toolchain_table["toolchain"].as_table().unwrap();
  let channel = toolchain["channel"].as_str().unwrap();
  println!("cargo:rustc-env=RUSTC_CHANNEL={channel}");
}
