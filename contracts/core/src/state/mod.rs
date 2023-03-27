mod fee;
mod pause;
mod rebalance;
mod units;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Item, Map};

pub use fee::Fee;
pub use pause::PauseInfo;
pub use rebalance::{Rebalance, TradeInfo};
pub use units::{assert_units, get_redeem_amounts, get_units, set_units};

#[cw_serde]
pub struct Token {
    pub denom: String,
    pub reserve_denom: String,
    pub total_supply: Uint128,
}

pub const RESERVE_DENOM: &str = "reserve";

pub const GOV_KEY: &str = "gov";
pub const GOV: Item<Addr> = Item::new(GOV_KEY);

pub const FEE_KEY: &str = "fee";
pub const FEE: Item<Fee> = Item::new(FEE_KEY);

pub const TOKEN_KEY: &str = "token";
pub const TOKEN: Item<Token> = Item::new(TOKEN_KEY);

pub const PAUSED_KEY: &str = "paused";
pub const PAUSED: Item<PauseInfo> = Item::new(PAUSED_KEY);

pub const UNITS_PREFIX: &str = "assets";
pub const UNITS: Map<String, Decimal> = Map::new(UNITS_PREFIX);

pub const LATEST_REBALANCE_ID_KEY: &str = "latest_rebalance_id";
pub const LATEST_REBALANCE_ID: Item<u64> = Item::new(LATEST_REBALANCE_ID_KEY);

pub const REBALANCES_PREFIX: &str = "rebalances";
pub const REBALANCES: Map<u64, Rebalance> = Map::new(REBALANCES_PREFIX);

pub const TRADE_INFOS_PREFIX: &str = "trade_infos";
pub const TRADE_INFOS: Map<String, TradeInfo> = Map::new(TRADE_INFOS_PREFIX);

pub const RESERVE_BUFFER_PREFIX: &str = "reserve_buffer";
pub const RESERVE_BUFFER: Map<String, Uint128> = Map::new(RESERVE_BUFFER_PREFIX);
