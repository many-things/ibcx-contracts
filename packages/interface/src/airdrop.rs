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

impl AirdropId {
    pub fn id(id: u64) -> Self {
        AirdropId::Id(id)
    }

    pub fn label(label: impl Into<String>) -> Self {
        AirdropId::Label(label.into())
    }
}

#[cw_serde]
pub enum AirdropIdOptional {
    Id(Option<u64>),
    Label(Option<String>),
}

#[cw_serde]
pub enum RegisterPayload {
    // Payload for open airdrop
    Open {
        // merkle root of airdrop
        merkle_root: String,

        // denomination of airdrop
        denom: String,

        // optional: label
        label: Option<String>,
    },

    // Payload for bearer airdrop
    Bearer {
        merkle_root: String,
        denom: String,
        label: Option<String>,
        signer: Option<String>,
    },
}

#[cw_serde]
pub enum ClaimPayload {
    // Payload for open airdrop
    Open {
        airdrop: AirdropId,        // airdrop specifier
        amount: Uint128,           // claim amount
        account: Option<String>,   // address who claims
        merkle_proof: Vec<String>, // merkle proof of airdrop
    },

    // Payload for bearer airdrop
    Bearer {
        airdrop: AirdropId,        // airdrop specifier
        amount: Uint128,           // claim amount
        claim_hash: String,        // salt hash to prevent double spending
        claim_sign: String,        // signature of signer
        merkle_proof: Vec<String>, // merkle proof of airdrop
    },
}

#[cw_serde]
pub enum ExecuteMsg {
    Register(RegisterPayload),
    Fund(AirdropId),

    Claim(ClaimPayload),
    MultiClaim(Vec<ClaimPayload>),

    Close(AirdropId),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetAirdropResponse)]
    GetAirdrop(AirdropId),

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
        airdrop: AirdropId,
        claim_key: String,
    },

    #[returns(ListClaimsResponse)]
    ListClaims {
        airdrop: AirdropId,
        start_after: Option<String>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },

    #[returns(ValidateClaimResponse)]
    ValidateClaim(ClaimPayload),
}

#[cw_serde]
pub enum GetAirdropResponse {
    Open {
        id: u64,
        creator: String,

        denom: String,
        total_amount: Uint128,
        total_claimed: Uint128,
        merkle_root: String,

        label: Option<String>,
        closed: bool,
    },

    Bearer {
        id: u64,
        creator: String,
        signer: String,

        denom: String,
        total_amount: Uint128,
        total_claimed: Uint128,
        merkle_root: String,

        label: Option<String>,
        closed: bool,
    },
}

#[cw_serde]
pub struct ListAirdropsResponse(pub Vec<GetAirdropResponse>);

#[cw_serde]
pub struct LatestAirdropResponse(pub u64);

#[cw_serde]
pub struct GetClaimResponse {
    pub id: u64,
    pub amount: Uint128,
    pub claim_key: String,
}

#[cw_serde]
pub struct ListClaimsResponse(pub Vec<GetClaimResponse>);

#[cw_serde]
pub struct ValidateClaimResponse(pub bool);

#[cw_serde]
pub struct MigrateMsg {
    pub force: Option<bool>,
}
