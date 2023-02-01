# `pixelmosh`
[![Crates.io](https://img.shields.io/crates/v/pixelmosh)](https://crates.io/crates/pixelmosh)
[![Tests](https://github.com/charlesrocket/pixelmosh/actions/workflows/tests.yml/badge.svg?branch=master)](https://github.com/charlesrocket/pixelmosh/actions/workflows/tests.yml)
[![codecov](https://codecov.io/gh/charlesrocket/pixelmosh/branch/master/graph/badge.svg)](https://codecov.io/gh/charlesrocket/pixelmosh)

### Compilation

#### CLI

```
cargo install --locked pixelmosh
```

#### GUI

```
cargo install --locked pixelmosh --features gui
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
  -n, --min-rate <MIN_RATE>            Minimum chunks to process [default: 1]
  -m, --max-rate <MAX_RATE>            Maximum chunks to process [default: 7]
  -p, --pixelation <PIXELATION>        Pixelation rate [default: 10]
  -l, --line-shift <LINE_SHIFT>        Line shift rate [default: 0.3]
  -r, --reverse <REVERSE>              Reverse rate [default: 0.3]
  -f, --flip <FLIP>                    Flip rate [default: 0.3]
  -c, --channel-swap <CHANNEL_SWAP>    Channel swap rate [default: 0.3]
  -t, --channel-shift <CHANNEL_SHIFT>  Channel shift rate [default: 0.3]
  -s, --seed <SEED>                    Custom seed
  -b, --batch <BATCH>                  Number of files to output
  -o, --output <OUTPUT>                Output filename
  -h, --help                           Print help (see more with '--help')
  -V, --version                        Print version
```

## `libmosh`
Follow the [example](https://docs.rs/pixelmosh/latest/libmosh/struct.MoshCore.html#example) from [docs.rs](https://docs.rs/pixelmosh/latest/libmosh/).
