# Advent of Code 2015
Not-very-good solutions to [Advent of Code](https://adventofcode.com/2015).

## Project Structure
Each day currently stands alone. From any dayXX folder, `cargo run` or `cargo build` should work.

## How to Interact
### Days 1-7
None of the challenge inputs are stored in this repo. Each of the solutions expects to have the inputs provided to standard input. This can take the form of:
- `pbpaste | cargo run`
- `cargo run <enter>` followed by one or more lines, followed by `ctrl-d`

The second form lets you do some pseudo-REPL activities, where you can enter a few trial lines, then find out how the solver solves them.

### Day 8
To avoid issues with quotes and shells, this expects the input to be in a a file. Without a filename being passed, it'll try to open `example.txt`. Passing a filename as the only argument to the binary will use that filename instead.