#[cfg(not(feature = "library"))]
pub mod contract;
pub mod error;
pub mod execute;
pub mod query;
pub mod state;

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const REPLY_ID_DENOM_CREATION: u64 = 0;

#[cfg(test)]
mod test {
    pub const SENDER_OWNER: &str = "owner";
    pub const SENDER_GOV: &str = "gov";
    pub const SENDER_ABUSER: &str = "abuser";
    pub const SENDER_VALID: &str = "osmo10yaagy0faggta0085hkzr3ckq7p7z9996nrn0m";

    pub const DENOM_DEFAULT: &str = "uibcx";
    pub const DENOM_RESERVE: &str = "uosmo";
}
