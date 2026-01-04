# Development Guide

## Prerequisites

- Rust 1.75+ (stable)
- Rust nightly (for formatting and linting)

```bash
rustup install stable
rustup install nightly
```

## Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

## Running

```bash
# Run the MCP server (connects via stdio)
cargo run

# Run release build
./target/release/tempo-mcp
```

## Testing

### Unit Tests

Fast tests that don't require network:

```bash
cargo test
```

### Integration Tests

Tests that spawn the MCP server and verify protocol compliance:

```bash
cargo test --test mcp_protocol -- --ignored --test-threads=1
```

### Live RPC Tests

Tests that connect to Tempo testnet:

```bash
cargo test -- --ignored
```

### All Tests

```bash
# Run everything
cargo test -- --ignored --test-threads=1
```

## Code Quality

### Formatting

```bash
# Check formatting
cargo +nightly fmt --check

# Apply formatting
cargo +nightly fmt
```

### Linting

```bash
cargo +nightly clippy --all-targets --all-features -- -D warnings
```

### Pre-commit Checks

Run before committing:

```bash
cargo +nightly fmt
cargo +nightly clippy --all-targets --all-features -- -D warnings
cargo test
```

## Project Structure

```
tempo-mcp-rs/
├── src/
│   ├── main.rs           # MCP server entry point
│   ├── lib.rs            # Library exports
│   ├── tempo.rs          # Tempo blockchain client
│   └── tools/
│       └── mod.rs        # Tool implementations
├── tests/
│   └── mcp_protocol.rs   # Integration tests
├── scripts/
│   ├── test-mcp.sh       # Manual MCP testing
│   └── test-with-claude.py  # Claude API testing
├── docs/
│   ├── DEVELOPMENT.md    # This file
│   ├── TESTING.md        # Testing documentation
│   └── IMPROVEMENT_PLAN.md  # Design improvements
└── examples/
    └── claude-desktop-config.json
```

## Architecture

### MCP Server (main.rs)

- Uses `rmcp` crate for MCP protocol
- Implements 8 tools via `#[tool]` macro
- Single `TempoService` struct handles all tool calls

### Tempo Client (tempo.rs)

- HTTP client for Tempo testnet RPC
- Token management (TUSD, TEUR, TGBP)
- Transaction signing (EIP-155 legacy)
- Contract interaction helpers

### Tools (tools/mod.rs)

- Input validation and parsing
- Token resolution (symbol or address)
- Amount formatting (human-readable)
- Response formatting

## Network Configuration

| Setting | Value |
|---------|-------|
| Network | Tempo Testnet |
| Chain ID | 62320 |
| RPC URL | https://rpc.testnet.tempo.xyz |
| Explorer | https://explore.tempo.xyz |

## Debugging

### Enable Logging

```bash
RUST_LOG=debug cargo run
```

### Manual MCP Testing

```bash
./scripts/test-mcp.sh
```

### Test with Claude API

```bash
export ANTHROPIC_API_KEY=your_key
python scripts/test-with-claude.py
```

## CI/CD

GitHub Actions runs on every push/PR:

1. **Format** - Check code formatting
2. **Clippy** - Lint for issues
3. **Test** - Run unit tests
4. **Build** - Build release binary

See `.github/workflows/ci.yml` for details.
