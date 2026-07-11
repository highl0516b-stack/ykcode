# ykcode — Zero-Code Generation Platform

A revolutionary platform where users build everything from a button to a complete website through tactile drag-and-drop interactions — no code required.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust 1.91.0 |
| Frontend/Fullstack | Leptos 0.8.20 (fine-grained reactive) |
| Backend API | Axum 0.8.9 |
| Async Runtime | Tokio 1.52 |
| Storage | SQLite (sqlx), Fjall 3.x, Tonbo 0.3.x |
| WebAssembly | Wasmer 7.x, wasm-bindgen 0.2 |
| Bundler (SSR) | cargo-leptos 0.3.7 |
| Bundler (CSR) | Trunk 0.21.x |

## Prerequisites

```bash
# Rust toolchain (1.91.0 is pinned via rust-toolchain.toml)
rustup show

# WebAssembly target
rustup target add wasm32-unknown-unknown

# cargo-leptos (fullstack dev server)
cargo install --locked cargo-leptos@0.3.7

# Optional: wasm-opt (post-build size optimization)
sudo apt-get install binaryen   # Linux
brew install binaryen            # macOS
```

## Development

### Fullstack SSR + Hydration (recommended)

```bash
# Dev server with hot-reload
cargo leptos watch

# Production build
cargo leptos build --release

# Server runs on http://127.0.0.1:3000
```

### CSR-only (Trunk)

```bash
cd app
trunk serve --features csr --open
# Dev server on http://127.0.0.1:8080
```

## Quality Gates

Run before every commit:

```bash
cargo fmt --all
cargo clippy --workspace -- -D warnings
cargo clippy -p app --features ssr -- -D warnings
cargo test --workspace
```

## Workspace Structure

```
ykcode/
├── Cargo.toml          # Workspace root + cargo-leptos metadata
├── rust-toolchain.toml # Rust 1.91.0 pinned
├── .cargo/config.toml  # wasm32 rustflags
├── app/                # Leptos fullstack UI
│   ├── src/
│   │   ├── lib.rs      # App, shell(), hydrate()
│   │   └── components/ # EditorShell, Canvas, Palette, Properties…
│   └── Trunk.toml      # CSR-only builds
├── server/             # Axum SSR host
├── shared/             # Domain types (Project, Artboard, CanvasComponent…)
├── style/main.scss     # "Chromatic Precision" design system
└── public/             # Static assets
```

## License

MIT OR Apache-2.0
