use crate::utils::server::{
    get_price_avail, get_price_babe, get_price_baselayer, get_price_celestia, root,
};

use axum::{
    Router,
    http::{Method, header},
    routing::get,
};
use std::time::Duration;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;

pub mod utils;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let timeout_layer = TimeoutLayer::new(Duration::from_secs(10));

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT]);

    // server routes
    let router = Router::new()
        .route("/", get(root))
        .route("/v1", get(root))
        // v1 routes
        // load network
        .route("/v1/baselayer/:size", get(get_price_baselayer))
        // load network
        .route("/v1/babe/:size", get(get_price_babe))
        .route("/v1/celestia/:size", get(get_price_celestia))
        .route("/v1/avail/:size", get(get_price_avail))
        .layer(timeout_layer)
        .layer(cors);

    Ok(router.into())
}
