use cosmwasm_std::{
    attr, coin, entry_point, to_binary, Env, MessageInfo, QueryResponse, Reply, Uint128,
};
use cosmwasm_std::{Deps, DepsMut, Response};
use ibcx_interface::{
    helpers::IbcCore,
    periphery::{
        ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, SimulateBurnExactAmountInResponse,
        SimulateMintExactAmountOutResponse,
    },
};

use crate::state::{Context, CONTEXT};
use crate::REPLY_ID_BURN_EXACT_AMOUNT_IN;
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
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        REPLY_ID_BURN_EXACT_AMOUNT_IN => {
            let context = CONTEXT.load(deps.storage)?;
            CONTEXT.remove(deps.storage);

            match context {
                Context::Burn {
                    sender,
                    min_output,
                    redeem_amounts,
                    swap_info,
                    ..
                } => {
                    let (swap_msgs, refunds) = make_burn_swap_msgs(
                        &deps.querier,
                        &env.contract.address,
                        &sender,
                        swap_info,
                        redeem_amounts,
                        &min_output,
                    )?;

                    let resp = Response::new().add_messages(swap_msgs).add_attributes(vec![
                        attr("method", "reply::burn_exact_amount_in"),
                        attr("executor", sender),
                        attr("refunds", refunds),
                    ]);

                    Ok(resp)
                }
                _ => Err(ContractError::InvalidContextType(context.kind())),
            }
        }
        _ => Err(ContractError::InvalidReplyId(reply.id)),
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
            // query to core contract
            let core = IbcCore(deps.api.addr_validate(&core_addr)?);
            let core_config = core.get_config(&deps.querier, None)?;

            // input & output
            let output = coin(output_amount.u128(), core_config.index_denom);

            let sim_resp = core.simulate_burn(&deps.querier, output.amount, None)?;
            let sim_amount_desired = sim_resp.redeem_amount;

            let (_, refund) = make_mint_swap_exact_out_msgs(
                &deps.querier,
                &env.contract.address,
                &env.contract.address,
                swap_info,
                sim_amount_desired.clone(),
                &coin(Uint128::MAX.u128(), &input_asset),
            )?;

            Ok(to_binary(&SimulateMintExactAmountOutResponse {
                mint_amount: output.amount,
                mint_spend_amount: sim_amount_desired,
                swap_result_amount: coin(Uint128::MAX.checked_sub(refund)?.u128(), input_asset),
            })?)
        }

        SimulateBurnExactAmountIn {
            core_addr,
            input_amount,
            output_asset,
            swap_info,
        } => {
            // query to core contract
            let core = IbcCore(deps.api.addr_validate(&core_addr)?);

            let sim_resp = core.simulate_burn(&deps.querier, input_amount, None)?;
            let expected = sim_resp.redeem_amount.clone();

            let (_, receive) = make_burn_swap_msgs(
                &deps.querier,
                &env.contract.address,
                &env.contract.address,
                swap_info,
                expected,
                &coin(Uint128::zero().u128(), &output_asset),
            )?;

            Ok(to_binary(&SimulateBurnExactAmountInResponse {
                burn_amount: sim_resp.burn_amount,
                burn_redeem_amount: sim_resp.redeem_amount,
                swap_result_amount: coin(receive.u128(), &output_asset),
            })?)
        }
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    if !msg.force.unwrap_or_default() {
        ibcx_utils::store_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    } else {
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    }

    Ok(Default::default())
}
