#!/bin/sh

cargo clean
rm snip.pkg
cargo zigbuild --release
cargo bundle --release
pkgbuild --root ./target/release/bundle/osx/snip.app --install-location "/Applications/snip.app" --identifier com.codeitlikemiley.snip --version 0.1.0 --scripts ./scripts snip.pkg