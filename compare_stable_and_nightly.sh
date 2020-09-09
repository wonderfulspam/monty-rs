#!/bin/bash

# Small script for comparing binaries compiled with nightly
# and stable.

########################
##  HELPER FUNCTIONS  ##
########################
# Exit if binary does not exist
function test_bin_exists () {
    BIN="$1"
    which "$BIN" > /dev/null 2>&1 || { echo "$BIN not found in PATH"; exit 1; }
}

# Run cachegrind and extract total number of instructions
function run_cachegrind () {
    INSTRUCTIONS=$(valgrind --tool=cachegrind --cachegrind-out-file=cachegrind.$4.out $1 $2 2>&1 | grep -oP "(I\s+refs:\s+)\K([\d,]+)")
    INS=$(echo $INSTRUCTIONS | tr -d ',')
    INS_PER_SIM=$(echo "scale=3; $INS/$3" | bc)
    printf "%s (%g instructions per simulation)\n" $INSTRUCTIONS $INS_PER_SIM
}

########################
##     ARG PARSING    ##
########################
USE_CACHEGRIND=
CACHEGRIND_ITERS=5 # Number of simulations to run, given as an exponent of 10
RERUN_REVERSE=

_usage="Usage: $0 [-c] [-i value] [-r]

Options:
  -c            Enable cachegrind
  -i <iters>    No. of simulations to run with cachegrind, given as an exponent of 10.
                  Default 5, implies -c.

  -r            Re-run hyperfine benchmark in reverse order
  -h            Show usage
"

while getopts ci:rh arg
do
    case $arg in
    c) USE_CACHEGRIND=1;;
    i) USE_CACHEGRIND=1; CACHEGRIND_ITERS="$OPTARG";;
    r) RERUN_REVERSE=1;;
    h) echo "$_usage"; exit 0;;
    *) echo "$_usage"; exit 1;;
    esac
done

########################
##   SANITY CHECKS    ##
########################
test_bin_exists cargo
test_bin_exists hyperfine

if [ ! -z "$USE_CACHEGRIND" ]; then
    test_bin_exists valgrind
fi

########################
##  BINARY BUILDING   ##
########################
echo -n "Building using stable toolchain:  "
rustc +stable --version || exit 1;
cargo +stable build -q --release
cp ./target/release/monty-rs /tmp/monty-rs-stable
echo -n "Building using nightly toolchain: "
rustc +nightly --version || exit 1;
cargo +nightly build -q --release

########################
##     CACHEGRIND     ##
########################
if [ ! -z "$USE_CACHEGRIND" ]; then
    echo "-----------------------------------------"
    ITERS=$((10**$CACHEGRIND_ITERS)) 
    printf "Running cachegrind on %s simulations\n" $(numfmt --grouping $ITERS)
    echo -n "Total number of instructions (stable):  "
    run_cachegrind /tmp/monty-rs-stable $CACHEGRIND_ITERS $ITERS stable
    echo -n "Total number of instructions (nightly): "
    run_cachegrind ./target/release/monty-rs $CACHEGRIND_ITERS $ITERS nightly
    echo "Cachegrind output printed to cachegrind.<toolchain>.out"
fi

########################
##     HYPERFINE      ##
########################
echo "-----------------------------------------"
echo "Benchmarking 1,000,000,000 simulations"
hyperfine /tmp/monty-rs-stable ./target/release/monty-rs

if [ ! -z "$RERUN_REVERSE" ]; then
    # Due to the full utilisation of all available cores,
    # CPU clock speed may be throttled to prevent overheating.
    # Re-running in reverse order should prove that this isn't
    # a fluke
    echo "-----------------------------------------"
    echo "Re-running benchmarks in reverse order"
    hyperfine ./target/release/monty-rs /tmp/monty-rs-stable
fi