# ZSA Voyager Matrix

Matrix rain animation for ZSA Voyager keyboard LEDs, controlled via the [kontroll](https://github.com/zsa/kontroll) Keymapp API.

![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)

## Overview

A lightweight, smooth Matrix-style waterfall animation that runs directly on your ZSA Voyager's RGB LEDs.

## Prerequisites

- **ZSA Voyager** keyboard (see [Keyboard Compatibility](#keyboard-compatibility) below)
- **[Keymapp](https://www.zsa.io/voyager) v1.3.2+** running with the **API enabled** in settings
- Linux with a Rust toolchain (or Nix)

> **Note:** Keymapp must be running and the API must be active *before* starting the animation. See the [kontroll announcement blog post](https://blog.zsa.io/introducing-kontroll/) for details on the Keymapp API.

## Keyboard Compatibility

| Keyboard | Status | Notes |
|----------|--------|-------|
| **ZSA Voyager** | ✅ Native | Designed for the Voyager 5×12 LED grid |
| **Moonlander** | ⚠️ Adoptable | Uses the same RGB API, but the 5×12 grid mapping in `src/main.rs` would need adjustment for the Moonlander's layout |
| **ErgoDox EZ** | ❌ Not supported | Kontroll cannot control RGB LEDs on the ErgoDox EZ ([see upstream docs](https://github.com/zsa/kontroll)) |

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
| `-d, --drops` | `10` | Number of concurrent rain drops |

### Examples

```bash
# Default green matrix rain
zsa-voyager-matrix

# Classic Matrix green at 30 FPS
zsa-voyager-matrix --color "#00ff00" --fps 30

# Sparse, slow red rain
zsa-voyager-matrix --color "#ff0000" --fps 10 --drops 3

# Dense cyan waterfall
zsa-voyager-matrix --color "#00ffff" --fps 25 --drops 10
```

> **Note:** Always quote hex colors (e.g., `"#ff0000"`) — the `#` character starts a shell comment otherwise.

## How It Works

- Connects to Keymapp via Unix domain socket (`~/.config/.keymapp/keymapp.sock`)
- Animates 10 concurrent "rain drops" falling down a 5×12 LED grid
- Each drop is 3–5 LEDs long with a brightness gradient (head bright, tail fades to dim)
- 15% of drops get a "sparkle" effect — a brighter leading LED
- Only changed LEDs are updated each frame (~15–20 gRPC calls at 20 FPS)
- Restores keyboard LEDs to default on exit (SIGTERM/SIGINT)
- Exits silently if Keymapp is unavailable (exits 0)

## Performance

- ~1–2% CPU on a modern desktop
- ~10–15 gRPC calls per frame (local Unix socket, <1 ms latency)
- No busy-waiting; precise `tokio::time::interval` scheduling

## License

MIT — see [LICENSE](LICENSE).
