# ZSA Voyager Matrix

Matrix rain animation for ZSA Voyager keyboard LEDs, controlled via the [kontroll](https://github.com/zsa/kontroll) Keymapp API.

![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)

## Overview

A lightweight, smooth Matrix-style waterfall animation that runs directly on your ZSA Voyager's RGB LEDs. Designed for use as a tmux screensaver companion, but works standalone too.

## Prerequisites

- ZSA Voyager keyboard (or any ZSA keyboard supported by kontroll)
- [Keymapp](https://www.zsa.io/voyager) v1.3.2+ running with API enabled
- Linux with a Rust toolchain (or Nix)

## Installation

### Via Nix (recommended)

```bash
nix run github:martelo11/zsa-voyager-matrix
```

### From source

```bash
git clone https://github.com/martelo11/zsa-voyager-matrix
cd zsa-voyager-matrix
cargo build --release
./target/release/zsa-voyager-matrix
```

## Usage

```bash
zsa-voyager-matrix [OPTIONS]
```

### Options

| Flag | Default | Description |
|------|---------|-------------|
| `-c, --color` | `#69c11d` | LED color in hex format |
| `-f, --fps` | `20` | Animation frames per second |
| `-d, --drops` | `6` | Number of concurrent rain drops |

### Examples

```bash
# Default green matrix rain
zsa-voyager-matrix

# Classic Matrix green at 30 FPS
zsa-voyager-matrix --color #00ff00 --fps 30

# Sparse, slow red rain
zsa-voyager-matrix --color #ff0000 --fps 10 --drops 3

# Dense cyan waterfall
zsa-voyager-matrix --color #00ffff --fps 25 --drops 10
```

## Tmux Integration

Use with [tmux-cmatrix-screensaver](https://github.com/martelo/nixos-rised-workhorses) for a synchronized terminal + keyboard screensaver:

```bash
set -g lock-after-time 300
set -g lock-command "tmux-cmatrix-screensaver"
```

## How It Works

- Connects to Keymapp via Unix domain socket (`~/.config/.keymapp/keymapp.sock`)
- Animates 6 concurrent "rain drops" falling down a 5×12 LED grid
- Each drop is 2–4 LEDs long, moving at randomized speeds
- Only changed LEDs are updated each frame (~10–15 gRPC calls at 20 FPS)
- Restores keyboard LEDs to default on exit (SIGTERM/SIGINT)
- Silent failure if Keymapp is unavailable (exits 0)

## Performance

- ~1–2% CPU on a modern desktop
- ~10–15 gRPC calls per frame (local Unix socket, <1 ms latency)
- No busy-waiting; precise `tokio::time::interval` scheduling

## License

MIT — see [LICENSE](LICENSE).
