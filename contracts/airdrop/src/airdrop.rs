use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Uint128};
use ibcx_interface::airdrop::GetAirdropResponse;

use crate::error::ContractError;

#[cw_serde]
pub struct OpenAirdrop {
    pub creator: Addr,

    pub denom: String,
    pub total_amount: Uint128,
    pub total_claimed: Uint128,
    pub merkle_root: String,

    pub label: Option<String>,
    pub created_at: u64,
    pub closed_at: Option<u64>,
}

impl OpenAirdrop {
    pub fn wrap(&self) -> Airdrop {
        Airdrop::Open(self.clone())
    }
}

#[cw_serde]
pub struct BearerAirdrop {
    pub creator: Addr,
    pub signer: Addr,
    pub signer_pub: Binary,

    pub denom: String,
    pub total_amount: Uint128,
    pub total_claimed: Uint128,
    pub merkle_root: String,

    pub label: Option<String>,
    pub created_at: u64,
    pub closed_at: Option<u64>,
}

impl BearerAirdrop {
    pub fn wrap(&self) -> Airdrop {
        Airdrop::Bearer(self.clone())
    }
}

#[cw_serde]
pub enum Airdrop {
    Open(OpenAirdrop),

    Bearer(BearerAirdrop),
}

impl Airdrop {
    pub fn type_str(&self) -> &str {
        match self {
            Airdrop::Open { .. } => "open",
            Airdrop::Bearer { .. } => "bearer",
        }
    }

    pub fn unwrap_open(self) -> Result<OpenAirdrop, ContractError> {
        match self {
            Airdrop::Open(airdrop) => Ok(airdrop),
            _ => Err(ContractError::invalid_airdrop_type("open", self.type_str())),
        }
    }

    pub fn unwrap_bearer(self) -> Result<BearerAirdrop, ContractError> {
        match self {
            Airdrop::Bearer(airdrop) => Ok(airdrop),
            _ => Err(ContractError::invalid_airdrop_type(
                "bearer",
                self.type_str(),
            )),
        }
    }

    pub fn to_resp((id, airdrop): (u64, Self)) -> GetAirdropResponse {
        match airdrop {
            Self::Open(inner) => GetAirdropResponse::Open {
                id,
                creator: inner.creator.to_string(),
                denom: inner.denom,
                total_amount: inner.total_amount,
                total_claimed: inner.total_claimed,
                merkle_root: inner.merkle_root,
                label: inner.label,
                created_at: inner.created_at,
                closed_at: inner.closed_at,
            },
            Self::Bearer(inner) => GetAirdropResponse::Bearer {
                id,
                creator: inner.creator.to_string(),
                signer: inner.signer.to_string(),
                signer_pub: hex::encode(inner.signer_pub),

                denom: inner.denom,
                total_amount: inner.total_amount,
                total_claimed: inner.total_claimed,
                merkle_root: inner.merkle_root,
                label: inner.label,
                created_at: inner.created_at,
                closed_at: inner.closed_at,
            },
        }
    }
}
