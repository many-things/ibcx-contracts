use cosmwasm_schema::serde::Serialize;
use cosmwasm_std::{attr, entry_point, Env, MessageInfo, QueryResponse};
use cosmwasm_std::{Deps, DepsMut, Response};
use ibcx_interface::periphery::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::query;
use crate::{error::ContractError, execute, CONTRACT_NAME, CONTRACT_VERSION};

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
        MintExactAmountIn {
            core_addr,
            input_asset,
            min_output_amount,
            swap_info,
        } => execute::mint_exact_amount_in(
            deps,
            env,
            info,
            core_addr,
            input_asset,
            min_output_amount,
            swap_info.into(),
        ),
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
            swap_info.into(),
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
            swap_info.into(),
        ),
        BurnExactAmountOut {
            core_addr,
            output_asset,
            swap_info,
        } => execute::burn_exact_amount_out(
            deps,
            env,
            info,
            core_addr,
            output_asset,
            swap_info.into(),
        ),
        FinishOperation {
            refund_to,
            refund_asset,
        } => execute::finish_operation(deps, env, info, refund_to, refund_asset),
    }
}

pub fn to_binary<T: Serialize>(
    r: Result<T, ContractError>,
) -> Result<QueryResponse, ContractError> {
    Ok(r.map(|v| cosmwasm_std::to_json_binary(&v))??)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use QueryMsg::*;

    match msg {
        SimulateMintExactAmountIn {
            core_addr,
            input_asset,
            swap_info,
        } => to_binary(query::simulate_mint_exact_amount_in(
            deps,
            env,
            core_addr,
            input_asset,
            swap_info.into(),
        )),
        SimulateMintExactAmountOut {
            core_addr,
            output_amount,
            input_asset,
            swap_info,
        } => to_binary(query::simulate_mint_exact_amount_out(
            deps,
            env,
            core_addr,
            output_amount,
            input_asset,
            swap_info.into(),
        )),

        SimulateBurnExactAmountIn {
            core_addr,
            input_amount,
            output_asset,
            swap_info,
        } => to_binary(query::simulate_burn_exact_amount_in(
            deps,
            env,
            core_addr,
            input_amount,
            output_asset,
            swap_info.into(),
        )),
        SimulateBurnExactAmountOut {
            core_addr,
            swap_info,
            output_asset,
        } => to_binary(query::simulate_burn_exact_amount_out(
            deps,
            env,
            core_addr,
            output_asset,
            swap_info.into(),
        )),
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
