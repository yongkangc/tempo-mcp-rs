# tempo-mcp
<img width="567" height="199" alt="image" src="https://github.com/user-attachments/assets/a6bed97c-23b4-4ff0-b9ab-3a482d799395" />


[![Crates.io](https://img.shields.io/crates/v/tempo-mcp.svg)](https://crates.io/crates/tempo-mcp)
[![smithery badge](https://smithery.ai/badge/@yongkangc/tempo-mcp-rs)](https://smithery.ai/server/@yongkangc/tempo-mcp-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

MCP server for Tempo blockchain - query balances, swap tokens, and interact with onchain data through AI assistants.

## Demo

https://github.com/user-attachments/assets/efea376f-1723-4095-acc2-ea72323e0425



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

<details>
<summary>MCP Clients</summary>

### Claude Code

```bash
claude mcp add tempo tempo-mcp
```

### Smithery

[![Install MCP Server](https://smithery.ai/badge/@yongkangc/tempo-mcp-rs)](https://smithery.ai/server/@yongkangc/tempo-mcp-rs)

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

### Codex

Add to `~/.codex/config.toml`:

```toml
[mcp_servers.tempo]
command = "tempo-mcp"
```

Or via CLI:

```bash
codex mcp add tempo -- tempo-mcp
```

### Amp

Add to `.amp/settings.json` in your project:

```json
{
  "amp.mcpServers": {
    "tempo": {
      "command": "tempo-mcp"
    }
  }
}
```

Or via CLI:

```bash
amp mcp add --workspace tempo -- tempo-mcp
```

### VS Code

Add to `.vscode/mcp.json` in your workspace:

```json
{
  "servers": {
    "tempo": {
      "command": "tempo-mcp"
    }
  }
}
```

Or add globally via Command Palette: `MCP: Add Server`

### Gemini CLI

Add to `~/.gemini/settings.json`:

```json
{
  "mcpServers": {
    "tempo": {
      "command": "tempo-mcp"
    }
  }
}
```

Or via CLI:

```bash
gemini mcp add tempo -- tempo-mcp
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

## Security: Private Key Configuration

For transaction tools (`tempo_transfer`, `tempo_swap`, `tempo_faucet`), configure your private key via environment variable instead of passing it in tool calls. This prevents keys from appearing in MCP transcripts.

<details>
<summary>Client configuration examples</summary>

**Claude Desktop / VS Code / Cursor:**

```json
{
  "mcpServers": {
    "tempo": {
      "command": "tempo-mcp",
      "env": {
        "TEMPO_PRIVATE_KEY": "0x..."
      }
    }
  }
}
```

**Codex:**

```toml
[mcp_servers.tempo]
command = "tempo-mcp"

[mcp_servers.tempo.env]
TEMPO_PRIVATE_KEY = "0x..."
```

**Amp:**

```json
{
  "amp.mcpServers": {
    "tempo": {
      "command": "tempo-mcp",
      "env": {
        "TEMPO_PRIVATE_KEY": "0x..."
      }
    }
  }
}
```

</details>

Once configured, transaction tools work without passing the key:
```
You: Send 50 TUSD to 0xabcd...
Claude: Transfer submitted! Tx: 0x...
```

## Example Prompts

<details>
<summary>Try these prompts to test all tools</summary>

**List tokens** (`tempo_list_tokens`)
> "What tokens are available on Tempo?"

**Check balance** (`tempo_get_balance`)
> "What's the TUSD balance for 0x1234...?"

**Get DEX quote** (`tempo_get_dex_quote`)
> "How much TEUR would I get for 100 TUSD?"

**Get transaction** (`tempo_get_transaction`)
> "Show me transaction 0xabc..."

**Decode transaction** (`tempo_decode_transaction`)
> "What happened in transaction 0xabc...?"

**Request from faucet** (`tempo_faucet`)
> "Get me some testnet TUSD from the faucet"

**Transfer tokens** (`tempo_transfer`)
> "Send 50 TUSD to 0xabcd..."

**Swap tokens** (`tempo_swap`)
> "Swap 100 TUSD for TEUR"

</details>

## Network

| | |
|---|---|
| Network | Tempo Testnet |
| Chain ID | 62320 |
| RPC | https://rpc.testnet.tempo.xyz |
| Explorer | https://explore.tempo.xyz |

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
