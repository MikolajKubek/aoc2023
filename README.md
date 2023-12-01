# 2023 Advent of Code repository
Repository containing my solutions to [2023 AoC problems](https://adventofcode.com/2023). This year the goal is to attempt solving all AoC tasks using ***Rust*** programming language.

## Project structure
Project is a [cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) so it contains shared `Cargo.lock` file,
top level `Cargo.toml` file and member package for each day.

Each package contains two binaries - `part1` and `part2` that contain the solutions and tests.

```
aoc2023
├── Cargo.lock
├── Cargo.toml
├── day-01
│   ├── Cargo.toml
│   └── src
│       └── bin
│           ├── part1.rs
│           └── part2.rs
├── day-02
│   ├── Cargo.toml
│   └── src
│       └── bin
│           ├── part1.rs
│           └── part2.rs
├── README.md
└── target
```

## How to run
### Run from workspace root
Use `cargo run` specifying desired package to choose a day and binary to indicate part.
```
cargo run -p day-01 --bin part1
```

### Run from package
Use `cargo run` with `--bin` parameter to specify which part should be executed.
```
cd day-01
cargo run --bin part2
```

## TODOs
- [ ] configure `run-all` target
