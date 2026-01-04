# Usage with Claude Desktop

Add to `~/.config/claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "tempo": {
      "command": "/path/to/tempo-mcp-rs/target/release/tempo-mcp"
    }
  }
}
```

Restart Claude Desktop, then ask:

> "What tokens are available on Tempo?"

> "Check my TUSD balance at 0x..."

> "Get a quote to swap 100 TUSD to TEUR"
