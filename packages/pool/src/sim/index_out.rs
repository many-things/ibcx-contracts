use std::collections::BTreeMap;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Coin, Decimal, Uint128, Uint256};
use ibcx_interface::periphery::{RouteKey, SwapInfo};

use crate::{OsmosisPool, PoolError};

use super::{
    route::{SimAmountInRoute, SimAmountInRoutes},
    Simulator, MAX_ERROR, MAX_LOOP,
};

#[cw_serde]
pub struct SimIndexOutResp {
    pub total_spent: Uint128,
    pub sim_routes: SimAmountInRoutes,
}

#[cw_serde]
pub struct SearchAmountForInputResp {
    pub max_token_in: Uint128,
    pub est_min_token_out: Uint128,
    pub sim_routes: SimAmountInRoutes,
}

impl<'a> Simulator<'a> {
    fn estimate_in_given_out(
        &self,
        token_in: &str,
        token_out: Coin,
        pools: &mut BTreeMap<u64, Box<dyn OsmosisPool>>,
    ) -> Result<SimAmountInRoute, PoolError> {
        let SwapInfo((_, routes)) = self
            .swap_info
            .iter()
            .find(|SwapInfo((RouteKey((from, to)), _))| from == token_in && to == &token_out.denom)
            .ok_or(PoolError::SwapRouteNotFound {
                from: token_in.to_string(),
                to: token_out.denom.clone(),
            })?;

        let mut routes = routes.clone();
        routes.0.reverse();

        let ret = routes.0.iter().try_fold(token_out.clone(), |acc, route| {
            let pool = pools
                .get_mut(&route.pool_id)
                .ok_or(PoolError::PoolNotFound(route.pool_id))?;

            let spread_factor = pool.get_spread_factor()?;
            let amount_in = pool.swap_exact_amount_out(
                self.deps,
                route.token_denom.clone(),
                Uint256::from(u128::MAX), // infinite
                acc,
                spread_factor,
            )?;

            Ok::<_, PoolError>(coin(
                amount_in.to_string().parse::<u128>()?,
                &route.token_denom,
            ))
        })?;

        Ok(SimAmountInRoute {
            sim_amount_in: ret.amount,
            amount_out: token_out,
            routes: Some(routes),
        })
    }

    pub fn estimate_token_given_index_out(
        &self,
        token_out: Uint128,
        input_asset: &str,
    ) -> Result<SimIndexOutResp, PoolError> {
        let mut pools_map = self
            .pools
            .iter()
            .map(|v| (v.get_id(), v.clone()))
            .collect::<BTreeMap<_, _>>();

        let routes_with_amount = self
            .index_units
            .iter()
            .map(|(denom, unit)| coin((token_out * *unit).u128(), denom))
            .map(|token_out| {
                // if this unit does not have to do swap
                if token_out.denom == input_asset {
                    Ok(SimAmountInRoute {
                        // amount_in = amount_out
                        sim_amount_in: token_out.amount,
                        amount_out: token_out,
                        routes: None,
                    })
                } else {
                    self.estimate_in_given_out(input_asset, token_out, &mut pools_map)
                }
            })
            .collect::<Result<Vec<_>, PoolError>>()?;

        let total_spent = routes_with_amount
            .iter()
            .try_fold(Uint128::zero(), |acc, v| {
                Ok::<_, PoolError>(acc.checked_add(v.sim_amount_in)?)
            })?;

        let ret = SimIndexOutResp {
            total_spent,
            sim_routes: SimAmountInRoutes(routes_with_amount),
        };

        Ok(ret)
    }

    fn search_efficient_amount_for_input_f(
        &self,
        desired_input: Coin,
        token_test_out: Uint128,
        token_test_in: Uint128,
    ) -> Result<SearchAmountForInputResp, PoolError> {
        let est_min_token_out =
            token_test_out * Decimal::checked_from_ratio(desired_input.amount, token_test_in)?;

        let res = self.estimate_token_given_index_out(est_min_token_out, &desired_input.denom)?;

        let ret = SearchAmountForInputResp {
            max_token_in: res.total_spent,
            est_min_token_out,
            sim_routes: res.sim_routes,
        };

        Ok(ret)
    }

    pub fn search_efficient_amount_for_input(
        &self,
        desired_input: Coin,
        init_output_amount: Option<Uint128>,
    ) -> Result<SearchAmountForInputResp, PoolError> {
        let mut acc_res = {
            let token_out = init_output_amount.unwrap_or(Uint128::new(1000000));
            let est_res = self.estimate_token_given_index_out(token_out, &desired_input.denom)?;

            SearchAmountForInputResp {
                max_token_in: est_res.total_spent,
                est_min_token_out: token_out,
                sim_routes: est_res.sim_routes,
            }
        };

        let mut loop_count = 0;
        loop {
            if loop_count >= MAX_LOOP {
                return Err(PoolError::MaxLoopExceeded);
            }

            acc_res = self.search_efficient_amount_for_input_f(
                desired_input.clone(),
                acc_res.est_min_token_out,
                acc_res.max_token_in,
            )?;

            let err = Decimal::checked_from_ratio(desired_input.amount, acc_res.max_token_in)?;
            // self.deps.api.debug(&format!(
            //     "loop {}. input_amount={}, token_in={}, token_out={}, err={}",
            //     loop_count,
            //     desired_input.amount,
            //     acc_res.max_token_in,
            //     acc_res.est_min_token_out,
            //     err
            // ));

            if Decimal::one() <= err
                && desired_input.amount.checked_sub(acc_res.max_token_in)?
                    < Uint128::from(MAX_ERROR)
            {
                return Ok(acc_res);
            }
            loop_count += 1;
        }
    }
}
