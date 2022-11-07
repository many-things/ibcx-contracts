use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

use crate::types::SwapRoute;

#[cw_serde]
pub struct InstantiateMsg {
    pub core: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    SwapExactAmountIn {
        asset: String,
        routes_to_reserve: Vec<SwapRoute>,
        min_reserve_amount_out: Uint128,
    },
    SwapExactAmountOut {},
}

#[cw_serde]
pub enum QueryMsg {}

#[cw_serde]
pub struct MigrateMsg {}
