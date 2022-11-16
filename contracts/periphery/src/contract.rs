use std::collections::BTreeMap;

use cosmwasm_std::{
    attr, coin, coins, entry_point, Addr, BankMsg, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    QueryResponse, Reply, Response, SubMsg, Uint128,
};
use ibc_interface::{
    core,
    helpers::IbcCore,
    periphery::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    types::SwapRoute,
};
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin,
    osmosis::gamm::v1beta1::{MsgSwapExactAmountIn, SwapAmountInRoute},
};

use crate::{
    error::ContractError,
    state::{Config, Context, CONFIG, CONTEXTS, CURRENT_CONTEXT_ID},
    CONTRACT_NAME, CONTRACT_VERSION, REPLY_ID_BURN, REPLY_ID_MINT,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(
        deps.storage,
        &Config {
            core: deps.api.addr_validate(&msg.core)?,
        },
    )?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "instantiate"),
        attr("core", msg.core),
        // TODO: add more attributes
    ]);

    Ok(resp)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        MintExactAmountOut {
            core_addr,
            amount,
            input_asset,
            max_input_amount,
            swap_info,
        } => {
            let core = IbcCore(deps.api.addr_validate(&core_addr)?);
            let core_config = core.get_config(&deps.querier)?;
            let core_portfolio = core.get_portfolio(&deps.querier)?;

            let reserve_unit = core_portfolio
                .reserve
                .checked_div(core_portfolio.total_supply)?;

            let mut portfolio: BTreeMap<_, _> = core_portfolio.assets.into_iter().collect();

            portfolio
                .entry(core_config.reserve_denom.clone())
                .and_modify(|v| *v += reserve_unit)
                .or_insert(reserve_unit);

            let desired = portfolio
                .into_iter()
                .map(|(denom, unit)| (denom, unit * amount))
                .collect::<BTreeMap<_, _>>();

            let swap_info = swap_info
                .into_iter()
                .map(|v| (v.asset.clone(), v))
                .collect::<BTreeMap<_, _>>();

            let funds = desired
                .iter()
                .map(|(denom, want)| coin(want.u128(), denom))
                .collect();
            let mut swap_msgs: Vec<CosmosMsg> = Vec::new();
            let mut simulated_total_spend_amount = Uint128::zero();

            for (denom, want) in desired {
                if denom == core_config.reserve_denom {
                    simulated_total_spend_amount += want
                }

                let swap_info = swap_info.get(&denom).unwrap();

                let simulated_token_in = swap_info.simulate_swap_exact_out(
                    &deps.querier,
                    env.contract.address.to_string(),
                    denom.clone(),
                    want.clone(),
                )?;

                simulated_total_spend_amount += simulated_token_in;

                swap_msgs.push(swap_info.msg_swap_exact_out(
                    env.contract.address.to_string(),
                    simulated_token_in,
                    denom,
                    want,
                ));
            }

            if max_input_amount < simulated_total_spend_amount {
                return Err(ContractError::TradeAmountExceeded {});
            }

            let mint_msg = core.call_with_funds(
                core::ExecuteMsg::Mint {
                    amount,
                    receiver: info.sender.to_string(),
                },
                funds,
            )?;

            let current_context_id = CURRENT_CONTEXT_ID.load(deps.storage)?;

            CONTEXTS.save(
                deps.storage,
                current_context_id,
                &Context {
                    executor: info.sender,
                    asset_to_check: input_asset.clone(),
                },
            )?;

            let resp = Response::new()
                .add_messages(swap_msgs)
                .add_submessage(SubMsg::reply_on_success(mint_msg, REPLY_ID_MINT))
                .add_attributes(vec![
                    attr("method", "mint_exact_amount_out"),
                    attr("input_asset", input_asset),
                    attr("max_input_amount", max_input_amount),
                ]);

            Ok(resp)
        }

        BurnExactAmountIn {
            core_addr,
            output_asset,
            min_output_amount,
            swap_info,
        } => {
            let core = IbcCore(deps.api.addr_validate(&core_addr)?);
            let core_config = core.get_config(&deps.querier)?;
            let core_portfolio = core.get_portfolio(&deps.querier)?;

            let received = cw_utils::must_pay(&info, &core_config.denom)?;

            let reserve_unit = core_portfolio
                .reserve
                .checked_div(core_portfolio.total_supply)?;

            let mut portfolio: BTreeMap<_, _> = core_portfolio.assets.into_iter().collect();

            portfolio
                .entry(core_config.reserve_denom.clone())
                .and_modify(|v| *v += reserve_unit)
                .or_insert(reserve_unit);

            let expected = portfolio
                .into_iter()
                .map(|(denom, unit)| (denom, unit * received))
                .collect::<BTreeMap<_, _>>();

            let swap_info = swap_info
                .into_iter()
                .map(|v| (v.asset.clone(), v))
                .collect::<BTreeMap<_, _>>();

            let burn_msg = core.call_with_funds(
                core::ExecuteMsg::Burn {},
                coins(received.u128(), core_config.reserve_denom),
            )?;

            let mut swap_msgs: Vec<CosmosMsg> = Vec::new();
            let mut simulated_total_receive_amount = Uint128::zero();

            for (denom, expected) in expected {
                let swap_info = swap_info.get(&denom).unwrap();

                let simulated_token_out = swap_info.simulate_swap_exact_in(
                    &deps.querier,
                    env.contract.address.to_string(),
                    denom.clone(),
                    expected.clone(),
                )?;

                simulated_total_receive_amount += simulated_token_out;

                swap_msgs.push(swap_info.msg_swap_exact_in(
                    env.contract.address.to_string(),
                    simulated_token_out,
                    denom,
                    expected,
                ));
            }

            if min_output_amount > simulated_total_receive_amount {
                return Err(ContractError::TradeAmountExceeded {});
            }

            let last_swap_msg = swap_msgs.pop().unwrap();

            let current_context_id = CURRENT_CONTEXT_ID.load(deps.storage)?;

            CONTEXTS.save(
                deps.storage,
                current_context_id,
                &Context {
                    executor: info.sender,
                    asset_to_check: output_asset.clone(),
                },
            )?;

            let resp = Response::new()
                .add_message(burn_msg)
                .add_messages(swap_msgs)
                .add_submessage(SubMsg::reply_on_success(last_swap_msg, REPLY_ID_BURN))
                .add_attributes(vec![
                    attr("method", "burn_exact_amount_in"),
                    attr("output_asset", output_asset),
                    attr("min_output_amount", min_output_amount),
                ]);

            Ok(resp)
        }
    }
}

#[entry_point]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_ID_MINT => {
            // parse = validate
            cw_utils::parse_reply_execute_data(msg)?;

            let current_context_id = CURRENT_CONTEXT_ID.load(deps.storage)?;
            let context = CONTEXTS.load(deps.storage, current_context_id)?;
            CURRENT_CONTEXT_ID.save(deps.storage, &(current_context_id + 1))?;

            let balance = deps
                .querier
                .query_balance(&env.contract.address, context.asset_to_check)?;

            let resp = Response::new()
                .add_message(BankMsg::Send {
                    to_address: context.executor.to_string(),
                    amount: vec![balance.clone()],
                })
                .add_attributes(vec![
                    attr("method", "reply_mint_exact_amount_out"),
                    attr("dust", balance.to_string()),
                    attr("return_to", context.executor.into_string()),
                ]);

            Ok(resp)
        }
        REPLY_ID_BURN => {
            // parse = validate
            cw_utils::parse_reply_execute_data(msg)?;

            let current_context_id = CURRENT_CONTEXT_ID.load(deps.storage)?;
            let context = CONTEXTS.load(deps.storage, current_context_id)?;
            CURRENT_CONTEXT_ID.save(deps.storage, &(current_context_id + 1))?;

            let balance = deps
                .querier
                .query_balance(&env.contract.address, context.asset_to_check)?;

            let resp = Response::new()
                .add_message(BankMsg::Send {
                    to_address: context.executor.to_string(),
                    amount: vec![balance.clone()],
                })
                .add_attributes(vec![
                    attr("method", "reply_burn_exact_amount_out"),
                    attr("dust", balance.to_string()),
                    attr("return_to", context.executor.into_string()),
                ]);

            Ok(resp)
        }
        _ => Err(ContractError::InvalidReplyId {}),
    }
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    Ok(Default::default())
}

#[entry_point]
pub fn migrate(_deps: Deps, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Default::default())
}
