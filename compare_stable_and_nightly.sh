#!/bin/bash

# Helper function for checking for binaries
function test_bin_exists () {
    BIN="$1"
    which "$BIN" > /dev/null 2>&1 || { echo "$BIN not found in PATH"; exit 1; }
}

test_bin_exists cargo
test_bin_exists hyperfine

echo "Building nightly version"
cargo +nightly build -q --release
cp ./target/release/monty-rs /tmp/monty-rs-nightly
echo "Building stable version"
cargo +stable build -q --release
echo "Running benchmarks"
hyperfine /tmp/monty-rs-nightly ./target/release/monty-rs