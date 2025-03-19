
use axum::{routing::get, Router};
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;
use crate::utils::storage_calculator::cost_calculator;
use crate::utils::types::TxType;
use crate::utils::server::{get_price_babe, root, get_price_baselayer};

pub mod utils;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let timeout_layer = TimeoutLayer::new(Duration::from_secs(10));
    let cost = cost_calculator(TxType::Babe2, 4216212).unwrap();
    println!("{:?}", cost);
    
    // server routes
    let router = Router::new()
        .route("/", get(root))
        .route("/v1", get(root))
        // v1 routes
        .route("/v1/baselayer/:size", get(get_price_baselayer))
        .route("/v1/babe/:size", get(get_price_babe))
        .layer(timeout_layer);

    Ok(router.into())
}