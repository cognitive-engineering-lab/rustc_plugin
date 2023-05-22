#!/usr/bin/env bash
set -e

VERSION=$(awk -F'"' '/^version = / {print $2}' crates/rustc_utils/Cargo.toml)
TAG="v${VERSION}"
git tag $TAG
echo "Created tag: $TAG"
echo "Don't forget to update the docs link in the README!"