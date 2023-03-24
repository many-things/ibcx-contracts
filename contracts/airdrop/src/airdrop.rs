use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Uint128};

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
}
