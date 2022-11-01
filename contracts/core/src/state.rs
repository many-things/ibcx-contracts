use std::collections::BTreeMap;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Env, MessageInfo, StdResult, Storage, Uint128};
use cw_storage_plus::{Item, Map};
use ibc_interface::core::SwapRoute;

use crate::error::ContractError;

#[cw_serde]
pub struct Config {
    pub gov: Addr,
    pub denom: String,
    pub reserve_denom: String,
    pub assets: BTreeMap<String, Uint128>,
}

impl Config {
    pub fn assert_funds(&self, info: &MessageInfo, desired: &Uint128) -> Result<(), ContractError> {
        for (denom, unit) in &self.assets {
            let required = unit * desired;
            let received = cw_utils::must_pay(&info, &denom)?;
            if required != received {
                return Err(ContractError::MismatchedFunds {
                    denom: denom.clone(),
                    required,
                    received,
                });
            }
        }

        Ok(())
    }
}

#[cw_serde]
pub struct PauseInfo {
    pub paused: bool,
    pub expires_at: Option<u64>,
}

impl Default for PauseInfo {
    fn default() -> Self {
        Self {
            paused: false,
            expires_at: None,
        }
    }
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
pub struct Rebalance {
    pub manager: Addr,
    pub prev_assets: BTreeMap<String, Uint128>, // denom -> unit
    pub deflation: BTreeMap<String, Uint128>,   // denom -> unit contraction
    pub inflation: BTreeMap<String, Uint128>,   // denom -> weight
    pub buffer: BTreeMap<String, Uint128>,
}

#[cw_serde]
pub struct TradeStrategy {
    pub routes: Vec<SwapRoute>,
    pub max_trade_amount: Uint128,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const PAUSED: Item<PauseInfo> = Item::new("paused");
pub const REBALANCE: Item<Rebalance> = Item::new("rebalance");
pub const TRADE_STRATEGY: Map<&str, TradeStrategy> = Map::new("trade:strategy");
