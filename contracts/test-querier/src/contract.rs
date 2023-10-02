use cosmwasm_std::{
    entry_point, to_vec, ContractResult, Deps, DepsMut, Empty, Env, MessageInfo, QueryRequest,
    QueryResponse, Response, StdError, StdResult, SystemResult,
};

use crate::{CONTRACT_NAME, CONTRACT_VERSION};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryRequest<Empty>) -> StdResult<QueryResponse> {
    let req_bin = to_vec(&msg).map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {serialize_err}"))
    })?;

    match deps.querier.raw_query(&req_bin) {
        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
            "Querier system error: {system_err}"
        ))),
        SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!(
            "Querier contract error: {contract_err}"
        ))),
        SystemResult::Ok(ContractResult::Ok(value)) => Ok(value),
    }
}
