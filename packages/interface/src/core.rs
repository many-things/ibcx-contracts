use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub gov: String,
    pub denom: String,
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
    Mint { amount: Uint128, receiver: String }, // put some input tokens to tx payload
    Burn {},                                    // pub some ibc tokens to tx payload

    Gov(GovMsg),
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
    pub assets: Vec<Coin>,
}

#[cw_serde]
pub struct MigrateMsg {}
