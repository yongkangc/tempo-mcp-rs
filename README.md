# tempo-mcp

[![Crates.io](https://img.shields.io/crates/v/tempo-mcp.svg)](https://crates.io/crates/tempo-mcp)
[![smithery badge](https://smithery.ai/badge/@yongkangc/tempo-mcp-rs)](https://smithery.ai/server/@yongkangc/tempo-mcp-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

MCP server for Tempo blockchain - query balances, swap tokens, and interact with onchain data through AI assistants.

## Quick Example

```
You: What's my TUSD balance at 0x1234...?
Claude: Your balance is 1,000.50 TUSD

You: Swap 100 TUSD for TEUR
Claude: Swap submitted! Tx: 0xabc... (link to explorer)
```

## Installation

```bash
cargo install tempo-mcp
```

Add to Claude Code:

```bash
claude mcp add tempo tempo-mcp
```

<details>
<summary>Other MCP Clients</summary>

### Claude Desktop

Add to `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "tempo": {
      "command": "tempo-mcp"
    }
  }
}
```

### Cursor

Add to Cursor MCP settings with the same config above.

### Windsurf

Add to `~/.codeium/windsurf/mcp_config.json` with the same config.

### Build from Source

```bash
git clone https://github.com/yongkangc/tempo-mcp-rs.git
cd tempo-mcp-rs
cargo build --release
```

</details>

## Tools

| Tool | Description |
|------|-------------|
| `tempo_get_balance` | Get token balance for an address |
| `tempo_get_transaction` | Get transaction details by hash |
| `tempo_decode_transaction` | Decode and explain a transaction |
| `tempo_get_dex_quote` | Get swap quote from DEX |
| `tempo_list_tokens` | List available tokens |
| `tempo_transfer` | Transfer tokens to an address |
| `tempo_swap` | Swap tokens on DEX |
| `tempo_faucet` | Request testnet tokens |

## Usage

**Check balance:**
> "What's the TUSD balance for 0x1234...?"

**Get a quote:**
> "How much TEUR would I get for 100 TUSD?"

**Transfer tokens:**
> "Send 50 TUSD to 0xabcd..."

**Swap tokens:**
> "Swap 100 TUSD for TEUR"

**Request from faucet:**
> "Get me some testnet TUSD"

## Network

| | |
|---|---|
| Network | Tempo Testnet |
| Chain ID | 62320 |
| RPC | https://rpc.testnet.tempo.xyz |
| Explorer | https://explorer.testnet.tempo.xyz |

## Supported Tokens

| Symbol | Address |
|--------|---------|
| TUSD | `0x20C0...0000` |
| TEUR | `0x20C0...0001` |
| TGBP | `0x20C0...0002` |

## Development

See [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) for build, test, and contribution instructions.

## License

MIT
