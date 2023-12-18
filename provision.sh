#!/bin/sh

cargo clean
rm rsnippet.pkg
cargo build --release
cargo bundle --release
pkgbuild --root ./target/release/bundle/osx/Rsnippet.app --install-location "/Applications/Rsnippet.app" --identifier com.codeitlikemiley.rsnippet --version 0.1.0 --scripts ./scripts rsnippet.pkg