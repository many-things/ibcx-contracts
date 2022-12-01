use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Uint128};
use ibc_interface::core::GovMsg;
use ibc_interface::types::SwapRoute;

use crate::{
    error::ContractError,
    state::{GOV, PAUSED, TOKEN},
};

pub fn handle_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: GovMsg,
) -> Result<Response, ContractError> {
    use GovMsg::*;

    if info.sender != GOV.load(deps.storage)? {
        return Err(ContractError::Unauthorized {});
    }

    match msg {
        Pause { expires_at } => pause(deps, env, info, expires_at),
        Release {} => release(deps, env, info),

        UpdateReserveDenom { new_denom } => update_reserve_denom(deps, info, new_denom),
    }
}

fn pause(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    expires_at: u64,
) -> Result<Response, ContractError> {
    let mut pause_info = PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_paused()?;

    pause_info.paused = true;
    pause_info.expires_at = Some(expires_at);

    PAUSED.save(deps.storage, &pause_info)?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "gov::pause"),
        attr("executor", info.sender),
    ]);

    Ok(resp)
}

fn release(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    PAUSED.save(deps.storage, &Default::default())?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "gov::release"),
        attr("executor", info.sender),
    ]);

    Ok(resp)
}

fn update_reserve_denom(
    deps: DepsMut,
    info: MessageInfo,
    new_denom: String,
) -> Result<Response, ContractError> {
    let mut token = TOKEN.load(deps.storage)?;

    token.reserve_denom = new_denom;

    TOKEN.save(deps.storage, &token)?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "gov::update_reserve_denom"),
        attr("executor", info.sender),
    ]);

    Ok(resp)
}
