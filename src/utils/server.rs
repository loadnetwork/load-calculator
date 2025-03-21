use crate::utils::avail;
use crate::utils::celestia;
use crate::utils::storage_calculator::cost_calculator;
use crate::utils::types::TxType;
use axum::{extract::Path, response::Json};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ServerResponse {
    pub network: String,
    pub data_size: u64,
    pub cost_in_usd: f64,
    pub permanent_backup: bool,
}

impl ServerResponse {
    pub fn new(network: String, data_size: u64, cost_in_usd: f64, permanent_backup: bool) -> Self {
        Self {
            network,
            data_size,
            cost_in_usd,
            permanent_backup,
        }
    }
}

pub async fn root() -> Json<Value> {
    Json(json!({"status": "running"}))
}

pub async fn get_price_baselayer(Path(data_size): Path<u64>) -> Json<Value> {
    let cost = cost_calculator(TxType::BaseLayer, data_size).unwrap();
    let res = ServerResponse::new("load_network_baselayer".to_string(), data_size, cost, true);
    Json(json!(res))
}

pub async fn get_price_babe(Path(data_size): Path<u64>) -> Json<Value> {
    let cost = cost_calculator(TxType::Babe2, data_size).unwrap();
    let res = ServerResponse::new("load_network_0xbabe".to_string(), data_size, cost, true);
    Json(json!(res))
}

pub async fn get_price_celestia(Path(data_size): Path<u64>) -> Json<Value> {
    let cost = celestia::cost_calculator(data_size).await.unwrap();
    let res = ServerResponse::new("celestia_da".to_string(), data_size, cost, false);
    Json(json!(res))
}

pub async fn get_price_avail(Path(data_size): Path<u64>) -> Json<Value> {
    let cost = avail::cost_calculator(data_size).await.unwrap();
    let res = ServerResponse::new("avail_da".to_string(), data_size, cost, false);
    Json(json!(res))
}
