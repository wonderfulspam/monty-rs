#!/bin/bash

# Small script for comparing binaries compiled with nightly
# and stable.

# Helper function for checking for binaries
function test_bin_exists () {
    BIN="$1"
    which "$BIN" > /dev/null 2>&1 || { echo "$BIN not found in PATH"; exit 1; }
}

test_bin_exists cargo
test_bin_exists hyperfine

echo -n "Building using stable toolchain:  "
rustc +stable --version
cargo +stable build -q --release
cp ./target/release/monty-rs /tmp/monty-rs-stable
echo -n "Building using nightly toolchain: "
rustc +nightly --version
cargo +nightly build -q --release
echo "-----------------------------------------"
hyperfine /tmp/monty-rs-stable ./target/release/monty-rs
# Due to the full utilisation of all available cores,
# CPU clock speed may be throttled to prevent overheating.
# Re-running in reverse order should prove that this isn't
# a fluke
echo "-----------------------------------------"
echo "Re-running benchmarks in opposite order"
hyperfine ./target/release/monty-rs /tmp/monty-rs-stable