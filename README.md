# invaders

A Space Invaders TUI in Rust, built using [`tui`](https://crates.io/crates/tui) and
[`crossterm`](https://crates.io/crates/crossterm).

## Demo

*To do: Add demo GIF.*

## Usage

To run the game:

```sh
# using pre-compiled binary
./invaders

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

# debug build
cargo build

# release build
cargo build --release
```

Build artefacts will be `target/debug/invaders` or `target/release/invaders` for debug and release
builds respectively.

Release builds can be created using `make` and output `invaders-<target>.tar.gz` in the root of the
repository, where `target` is the architecture and OS being built upon:

```sh
# local build
make build

# linux build (using docker)
make build-docker

# clean release artifacts
make clean
```

## Tasks

- Gameplay
    - Invaders should erase bunkers when coming into contact with them.
    - Add support for hiscores.
        - Store hiscores in `${XDG_CACHE_HOME}/invaders/hiscores.toml`
            - Commented-out helper function in `main.rs`.
        - Game over should check if the score is in the top 5.
        - If score is in top 5, write new hiscores to file.
            - Bonus: Accept name of player for hiscore.
        - Hiscores should be loaded on start, added to the start screen, game screen, and game over screen.
    - Make mystership appearances less frequent by default.
- Handle `Ctrl+Z` (`SIGSTOP`/`SIGCONT`).
    - Probably worth handling `SIGINT` properly as well.
- Add check on terminal size before attempting to render to avoid panics
    - Pause game if terminal is resized too small during gameplay.
- Add dockerfile for Linux binaries
