# Usage

[![smithery badge](https://smithery.ai/badge/tempo-mcp)](https://smithery.ai/server/tempo-mcp) [![Install MCP Server](https://cursor.com/deeplink/mcp-install-dark.svg)](https://cursor.com/en/install-mcp?name=tempo&config=eyJjb21tYW5kIjoidGVtcG8tbWNwIn0=)

## Quick Install

```bash
cargo install tempo-mcp
```

Then add to your MCP client:

```json
{
  "mcpServers": {
    "tempo": {
      "command": "tempo-mcp"
    }
  }
}
```

## MCP Client Configuration

<details>
  <summary>Claude Desktop</summary>

Add to `~/.config/claude/claude_desktop_config.json` (Linux) or `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS):

```json
{
  "mcpServers": {
    "tempo": {
      "command": "tempo-mcp"
    }
  }
}
```

Restart Claude Desktop after updating the config.

</details>

<details>
  <summary>Claude Code</summary>

Use the Claude Code CLI to add the Tempo MCP server:

```bash
claude mcp add tempo tempo-mcp
```

</details>

<details>
  <summary>Codex</summary>

Follow the [configure MCP guide](https://github.com/openai/codex/blob/main/docs/advanced.md#model-context-protocol-mcp) using the standard config from above. You can also install using the Codex CLI:

```bash
codex mcp add tempo -- tempo-mcp
```

</details>

<details>
  <summary>Cursor</summary>

[![Install MCP Server](https://cursor.com/deeplink/mcp-install-dark.svg)](https://cursor.com/en/install-mcp?name=tempo&config=eyJjb21tYW5kIjoidGVtcG8tbWNwIn0=)

> Note: The button assumes `tempo-mcp` is in your PATH. After clicking, update the command path in Cursor settings if needed.

Or manually: Go to `Cursor Settings` -> `MCP` -> `New MCP Server`. Use the config provided above.

</details>

<details>
  <summary>VS Code / Copilot</summary>

Follow the MCP install [guide](https://code.visualstudio.com/docs/copilot/chat/mcp-servers#_add-an-mcp-server), with the standard config from above. You can also install using the VS Code CLI:

```bash
code --add-mcp '{"name":"tempo","command":"tempo-mcp","args":[],"env":{}}'
```

</details>

<details>
  <summary>Windsurf</summary>

Follow the [configure MCP guide](https://docs.windsurf.com/windsurf/cascade/mcp#mcp-config-json) using the standard config from above.

</details>

<details>
  <summary>Smithery</summary>

Install via [Smithery](https://smithery.ai/server/tempo-mcp) using the Smithery CLI:

```bash
npx -y @smithery/cli install tempo-mcp --client claude
```

</details>

## Your First Prompt

After configuring your MCP client, try these prompts:

> "What tokens are available on Tempo?"

> "Check my TUSD balance at 0x..."

> "Get a quote to swap 100 TUSD to TEUR"
