use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Addr, CosmosMsg, Decimal, QuerierWrapper, StdResult, Uint128};
use cw_storage_plus::{Item, Map};
use ibc_interface::types::SwapRoutes;
use osmosis_std::types::osmosis::gamm::v1beta1::{
    MsgSwapExactAmountIn, MsgSwapExactAmountOut, QuerySwapExactAmountInRequest,
    QuerySwapExactAmountOutRequest,
};

use crate::error::ContractError;

pub const LATEST_REBALANCE_ID_KEY: &str = "latest_rebalance_id";
pub const LATEST_REBALANCE_ID: Item<u64> = Item::new(LATEST_REBALANCE_ID_KEY);

pub const REBALANCES_PREFIX: &str = "rebalances";
pub const REBALANCES: Map<u64, Rebalance> = Map::new(REBALANCES_PREFIX);

pub const TRADE_INFOS_PREFIX: &str = "trade_infos";
pub const TRADE_INFOS: Map<String, TradeInfo> = Map::new(TRADE_INFOS_PREFIX);

pub const RESERVE_BUFFER_PREFIX: &str = "reserve_buffer";
pub const RESERVE_BUFFER: Map<String, Uint128> = Map::new(RESERVE_BUFFER_PREFIX);

#[cw_serde]
pub struct Rebalance {
    pub manager: Addr,
    pub deflation: Vec<(String, Decimal)>,
    pub inflation: Vec<(String, Decimal)>,
    pub finalized: bool,
}

impl Rebalance {
    pub fn validate(&self, assets: Vec<(String, Decimal)>) -> Result<(), ContractError> {
        // check current asset & deflation
        let f = assets
            .iter()
            .filter(|xc| self.deflation.iter().any(|yc| yc.0 == xc.0 && yc.1 > xc.1))
            .collect::<Vec<_>>();
        if !f.is_empty() {
            return Err(ContractError::InvalidArgument(format!(
                "cannot deflate non-portfolio asset: {:?}",
                f
            )));
        }

        // check duplication
        let mut y = self.deflation.iter();
        let f = self
            .inflation
            .iter()
            .filter(|xc| y.any(|yc| yc.0 == xc.0))
            .collect::<Vec<_>>();
        if !f.is_empty() {
            return Err(ContractError::InvalidArgument(format!(
                "duplicated coin: {:?}",
                f
            )));
        }

        Ok(())
    }
}

#[cw_serde]
pub struct TradeInfo {
    pub routes: SwapRoutes,
    pub cooldown: u64,
    pub max_trade_amount: Uint128,
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

    pub fn sim_swap_exact_amount_in(
        &self,
        querier: &QuerierWrapper,
        sender: &Addr,
        token_in: &str,
        token_in_amount: Uint128,
    ) -> StdResult<Uint128> {
        querier.query(
            &QuerySwapExactAmountInRequest {
                sender: sender.to_string(),
                routes: self.routes.clone().into(),
                token_in: coin(token_in_amount.u128(), token_in).to_string(),
                pool_id: self.routes.0[0].pool_id,
            }
            .into(),
        )
    }

    pub fn sim_swap_exact_amount_out(
        &self,
        querier: &QuerierWrapper,
        sender: &Addr,
        token_out: &str,
        token_out_amount: Uint128,
    ) -> StdResult<Uint128> {
        querier.query(
            &QuerySwapExactAmountOutRequest {
                sender: sender.to_string(),
                routes: self.routes.clone().into(),
                token_out: coin(token_out_amount.u128(), token_out).to_string(),
                pool_id: self.routes.0[0].pool_id,
            }
            .into(),
        )
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
