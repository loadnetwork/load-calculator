use anyhow::Error;

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
