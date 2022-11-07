use std::collections::BTreeMap;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Addr, Coin, Env, StdError, StdResult, Storage, Uint128};
use cw_storage_plus::{Item, Map};
use ibc_interface::types::SwapRoute;
use osmosis_std::types::osmosis::gamm::v1beta1::SwapAmountInRoute;

use crate::error::ContractError;

#[cw_serde]
pub struct Config {
    pub gov: Addr,
    pub denom: String,
    pub reserve_denom: String,
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
pub struct State {
    pub assets: BTreeMap<String, Uint128>, // denom -> unit
    pub total_reserve: Uint128,
    pub total_supply: Uint128,
}

impl State {
    pub fn new(asset_vec: Vec<(String, Uint128)>) -> StdResult<Self> {
        let mut assets = BTreeMap::new();
        for (denom, unit) in asset_vec {
            let res = assets.insert(denom, unit);
            if res.is_some() {
                return Err(StdError::generic_err("duplicate denom"));
            }
        }

        Ok(Self {
            assets,
            total_reserve: Uint128::zero(),
            total_supply: Uint128::zero(),
        })
    }

    pub fn assert_funds(
        &self,
        funds: BTreeMap<String, Uint128>,
        reserve: &str,
        desired: &Uint128,
    ) -> Result<(), ContractError> {
        let mut assets = self.assets.clone();

        let reserve_unit = self.total_reserve.checked_div(self.total_supply)?;
        assets
            .entry(reserve.to_string())
            .and_modify(|v| *v += reserve_unit)
            .or_insert(reserve_unit);

        for (denom, unit) in &self.assets {
            let required = unit * desired;
            let received = match funds.get(denom) {
                Some(r) => r,
                None => {
                    return Err(ContractError::PaymentError(
                        cw_utils::PaymentError::NoFunds {},
                    ))
                }
            };

            if required != received {
                return Err(ContractError::MismatchedFunds {
                    denom: denom.clone(),
                    required,
                    received: received.clone(),
                });
            }
        }

        Ok(())
    }

    pub fn calc_redeem_amount(&self, reserve: &str, desired: Uint128) -> StdResult<Vec<Coin>> {
        let mut assets = self.assets.clone();

        let reserve_unit = self.total_reserve.checked_div(self.total_supply)?;
        assets
            .entry(reserve.to_string())
            .and_modify(|v| *v += reserve_unit)
            .or_insert(reserve_unit);

        Ok(assets
            .iter()
            .map(|(denom, unit)| coin((unit * desired).u128(), denom.clone()))
            .collect())
    }
}

#[cw_serde]
pub struct RebalanceInfo {
    pub manager: Addr,
    pub from: BTreeMap<String, Uint128>,         // denom -> unit
    pub deflation: BTreeMap<String, Uint128>,    // denom -> unit contraction
    pub amortization: BTreeMap<String, Uint128>, // denom -> weight
    pub finished: bool,
}

impl RebalanceInfo {
    pub fn new(
        manager: Addr,
        from: BTreeMap<String, Uint128>,
        deflation_vec: Vec<(String, Uint128)>,
        amortization_vec: Vec<(String, Uint128)>,
    ) -> StdResult<Self> {
        let mut deflation = BTreeMap::new();
        for (denom, unit) in deflation_vec {
            let res = deflation.insert(denom, unit);
            if res.is_some() {
                return Err(StdError::generic_err("duplicate denom"));
            }
        }

        let mut amortization = BTreeMap::new();
        for (denom, unit) in amortization_vec {
            let res = amortization.insert(denom, unit);
            if res.is_some() {
                return Err(StdError::generic_err("duplicate denom"));
            }
        }

        Ok(Self {
            manager,
            from,
            deflation,
            amortization,
            finished: false,
        })
    }
}

#[cw_serde]
pub struct TradeStrategy {
    pub routes: Vec<SwapRoute>, // token > ... routes ... > reserve token
    pub cool_down: Option<u64>,
    pub max_trade_amount: Uint128,
    pub last_traded_at: u64,
}

impl TradeStrategy {
    pub fn validate(&self, reserve_denom: &str) -> StdResult<()> {
        if self.routes.is_empty() {
            return Err(StdError::generic_err("route is empty"));
        }

        if self.routes.last().unwrap().token_denom != reserve_denom {
            return Err(StdError::generic_err(
                "swap dest should same as reserve denom",
            ));
        }

        Ok(())
    }

    // token -> reserve
    pub fn route_sell(&self) -> Vec<SwapAmountInRoute> {
        self.routes
            .iter()
            .map(|v| SwapAmountInRoute {
                pool_id: v.pool_id,
                token_out_denom: v.token_denom.clone(),
            })
            .collect()
    }

    // reserve -> token
    pub fn route_buy(&self) -> Vec<SwapAmountInRoute> {
        let mut routes = self.routes.clone();

        routes.reverse();

        routes
            .iter()
            .map(|v| SwapAmountInRoute {
                pool_id: v.pool_id,
                token_out_denom: v.token_denom.clone(),
            })
            .collect()
    }
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const PAUSED: Item<PauseInfo> = Item::new("paused");
pub const STATE: Item<State> = Item::new("portfolio");

pub const REBALANCE_LATEST_ID: Item<u64> = Item::new("rebalance:latest");
pub const REBALANCES: Map<u64, RebalanceInfo> = Map::new("rebalances");

pub const TRADE_TOTAL_ALLOCATION: Item<Uint128> = Item::new("trade:total_allocation");
pub const TRADE_ALLOCATIONS: Map<&str, Uint128> = Map::new("trade:allocation");
pub const TRADE_STRATEGIES: Map<&str, TradeStrategy> = Map::new("trade:strategy");
