use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use cosmwasm_std::{CosmosMsg, QuerierWrapper};
use ibcx_interface::periphery::{RouteKey, SwapInfo};

use crate::error::ContractError;

pub fn make_mint_swap_exact_out_msgs(
    querier: &QuerierWrapper,
    contract: &Addr,
    swap_info: Vec<SwapInfo>,
    desired: Vec<Coin>,
    max_input: &Coin,
) -> Result<(Vec<CosmosMsg>, Uint128), ContractError> {
    let mut swap_msgs: Vec<CosmosMsg> = Vec::new();

    let simulated = desired
        .into_iter()
        .map(|v| {
            if v.denom == max_input.denom {
                return Ok((v.clone(), None, v.amount));
            }

            let SwapInfo((_, swap_info)) = swap_info
                .iter()
                .find(|SwapInfo((RouteKey((from, to)), _))| {
                    from == &max_input.denom && to == &v.denom
                })
                .ok_or(ContractError::SwapRouteNotFound {
                    from: max_input.denom.clone(),
                    to: v.denom.clone(),
                })?;

            let simulated_token_in =
                swap_info
                    .sim_swap_exact_out(querier, v.clone())
                    .map_err(|e| ContractError::SimulateQueryError {
                        err: e.to_string(),
                        input: max_input.denom.clone(),
                        output: v.denom.clone(),
                        amount: v.amount,
                    })?;

            Ok((v, Some(swap_info), simulated_token_in))
        })
        .collect::<Result<Vec<_>, ContractError>>()?;

    let simulated_total_spend_amount = simulated
        .iter()
        .fold(Uint128::zero(), |acc, (_, _, v)| acc + v);
    if max_input.amount < simulated_total_spend_amount {
        return Err(ContractError::TradeAmountExceeded {});
    }

    for (
        Coin {
            denom,
            amount: amount_out,
        },
        swap_info,
        sim_amount_in,
    ) in simulated
    {
        if let Some(swap_info) = swap_info {
            let ratio =
                Decimal::checked_from_ratio(max_input.amount, simulated_total_spend_amount)?;
            let amount_in_max = ratio * sim_amount_in;

            swap_msgs.push(swap_info.msg_swap_exact_out(
                contract,
                &denom,
                amount_out,
                amount_in_max,
            ));
        }
    }

    let refund = max_input.amount.checked_sub(simulated_total_spend_amount)?;

    Ok((swap_msgs, refund))
}

pub fn make_burn_swap_msgs(
    querier: &QuerierWrapper,
    contract: &Addr,
    swap_info: Vec<SwapInfo>,
    expected: Vec<Coin>,
    min_output: &Coin,
) -> Result<(Vec<CosmosMsg>, Uint128), ContractError> {
    let mut swap_msgs: Vec<CosmosMsg> = Vec::new();

    let simulated = expected
        .into_iter()
        .map(|v| {
            if v.denom == min_output.denom {
                return Ok((v.clone(), None, v.amount));
            }

            let SwapInfo((_, swap_info)) = swap_info
                .iter()
                .find(|SwapInfo((RouteKey((from, to)), _))| {
                    from == &v.denom && to == &min_output.denom
                })
                .ok_or(ContractError::SwapRouteNotFound {
                    from: min_output.denom.clone(),
                    to: v.denom.clone(),
                })?;

            let simulated_token_out =
                swap_info
                    .sim_swap_exact_in(querier, v.clone())
                    .map_err(|e| ContractError::SimulateQueryError {
                        err: e.to_string(),
                        input: v.denom.clone(),
                        output: min_output.denom.clone(),
                        amount: v.amount,
                    })?;

            Ok((v, Some(swap_info), simulated_token_out))
        })
        .collect::<Result<Vec<_>, ContractError>>()?;

    let simulated_total_receive_amount = simulated
        .iter()
        .fold(Uint128::zero(), |acc, (_, _, v)| acc + v);
    if min_output.amount > simulated_total_receive_amount {
        return Err(ContractError::TradeAmountExceeded {});
    }

    for (
        Coin {
            denom,
            amount: amount_in,
        },
        swap_info,
        sim_amount_out,
    ) in simulated
    {
        if let Some(swap_info) = swap_info {
            let ratio =
                Decimal::checked_from_ratio(min_output.amount, simulated_total_receive_amount)?;
            let amount_out_min = ratio * sim_amount_out;

            swap_msgs.push(swap_info.msg_swap_exact_in(
                contract,
                &denom,
                amount_in,
                amount_out_min,
            ));
        }
    }

    Ok((swap_msgs, simulated_total_receive_amount))
}
