use cosmwasm_std::{entry_point, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response};
use ibc_interface::core::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;

use crate::{
    error::ContractError,
    state::{Config, State, CONFIG, PAUSED, REBALANCE_LATEST_ID, STATE},
    CONTRACT_NAME, CONTRACT_VERSION,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(
        deps.storage,
        &Config {
            gov: deps.api.addr_validate(&msg.gov)?,
            denom: msg.denom.clone(),
            reserve_denom: msg.reserve_denom,
        },
    )?;

    PAUSED.save(deps.storage, &Default::default())?;

    STATE.save(deps.storage, &State::new(msg.initial_assets)?)?;

    REBALANCE_LATEST_ID.save(deps.storage, &0)?;

    let msg = MsgCreateDenom {
        sender: env.contract.address.into_string(),
        subdenom: msg.denom,
    };

    let resp = Response::new()
        .add_message(msg)
        .add_attribute("method", "instantiate");

    Ok(resp)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use crate::execute;
    use ExecuteMsg::*;

    match msg {
        Mint { amount, receiver } => execute::mint(deps, env, info, amount, receiver),
        Burn {} => execute::burn(deps, env, info),
        Gov(msg) => execute::gov::handle_msg(deps, env, info, msg),
        Rebalance(msg) => execute::rebalance::handle_msg(deps, env, info, msg),
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use crate::query;
    use QueryMsg::*;

    match msg {
        Config {} => query::config(deps, env),
        PauseInfo {} => query::pause_info(deps, env),
        Portfolio {} => query::portfolio(deps, env),
        RebalanceInfo { id } => query::rebalance_info(deps, env, id),
        ListRebalanceInfo {
            start_after,
            limit,
            order,
        } => query::list_rebalance_info(deps, env, start_after, limit, order),
        Strategy { asset } => query::strategy(deps, env, asset),
        ListStrategy {
            start_after,
            limit,
            order,
        } => query::list_strategy(deps, env, start_after, limit, order),
        Allocation { asset } => query::allocation(deps, env, asset),
        ListAllocation {
            start_after,
            limit,
            order,
        } => query::list_allocation(deps, env, start_after, limit, order),
    }
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Default::default())
}
