use std::collections::BTreeMap;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Coin, Decimal, Deps, Uint128};
use ibcx_interface::{
    periphery::{RouteKey, SwapInfo},
    types::SwapRoutes,
};

use crate::{error::ContractError, pool::OsmosisPool};

const MAX_LOOP: u64 = 256;
const MAX_ERROR: u64 = 10000;

#[cw_serde]
pub struct SimAmountOutRoute {
    pub amount_in: Coin,
    pub sim_amount_out: Uint128,
    pub routes: Option<SwapRoutes>,
}

#[cw_serde]
pub struct SimAmountInRoute {
    pub sim_amount_in: Uint128,
    pub amount_out: Coin,
    pub routes: Option<SwapRoutes>,
}

pub fn estimate_out_given_in(
    deps: &Deps,
    token_in: Coin,
    token_out: &str,
    pools: &mut BTreeMap<u64, Box<dyn OsmosisPool>>,
    swap_info: &[SwapInfo],
) -> Result<SimAmountOutRoute, ContractError> {
    let SwapInfo((_, routes)) = swap_info
        .iter()
        .find(|SwapInfo((RouteKey((from, to)), _))| from == &token_in.denom && to == token_out)
        .ok_or(ContractError::SwapRouteNotFound {
            from: token_in.denom.clone(),
            to: token_out.to_string(),
        })?;

    let ret = routes.0.iter().try_fold(token_in.clone(), |acc, route| {
        let pool = pools
            .get_mut(&route.pool_id)
            .ok_or(ContractError::PoolNotFound(route.pool_id))?;

        let spread_factor = pool.get_spread_factor()?;
        let amount_out = pool.swap_exact_amount_in(
            deps,
            acc,
            route.token_denom.clone(),
            Uint128::zero(),
            spread_factor,
        )?;

        Ok::<_, ContractError>(coin(amount_out.u128(), &route.token_denom))
    })?;

    Ok(SimAmountOutRoute {
        amount_in: token_in,
        sim_amount_out: ret.amount,
        routes: Some(routes.clone()),
    })
}

pub fn estimate_in_given_out(
    deps: &Deps,
    token_in: &str,
    token_out: Coin,
    pools: &mut BTreeMap<u64, Box<dyn OsmosisPool>>,
    swap_info: &[SwapInfo],
) -> Result<SimAmountInRoute, ContractError> {
    let SwapInfo((_, routes)) = swap_info
        .iter()
        .find(|SwapInfo((RouteKey((from, to)), _))| from == token_in && to == &token_out.denom)
        .ok_or(ContractError::SwapRouteNotFound {
            from: token_in.to_string(),
            to: token_out.denom.clone(),
        })?;

    let mut routes = routes.clone();
    routes.0.reverse();

    let ret = routes.0.iter().try_fold(token_out.clone(), |acc, route| {
        let pool = pools
            .get_mut(&route.pool_id)
            .ok_or(ContractError::PoolNotFound(route.pool_id))?;

        let spread_factor = pool.get_spread_factor()?;
        let amount_in = pool.swap_exact_amount_out(
            deps,
            route.token_denom.clone(),
            Uint128::from(u64::MAX), // infinite
            acc,
            spread_factor,
        )?;

        Ok::<_, ContractError>(coin(amount_in.u128(), &route.token_denom))
    })?;

    Ok(SimAmountInRoute {
        sim_amount_in: ret.amount,
        amount_out: token_out,
        routes: Some(routes),
    })
}

pub fn calc_index_per_token(
    deps: &Deps,
    units: &[(String, Decimal)],
    token_out: Uint128,
    input_asset: &str,
    pools: &[Box<dyn OsmosisPool>],
    swap_info: &[SwapInfo],
) -> Result<(Uint128, Vec<SimAmountInRoute>), ContractError> {
    let mut pools_map = pools
        .iter()
        .map(|v| (v.get_id(), v.clone()))
        .collect::<BTreeMap<_, _>>();

    let routes_with_amount = units
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
                estimate_in_given_out(deps, input_asset, token_out, &mut pools_map, swap_info)
            }
        })
        .collect::<Result<Vec<_>, ContractError>>()?;

    let total_spent = routes_with_amount
        .iter()
        .fold(Uint128::zero(), |acc, v| v.sim_amount_in + acc);

    Ok((total_spent, routes_with_amount))
}

pub fn search_efficient_f(
    deps: &Deps,
    units: &[(String, Decimal)],
    input_asset: Coin,
    token_out: Uint128,
    token_in: Uint128,
    pools: &[Box<dyn OsmosisPool>],
    swap_info: &[SwapInfo],
) -> Result<(Uint128, Uint128, Vec<SimAmountInRoute>), ContractError> {
    let est_min_token_out = token_out * Decimal::checked_from_ratio(input_asset.amount, token_in)?;

    let (max_token_in, routes_with_amount) = calc_index_per_token(
        deps,
        units,
        est_min_token_out,
        &input_asset.denom,
        pools,
        swap_info,
    )?;

    Ok((max_token_in, est_min_token_out, routes_with_amount))
}

pub fn search_efficient(
    deps: &Deps,
    units: &[(String, Decimal)],
    input_asset: Coin,
    init_output_amount: Option<Uint128>,
    pools: &[Box<dyn OsmosisPool>],
    swap_info: &[SwapInfo],
) -> Result<(Uint128, Uint128, Vec<SimAmountInRoute>), ContractError> {
    let mut token_out = init_output_amount.unwrap_or(Uint128::new(1000000));
    let (mut token_in, _) =
        calc_index_per_token(deps, units, token_out, &input_asset.denom, pools, swap_info)?;

    let mut loop_count = 0;
    loop {
        if loop_count >= MAX_LOOP {
            return Err(ContractError::MaxLoopExceeded);
        }

        let (token_in_res, token_out_res, routes_with_amount) = search_efficient_f(
            deps,
            units,
            input_asset.clone(),
            token_out,
            token_in,
            pools,
            swap_info,
        )?;
        token_in = token_in_res;
        token_out = token_out_res;

        let err = Decimal::checked_from_ratio(input_asset.amount, token_in)?;
        deps.api.debug(&format!(
            "loop {}. input_amount={}, token_in={}, token_out={}, err={}",
            loop_count, input_asset.amount, token_in, token_out, err
        ));

        if Decimal::one() <= err && input_asset.amount - token_in < Uint128::from(MAX_ERROR) {
            return Ok((token_in, token_out, routes_with_amount));
        }
        loop_count += 1;
    }
}

#[cw_serde]
pub struct EstimateResult {
    pub max_est_in: Uint128,
    pub max_est_out: Uint128,
    pub max_routes: Vec<SimAmountInRoute>,

    pub est_in: Uint128,
    pub est_out: Uint128,
    pub routes: Vec<SimAmountInRoute>,
}

#[allow(clippy::too_many_arguments)]
pub fn estimate_max_index_for_input(
    deps: &Deps,
    units: &[(String, Decimal)],
    input_asset: Coin,
    init_index_amount: Option<Uint128>,
    (min_index_amount, max_index_amount): (Option<Uint128>, Option<Uint128>), // min / max
    pools: &[Box<dyn OsmosisPool>],
    swap_info: &[SwapInfo],
    err_tolerance: Option<Decimal>,
) -> Result<EstimateResult, ContractError> {
    let (max_est_in, max_est_out, max_routes) = search_efficient(
        deps,
        units,
        input_asset.clone(),
        init_index_amount,
        pools,
        swap_info,
    )?;
    deps.api.debug(&format!(
        "from_contract    => max_est_in: {}, max_est_out: {}",
        max_est_in, max_est_out
    ));
    if let Some(boundary) = min_index_amount {
        if max_est_out < boundary {
            return Err(ContractError::TradeAmountExceeded {});
        }
    }

    let tol = err_tolerance.unwrap_or_else(|| Decimal::from_ratio(1u64, 3u64));
    let est_out = match (min_index_amount, max_index_amount) {
        (None, Some(max)) => {
            let gap = max.checked_sub(max_est_out)?;
            max_est_out.checked_add(gap * tol)?
        }
        (Some(min), None) => {
            let gap = max_est_out.checked_sub(min)?;
            max_est_out.checked_sub(gap * tol)?
        }
        _ => return Err(ContractError::InvalidIndexAmountRange),
    };

    let (est_in, routes) =
        calc_index_per_token(deps, units, est_out, &input_asset.denom, pools, swap_info)?;
    deps.api.debug(&format!(
        "from_contract    =>     est_in: {},     est_out: {}",
        est_in, est_out
    ));
    if let Some(boundary) = max_index_amount {
        if boundary < est_out {
            return Err(ContractError::TradeAmountExceeded {});
        }
    }

    Ok(EstimateResult {
        max_est_in,
        max_est_out,
        max_routes,
        est_in,
        est_out,
        routes,
    })
}
