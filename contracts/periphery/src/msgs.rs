use cosmwasm_std::{coin, Addr, BankMsg, Coin, Uint128};
use cosmwasm_std::{CosmosMsg, QuerierWrapper};
use ibcx_interface::periphery::RouteKey;
use ibcx_interface::{core, types::SwapRoutes};

use crate::error::ContractError;

pub fn make_mint_swap_exact_out_msgs(
    querier: &QuerierWrapper,
    compat: &Addr,
    contract: &Addr,
    sender: &Addr,
    swap_info: Vec<(RouteKey, SwapRoutes)>,
    desired: Vec<Coin>,
    max_input: &Coin,
) -> Result<(Vec<CosmosMsg>, Uint128), ContractError> {
    let mut swap_msgs: Vec<CosmosMsg> = Vec::new();
    let mut simulated_total_spend_amount = Uint128::zero();

    for Coin {
        denom,
        amount: want,
    } in desired
    {
        if denom == max_input.denom {
            // skip swap for reserve denom
            simulated_total_spend_amount += want;
            continue;
        }

        let (_, swap_info) = swap_info
            .iter()
            .find(|(RouteKey((from, to)), _)| from == &max_input.denom && to == &denom)
            .ok_or(ContractError::SwapRouteNotFound {
                from: max_input.denom.clone(),
                to: denom.clone(),
            })?
            .clone();

        let simulated_token_in = swap_info
            .sim_swap_exact_out(querier, compat, contract, coin(want.u128(), &denom))
            .map_err(|e| ContractError::SimulateQueryError {
                err: e.to_string(),
                input: max_input.denom.clone(),
                output: denom.clone(),
                amount: want,
            })?;

        simulated_total_spend_amount += simulated_token_in;

        swap_msgs.push(swap_info.msg_swap_exact_out(contract, &denom, want, simulated_token_in));
    }

    if max_input.amount < simulated_total_spend_amount {
        return Err(ContractError::TradeAmountExceeded {});
    }

    let refund = max_input.amount.checked_sub(simulated_total_spend_amount)?;

    swap_msgs.push(
        BankMsg::Send {
            to_address: sender.to_string(),
            amount: vec![coin(refund.u128(), &max_input.denom)],
        }
        .into(),
    );

    Ok((swap_msgs, refund))
}

pub fn make_burn_swap_msgs(
    querier: &QuerierWrapper,
    config: &core::GetConfigResponse,
    contract: &Addr,
    sender: &Addr,
    swap_info: Vec<(RouteKey, SwapRoutes)>,
    expected: Vec<Coin>,
    min_output: &Coin,
) -> Result<(Vec<CosmosMsg>, Uint128), ContractError> {
    let mut swap_msgs: Vec<CosmosMsg> = Vec::new();
    let mut simulated_total_receive_amount = Uint128::zero();

    for Coin {
        denom,
        amount: expected,
    } in expected
    {
        if min_output.denom == denom {
            // skip swap for reserve denom
            simulated_total_receive_amount += expected;
            continue;
        }

        let (_, swap_info) = swap_info
            .iter()
            .find(|(RouteKey((from, to)), _)| from == &denom && to == &min_output.denom)
            .ok_or(ContractError::SwapRouteNotFound {
                from: min_output.denom.clone(),
                to: denom.clone(),
            })?;

        let simulated_token_out = swap_info
            .sim_swap_exact_in(
                querier,
                &config.compat,
                contract,
                coin(expected.u128(), &denom),
            )
            .map_err(|e| ContractError::SimulateQueryError {
                err: e.to_string(),
                input: min_output.denom.clone(),
                output: denom.clone(),
                amount: expected,
            })?;

        simulated_total_receive_amount += simulated_token_out;

        swap_msgs.push(swap_info.msg_swap_exact_in(
            contract,
            &denom,
            expected,
            simulated_token_out,
        ));
    }

    if min_output.amount > simulated_total_receive_amount {
        return Err(ContractError::TradeAmountExceeded {});
    }

    swap_msgs.push(
        BankMsg::Send {
            to_address: sender.to_string(),
            amount: vec![coin(
                simulated_total_receive_amount.u128(),
                &min_output.denom,
            )],
        }
        .into(),
    );

    Ok((swap_msgs, simulated_total_receive_amount))
}
