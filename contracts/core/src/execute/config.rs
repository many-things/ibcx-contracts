use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Uint128};
use ibc_interface::core::{ConfigMsg, SwapRoute};

use crate::{
    error::ContractError,
    state::{TradeStrategy, CONFIG, PAUSED, TRADE_STRATEGIES},
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
        UpdateTradeStrategy {
            asset,
            routes,
            cool_down,
            max_trade_amount,
        } => update_trade_strategy(deps, env, info, asset, routes, cool_down, max_trade_amount),
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

fn update_trade_strategy(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    asset: String,
    routes: Vec<SwapRoute>,
    cool_down: Option<u64>,
    max_trade_amount: Uint128,
) -> Result<Response, ContractError> {
    let strategy = TradeStrategy {
        routes,
        cool_down,
        max_trade_amount,
        last_traded_at: TRADE_STRATEGIES
            .may_load(deps.storage, &asset)?
            .map(|v| v.last_traded_at)
            .unwrap_or_default(),
    };
    strategy.validate(&CONFIG.load(deps.storage)?.reserve_denom)?;

    TRADE_STRATEGIES.save(deps.storage, &asset, &strategy)?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "update_trade_route"),
        attr("exector", info.sender),
        attr("asset", asset),
    ]);

    Ok(resp)
}
