#![feature(rustc_private)]

fn main() {
  env_logger::init();
  rustc_plugin::driver_main(print_all_items::PrintAllItemsPlugin);
}
