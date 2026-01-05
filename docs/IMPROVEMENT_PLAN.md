# tempo-mcp-rs Improvement Plan

## Status: Completed (Phase 1 + Modernization)

All design, performance, and modernization improvements have been implemented.

## Completed Changes

### Phase 1: Original Design Fixes (v0.2.0)

| Fix | Status |
|-----|--------|
| Deduplicate error handling with `to_mcp_result` helper | Done |
| Fix silent parsing failures in `parse_token_amount` | Done |
| Extract `send_contract_call` helper | Done |
| Remove unused `thiserror` dependency | Done |
| Use static token list (`&'static [TokenInfo]`) | Done |

### Phase 1: Original Performance Fixes (v0.2.0)

| Fix | Status |
|-----|--------|
| Parallelize nonce/gas_price with `try_join!` | Done |
| Parallelize tx/receipt fetch with `try_join!` | Done |
| Add `#[inline]` hints to hot functions | Done |

### Phase 2: Modernization (v0.2.2)

| Fix | Status |
|-----|--------|
| Add request timeouts (30s default) | Done |
| Add HTTP client connection pooling | Done |
| Make RPC URL configurable via `TEMPO_RPC_URL` | Done |
| Make chain ID configurable via `TEMPO_CHAIN_ID` | Done |
| Add timeout configuration via `TEMPO_TIMEOUT` | Done |
| Add ABI encoding helpers (selectors module) | Done |
| Simplify contract call encoding | Done |
| Add typed error handling (`TempoError`) | Done |
| Fix explorer URL (explore.tempo.xyz) | Done |

## Impact Summary

| Metric | Before | After |
|--------|--------|-------|
| Error handling duplication | 8 match blocks | 1 helper function |
| Token list allocations | Vec + 3 Strings per call | Zero (static) |
| RPC latency (write ops) | Sequential | Parallel (~50% faster) |
| HTTP client | New client per request | Pooled connections |
| Request timeout | None | 30s (configurable) |
| Configuration | Hardcoded | Env var configurable |
| ABI encoding | Manual byte manipulation | Helper functions |
| Error types | `anyhow::Error` only | Typed `TempoError` |
| Unit tests | 22 | 29 |
| Integration tests | 4 (broken) | 4 (passing) |

## Configuration Options

| Env Variable | Default | Description |
|--------------|---------|-------------|
| `TEMPO_RPC_URL` | `https://rpc.testnet.tempo.xyz` | RPC endpoint |
| `TEMPO_CHAIN_ID` | `42429` | Chain ID for transaction signing |
| `TEMPO_TIMEOUT` | `30` | Request timeout in seconds |
| `TEMPO_PRIVATE_KEY` | None | Private key for write operations |

## Future Opportunities (Low Priority)

| Item | Notes |
|------|-------|
| Upgrade rmcp to 0.12 | Major version, needs API migration |
| Upgrade alloy-primitives to 1.x | Major version, needs testing |
| Add SSE transport | Enable HTTP-based deployment |
| Add structured JSON responses | Better for tool chaining |
| Add logging infrastructure | Observability |
| Add metrics collection | Performance monitoring |
