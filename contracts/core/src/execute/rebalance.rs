use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response, Uint128};
use ibc_interface::core::RebalanceMsg;

use crate::error::ContractError;

pub fn handle_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: RebalanceMsg,
) -> Result<Response, ContractError> {
    use RebalanceMsg::*;

    match msg {
        Init {
            manager,
            deflation,
            inflation,
        } => init(deps, env, info, manager, deflation, inflation),
        Trade { denom, amount } => trade(deps, env, info, denom, amount),
        Finalize {} => finalize(deps, env, info),
    }
}

fn init(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    manager: String,
    deflation: Vec<Coin>,
    inflation: Vec<Coin>,
) -> Result<Response, ContractError> {
    Ok(Default::default())
}

fn trade(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    Ok(Default::default())
}

fn finalize(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    Ok(Default::default())
}
