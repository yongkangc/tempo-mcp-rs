use alloy_primitives::{keccak256, Address, B256, U256};
use alloy_rlp::{Encodable, RlpEncodable};
use anyhow::Result;
use k256::ecdsa::{signature::hazmat::PrehashSigner, SigningKey};
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

// Faucet contract address
pub const FAUCET_ADDRESS: Address = Address::new([
    0x20, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x02, 0x00,
]);

/// EIP-155 Legacy Transaction for signing
#[derive(RlpEncodable)]
pub struct LegacyTxForSigning {
    pub nonce: u64,
    pub gas_price: U256,
    pub gas_limit: u64,
    pub to: Address,
    pub value: U256,
    pub data: Vec<u8>,
    pub chain_id: u64,
    pub zero1: u8,
    pub zero2: u8,
}

/// Signed Legacy Transaction for broadcasting
pub struct SignedLegacyTx {
    pub nonce: u64,
    pub gas_price: U256,
    pub gas_limit: u64,
    pub to: Address,
    pub value: U256,
    pub data: Vec<u8>,
    pub v: u64,
    pub r: U256,
    pub s: U256,
}

impl Encodable for SignedLegacyTx {
    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        alloy_rlp::Header {
            list: true,
            payload_length: self.nonce.length()
                + self.gas_price.length()
                + self.gas_limit.length()
                + self.to.length()
                + self.value.length()
                + self.data.length()
                + self.v.length()
                + self.r.length()
                + self.s.length(),
        }
        .encode(out);
        self.nonce.encode(out);
        self.gas_price.encode(out);
        self.gas_limit.encode(out);
        self.to.encode(out);
        self.value.encode(out);
        self.data.encode(out);
        self.v.encode(out);
        self.r.encode(out);
        self.s.encode(out);
    }
}

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

    pub async fn get_nonce(&self, address: Address) -> Result<u64> {
        let result: String = self
            .call_rpc(
                "eth_getTransactionCount",
                json!([format!("{:?}", address), "pending"]),
            )
            .await?;
        let nonce = u64::from_str_radix(result.trim_start_matches("0x"), 16)?;
        Ok(nonce)
    }

    pub async fn get_gas_price(&self) -> Result<U256> {
        let result: String = self.call_rpc("eth_gasPrice", json!([])).await?;
        let price = U256::from_str_radix(result.trim_start_matches("0x"), 16)?;
        Ok(price)
    }

    pub async fn send_raw_transaction(&self, raw_tx: &[u8]) -> Result<B256> {
        let hex_tx = format!("0x{}", hex::encode(raw_tx));
        let result: String = self
            .call_rpc("eth_sendRawTransaction", json!([hex_tx]))
            .await?;
        let hash: B256 = result.parse()?;
        Ok(hash)
    }

    pub fn sign_transaction(
        private_key: &[u8; 32],
        to: Address,
        value: U256,
        data: Vec<u8>,
        nonce: u64,
        gas_price: U256,
        gas_limit: u64,
    ) -> Result<Vec<u8>> {
        let signing_key = SigningKey::from_bytes(private_key.into())?;

        let tx_for_signing = LegacyTxForSigning {
            nonce,
            gas_price,
            gas_limit,
            to,
            value,
            data: data.clone(),
            chain_id: TEMPO_TESTNET_CHAIN_ID,
            zero1: 0,
            zero2: 0,
        };

        let mut rlp_buf = Vec::new();
        tx_for_signing.encode(&mut rlp_buf);
        let tx_hash = keccak256(&rlp_buf);

        let (signature, recovery_id) = signing_key.sign_prehash(tx_hash.as_slice())?;
        let sig_bytes = signature.to_bytes();
        let r = U256::from_be_slice(&sig_bytes[..32]);
        let s = U256::from_be_slice(&sig_bytes[32..]);

        // EIP-155: v = recovery_id + chain_id * 2 + 35
        let v = recovery_id.to_byte() as u64 + TEMPO_TESTNET_CHAIN_ID * 2 + 35;

        let signed_tx = SignedLegacyTx {
            nonce,
            gas_price,
            gas_limit,
            to,
            value,
            data,
            v,
            r,
            s,
        };

        let mut signed_rlp = Vec::new();
        signed_tx.encode(&mut signed_rlp);
        Ok(signed_rlp)
    }

    pub fn get_address_from_private_key(private_key: &[u8; 32]) -> Result<Address> {
        let signing_key = SigningKey::from_bytes(private_key.into())?;
        let verifying_key = signing_key.verifying_key();
        let public_key = verifying_key.to_encoded_point(false);
        let public_key_bytes = &public_key.as_bytes()[1..]; // Skip the 0x04 prefix
        let hash = keccak256(public_key_bytes);
        Ok(Address::from_slice(&hash[12..]))
    }

    /// Transfer TIP-20 tokens
    pub async fn transfer(
        &self,
        private_key: &[u8; 32],
        token: Address,
        to: Address,
        amount: U256,
    ) -> Result<B256> {
        let from = Self::get_address_from_private_key(private_key)?;
        let nonce = self.get_nonce(from).await?;
        let gas_price = self.get_gas_price().await?;

        // transfer(address,uint256) selector = 0xa9059cbb
        let mut data = vec![0xa9, 0x05, 0x9c, 0xbb];
        data.extend_from_slice(&[0u8; 12]); // pad address to 32 bytes
        data.extend_from_slice(to.as_slice());
        let mut amount_bytes = [0u8; 32];
        amount
            .to_be_bytes::<32>()
            .iter()
            .enumerate()
            .for_each(|(i, b)| amount_bytes[i] = *b);
        data.extend_from_slice(&amount_bytes);

        let raw_tx = Self::sign_transaction(
            private_key,
            token,
            U256::ZERO,
            data,
            nonce,
            gas_price,
            100_000, // gas limit for transfer
        )?;

        self.send_raw_transaction(&raw_tx).await
    }

    /// Swap tokens on DEX
    pub async fn swap(
        &self,
        private_key: &[u8; 32],
        token_in: Address,
        token_out: Address,
        amount_in: u128,
        min_amount_out: u128,
    ) -> Result<B256> {
        let from = Self::get_address_from_private_key(private_key)?;
        let nonce = self.get_nonce(from).await?;
        let gas_price = self.get_gas_price().await?;

        // swapExactAmountIn(address,address,uint128,uint128,address) selector
        // We'll use a simplified version
        let selector: [u8; 4] = [0x8a, 0x65, 0x4b, 0x51]; // placeholder, need actual selector
        let mut data = selector.to_vec();
        data.extend_from_slice(&[0u8; 12]);
        data.extend_from_slice(token_in.as_slice());
        data.extend_from_slice(&[0u8; 12]);
        data.extend_from_slice(token_out.as_slice());
        data.extend_from_slice(&[0u8; 16]);
        data.extend_from_slice(&amount_in.to_be_bytes());
        data.extend_from_slice(&[0u8; 16]);
        data.extend_from_slice(&min_amount_out.to_be_bytes());
        data.extend_from_slice(&[0u8; 12]);
        data.extend_from_slice(from.as_slice()); // recipient

        let raw_tx = Self::sign_transaction(
            private_key,
            DEX_ADDRESS,
            U256::ZERO,
            data,
            nonce,
            gas_price,
            200_000, // gas limit for swap
        )?;

        self.send_raw_transaction(&raw_tx).await
    }

    /// Request tokens from faucet
    pub async fn faucet(&self, private_key: &[u8; 32], token: Address) -> Result<B256> {
        let from = Self::get_address_from_private_key(private_key)?;
        let nonce = self.get_nonce(from).await?;
        let gas_price = self.get_gas_price().await?;

        // drip(address) selector = 0x23bc6c5d (common faucet function)
        let mut data = vec![0x23, 0xbc, 0x6c, 0x5d];
        data.extend_from_slice(&[0u8; 12]);
        data.extend_from_slice(token.as_slice());

        let raw_tx = Self::sign_transaction(
            private_key,
            FAUCET_ADDRESS,
            U256::ZERO,
            data,
            nonce,
            gas_price,
            100_000, // gas limit for faucet
        )?;

        self.send_raw_transaction(&raw_tx).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_tokens() {
        let tokens = known_tokens();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].symbol, "TUSD");
        assert_eq!(tokens[1].symbol, "TEUR");
        assert_eq!(tokens[2].symbol, "TGBP");
    }

    #[test]
    fn test_get_token_by_symbol() {
        assert!(get_token_by_symbol("TUSD").is_some());
        assert!(get_token_by_symbol("tusd").is_some()); // case insensitive
        assert!(get_token_by_symbol("TEUR").is_some());
        assert!(get_token_by_symbol("UNKNOWN").is_none());
    }

    #[test]
    fn test_get_token_by_address() {
        assert!(get_token_by_address(TUSD_ADDRESS).is_some());
        assert!(get_token_by_address(TEUR_ADDRESS).is_some());
        assert!(get_token_by_address(TGBP_ADDRESS).is_some());
        assert!(get_token_by_address(Address::ZERO).is_none());
    }

    #[test]
    fn test_format_token_amount_whole() {
        let amount = U256::from(1_000_000u64); // 1.0 with 6 decimals
        assert_eq!(format_token_amount(amount, 6), "1");
    }

    #[test]
    fn test_format_token_amount_fractional() {
        let amount = U256::from(1_500_000u64); // 1.5 with 6 decimals
        assert_eq!(format_token_amount(amount, 6), "1.5");
    }

    #[test]
    fn test_format_token_amount_small() {
        let amount = U256::from(100u64); // 0.0001 with 6 decimals
        assert_eq!(format_token_amount(amount, 6), "0.0001");
    }

    #[test]
    fn test_parse_token_amount_whole() {
        let result = parse_token_amount("100", 6).unwrap();
        assert_eq!(result, U256::from(100_000_000u64));
    }

    #[test]
    fn test_parse_token_amount_decimal() {
        let result = parse_token_amount("1.5", 6).unwrap();
        assert_eq!(result, U256::from(1_500_000u64));
    }

    #[test]
    fn test_parse_token_amount_small() {
        let result = parse_token_amount("0.001", 6).unwrap();
        assert_eq!(result, U256::from(1_000u64));
    }

    #[test]
    fn test_roundtrip_amount() {
        let original = "123.456789";
        let parsed = parse_token_amount(original, 6).unwrap();
        let formatted = format_token_amount(parsed, 6);
        // Note: may lose precision beyond 6 decimals
        assert_eq!(formatted, "123.456789");
    }

    #[test]
    fn test_get_address_from_private_key() {
        // Well-known test private key
        let private_key: [u8; 32] = [
            0xac, 0x09, 0x74, 0xbe, 0xc3, 0x9a, 0x17, 0xe3, 0x6b, 0xa4, 0xa6, 0xb4, 0xd2, 0x38,
            0xff, 0x94, 0x4b, 0xac, 0xb4, 0x78, 0xcb, 0xed, 0x5e, 0xfc, 0xae, 0x78, 0x4d, 0x7b,
            0xf4, 0xf2, 0xff, 0x80,
        ];
        let address = TempoClient::get_address_from_private_key(&private_key).unwrap();
        // Address should be derived correctly (not checking exact value)
        assert!(!address.is_zero());
    }

    #[test]
    fn test_sign_transaction() {
        let private_key: [u8; 32] = [1u8; 32]; // Simple test key
        let to = Address::ZERO;
        let value = U256::from(100u64);
        let data = vec![];
        let nonce = 0;
        let gas_price = U256::from(1_000_000_000u64);
        let gas_limit = 21000;

        let result = TempoClient::sign_transaction(
            &private_key,
            to,
            value,
            data,
            nonce,
            gas_price,
            gas_limit,
        );
        assert!(result.is_ok());
        let signed_tx = result.unwrap();
        assert!(!signed_tx.is_empty());
    }

    // Integration tests (require network, run with --ignored)
    #[tokio::test]
    #[ignore]
    async fn test_get_balance_live() {
        let client = TempoClient::new();
        let address: Address = "0x0000000000000000000000000000000000000001"
            .parse()
            .unwrap();
        let result = client.get_balance(address, TUSD_ADDRESS).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_gas_price_live() {
        let client = TempoClient::new();
        let result = client.get_gas_price().await;
        assert!(result.is_ok());
        assert!(!result.unwrap().is_zero());
    }
}
