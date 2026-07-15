# Rizki Agustiawan | Portfolio

Environmental Engineer | GIS, Remote Sensing & ESG Analytics portfolio built with Rust, WebAssembly, and Leptos Server-Side Rendering (SSR).

## Features

- **Blazing Fast SSR:** Powered by Axum and Leptos for maximum performance and SEO capabilities.
- **God Supreme Max SEO:** Full JSON-LD structured data integration, dynamic Open Graph tags, canonical URLs, and strict semantic HTML routing.
- **Integrated Markdown Blog:** Built-in engine to parse and render `.md` files equipped with YAML frontmatter without requiring a database.
- **Responsive & Accessible (A11y):** Screen-reader friendly with high contrast, semantic layout, and ARIA labels.
- **End-to-End Tested:** Integrated E2E Axum router tests ensuring robust deployment stability.

## Tech Stack

- **Rust** (Backend & Frontend via WebAssembly)
- **Leptos** (Reactive UI Framework)
- **Axum** (Server)
- **pulldown-cmark & serde** (Markdown Engine)

## Development

Prerequisites:
1. Ensure Rust toolchain is installed.
2. Install `cargo-leptos`:
   ```bash
   cargo install cargo-leptos
   ```
3. Add WebAssembly target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

Run locally:
```bash
cargo leptos watch
```
Access at `http://localhost:3000`

## Deployment

To build a production release:
```bash
cargo leptos build --release
```
The compiled server will be available in the `target/release/` directory.

## License
© 2026 Rizki Agustiawan. All rights reserved.
