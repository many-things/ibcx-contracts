use std::collections::BTreeMap;

use cosmwasm_std::{
    attr, coin, coins, entry_point, Addr, BankMsg, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, QuerierWrapper, QueryResponse, Reply, Response, StdResult, Storage, SubMsg,
    Uint128,
};
use ibc_interface::{
    core,
    helpers::IbcCore,
    periphery::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, SwapInfo},
};

use crate::{
    error::ContractError,
    state::{Context, CONTEXTS, CURRENT_CONTEXT_ID},
    CONTRACT_NAME, CONTRACT_VERSION, REPLY_ID_BURN, REPLY_ID_MINT,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let resp = Response::new().add_attributes(vec![attr("method", "instantiate")]);

    Ok(resp)
}

fn make_mint_swap_msgs(
    querier: &QuerierWrapper,
    contract: &Addr,
    reserve_denom: String,
    swap_info: BTreeMap<String, SwapInfo>,
    desired: BTreeMap<String, Uint128>,
    max_input_amount: Uint128,
) -> Result<Vec<CosmosMsg>, ContractError> {
    let mut swap_msgs: Vec<CosmosMsg> = Vec::new();
    let mut simulated_total_spend_amount = Uint128::zero();

    for (denom, want) in desired {
        if denom == reserve_denom {
            simulated_total_spend_amount += want
        }

        let swap_info = swap_info.get(&denom).unwrap();

        let simulated_token_in = swap_info.simulate_swap_exact_out(
            querier,
            contract.to_string(),
            denom.clone(),
            want,
        )?;

        simulated_total_spend_amount += simulated_token_in;

        swap_msgs.push(swap_info.msg_swap_exact_out(
            contract.to_string(),
            simulated_token_in,
            denom,
            want,
        ));
    }

    if max_input_amount < simulated_total_spend_amount {
        return Err(ContractError::TradeAmountExceeded {});
    }

    Ok(swap_msgs)
}

fn make_burn_swap_msgs(
    querier: &QuerierWrapper,
    contract: &Addr,
    swap_info: BTreeMap<String, SwapInfo>,
    expected: BTreeMap<String, Uint128>,
    min_output_amount: Uint128,
) -> Result<Vec<CosmosMsg>, ContractError> {
    let mut swap_msgs: Vec<CosmosMsg> = Vec::new();
    let mut simulated_total_receive_amount = Uint128::zero();

    for (denom, expected) in expected {
        let swap_info = swap_info.get(&denom).unwrap();

        let simulated_token_out = swap_info.simulate_swap_exact_in(
            querier,
            contract.to_string(),
            denom.clone(),
            expected,
        )?;

        simulated_total_receive_amount += simulated_token_out;

        swap_msgs.push(swap_info.msg_swap_exact_in(
            contract.to_string(),
            simulated_token_out,
            denom,
            expected,
        ));
    }

    if min_output_amount > simulated_total_receive_amount {
        return Err(ContractError::TradeAmountExceeded {});
    }

    Ok(swap_msgs)
}

fn save_context(
    storage: &mut dyn Storage,
    executor: Addr,
    asset_to_check: String,
) -> StdResult<()> {
    let current_context_id = CURRENT_CONTEXT_ID.load(storage)?;

    CONTEXTS.save(
        storage,
        current_context_id,
        &Context {
            executor,
            asset_to_check,
        },
    )?;

    Ok(())
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
            output_amount,
            input_asset,
            swap_info,
        } => {
            // pre-transform swap_info
            let swap_info = swap_info.into_iter().collect::<BTreeMap<_, _>>();

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
                .map(|Coin { denom, amount }| (denom, amount * output.amount))
                .collect::<BTreeMap<_, _>>();

            let funds = desired
                .iter()
                .map(|(denom, want)| coin(want.u128(), denom))
                .collect();

            let swap_msgs = make_mint_swap_msgs(
                &deps.querier,
                &env.contract.address,
                core_config.reserve_denom,
                swap_info,
                desired,
                max_input.amount,
            )?;

            let mint_msg = core.call_with_funds(
                core::ExecuteMsg::Mint {
                    amount: output.amount,
                    receiver: info.sender.to_string(),
                },
                funds,
            )?;

            save_context(deps.storage, info.sender.clone(), max_input.denom.clone())?;

            let resp = Response::new()
                .add_messages(swap_msgs)
                .add_submessage(SubMsg::reply_on_success(mint_msg, REPLY_ID_MINT))
                .add_attributes(vec![
                    attr("method", "mint_exact_amount_out"),
                    attr("executor", info.sender),
                    attr("max_input", max_input.to_string()),
                    attr("output", output.to_string()),
                ]);

            Ok(resp)
        }

        BurnExactAmountIn {
            core_addr,
            output_asset,
            min_output_amount,
            swap_info,
        } => {
            // pre-transform swap_info
            let swap_info = swap_info.into_iter().collect::<BTreeMap<_, _>>();

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
                .map(|Coin { denom, amount }| (denom, amount * input.amount))
                .collect::<BTreeMap<_, _>>();

            let burn_msg = core.call_with_funds(
                core::ExecuteMsg::Burn {},
                coins(input.amount.u128(), core_config.reserve_denom),
            )?;

            let mut swap_msgs = make_burn_swap_msgs(
                &deps.querier,
                &env.contract.address,
                swap_info,
                expected,
                min_output.amount,
            )?
            .into_iter()
            .map(SubMsg::new)
            .collect::<Vec<SubMsg>>();

            // add reply setting to last message
            swap_msgs
                .last_mut()
                .map(|v| SubMsg::reply_on_success(v.msg.clone(), REPLY_ID_BURN));

            save_context(deps.storage, info.sender.clone(), min_output.denom.clone())?;

            let resp = Response::new()
                .add_message(burn_msg)
                .add_submessages(swap_msgs)
                .add_attributes(vec![
                    attr("method", "burn_exact_amount_in"),
                    attr("executor", info.sender),
                    attr("input_amount", input.to_string()),
                    attr("min_output_amount", min_output.to_string()),
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
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Default::default())
}
