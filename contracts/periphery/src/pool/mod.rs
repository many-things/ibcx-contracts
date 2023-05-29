mod stable;
mod weighted;

use cosmwasm_std::{Binary, Coin, Decimal, StdResult, Uint128};
use osmosis_std::types::osmosis::gamm::{self, poolmodels::stableswap, v1beta1::QueryPoolResponse};
pub use stable::StablePool;
pub use weighted::WeightedPool;

use crate::error::ContractError;

pub trait OsmosisPool {
    fn get_id(&self) -> u64;

    fn get_type(&self) -> &str;

    fn get_spread_factor(&self) -> StdResult<Decimal>;

    fn get_exit_fee(&self) -> StdResult<Decimal>;

    fn swap_exact_amount_in(
        &mut self,
        input_amount: Coin,
        output_denom: String,
        min_output_amount: Uint128,
        spread_factor: Decimal,
    ) -> Result<Uint128, ContractError>; // returns simulated amount out
    fn swap_exact_amount_out(
        &mut self,
        input_denom: String,
        max_input_amount: Uint128,
        output_amount: Coin,
        spread_factor: Decimal,
    ) -> Result<Uint128, ContractError>; // returns simulated amount in
}

pub fn resp_to_pool(v: QueryPoolResponse) -> StdResult<Box<dyn OsmosisPool>> {
    let pool = v.pool.unwrap();
    let type_url = pool.type_url;
    let value = Binary(pool.value);

    if type_url == "/osmosis.gamm.v1beta1.Pool" {
        Ok(Box::new(WeightedPool::from(gamm::v1beta1::Pool::try_from(
            value,
        )?)))
    } else {
        // handle stableswap
        Ok(Box::new(StablePool::from(
            stableswap::v1beta1::Pool::try_from(value)?,
        )))
    }
}

pub fn resps_to_pools(v: Vec<QueryPoolResponse>) -> StdResult<Vec<Box<dyn OsmosisPool>>> {
    v.into_iter().map(resp_to_pool).collect()
}
