# `pixelmosh`
[![Crates.io](https://img.shields.io/crates/v/pixelmosh)](https://crates.io/crates/pixelmosh)
[![Tests](https://github.com/charlesrocket/pixelmosh/actions/workflows/ci.yml/badge.svg?branch=trunk)](https://github.com/charlesrocket/pixelmosh/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/charlesrocket/pixelmosh/branch/trunk/graph/badge.svg)](https://codecov.io/gh/charlesrocket/pixelmosh)

[![Packaging status](https://repology.org/badge/vertical-allrepos/pixelmosh.svg)](https://repology.org/project/pixelmosh/versions)

### Compilation

#### CLI

```
cargo install pixelmosh
```

#### GUI (GTK)

```
cargo install pixelmosh --features gui
```

### Basic usage

```
pixelmosh foo.png
```

### Options

```
┌─────────────────────────────────────┐
│ █▀▄ █ ▀▄▀ ██▀ █   █▄ ▄█ ▄▀▄ ▄▀▀ █▄█ │
│ █▀  █ █ █ █▄▄ █▄▄ █ ▀ █ ▀▄▀ ▄██ █ █ │
└─────────────────────────────────────┘
PNG corrupter

Usage: pixelmosh [OPTIONS] <FILE>

Arguments:
  <FILE>  File path

Options:
  -n, --min-rate <VALUE>       Minimum chunks to process [default: 1]
  -m, --max-rate <VALUE>       Maximum chunks to process [default: 7]
  -p, --pixelation <VALUE>     Pixelation rate [default: 10]
  -l, --line-shift <VALUE>     Line shift rate [default: 0.3]
  -r, --reverse <VALUE>        Reverse rate [default: 0.3]
  -f, --flip <VALUE>           Flip rate [default: 0.3]
  -c, --channel-swap <VALUE>   Channel swap rate [default: 0.3]
  -t, --channel-shift <VALUE>  Channel shift rate [default: 0.3]
  -a, --ansi                   Use ANSI color palette
  -s, --seed <VALUE>           Custom seed
  -b, --batch <VALUE>          Number of files to output
  -o, --output <VALUE>         Output filename
  -h, --help                   Print help (see more with '--help')
  -V, --version                Print version
```

## `libmosh`
Follow the [example](https://docs.rs/pixelmosh/latest/libmosh/struct.MoshCore.html#example) from [docs.rs](https://docs.rs/pixelmosh/latest/libmosh/).
