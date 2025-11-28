use serde::{Deserialize, Serialize};
use uint::construct_uint;
construct_uint! {
// consisting of 4 x 64-bit words
#[derive(Serialize, Deserialize)]
pub struct U256(4);
}
pub mod crypto;
pub mod error;
pub mod network;
pub mod sha256;
pub mod types;
pub mod util;

pub const DIFFICULTY_UPDATE_INTERVAL: u64 = 50;
// initial reward in bitcoin - multiply by 10^8 to get satoshis
pub const INITIAL_REWARD: u64 = 50;
// halving interval in blocks
pub const HALVING_INTERVAL: u64 = 210;
// ideal block time in seconds
pub const IDEAL_BLOCK_TIME: u64 = 10;
// minimum target
// In Bitcoin, the minimum target is defined as: 0x00000000FFFF0000000000000000000000000000000000000000000000000000
pub const MIN_TARGET: U256 = U256([
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0x0000_FFFF_FFFF_FFFF,
]);
// difficulty update interval in blocks

// maximum age of a transaction in the mempool in seconds -> btc 72h
pub const MAX_MEMPOOL_TRANSACTION_AGE: u64 = 600;
