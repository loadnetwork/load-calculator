use crate::utils::storage_calculator::cost_calculator;
use axum::{extract::Path, response::Json};
use serde_json::{Value, json};

use super::types::TxType;

pub async fn root() -> Json<Value> {
    Json(json!({"status": "running"}))
}

pub async fn get_price_baselayer(Path(data_size): Path<u64>) -> Json<Value> {
    let cost = cost_calculator(TxType::BaseLayer, data_size).unwrap();
    Json(json!({"tx_type": "base_layer", "data_size": data_size, "cost_in_usd": cost}))
}

pub async fn get_price_babe(Path(data_size): Path<u64>) -> Json<Value> {
    let cost = cost_calculator(TxType::Babe2, data_size).unwrap();
    Json(json!({"tx_type": "0xbabe", "data_size": data_size, "cost_in_usd": cost}))
}
