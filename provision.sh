#!/bin/sh

cargo clean
rm snipr.pkg
cargo zigbuild --release
cargo bundle --release
pkgbuild --root ./target/release/bundle/osx/snipr.app --install-location "/Applications/snipr.app" --identifier com.codeitlikemiley.snipr --version 0.1.0 --scripts ./scripts snipr.pkg