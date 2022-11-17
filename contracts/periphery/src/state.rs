use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Context {
    pub executor: Addr,
    pub asset_to_check: String,
}

pub const CURRENT_CONTEXT_ID: Item<u64> = Item::new("current_context_id");
pub const CONTEXTS: Map<u64, Context> = Map::new("contexts");
