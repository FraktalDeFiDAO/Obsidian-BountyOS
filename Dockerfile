FROM rust:1.85 AS builder

WORKDIR /build

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Clean any corrupted cache and build
RUN cargo clean || true && \
    rm -rf /usr/local/cargo/registry/index/* || true && \
    rm -rf /usr/local/cargo/registry/cache/* || true && \
    cargo build --release --locked || cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /build/target/release/obsidian-bounty-finder /usr/local/bin/

RUN useradd -m -u 1000 appuser

USER appuser

EXPOSE 8080

CMD ["obsidian-bounty-finder", "serve", "--host", "0.0.0.0", "--port", "8080"]
