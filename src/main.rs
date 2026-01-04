use anyhow::Result;
use rmcp::{
    model::{CallToolResult, Content, ServerCapabilities, ServerInfo},
    tool,
    transport::io::stdio,
    ServerHandler, ServiceExt,
};
use tokio::sync::OnceCell;

mod tempo;
mod tools;

use tempo::TempoClient;
use tools::*;

static CLIENT: OnceCell<TempoClient> = OnceCell::const_new();

async fn get_client() -> &'static TempoClient {
    CLIENT.get_or_init(|| async { TempoClient::new() }).await
}

#[derive(Clone)]
struct TempoService;

#[tool(tool_box)]
impl TempoService {
    #[tool(description = "Get the token balance for an address on Tempo blockchain")]
    async fn tempo_get_balance(
        &self,
        #[tool(param)]
        #[schemars(description = "Wallet address to check balance for")]
        address: String,
        #[tool(param)]
        #[schemars(description = "Token symbol (TUSD, TEUR, TGBP) or address. Defaults to TUSD")]
        token: Option<String>,
    ) -> Result<CallToolResult, rmcp::Error> {
        let client = get_client().await;
        let input = GetBalanceInput { address, token };
        match get_balance(client, input).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Get transaction details by hash from Tempo blockchain")]
    async fn tempo_get_transaction(
        &self,
        #[tool(param)]
        #[schemars(description = "Transaction hash")]
        hash: String,
    ) -> Result<CallToolResult, rmcp::Error> {
        let client = get_client().await;
        let input = GetTransactionInput { hash };
        match get_transaction(client, input).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Decode a Tempo transaction and explain what happened")]
    async fn tempo_decode_transaction(
        &self,
        #[tool(param)]
        #[schemars(description = "Transaction hash")]
        hash: String,
    ) -> Result<CallToolResult, rmcp::Error> {
        let client = get_client().await;
        let input = GetTransactionInput { hash };
        match decode_transaction(client, input).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Get a quote for swapping tokens on Tempo DEX")]
    async fn tempo_get_dex_quote(
        &self,
        #[tool(param)]
        #[schemars(description = "Token to sell (symbol or address)")]
        token_in: String,
        #[tool(param)]
        #[schemars(description = "Token to buy (symbol or address)")]
        token_out: String,
        #[tool(param)]
        #[schemars(description = "Amount (human-readable, e.g., '100')")]
        amount: String,
        #[tool(param)]
        #[schemars(description = "Quote direction: 'sell' (default) or 'buy'")]
        direction: Option<String>,
    ) -> Result<CallToolResult, rmcp::Error> {
        let client = get_client().await;
        let input = GetDexQuoteInput {
            token_in,
            token_out,
            amount,
            direction,
        };
        match get_dex_quote(client, input).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "List known tokens on Tempo blockchain")]
    fn tempo_list_tokens(&self) -> Result<CallToolResult, rmcp::Error> {
        let result = list_tokens();
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Transfer TIP-20 tokens to another address on Tempo blockchain")]
    async fn tempo_transfer(
        &self,
        #[tool(param)]
        #[schemars(description = "Private key (hex, with or without 0x prefix)")]
        private_key: String,
        #[tool(param)]
        #[schemars(description = "Recipient address")]
        to: String,
        #[tool(param)]
        #[schemars(description = "Amount to transfer (human-readable, e.g., '100')")]
        amount: String,
        #[tool(param)]
        #[schemars(description = "Token symbol (TUSD, TEUR, TGBP) or address. Defaults to TUSD")]
        token: Option<String>,
    ) -> Result<CallToolResult, rmcp::Error> {
        let client = get_client().await;
        let input = TransferInput {
            private_key,
            to,
            amount,
            token,
        };
        match transfer(client, input).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Swap tokens on Tempo DEX")]
    async fn tempo_swap(
        &self,
        #[tool(param)]
        #[schemars(description = "Private key (hex, with or without 0x prefix)")]
        private_key: String,
        #[tool(param)]
        #[schemars(description = "Token to sell (symbol or address)")]
        token_in: String,
        #[tool(param)]
        #[schemars(description = "Token to buy (symbol or address)")]
        token_out: String,
        #[tool(param)]
        #[schemars(description = "Amount to sell (human-readable, e.g., '100')")]
        amount_in: String,
        #[tool(param)]
        #[schemars(description = "Minimum amount to receive (human-readable). Defaults to 0")]
        min_amount_out: Option<String>,
    ) -> Result<CallToolResult, rmcp::Error> {
        let client = get_client().await;
        let input = SwapInput {
            private_key,
            token_in,
            token_out,
            amount_in,
            min_amount_out,
        };
        match swap(client, input).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Request tokens from Tempo testnet faucet")]
    async fn tempo_faucet(
        &self,
        #[tool(param)]
        #[schemars(description = "Private key (hex, with or without 0x prefix)")]
        private_key: String,
        #[tool(param)]
        #[schemars(description = "Token symbol (TUSD, TEUR, TGBP) or address. Defaults to TUSD")]
        token: Option<String>,
    ) -> Result<CallToolResult, rmcp::Error> {
        let client = get_client().await;
        let input = FaucetInput { private_key, token };
        match faucet(client, input).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }
}

#[tool(tool_box)]
impl ServerHandler for TempoService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Tempo blockchain MCP server - read and write onchain data. Query balances, transactions, and DEX quotes on Tempo testnet.".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("Tempo MCP Server starting...");

    let service = TempoService;
    let server = service.serve(stdio()).await?;

    eprintln!("Tempo MCP Server running on stdio");

    server.waiting().await?;

    Ok(())
}
