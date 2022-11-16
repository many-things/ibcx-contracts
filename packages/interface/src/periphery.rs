use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, CosmosMsg, QuerierWrapper, StdResult, Uint128};
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin,
    osmosis::gamm::v1beta1::{
        MsgSwapExactAmountOut, QuerySwapExactAmountInRequest, QuerySwapExactAmountInResponse,
        QuerySwapExactAmountOutRequest, QuerySwapExactAmountOutResponse, SwapAmountInRoute,
        SwapAmountOutRoute,
    },
};

use crate::types::SwapRoute;

#[cw_serde]
pub struct InstantiateMsg {
    pub core: String,
}

#[cw_serde]
pub struct SwapInfo {
    pub asset: String,
    pub pool_id: u64,
    pub routes: Vec<SwapRoute>,
}

impl SwapInfo {
    pub fn msg_swap_exact_out(
        &self,
        sender: String,
        max_token_in_amount: Uint128,
        token_out: String,
        token_out_amount: Uint128,
    ) -> CosmosMsg {
        MsgSwapExactAmountOut {
            sender,
            routes: self
                .routes
                .iter()
                .map(|v| SwapAmountOutRoute {
                    pool_id: v.pool_id,
                    token_in_denom: v.token_denom.clone(),
                })
                .collect(),
            token_in_max_amount: max_token_in_amount.to_string(),
            token_out: Some(Coin {
                denom: token_out,
                amount: token_out_amount.to_string(),
            }),
        }
        .into()
    }

    pub fn simulate_swap_exact_in(
        &self,
        querier: &QuerierWrapper,
        sender: String,
        token_in: String,
        token_in_amount: Uint128,
    ) -> StdResult<Uint128> {
        Uint128::from_str(
            &querier
                .query::<QuerySwapExactAmountInResponse>(
                    &QuerySwapExactAmountInRequest {
                        sender,
                        pool_id: self.pool_id,
                        routes: self
                            .routes
                            .iter()
                            .map(|v| SwapAmountInRoute {
                                pool_id: v.pool_id,
                                token_out_denom: v.token_denom.clone(),
                            })
                            .collect(),
                        token_in: coin(token_in_amount.u128(), token_in).to_string(),
                    }
                    .into(),
                )?
                .token_out_amount,
        )
    }

    pub fn simulate_swap_exact_out(
        &self,
        querier: &QuerierWrapper,
        sender: String,
        token_out: String,
        token_out_amount: Uint128,
    ) -> StdResult<Uint128> {
        Uint128::from_str(
            &querier
                .query::<QuerySwapExactAmountOutResponse>(
                    &QuerySwapExactAmountOutRequest {
                        sender,
                        pool_id: self.pool_id,
                        routes: self
                            .routes
                            .iter()
                            .map(|v| SwapAmountOutRoute {
                                pool_id: v.pool_id,
                                token_in_denom: v.token_denom.clone(),
                            })
                            .collect(),
                        token_out: coin(token_out_amount.u128(), token_out).to_string(),
                    }
                    .into(),
                )?
                .token_in_amount,
        )
    }
}

#[cw_serde]
pub enum ExecuteMsg {
    MintExactAmountOut {
        asset: String,
        amount: Uint128,
        input_asset: String,
        max_input_amount: Uint128,
        swap_info: Vec<SwapInfo>,
    },
    BurnExactAmountIn {
        asset: String,
        amount: Uint128,
        output_asset: String,
        min_output_amount: Uint128,
        swap_info: Vec<SwapInfo>,
    },
}

#[cw_serde]
pub enum QueryMsg {}

#[cw_serde]
pub struct MigrateMsg {}
