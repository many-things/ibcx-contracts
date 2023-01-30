use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Uint128};

use crate::types::SwapRoutes;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct RouteKey(pub (String, String));

#[cw_serde]
pub enum ExecuteMsg {
    MintExactAmountOut {
        core_addr: String,
        output_amount: Uint128,
        input_asset: String,
        swap_info: Vec<(RouteKey, SwapRoutes)>,
    },
    BurnExactAmountIn {
        core_addr: String,
        output_asset: String,
        min_output_amount: Uint128,
        swap_info: Vec<(RouteKey, SwapRoutes)>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(SimulateMintExactAmountOutResponse)]
    SimulateMintExactAmountOut {
        core_addr: String,
        output_amount: Uint128,
        input_asset: Coin,
        swap_info: Vec<(RouteKey, SwapRoutes)>,
    },

    #[returns(SimulateBurnExactAmountInResponse)]
    SimulateBurnExactAmountIn {
        core_addr: String,
        input_amount: Uint128,
        output_asset: String,
        min_output_amount: Uint128,
        swap_info: Vec<(RouteKey, SwapRoutes)>,
    },
}

#[cw_serde]
pub struct SimulateMintExactAmountOutResponse {
    pub mint_amount: Uint128,
    pub mint_spend_amount: Vec<Coin>,
    pub mint_refund_amount: Vec<Coin>,
    pub swap_refund_amount: Coin,
}

#[cw_serde]
pub struct SimulateBurnExactAmountInResponse {
    pub burn_amount: Uint128,
    pub burn_redeem_amount: Vec<Coin>,
    pub swap_result_amount: Coin,
}

#[cw_serde]
pub struct MigrateMsg {
    pub force: Option<bool>,
}
