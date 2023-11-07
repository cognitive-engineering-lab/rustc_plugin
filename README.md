# rustc_plugin

[![Tests](https://github.com/cognitive-engineering-lab/rustc_plugin/actions/workflows/tests.yaml/badge.svg)](https://github.com/cognitive-engineering-lab/rustc_plugin/actions/workflows/tests.yaml)
[![docs](https://img.shields.io/badge/docs-built-blue)][docs]


`rustc_plugin` is a framework for writing programs that use the Rust compiler API. We wrote `rustc_plugin` to support our research on experimental Rust tools like [Flowistry] and [Aquascope]. `rustc_plugin` is a kind of generalized version of the infrastructure in [Clippy].

## Installation

The Rust compiler's interface is not stable, so the only sensible way to develop a Rust compiler plugin is by pinning to a specific nightly. Each version of `rustc_plugin` is pinned to one nightly, and you *have* to use the same nightly version that we do. Therefore each release of `rustc_plugin` has a semantic version number (e.g. `0.1.0`) and the nightly version is added as a prerelease label (e.g. `-nightly-2023-08-25`). You can add a dependency to your `Cargo.toml` like this:
 
```toml
rustc_plugin = "=0.7.3-nightly-2023-08-25"
```

We will treat a change to the nightly version as a breaking change, so the semantic version will be correspondingly updated as a breaking update.

> While you can still technically register a dependency to a plain version number like `0.1.0`, we encourage you to explicitly list the prerelease tag with the `=` requirement for maximum clarity about nightly compatibility.

## Usage

> If you are unfamiliar with the Rust compiler API, then we recommend reading the [Rust Compiler Development Guide](https://rustc-dev-guide.rust-lang.org/). Also check out the [Rustc API documentation](https://doc.rust-lang.org/nightly/nightly-rustc/).

[See the `print-all-items` crate][example] for an example of how to use `rustc_plugin`. [See the `rustc_plugin` docs][docs] for an explanation of each API component. In short, a Rustc plugin is structured like this:

* [`rust-toolchain.toml`](https://github.com/cognitive-engineering-lab/rustc_plugin/blob/main/crates/rustc_plugin/examples/print-all-items/rust-toolchain.toml): specifies the nightly version for your plugin.
* [`Cargo.toml`](https://github.com/cognitive-engineering-lab/rustc_plugin/blob/main/crates/rustc_plugin/examples/print-all-items/Cargo.toml): normal Cargo manifest. Make sure to specify `rustc_private = true` to get Rust Analyzer support for the rustc API.
* [`src/`](https://github.com/cognitive-engineering-lab/rustc_plugin/tree/main/crates/rustc_plugin/examples/print-all-items/src)
  * [`bin/`](https://github.com/cognitive-engineering-lab/rustc_plugin/tree/main/crates/rustc_plugin/examples/print-all-items/src/bin)
    * [`cargo-print-all-items.rs`](https://github.com/cognitive-engineering-lab/rustc_plugin/blob/main/crates/rustc_plugin/examples/print-all-items/src/bin/cargo-print-all-items.rs): the CLI binary run directly by the user, e.g. by invoking `cargo print-all-items`. 
    * [`print-all-items-driver.rs`](https://github.com/cognitive-engineering-lab/rustc_plugin/blob/main/crates/rustc_plugin/examples/print-all-items/src/bin/print-all-items-driver.rs): the implementation binary used by the CLI.
  * [`lib.rs`](https://github.com/cognitive-engineering-lab/rustc_plugin/blob/main/crates/rustc_plugin/examples/print-all-items/src/lib.rs): Your plugin implementation, which exports a data structure that implements the `RustcPlugin` trait.

The `rustc_plugin` framework is responsible for marshalling arguments from the top-level CLI into the individual invocations of the driver. It handles issues like setting the sysroot (so the compiler can locate the Rust standard libraries) and finding the crate that contains a given file (if you only want to run on a specific file). It calls your plugin in a manner that integrates with Cargo, so it handles dependencies and such. Everything else is up to you!


## Utilities

`rustc_plugin` comes with a utilities crate `rustc_utils` that combines many functions that we've found helpful for working with the Rust compiler, especially the MIR. [Check out the `rustc_utils` docs for details.][docs-utils]


## Maximum Supported Rust Version

Normally, Rust libraries have a [minimum supported Rust version][msrv] because they promise to not use any breaking features implemented after that version. Rust compiler plugins are the opposite &mdash; they have a **maximum** supported Rust version (MaxSRV). A compiler plugin cannot analyze programs that use features implemented after the release date of the plugin's toolchain. The MaxSRV for every version of `rustc_plugin` is listed below:

* v0.7 (`nightly-2023-08-25`) - rustc 1.73
* v0.6 (`nightly-2023-04-12`) - rustc 1.69


[Flowistry]: https://github.com/willcrichton/flowistry/
[Aquascope]: https://github.com/cognitive-engineering-lab/aquascope
[Clippy]: https://github.com/rust-lang/rust-clippy
[example]: https://github.com/cognitive-engineering-lab/rustc_plugin/tree/main/crates/rustc_plugin/examples/print-all-items
[docs]: https://cognitive-engineering-lab.github.io/rustc_plugin/v0.7.3-nightly-2023-08-25/rustc_plugin/
[docs-utils]: https://cognitive-engineering-lab.github.io/rustc_plugin/v0.7.3-nightly-2023-08-25/rustc_utils/
[msrv]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field

