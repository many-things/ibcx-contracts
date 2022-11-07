use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, Uint128};

use crate::types::{RangeOrder, SwapRoute};

#[cw_serde]
pub struct InstantiateMsg {
    pub gov: String,
    pub denom: String,
    pub reserve_denom: String,
    pub initial_assets: Vec<(String, Uint128)>,
}

#[cw_serde]
pub enum GovMsg {
    // pause mint / burn
    Pause {
        expires_at: u64,
    },
    Release {},

    // sweep all untracked assets to reserve
    Sweep {},

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

    Gov(GovMsg),
    Rebalance(RebalanceMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},

    #[returns(PauseInfoResponse)]
    PauseInfo {},

    #[returns(PortfolioResponse)]
    Portfolio {},

    #[returns(RebalanceInfoResponse)]
    RebalanceInfo { id: Option<u64> },

    #[returns(ListRebalanceInfoResponse)]
    ListRebalanceInfo {
        start_after: Option<u64>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },

    #[returns(StrategyResponse)]
    Strategy { asset: String },

    #[returns(ListStrategyResponse)]
    ListStrategy {
        start_after: Option<String>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },

    #[returns(AllocationResponse)]
    Allocation { asset: String },

    #[returns(ListAllocationResponse)]
    ListAllocation {
        start_after: Option<String>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },
}

#[cw_serde]
pub struct ConfigResponse {
    pub gov: Addr,
    pub denom: String,
    pub reserve_denom: String,
}

#[cw_serde]
pub struct PauseInfoResponse {
    pub paused: bool,
    pub expires_at: Option<u64>,
}

#[cw_serde]
pub struct PortfolioResponse {
    pub total_supply: Uint128,
    pub reserve: Uint128,
    pub assets: Vec<(String, Uint128)>,
}

#[cw_serde]
pub struct RebalanceInfoResponse {
    pub id: u64,
    pub manager: Addr,
    pub init_status: Vec<(String, Uint128)>,
    pub deflation: Vec<(String, Uint128)>,
    pub amortization: Vec<(String, Uint128)>,
    pub finished: bool,
}

#[cw_serde]
pub struct ListRebalanceInfoResponse(pub Vec<RebalanceInfoResponse>);

#[cw_serde]
pub struct StrategyResponse {
    pub asset: String,
    pub routes: Vec<SwapRoute>,
    pub cool_down: Option<u64>,
    pub max_trade_amount: Uint128,
    pub last_traded_at: u64,
}

#[cw_serde]
pub struct ListStrategyResponse(pub Vec<StrategyResponse>);

#[cw_serde]
pub struct AllocationResponse {
    pub asset: String,
    pub allocation: Uint128,
    pub ratio: Decimal,
    pub extracted: Uint128, // in amount of reserve token
}

#[cw_serde]
pub struct ListAllocationResponse {
    pub allocations: Vec<AllocationResponse>,
    pub total: Uint128, // sigma allocations
    pub total_reserve: Uint128,
}

#[cw_serde]
pub enum MigrateMsg {}
