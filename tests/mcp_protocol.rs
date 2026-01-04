//! MCP Protocol Integration Tests
//!
//! These tests spawn the tempo-mcp server and communicate with it
//! using the MCP protocol over stdio.

use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};

struct McpTestClient {
    child: Child,
    request_id: u64,
}

impl McpTestClient {
    fn new() -> Self {
        let child = Command::new("cargo")
            .args(["run", "--bin", "tempo-mcp"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to spawn tempo-mcp");

        Self {
            child,
            request_id: 0,
        }
    }

    fn send_request(&mut self, method: &str, params: Value) -> Value {
        self.request_id += 1;
        let request = json!({
            "jsonrpc": "2.0",
            "id": self.request_id,
            "method": method,
            "params": params
        });

        let stdin = self.child.stdin.as_mut().expect("No stdin");
        let request_str = serde_json::to_string(&request).unwrap();
        writeln!(stdin, "{}", request_str).expect("Failed to write");
        stdin.flush().expect("Failed to flush");

        let stdout = self.child.stdout.as_mut().expect("No stdout");
        let mut reader = BufReader::new(stdout);
        let mut response_line = String::new();
        reader
            .read_line(&mut response_line)
            .expect("Failed to read response");

        serde_json::from_str(&response_line).expect("Failed to parse response")
    }

    fn initialize(&mut self) -> Value {
        self.send_request(
            "initialize",
            json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {
                    "name": "test-client",
                    "version": "1.0.0"
                }
            }),
        )
    }

    fn list_tools(&mut self) -> Value {
        self.send_request("tools/list", json!({}))
    }

    fn call_tool(&mut self, name: &str, arguments: Value) -> Value {
        self.send_request(
            "tools/call",
            json!({
                "name": name,
                "arguments": arguments
            }),
        )
    }
}

impl Drop for McpTestClient {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

#[test]
#[ignore] // Requires cargo build first
fn test_mcp_initialize() {
    let mut client = McpTestClient::new();
    let response = client.initialize();

    assert!(response.get("result").is_some());
    let result = &response["result"];
    assert!(result.get("serverInfo").is_some());
    assert!(result.get("capabilities").is_some());
}

#[test]
#[ignore] // Requires cargo build first
fn test_mcp_list_tools() {
    let mut client = McpTestClient::new();
    client.initialize();
    let response = client.list_tools();

    assert!(response.get("result").is_some());
    let tools = response["result"]["tools"].as_array().unwrap();

    let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();

    assert!(tool_names.contains(&"tempo_get_balance"));
    assert!(tool_names.contains(&"tempo_get_transaction"));
    assert!(tool_names.contains(&"tempo_decode_transaction"));
    assert!(tool_names.contains(&"tempo_get_dex_quote"));
    assert!(tool_names.contains(&"tempo_list_tokens"));
    assert!(tool_names.contains(&"tempo_transfer"));
    assert!(tool_names.contains(&"tempo_swap"));
    assert!(tool_names.contains(&"tempo_faucet"));
}

#[test]
#[ignore] // Requires cargo build first
fn test_mcp_tool_list_tokens() {
    let mut client = McpTestClient::new();
    client.initialize();
    let response = client.call_tool("tempo_list_tokens", json!({}));

    assert!(response.get("result").is_some());
    let content = &response["result"]["content"][0]["text"];
    let text = content.as_str().unwrap();

    assert!(text.contains("TUSD"));
    assert!(text.contains("TEUR"));
    assert!(text.contains("TGBP"));
}

#[test]
#[ignore] // Requires network
fn test_mcp_tool_get_balance() {
    let mut client = McpTestClient::new();
    client.initialize();
    let response = client.call_tool(
        "tempo_get_balance",
        json!({
            "address": "0x0000000000000000000000000000000000000001",
            "token": "TUSD"
        }),
    );

    assert!(response.get("result").is_some());
}
