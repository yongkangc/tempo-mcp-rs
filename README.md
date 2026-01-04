# tempo-mcp-rs

MCP server for Tempo blockchain - enabling AI agents to read and write onchain data.

## Features

- **Read Operations**: Query balances, transactions, DEX quotes
- **Write Operations**: Transfer tokens, swap on DEX, request from faucet
- **MCP Protocol**: Compatible with Claude Desktop and other MCP clients

## Installation

```bash
cargo install tempo-mcp
```

Then add to your MCP client:

```bash
claude mcp add tempo tempo-mcp
```

Or build from source:

```bash
git clone https://github.com/yongkangc/tempo-mcp-rs.git
cd tempo-mcp-rs
cargo build --release
```

## Tools

### Read Tools

| Tool | Description |
|------|-------------|
| `tempo_get_balance` | Get token balance for an address |
| `tempo_get_transaction` | Get transaction details by hash |
| `tempo_decode_transaction` | Decode and explain a transaction |
| `tempo_get_dex_quote` | Get swap quote from DEX |
| `tempo_list_tokens` | List known tokens on Tempo |

### Write Tools

| Tool | Description |
|------|-------------|
| `tempo_transfer` | Transfer TIP-20 tokens |
| `tempo_swap` | Swap tokens on DEX |
| `tempo_faucet` | Request testnet tokens |

## Examples

See [docs/USAGE.md](docs/USAGE.md) for MCP client setup instructions.

### Check Balance

```
User: What's the TUSD balance for 0x1234...?

Claude calls: tempo_get_balance
Arguments: { "address": "0x1234...", "token": "TUSD" }

Response: Balance for 0x1234...: 1000.5 TUSD
```

### Get DEX Quote

```
User: How much TEUR would I get for 100 TUSD?

Claude calls: tempo_get_dex_quote
Arguments: { "token_in": "TUSD", "token_out": "TEUR", "amount": "100" }

Response: Sell Quote: Selling 100 TUSD will get you 92.5 TEUR
```

### Transfer Tokens

```
User: Send 50 TUSD to 0xabcd...

Claude calls: tempo_transfer
Arguments: {
  "private_key": "0x...",
  "to": "0xabcd...",
  "amount": "50",
  "token": "TUSD"
}

Response: Transfer submitted!
From: 0x1234...
To: 0xabcd...
Amount: 50 TUSD
Transaction: 0x...
Explorer: https://explorer.testnet.tempo.xyz/tx/0x...
```

### Swap Tokens

```
User: Swap 100 TUSD for TEUR

Claude calls: tempo_swap
Arguments: {
  "private_key": "0x...",
  "token_in": "TUSD",
  "token_out": "TEUR",
  "amount_in": "100"
}

Response: Swap submitted!
From: 0x1234...
Selling: 100 TUSD
Buying: TEUR
Transaction: 0x...
```

## Supported Tokens

| Symbol | Address | Decimals |
|--------|---------|----------|
| TUSD | 0x20C0000000000000000000000000000000000000 | 6 |
| TEUR | 0x20C0000000000000000000000000000000000001 | 6 |
| TGBP | 0x20C0000000000000000000000000000000000002 | 6 |

## Network

- **Network**: Tempo Testnet
- **Chain ID**: 62320
- **RPC**: https://rpc.testnet.tempo.xyz
- **Explorer**: https://explorer.testnet.tempo.xyz

## Development

See [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) for the full development guide.

```bash
# Run tests
cargo test

# Format code
cargo +nightly fmt

# Lint
cargo +nightly clippy

# Build release
cargo build --release
```

## License

MIT
