# generate the boilerplate for a new day's problem `just new 1 foo-bar-baz`
new DAY NAME:
    scripts/new.sh {{DAY}} {{NAME}}

# run benchmarks for a given padded day `just bench 001`
bench DAY:
    cargo bench -p aoc-benchmarking -- {{DAY}}

# run all benchmarks
bench_all:
    cargo bench -p aoc-benchmarking

flame DAY:
    scripts/flame.sh {{DAY}}
