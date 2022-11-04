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
    Ok(Default::default())
}

#[entry_point]
pub fn migrate(deps: Deps, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Default::default())
}
