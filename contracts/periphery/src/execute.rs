use std::collections::HashSet;

use cosmwasm_std::{
    attr, coin, to_binary, BankMsg, Env, MessageInfo, QuerierWrapper, StdResult, SubMsg, Uint128,
    WasmMsg,
};
use cosmwasm_std::{DepsMut, Response};
use ibcx_interface::periphery::{ExecuteMsg, SwapInfo};
use ibcx_interface::{core, helpers::IbcCore};

use osmosis_std::types::osmosis::gamm::v1beta1::{QueryPoolRequest, QueryPoolResponse};

use crate::pool::resps_to_pools;
use crate::state::{Context, CONTEXT};
use crate::REPLY_ID_BURN_EXACT_AMOUNT_IN;
use crate::{error::ContractError, msgs::make_mint_swap_exact_out_msgs};

fn query_pool_infos(
    querier: &QuerierWrapper,
    pool_ids: Vec<u64>,
) -> StdResult<Vec<QueryPoolResponse>> {
    pool_ids
        .into_iter()
        .map(|v| querier.query(&QueryPoolRequest { pool_id: v }.into()))
        .collect()
}

fn extract_pool_ids(swap_info: Vec<SwapInfo>) -> Vec<u64> {
    swap_info
        .into_iter()
        .flat_map(|v| v.0 .1 .0.into_iter().map(|r| r.pool_id))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
}

pub fn mint_exact_amount_in(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _core_addr: String,
    _input_asset: String,
    _min_output_amount: Uint128,
    swap_info: Vec<SwapInfo>,
) -> Result<Response, ContractError> {
    let pool_ids = extract_pool_ids(swap_info);
    let pool_resps = query_pool_infos(&deps.querier, pool_ids)?;

    let pools = resps_to_pools(pool_resps)?;

    deps.api.debug(&format!("{:?}", pools));

    Ok(Response::default())
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
        &deps.querier,
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
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _core_addr: String,
    _output_asset: String,
    _output_amount: Uint128,
    _swap_info: Vec<SwapInfo>,
) -> Result<Response, ContractError> {
    Ok(Response::default())
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

    deps.api
        .debug(format!("finish balance {balance:?}").as_str());

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
