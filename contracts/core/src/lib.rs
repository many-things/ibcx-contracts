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
    use cosmwasm_std::{Addr, Uint128};

    use crate::state;

    pub const SENDER_OWNER: &str = "owner";
    pub const SENDER_GOV: &str = "gov";
    pub const SENDER_ABUSER: &str = "abuser";
    pub const SENDER_VALID: &str = "osmo10yaagy0faggta0085hkzr3ckq7p7z9996nrn0m";

    pub const DENOM_DEFAULT: &str = "uibcx";
    pub const DENOM_RESERVE: &str = "uosmo";

    pub fn default_fee() -> state::Fee {
        state::Fee {
            collector: Addr::unchecked("collector"),
            mint: Default::default(),
            burn: Default::default(),
            stream: Default::default(),
            stream_last_collected_at: Default::default(),
        }
    }

    pub fn default_token() -> state::Token {
        state::Token {
            denom: DENOM_DEFAULT.to_string(),
            reserve_denom: DENOM_RESERVE.to_string(),
            total_supply: Uint128::new(100),
        }
    }
}
