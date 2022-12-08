use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub gov: String,
    pub denom: String,
    pub decimal: u8,
    pub reserve_denom: String,
    pub initial_assets: Vec<Coin>,
}

#[cw_serde]
pub enum GovMsg {
    // pause mint / burn
    Pause { expires_at: u64 },
    Release {},

    UpdateReserveDenom { new_denom: String },
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint {
        amount: Uint128,
        receiver: Option<String>,
    }, // put some input tokens to tx payload
    Burn {}, // pub some ibc tokens to tx payload

    Gov(GovMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetConfigResponse)]
    GetConfig {},

    #[returns(GetPauseInfoResponse)]
    GetPauseInfo {},

    #[returns(GetPortfolioResponse)]
    GetPortfolio {},
}

#[cw_serde]
pub struct GetConfigResponse {
    pub gov: Addr,
    pub denom: String,
    pub decimal: u8,
    pub reserve_denom: String,
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
    pub units: Vec<Coin>,
}

#[cw_serde]
pub struct MigrateMsg {}
