#[cfg(not(feature = "library"))]
pub mod contract;
pub mod error;
pub mod execute;
pub mod msgs;

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
