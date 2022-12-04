pub mod helpers;

pub mod airdrop;
pub mod core;
pub mod faucet;
pub mod periphery;
pub mod types;

use cosmwasm_std::{StdError, StdResult};

// Settings for pagination
pub const MAX_LIMIT: u32 = 30;
pub const DEFAULT_LIMIT: u32 = 10;

pub fn get_and_check_limit(limit: Option<u32>, max: u32, default: u32) -> StdResult<u32> {
    match limit {
        Some(l) => {
            if l <= max {
                Ok(l)
            } else {
                Err(StdError::generic_err(format!(
                    "oversized request. size: {:?}, max: {:?}",
                    l as u64, max as u64,
                )))
            }
        }
        None => Ok(default),
    }
}
