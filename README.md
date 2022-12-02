# aoc-2022-rs

Advent of Code 2022 Solutions

## Test a solution

```sh
cargo test d<n>p<xn> # e.g. day 1 part 1 --> d1p1
```

You can use the `RUST_LOG` environment variable to see logs from a specific function:

```sh
RUST_LOG="[process_contestant]=trace" cargo test d1p2
```
