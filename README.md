# pixelmosh
[![Crates.io](https://img.shields.io/crates/v/pixelmosh)](https://crates.io/crates/pixelmosh)
[![Tests](https://github.com/charlesrocket/pixelmosh/actions/workflows/tests.yml/badge.svg?branch=master)](https://github.com/charlesrocket/pixelmosh/actions/workflows/tests.yml)
[![codecov](https://codecov.io/gh/charlesrocket/pixelmosh/branch/master/graph/badge.svg)](https://codecov.io/gh/charlesrocket/pixelmosh)

### Compilation

```shell
make
sudo make install
```

### Usage

```
┌─────────────────────────────────────┐
│ █▀▄ █ ▀▄▀ ██▀ █   █▄ ▄█ ▄▀▄ ▄▀▀ █▄█ │
│ █▀  █ █ █ █▄▄ █▄▄ █ ▀ █ ▀▄▀ ▄██ █ █ │
└─────────────────────────────────────┘
PNG corrupter

USAGE:
    pixelmosh [OPTIONS] <FILE>

ARGS:
    <FILE>

OPTIONS:
    -m, --min-rate <MIN_RATE>              [default: 1]
    -n, --max-rate <MAX_RATE>              [default: 7]
    -p, --pixelation <PIXELATION>          [default: 10]
    -l, --line-shift <LINE_SHIFT>          [default: 0.3]
    -r, --reverse <REVERSE>                [default: 0.3]
    -f, --flip <FLIP>                      [default: 0.3]
    -c, --channel-swap <CHANNEL_SWAP>      [default: 0.3]
    -t, --channel-shift <CHANNEL_SHIFT>    [default: 0.3]
    -s, --seed <SEED>
    -o, --output <OUTPUT>
    -h, --help                             Print help information
    -V, --version                          Print version information
```
