use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};
use ibc_interface::core::{ConfigMsg, SwapRoute};

use crate::{
    error::ContractError,
    state::{CONFIG, PAUSED, TRADE_ROUTE},
};

pub fn handle_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ConfigMsg,
) -> Result<Response, ContractError> {
    use ConfigMsg::*;

    match msg {
        Pause { expires_at } => pause(deps, env, info, expires_at),
        Release {} => release(deps, env, info),
        UpdateReserveDenom { new_denom } => update_reserve_denom(deps, info, new_denom),
        UpdateTradeRoute { asset, routes } => update_trade_route(deps, info, asset, routes),
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

    let resp = Response::new()
        .add_attributes(vec![attr("method", "pause"), attr("executor", info.sender)]);

    Ok(resp)
}

fn release(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    PAUSED.save(deps.storage, &Default::default())?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "release"),
        attr("executor", info.sender),
    ]);

    Ok(resp)
}

fn update_reserve_denom(
    deps: DepsMut,
    info: MessageInfo,
    new_denom: String,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    config.reserve_denom = new_denom;

    CONFIG.save(deps.storage, &config)?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "update_reserve_denom"),
        attr("executor", info.sender),
    ]);

    Ok(resp)
}

fn update_trade_route(
    deps: DepsMut,
    info: MessageInfo,
    asset: String,
    routes: Vec<SwapRoute>,
) -> Result<Response, ContractError> {
    TRADE_ROUTE.save(deps.storage, &asset, &routes)?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "update_trade_route"),
        attr("exector", info.sender),
        attr("asset", asset),
    ]);

    Ok(resp)
}
