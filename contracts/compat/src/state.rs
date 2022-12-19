use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use ibc_interface::compat::QueryMode;

pub const GOV: Item<Addr> = Item::new("gov");
pub const QUERY_MODE: Item<QueryMode> = Item::new("query_mode");
