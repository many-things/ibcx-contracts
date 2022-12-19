use std::collections::BTreeMap;

use cosmwasm_std::{coin, Addr, BankMsg, Coin, Uint128};
use cosmwasm_std::{CosmosMsg, QuerierWrapper};
use ibc_interface::{core, types::SwapRoutes};

use crate::error::ContractError;

pub fn make_mint_swap_exact_out_msgs(
    querier: &QuerierWrapper,
    config: &core::GetConfigResponse,
    contract: &Addr,
    sender: &Addr,
    swap_info: BTreeMap<(String, String), SwapRoutes>,
    desired: BTreeMap<String, Uint128>,
    max_input: &Coin,
) -> Result<(Vec<CosmosMsg>, Uint128), ContractError> {
    let mut swap_msgs: Vec<CosmosMsg> = Vec::new();
    let mut simulated_total_spend_amount = Uint128::zero();

    for (denom, want) in desired {
        if denom == config.reserve_denom {
            // skip swap for reserve denom
            simulated_total_spend_amount += want;
            continue;
        }

        let swap_info = swap_info
            .get(&(config.reserve_denom.clone(), denom.clone()))
            .unwrap();

        let simulated_token_in = swap_info.sim_swap_exact_out(
            querier,
            &config.compat,
            contract,
            coin(want.u128(), &denom),
        )?;

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
    swap_info: BTreeMap<(String, String), SwapRoutes>,
    expected: BTreeMap<String, Uint128>,
    min_output: &Coin,
) -> Result<(Vec<CosmosMsg>, Uint128), ContractError> {
    let mut swap_msgs: Vec<CosmosMsg> = Vec::new();
    let mut simulated_total_receive_amount = Uint128::zero();

    for (denom, expected) in expected {
        let swap_info = swap_info
            .get(&(min_output.denom.clone(), denom.clone()))
            .unwrap();

        let simulated_token_out = swap_info.sim_swap_exact_in(
            querier,
            &config.compat,
            contract,
            coin(expected.u128(), &denom),
        )?;

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
