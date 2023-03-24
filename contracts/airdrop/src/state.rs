use crate::{airdrop::Airdrop, error::ContractError};
use cosmwasm_std::{StdResult, Storage, Uint128};
use cw_storage_plus::{Item, Map};
use ibcx_interface::airdrop::AirdropId;

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
        AirdropId::Label(label) => LABELS.load(storage, &label)?,
    };

    let airdrop = AIRDROPS.load(storage, airdrop_id)?;

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

pub fn assert_not_claimed(
    storage: &dyn Storage,
    id: u64,
    claim_key: &str,
) -> Result<(), ContractError> {
    if CLAIM_LOGS.may_load(storage, (id, claim_key))?.is_some() {
        return Err(ContractError::AlreadyClaimed {
            airdrop_id: id,
            claim_key: claim_key.to_string(),
        });
    }

    Ok(())
}
