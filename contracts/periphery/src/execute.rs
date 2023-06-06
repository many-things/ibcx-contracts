use cosmwasm_std::{
    attr, coin, to_binary, BankMsg, Coin, CosmosMsg, Decimal, Env, MessageInfo, SubMsg, Uint128,
    WasmMsg,
};
use cosmwasm_std::{DepsMut, Response};
use ibcx_interface::periphery::{extract_pool_ids, ExecuteMsg, SwapInfo};
use ibcx_interface::{core, helpers::IbcCore};

use crate::pool::query_pools;
use crate::sim::estimate_max_index_for_input;
use crate::state::{Context, CONTEXT};
use crate::REPLY_ID_BURN_EXACT_AMOUNT_IN;
use crate::{error::ContractError, msgs::make_mint_swap_exact_out_msgs};

pub fn mint_exact_amount_in(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    core_addr: String,
    input_asset: String,
    min_output_amount: Uint128,
    swap_info: Vec<SwapInfo>,
) -> Result<Response, ContractError> {
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_config = core.get_config(&deps.querier, None)?;
    let core_portfolio = core.get_portfolio(&deps.querier, None)?;

    let input_amount = cw_utils::must_pay(&info, &input_asset)?;
    let input_token = coin(input_amount.u128(), &input_asset);

    let pool_ids = extract_pool_ids(swap_info.clone());
    let pools = query_pools(&deps.as_ref(), pool_ids)?;

    let est_res = estimate_max_index_for_input(
        &deps.as_ref(),
        &core_portfolio.units,
        input_token.clone(),
        Some(min_output_amount),
        (Some(min_output_amount), None),
        &pools,
        &swap_info,
        None,
    )?;

    let amplifier = Decimal::checked_from_ratio(est_res.max_est_out, est_res.est_out)?;
    let swap_msgs = est_res
        .routes
        .into_iter()
        .filter_map(|r| {
            r.routes.map(|mut routes| {
                routes.0.reverse();
                routes.msg_swap_exact_out(
                    &env.contract.address,
                    &r.amount_out.denom,
                    r.amount_out.amount,
                    r.sim_amount_in * amplifier,
                )
            })
        })
        .collect::<Vec<_>>();

    let finish_msg = WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        msg: to_binary(&ExecuteMsg::FinishOperation {
            refund_to: info.sender.to_string(),
            refund_asset: input_asset,
        })?,
        funds: vec![],
    };

    let mut mint_msg_funds = core_portfolio
        .units
        .into_iter()
        .map(|(denom, unit)| coin((est_res.est_out * unit).u128(), denom))
        .collect::<Vec<_>>();
    mint_msg_funds.sort_by(|a, b| a.denom.cmp(&b.denom));

    let mint_msg = core.call_with_funds(
        core::ExecuteMsg::Mint {
            amount: est_res.est_out,
            receiver: Some(info.sender.to_string()),
            refund_to: Some(info.sender.to_string()),
        },
        mint_msg_funds,
    )?;

    let resp = Response::new()
        .add_messages(swap_msgs)
        .add_message(mint_msg)
        .add_message(finish_msg)
        .add_attributes(vec![
            attr("method", "mint_exact_amount_in"),
            attr("executor", info.sender),
            attr("input", input_token.to_string()),
            attr(
                "min_output",
                coin(est_res.est_out.u128(), core_config.index_denom).to_string(),
            ),
        ]);

    Ok(resp)
}

pub fn mint_exact_amount_out(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    core_addr: String,
    output_amount: Uint128,
    input_asset: String,
    swap_info: Vec<SwapInfo>,
) -> Result<Response, ContractError> {
    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_config = core.get_config(&deps.querier, None)?;

    // input & output
    let max_input_amount = cw_utils::must_pay(&info, &input_asset)?;
    let max_input = coin(max_input_amount.u128(), &input_asset);
    let output = coin(output_amount.u128(), core_config.index_denom);

    let sim_resp = core.simulate_mint(&deps.querier, output.amount, None, None)?;
    let mut sim_amount_desired = sim_resp.fund_spent;
    sim_amount_desired.sort_by(|a, b| a.denom.cmp(&b.denom));

    let (swap_msgs, refund) = make_mint_swap_exact_out_msgs(
        &deps.as_ref(),
        &env.contract.address,
        swap_info,
        sim_amount_desired.clone(),
        &max_input,
    )?;

    let finish_msg = WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        msg: to_binary(&ExecuteMsg::FinishOperation {
            refund_to: info.sender.to_string(),
            refund_asset: input_asset,
        })?,
        funds: vec![],
    };

    let mint_msg = core.call_with_funds(
        core::ExecuteMsg::Mint {
            amount: output.amount,
            receiver: Some(info.sender.to_string()),
            refund_to: Some(info.sender.to_string()),
        },
        sim_amount_desired,
    )?;

    let resp = Response::new()
        .add_messages(swap_msgs)
        .add_message(mint_msg)
        .add_message(finish_msg)
        .add_attributes(vec![
            attr("method", "mint_exact_amount_out"),
            attr("executor", info.sender),
            attr("max_input", max_input.to_string()),
            attr("output", output.to_string()),
            attr("refund", refund.to_string()),
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
    swap_info: Vec<SwapInfo>,
) -> Result<Response, ContractError> {
    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_config = core.get_config(&deps.querier, None)?;

    // input & output
    let input_amount = cw_utils::must_pay(&info, &core_config.index_denom)?;
    let input = coin(input_amount.u128(), &core_config.index_denom);
    let min_output = coin(min_output_amount.u128(), output_asset);

    let expected = core
        .simulate_burn(&deps.querier, input.amount, None)?
        .redeem_amount;

    let burn_msg = core.call_with_funds(
        core::ExecuteMsg::Burn { redeem_to: None },
        vec![coin(input.amount.u128(), &core_config.index_denom)],
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

pub fn burn_exact_amount_out(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    core_addr: String,
    output_asset: Coin,
    swap_info: Vec<SwapInfo>,
) -> Result<Response, ContractError> {
    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_config = core.get_config(&deps.querier, None)?;
    let core_fee = core.get_fee(&deps.querier, None)?;
    let core_portfolio = core.get_portfolio(&deps.querier, None)?;

    let input_amount = cw_utils::must_pay(&info, &core_config.index_denom)?;
    let input_token = coin(input_amount.u128(), &core_config.index_denom);

    let pool_ids = extract_pool_ids(swap_info.clone());
    let pools = query_pools(&deps.as_ref(), pool_ids)?;

    // index -> units -> token
    let est_res = estimate_max_index_for_input(
        &deps.as_ref(),
        &core_portfolio.units,
        output_asset.clone(),
        Some(input_token.amount),
        (None, Some(input_token.amount)),
        &pools,
        &swap_info,
        None,
    )?;

    let invert = Decimal::one().checked_div(
        core_fee
            .burn_fee
            .map(|v| Decimal::one() - v)
            .unwrap_or(Decimal::one()),
    )?;

    let burn_amount = coin(
        (est_res.max_est_out * invert).u128(),
        &core_config.index_denom,
    );
    deps.api.debug(&format!("input_token: {}", input_token));
    deps.api.debug(&format!("burn_amount: {}", burn_amount));
    let burn_msg = core.call_with_funds(
        core::ExecuteMsg::Burn { redeem_to: None },
        vec![burn_amount],
    )?;

    let _amplifier = Decimal::checked_from_ratio(est_res.est_out, est_res.max_est_out)?;
    let swap_msgs = est_res
        .max_routes
        .into_iter()
        .filter_map(|r| {
            r.routes.map(|routes| {
                routes.msg_swap_exact_in(
                    &env.contract.address,
                    &r.amount_out.denom,
                    r.amount_out.amount,
                    Uint128::one(),
                    // r.sim_amount_in * amplifier,
                )
            })
        })
        .collect::<Vec<_>>();

    let finish_msgs: Vec<CosmosMsg> = vec![
        WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_binary(&ExecuteMsg::FinishOperation {
                refund_to: info.sender.to_string(),
                refund_asset: core_config.index_denom,
            })?,
            funds: vec![],
        }
        .into(),
        WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_binary(&ExecuteMsg::FinishOperation {
                refund_to: info.sender.to_string(),
                refund_asset: output_asset.denom.clone(),
            })?,
            funds: vec![],
        }
        .into(),
    ];

    let resp = Response::new()
        .add_message(burn_msg)
        .add_messages(swap_msgs)
        .add_messages(finish_msgs)
        .add_attributes(vec![
            attr("method", "burn_exact_amount_out"),
            attr("executor", info.sender),
            attr("max_input", input_token.to_string()),
            attr("output", output_asset.to_string()),
        ]);

    Ok(resp)
}

pub fn finish_operation(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    refund_to: String,
    refund_asset: String,
) -> Result<Response, ContractError> {
    assert_eq!(info.sender, env.contract.address, "internal function");
    deps.api.addr_validate(&refund_to)?;

    let balance = deps
        .querier
        .query_balance(env.contract.address, &refund_asset)?;

    let resp = Response::new()
        .add_attributes(vec![
            attr("method", "finish_operation"),
            attr("refund_to", &refund_to),
            attr("refund_asset", refund_asset),
            attr("amount", balance.amount),
        ])
        .add_message(BankMsg::Send {
            to_address: refund_to,
            amount: vec![balance],
        });

    Ok(resp)
}
