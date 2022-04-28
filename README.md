# invaders

![msrv 1.60]

[msrv 1.60]: https://img.shields.io/badge/msrv-1.60-blue

## UI Design

```
+-Space Invaders---------------------------------------------------------------+
| [H]elp | [P]ause | [Q]uit                                                    |
+-Score--------------------------------+-Hiscore-------------------------------+
| 100                                  | 1000                                  |
+--------------------------------------+---------------------------------------+
|        =O=                                                                   |
|                                                                              |
|                     AA AA AA AA AA AA AA AA AA AA AA AA                      |
|                     BB BB BB BB BB BB BB BB BB BB BB BB                      |
|                     BB BB BB BB BB BB BB BB BB BB BB BB                      |
|                     CC CC CC CC CC CC CC CC CC CC CC CC                      |
|                     CC CC CC CC CC CC CC CC CC CC CC CC                      |
|                                                                              |
|                                                                              |
|                                                                              |
|                                                                              |
|      ####                  ####                  ####                ####    |
|     ######                ######                ######              ######   |
|     ######                ######                ######              ######   |
|                                                                              |
|                                              =M=                             |
+------------------------------------------------------------------------------+
| Lives: =M= =M= =M=                                                           |
+------------------------------------------------------------------------------+
```

## Tasks

- Gameplay
    - One cannon laser can exist at a time.
    - Alien deletion should increase the speed of the alien movement.
        - TODO: this needs tuning
    - Lasers should erode bunkers.
    - Aliens should be able to fire lasers.
        - TODO: add support for bidirectional lasers.
    - Up to 3 alien lasers may exist at a time.
    - Alien lasers that hit the cannon should remove a life and reset cannon to initial location.
    - Store hiscores in `${XDG_CACHE_HOME}/invaders/hiscores.toml` and load on start.
    - Add start screen.
- Handle `Ctrl+Z` (`SIGSTOP`/`SIGCONT`).
    - Probably worth handling `SIGINT` properly as well.

## Research

https://www.classicgaming.cc/classics/space-invaders/play-guide

> The more invaders the player shoots, the faster the remaining invaders move. When the invaders are low enough to touch the base shelters they erase them as they pass. The last invader moves very quickly, but slightly faster left to right than right to left.

https://spaceinvaders.fandom.com/wiki/UFO

> If a player hits [the mystery ship], they will receive a random number of either 50, 100, 150, or 200 points. However, if they perform the secret of firing 22 shots, then hitting it on the 23rd shot, then on the 15th shot thereafter, they will receive 300 points every time.

## Build

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
