use anyhow::Error;
use serde_json::Value;

use crate::utils::constants::{BASE_FEE, TOKEN_PRICE};
use crate::utils::types::TxType;

pub fn cost_calculator(tx_type: TxType, mut data_size: u64) -> Result<f32, Error> {
    let gas_price = TxType::gas_price(tx_type.clone());
    // byte size * 2 -> hex size
    data_size *= 2;
    let base_fee_amount = BASE_FEE * (TxType::base_fee_count(tx_type, data_size));
    // convert to gas amount, base tx fee is 500_000 gas
    let gas_amount = data_size * gas_price + base_fee_amount;
    let token_amount = gas_amount as f32 * 1e-9;
    Ok(token_amount * TOKEN_PRICE)
}

pub async fn get_token_price(ticker: &str) -> Result<f64, Error> {
    let url: String = format!("https://api.redstone.finance/prices/?symbol={}&provider=redstone&limit=1", ticker)
        .parse()
        .unwrap();
    let req: Value = reqwest::get(url).await?.json().await?;
    let res: f64 = req.get(0).unwrap().get("value").unwrap().as_f64().unwrap();
    Ok(res)
}