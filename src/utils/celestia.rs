use crate::utils::constants::{CELESTIA_FIXED_COST, CELESTIA_GAS_PER_BYTE, CELESTIA_TX_SIZE_LIMIT};
use anyhow::Error;
use reqwest;
use serde_json::Value;

pub async fn get_gas_price() -> Result<f64, Error> {
    let url: String = "https://api.celenium.io/v1/gas/price".parse().unwrap();
    let req: Value = reqwest::get(url).await?.json().await?;
    let res = req
        .get("median")
        .unwrap()
        .as_str()
        .unwrap()
        .parse::<f64>()
        .unwrap();
    Ok(res)
}

pub async fn get_tia_price() -> Result<f64, Error> {
    let url: String = "https://api.redstone.finance/prices/?symbol=TIA&provider=redstone&limit=1"
        .parse()
        .unwrap();
    let req: Value = reqwest::get(url).await?.json().await?;
    let res: f64 = req.get(0).unwrap().get("value").unwrap().as_f64().unwrap();
    Ok(res)
}

pub async fn cost_calculator(data_size: u64) -> Result<f64, Error> {
    let gas_price = get_gas_price().await?;
    let tia_price = get_tia_price().await?;
    let fixed_cost_count = (data_size + CELESTIA_TX_SIZE_LIMIT - 1) / CELESTIA_TX_SIZE_LIMIT;
    let total_gas = data_size + (fixed_cost_count * CELESTIA_FIXED_COST);
    let cost = total_gas as f64 * CELESTIA_GAS_PER_BYTE as f64 * gas_price * 1e-6 * tia_price;
    Ok(cost)
}
