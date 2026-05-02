#![feature(rustc_private)]

fn main() -> std::process::ExitCode {
  env_logger::init();
  rustc_plugin::driver_main(print_all_items::PrintAllItemsPlugin)
}
