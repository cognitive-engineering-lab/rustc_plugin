#![feature(rustc_private)]

fn main() -> std::process::ExitCode {
  env_logger::init();
  rustc_plugin::cli_main(print_all_items::PrintAllItemsPlugin)
}
