use crate::utils::constants::ONE_MB;
use anyhow::Error;
use avail_rust::prelude::*;
use serde_json::Value;

pub async fn get_avail_price() -> Result<f64, Error> {
    let url: String = "https://calculator.availproject.org/api/price"
        .parse()
        .unwrap();
    let req: Value = reqwest::get(url).await?.json().await?;
    let price = req.get("price").unwrap().as_f64().unwrap();
    Ok(price)
}

pub async fn estimate_cost(client: SDK, avail_price: f64, data_size: u64) -> Result<f64, Error> {
    let account = account::alice();

    // data Submission to app id 1
    let data = String::from("l".repeat(data_size as usize)).into_bytes();
    let options = Options::new().app_id(1);
    let tx = client.tx.data_availability.submit_data(data);
    let data = tx
        .payment_query_info(&account, Some(options))
        .await
        .unwrap();
    let fee = data.partial_fee as f64;

    Ok(fee * 1e-18 * avail_price)
}

pub async fn cost_calculator(data_size: u64) -> Result<f64, Error> {
    let avail_price = get_avail_price().await.unwrap();
    let client: SDK = SDK::new("wss://mainnet-rpc.avail.so/ws").await.unwrap();

    if data_size <= ONE_MB {
        let cost = estimate_cost(client, avail_price, data_size).await?;
        Ok(cost)
    } else {
        // get how many 1MB blobs in the data size
        let blobs_count = data_size / ONE_MB;
        // get the remainder < 1MB blob size
        let remainder = data_size - (blobs_count * ONE_MB);
        // get cost of 1MB blob
        let one_mb_blobs_cost = estimate_cost(client.clone(), avail_price, ONE_MB).await?;
        // total 1MB blobs cost
        let blobs_count_cost = blobs_count as f64 * one_mb_blobs_cost;
        // get the remainder data cost
        let remainder_cost = estimate_cost(client, avail_price, remainder).await?;

        Ok(remainder_cost + blobs_count_cost)
    }
}
