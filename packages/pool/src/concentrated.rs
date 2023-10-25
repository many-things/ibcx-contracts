use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Deps, StdResult, Uint256};
use ibcx_interface::types::{SwapRoute, SwapRoutes};

use crate::{OsmosisPool, PoolError};

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

impl OsmosisPool for Pool {
    fn get_id(&self) -> u64 {
        self.id.parse().unwrap()
    }

    fn get_type(&self) -> &str {
        "concentrated_liquidity_pool"
    }

    fn get_spread_factor(&self) -> StdResult<Decimal> {
        self.spread_factor.parse()
    }

    fn clone_box(&self) -> Box<dyn OsmosisPool> {
        Box::new(self.clone())
    }

    fn swap_exact_amount_in(
        &mut self,
        deps: &Deps,
        input_amount: Coin,
        output_denom: String,
        _min_output_amount: Uint256,
        _spread_factor: Decimal,
    ) -> Result<Uint256, PoolError> {
        Ok(SwapRoutes(vec![SwapRoute {
            pool_id: self.get_id(),
            token_denom: output_denom,
        }])
        .sim_swap_exact_in(&deps.querier, &self.address, input_amount)?
        .into())
    }

    fn swap_exact_amount_out(
        &mut self,
        deps: &Deps,
        input_denom: String,
        _max_input_amount: Uint256,
        output_amount: Coin,
        _spread_factor: Decimal,
    ) -> Result<Uint256, PoolError> {
        Ok(SwapRoutes(vec![SwapRoute {
            pool_id: self.get_id(),
            token_denom: input_denom,
        }])
        .sim_swap_exact_out(&deps.querier, &self.address, output_amount)?
        .into())
    }
}
