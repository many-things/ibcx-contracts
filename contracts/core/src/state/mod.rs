mod fee;
mod pause;
mod rebalance;
mod units;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Item, Map};

pub use fee::{Fee, StreamingFee};
pub use pause::PauseInfo;
pub use rebalance::{Rebalance, TradeInfo};
pub use units::Units;

#[cw_serde]
pub struct Config {
    pub gov: Addr,
    pub paused: PauseInfo,
    pub index_denom: String,
    pub reserve_denom: String,
}

pub const RESERVE_DENOM: &str = "reserve";

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const FEE_KEY: &str = "fee";
pub const FEE: Item<Fee> = Item::new(FEE_KEY);

pub const TOTAL_SUPPLY_KEY: &str = "total_supply";
pub const TOTAL_SUPPLY: Item<Uint128> = Item::new(TOTAL_SUPPLY_KEY);

pub const INDEX_UNITS_KEY: &str = "index_units";
pub const INDEX_UNITS: Item<Units> = Item::new(INDEX_UNITS_KEY);

pub const RESERVE_UNIT_KEY: &str = "reserve_unit";
pub const RESERVE_UNIT: Item<Decimal> = Item::new(RESERVE_UNIT_KEY);

pub const LATEST_REBALANCE_ID_KEY: &str = "latest_rebalance_id";
pub const LATEST_REBALANCE_ID: Item<u64> = Item::new(LATEST_REBALANCE_ID_KEY);

pub const REBALANCES_PREFIX: &str = "rebalances";
pub const REBALANCES: Map<u64, Rebalance> = Map::new(REBALANCES_PREFIX);

pub const TRADE_INFOS_PREFIX: &str = "trade_infos";
pub const TRADE_INFOS: Map<String, TradeInfo> = Map::new(TRADE_INFOS_PREFIX);

pub const RESERVE_BUFFER_PREFIX: &str = "reserve_buffer";
pub const RESERVE_BUFFER: Map<String, Uint128> = Map::new(RESERVE_BUFFER_PREFIX);
