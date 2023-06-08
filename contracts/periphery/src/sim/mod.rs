mod index_in;
mod index_out;
mod route;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Deps, Uint128};
use ibcx_interface::periphery::SwapInfo;

use crate::{error::ContractError, pool::OsmosisPool};

use self::{index_in::SearchAmountForOutputResp, index_out::SearchAmountForInputResp};

pub use self::route::{SimAmountInRoutes, SimAmountOutRoutes};

pub const MAX_LOOP: u64 = 256;
pub const MAX_ERROR: u64 = 5000;

#[cw_serde]
pub struct EstimateInForIndexResult {
    pub total_input: Uint128,
    pub index_out: Uint128,
    pub sim_routes: SimAmountInRoutes,
}

#[cw_serde]
pub struct EstimateOutForIndexResult {
    pub index_in: Uint128,
    pub total_output: Uint128,
    pub sim_routes: SimAmountOutRoutes,
}

#[cw_serde]
pub struct EstimateIndexForOutResult {
    pub max: SearchAmountForInputResp,
    pub est: Option<SearchAmountForInputResp>,
}

#[cw_serde]
pub struct EstimateIndexForInResult {
    pub min: SearchAmountForOutputResp,
    pub est: Option<SearchAmountForOutputResp>,
}

pub struct Simulator<'a> {
    pub deps: &'a Deps<'a>,
    pub pools: &'a [Box<dyn OsmosisPool>],
    pub swap_info: &'a [SwapInfo],
    pub index_units: &'a [(String, Decimal)],
}

impl<'a> Simulator<'a> {
    pub fn new(
        deps: &'a Deps,
        pools: &'a [Box<dyn OsmosisPool>],
        swap_info: &'a [SwapInfo],
        index_units: &'a [(String, Decimal)],
    ) -> Self {
        Self {
            deps,
            pools,
            swap_info,
            index_units,
        }
    }

    pub fn estimate_input_for_index(
        &self,
        input_denom: &str,
        index_out: Uint128,
    ) -> Result<EstimateInForIndexResult, ContractError> {
        let sim_res = self.estimate_token_given_index_out(index_out, input_denom)?;

        let ret = EstimateInForIndexResult {
            total_input: sim_res.total_spent,
            index_out,
            sim_routes: sim_res.sim_routes,
        };

        Ok(ret)
    }

    pub fn estimate_output_for_index(
        &self,
        index_in: Uint128,
        output_denom: &str,
    ) -> Result<EstimateOutForIndexResult, ContractError> {
        let sim_res = self.estimate_token_given_index_in(index_in, output_denom)?;

        let ret = EstimateOutForIndexResult {
            index_in,
            total_output: sim_res.total_received,
            sim_routes: sim_res.sim_routes,
        };

        Ok(ret)
    }

    pub fn estimate_index_for_input(
        &self,
        desired_input: Coin,
        init_index_amount: Option<Uint128>,
        min_index_amount: Option<Uint128>,
        err_tolerance: Option<Decimal>,
    ) -> Result<EstimateIndexForOutResult, ContractError> {
        let sim_init_res =
            self.search_efficient_amount_for_input(desired_input.clone(), init_index_amount)?;
        if min_index_amount.is_none() {
            return Ok(EstimateIndexForOutResult {
                max: sim_init_res,
                est: None,
            });
        }

        let min_index_amount = min_index_amount.unwrap();

        // self.deps.api.debug(&format!(
        //     "from_contract    => max_est_in: {}, max_est_out: {}",
        //     sim_init_res.max_token_in, sim_init_res.est_min_token_out,
        // ));
        if sim_init_res.est_min_token_out < min_index_amount {
            return Err(ContractError::TradeAmountExceeded {});
        }

        let tol = err_tolerance.unwrap_or_else(|| Decimal::from_ratio(1u64, 3u64));
        let sim_revised_out = {
            let gap = sim_init_res
                .est_min_token_out
                .checked_sub(min_index_amount)?;
            sim_init_res.est_min_token_out.checked_sub(gap * tol)?
        };

        let sim_revised_res =
            self.estimate_token_given_index_out(sim_revised_out, &desired_input.denom)?;

        let ret = EstimateIndexForOutResult {
            max: sim_init_res,
            est: Some(SearchAmountForInputResp {
                max_token_in: sim_revised_res.total_spent,
                est_min_token_out: sim_revised_out,
                sim_routes: sim_revised_res.sim_routes,
            }),
        };

        Ok(ret)
    }

    pub fn estimate_index_for_output(
        &self,
        desired_output: Coin,
        init_index_amount: Option<Uint128>,
        max_index_amount: Option<Uint128>,
        err_tolerance: Option<Decimal>,
    ) -> Result<EstimateIndexForInResult, ContractError> {
        let sim_init_res =
            self.search_efficient_amount_for_output(desired_output.clone(), init_index_amount)?;
        if max_index_amount.is_none() {
            return Ok(EstimateIndexForInResult {
                min: sim_init_res,
                est: None,
            });
        }

        let max_index_amount = max_index_amount.unwrap();

        // self.deps.api.debug(&format!(
        //     "from_contract    => max_est_in: {}, max_est_out: {}",
        //     sim_init_res.est_min_token_in, sim_init_res.max_token_out,
        // ));
        if max_index_amount < sim_init_res.est_min_token_in {
            return Err(ContractError::TradeAmountExceeded {});
        }

        let tol = err_tolerance.unwrap_or_else(|| Decimal::from_ratio(1u64, 3u64));
        let sim_revised_in = {
            let gap = max_index_amount.checked_sub(sim_init_res.est_min_token_in)?;
            sim_init_res.est_min_token_in.checked_add(gap * tol)?
        };

        let sim_revised_res =
            self.estimate_token_given_index_in(sim_revised_in, &desired_output.denom)?;

        let ret = EstimateIndexForInResult {
            min: sim_init_res,
            est: Some(SearchAmountForOutputResp {
                est_min_token_in: sim_revised_in,
                max_token_out: sim_revised_res.total_received,
                sim_routes: sim_revised_res.sim_routes,
            }),
        };

        Ok(ret)
    }
}
