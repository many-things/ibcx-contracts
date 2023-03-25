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
pub enum AirdropType {
    Open,
    Bearer,
}

impl ToString for AirdropType {
    fn to_string(&self) -> String {
        match self {
            AirdropType::Open => "open".to_string(),
            AirdropType::Bearer => "bearer".to_string(),
        }
    }
}

impl From<AirdropType> for String {
    fn from(val: AirdropType) -> Self {
        val.to_string()
    }
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
        // merkle root of airdrop
        merkle_root: String,

        // denomination of airdrop
        denom: String,

        // public key of signer
        signer_pub: String,

        // signature of signer - sign(sha256({BECH32_SIGNER_ADDRESS}))
        signer_sig: String,

        // optional: label
        label: Option<String>,
    },
}

impl RegisterPayload {
    pub fn open(merkle_root: impl ToString, denom: impl ToString, label: Option<&str>) -> Self {
        Self::Open {
            merkle_root: merkle_root.to_string(),
            denom: denom.to_string(),
            label: label.map(|x| x.to_string()),
        }
    }

    pub fn bearer(
        merkle_root: impl ToString,
        denom: impl ToString,
        signer_pub: impl ToString,
        signer_sig: impl ToString,
        label: Option<&str>,
    ) -> Self {
        Self::Bearer {
            merkle_root: merkle_root.to_string(),
            denom: denom.to_string(),
            signer_pub: signer_pub.to_string(),
            signer_sig: signer_sig.to_string(),
            label: label.map(|x| x.to_string()),
        }
    }
}

#[cw_serde]
pub enum ClaimPayload {
    // Payload for open airdrop
    Open {
        airdrop: AirdropId,        // airdrop specifier
        amount: Uint128,           // claim amount
        account: Option<String>,   // address who claims - default is tx.sender
        merkle_proof: Vec<String>, // merkle proof of airdrop
    },

    // Payload for bearer airdrop
    Bearer {
        airdrop: AirdropId,        // airdrop specifier
        amount: Uint128,           // claim amount
        account: Option<String>,   // address who claims - default is tx.sender
        claim_hash: String,        // salt hash to prevent double spending
        claim_sign: String,        // signature of signer
        merkle_proof: Vec<String>, // merkle proof of airdrop
    },
}

impl ClaimPayload {
    pub fn open_id(
        id: u64,
        amount: u128,
        account: Option<impl ToString>,
        merkle_proof: &[&str],
    ) -> Self {
        Self::Open {
            airdrop: AirdropId::id(id),
            amount: Uint128::new(amount),
            account: account.map(|x| x.to_string()),
            merkle_proof: merkle_proof.iter().map(|x| x.to_string()).collect(),
        }
    }

    pub fn open_label(
        label: impl ToString + Into<String>,
        amount: u128,
        account: Option<impl ToString>,
        merkle_proof: &[&str],
    ) -> Self {
        Self::Open {
            airdrop: AirdropId::label(label),
            amount: Uint128::new(amount),
            account: account.map(|x| x.to_string()),
            merkle_proof: merkle_proof.iter().map(|x| x.to_string()).collect(),
        }
    }

    pub fn bearer_id(
        id: u64,
        amount: u128,
        account: Option<impl ToString>,
        claim_hash: impl ToString,
        claim_sign: impl ToString,
        merkle_proof: &[&str],
    ) -> Self {
        Self::Bearer {
            airdrop: AirdropId::id(id),
            amount: Uint128::new(amount),
            account: account.map(|x| x.to_string()),
            claim_hash: claim_hash.to_string(),
            claim_sign: claim_sign.to_string(),
            merkle_proof: merkle_proof.iter().map(|x| x.to_string()).collect(),
        }
    }

    pub fn bearer_label(
        label: impl ToString + Into<String>,
        amount: u128,
        account: Option<impl ToString>,
        claim_hash: impl ToString,
        claim_sign: impl ToString,
        merkle_proof: &[&str],
    ) -> Self {
        Self::Bearer {
            airdrop: AirdropId::label(label),
            amount: Uint128::new(amount),
            account: account.map(|x| x.to_string()),
            claim_hash: claim_hash.to_string(),
            claim_sign: claim_sign.to_string(),
            merkle_proof: merkle_proof.iter().map(|x| x.to_string()).collect(),
        }
    }
}

#[cw_serde]
pub enum ExecuteMsg {
    Register(RegisterPayload),
    Fund(AirdropId),
    Claim(ClaimPayload),
    Close(AirdropId),
}

#[cw_serde]
pub enum ListAirdropsQueryOptions {
    ByID {
        start_after: Option<u64>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },

    ByType {
        #[serde(rename = "type")]
        typ: AirdropType,
        start_after: Option<u64>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },

    ByLabel {
        start_after: Option<String>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },

    ByCreator {
        creator: String,
        start_after: Option<u64>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },
}

impl ListAirdropsQueryOptions {
    pub fn by_id(start_after: Option<u64>, limit: Option<u32>, order: Option<RangeOrder>) -> Self {
        Self::ByID {
            start_after,
            limit,
            order,
        }
    }

    pub fn by_type(
        typ: AirdropType,
        start_after: Option<u64>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    ) -> Self {
        Self::ByType {
            typ,
            start_after,
            limit,
            order,
        }
    }

    pub fn by_label(
        start_after: Option<&str>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    ) -> Self {
        Self::ByLabel {
            start_after: start_after.map(|x| x.to_string()),
            limit,
            order,
        }
    }

    pub fn by_creator(
        creator: impl ToString,
        start_after: Option<u64>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    ) -> Self {
        Self::ByCreator {
            creator: creator.to_string(),
            start_after,
            limit,
            order,
        }
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetAirdropResponse)]
    GetAirdrop(AirdropId),

    #[returns(ListAirdropsResponse)]
    ListAirdrops(ListAirdropsQueryOptions),

    #[returns(LatestAirdropResponse)]
    LatestAirdropId {},

    #[returns(GetClaimResponse)]
    GetClaim {
        airdrop: AirdropId,
        claim_key: String,
    },

    #[returns(VerifyClaimResponse)]
    VerifyClaim(ClaimPayload),

    #[returns(ListClaimsResponse)]
    ListClaims {
        airdrop: AirdropId,
        start_after: Option<String>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },

    #[returns(GetLabelResponse)]
    GetLabel(String),

    #[returns(ListLabelsResponse)]
    ListLabels {
        start_after: Option<String>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },
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
        created_at: u64,
        closed_at: Option<u64>,
    },

    Bearer {
        id: u64,
        creator: String,
        signer: String,
        signer_pub: String,

        denom: String,
        total_amount: Uint128,
        total_claimed: Uint128,
        merkle_root: String,

        label: Option<String>,
        created_at: u64,
        closed_at: Option<u64>,
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
#[derive(Default)]
pub struct VerifyClaimResponse {
    pub valid: bool,
    pub reason: Option<String>,
}

impl VerifyClaimResponse {
    pub fn fail(mut self, reason: impl ToString) -> Self {
        self.reason = Some(reason.to_string());
        self
    }

    pub fn ok(mut self) -> Self {
        self.valid = true;
        self
    }
}

#[cw_serde]
pub struct ListClaimsResponse(pub Vec<GetClaimResponse>);

#[cw_serde]
pub struct GetLabelResponse {
    pub creator: String,
    pub label: String,
    pub airdrop_id: u64,
}

#[cw_serde]
pub struct ListLabelsResponse(pub Vec<GetLabelResponse>);

#[cw_serde]
pub struct MigrateMsg {
    pub force: Option<bool>,
}
