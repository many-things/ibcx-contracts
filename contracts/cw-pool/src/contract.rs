use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response};
use ibcx_interface::cw_pool::{InstantiateMsg, MigrateMsg, QueryMsg, SudoMsg};

use crate::{StdResult, CONTRACT_NAME, CONTRACT_VERSION};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new())
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<QueryResponse> {
    Ok(Binary::default())
}

#[entry_point]
pub fn sudo(_deps: DepsMut, _env: Env, _msg: SudoMsg) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::new())
}
