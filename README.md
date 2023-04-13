# rustc-plugin

[![Tests](https://github.com/cognitive-engineering-lab/rustc-plugin/actions/workflows/tests.yaml/badge.svg)](https://github.com/cognitive-engineering-lab/rustc-plugin/actions/workflows/tests.yaml)
[![docs](https://img.shields.io/badge/docs-built-blue)][docs]


`rustc-plugin` is a framework for writing plugins that integrate with the Rust compiler. We wrote `rustc-plugin` to support our research on experimental Rust tools like [Flowistry] and [Aquascope]. `rustc-plugin` is a kind of generalized version of the infrastructure in [Clippy].

## Installation

The Rust compiler's interface is not stable (and nightly-only), so the only sensible way to develop Rust compiler plugin is to pin to a specific nightly. Each version of `rustc-plugin` is pinned to a specific nightly, and you *have* to use the same nightly version that we do. Therefore each release of `rustc-plugin` will be tagged with its nightly (e.g. `nightly-2022-12-07`) and its semantic version (e.g. `v0.1.0`). The extra nightly metadata breaks Cargo's semver rules, so we won't be publishing to crates.io. Instead, you should add a git dependency like this:
 
```toml
[dependencies.rustc-plugin]
git = "https://github.com/cognitive-engineering-lab/rustc-plugin"
tag = "nightly-2022-12-07-v0.1.0"
```

## Usage

[See the `print-all-items` crate][example] for an example of how to use `rustc-plugin`. [See the docs][docs] for an explanation of each API component.


[Flowistry]: https://github.com/willcrichton/flowistry/
[Aquascope]: https://github.com/cognitive-engineering-lab/aquascope
[Clippy]: https://github.com/rust-lang/rust-clippy
[example]: https://github.com/cognitive-engineering-lab/rustc-plugin/tree/main/examples/print-all-items
[docs]: https://cognitive-engineering-lab.github.io/rustc-plugin/nightly-2022-12-07-v0.0.1/rustc_plugin/