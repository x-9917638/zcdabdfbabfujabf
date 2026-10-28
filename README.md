# Cli tool for getting Hacker News stuff

This displays the top posts in Hacker News, supporting all categories in `[new, top, best, ask, show, job]`

## Building
Requires the Rust toolchain.

```bash
cargo build --release
```

Artifact located in `target/release`

## Usage
```
$ hn_cli -h
Usage: hn_cli [ARG]

Arguments:
  [ARG]  [possible values: new, top, best, ask, show, job]

Options:
  -h, --help  Print help
```
