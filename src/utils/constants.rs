pub const BASE_FEE: u64 = 500_000;
pub const TOKEN_PRICE: f64 = 12.5; // dummy hardcoded, will be dynamic once $LOAD is tradeable
pub const ONE_MB: u64 = 1 * 1024 * 1024; // 1_048_576 bytes
pub const FOUR_MB: u64 = 4 * 1024 * 1024; // 4_194_304 bytes
pub const EIGHT_MB: u64 = 8 * 1024 * 1024; // 8_388_608 bytes
pub const CELESTIA_TX_SIZE_LIMIT: u64 = 2 * ONE_MB; // 2_097_152 bytes
pub const CELESTIA_FIXED_COST: u64 = 65_000; // 65k gas -- base tx fee
pub const CELESTIA_GAS_PER_BYTE: u64 = 8;
