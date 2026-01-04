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

# Install rmcp-proxy for HTTP transport
RUN cargo install rmcp-proxy

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy tempo-mcp binary
COPY --from=builder /app/target/release/tempo-mcp /usr/local/bin/tempo-mcp

# Copy rmcp-proxy binary
COPY --from=builder /usr/local/cargo/bin/rmcp-proxy /usr/local/bin/rmcp-proxy

ENV PORT=8000
EXPOSE 8000

# Use rmcp-proxy to bridge stdio to HTTP
# rmcp-proxy runs the stdio server and exposes it via SSE/HTTP
# --host 0.0.0.0 needed for Docker container networking
ENTRYPOINT ["sh", "-c", "rmcp-proxy --host 0.0.0.0 --port=$PORT -- tempo-mcp"]
