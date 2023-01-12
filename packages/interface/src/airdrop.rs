use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

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
pub enum ClaimProof {
    Account(String),
    ClaimProof(String),
}

#[cw_serde]
pub enum ClaimProofOptional {
    Account(Option<String>),
    ClaimProof(String),
}

#[cw_serde]
pub enum ExecuteMsg {
    Register {
        merkle_root: String,
        denom: String,
        label: Option<String>,
        bearer: Option<bool>,
    },

    Fund {
        id: AirdropId,
    },

    Claim {
        id: AirdropId,
        amount: Uint128,
        claim_proof: ClaimProofOptional,
        merkle_proof: Vec<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetAirdropResponse)]
    GetAirdrop { id: AirdropId },

    #[returns(ListAirdropsResponse)]
    ListAirdrops {
        start_after: AirdropIdOptional,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },

    #[returns(LatestAirdropResponse)]
    LatestAirdropId {},

    #[returns(GetClaimResponse)]
    GetClaim {
        id: AirdropId,
        claim_proof: ClaimProof,
    },

    #[returns(ListClaimsResponse)]
    ListClaims {
        id: AirdropId,
        start_after: Option<String>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },

    #[returns(CheckQualificationResponse)]
    CheckQualification {
        id: AirdropId,
        amount: Uint128,
        claim_proof: ClaimProof,
        merkle_proof: Vec<String>,
    },
}

#[cw_serde]
pub struct GetAirdropResponse {
    pub id: u64,
    pub label: Option<String>,
    pub denom: String,
    pub total_amount: Uint128,
    pub total_claimed: Uint128,
    pub bearer: bool,
}

#[cw_serde]
pub struct ListAirdropsResponse(pub Vec<GetAirdropResponse>);

#[cw_serde]
pub struct LatestAirdropResponse(pub u64);

#[cw_serde]
pub struct GetClaimResponse {
    pub amount: Uint128,
    pub claim_proof: String,
}

#[cw_serde]
pub struct ListClaimsResponse(pub Vec<GetClaimResponse>);

#[cw_serde]
pub struct CheckQualificationResponse(pub bool);

#[cw_serde]
pub struct MigrateMsg {}
