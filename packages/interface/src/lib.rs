pub mod helpers;

pub mod airdrop;
pub mod core;
pub mod cw_pool;
pub mod periphery;
pub mod types;

use cosmwasm_std::{Order, StdError, StdResult};
use cw_storage_plus::Bound;
use types::RangeOrder;

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

type RangeOptionRespBound<'a, T> = Option<Bound<'a, T>>;
type RangeOptionResp<'a, T> = (
    (RangeOptionRespBound<'a, T>, RangeOptionRespBound<'a, T>),
    usize,
    Order,
);

pub fn range_option<'a, T: cw_storage_plus::PrimaryKey<'a>>(
    start: Option<T>,
    limit: Option<u32>,
    order: Option<RangeOrder>,
) -> StdResult<RangeOptionResp<'a, T>> {
    let limit = get_and_check_limit(limit, MAX_LIMIT, DEFAULT_LIMIT)? as usize;
    let order = order.unwrap_or(RangeOrder::Asc).into();
    let (min, max) = match order {
        Order::Ascending => (start.map(Bound::exclusive), None),
        Order::Descending => (None, start.map(Bound::exclusive)),
    };

    Ok(((min, max), limit, order))
}
