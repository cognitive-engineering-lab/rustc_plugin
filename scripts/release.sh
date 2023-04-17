#!/usr/bin/env bash
set -e

CHANNEL=$(awk -F'"' '/^channel = / {print $2}' rust-toolchain.toml)
VERSION=$(awk -F'"' '/^version = / {print $2}' crates/rustc-utils/Cargo.toml)
TAG="${CHANNEL}-v${VERSION}"
git tag $TAG
echo "Created tag: $TAG"
echo "Don't forget to update the docs link in the README!"