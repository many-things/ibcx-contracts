use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, StdResult, Uint128};
use osmosis_std::types::osmosis::gamm::poolmodels::stableswap;

use crate::error::ContractError;

use super::OsmosisPool;

#[cw_serde]
pub struct StablePool(stableswap::v1beta1::Pool);

impl From<stableswap::v1beta1::Pool> for StablePool {
    fn from(v: stableswap::v1beta1::Pool) -> Self {
        Self(v)
    }
}

impl OsmosisPool for StablePool {
    fn get_id(&self) -> u64 {
        self.0.id
    }

    fn get_type(&self) -> &str {
        "stable_pool"
    }

    fn get_spread_factor(&self) -> StdResult<Decimal> {
        Ok(self
            .0
            .pool_params
            .clone()
            .map(|v| Decimal::from_str(&v.swap_fee))
            .transpose()?
            .unwrap_or_default())
    }

    fn get_exit_fee(&self) -> StdResult<Decimal> {
        Ok(self
            .0
            .pool_params
            .clone()
            .map(|v| Decimal::from_str(&v.exit_fee))
            .transpose()?
            .unwrap_or_default())
    }

    // fn apply_new_liquidity(&mut self) {
    //     self.0.pool_liquidity
    // }

    fn swap_exact_amount_in(
        &mut self,
        input_amount: cosmwasm_std::Coin,
        output_denom: String,
        min_output_amount: cosmwasm_std::Uint128,
        spread_factor: Decimal,
    ) -> Result<Uint128, ContractError> {
        todo!()
    }

    fn swap_exact_amount_out(
        &mut self,
        input_denom: String,
        max_input_amount: cosmwasm_std::Uint128,
        output_amount: cosmwasm_std::Coin,
        spread_factor: Decimal,
    ) -> Result<Uint128, ContractError> {
        todo!()
    }
}
