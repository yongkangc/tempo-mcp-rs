#!/usr/bin/env python3
"""
Integration test for tempo-mcp with Claude API.

This script:
1. Spawns the tempo-mcp server
2. Connects to Claude API
3. Sends prompts that should trigger tool calls
4. Verifies the tools are called correctly

Requirements:
    pip install anthropic

Usage:
    export ANTHROPIC_API_KEY=your_key
    python scripts/test-with-claude.py
"""

import json
import os
import subprocess
import sys
from typing import Any

try:
    import anthropic
except ImportError:
    print("Error: anthropic package not installed")
    print("Run: pip install anthropic")
    sys.exit(1)


def build_tempo_mcp():
    """Build the tempo-mcp binary."""
    print("Building tempo-mcp...")
    result = subprocess.run(
        ["cargo", "build", "--release"],
        capture_output=True,
        text=True
    )
    if result.returncode != 0:
        print(f"Build failed: {result.stderr}")
        sys.exit(1)
    print("Build successful")


def get_tempo_tools() -> list[dict[str, Any]]:
    """Define the tools for Claude API."""
    return [
        {
            "name": "tempo_get_balance",
            "description": "Get the token balance for an address on Tempo blockchain",
            "input_schema": {
                "type": "object",
                "properties": {
                    "address": {
                        "type": "string",
                        "description": "Wallet address to check balance for"
                    },
                    "token": {
                        "type": "string",
                        "description": "Token symbol (TUSD, TEUR, TGBP) or address"
                    }
                },
                "required": ["address"]
            }
        },
        {
            "name": "tempo_list_tokens",
            "description": "List known tokens on Tempo blockchain",
            "input_schema": {
                "type": "object",
                "properties": {}
            }
        },
        {
            "name": "tempo_get_dex_quote",
            "description": "Get a quote for swapping tokens on Tempo DEX",
            "input_schema": {
                "type": "object",
                "properties": {
                    "token_in": {
                        "type": "string",
                        "description": "Token to sell"
                    },
                    "token_out": {
                        "type": "string",
                        "description": "Token to buy"
                    },
                    "amount": {
                        "type": "string",
                        "description": "Amount"
                    }
                },
                "required": ["token_in", "token_out", "amount"]
            }
        }
    ]


def call_tempo_tool(proc: subprocess.Popen, tool_name: str, arguments: dict) -> str:
    """Call a tempo-mcp tool via MCP protocol."""
    request = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": tool_name,
            "arguments": arguments
        }
    }

    proc.stdin.write(json.dumps(request) + "\n")
    proc.stdin.flush()

    response_line = proc.stdout.readline()
    response = json.loads(response_line)

    if "result" in response:
        content = response["result"].get("content", [])
        if content and "text" in content[0]:
            return content[0]["text"]

    return str(response)


def run_test():
    """Run the integration test."""
    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        print("Error: ANTHROPIC_API_KEY environment variable not set")
        print("Set it with: export ANTHROPIC_API_KEY=your_key")
        sys.exit(1)

    build_tempo_mcp()

    # Start tempo-mcp server
    print("Starting tempo-mcp server...")
    proc = subprocess.Popen(
        ["./target/release/tempo-mcp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        text=True,
        bufsize=1
    )

    try:
        # Initialize MCP
        init_request = {
            "jsonrpc": "2.0",
            "id": 0,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "test", "version": "1.0"}
            }
        }
        proc.stdin.write(json.dumps(init_request) + "\n")
        proc.stdin.flush()
        proc.stdout.readline()  # Read init response

        # Create Claude client
        client = anthropic.Anthropic(api_key=api_key)

        # Test prompts
        test_cases = [
            {
                "prompt": "What tokens are available on Tempo blockchain?",
                "expected_tool": "tempo_list_tokens"
            },
            {
                "prompt": "Check the TUSD balance for address 0x0000000000000000000000000000000000000001",
                "expected_tool": "tempo_get_balance"
            }
        ]

        for i, test in enumerate(test_cases, 1):
            print(f"\n=== Test {i}: {test['expected_tool']} ===")
            print(f"Prompt: {test['prompt']}")

            # Call Claude
            response = client.messages.create(
                model="claude-sonnet-4-20250514",
                max_tokens=1024,
                tools=get_tempo_tools(),
                messages=[{"role": "user", "content": test["prompt"]}]
            )

            # Check if Claude called the expected tool
            tool_use = next(
                (block for block in response.content if block.type == "tool_use"),
                None
            )

            if tool_use:
                print(f"Claude called: {tool_use.name}")
                print(f"Arguments: {tool_use.input}")

                # Execute the tool
                result = call_tempo_tool(proc, tool_use.name, tool_use.input)
                print(f"Result: {result[:200]}..." if len(result) > 200 else f"Result: {result}")

                if tool_use.name == test["expected_tool"]:
                    print("PASS: Correct tool called")
                else:
                    print(f"FAIL: Expected {test['expected_tool']}, got {tool_use.name}")
            else:
                print("FAIL: No tool called")
                print(f"Response: {response.content}")

        print("\n=== Integration test complete ===")

    finally:
        proc.terminate()
        proc.wait()


if __name__ == "__main__":
    run_test()
