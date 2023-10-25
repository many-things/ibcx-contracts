use std::cmp::Ordering;

use cosmwasm_std::{coin, Coin, Decimal, Uint128};
use error::ContractError;

#[cfg(not(feature = "library"))]
pub mod contract;
pub mod error;
pub mod execute;
pub mod query;

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const REPLY_ID_BURN_EXACT_AMOUNT_IN: u64 = 0;

pub fn deduct_fee(rate: Option<Decimal>) -> Result<Decimal, ContractError> {
    Ok(rate
        .map(|v| Ok::<_, ContractError>(Decimal::one().checked_sub(v)?))
        .transpose()?
        .unwrap_or(Decimal::one()))
}

pub fn expand_fee(rate: Option<Decimal>) -> Result<Decimal, ContractError> {
    Ok(rate
        .map(|v| {
            Ok::<_, ContractError>(Decimal::one().checked_div(Decimal::one().checked_sub(v)?)?)
        })
        .transpose()?
        .unwrap_or(Decimal::one()))
}

pub fn make_unit_converter(v: Uint128) -> Box<dyn Fn((String, Decimal)) -> Coin> {
    Box::new(move |(denom, unit)| coin((v * unit).u128(), denom))
}

pub fn coin_sorter(a: &Coin, b: &Coin) -> Ordering {
    a.denom.cmp(&b.denom)
}
