use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Addr, Coin, CosmosMsg, Uint128};
use cw_storage_plus::{Item, Map};
use ibc_interface::types::SwapRoutes;
use osmosis_std::types::osmosis::gamm::v1beta1::{MsgSwapExactAmountIn, MsgSwapExactAmountOut};

use crate::error::ContractError;

pub const LATEST_REBALANCE_ID_KEY: &str = "latest_rebalance_id";
pub const LATEST_REBALANCE_ID: Item<u64> = Item::new(LATEST_REBALANCE_ID_KEY);

pub const REBALANCES_PREFIX: &str = "rebalances";
pub const REBALANCES: Map<u64, Uint128> = Map::new(REBALANCES_PREFIX);

pub const TRADE_INFOS_PREFIX: &str = "trade_infos";
pub const TRADE_INFOS: Map<String, TradeInfo> = Map::new(TRADE_INFOS_PREFIX);

#[cw_serde]
pub struct Rebalance {
    pub manager: Addr,
    pub snapshot: Vec<Coin>,
    pub deflation: Vec<Coin>,
    pub inflation: Vec<Coin>,
    pub finalized: bool,
}

impl Rebalance {
    pub fn assert_basic(&self, manager: &Addr) -> Result<(), ContractError> {
        if &self.manager != manager {
            return Err(ContractError::Unauthorized {});
        }
        if self.finalized {
            return Err(ContractError::Finalized {});
        }

        Ok(())
    }
}

#[cw_serde]
pub struct TradeInfo {
    pub routes: SwapRoutes,
    pub cooldown: u64,
    pub last_traded_at: Option<u64>,
}

impl TradeInfo {
    pub fn checked_update_cooldown(&mut self, now: u64) -> Result<(), ContractError> {
        if let Some(last_trade_time) = self.last_traded_at {
            if now < last_trade_time + self.cooldown {
                return Err(ContractError::CooldownNotExpired {});
            }
        }

        self.last_traded_at = Some(now);

        Ok(())
    }

    pub fn swap_exact_amount_in(
        &self,
        sender: &Addr,
        token_in: &str,
        token_in_amount: Uint128,
        token_out_min: Uint128,
    ) -> CosmosMsg {
        MsgSwapExactAmountIn {
            sender: sender.to_string(),
            routes: self.routes.clone().into(),
            token_in: Some(coin(token_in_amount.u128(), token_in).into()),
            token_out_min_amount: token_out_min.to_string(),
        }
        .into()
    }

    pub fn swap_exact_amount_out(
        &self,
        sender: &Addr,
        token_out: &str,
        token_out_amount: Uint128,
        token_in_max: Uint128,
    ) -> CosmosMsg {
        MsgSwapExactAmountOut {
            sender: sender.to_string(),
            routes: self.routes.clone().into(),
            token_in_max_amount: token_in_max.to_string(),
            token_out: Some(coin(token_out_amount.u128(), token_out).into()),
        }
        .into()
    }
}
