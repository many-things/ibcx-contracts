use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

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

pub const LATEST_AIRDROP_KEY: &str = "latest_airdrop";
pub const LATEST_AIRDROP_ID: Item<u64> = Item::new(LATEST_AIRDROP_KEY);

pub const AIRDROPS_PREFIX: &str = "airdrops";
pub const AIRDROPS: Map<u64, Airdrop> = Map::new(AIRDROPS_PREFIX);

pub const LABELS_PREFIX: &str = "labels";
pub const LABELS: Map<&str, u64> = Map::new(LABELS_PREFIX);

pub const CLAIM_LOGS_PREFIX: &str = "claim_logs";
pub const CLAIM_LOGS: Map<(u64, &str), Uint128> = Map::new(CLAIM_LOGS_PREFIX);
