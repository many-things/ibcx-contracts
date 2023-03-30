use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Decimal, Uint128};

use crate::types::SwapRoutes;

#[cw_serde]
#[derive(Default)]
pub struct StreamingFeePayload {
    pub rate: Decimal,
    pub freeze: bool,
}

#[cw_serde]
#[derive(Default)]
pub struct FeePayload {
    pub collector: String,
    pub mint_fee: Option<Decimal>,
    pub burn_fee: Option<Decimal>,
    pub streaming_fee: Option<StreamingFeePayload>,
}

#[cw_serde]
#[derive(Default)]
pub struct InstantiateMsg {
    pub gov: String,
    pub fee: FeePayload,
    pub index_denom: String,
    pub index_units: Vec<(String, Decimal)>,
    pub reserve_denom: String,
}

#[cw_serde]
pub enum GovMsg {
    // pause mint / burn
    Pause {
        expires_at: Option<u64>,
    },
    Release {},

    UpdateGov(String),
    UpdateFeeStrategy(FeePayload),
    UpdateReserveDenom(String),
    UpdateTradeInfo {
        denom: String,
        routes: SwapRoutes,
        cooldown: u64,
        max_trade_amount: Uint128,
    },
}

#[cw_serde]
pub enum RebalanceTradeMsg {
    // TOKEN => RESERVE
    Deflate {
        target_denom: String,
        amount_out: Uint128,
        max_amount_in: Uint128,
    },
    // RESERVE => TOKEN
    Inflate {
        target_denom: String,
        amount_in: Uint128,
        min_amount_out: Uint128,
    },
}

#[cw_serde]
pub enum RebalanceMsg {
    Init {
        manager: Option<String>,
        deflation: Vec<(String, Decimal)>, // target units
        inflation: Vec<(String, Decimal)>, // conversion weights
    },
    Trade(RebalanceTradeMsg),
    Finalize {},
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint {
        amount: Uint128,
        receiver: Option<String>,
        refund_to: Option<String>,
    }, // put some input tokens to tx payload
    Burn {
        redeem_to: Option<String>,
    }, // pub some ibc tokens to tx payload
    Realize {},

    Gov(GovMsg),
    Rebalance(RebalanceMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Uint128)]
    GetBalance { account: String },

    #[returns(GetConfigResponse)]
    GetConfig {},

    #[returns(GetFeeResponse)]
    GetFee { time: Option<u64> },

    #[returns(GetPauseInfoResponse)]
    GetPauseInfo { time: Option<u64> },

    #[returns(GetPortfolioResponse)]
    GetPortfolio { time: Option<u64> },

    #[returns(SimulateMintResponse)]
    SimulateMint {
        amount: Uint128,
        funds: Vec<Coin>,
        time: Option<u64>,
    },

    #[returns(SimulateBurnResponse)]
    SimulateBurn { amount: Uint128, time: Option<u64> },
}

#[cw_serde]
pub struct GetConfigResponse {
    pub gov: Addr,
    pub index_denom: String,
    pub reserve_denom: String,
}

#[cw_serde]
pub struct StreamingFeeResponse {
    pub rate: Decimal,
    pub collected: Vec<Coin>,
    pub freeze: bool,
    pub last_collected_at: u64,
}

#[cw_serde]
pub struct GetFeeResponse {
    pub collector: Addr,
    pub mint_fee: Option<Decimal>,
    pub burn_fee: Option<Decimal>,
    pub streaming_fee: Option<StreamingFeeResponse>,
}

#[cw_serde]
pub struct GetPauseInfoResponse {
    pub paused: bool,
    pub expires_at: Option<u64>,
}

#[cw_serde]
pub struct GetPortfolioResponse {
    pub total_supply: Uint128,
    pub assets: Vec<Coin>,
    pub units: Vec<(String, Decimal)>,
}

#[cw_serde]
pub struct SimulateMintResponse {
    pub mint_amount: Uint128,
    pub refund_amount: Vec<Coin>,
    pub fund_spent: Vec<Coin>,
}

#[cw_serde]
pub struct SimulateBurnResponse {
    pub burn_amount: Uint128,
    pub redeem_amount: Vec<Coin>,
}

#[cw_serde]
pub struct MigrateMsg {
    pub force: Option<bool>,
}
