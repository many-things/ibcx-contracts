use std::collections::BTreeMap;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Coin, Decimal, Uint128, Uint256};
use ibcx_interface::periphery::{RouteKey, SwapInfo};

use crate::{OsmosisPool, PoolError};

use super::{
    route::{SimAmountOutRoute, SimAmountOutRoutes},
    Simulator, MAX_ERROR, MAX_LOOP,
};

#[cw_serde]
pub struct SimIndexInResp {
    pub total_received: Uint128,
    pub sim_routes: SimAmountOutRoutes,
}

#[cw_serde]
pub struct SearchAmountForOutputResp {
    pub est_min_token_in: Uint128,
    pub max_token_out: Uint128,
    pub sim_routes: SimAmountOutRoutes,
}

impl<'a> Simulator<'a> {
    fn estimate_out_given_in(
        &self,
        token_in: Coin,
        token_out: &str,
        pools: &mut BTreeMap<u64, Box<dyn OsmosisPool>>,
    ) -> Result<SimAmountOutRoute, PoolError> {
        let SwapInfo((_, routes)) = self
            .swap_info
            .iter()
            .find(|SwapInfo((RouteKey((from, to)), _))| from == &token_in.denom && to == token_out)
            .ok_or(PoolError::SwapRouteNotFound {
                from: token_in.denom.clone(),
                to: token_out.to_string(),
            })?;

        let ret = routes.0.iter().try_fold(token_in.clone(), |acc, route| {
            let pool = pools
                .get_mut(&route.pool_id)
                .ok_or(PoolError::PoolNotFound(route.pool_id))?;

            let spread_factor = pool.get_spread_factor()?;
            let amount_out = pool.swap_exact_amount_in(
                self.deps,
                acc,
                route.token_denom.clone(),
                Uint256::zero(),
                spread_factor,
            )?;

            Ok::<_, PoolError>(coin(
                amount_out.to_string().parse::<u128>()?,
                &route.token_denom,
            ))
        })?;

        Ok(SimAmountOutRoute {
            amount_in: token_in,
            sim_amount_out: ret.amount,
            routes: Some(routes.clone()),
        })
    }

    pub fn estimate_token_given_index_in(
        &self,
        token_in: Uint128,
        output_asset: &str,
    ) -> Result<SimIndexInResp, PoolError> {
        let mut pools_map = self
            .pools
            .iter()
            .map(|v| (v.get_id(), v.clone()))
            .collect::<BTreeMap<_, _>>();

        let routes_with_amount = self
            .index_units
            .iter()
            .map(|(denom, unit)| {
                let token_in = coin((token_in * *unit).u128(), denom);

                // if this unit does not have to do swap
                if token_in.denom == output_asset {
                    Ok(SimAmountOutRoute {
                        // amount_in = amount_out
                        sim_amount_out: token_in.amount,
                        amount_in: token_in,
                        routes: None,
                    })
                } else {
                    self.estimate_out_given_in(token_in, output_asset, &mut pools_map)
                }
            })
            .collect::<Result<Vec<_>, PoolError>>()?;

        let total_received = routes_with_amount
            .iter()
            .try_fold(Uint128::zero(), |acc, v| {
                Ok::<_, PoolError>(acc.checked_add(v.sim_amount_out)?)
            })?;

        let ret = SimIndexInResp {
            total_received,
            sim_routes: SimAmountOutRoutes(routes_with_amount),
        };

        Ok(ret)
    }

    fn search_efficient_amount_for_output_f(
        &self,
        desired_output: Coin,
        token_test_out: Uint128,
        token_test_in: Uint128,
    ) -> Result<SearchAmountForOutputResp, PoolError> {
        let est_min_token_in =
            token_test_in * Decimal::checked_from_ratio(desired_output.amount, token_test_out)?;

        let res = self.estimate_token_given_index_in(est_min_token_in, &desired_output.denom)?;

        let ret = SearchAmountForOutputResp {
            est_min_token_in,
            max_token_out: res.total_received,
            sim_routes: res.sim_routes,
        };

        Ok(ret)
    }

    pub fn search_efficient_amount_for_output(
        &self,
        desired_output: Coin,
        init_input_amount: Option<Uint128>,
    ) -> Result<SearchAmountForOutputResp, PoolError> {
        let mut acc_res = {
            let token_in = init_input_amount.unwrap_or(Uint128::new(1000000));
            let est_res = self.estimate_token_given_index_in(token_in, &desired_output.denom)?;

            SearchAmountForOutputResp {
                est_min_token_in: token_in,
                max_token_out: est_res.total_received,
                sim_routes: est_res.sim_routes,
            }
        };

        let mut loop_count = 0;
        loop {
            if loop_count >= MAX_LOOP {
                return Err(PoolError::MaxLoopExceeded);
            }

            acc_res = self.search_efficient_amount_for_output_f(
                desired_output.clone(),
                acc_res.max_token_out,
                acc_res.est_min_token_in,
            )?;

            let err = Decimal::checked_from_ratio(desired_output.amount, acc_res.max_token_out)?;
            // self.deps.api.debug(&format!(
            //     "loop {}. input_amount={}, token_in={}, token_out={}, err={}",
            //     loop_count,
            //     desired_output.amount,
            //     acc_res.est_min_token_in,
            //     acc_res.max_token_out,
            //     err
            // ));

            if Decimal::one() <= err
                && desired_output.amount - acc_res.max_token_out < Uint128::from(MAX_ERROR)
            {
                return Ok(acc_res);
            }
            loop_count += 1;
        }
    }
}
