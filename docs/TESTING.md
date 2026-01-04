# Testing Plan for tempo-mcp-rs

## Overview

This document outlines the testing strategy for the Tempo MCP server.

## Test Categories

### 1. Unit Tests

Test individual functions in isolation (22 tests):

| Module | Function | Test Cases |
|--------|----------|------------|
| `tools` | `resolve_token()` | Valid symbols, addresses, unknown tokens |
| `tools` | `parse_private_key()` | With/without 0x prefix, invalid lengths |
| `tempo` | `parse_token_amount()` | Integer, decimal, edge cases |
| `tempo` | `format_token_amount()` | Various decimal places |
| `tempo` | `get_token_by_*()` | Symbol lookup, address lookup |
| `tempo` | `sign_transaction()` | Transaction signing |

### 2. Integration Tests

Test MCP protocol compliance:

| Test | Description |
|------|-------------|
| Server initialization | Verify `initialize` response format |
| Tool listing | Verify `tools/list` returns all 8 tools |
| Tool schemas | Verify JSON schemas are valid |
| Tool execution | Test each tool with mock/real RPC |

### 3. E2E Tests with Claude

Test the full loop: Claude → MCP → Tempo → Response

## Test Implementation

### Unit Tests Location
```
src/
├── tempo.rs      # #[cfg(test)] mod tests (14 tests)
└── tools/
    └── mod.rs    # #[cfg(test)] mod tests (8 tests)
```

### Integration Tests Location
```
tests/
└── mcp_protocol.rs   # MCP message format tests
```

### Test Scripts
```
scripts/
├── test-mcp.sh          # Manual MCP protocol testing
└── test-with-claude.py  # Automated Claude API testing
```

## Running Tests

```bash
# Unit tests (fast, no network required)
cargo test

# All tests including ignored
cargo test -- --include-ignored

# Integration tests only
cargo test --test '*' -- --ignored

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
- **Integration tests**: Can use live testnet (marked `#[ignore]`)
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

## Test Priorities

1. **P0**: Token resolution, amount parsing (core logic) - done
2. **P1**: MCP protocol compliance (server works) - done
3. **P2**: Tool execution with mocks
4. **P3**: Live testnet tests

## Coverage

- 22 unit tests
- 4 integration tests (ignored by default)
- 2 test scripts
