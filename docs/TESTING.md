# Testing Plan for tempo-mcp-rs

## Overview

This document outlines the testing strategy for the Tempo MCP server.

## Test Categories

### 1. Unit Tests

Test individual functions in isolation (28 tests):

| Module | Function | Test Cases |
|--------|----------|------------|
| `tools` | `resolve_token()` | Valid symbols, addresses, unknown tokens |
| `tools` | `parse_private_key()` | With/without 0x prefix, invalid lengths |
| `tempo` | `parse_token_amount()` | Integer, decimal, empty, invalid, multiple dots |
| `tempo` | `format_token_amount()` | Various decimal places |
| `tempo` | `get_token_by_*()` | Symbol lookup, address lookup |
| `tempo` | `sign_transaction()` | Transaction signing |

### 2. Integration Tests

Test MCP protocol compliance (4 tests):

| Test | Description |
|------|-------------|
| `test_mcp_initialize` | Verify `initialize` response format |
| `test_mcp_list_tools` | Verify `tools/list` returns all 8 tools |
| `test_mcp_tool_list_tokens` | Verify `tempo_list_tokens` tool execution |
| `test_mcp_tool_get_balance` | Verify `tempo_get_balance` with live RPC |

### 3. Live RPC Tests

Test actual blockchain connectivity (2 tests):

| Test | Description |
|------|-------------|
| `test_get_balance_live` | Query balance from Tempo testnet |
| `test_get_gas_price_live` | Query gas price from Tempo testnet |

### 4. E2E Tests with Claude

Test the full loop: Claude -> MCP -> Tempo -> Response

## Test Implementation

### Unit Tests Location
```
src/
├── tempo.rs      # #[cfg(test)] mod tests (17 tests)
└── tools/
    └── mod.rs    # #[cfg(test)] mod tests (11 tests)
```

### Integration Tests Location
```
tests/
└── mcp_protocol.rs   # MCP message format tests (4 tests)
```

### Test Scripts
```
scripts/
├── test-mcp.sh          # Manual MCP protocol testing
└── test-with-claude.py  # Automated Claude API testing
```

## Running Tests

```bash
# Unit tests only (fast, no network)
cargo test

# All tests including integration tests
cargo test -- --ignored --test-threads=1

# Integration tests only
cargo test --test mcp_protocol -- --ignored --test-threads=1

# Live RPC tests only
cargo test test_live -- --ignored

# With logging
RUST_LOG=debug cargo test

# Manual MCP protocol test
./scripts/test-mcp.sh

# Claude API integration test
export ANTHROPIC_API_KEY=your_key
python scripts/test-with-claude.py
```

## Mock vs Live Testing

- **Unit tests**: No network required, fast
- **Integration tests**: Spawn MCP server, test protocol compliance
- **Live RPC tests**: Require Tempo testnet connectivity (marked `#[ignore]`)
- **CI**: Run unit tests only, integration tests on demand

## Claude Desktop Integration

1. Build the server:
```bash
cargo build --release
```

2. Add to Claude Desktop config (`~/.config/claude/claude_desktop_config.json`):
```json
{
  "mcpServers": {
    "tempo": {
      "command": "/path/to/tempo-mcp-rs/target/release/tempo-mcp"
    }
  }
}
```

3. Restart Claude Desktop

4. Test by asking:
   - "What tokens are available on Tempo?"
   - "Check TUSD balance for address 0x..."
   - "Get a quote to swap 100 TUSD to TEUR"

## Claude API Integration Test

The `scripts/test-with-claude.py` script:
1. Spawns tempo-mcp server
2. Connects to Claude API
3. Sends test prompts
4. Verifies correct tools are called
5. Executes tools and shows results

Requirements:
```bash
pip install anthropic
export ANTHROPIC_API_KEY=your_key
```

## Coverage Summary

| Category | Count | Status |
|----------|-------|--------|
| Unit tests | 28 | All passing |
| Integration tests | 4 | All passing |
| Live RPC tests | 2 | All passing |
| Test scripts | 2 | Available |
