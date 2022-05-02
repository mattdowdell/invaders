# invaders

A Space Invaders TUI in Rust, built using [`tui`](https://crates.io/crates/tui) and
[`crossterm`](https://crates.io/crates/crossterm).

## Demo

https://user-images.githubusercontent.com/20556265/166218629-518bca8d-f492-4357-a1e4-f8e9d6b58069.mov

## Install

To install, use [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```sh
cargo install --git https://github.com/mattdowdell/invaders --tag v0.0.1
```

## Usage

To run the game:

```sh
# using build from `cargo install`
invaders

# with source code
cargo run
```

Some settings are available to tune gameplay:

- `-h, --help`: Print help information
- `--max-cannon-lasers <MAX_CANNON_LASERS>`: The maximum number of cannon lasers that can be present
  (default: 1).
- `--max-invader-lasers <MAX_INVADER_LASERS>`: The maximum number of invader lasers that can be
  present (default: 3).
- `--mystery-ship-interval <MYSTERY_SHIP_INTERVAL>`: The interval in ticks between appearances of
  the mystery ship (default: 200).
- `--start-level <START_LEVEL>`: The level to start at (default: 0).
- `--tick-length <TICK_LENGTH>`: The number of milliseconds per tick (default: 50)

## Build

*Developed using Rust 1.60, but should be compatible with Rust 1.56 (Edition 2021) and later.*

Using [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```sh
# run locally
cargo run

# create debug build
cargo build

# create release build (binary is smaller and runs faster)
cargo build --release
```

Build artefacts will be `target/debug/invaders` or `target/release/invaders` for debug and release
builds respectively.
