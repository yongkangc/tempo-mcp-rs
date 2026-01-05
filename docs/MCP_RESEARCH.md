# MCP Best Practices Research

## Overview

This document summarizes best practices for Model Context Protocol (MCP) server implementation, with focus on Rust-specific patterns using the `rmcp` crate.

## Sources

- [MCP Best Practices Architecture Guide](https://modelcontextprotocol.info/docs/best-practices/)
- [MCP Specification 2025-11-25](https://modelcontextprotocol.io/specification/2025-11-25)
- [Official Rust SDK (modelcontextprotocol/rust-sdk)](https://github.com/modelcontextprotocol/rust-sdk)
- [rmcp Crate Documentation](https://docs.rs/rmcp/latest/rmcp/)
- [Shuttle MCP Server Guide](https://www.shuttle.dev/blog/2025/07/18/how-to-build-a-stdio-mcp-server-in-rust)

---

## 1. Architectural Principles

### Single Responsibility
Each MCP server should have **one clear, well-defined purpose**. Benefits:
- Improved maintainability
- Independent scaling
- Prevents cascading failures

### Defense in Depth
Layer multiple security controls:
- Network isolation
- Authentication/Authorization
- Input validation
- Output sanitization

### Fail-Safe Design
- Circuit breakers
- Caching
- Rate limiting
- Return safe defaults on failure

---

## 2. Tool Design Best Practices

### Clear Descriptions
- Annotate tools with meaningful descriptions
- Help AI clients understand when to invoke each tool
- Use `#[schemars(description = "...")]` for parameter documentation

### Type-Safe Parameters
```rust
#[derive(Deserialize, JsonSchema)]
pub struct GetBalanceInput {
    #[schemars(description = "Wallet address to check")]
    pub address: String,
}
```

### Focused Toolsets
- Avoid mapping every API endpoint to a new tool
- Group related tasks into higher-level functions
- Keep tool count manageable (each tool has a cost)

### Security Requirements
From MCP Spec:
> "Tools represent arbitrary code execution and must be treated with appropriate caution."
> "Tool descriptions should be considered untrusted unless obtained from a trusted server."

---

## 3. Error Handling

### Classification
Classify errors by category:
- **CLIENT_ERROR (4xx)**: Invalid input, missing parameters
- **SERVER_ERROR (5xx)**: Internal failures
- **EXTERNAL_ERROR (502/503)**: Dependency failures

### Structured Responses
- Provide error codes matching JSON-RPC standards
- Include retry guidance where appropriate
- Log unexpected errors with full context

### Protocol Compliance
Return `Result<CallToolResult, McpError>` consistently. Use `map_err()` to convert external errors to protocol errors.

---

## 4. Performance Optimization

### Async/Await
- Use tokio for non-blocking operations
- Parallelize independent operations with `tokio::try_join!`

### Connection Pooling
- Configure HTTP client pool for connection reuse
- Reuse single client instance across requests

### Caching
- Multi-level caching (in-memory, persistent)
- Check caches before expensive queries

### Minimal Allocations
- Use static data where possible (`&'static str`, `const`)
- Pre-size buffers for known lengths
- Avoid unnecessary clones

### Performance Targets
- Throughput: >1000 req/s per instance
- P95 latency: <100ms for simple operations
- P99 latency: <500ms for complex operations
- Error rate: <0.1% under normal conditions

---

## 5. rmcp SDK Best Practices (v0.12.0)

### Current Version
The official SDK is at version **0.12.0** (December 2025). Our codebase uses **0.1.5** - should evaluate upgrade.

### Tool Macros
```rust
#[tool(tool_box)]
impl TempoService {
    #[tool(description = "Get balance for address")]
    async fn tempo_get_balance(
        &self,
        #[tool(param)]
        #[schemars(description = "Wallet address")]
        address: String,
    ) -> Result<CallToolResult, rmcp::Error> {
        // ...
    }
}
```

### ServerHandler Pattern
```rust
#[tool(tool_box)]
impl ServerHandler for TempoService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("...".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
```

### Transport Support
- `transport-io`: stdio for servers (current)
- `transport-sse-server`: SSE for HTTP servers
- Consider supporting both for flexibility

### State Management
Use `Arc` for shared state across async tool calls:
```rust
static CLIENT: OnceCell<TempoClient> = OnceCell::const_new();
```

---

## 6. Security Best Practices

### User Consent (from MCP Spec)
- Users must explicitly consent to data access
- Clear UIs for reviewing/authorizing activities
- Users retain control over shared data

### Private Key Handling
- Use environment variables (`TEMPO_PRIVATE_KEY`)
- Never log or expose keys in responses
- Keys should not appear in MCP transcripts

### Input Validation
- Validate all user inputs at boundaries
- Parse addresses before use
- Validate amount formats

---

## 7. Configuration Management

### Externalize Configuration
- Use environment variables
- Support environment-specific overrides
- Validate configuration on startup

### Current Implementation
```rust
pub const TEMPO_TESTNET_RPC: &str = "https://rpc.testnet.tempo.xyz";
pub const TEMPO_TESTNET_CHAIN_ID: u64 = 42429;
```

Consider making configurable via env vars.

---

## 8. Testing Strategy

### Multi-Layer Approach
- **Unit tests**: Individual component validation
- **Integration tests**: Component interactions
- **Contract tests**: MCP protocol compliance
- **Load tests**: Concurrent traffic handling

### Current Coverage
- 28 unit tests
- 4 integration tests
- Protocol compliance tested via `mcp_protocol.rs`

---

## 9. Recommendations for tempo-mcp-rs

### High Priority

| Item | Rationale |
|------|-----------|
| Upgrade rmcp to 0.12.0 | Access latest features, bug fixes |
| Add structured error types | Replace anyhow with typed errors |
| Make RPC URL configurable | Enable mainnet/custom networks |
| Add request timeouts | Prevent hanging on slow RPCs |

### Medium Priority

| Item | Rationale |
|------|-----------|
| Add structured JSON responses | Better for tool chaining |
| Implement connection pooling | HTTP client reuse |
| Add health check/ping support | Protocol compliance |
| Consider SSE transport | Enable HTTP-based deployment |

### Low Priority

| Item | Rationale |
|------|-----------|
| Add logging infrastructure | Observability |
| Add metrics collection | Performance monitoring |
| Add cancellation support | Long-running operations |

---

## 10. Code Simplification Opportunities

### Current Pain Points

1. **Verbose tool implementations**: Each tool has similar boilerplate
2. **String formatting in responses**: Could use structured types
3. **Manual ABI encoding**: Could use `alloy` crate's encoder
4. **Separate input types per tool**: Could consolidate

### Simplification Ideas

1. **Use derive macros more**: Let rmcp generate more boilerplate
2. **Structured tool results**: Return typed data, format in caller
3. **alloy contract bindings**: Auto-generate ABI encoding
4. **Builder patterns**: For complex operations like swap

---

## Conclusion

The current implementation follows many best practices:
- Clear tool descriptions
- Type-safe parameters with schemars
- Parallel RPC calls with try_join!
- Static token list
- Secure private key handling

Key improvements:
1. Upgrade rmcp SDK
2. Add structured error handling
3. Make configuration more flexible
4. Simplify ABI encoding
