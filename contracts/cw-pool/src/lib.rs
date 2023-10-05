#[cfg(not(feature = "library"))]
pub mod contract;
pub mod error;

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

type StdResult<T> = Result<T, error::ContractError>;
