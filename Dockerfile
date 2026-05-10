# ============================================
# rizkiagustiawan.tech — Multi-stage Dockerfile
# Leptos SSR + Axum + WASM
# ============================================

# --- Stage 1: Builder ---
FROM rust:1-bookworm AS builder

# Install WASM target
RUN rustup target add wasm32-unknown-unknown

# Install cargo-leptos
RUN cargo install cargo-leptos

WORKDIR /app

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock* ./
COPY .cargo .cargo

# Create dummy source for dependency pre-build
RUN mkdir -p src/components && \
    echo 'pub mod app; pub mod components;' > src/lib.rs && \
    echo 'fn main() {}' > src/main.rs && \
    echo 'use leptos::prelude::*; pub fn shell(_: leptos::prelude::LeptosOptions) -> impl IntoView { view! { <p>"build"</p> } } #[leptos::prelude::component] pub fn App() -> impl IntoView { view! { <p>"build"</p> } }' > src/app.rs && \
    echo '' > src/components/mod.rs

# Pre-build dependencies
RUN cargo leptos build --release 2>/dev/null || true

# Copy actual source
COPY src src
COPY style style
COPY public public

# Touch source files to invalidate cache
RUN touch src/main.rs src/lib.rs src/app.rs

# Full release build
RUN cargo leptos build --release

# --- Stage 2: Runtime ---
FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the server binary
COPY --from=builder /app/target/server/release/rizkiagustiawan-tech ./server

# Copy the site assets (JS/WASM/CSS/public)
COPY --from=builder /app/target/site ./site

ENV LEPTOS_SITE_ROOT=site
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000

EXPOSE 3000

CMD ["./server"]
