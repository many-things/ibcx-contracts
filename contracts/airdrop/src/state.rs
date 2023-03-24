use crate::{airdrop::Airdrop, error::ContractError};
use cosmwasm_std::{Addr, Storage, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};
use ibcx_interface::airdrop::AirdropId;

pub struct AirdropIndexes<'a> {
    pub by_type: MultiIndex<'a, &'a str, Airdrop, u64>,
    pub by_creator: MultiIndex<'a, Addr, Airdrop, u64>,
}

impl<'a> IndexList<Airdrop> for AirdropIndexes<'a> {
    fn get_indexes(
        &'_ self,
    ) -> Box<dyn Iterator<Item = &'_ dyn cw_storage_plus::Index<Airdrop>> + '_> {
        let v: Vec<&dyn Index<Airdrop>> = vec![&self.by_type, &self.by_creator];
        Box::new(v.into_iter())
    }
}

pub fn airdrops<'a>() -> IndexedMap<'a, u64, Airdrop, AirdropIndexes<'a>> {
    let indexes = AirdropIndexes {
        by_type: MultiIndex::new(
            |_, k| match k {
                Airdrop::Open(_) => "open",
                Airdrop::Bearer(_) => "bearer",
            },
            "airdrop",
            "airdrop__by_type",
        ),
        by_creator: MultiIndex::new(
            |_, k| match k {
                Airdrop::Open(inner) => inner.creator.clone(),
                Airdrop::Bearer(inner) => inner.creator.clone(),
            },
            "airdrop",
            "airdrop__by_creator",
        ),
    };
    IndexedMap::new("airdrops", indexes)
}

pub const LATEST_AIRDROP_KEY: &str = "latest_airdrop";
pub const LATEST_AIRDROP_ID: Item<u64> = Item::new(LATEST_AIRDROP_KEY);

pub const LABELS_PREFIX: &str = "labels";
pub const LABELS: Map<&str, u64> = Map::new(LABELS_PREFIX);

pub const CLAIM_LOGS_PREFIX: &str = "claim_logs";
pub const CLAIM_LOGS: Map<(u64, &str), Uint128> = Map::new(CLAIM_LOGS_PREFIX);

pub fn load_airdrop(storage: &dyn Storage, id: AirdropId) -> Result<(u64, Airdrop), ContractError> {
    let airdrop_id = match id {
        AirdropId::Id(id) => id,
        AirdropId::Label(label) => LABELS.load(storage, &label)?,
    };

    let airdrop = airdrops().load(storage, airdrop_id)?;

    Ok((airdrop_id, airdrop))
}

pub fn save_label(
    storage: &mut dyn Storage,
    id: u64,
    label: &Option<String>,
) -> Result<(), ContractError> {
    if let Some(label) = label {
        if LABELS.has(storage, label) {
            return Err(ContractError::KeyAlreadyExists {
                typ: "label".to_string(),
                key: label.clone(),
            });
        }
        LABELS.save(storage, label, &id)?;
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
