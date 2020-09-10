# Monty-rs - A blazing fast, mostly stupid Monty Hall problem simulator

See the [Wikipedia article](https://en.wikipedia.org/wiki/Monty_Hall_problem)
for an explanation of the Monty Hall problem and its origins.

This project aims to be the fastest Monty Hall simulator in existence.
To that end, some corners are cut:

- Random number generation is fast rather than properly random
- The first option is always chosen as the initial guess
- When an incorrect option is removed following the initial choice, the
simulation does not randomly pick an option to remove, it simply removes
the first incorrect option. 2/3 of the time (ie. when the initial choice is not
correct and there is only one possible option that can be removed) this
makes no difference, and regardless, it doesn't really matter. What we
care about is whether switching is more successful than not switching.

This is likely also the silliest Monty Hall problem simulator in existence.
This is a non-goal.

## Usage

```bash
$ cargo build --release
$ ./target/release/monty-rs
```

### Optional features

The following features can be enabled when building the binary by passing
`--features <feature>` to `cargo`:

* `single-threaded` - use only a single core instead of spawning threads
* `smallvec` - use `smallvec` instead of `tinyvec`
* `tinyvec_v1` - use `tinyvec` 1.0 instead of 0.4
* `vecless` - don't use vecs at all

### Benchmarking with Criterion

```bash
$ cargo bench
$ xdg-open ./target/criterion/report/index.html
```

Ensuing calls to `cargo bench` will provide comparisons to previous measurement.
Depending on CPU/environment, false regressions/improvements may occasionally
be reported, especially if there are many (5+) outliers within the 100 samples
taken by Criterion.

Benchmarks can be run separately with `cargo bench --bench <name of benchmark>`.

### Benchmarking stable vs nightly toolchain

Use the provided `compare_stable_and_nightly.sh` script to compare the performance
of release builds built with each toolchain. The benchmark runs 1,000,000,000 iterations
using [Hyperfine](https://github.com/sharkdp/hyperfine) and optionally performs
`cachegrind` analysis. Run `./compare_stable_and_nightly.sh -h` for a full set of
options.

### Checking memory usage

After building a release build, run `valgrind ./target/release/monty-rs` to check
the total number of allocations. At the time of writing, there are 158 allocations
totalling ~28K of RAM. Running the program in single-threaded mode reduces the
number of allocations to 30 (~4K). Memory usage is constant as the number of iterations
increase.

Running `/usr/bin/time -v ./target/release/monty-rs` shows a maximum resident set of
~2400K in multi-threaded mode compared to ~1800K in single-threaded mode. With massif,
peak heap usage is 8.5K and 1.7K respectively, measured using the following command:

```bash
# Run monty-rs with 10^5 iterations through massif and throw away output
valgrind --tool=massif --massif-out-file=massif.out ./target/release/monty-rs 5 >/dev/null 2>&1; \
# Find all occurrences of 'mem_heap_B', cut out the value and grab the biggest one
grep mem_heap_B massif.out | sed -e 's/mem_heap_B=\(.*\)/\1/' | sort -g | tail -n 1 \
# Find the biggest value and display it along with the neighbouring mem_heap_extra_B
| xargs -i grep -m 1 -A1 'mem_heap_B={}' massif.out
```
