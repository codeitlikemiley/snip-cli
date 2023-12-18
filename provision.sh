#!/bin/sh

cargo clean
rm snip.pkg
cargo build --release
cargo bundle --release
pkgbuild --root ./target/release/bundle/osx/Snip.app --install-location "/Applications/Snip.app" --identifier com.codeitlikemiley.snip --version 0.1.0 --scripts ./scripts snip.pkg