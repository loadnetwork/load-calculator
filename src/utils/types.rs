use crate::utils::constants::{EIGHT_MB, FOUR_MB};

#[derive(PartialEq, Clone)]
pub enum TxType {
    BaseLayer,
    Babe1,
    Babe2,
}

impl TxType {
    pub fn gas_price(self) -> u64 {
        match self {
            Self::BaseLayer => 8,
            Self::Babe1 | Self::Babe2 => 2,
        }
    }

    pub fn size_limit(self) -> u64 {
        match self {
            Self::BaseLayer => EIGHT_MB,          // base layer tx input
            Self::Babe1 | Self::Babe2 => FOUR_MB, // chunk size
        }
    }

    pub fn base_fee_count(self, data_size: u64) -> u64 {
        // how many baselayer tx we need to fit the data
        // regardless of type, data is splitted across baselayer txs
        // if type is 0xbabe, add 1 to count as we need a final tx to
        // collect all chunks receipts

        let mut count = (data_size + EIGHT_MB - 1) / EIGHT_MB;
        if self == Self::Babe1 || self == Self::Babe2 {
            count += 1;
        }
        count
    }
}
