use alloy_primitives::{Address, B256, U256};
use anyhow::Result;
use serde::Deserialize;
use serde_json::{json, Value};

pub const TEMPO_TESTNET_RPC: &str = "https://rpc.testnet.tempo.xyz";
pub const TEMPO_TESTNET_CHAIN_ID: u64 = 62320;

// Known token addresses on Tempo Testnet
pub const TUSD_ADDRESS: Address = Address::new([
    0x20, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
]);
pub const TEUR_ADDRESS: Address = Address::new([
    0x20, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x01,
]);
pub const TGBP_ADDRESS: Address = Address::new([
    0x20, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x02,
]);

// DEX contract address
pub const DEX_ADDRESS: Address = Address::new([
    0x20, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x01, 0x00,
]);

#[derive(Clone, Debug)]
pub struct TokenInfo {
    pub address: Address,
    pub symbol: String,
    pub decimals: u8,
}

pub fn known_tokens() -> Vec<TokenInfo> {
    vec![
        TokenInfo {
            address: TUSD_ADDRESS,
            symbol: "TUSD".to_string(),
            decimals: 6,
        },
        TokenInfo {
            address: TEUR_ADDRESS,
            symbol: "TEUR".to_string(),
            decimals: 6,
        },
        TokenInfo {
            address: TGBP_ADDRESS,
            symbol: "TGBP".to_string(),
            decimals: 6,
        },
    ]
}

pub fn get_token_by_symbol(symbol: &str) -> Option<TokenInfo> {
    known_tokens()
        .into_iter()
        .find(|t| t.symbol.eq_ignore_ascii_case(symbol))
}

pub fn get_token_by_address(address: Address) -> Option<TokenInfo> {
    known_tokens().into_iter().find(|t| t.address == address)
}

pub fn format_token_amount(amount: U256, decimals: u8) -> String {
    let divisor = U256::from(10u64.pow(decimals as u32));
    let whole = amount / divisor;
    let frac = amount % divisor;

    if frac.is_zero() {
        whole.to_string()
    } else {
        let frac_str = format!("{:0>width$}", frac, width = decimals as usize);
        let trimmed = frac_str.trim_end_matches('0');
        format!("{}.{}", whole, trimmed)
    }
}

pub fn parse_token_amount(amount: &str, decimals: u8) -> Result<U256> {
    let parts: Vec<&str> = amount.split('.').collect();
    let whole: U256 = parts[0].parse().unwrap_or(U256::ZERO);

    let frac = if parts.len() > 1 {
        let frac_str = format!("{:0<width$}", parts[1], width = decimals as usize);
        let frac_str = &frac_str[..decimals as usize];
        frac_str.parse().unwrap_or(U256::ZERO)
    } else {
        U256::ZERO
    };

    let multiplier = U256::from(10u64.pow(decimals as u32));
    Ok(whole * multiplier + frac)
}

#[derive(Debug, Deserialize)]
pub struct RpcResponse<T> {
    pub result: Option<T>,
    pub error: Option<RpcError>,
}

#[derive(Debug, Deserialize)]
pub struct RpcError {
    pub code: i64,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Transaction {
    pub hash: B256,
    pub from: Address,
    pub to: Option<Address>,
    pub value: U256,
    #[serde(rename = "blockNumber")]
    pub block_number: Option<String>,
    pub gas: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct TransactionReceipt {
    pub status: String,
    #[serde(rename = "gasUsed")]
    pub gas_used: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    pub logs: Vec<Log>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Log {
    pub address: Address,
    pub topics: Vec<B256>,
    pub data: String,
}

#[derive(Clone)]
pub struct TempoClient {
    client: reqwest::Client,
    rpc_url: String,
}

impl Default for TempoClient {
    fn default() -> Self {
        Self::new()
    }
}

impl TempoClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            rpc_url: TEMPO_TESTNET_RPC.to_string(),
        }
    }

    async fn call_rpc<T: for<'de> Deserialize<'de>>(
        &self,
        method: &str,
        params: Value,
    ) -> Result<T> {
        let body = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });

        let resp: RpcResponse<T> = self
            .client
            .post(&self.rpc_url)
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        if let Some(error) = resp.error {
            anyhow::bail!("RPC error {}: {}", error.code, error.message);
        }

        resp.result
            .ok_or_else(|| anyhow::anyhow!("No result in response"))
    }

    pub async fn get_balance(&self, address: Address, token: Address) -> Result<U256> {
        // balanceOf(address) selector = 0x70a08231
        let data = format!(
            "0x70a08231000000000000000000000000{}",
            hex::encode(address.as_slice())
        );

        let result: String = self
            .call_rpc(
                "eth_call",
                json!([{
                    "to": format!("{:?}", token),
                    "data": data
                }, "latest"]),
            )
            .await?;

        let balance = U256::from_str_radix(result.trim_start_matches("0x"), 16)?;
        Ok(balance)
    }

    pub async fn get_transaction(&self, hash: B256) -> Result<Option<Transaction>> {
        let result: Option<Transaction> = self
            .call_rpc("eth_getTransactionByHash", json!([format!("{:?}", hash)]))
            .await
            .ok();
        Ok(result)
    }

    pub async fn get_transaction_receipt(&self, hash: B256) -> Result<Option<TransactionReceipt>> {
        let result: Option<TransactionReceipt> = self
            .call_rpc("eth_getTransactionReceipt", json!([format!("{:?}", hash)]))
            .await
            .ok();
        Ok(result)
    }

    pub async fn get_dex_quote_sell(
        &self,
        token_in: Address,
        token_out: Address,
        amount_in: u128,
    ) -> Result<u128> {
        // quoteSwapExactAmountIn(address,address,uint128) selector
        let selector = "0x7d5f4c81";
        let data = format!(
            "{}000000000000000000000000{}000000000000000000000000{}{:064x}",
            selector,
            hex::encode(token_in.as_slice()),
            hex::encode(token_out.as_slice()),
            amount_in
        );

        let result: String = self
            .call_rpc(
                "eth_call",
                json!([{
                    "to": format!("{:?}", DEX_ADDRESS),
                    "data": data
                }, "latest"]),
            )
            .await?;

        let quote = u128::from_str_radix(result.trim_start_matches("0x"), 16)?;
        Ok(quote)
    }

    pub async fn get_dex_quote_buy(
        &self,
        token_in: Address,
        token_out: Address,
        amount_out: u128,
    ) -> Result<u128> {
        // quoteSwapExactAmountOut(address,address,uint128) selector
        let selector = "0x30c5e5e7";
        let data = format!(
            "{}000000000000000000000000{}000000000000000000000000{}{:064x}",
            selector,
            hex::encode(token_in.as_slice()),
            hex::encode(token_out.as_slice()),
            amount_out
        );

        let result: String = self
            .call_rpc(
                "eth_call",
                json!([{
                    "to": format!("{:?}", DEX_ADDRESS),
                    "data": data
                }, "latest"]),
            )
            .await?;

        let quote = u128::from_str_radix(result.trim_start_matches("0x"), 16)?;
        Ok(quote)
    }
}
