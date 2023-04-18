# rustc-plugin

[![Tests](https://github.com/cognitive-engineering-lab/rustc-plugin/actions/workflows/tests.yaml/badge.svg)](https://github.com/cognitive-engineering-lab/rustc-plugin/actions/workflows/tests.yaml)
[![docs](https://img.shields.io/badge/docs-built-blue)][docs]


`rustc-plugin` is a framework for writing plugins that integrate with the Rust compiler. We wrote `rustc-plugin` to support our research on experimental Rust tools like [Flowistry] and [Aquascope]. `rustc-plugin` is a kind of generalized version of the infrastructure in [Clippy].

## Installation

The Rust compiler's interface is not stable, so the only sensible way to develop a Rust compiler plugin is by pinning to a specific nightly. Each version of `rustc-plugin` is pinned to one nightly, and you *have* to use the same nightly version that we do. Therefore each release of `rustc-plugin` will be tagged with its nightly (e.g. `nightly-2023-04-12`) and its semantic version (e.g. `v0.1.0`). The extra nightly metadata breaks Cargo's semver rules, so we won't be publishing to crates.io. Instead, you should add a git dependency like this:
 
```toml
[dependencies.rustc-plugin]
git = "https://github.com/cognitive-engineering-lab/rustc-plugin"
tag = "nightly-2023-04-12-v0.1.4"
```

## Usage

[See the `print-all-items` crate][example] for an example of how to use `rustc-plugin`. [See the docs][docs] for an explanation of each API component. In short, a Rustc plugin is structured like this:

* [`rust-toolchain.toml`](https://github.com/cognitive-engineering-lab/rustc-plugin/blob/main/crates/rustc-plugin/examples/print-all-items/rust-toolchain.toml): specifies the nightly version for your plugin.
* [`src/`](https://github.com/cognitive-engineering-lab/rustc-plugin/tree/main/crates/rustc-plugin/examples/print-all-items/src)
  * [`bin/`](https://github.com/cognitive-engineering-lab/rustc-plugin/tree/main/crates/rustc-plugin/examples/print-all-items/src/bin)
    * [`cargo-print-all-items.rs`](https://github.com/cognitive-engineering-lab/rustc-plugin/blob/main/crates/rustc-plugin/examples/print-all-items/src/bin/cargo-print-all-items.rs): the CLI binary run directly by the user, e.g. by invoking `cargo print-all-items`. 
    * [`print-all-items-driver.rs`](https://github.com/cognitive-engineering-lab/rustc-plugin/blob/main/crates/rustc-plugin/examples/print-all-items/src/bin/print-all-items-driver.rs): the implementation binary used by the CLI.
  * [`lib.rs`](https://github.com/cognitive-engineering-lab/rustc-plugin/blob/main/crates/rustc-plugin/examples/print-all-items/src/lib.rs): Your plugin implementation, which exports a data structure that implements the `RustcPlugin` trait.

The `rustc-plugin` framework is responsible for marshalling arguments from the top-level CLI into the individual invocations of the driver. It handles issues like setting the sysroot (so the compiler can locate the Rust standard libraries) and finding the crate that contains a given file (if you only want to run on a specific file). Everything else is up to you!

## Utilities

`rustc-plugin` comes with a utilities crate `rustc-utils` that combines many functions that we've found helpful for working with the Rust compiler, especially the MIR. [Check out the docs for details.][docs-utils]

[Flowistry]: https://github.com/willcrichton/flowistry/
[Aquascope]: https://github.com/cognitive-engineering-lab/aquascope
[Clippy]: https://github.com/rust-lang/rust-clippy
[example]: https://github.com/cognitive-engineering-lab/rustc-plugin/tree/main/crates/rustc-plugin/examples/print-all-items
[docs]: https://cognitive-engineering-lab.github.io/rustc-plugin/nightly-2023-04-12-v0.1.4/rustc_plugin/
[docs-utils]: https://cognitive-engineering-lab.github.io/rustc-plugin/nightly-2023-04-12-v0.1.4/rustc_utils/
