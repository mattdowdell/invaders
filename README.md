# invaders

![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/mattdowdell/invaders?label=version&style=for-the-badge)
![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-V1.70.0-blue?style=for-the-badge)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/mattdowdell/invaders/continuous-integration.yml?branch=master&style=for-the-badge)

A Space Invaders TUI in Rust, built using [`tui`](https://crates.io/crates/tui) and
[`crossterm`](https://crates.io/crates/crossterm).

## Demo

https://user-images.githubusercontent.com/20556265/166218629-518bca8d-f492-4357-a1e4-f8e9d6b58069.mov

## Install

To install, use [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```sh
cargo install --git https://github.com/mattdowdell/invaders --tag v0.0.3
```

See [here](https://github.com/mattdowdell/invaders/tags) for a full list of tags.

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
  the mystery ship (default: 2000).
- `--start-level <START_LEVEL>`: The level to start at (default: 0).
- `--tick-length <TICK_LENGTH>`: The number of milliseconds per tick (default: 50)
