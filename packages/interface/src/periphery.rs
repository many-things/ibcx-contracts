use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

use crate::types::SwapRoutes;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    MintExactAmountOut {
        core_addr: String,
        output_amount: Uint128,
        input_asset: String,
        swap_info: Vec<(String, SwapRoutes)>,
    },
    BurnExactAmountIn {
        core_addr: String,
        output_asset: String,
        min_output_amount: Uint128,
        swap_info: Vec<(String, SwapRoutes)>,
    },
}

#[cw_serde]
pub enum QueryMsg {}

#[cw_serde]
pub struct MigrateMsg {}
