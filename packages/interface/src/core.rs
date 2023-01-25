use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Decimal, Uint128};

use crate::types::SwapRoutes;

#[cw_serde]
#[derive(Default)]
pub struct Fee {
    pub collector: String,
    pub mint: Option<Decimal>,
    pub burn: Option<Decimal>,
    pub stream: Option<Decimal>,
}

#[cw_serde]
#[derive(Default)]
pub struct InstantiateMsg {
    pub gov: String,
    pub denom: String,
    pub reserve_denom: String,
    pub initial_assets: Vec<(String, Decimal)>,
    pub fee_strategy: Fee,
}

#[cw_serde]
pub enum GovMsg {
    // pause mint / burn
    Pause {
        expires_at: u64,
    },
    Release {},

    UpdateGov(String),
    UpdateFeeStrategy(Fee),
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
        denom: String,
        amount: Uint128,
        max_amount_in: Uint128,
    },
    // RESERVE => TOKEN
    Inflate {
        denom: String,
        amount: Uint128,
        min_amount_out: Uint128,
    },
}

#[cw_serde]
pub enum RebalanceMsg {
    Init {
        manager: String,
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

    #[returns(GetPauseInfoResponse)]
    GetPauseInfo {},

    #[returns(GetPortfolioResponse)]
    GetPortfolio {},

    #[returns(SimulateMintResponse)]
    SimulateMint { amount: Uint128, funds: Vec<Coin> },

    #[returns(SimulateBurnResponse)]
    SimulateBurn { amount: Uint128 },
}

#[cw_serde]
pub struct FeeResponse {
    pub collector: Addr,
    pub mint: Option<Decimal>,
    pub burn: Option<Decimal>,
    pub stream: Option<Decimal>,
    pub stream_last_collected_at: u64,
}

#[cw_serde]
pub struct GetConfigResponse {
    pub gov: Addr,
    pub denom: String,
    pub reserve_denom: String,
    pub fee_strategy: FeeResponse,
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
}

#[cw_serde]
pub struct SimulateBurnResponse {
    pub burn_amount: Uint128,
    pub redeem_amount: Vec<Coin>,
}

#[cw_serde]
pub struct MigrateMsg {}
