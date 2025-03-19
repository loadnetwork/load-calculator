use crate::utils::server::{get_price_babe, get_price_baselayer, root};
use axum::{Router, routing::get};
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;

pub mod utils;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let timeout_layer = TimeoutLayer::new(Duration::from_secs(10));

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
