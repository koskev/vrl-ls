# Language Server for the Vector Remap Language

This is just a PoC implementation and thus not very pretty.

[Example videos](https://koskev.github.io/vrl-ls/showcase.html)

## Installation

Either compile locally with
```
cargo build --release
```
And run `./target/release/vrl-ls`

Or install to `~/.cargo/bin` with
```
cargo install --path .
```

## Known Bugs
 - "." Assignments also complete the stdlib

## Features
It is just a very small ls with only a few features
 - Complete variables
 - Show diagnostics (errors and warnings)
 - Complete the standard library
 - Goto definition
 - Find references
 - Rename
