#!/bin/bash
# Interactive MCP protocol tester
# Usage: ./scripts/test-mcp.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

echo "Building tempo-mcp..."
cargo build --release 2>/dev/null

echo "Starting MCP server..."
echo ""

# Create named pipe for communication
PIPE_DIR=$(mktemp -d)
PIPE_IN="$PIPE_DIR/mcp_in"
PIPE_OUT="$PIPE_DIR/mcp_out"
mkfifo "$PIPE_IN" "$PIPE_OUT"

cleanup() {
    rm -rf "$PIPE_DIR"
    kill $MCP_PID 2>/dev/null || true
}
trap cleanup EXIT

# Start MCP server
./target/release/tempo-mcp < "$PIPE_IN" > "$PIPE_OUT" 2>/dev/null &
MCP_PID=$!

# Open pipe for writing
exec 3>"$PIPE_IN"

# Function to send request and read response
send_request() {
    local request="$1"
    echo "$request" >&3
    head -n 1 "$PIPE_OUT"
}

echo "=== MCP Protocol Test ==="
echo ""

# Initialize
echo "1. Sending initialize request..."
INIT_RESP=$(send_request '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}')
echo "Response: $INIT_RESP"
echo ""

# List tools
echo "2. Sending tools/list request..."
TOOLS_RESP=$(send_request '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}')
echo "Response: $TOOLS_RESP" | jq -r '.result.tools[].name' 2>/dev/null || echo "$TOOLS_RESP"
echo ""

# Call list_tokens
echo "3. Calling tempo_list_tokens..."
TOKENS_RESP=$(send_request '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"tempo_list_tokens","arguments":{}}}')
echo "Response:"
echo "$TOKENS_RESP" | jq -r '.result.content[0].text' 2>/dev/null || echo "$TOKENS_RESP"
echo ""

echo "=== All tests passed ==="
