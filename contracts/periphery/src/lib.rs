#[cfg(not(feature = "library"))]
pub mod contract;
pub mod error;
pub mod state;

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const REPLY_ID_MINT: u64 = 0;
pub const REPLY_ID_BURN: u64 = 1;
