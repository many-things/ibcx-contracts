use std::collections::BTreeMap;

use cosmwasm_std::{
    attr, coin, coins, entry_point, to_binary, Addr, BankMsg, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, QuerierWrapper, QueryResponse, Response, Uint128,
};
use ibc_interface::{
    core,
    helpers::IbcCore,
    periphery::{
        ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, SimulateBurnExactAmountInResponse,
        SimulateMintExactAmountOutResponse,
    },
    types::SwapRoutes,
};

use crate::{error::ContractError, CONTRACT_NAME, CONTRACT_VERSION};

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
    sender: &Addr,
    reserve_denom: String,
    swap_info: BTreeMap<String, SwapRoutes>,
    desired: BTreeMap<String, Uint128>,
    max_input: &Coin,
) -> Result<(Vec<CosmosMsg>, Uint128), ContractError> {
    let mut swap_msgs: Vec<CosmosMsg> = Vec::new();
    let mut simulated_total_spend_amount = Uint128::zero();

    for (denom, want) in desired {
        if denom == reserve_denom {
            simulated_total_spend_amount += want
        }

        let swap_info = swap_info.get(&denom).unwrap();

        let simulated_token_in = swap_info.sim_swap_exact_out(querier, contract, &denom, want)?;

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

fn make_burn_swap_msgs(
    querier: &QuerierWrapper,
    contract: &Addr,
    sender: &Addr,
    swap_info: BTreeMap<String, SwapRoutes>,
    expected: BTreeMap<String, Uint128>,
    min_output: &Coin,
) -> Result<(Vec<CosmosMsg>, Uint128), ContractError> {
    let mut swap_msgs: Vec<CosmosMsg> = Vec::new();
    let mut simulated_total_receive_amount = Uint128::zero();

    for (denom, expected) in expected {
        let swap_info = swap_info.get(&denom).unwrap();

        let simulated_token_out =
            swap_info.sim_swap_exact_in(querier, contract, &denom, expected)?;

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

            let (swap_msgs, _) = make_mint_swap_msgs(
                &deps.querier,
                &env.contract.address,
                &info.sender,
                core_config.reserve_denom,
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

            let (swap_msgs, _) = make_burn_swap_msgs(
                &deps.querier,
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
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use QueryMsg::*;

    match msg {
        SimulateMintExactAmountOut {
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

            let (_, refund) = make_mint_swap_msgs(
                &deps.querier,
                &env.contract.address,
                &env.contract.address,
                core_config.reserve_denom,
                swap_info,
                desired,
                &input_asset,
            )?;

            let sim_resp = core.simulate_mint(&deps.querier, output_amount, funds)?;

            Ok(to_binary(&SimulateMintExactAmountOutResponse {
                mint_amount: sim_resp.mint_amount,
                mint_refund_amount: sim_resp.refund_amount,
                swap_refund_amount: coin(refund.u128(), &input_asset.denom),
            })?)
        }
        SimulateBurnExactAmountIn {
            core_addr,
            input_amount,
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
            let input = coin(input_amount.u128(), &core_config.denom);
            let min_output = coin(min_output_amount.u128(), &output_asset);

            let expected = core_portfolio
                .assets
                .into_iter()
                .map(|Coin { denom, amount }| (denom, amount * input.amount))
                .collect::<BTreeMap<_, _>>();

            let (_, receive) = make_burn_swap_msgs(
                &deps.querier,
                &env.contract.address,
                &env.contract.address,
                swap_info,
                expected,
                &min_output,
            )?;

            let sim_resp = core.simulate_burn(&deps.querier, input_amount)?;

            Ok(to_binary(&SimulateBurnExactAmountInResponse {
                burn_amount: sim_resp.burn_amount,
                burn_redeem_amount: sim_resp.redeem_amount,
                swap_result_amount: coin(receive.u128(), &output_asset),
            })?)
        }
    }
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Default::default())
}
