use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, from_json, Addr, Coin, Empty, Order, StdResult, Uint128};
use cosmwasm_std::{CosmosMsg, QuerierWrapper};
use ibcx_utils::raw_query_bin;
use osmosis_std::types::osmosis::poolmanager::v1beta1::{
    EstimateSwapExactAmountOutRequest, EstimateSwapExactAmountOutResponse, MsgSwapExactAmountIn,
    MsgSwapExactAmountOut, PoolmanagerQuerier, SwapAmountInRoute, SwapAmountOutRoute,
};

#[cw_serde]
pub struct SwapRoute {
    pub pool_id: u64,
    pub token_denom: String,
}

impl SwapRoute {
    pub fn new(pool_id: u64, token_denom: &str) -> Self {
        Self {
            pool_id,
            token_denom: token_denom.to_string(),
        }
    }
}

#[cw_serde]
pub struct SwapRoutes(pub Vec<SwapRoute>);

impl From<Vec<SwapRoute>> for SwapRoutes {
    fn from(val: Vec<SwapRoute>) -> Self {
        Self(val)
    }
}

impl<'a> From<Vec<(u64, &'a str)>> for SwapRoutes {
    fn from(val: Vec<(u64, &'a str)>) -> Self {
        Self(
            val.into_iter()
                .map(|(pool_id, token_denom)| SwapRoute::new(pool_id, token_denom))
                .collect(),
        )
    }
}

impl From<SwapRoutes> for Vec<SwapAmountInRoute> {
    fn from(val: SwapRoutes) -> Self {
        val.0
            .into_iter()
            .map(|v| SwapAmountInRoute {
                pool_id: v.pool_id,
                token_out_denom: v.token_denom,
            })
            .collect()
    }
}

impl From<SwapRoutes> for Vec<SwapAmountOutRoute> {
    fn from(val: SwapRoutes) -> Self {
        val.0
            .into_iter()
            .map(|v| SwapAmountOutRoute {
                pool_id: v.pool_id,
                token_in_denom: v.token_denom,
            })
            .collect()
    }
}

impl SwapRoutes {
    pub fn dneom_first(&self) -> String {
        self.0.last().unwrap().token_denom.clone()
    }

    pub fn denom_last(&self) -> String {
        self.0.first().unwrap().token_denom.clone()
    }

    pub fn sim_swap_exact_in(
        &self,
        querier: &QuerierWrapper,
        _sender: &str,
        token_in: Coin,
    ) -> StdResult<Uint128> {
        let client = PoolmanagerQuerier::new(querier);

        let resp = client.estimate_swap_exact_amount_in(
            self.0.first().unwrap().pool_id,
            token_in.to_string(),
            self.clone().into(),
        )?;

        Uint128::from_str(&resp.token_out_amount)
    }

    pub fn sim_swap_exact_out(
        &self,
        querier: &QuerierWrapper,
        _sender: &str,
        token_out: Coin,
    ) -> StdResult<Uint128> {
        let raw_res = raw_query_bin::<Empty>(
            querier,
            &EstimateSwapExactAmountOutRequest {
                pool_id: self.0.first().unwrap().pool_id,
                routes: self.clone().into(),
                token_out: token_out.to_string(),
            }
            .into(),
        )?;

        let res = from_json::<EstimateSwapExactAmountOutResponse>(&raw_res)?;

        Uint128::from_str(&res.token_in_amount)
    }

    pub fn msg_swap_exact_in(
        &self,
        sender: &Addr,
        token_in: &str,
        token_in_amount: Uint128,
        token_out_min: Uint128,
    ) -> CosmosMsg {
        MsgSwapExactAmountIn {
            sender: sender.to_string(),
            routes: self.clone().into(),
            token_in: Some(coin(token_in_amount.u128(), token_in).into()),
            token_out_min_amount: token_out_min.to_string(),
        }
        .into()
    }

    pub fn msg_swap_exact_out(
        &self,
        sender: &Addr,
        token_out: &str,
        token_out_amount: Uint128,
        token_in_max: Uint128,
    ) -> CosmosMsg {
        MsgSwapExactAmountOut {
            sender: sender.to_string(),
            routes: self.clone().into(),
            token_in_max_amount: token_in_max.to_string(),
            token_out: Some(coin(token_out_amount.u128(), token_out).into()),
        }
        .into()
    }
}

#[cw_serde]
pub enum RangeOrder {
    Asc,
    Desc,
}

impl From<Order> for RangeOrder {
    fn from(order: Order) -> Self {
        match order {
            Order::Ascending => Self::Asc,
            Order::Descending => Self::Desc,
        }
    }
}

impl From<RangeOrder> for Order {
    fn from(order: RangeOrder) -> Self {
        match order {
            RangeOrder::Asc => Order::Ascending,
            RangeOrder::Desc => Order::Descending,
        }
    }
}
