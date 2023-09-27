use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal256, Timestamp, Uint256};

#[cw_serde]
pub struct Pool {
    #[serde(rename = "@type")]
    pub type_url: String,
    pub address: String,
    pub id: String,

    pub incentives_address: String,
    pub spread_rewards_address: String,

    pub token0: String,
    pub token1: String,

    pub current_tick_liquidity: String,
    pub current_sqrt_price: String,
    pub current_tick: String,
    pub tick_spacing: String,

    pub spread_factor: String,
    pub exponent_at_price_one: String,
    pub last_liquidity_update: String,
}
