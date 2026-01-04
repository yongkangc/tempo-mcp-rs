# Build stage for tempo-mcp
FROM rust:1.85-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build release binary
RUN cargo build --release

# Install mcp-proxy for HTTP transport
RUN cargo install mcp-proxy

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy tempo-mcp binary
COPY --from=builder /app/target/release/tempo-mcp /usr/local/bin/tempo-mcp

# Copy mcp-proxy binary
COPY --from=builder /usr/local/cargo/bin/mcp-proxy /usr/local/bin/mcp-proxy

ENV PORT=8000
EXPOSE 8000

# Use mcp-proxy to bridge stdio to HTTP
# mcp-proxy runs the stdio server and exposes it via SSE/HTTP
# --host 0.0.0.0 needed for Docker container networking
ENTRYPOINT ["sh", "-c", "mcp-proxy --host 0.0.0.0 --port=$PORT tempo-mcp"]
