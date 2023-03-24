use crate::error::ContractError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, StdResult, Storage, Uint128};
use cw_storage_plus::{Item, Map};
use ibcx_interface::airdrop::AirdropId;

#[cw_serde]
pub enum Airdrop {
    Open {
        creator: Addr,

        denom: String,
        total_amount: Uint128,
        total_claimed: Uint128,
        merkle_root: String,

        label: Option<String>,
        closed: bool,
    },

    Bearer {
        creator: Addr,
        signer: Addr,

        denom: String,
        total_amount: Uint128,
        total_claimed: Uint128,
        merkle_root: String,

        label: Option<String>,
        closed: bool,
    },
}

impl Airdrop {
    pub fn type_str(&self) -> &str {
        match self {
            Airdrop::Open { .. } => "open",
            Airdrop::Bearer { .. } => "bearer",
        }
    }

    pub fn creator(&self) -> &Addr {
        match self {
            Airdrop::Open { creator, .. } => creator,
            Airdrop::Bearer { creator, .. } => creator,
        }
    }

    pub fn denom(&self) -> &str {
        match self {
            Airdrop::Open { denom, .. } => denom,
            Airdrop::Bearer { denom, .. } => denom,
        }
    }

    pub fn total_amount(&self) -> &Uint128 {
        match self {
            Airdrop::Open { total_amount, .. } => total_amount,
            Airdrop::Bearer { total_amount, .. } => total_amount,
        }
    }

    pub fn total_claimed(&self) -> &Uint128 {
        match self {
            Airdrop::Open { total_claimed, .. } => total_claimed,
            Airdrop::Bearer { total_claimed, .. } => total_claimed,
        }
    }

    pub fn merkle_root(&self) -> &str {
        match self {
            Airdrop::Open { merkle_root, .. } => merkle_root,
            Airdrop::Bearer { merkle_root, .. } => merkle_root,
        }
    }

    pub fn label(&self) -> &Option<String> {
        match self {
            Airdrop::Open { label, .. } => label,
            Airdrop::Bearer { label, .. } => label,
        }
    }

    pub fn closed(&self) -> bool {
        match self {
            Airdrop::Open { closed, .. } => *closed,
            Airdrop::Bearer { closed, .. } => *closed,
        }
    }
}

pub const LATEST_AIRDROP_KEY: &str = "latest_airdrop";
pub const LATEST_AIRDROP_ID: Item<u64> = Item::new(LATEST_AIRDROP_KEY);

pub const AIRDROPS_PREFIX: &str = "airdrops";
pub const AIRDROPS: Map<u64, Airdrop> = Map::new(AIRDROPS_PREFIX);

pub const LABELS_PREFIX: &str = "labels";
pub const LABELS: Map<&str, u64> = Map::new(LABELS_PREFIX);

pub const CLAIM_LOGS_PREFIX: &str = "claim_logs";
pub const CLAIM_LOGS: Map<(u64, &str), Uint128> = Map::new(CLAIM_LOGS_PREFIX);

pub fn load_airdrop(storage: &dyn Storage, id: AirdropId) -> StdResult<(u64, Airdrop)> {
    let airdrop_id = match id {
        AirdropId::Id(id) => id,
        AirdropId::Label(label) => LABELS.load(deps.storage, &label)?,
    };

    let airdrop = AIRDROPS.load(deps.storage, airdrop_id)?;

    Ok((airdrop_id, airdrop))
}

pub fn save_label(
    storage: &mut dyn Storage,
    id: u64,
    label: &Option<String>,
) -> Result<(), ContractError> {
    if let Some(label) = label {
        if LABELS.has(storage, &label) {
            return Err(ContractError::KeyAlreadyExists {
                typ: "label".to_string(),
                key: label.clone(),
            });
        }
        LABELS.save(storage, &label, &id)?;
    }

    Ok(())
}
