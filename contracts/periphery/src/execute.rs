use cosmwasm_std::{attr, coin, Env, MessageInfo, SubMsg, Uint128};
use cosmwasm_std::{DepsMut, Response};
use ibcx_interface::periphery::RouteKey;
use ibcx_interface::{core, helpers::IbcCore, types::SwapRoutes};

use crate::state::{Context, CONTEXT};
use crate::REPLY_ID_BURN_EXACT_AMOUNT_IN;
use crate::{error::ContractError, msgs::make_mint_swap_exact_out_msgs};

pub fn mint_exact_amount_out(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    core_addr: String,
    output_amount: Uint128,
    input_asset: String,
    swap_info: Vec<(RouteKey, SwapRoutes)>,
) -> Result<Response, ContractError> {
    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_config = core.get_config(&deps.querier)?;

    // input & output
    let max_input_amount = cw_utils::must_pay(&info, &input_asset)?;
    let max_input = coin(max_input_amount.u128(), &input_asset);
    let output = coin(output_amount.u128(), core_config.denom);

    let sim_resp = core.simulate_mint(&deps.querier, output.amount, None)?;

    let (swap_msgs, _) = make_mint_swap_exact_out_msgs(
        &deps.querier,
        &env.contract.address,
        &info.sender,
        swap_info,
        sim_resp.fund_spent.clone(),
        &max_input,
    )?;

    let mint_msg = core.call_with_funds(
        core::ExecuteMsg::Mint {
            amount: output.amount,
            receiver: Some(info.sender.to_string()),
            refund_to: Some(info.sender.to_string()),
        },
        sim_resp.fund_spent,
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
    _env: Env,
    info: MessageInfo,
    core_addr: String,
    output_asset: String,
    min_output_amount: Uint128,
    swap_info: Vec<(RouteKey, SwapRoutes)>,
) -> Result<Response, ContractError> {
    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_config = core.get_config(&deps.querier)?;

    // input & output
    let input_amount = cw_utils::must_pay(&info, &core_config.denom)?;
    let input = coin(input_amount.u128(), &core_config.denom);
    let min_output = coin(min_output_amount.u128(), output_asset);

    let expected = core
        .simulate_burn(&deps.querier, input.amount)?
        .redeem_amount;

    let burn_msg = core.call_with_funds(
        core::ExecuteMsg::Burn { redeem_to: None },
        vec![coin(input.amount.u128(), &core_config.denom)],
    )?;

    // save to context
    CONTEXT.save(
        deps.storage,
        &Context::Burn {
            core: core.addr(),
            sender: info.sender.clone(),
            input: input.clone(),
            min_output: min_output.clone(),
            redeem_amounts: expected,
            swap_info,
        },
    )?;

    let resp = Response::new()
        .add_submessage(SubMsg::reply_on_success(
            burn_msg,
            REPLY_ID_BURN_EXACT_AMOUNT_IN,
        ))
        .add_attributes(vec![
            attr("method", "burn_exact_amount_in"),
            attr("executor", info.sender),
            attr("input_amount", input.to_string()),
            attr("min_output_amount", min_output.to_string()),
        ]);

    Ok(resp)
}
