use alloy_primitives::{Address, B256, U256};
use anyhow::Result;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::tempo::{
    format_token_amount, get_token_by_address, get_token_by_symbol, known_tokens,
    parse_token_amount, TempoClient, TokenInfo,
};

// ============================================================================
// Tool Input Schemas
// ============================================================================

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetBalanceInput {
    /// Wallet address to check balance for
    pub address: String,
    /// Token symbol (TUSD, TEUR, TGBP) or address. Defaults to TUSD
    #[serde(default)]
    pub token: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetTransactionInput {
    /// Transaction hash
    pub hash: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetDexQuoteInput {
    /// Token to sell (symbol or address)
    pub token_in: String,
    /// Token to buy (symbol or address)
    pub token_out: String,
    /// Amount (human-readable, e.g., "100")
    pub amount: String,
    /// Quote direction: "sell" (default) or "buy"
    #[serde(default)]
    pub direction: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TransferInput {
    /// Private key (hex, with or without 0x prefix)
    pub private_key: String,
    /// Recipient address
    pub to: String,
    /// Amount to transfer (human-readable, e.g., "100")
    pub amount: String,
    /// Token symbol (TUSD, TEUR, TGBP) or address. Defaults to TUSD
    #[serde(default)]
    pub token: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SwapInput {
    /// Private key (hex, with or without 0x prefix)
    pub private_key: String,
    /// Token to sell (symbol or address)
    pub token_in: String,
    /// Token to buy (symbol or address)
    pub token_out: String,
    /// Amount to sell (human-readable, e.g., "100")
    pub amount_in: String,
    /// Minimum amount to receive (human-readable). Defaults to 0 (no slippage protection)
    #[serde(default)]
    pub min_amount_out: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FaucetInput {
    /// Private key (hex, with or without 0x prefix)
    pub private_key: String,
    /// Token symbol (TUSD, TEUR, TGBP) or address. Defaults to TUSD
    #[serde(default)]
    pub token: Option<String>,
}

// ============================================================================
// Tool Implementations
// ============================================================================

fn parse_private_key(key: &str) -> Result<[u8; 32]> {
    let key = key.trim_start_matches("0x");
    let bytes = hex::decode(key)?;
    if bytes.len() != 32 {
        anyhow::bail!("Private key must be 32 bytes");
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes);
    Ok(arr)
}

fn resolve_token(token: Option<&str>) -> Result<TokenInfo> {
    let token_str = token.unwrap_or("TUSD");

    if let Some(info) = get_token_by_symbol(token_str) {
        return Ok(info);
    }

    if token_str.starts_with("0x") && token_str.len() == 42 {
        let address: Address = token_str.parse()?;
        if let Some(info) = get_token_by_address(address) {
            return Ok(info);
        }
        return Ok(TokenInfo {
            address,
            symbol: "UNKNOWN".to_string(),
            decimals: 6,
        });
    }

    anyhow::bail!(
        "Unknown token: {}. Known tokens: TUSD, TEUR, TGBP",
        token_str
    )
}

pub async fn get_balance(client: &TempoClient, input: GetBalanceInput) -> Result<String> {
    let address: Address = input.address.parse()?;
    let token_info = resolve_token(input.token.as_deref())?;

    let balance = client.get_balance(address, token_info.address).await?;
    let formatted = format_token_amount(balance, token_info.decimals);

    Ok(format!(
        "Balance for {}:\n{} {} ({} raw units)",
        input.address, formatted, token_info.symbol, balance
    ))
}

pub async fn get_transaction(client: &TempoClient, input: GetTransactionInput) -> Result<String> {
    let hash: B256 = input.hash.parse()?;

    let tx = client
        .get_transaction(hash)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Transaction not found"))?;

    let receipt = client.get_transaction_receipt(hash).await?;

    let status = match &receipt {
        Some(r) => {
            if r.status == "0x1" {
                "Success"
            } else {
                "Failed"
            }
        }
        None => "Pending",
    };

    let gas_used = receipt
        .as_ref()
        .map(|r| r.gas_used.clone())
        .unwrap_or_else(|| "N/A".to_string());

    let block = tx
        .block_number
        .clone()
        .unwrap_or_else(|| "Pending".to_string());

    let to_addr = tx
        .to
        .map(|a| format!("{:?}", a))
        .unwrap_or_else(|| "Contract Creation".to_string());

    let log_count = receipt.as_ref().map(|r| r.logs.len()).unwrap_or(0);

    let mut result = format!(
        "Transaction: {}\n\
         Status: {}\n\
         From: {:?}\n\
         To: {}\n\
         Value: {} wei\n\
         Gas Used: {}\n\
         Block: {}",
        input.hash, status, tx.from, to_addr, tx.value, gas_used, block
    );

    if log_count > 0 {
        result.push_str(&format!("\n\nEvents: {} log(s) emitted", log_count));
    }

    Ok(result)
}

pub async fn decode_transaction(
    client: &TempoClient,
    input: GetTransactionInput,
) -> Result<String> {
    let hash: B256 = input.hash.parse()?;

    let tx = client
        .get_transaction(hash)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Transaction not found"))?;

    let receipt = client
        .get_transaction_receipt(hash)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Transaction still pending"))?;

    let status = if receipt.status == "0x1" {
        "Successful"
    } else {
        "Failed"
    };
    let to_addr = tx
        .to
        .map(|a| format!("{:?}", a))
        .unwrap_or_else(|| "Contract Creation".to_string());

    let mut actions: Vec<String> = Vec::new();

    // Transfer event topic
    let transfer_topic: B256 =
        "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".parse()?;

    for log in &receipt.logs {
        if !log.topics.is_empty() && log.topics[0] == transfer_topic && log.topics.len() >= 3 {
            let from = Address::from_slice(&log.topics[1].as_slice()[12..]);
            let to = Address::from_slice(&log.topics[2].as_slice()[12..]);

            let data_bytes = hex::decode(log.data.trim_start_matches("0x")).unwrap_or_default();
            let amount = if data_bytes.len() >= 32 {
                U256::from_be_slice(&data_bytes[..32])
            } else {
                U256::ZERO
            };

            let token_info = get_token_by_address(log.address).unwrap_or(TokenInfo {
                address: log.address,
                symbol: "tokens".to_string(),
                decimals: 6,
            });

            let formatted = format_token_amount(amount, token_info.decimals);
            actions.push(format!(
                "Transferred {} {} from {:?} to {:?}",
                formatted, token_info.symbol, from, to
            ));
        }
    }

    let mut result = format!(
        "Transaction Analysis: {}\n\
         Status: {}\n\
         From: {:?}\n\
         To: {}\n\n",
        input.hash, status, tx.from, to_addr
    );

    if !actions.is_empty() {
        result.push_str("Actions:\n");
        for (i, action) in actions.iter().enumerate() {
            result.push_str(&format!("  {}. {}\n", i + 1, action));
        }
    } else if !tx.value.is_zero() {
        result.push_str(&format!(
            "Actions:\n  1. Sent {} wei to {}\n",
            tx.value, to_addr
        ));
    } else {
        result
            .push_str("Actions:\n  Contract interaction (specific action could not be decoded)\n");
    }

    Ok(result)
}

pub async fn get_dex_quote(client: &TempoClient, input: GetDexQuoteInput) -> Result<String> {
    let token_in_info = resolve_token(Some(&input.token_in))?;
    let token_out_info = resolve_token(Some(&input.token_out))?;
    let direction = input.direction.as_deref().unwrap_or("sell");

    if direction == "buy" {
        let amount_out = parse_token_amount(&input.amount, token_out_info.decimals)?;
        let quote = client
            .get_dex_quote_buy(
                token_in_info.address,
                token_out_info.address,
                amount_out.try_into()?,
            )
            .await?;
        let formatted = format_token_amount(U256::from(quote), token_in_info.decimals);

        Ok(format!(
            "Buy Quote:\nTo buy {} {}, you need {} {}",
            input.amount, token_out_info.symbol, formatted, token_in_info.symbol
        ))
    } else {
        let amount_in = parse_token_amount(&input.amount, token_in_info.decimals)?;
        let quote = client
            .get_dex_quote_sell(
                token_in_info.address,
                token_out_info.address,
                amount_in.try_into()?,
            )
            .await?;
        let formatted = format_token_amount(U256::from(quote), token_out_info.decimals);

        Ok(format!(
            "Sell Quote:\nSelling {} {} will get you {} {}",
            input.amount, token_in_info.symbol, formatted, token_out_info.symbol
        ))
    }
}

pub fn list_tokens() -> String {
    let mut result = String::from("Known Tempo Tokens:\n\n");

    for token in known_tokens() {
        result.push_str(&format!(
            "{}:\n  Address: {:?}\n  Decimals: {}\n\n",
            token.symbol, token.address, token.decimals
        ));
    }

    result.push_str(&format!(
        "Network: Tempo Testnet\nChain ID: {}\nExplorer: https://explorer.testnet.tempo.xyz",
        crate::tempo::TEMPO_TESTNET_CHAIN_ID
    ));

    result
}

// ============================================================================
// Write Tool Implementations
// ============================================================================

pub async fn transfer(client: &TempoClient, input: TransferInput) -> Result<String> {
    let private_key = parse_private_key(&input.private_key)?;
    let to: Address = input.to.parse()?;
    let token_info = resolve_token(input.token.as_deref())?;
    let amount = parse_token_amount(&input.amount, token_info.decimals)?;

    let from = TempoClient::get_address_from_private_key(&private_key)?;
    let tx_hash = client
        .transfer(&private_key, token_info.address, to, amount)
        .await?;

    Ok(format!(
        "Transfer submitted!\n\
         From: {:?}\n\
         To: {:?}\n\
         Amount: {} {}\n\
         Transaction: {:?}\n\
         Explorer: https://explorer.testnet.tempo.xyz/tx/{:?}",
        from, to, input.amount, token_info.symbol, tx_hash, tx_hash
    ))
}

pub async fn swap(client: &TempoClient, input: SwapInput) -> Result<String> {
    let private_key = parse_private_key(&input.private_key)?;
    let token_in_info = resolve_token(Some(&input.token_in))?;
    let token_out_info = resolve_token(Some(&input.token_out))?;
    let amount_in = parse_token_amount(&input.amount_in, token_in_info.decimals)?;
    let min_out = input
        .min_amount_out
        .as_ref()
        .map(|s| parse_token_amount(s, token_out_info.decimals))
        .transpose()?
        .unwrap_or(U256::ZERO);

    let from = TempoClient::get_address_from_private_key(&private_key)?;
    let tx_hash = client
        .swap(
            &private_key,
            token_in_info.address,
            token_out_info.address,
            amount_in.try_into()?,
            min_out.try_into()?,
        )
        .await?;

    Ok(format!(
        "Swap submitted!\n\
         From: {:?}\n\
         Selling: {} {}\n\
         Buying: {}\n\
         Transaction: {:?}\n\
         Explorer: https://explorer.testnet.tempo.xyz/tx/{:?}",
        from, input.amount_in, token_in_info.symbol, token_out_info.symbol, tx_hash, tx_hash
    ))
}

pub async fn faucet(client: &TempoClient, input: FaucetInput) -> Result<String> {
    let private_key = parse_private_key(&input.private_key)?;
    let token_info = resolve_token(input.token.as_deref())?;

    let from = TempoClient::get_address_from_private_key(&private_key)?;
    let tx_hash = client.faucet(&private_key, token_info.address).await?;

    Ok(format!(
        "Faucet request submitted!\n\
         Address: {:?}\n\
         Token: {}\n\
         Transaction: {:?}\n\
         Explorer: https://explorer.testnet.tempo.xyz/tx/{:?}",
        from, token_info.symbol, tx_hash, tx_hash
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_private_key_with_prefix() {
        let key = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let result = parse_private_key(key);
        assert!(result.is_ok());
        assert_eq!(result.unwrap()[0], 0x01);
    }

    #[test]
    fn test_parse_private_key_without_prefix() {
        let key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let result = parse_private_key(key);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_private_key_invalid_length() {
        let key = "0123456789abcdef"; // too short
        let result = parse_private_key(key);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_token_by_symbol() {
        let result = resolve_token(Some("TUSD"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().symbol, "TUSD");
    }

    #[test]
    fn test_resolve_token_case_insensitive() {
        let result = resolve_token(Some("tusd"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().symbol, "TUSD");
    }

    #[test]
    fn test_resolve_token_default() {
        let result = resolve_token(None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().symbol, "TUSD");
    }

    #[test]
    fn test_resolve_token_by_address() {
        let result = resolve_token(Some("0x20C0000000000000000000000000000000000000"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().symbol, "TUSD");
    }

    #[test]
    fn test_resolve_token_unknown_address() {
        let result = resolve_token(Some("0x1234567890123456789012345678901234567890"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().symbol, "UNKNOWN");
    }

    #[test]
    fn test_resolve_token_invalid() {
        let result = resolve_token(Some("INVALID_TOKEN"));
        assert!(result.is_err());
    }

    #[test]
    fn test_list_tokens() {
        let result = list_tokens();
        assert!(result.contains("TUSD"));
        assert!(result.contains("TEUR"));
        assert!(result.contains("TGBP"));
        assert!(result.contains("Tempo Testnet"));
    }
}
