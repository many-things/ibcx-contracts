use std::collections::BTreeMap;

use cosmwasm_std::{attr, coin, entry_point, to_binary, Coin, Env, MessageInfo, QueryResponse};
use ibc_alias::{Deps, DepsMut, Response};
use ibc_interface::{
    helpers::IbcCore,
    periphery::{
        ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, SimulateBurnExactAmountInResponse,
        SimulateMintExactAmountOutResponse,
    },
};

use crate::{
    error::ContractError,
    execute,
    msgs::{make_burn_swap_msgs, make_mint_swap_exact_out_msgs},
    CONTRACT_NAME, CONTRACT_VERSION,
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
        } => execute::mint_exact_amount_out(
            deps,
            env,
            info,
            core_addr,
            output_amount,
            input_asset,
            swap_info,
        ),
        BurnExactAmountIn {
            core_addr,
            output_asset,
            min_output_amount,
            swap_info,
        } => execute::burn_exact_amount_in(
            deps,
            env,
            info,
            core_addr,
            output_asset,
            min_output_amount,
            swap_info,
        ),
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

            let (_, refund) = make_mint_swap_exact_out_msgs(
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
