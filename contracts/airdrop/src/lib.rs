pub mod airdrop;
pub mod contract;
pub mod error;
pub mod execute;
pub mod query;
pub mod state;
pub mod verify;

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
