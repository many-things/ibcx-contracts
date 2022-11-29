use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Regsiter {
        merkle_root: String,
        label: String,
    },

    Claim {
        airdrop_id: u64,
        beneficiary: Option<String>,
        merkle_proof: Vec<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AirdropResponse)]
    Airdrop {},

    #[returns(AirdropsResponse)]
    Airdrops {},

    #[returns(u64)]
    LatestAirdropId {},

    #[returns(QualificationResponse)]
    Qualification {
        beneficiary: String,
        merkle_proof: Vec<String>,
    },
}

#[cw_serde]
pub struct AirdropResponse {
    pub id: u64,
}

#[cw_serde]
pub struct AirdropsResponse(pub Vec<AirdropResponse>);

#[cw_serde]
pub struct QualificationResponse {}

#[cw_serde]
pub struct MigrateMsg {}
