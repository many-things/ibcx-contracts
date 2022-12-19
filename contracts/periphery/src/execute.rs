use std::collections::BTreeMap;

use cosmwasm_std::{attr, coin, Env, MessageInfo, Uint128};
use cosmwasm_std::{DepsMut, Response};
use ibc_interface::periphery::RouteKey;
use ibc_interface::{core, helpers::IbcCore, types::SwapRoutes};

use crate::{
    error::ContractError,
    msgs::{make_burn_swap_msgs, make_mint_swap_exact_out_msgs},
};

pub fn mint_exact_amount_out(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    core_addr: String,
    output_amount: Uint128,
    input_asset: String,
    swap_info: Vec<(RouteKey, SwapRoutes)>,
) -> Result<Response, ContractError> {
    // pre-transform swap_info
    let swap_info = swap_info
        .into_iter()
        .map(|(RouteKey((from, to)), routes)| ((from, to), routes))
        .collect::<BTreeMap<_, _>>();

    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_config = core.get_config(&deps.querier)?;
    let core_portfolio = core.get_portfolio(&deps.querier)?;

    // input & output
    let max_input_amount = cw_utils::must_pay(&info, &input_asset)?;
    let max_input = coin(max_input_amount.u128(), &input_asset);
    let output = coin(output_amount.u128(), &core_config.denom);

    let desired = core_portfolio
        .assets
        .into_iter()
        .map(|c| (c.denom, c.amount * output.amount))
        .collect::<BTreeMap<_, _>>();

    let funds = desired
        .iter()
        .map(|(denom, want)| coin(want.u128(), denom))
        .collect();

    let (swap_msgs, _) = make_mint_swap_exact_out_msgs(
        &deps.querier,
        &core_config,
        &env.contract.address,
        &info.sender,
        swap_info,
        desired,
        &max_input,
    )?;

    let mint_msg = core.call_with_funds(
        core::ExecuteMsg::Mint {
            amount: output.amount,
            receiver: Some(info.sender.to_string()),
            refund_to: Some(info.sender.to_string()),
        },
        funds,
    )?;

    let resp = Response::new()
        .add_messages(swap_msgs)
        .add_message(mint_msg)
        .add_attributes(vec![
            attr("method", "mint_exact_amount_out"),
            attr("executor", info.sender),
            attr("max_input", max_input.to_string()),
            attr("output", output.to_string()),
        ]);

    Ok(resp)
}

pub fn burn_exact_amount_in(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    core_addr: String,
    output_asset: String,
    min_output_amount: Uint128,
    swap_info: Vec<(RouteKey, SwapRoutes)>,
) -> Result<Response, ContractError> {
    // pre-transform swap_info
    let swap_info = swap_info
        .into_iter()
        .map(|(RouteKey((from, to)), routes)| ((from, to), routes))
        .collect::<BTreeMap<_, _>>();

    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_config = core.get_config(&deps.querier)?;
    let core_portfolio = core.get_portfolio(&deps.querier)?;

    // input & output
    let input_amount = cw_utils::must_pay(&info, &core_config.denom)?;
    let input = coin(input_amount.u128(), &core_config.denom);
    let min_output = coin(min_output_amount.u128(), output_asset);

    let expected = core_portfolio
        .assets
        .into_iter()
        .map(|c| (c.denom, c.amount * input.amount))
        .collect::<BTreeMap<_, _>>();

    let burn_msg = core.call_with_funds(
        core::ExecuteMsg::Burn {},
        vec![coin(input.amount.u128(), &core_config.reserve_denom)],
    )?;

    let (swap_msgs, _) = make_burn_swap_msgs(
        &deps.querier,
        &core_config,
        &env.contract.address,
        &info.sender,
        swap_info,
        expected,
        &min_output,
    )?;

    let resp = Response::new()
        .add_message(burn_msg)
        .add_messages(swap_msgs)
        .add_attributes(vec![
            attr("method", "burn_exact_amount_in"),
            attr("executor", info.sender),
            attr("input_amount", input.to_string()),
            attr("min_output_amount", min_output.to_string()),
        ]);

    Ok(resp)
}
