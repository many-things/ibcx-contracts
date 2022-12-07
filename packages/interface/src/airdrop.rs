use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

use crate::types::RangeOrder;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum AirdropId {
    Id(u64),
    Label(String),
}

#[cw_serde]
pub enum AirdropIdOptional {
    Id(Option<u64>),
    Label(Option<String>),
}

#[cw_serde]
pub enum ExecuteMsg {
    Regsiter {
        merkle_root: String,
        denom: String,
        label: Option<String>,
    },

    Fund {
        id: AirdropId,
    },

    Claim {
        id: AirdropId,
        amount: Uint128,
        beneficiary: Option<String>,
        merkle_proof: Vec<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AirdropResponse)]
    Airdrop { id: AirdropId },

    #[returns(AirdropsResponse)]
    Airdrops {
        start_after: AirdropIdOptional,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },

    #[returns(LatestAirdropResponse)]
    LatestAirdropId {},

    #[returns(ClaimResponse)]
    Claim { id: AirdropId, account: String },

    #[returns(ClaimsResponse)]
    Claims {
        id: AirdropId,
        start_after: Option<String>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },

    #[returns(QualificationResponse)]
    Qualification {
        id: AirdropId,
        amount: Uint128,
        beneficiary: String,
        merkle_proof: Vec<String>,
    },
}

#[cw_serde]
pub struct AirdropResponse {
    pub id: u64,
    pub label: Option<String>,
    pub denom: String,
    pub total_amount: Uint128,
    pub total_claimed: Uint128,
}

#[cw_serde]
pub struct AirdropsResponse(pub Vec<AirdropResponse>);

#[cw_serde]
pub struct LatestAirdropResponse(pub u64);

#[cw_serde]
pub struct ClaimResponse {
    pub amount: Uint128,
    pub account: Addr,
}

#[cw_serde]
pub struct ClaimsResponse(pub Vec<ClaimResponse>);

#[cw_serde]
pub struct QualificationResponse(pub bool);

#[cw_serde]
pub struct MigrateMsg {}
