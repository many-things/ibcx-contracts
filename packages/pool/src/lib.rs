mod concentrated;
mod error;
mod query;
mod sim;
mod stable;
mod weighted;

use std::num::ParseIntError;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Deps, StdResult, Uint256};

pub use error::PoolError;
pub use query::query_pools;
pub use sim::Simulator;

pub use concentrated::Pool as ConcentratedPool;
pub use stable::Pool as StablePool;
pub use weighted::Pool as WeightedPool;

pub trait OsmosisPool {
    fn get_id(&self) -> u64;

    fn get_type(&self) -> &str;

    fn get_spread_factor(&self) -> StdResult<Decimal>;

    fn clone_box(&self) -> Box<dyn OsmosisPool>;

    fn swap_exact_amount_in(
        &mut self,
        deps: &Deps,
        input_amount: Coin,
        output_denom: String,
        min_output_amount: Uint256,
        spread_factor: Decimal,
    ) -> Result<Uint256, PoolError>; // returns simulated amount out
    fn swap_exact_amount_out(
        &mut self,
        deps: &Deps,
        input_denom: String,
        max_input_amount: Uint256,
        output_amount: Coin,
        spread_factor: Decimal,
    ) -> Result<Uint256, PoolError>; // returns simulated amount in
}

impl Clone for Box<dyn OsmosisPool> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[cw_serde]
pub enum Pool {
    Stable(StablePool),
    Weighted(WeightedPool),
    Concentrated(ConcentratedPool),
}

impl Pool {
    pub fn get_id(&self) -> Result<u64, ParseIntError> {
        match self {
            Pool::Stable(p) => p.id.parse(),
            Pool::Weighted(p) => p.id.parse(),
            Pool::Concentrated(p) => p.id.parse(),
        }
    }
}

#[cfg(test)]
mod test;
