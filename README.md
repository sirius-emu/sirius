<div align="center">

<img src="sirius.svg" alt="Sirius" height="50%">

*A highly concurrent retro emulator written in Rust.*

[![Status](https://img.shields.io/badge/status-early%20development-b8e1ff?style=flat-square&labelColor=dff2ff)](#)
[![License](https://img.shields.io/badge/license-AGPL--3.0-c9b8f5?style=flat-square&labelColor=e8ddff)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.78%2B-ffc8a0?style=flat-square&labelColor=ffe5d0)](#)
[![Made with love](https://img.shields.io/badge/made%20with-♡-ffb3c6?style=flat-square&labelColor=ffd6e0)](#)

</div>

---

Sirius is a Habbo hotel emulator targeting the Nitro client.

It's still early. The foundation is solid but everything above that is still being built.

## Getting Started
```bash
git clone https://github.com/sirius-emu/sirius
cd sirius
# Edit the config config/default.toml
cargo run -p sirius-server
```

The server binds to `0.0.0.0:3000` by default. Point your Nitro client at `ws://localhost:3000`.

## Configuration

Configuration is layered TOML with environment variable overrides. The server reads `config/default.toml` first, then
merges an environment-specific file on top (`development.toml`, `production.toml`). Any field can be overridden at runtime:

```bash
SIRIUS_DATABASE__URL=postgres://user:pass@localhost/sirius
SIRIUS_SERVER__PORT=3000
```

The server refuses to start with a missing or invalid configuration.

## Development
```bash
# Run all tests
cargo test --workspace

# Check without building
cargo check --workspace

# Lint
cargo clippy --workspace --all-targets -- -D warnings

# Format
cargo fmt --all
```

Clippy warnings are treated as errors. Please run `cargo fmt` before commiting, it's a small
thing but it keeps diff clean.

## License

AGPL-3.0. See [`LICENSE`](LICENSE).
