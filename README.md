# 2022 Advent of Code Solutions

Trying a new repo layout for this year based on my `cargo-generate`
[template](https://github.com/mattcl/aoc-template)

### Dependencies

* `just` [crates.io](https://crates.io/crates/just)
* `cargo-generate` [crates.io](https://crates.io/crates/cargo-generate) cargo
* `flamegraph` [crates.io](https://crates.io/crates/flamegraph)

### Building

If you're just looking to build the solver,

```
cargo build -p aoc-cli --release
```

### Running a solution for a given day

The cli exposes subcommands for each day, which can be listed by:

```
./target/release/aoc --help
```

Or you can run a solution via the `run` subcommand:

```
./target/release/aoc run <DAY> <INPUT PATH>
```

Use the `--help` flag with the various subcommands to see more info.

### Tests

To run all the unit tests and problem example tests:

```
cargo test
```

The tests that assert on the actual solutions are ignored by default. To run
those, use the following:
```
cargo test --release -- --ignored
```

### Benchmarks

```
cargo bench
```

Individual benchmarks can be run with `just`

e.g.:

```
just bench 004
```

