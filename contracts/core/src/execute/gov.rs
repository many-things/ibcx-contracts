use std::collections::BTreeMap;

use cosmwasm_std::{
    attr, coins, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use ibc_interface::core::{GovMsg, SwapRoute};
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin, osmosis::gamm::v1beta1::MsgSwapExactAmountIn,
};

use crate::{
    error::ContractError,
    state::{TradeStrategy, CONFIG, PAUSED, STATE, TRADE_STRATEGIES},
};

use super::check_and_simulate_trade;

pub fn handle_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: GovMsg,
) -> Result<Response, ContractError> {
    use GovMsg::*;

    // TODO: access control

    match msg {
        Pause { expires_at } => pause(deps, env, info, expires_at),
        Release {} => release(deps, env, info),

        Sweep {} => sweep(deps, env, info),

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

fn sweep(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let state = STATE.load(deps.storage)?;

    // calculate expected amount that contract should hold
    let reserve_unit = state.total_reserve.checked_div(state.total_supply)?;
    let mut assets = state.assets.clone();
    assets
        .entry(config.reserve_denom.clone())
        .and_modify(|v| *v += reserve_unit)
        .or_insert(reserve_unit);
    let expected: BTreeMap<_, _> = assets
        .into_iter()
        .map(|(denom, unit)| (denom, unit * state.total_supply))
        .collect();

    // calculate diff between actual and expected
    let diff = expected
        .into_iter()
        .map(|(denom, amount)| {
            StdResult::Ok((
                denom.clone(),
                deps.querier
                    .query_balance(&env.contract.address, denom)?
                    .amount
                    .checked_sub(amount)?,
            ))
        })
        .collect::<StdResult<BTreeMap<_, _>>>()?;

    // run simulation & accumulate them
    let conversion = diff
        .iter()
        .map(|(denom, token_in)| {
            let strategy = TRADE_STRATEGIES.load(deps.storage, denom)?;

            let token_out_amount = check_and_simulate_trade(
                &deps.querier,
                &env.contract.address,
                &token_in,
                strategy.route_sell(),
                &Uint128::zero(), // TODO: pass slippage setting
            )?;

            Ok(token_out_amount)
        })
        .collect::<Result<Vec<Uint128>, ContractError>>()?
        .iter()
        .fold(Uint128::zero(), |i, v| i + v);

    // build messages
    let trade_msg: Vec<CosmosMsg> = diff
        .into_iter()
        .map(|(denom, token_in)| {
            Ok(MsgSwapExactAmountIn {
                sender: env.contract.address.to_string(),
                routes: TRADE_STRATEGIES.load(deps.storage, &denom)?.route_sell(),
                token_in: Some(Coin {
                    denom,
                    amount: token_in.to_string(),
                }),
                token_out_min_amount: Uint128::zero().to_string(),
            }
            .into())
        })
        .collect::<StdResult<_>>()?;

    // ========= TODO: define actions after trade
    // ex)
    let accumulate_msg = BankMsg::Send {
        to_address: config.gov.to_string(),
        amount: coins(conversion.u128(), &config.reserve_denom),
    };

    let resp = Response::new()
        // trade first
        .add_messages(trade_msg)
        // send accumulated tokens after trade
        .add_message(accumulate_msg)
        .add_attributes(vec![
            attr("method", "gov::sweep"),
            attr("executor", info.sender),
            attr("conversion", conversion),
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
        attr("method", "gov::update_reserve_denom"),
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
        attr("method", "gov::update_trade_route"),
        attr("exector", info.sender),
        attr("asset", asset),
    ]);

    Ok(resp)
}
