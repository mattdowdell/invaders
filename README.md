# invaders

![rustc 1.60]

[rustc 1.60]: https://img.shields.io/badge/rust-1.60-blue

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

- Add help tab (or popup?).
- Add pause support.
- Move shields into app.

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
