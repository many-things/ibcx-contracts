use core::fmt;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Uint128};

use crate::types::SwapRoutes;

#[cw_serde]
pub struct InstantiateMsg {
    pub gov: String,
    pub mode: QueryMode,
}

#[cw_serde]
pub enum QueryMode {
    Stargate,
    Binding,
}

impl fmt::Display for QueryMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryMode::Stargate => write!(f, "stargate"),
            QueryMode::Binding => write!(f, "binding"),
        }
    }
}

#[cw_serde]
pub enum ExecuteMsg {
    SwitchQueryMode(QueryMode),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryModeResponse)]
    QueryMode {},

    #[returns(AmountResponse)]
    EstimateSwapExactAmountIn {
        sender: String,
        amount: Coin,
        routes: SwapRoutes,
        mode: Option<QueryMode>,
    },
    #[returns(AmountResponse)]
    EstimateSwapExactAmountOut {
        sender: String,
        amount: Coin,
        routes: SwapRoutes,
        mode: Option<QueryMode>,
    },
}

#[cw_serde]
pub struct QueryModeResponse {
    pub mode: QueryMode,
}

#[cw_serde]
pub struct AmountResponse(pub Uint128);

#[cw_serde]
pub struct MigrateMsg {}
