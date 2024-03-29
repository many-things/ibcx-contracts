use cosmwasm_std::Addr;
use error::ContractError;

#[cfg(not(feature = "library"))]
pub mod contract;
pub mod error;
pub mod execute;
pub mod query;
pub mod state;

pub type StdResult<T> = Result<T, ContractError>;

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const REPLY_ID_DENOM_CREATION: u64 = 0;

#[cfg(test)]
mod test;

pub fn assert_sender(expected: &Addr, actual: &Addr) -> StdResult<()> {
    if expected != actual {
        Err(ContractError::Unauthorized {})
    } else {
        Ok(())
    }
}
