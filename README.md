# Monty-rs - A blazing fast, mostly stupid Monty Hall problem simulator

See the [Wikipedia article](https://en.wikipedia.org/wiki/Monty_Hall_problem)
for an explanation of the Monty Hall problem and its origins.

This project aims to be the fastest Monty Hall simulator in existence.
To that end, some corners are cut:

- Random number generation is fast rather than properly random
- The first option is always chosen as the initial guess
- When an incorrect option is removed following the initial choice, the
simulation does not randomly pick an option to remove, it simply removes
the first option. Half the time (ie. when the initial choice is not
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

### Benchmarking

```bash
$ cargo bench
```
