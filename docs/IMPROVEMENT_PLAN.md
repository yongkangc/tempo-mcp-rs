# tempo-mcp-rs Improvement Plan

## Overview

This document consolidates findings from testing coverage analysis, software design review, and performance analysis.

## 1. Testing Gaps

### Current Coverage
- 22 unit tests (11 in tempo.rs, 11 in tools/mod.rs)
- 5 integration tests (all ignored, require network)

### Missing Tests

| Category | Gap | Priority |
|----------|-----|----------|
| RPC Client | No mocked tests for `get_balance`, `get_transaction`, etc. | P1 |
| Error Paths | Limited error case coverage | P1 |
| Event Decoding | No tests for `decode_transaction` log parsing | P2 |
| Tool Handlers | No async tool function tests | P2 |
| Edge Cases | Invalid amounts, malformed inputs | P2 |
| Amount Parsing | No tests for negative numbers, overflow | P3 |

### Recommended Actions
1. Add mocked RPC tests using `mockall` or `wiremock`
2. Add error path tests (invalid addresses, network failures)
3. Add tests for `decode_transaction` event parsing
4. Add edge case tests for amount parsing

## 2. Design Issues

### Critical

| Issue | Location | Impact |
|-------|----------|--------|
| 8x duplicated error handling | main.rs:28-188 | Maintainability |
| Token data stored twice | tempo.rs:12-36, 98-126 | Inconsistency risk |
| Manual ABI encoding | tempo.rs:251-315, 448-498 | Error-prone |
| Leaky transaction abstraction | tempo.rs:374-535 | Complexity |

### Medium

| Issue | Location | Impact |
|-------|----------|--------|
| Mixed error types | Throughout | Confusion |
| Silent parsing failures | tempo.rs:142-156 | Hidden bugs |
| Unnecessary global state | main.rs:16-20 | Testability |

### Fixes

**1. Deduplicate error handling (main.rs)**

```rust
fn to_mcp_result(result: Result<String>) -> Result<CallToolResult, rmcp::Error> {
    match result {
        Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

// Then each tool becomes:
to_mcp_result(get_balance(client, input).await)
```

**2. Single source of truth for tokens (tempo.rs)**

```rust
const KNOWN_TOKENS: &[(Address, &str, u8)] = &[
    (TUSD_ADDRESS, "TUSD", 6),
    (TEUR_ADDRESS, "TEUR", 6),
    (TGBP_ADDRESS, "TGBP", 6),
];
```

**3. Extract transaction submission helper (tempo.rs)**

```rust
async fn send_contract_call(
    &self,
    private_key: &[u8; 32],
    to: Address,
    data: Vec<u8>,
    gas_limit: u64,
) -> Result<B256> {
    let from = Self::get_address_from_private_key(private_key)?;
    let nonce = self.get_nonce(from).await?;
    let gas_price = self.get_gas_price().await?;
    let raw_tx = Self::sign_transaction(private_key, to, U256::ZERO, data, nonce, gas_price, gas_limit)?;
    self.send_raw_transaction(&raw_tx).await
}
```

**4. Fix silent parsing (tempo.rs)**

```rust
// Before:
let whole: U256 = parts[0].parse().unwrap_or(U256::ZERO);

// After:
let whole: U256 = parts[0].parse()
    .map_err(|_| anyhow::anyhow!("Invalid number: {}", parts[0]))?;
```

## 3. Performance Optimizations

### High Impact

| Optimization | Location | Benefit |
|--------------|----------|---------|
| Parallelize nonce/gas_price | tempo.rs:443-445 | ~50% latency reduction |
| Parallelize tx/receipt fetch | tools/mod.rs:136-141 | ~50% latency reduction |

```rust
use tokio::try_join;

let (nonce, gas_price) = try_join!(
    self.get_nonce(from),
    self.get_gas_price()
)?;
```

### Medium Impact

| Optimization | Location | Benefit |
|--------------|----------|---------|
| Static token list | tempo.rs:98-116 | Eliminate allocations |
| Configure HTTP client | tempo.rs:217 | Better connection reuse |
| Use &'static str for rpc_url | tempo.rs:205-218 | Avoid allocation |

### Low Impact (Quick Wins)

| Optimization | Location | Benefit |
|--------------|----------|---------|
| Add #[inline] hints | Various | Better inlining |
| Pre-size RLP buffers | tempo.rs:397, 421 | Avoid reallocations |
| #[cold] on error paths | tools/mod.rs | Better branch prediction |

## 4. Implementation Plan

### Phase 1: Testing (P1)
1. Add mocked RPC client tests
2. Add error handling tests
3. Add edge case tests

### Phase 2: Design Fixes (P1-P2)
1. Extract `to_mcp_result` helper to deduplicate main.rs
2. Fix silent parsing failures in tempo.rs
3. Remove unused thiserror dependency
4. Extract `send_contract_call` helper

### Phase 3: Performance (P2)
1. Add `try_join!` for parallel RPC calls
2. Use static token list
3. Configure HTTP client properly

### Phase 4: Cleanup (P3)
1. Move client to TempoService struct
2. Add #[inline] hints
3. Pre-size buffers

## 5. Estimated Impact

| Change | Lines Changed | Lines Removed |
|--------|---------------|---------------|
| Deduplicate error handling | +5 | -40 |
| Single token source | +10 | -20 |
| Transaction helper | +15 | -30 |
| Parallel RPC | +10 | -5 |
| **Total** | ~+40 | ~-95 |

Net reduction: ~55 lines while adding features.
