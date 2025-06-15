# Advent of code exercises, in Rust

This repo contains solutions to all advent of code challenges. It's organised as a cargo workspace, where each year corresponds to a crate.

## Prerequisites

Install rust with your favourite package manager, or with [`rustup`](https://rustup.rs).

## Running the solutions

Each solution needs an input. To fetch the inputs for a specific year and day, run this command:

```
# Log in to https://adventofcode.com/ and copy the value from your `session` cookie in your browser
ADVENT_OF_CODE_SESSION="...." ./fetch_input.sh [YEAR] [DAY]
```

To run a specific day's solution, navigate to the corresponding crate and run:

```
cargo run -p year_{YEAR} {DAY}
```

For example, to fetch the input and run the solution for the first day of 2015, run:

```
$ ADVENT_OF_CODE_SESSION="...." ./fetch_input.sh 2015 1
$ cargo run -p year_2015 1
```
