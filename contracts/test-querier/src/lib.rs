#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_vec, ContractResult, Deps, DepsMut, Empty, Env, MessageInfo, QueryRequest,
    QueryResponse, Response, StdError, StdResult, SystemResult,
};

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(not(feature = "library"))]
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

#[cfg(not(feature = "library"))]
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryRequest<Empty>) -> StdResult<QueryResponse> {
    let req_bin = to_json_vec(&msg).map_err(|serialize_err| {
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
