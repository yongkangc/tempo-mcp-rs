# tempo-mcp-rs Improvement Plan

## Status: Completed

All design and performance improvements have been implemented.

## Completed Changes

### Design Fixes

| Fix | Status |
|-----|--------|
| Deduplicate error handling with `to_mcp_result` helper | Done |
| Fix silent parsing failures in `parse_token_amount` | Done |
| Extract `send_contract_call` helper | Done |
| Remove unused `thiserror` dependency | Done |
| Use static token list (`&'static [TokenInfo]`) | Done |

### Performance Fixes

| Fix | Status |
|-----|--------|
| Parallelize nonce/gas_price with `try_join!` | Done |
| Parallelize tx/receipt fetch with `try_join!` | Done |
| Add `#[inline]` hints to hot functions | Done |

## Impact Summary

| Metric | Before | After |
|--------|--------|-------|
| Error handling duplication | 8 match blocks | 1 helper function |
| Token list allocations | Vec + 3 Strings per call | Zero (static) |
| RPC latency (write ops) | Sequential | Parallel (~50% faster) |
| Unit tests | 22 | 28 |
| Integration tests | 4 (broken) | 4 (passing) |

## Remaining Opportunities

### Low Priority (P3)

| Item | Notes |
|------|-------|
| Move client to TempoService struct | Improves testability |
| Add mocked RPC tests | Better unit test coverage |
| Pre-size RLP buffers | Minor allocation savings |
| Configure HTTP client pool | Better connection reuse |
