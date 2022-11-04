use std::collections::BTreeMap;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub gov: String,
    pub denom: String,
    pub reserve_denom: String,
    pub initial_assets: Vec<(String, Uint128)>,
}

#[cw_serde]
pub struct SwapRoute {
    pub pool_id: u64,
    pub token_denom: String,
}

#[cw_serde]
pub enum ConfigMsg {
    Pause {
        expires_at: u64,
    },
    Release {},
    UpdateReserveDenom {
        new_denom: String,
    },
    UpdateTradeStrategy {
        asset: String,
        routes: Vec<SwapRoute>,
        cool_down: Option<u64>,
        max_trade_amount: Uint128, // in reserve denom
    },
}

#[cw_serde]
pub enum RebalanceMsg {
    Init {
        manager: String,
        deflation: Vec<(String, Uint128)>,    // in unit
        amortization: Vec<(String, Uint128)>, // in ratio
    },
    Deflate {
        asset: String,
        amount_token_in: Uint128,
        amount_reserve_min: Uint128,
    },
    Amortize {
        asset: String,
        amount_reserve_in: Uint128,
        amount_token_min: Uint128,
    },
    Finish {},
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint { amount: Uint128, receiver: String }, // put some input tokens to tx payload
    Burn {},                                    // pub some ibc tokens to tx payload

    Config(ConfigMsg),
    Rebalance(RebalanceMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},

    #[returns(PortfolioResponse)]
    Portfolio {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub manager: Addr,
}

#[cw_serde]
pub struct PortfolioResponse {
    pub total_supply: Uint128,
    pub assets: BTreeMap<String, Uint128>,
}

#[cw_serde]
pub enum MigrateMsg {}
