#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;

use clap::Parser;
use rustc_middle::ty::TyCtxt;
use rustc_plugin::{RustcPlugin, RustcPluginArgs, Utf8Path};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, env};

pub struct PrintAllItemsPlugin;

#[derive(Parser, Serialize, Deserialize)]
pub struct PrintAllItemsPluginArgs {
  #[arg(short, long)]
  allcaps: bool,
}

impl RustcPlugin for PrintAllItemsPlugin {
  type Args = PrintAllItemsPluginArgs;

  fn version(&self) -> Cow<'static, str> {
    env!("CARGO_PKG_VERSION").into()
  }

  fn driver_name(&self) -> Cow<'static, str> {
    "print-all-items-driver".into()
  }

  fn args(&self, _target_dir: &Utf8Path) -> RustcPluginArgs<Self::Args> {
    let args = PrintAllItemsPluginArgs::parse_from(env::args().skip(1));
    RustcPluginArgs {
      flags: None,
      file: None,
      args,
    }
  }

  fn run(
    self,
    compiler_args: Vec<String>,
    plugin_args: Self::Args,
  ) -> rustc_interface::interface::Result<()> {
    let mut callbacks = PrintAllItemsCallbacks { args: plugin_args };
    let compiler = rustc_driver::RunCompiler::new(&compiler_args, &mut callbacks);
    compiler.run()
  }
}

struct PrintAllItemsCallbacks {
  args: PrintAllItemsPluginArgs,
}

impl rustc_driver::Callbacks for PrintAllItemsCallbacks {
  fn after_parsing<'tcx>(
    &mut self,
    _compiler: &rustc_interface::interface::Compiler,
    queries: &'tcx rustc_interface::Queries<'tcx>,
  ) -> rustc_driver::Compilation {
    queries
      .global_ctxt()
      .unwrap()
      .take()
      .enter(|tcx| print_all_items(tcx, &self.args));

    rustc_driver::Compilation::Stop
  }
}

fn print_all_items(tcx: TyCtxt, args: &PrintAllItemsPluginArgs) {
  let hir = tcx.hir();
  for item_id in hir.items() {
    let item = hir.item(item_id);
    let mut msg = format!(
      "There is an item \"{}\" of type \"{}\"",
      item.ident,
      item.kind.descr()
    );
    if args.allcaps {
      msg = msg.to_uppercase();
    }
    println!("{msg}");
  }
}
