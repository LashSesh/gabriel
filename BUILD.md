# Build & Installation Guide

## Voraussetzungen

### Rust Toolchain

```bash
# Installation von Rust (stabil)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Nach Installation: Reload Shell
source $HOME/.cargo/env

# Verifizierung
rustc --version
cargo --version
```

Mindestversion: **Rust 1.70+**

---

## Build-Prozess

### 1. Debug Build (schneller kompilieren)

```bash
cd gabriel-organism
cargo build
```

Binary: `target/debug/gabriel-organism`

### 2. Release Build (optimiert für Performance)

```bash
cargo build --release
```

Binary: `target/release/gabriel-organism`

**Empfohlen für Production!**

### 3. Build-Optionen

```bash
# Mit allen Optimierungen
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Parallel Build (nutzt alle CPU Cores)
cargo build --release -j $(nproc)

# Mit LTO (Link Time Optimization)
cargo rustc --release -- -C lto=fat
```

---

## Tests

### Unit Tests

```bash
# Alle Tests
cargo test

# Einzelnes Crate
cargo test -p gabriel-core
cargo test -p trichter-geometry
cargo test -p metabolics
cargo test -p emergence
cargo test -p organism

# Mit Output
cargo test -- --nocapture

# Verbose
cargo test -- --test-threads=1 --nocapture
```

### Integration Tests

```bash
# Falls vorhanden
cargo test --test '*'
```

---

## Benchmarks

```bash
# Alle Benchmarks
cargo bench

# Spezifisches Benchmark
cargo bench --bench gabriel_cell_bench
```

---

## Dokumentation

### Rust Docs generieren

```bash
cargo doc --open --no-deps
```

Öffnet automatisch die vollständige API-Dokumentation im Browser.

### Für einzelne Crates

```bash
cargo doc -p gabriel-core --open
```

---

## Ausführen

### Direkt mit Cargo

```bash
# Standard (100 Zyklen, 50 Quanten)
cargo run --release

# Custom Parameter
cargo run --release -- --cycles 500 --feed-count 100 --radius 2.0

# Mit Verbose Logging
cargo run --release -- --cycles 100 --verbose

# Export zu JSON
cargo run --release -- --cycles 100 --output state.json
```

### Binary direkt

```bash
# Nach Build
./target/release/gabriel-organism --help
./target/release/gabriel-organism --cycles 100
```

---

## Als Library verwenden

### In Cargo.toml

```toml
[dependencies]
gabriel-core = { path = "path/to/gabriel-organism/gabriel-core" }
trichter-geometry = { path = "path/to/gabriel-organism/trichter-geometry" }
metabolics = { path = "path/to/gabriel-organism/metabolics" }
emergence = { path = "path/to/gabriel-organism/emergence" }
organism = { path = "path/to/gabriel-organism/organism" }
```

Oder bei Publication auf crates.io:

```toml
[dependencies]
gabriel-core = "0.1"
organism = "0.1"
```

---

## Optimierungen

### Cargo.toml Profile

Bereits konfiguriert in Workspace:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

### CPU-spezifische Optimierungen

```bash
# Für native CPU
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Für maximale Performance
RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo build --release
```

---

## Troubleshooting

### Problem: Compilation Error

```bash
# Clean und rebuild
cargo clean
cargo build --release
```

### Problem: Link Errors

```bash
# Update Rust
rustup update stable

# Update Dependencies
cargo update
```

### Problem: Out of Memory während Build

```bash
# Weniger parallele Jobs
cargo build --release -j 2

# Oder sequenziell
cargo build --release -j 1
```

### Problem: Tests schlagen fehl

```bash
# Einzeln debuggen
cargo test --lib
cargo test --bins
cargo test --tests

# Mit Backtrace
RUST_BACKTRACE=1 cargo test
```

---

## Performance Profiling

### Flamegraph

```bash
# Installation
cargo install flamegraph

# Profiling
cargo flamegraph --bin gabriel-organism -- --cycles 1000
```

### Criterion Benchmarks

```bash
# Detaillierte Benchmarks
cargo bench -- --verbose

# Vergleich zwischen Versionen
cargo bench -- --save-baseline main
# ... Änderungen ...
cargo bench -- --baseline main
```

---

## Cross-Compilation

### Für andere Targets

```bash
# Linux -> Windows
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

# Linux -> macOS (komplizierter, benötigt osxcross)
rustup target add x86_64-apple-darwin
```

---

## CI/CD Integration

### GitHub Actions Beispiel

```yaml
name: Build and Test

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build
        run: cargo build --release --verbose
      - name: Run tests
        run: cargo test --verbose
```

---

## Packaging

### Debian Package

```bash
# Installation cargo-deb
cargo install cargo-deb

# Package erstellen
cargo deb -p organism
```

### Archive erstellen

```bash
# Nach Build
cd target/release
tar czf gabriel-organism-v0.1.0-linux-x86_64.tar.gz gabriel-organism
```

---

## Installation System-weit

```bash
# Nach Build
sudo cp target/release/gabriel-organism /usr/local/bin/

# Oder mit Cargo
cargo install --path organism
```

---

## Entwicklung

### Format Code

```bash
cargo fmt
```

### Linting

```bash
cargo clippy -- -D warnings
```

### Watch Mode (auto-rebuild)

```bash
# Installation
cargo install cargo-watch

# Auto-rebuild on changes
cargo watch -x build
cargo watch -x test
cargo watch -x run
```

---

## Ressourcen

- **Rust Book**: https://doc.rust-lang.org/book/
- **Cargo Book**: https://doc.rust-lang.org/cargo/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/

---

**Bei Problemen**: Issue auf GitHub erstellen oder Dokumentation konsultieren.
