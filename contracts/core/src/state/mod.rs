mod assets;
mod rebalance;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Env, StdResult, Storage, Uint128};
use cw_storage_plus::Item;

use crate::error::ContractError;

pub use crate::state::assets::{assert_assets, get_assets, get_redeem_amounts, set_assets, ASSETS};
pub use crate::state::rebalance::{
    Rebalance, TradeInfo, LATEST_REBALANCE_ID, REBALANCES, RESERVE_BUFFER, TRADE_INFOS,
};

pub const RESERVE_DENOM: &str = "reserve";

pub const GOV_KEY: &str = "gov";
pub const GOV: Item<Addr> = Item::new(GOV_KEY);

pub const TOKEN_KEY: &str = "token";
pub const TOKEN: Item<Token> = Item::new(TOKEN_KEY);

pub const PAUSED_KEY: &str = "paused";
pub const PAUSED: Item<PauseInfo> = Item::new(PAUSED_KEY);

#[cw_serde]
#[derive(Default)]
pub struct PauseInfo {
    pub paused: bool,
    pub expires_at: Option<u64>,
}

impl PauseInfo {
    pub fn refresh(self, storage: &mut dyn Storage, env: &Env) -> StdResult<Self> {
        if self.paused {
            if let Some(expiry) = self.expires_at {
                if expiry <= env.block.time.seconds() {
                    PAUSED.save(storage, &Default::default())?;
                    return Ok(Default::default());
                }
            }
        }

        Ok(self)
    }

    pub fn assert_paused(self) -> Result<Self, ContractError> {
        if !self.paused {
            return Err(ContractError::NotPaused {});
        }

        Ok(self)
    }

    pub fn assert_not_paused(self) -> Result<Self, ContractError> {
        if self.paused {
            return Err(ContractError::Paused {});
        }

        Ok(self)
    }
}

#[cw_serde]
pub struct Token {
    pub denom: String,
    pub decimal: u8,
    pub reserve_denom: String,
    pub total_supply: Uint128,
}
