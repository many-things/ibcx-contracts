use cosmwasm_schema::serde::Serialize;
use cosmwasm_std::{to_vec, Binary, QueryResponse};
use error::ContractError;

pub mod airdrop;
pub mod contract;
pub mod error;
pub mod execute;
pub mod query;
pub mod state;
pub mod verify;

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn to_binary<T>(data: &T) -> Result<QueryResponse, ContractError>
where
    T: Serialize + ?Sized,
{
    Ok(to_vec(data).map(Binary)?)
}
