# rustc-plugin

[![Tests](https://github.com/cognitive-engineering-lab/rustc-plugin/actions/workflows/tests.yaml/badge.svg)](https://github.com/cognitive-engineering-lab/rustc-plugin/actions/workflows/tests.yaml)

`rustc-plugin` is a framework for writing plugins that integrate with the Rust compiler. We wrote `rustc-plugin` to support our research on experimental Rust tools like [Flowistry](https://github.com/willcrichton/flowistry/) and [Aquascope](https://github.com/cognitive-engineering-lab/aquascope). `rustc-plugin` is a kind of generalized version of the infrastructure in [Clippy](https://github.com/rust-lang/rust-clippy).

## Installation

The Rust compiler's interface is not stable (and nightly-only), so the only sensible way to develop Rust compiler plugin is to pin to a specific nightly. Each version of `rustc-plugin` is pinned to a specific nightly, and you *have* to use the same nightly version that we do. Therefore each release of `rustc-plugin` will be tagged with its nightly (e.g. `nightly-2022-12-07`) and its semantic version (e.g. `v0.1.0`). The extra nightly metadata breaks Cargo's semver rules, so we won't be publishing to crates.io. Instead, you should add a git dependency like this:
 
```toml
[dependencies.rustc-plugin]
git = "https://github.com/cognitive-engineering-lab/rustc-plugin"
tag = "nightly-2022-12-07-v0.1.0"
```

## Usage

See the [`print-all-items`](https://github.com/cognitive-engineering-lab/rustc-plugin/tree/main/examples/print-all-items) crate for an example of how to use `rustc-plugin`. 
