mod concentrated;
mod error;
mod sim;
mod stable;
mod weighted;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Coin, Decimal, Deps, StdResult, Uint256};

#[allow(deprecated)]
use osmosis_std::types::osmosis::poolmanager::v1beta1::PoolRequest;

pub use error::PoolError;
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
#[serde(untagged)]
pub enum Pool {
    CW {
        #[serde(rename = "@type")]
        type_url: String,
        contract_address: String,
        pool_id: String,
        code_id: String,
        instantiate_msg: Binary,
    },
    Stable(StablePool),
    Weighted(WeightedPool),
    Concentrated(ConcentratedPool),
}

impl Pool {
    pub fn get_id(&self) -> u64 {
        match self {
            Pool::CW { pool_id, .. } => pool_id.parse().unwrap(),
            Pool::Stable(p) => p.id.parse().unwrap(),
            Pool::Weighted(p) => p.id.parse().unwrap(),
            Pool::Concentrated(p) => p.id.parse().unwrap(),
        }
    }
}

pub fn query_pools(
    deps: &Deps,
    pool_ids: Vec<u64>,
) -> Result<Vec<Box<dyn OsmosisPool>>, PoolError> {
    #[cw_serde]
    pub struct PoolResponse {
        pub pool: Pool,
    }

    let pool_resps = pool_ids
        .into_iter()
        .map(|v| deps.querier.query(&PoolRequest { pool_id: v }.into()))
        .collect::<StdResult<Vec<PoolResponse>>>()?;

    let pools = pool_resps
        .into_iter()
        .map(|v| -> Result<Box<dyn OsmosisPool>, PoolError> {
            match v.pool {
                Pool::Stable(p) => Ok(Box::new(p)),
                Pool::Weighted(p) => Ok(Box::new(p)),
                Pool::Concentrated(p) => Ok(Box::new(p)),
                Pool::CW { .. } => Err(PoolError::UnsupportedPoolType),
            }
        })
        .collect::<Result<_, _>>()?;

    Ok(pools)
}

#[cfg(test)]
mod test;
