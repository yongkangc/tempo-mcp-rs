# tempo-mcp-rs Modernization Plan

## Goal

Modernize the MCP implementation to:
1. Follow MCP best practices
2. Simplify the codebase
3. Improve performance

## Phases

### Phase 1: Code Simplification (No Dependency Changes)

Focus on making the existing code cleaner and more maintainable.

#### 1.1 Consolidate Response Formatting
**Current**: Each tool formats its own string response
**Target**: Return structured data, centralize formatting

```rust
// Before: String formatting in each tool
Ok(format!(
    "Balance for {}:\n{} {} ({} raw units)",
    input.address, formatted, token_info.symbol, balance
))

// After: Structured response
struct BalanceResult {
    address: String,
    amount: String,
    symbol: String,
    raw: String,
}
```

#### 1.2 Simplify ABI Encoding
**Current**: Manual byte manipulation for contract calls
**Target**: Use helper functions or consider alloy-sol-types

```rust
// Before: Manual ABI encoding
let mut data = vec![0xa9, 0x05, 0x9c, 0xbb];
data.extend_from_slice(&[0u8; 12]);
data.extend_from_slice(to.as_slice());
data.extend_from_slice(&amount.to_be_bytes::<32>());

// After: Helper function
fn encode_transfer(to: Address, amount: U256) -> Vec<u8> {
    abi_encode(TRANSFER_SELECTOR, &[to.into(), amount])
}
```

#### 1.3 Configuration via Environment Variables
**Current**: Hardcoded RPC URL and chain ID
**Target**: Configurable via env vars with defaults

```rust
// After
fn rpc_url() -> &'static str {
    static URL: OnceLock<String> = OnceLock::new();
    URL.get_or_init(|| {
        std::env::var("TEMPO_RPC_URL")
            .unwrap_or_else(|_| "https://rpc.testnet.tempo.xyz".to_string())
    })
}
```

#### 1.4 Add Request Timeouts
**Current**: No timeout on HTTP requests
**Target**: Configurable timeout with sensible default

```rust
let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(30))
    .build()?;
```

### Phase 2: Error Handling Improvements

#### 2.1 Typed Error Handling
**Current**: Using `anyhow::Error` everywhere
**Target**: Domain-specific error types

```rust
#[derive(Debug, thiserror::Error)]
pub enum TempoError {
    #[error("RPC error {code}: {message}")]
    Rpc { code: i64, message: String },

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Invalid amount: {0}")]
    InvalidAmount(String),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    #[error("Token not found: {0}")]
    TokenNotFound(String),
}
```

#### 2.2 Structured Error Responses
Map errors to appropriate MCP error codes.

### Phase 3: Performance Optimizations

#### 3.1 HTTP Client Pooling
**Current**: Using default reqwest client
**Target**: Configure connection pool

```rust
let client = reqwest::Client::builder()
    .pool_max_idle_per_host(5)
    .pool_idle_timeout(Duration::from_secs(60))
    .build()?;
```

#### 3.2 Pre-allocate Buffers
For RLP encoding, pre-allocate with estimated size.

### Phase 4: Dependency Updates (Separate PR)

**Risk**: Breaking changes require testing

| Dependency | Current | Target | Risk |
|------------|---------|--------|------|
| rmcp | 0.1.5 | 0.12.0 | High - API changes |
| alloy-primitives | 0.8 | 1.x | High - API changes |
| schemars | 0.8 | 1.x | Medium |
| reqwest | 0.12 | 0.13 | Low |

**Recommendation**: Do dependency updates as a separate PR with thorough testing.

---

## Implementation Priority

### High Priority (Do Now)
1. Add request timeouts
2. Make RPC URL configurable
3. Simplify ABI encoding with helpers

### Medium Priority
1. Typed error handling
2. HTTP client pooling
3. Pre-allocate buffers

### Low Priority (Future)
1. Update to rmcp 0.12
2. Update alloy-primitives to 1.x
3. Structured JSON responses

---

## Success Criteria

1. **Simpler**: Fewer lines of code, less duplication
2. **Performant**: Maintain or improve current performance
3. **Modern**: Follow MCP 2025 best practices
4. **Maintainable**: Clear error handling, configurable

---

## Non-Goals

- Adding new tools
- Changing the MCP interface
- Supporting mainnet (yet)
