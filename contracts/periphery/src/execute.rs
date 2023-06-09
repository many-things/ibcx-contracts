use cosmwasm_std::{
    attr, coin, to_binary, BankMsg, Coin, CosmosMsg, Env, MessageInfo, Uint128, WasmMsg,
};
use cosmwasm_std::{DepsMut, Response};
use ibcx_interface::periphery::{extract_pool_ids, ExecuteMsg, SwapInfo};
use ibcx_interface::{core, helpers::IbcCore};

use crate::error::ContractError;
use crate::pool::query_pools;
use crate::sim::Simulator;
use crate::{coin_sorter, deduct_fee, expand_fee, make_unit_converter};

pub fn mint_exact_amount_in(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    core_addr: String,
    desired_denom: String,
    min_index_amount: Uint128,
    swap_info: Vec<SwapInfo>,
) -> Result<Response, ContractError> {
    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_fee = core.get_fee(&deps.querier, None)?;
    let core_config = core.get_config(&deps.querier, None)?;
    let core_portfolio = core.get_portfolio(&deps.querier, None)?;

    let desired_asset =
        cw_utils::must_pay(&info, &desired_denom).map(|v| coin(v.u128(), &desired_denom))?;

    let pool_ids = extract_pool_ids(swap_info.clone());
    let pools = query_pools(&deps.as_ref(), pool_ids)?;

    let deps_ref = deps.as_ref();
    let sim = Simulator::new(&deps_ref, &pools, &swap_info, &core_portfolio.units);
    let sim_res = sim
        .estimate_index_for_input(
            desired_asset.clone(),
            Some(min_index_amount),
            Some(min_index_amount),
            None,
        )?
        .est
        .unwrap();

    let swap_msgs = sim_res.sim_routes.to_msgs(
        &env.contract.address,
        sim_res.max_token_in + Uint128::new(10000),
    )?;

    let finish_msg = WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        msg: to_binary(&ExecuteMsg::FinishOperation {
            refund_to: info.sender.to_string(),
            refund_asset: desired_denom,
        })?,
        funds: vec![],
    };

    let mut mint_msg_funds = core_portfolio
        .units
        .into_iter()
        .map(|(denom, unit)| coin((sim_res.est_min_token_out * unit).u128(), denom))
        .collect::<Vec<_>>();
    mint_msg_funds.sort_by(coin_sorter);

    let mint_msg = core.call_with_funds(
        core::ExecuteMsg::Mint {
            amount: sim_res.est_min_token_out,
            receiver: Some(info.sender.to_string()),
            refund_to: Some(info.sender.to_string()),
        },
        mint_msg_funds,
    )?;

    let act_mint_amount = sim_res.est_min_token_out * deduct_fee(core_fee.mint_fee)?;
    let act_mint_asset = coin(act_mint_amount.u128(), core_config.index_denom);

    let resp = Response::new()
        .add_messages(swap_msgs)
        .add_message(mint_msg)
        .add_message(finish_msg)
        .add_attributes(vec![
            attr("method", "mint_exact_amount_in"),
            attr("executor", info.sender),
            attr("input", desired_asset.to_string()),
            attr("min_output", act_mint_asset.to_string()),
        ]);

    Ok(resp)
}

pub fn mint_exact_amount_out(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    core_addr: String,
    index_amount: Uint128,
    input_denom: String,
    swap_info: Vec<SwapInfo>,
) -> Result<Response, ContractError> {
    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_fee = core.get_fee(&deps.querier, None)?;
    let core_config = core.get_config(&deps.querier, None)?;
    let core_portfolio = core.get_portfolio(&deps.querier, None)?;

    // input & output
    let input_asset =
        cw_utils::must_pay(&info, &input_denom).map(|v| coin(v.u128(), &input_denom))?;
    let index_asset = coin(index_amount.u128(), &core_config.index_denom);

    let pool_ids = extract_pool_ids(swap_info.clone());
    let pools = query_pools(&deps.as_ref(), pool_ids)?;

    let deps_ref = deps.as_ref();
    let sim = Simulator::new(&deps_ref, &pools, &swap_info, &core_portfolio.units);
    let sim_res = sim.estimate_input_for_index(&input_asset.denom, index_asset.amount)?;
    let sim_refund = input_asset.amount.checked_sub(sim_res.total_input)?;

    let swap_msgs = sim_res
        .sim_routes
        .to_msgs(&env.contract.address, input_asset.amount)?;

    let conv = make_unit_converter(sim_res.index_out);
    let mut mint_msg_funds: Vec<_> = core_portfolio.units.into_iter().map(conv).collect();
    mint_msg_funds.sort_by(coin_sorter);

    let mint_msg = core.call_with_funds(
        core::ExecuteMsg::Mint {
            amount: index_asset.amount,
            receiver: Some(info.sender.to_string()),
            refund_to: Some(info.sender.to_string()),
        },
        mint_msg_funds,
    )?;

    let finish_msg = WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        msg: to_binary(&ExecuteMsg::FinishOperation {
            refund_to: info.sender.to_string(),
            refund_asset: input_asset.denom.clone(),
        })?,
        funds: vec![],
    };

    let act_mint_amount = index_asset.amount * deduct_fee(core_fee.mint_fee)?;
    let act_mint_asset = coin(act_mint_amount.u128(), &core_config.index_denom);

    let resp = Response::new()
        .add_messages(swap_msgs)
        .add_message(mint_msg)
        .add_message(finish_msg)
        .add_attributes(vec![
            attr("method", "mint_exact_amount_out"),
            attr("executor", info.sender),
            attr("max_input", input_asset.to_string()),
            attr("output", act_mint_asset.to_string()),
            attr("refund", sim_refund.to_string()),
        ]);

    Ok(resp)
}

pub fn burn_exact_amount_in(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    core_addr: String,
    output_denom: String,
    min_output_amount: Uint128,
    swap_info: Vec<SwapInfo>,
) -> Result<Response, ContractError> {
    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_fee = core.get_fee(&deps.querier, None)?;
    let core_config = core.get_config(&deps.querier, None)?;
    let core_portfolio = core.get_portfolio(&deps.querier, None)?;

    // input & output
    let index_asset = cw_utils::must_pay(&info, &core_config.index_denom)
        .map(|v| coin(v.u128(), &core_config.index_denom))?;
    let output_asset = coin(min_output_amount.u128(), output_denom);

    let pool_ids = extract_pool_ids(swap_info.clone());
    let pools = query_pools(&deps.as_ref(), pool_ids)?;

    let act_burn_amount = index_asset.amount * deduct_fee(core_fee.burn_fee)?;

    let deps_ref = deps.as_ref();
    let sim = Simulator::new(&deps_ref, &pools, &swap_info, &core_portfolio.units);
    let sim_res = sim.estimate_output_for_index(act_burn_amount, &output_asset.denom)?;

    let burn_msg = core.call_with_funds(
        core::ExecuteMsg::Burn { redeem_to: None },
        vec![index_asset.clone()],
    )?;

    let swap_msgs = sim_res
        .sim_routes
        .to_msgs(&env.contract.address, sim_res.total_output)?;

    let finish_msg = WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        msg: cosmwasm_std::to_binary(&ExecuteMsg::FinishOperation {
            refund_to: info.sender.to_string(),
            refund_asset: output_asset.denom.clone(),
        })?,
        funds: vec![],
    };

    let resp = Response::new()
        .add_message(burn_msg)
        .add_messages(swap_msgs)
        .add_message(finish_msg)
        .add_attributes(vec![
            attr("method", "burn_exact_amount_in"),
            attr("executor", info.sender),
            attr("input_amount", index_asset.to_string()),
            attr("min_output_amount", output_asset.to_string()),
        ]);

    Ok(resp)
}

pub fn burn_exact_amount_out(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    core_addr: String,
    desired_output: Coin,
    swap_info: Vec<SwapInfo>,
) -> Result<Response, ContractError> {
    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_fee = core.get_fee(&deps.querier, None)?;
    let core_config = core.get_config(&deps.querier, None)?;
    let core_portfolio = core.get_portfolio(&deps.querier, None)?;

    let index_asset = cw_utils::must_pay(&info, &core_config.index_denom)
        .map(|v| coin(v.u128(), &core_config.index_denom))?;

    let pool_ids = extract_pool_ids(swap_info.clone());
    let pools = query_pools(&deps.as_ref(), pool_ids)?;

    let deps_ref = deps.as_ref();
    let sim = Simulator::new(&deps_ref, &pools, &swap_info, &core_portfolio.units);
    let sim_res = sim.estimate_index_for_output(
        desired_output.clone(),
        Some(index_asset.amount),
        Some(index_asset.amount),
        None,
    )?;

    let burn_amount =
        sim_res.min.est_min_token_in * expand_fee(core_fee.burn_fee)? - Uint128::new(1);
    if index_asset.amount < burn_amount {
        return Err(ContractError::TradeAmountExceeded {});
    }

    let burn_msg = core.call_with_funds(
        core::ExecuteMsg::Burn { redeem_to: None },
        vec![coin(burn_amount.u128(), &core_config.index_denom)],
    )?;

    let act_burn_amount = burn_amount * deduct_fee(core_fee.burn_fee)?;

    let sim_res = sim.estimate_output_for_index(act_burn_amount, &desired_output.denom)?;

    let swap_msgs = sim_res
        .sim_routes
        .to_msgs(&env.contract.address, sim_res.total_output)?;

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
                refund_asset: desired_output.denom.clone(),
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
            attr("max_input", index_asset.to_string()),
            attr("output", desired_output.to_string()),
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
