use std::collections::BTreeMap;

use cosmwasm_std::CosmosMsg;
use cosmwasm_std::{Addr, Coin, Decimal, Deps, Uint128};
use ibcx_interface::periphery::{extract_pool_ids, SwapInfo};

use crate::error::ContractError;
use crate::pool::query_pools;
use crate::sim::{
    estimate_in_given_out, estimate_out_given_in, SimAmountInRoute, SimAmountOutRoute,
};

pub fn make_mint_swap_exact_out_msgs(
    deps: &Deps,
    contract: &Addr,
    swap_info: Vec<SwapInfo>,
    desired: Vec<Coin>,
    max_input: &Coin,
) -> Result<(Vec<CosmosMsg>, Uint128), ContractError> {
    let pool_ids = extract_pool_ids(swap_info.clone());
    let mut pools = query_pools(deps, pool_ids)?
        .into_iter()
        .map(|v| (v.get_id(), v))
        .collect::<BTreeMap<_, _>>();

    let simulated = desired
        .into_iter()
        .map(|v| {
            if v.denom == max_input.denom {
                return Ok(SimAmountInRoute {
                    sim_amount_in: v.amount,
                    amount_out: v,
                    routes: None,
                });
            }

            let route = estimate_in_given_out(deps, &max_input.denom, v, &mut pools, &swap_info)?;

            Ok(route)
        })
        .collect::<Result<Vec<_>, ContractError>>()?;

    let simulated_total_spend_amount = simulated
        .iter()
        .fold(Uint128::zero(), |acc, v| acc + v.sim_amount_in);
    if max_input.amount < simulated_total_spend_amount {
        return Err(ContractError::TradeAmountExceeded {});
    }

    let amplifier =
        Decimal::checked_from_ratio(max_input.amount, simulated_total_spend_amount).unwrap();
    let swap_msgs = simulated
        .into_iter()
        .filter_map(|r| {
            r.routes.map(|mut routes| {
                let amount_in_max = r.sim_amount_in * amplifier;

                routes.0.reverse();
                routes.msg_swap_exact_out(
                    contract,
                    &r.amount_out.denom,
                    r.amount_out.amount,
                    amount_in_max,
                )
            })
        })
        .collect::<Vec<_>>();

    let refund = max_input.amount.checked_sub(simulated_total_spend_amount)?;

    Ok((swap_msgs, refund))
}

pub fn make_burn_swap_exact_in_msgs(
    deps: &Deps,
    contract: &Addr,
    swap_info: Vec<SwapInfo>,
    expected: Vec<Coin>,
    min_output: &Coin,
) -> Result<(Vec<CosmosMsg>, Uint128), ContractError> {
    let pool_ids = extract_pool_ids(swap_info.clone());
    let mut pools = query_pools(deps, pool_ids)?
        .into_iter()
        .map(|v| (v.get_id(), v))
        .collect::<BTreeMap<_, _>>();

    let simulated = expected
        .into_iter()
        .map(|v| {
            if v.denom == min_output.denom {
                return Ok(SimAmountOutRoute {
                    sim_amount_out: v.amount,
                    amount_in: v,
                    routes: None,
                });
            }
            let route = estimate_out_given_in(deps, v, &min_output.denom, &mut pools, &swap_info)?;

            Ok(route)
        })
        .collect::<Result<Vec<_>, ContractError>>()?;

    let simulated_total_receive_amount = simulated
        .iter()
        .fold(Uint128::zero(), |acc, v| acc + v.sim_amount_out);
    if min_output.amount > simulated_total_receive_amount {
        return Err(ContractError::TradeAmountExceeded {});
    }

    let amplifier = Decimal::checked_from_ratio(min_output.amount, simulated_total_receive_amount)?;
    let swap_msgs = simulated
        .into_iter()
        .filter_map(|r| {
            r.routes.map(|routes| {
                let amount_out = r.sim_amount_out * amplifier;
                routes.msg_swap_exact_in(
                    contract,
                    &r.amount_in.denom,
                    r.amount_in.amount,
                    amount_out,
                )
            })
        })
        .collect::<Vec<_>>();

    Ok((swap_msgs, simulated_total_receive_amount))
}
