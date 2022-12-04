pub mod assets;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Env, StdResult, Storage, Uint128};
use cw_storage_plus::{Item, Map};

use crate::error::ContractError;
pub use crate::state::assets::{assert_assets, get_assets, get_redeem_amounts, set_assets};

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
                }
            }
        }

        Ok(self)
    }

    pub fn assert_paused(self) -> Result<Self, ContractError> {
        if self.paused {
            return Err(ContractError::Paused {});
        }

        Ok(self)
    }

    pub fn assert_not_paused(self) -> Result<Self, ContractError> {
        if !self.paused {
            return Err(ContractError::NotPaused {});
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

pub const GOV: Item<Addr> = Item::new("gov");
pub const TOKEN: Item<Token> = Item::new("token");

pub const RESERVE_DENOM: &str = "reserve";
pub const ASSETS: Map<String, Uint128> = Map::new("assets");
pub const PAUSED: Item<PauseInfo> = Item::new("paused");
