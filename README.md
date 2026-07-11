# ykcode — Zero-Code Generation Platform

A revolutionary tactile, gesture-based platform for building websites and UI components — with zero code writing.

## Vision

Users build everything — from a single button to an entire deployed website — through intuitive **drag-and-drop**, **pinch-to-zoom**, and **tap** interactions. No AI prompts. No boilerplate. Pure design.

**Learning curve targets:**
- ⏱ **8 minutes** — produce a standard button component
- ⏱ **4 hours** — craft professional nested multi-layer forms
- ⏱ **1–2 weeks** — build and deploy a full website

## Tech Stack

| Layer | Technology | Version |
|-------|-----------|---------|
| Language | Rust | ≥ 1.88.0 |
| Frontend/SSR | Leptos | 0.8.20 |
| Backend API | Axum | 0.8.9 |
| Async runtime | Tokio | 1.52.3 |
| Embedded KV | Fjall | 3.1.6 |
| WASM runtime | Wasmer | 7.x |
| Build tool | cargo-leptos | 0.3.7 |

## Workspace Layout

```
ykcode/
├── crates/
│   ├── ykcode-core/       Domain model: Node, Document, Layout, Style
│   ├── ykcode-storage/    Storage traits + Fjall/SQLite backends
│   ├── ykcode-ui/         Leptos components — the visual editor
│   └── ykcode-server/     Axum REST API routes
├── frontend/              WASM hydration entrypoint
├── backend/               Server binary (SSR + API)
└── style/
    └── main.css           Design system (Obsidian Spectrum)
```

## Design System: Obsidian Spectrum

**Dark, precise workspace** with luminous violet selection and mint spatial guides.

| Token | Value | Use |
|-------|-------|-----|
| Brand | `#9b7bff` | Selection, primary actions |
| Mint | `#42e8c3` | Snap guides, creation |
| Canvas | `#07080d` | Infinite workspace |
| Artboard | `#f8f9fc` | Page surface |
| Panel | `#10121a` | Sidebars |

Fonts: **Manrope** (UI) · **Space Grotesk** (display)

## Development

### Prerequisites

```bash
rustup update stable           # Rust ≥ 1.88
cargo install cargo-leptos@0.3.7
```

### Run dev server (hot reload)

```bash
cargo leptos watch
```

### Quality gate (run before committing)

```bash
cargo check
cargo fmt
cargo clippy -- -D warnings
cargo test
```

### Build for production

```bash
cargo leptos build --release
```

## Architecture Notes

### Storage layer

- **Fjall** — embedded LSM-tree KV for fast local persistence
- **Tonbo** — dispatcher and cache balancer (planned v0.2)
- **SQLite** — relational export and batch writes
- Browser: **OPFS** (Origin Private File System) via WASM
- Mobile: **Wasmer Edge Volumes**

### Reactivity

All editor state is managed via Leptos 0.8 `RwSignal`s shared through `EditorCtx` context. Fine-grained reactive updates ensure only changed parts re-render.

## License

AGPL-3.0-or-later — with a hybrid commercial exception for enterprise deployments.
