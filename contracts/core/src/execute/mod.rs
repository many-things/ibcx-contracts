pub mod gov;
pub mod rebalance;

use std::{collections::BTreeMap, str::FromStr};

use cosmwasm_std::{
    attr, coin, Addr, BankMsg, DepsMut, Env, MessageInfo, QuerierWrapper, Response, Storage,
    Uint128,
};
use osmosis_std::types::osmosis::{
    gamm::v1beta1::{
        QuerySwapExactAmountInRequest, QuerySwapExactAmountInResponse, SwapAmountInRoute,
    },
    tokenfactory::v1beta1::MsgMint,
};

use crate::{
    error::ContractError,
    state::{
        RebalanceInfo, TradeStrategy, CONFIG, PAUSED, REBALANCES, REBALANCE_LATEST_ID, STATE,
        TRADE_STRATEGIES,
    },
};

pub fn mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
    receiver: String,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_paused()?;

    // validate!
    deps.api.addr_validate(&receiver)?;

    let config = CONFIG.load(deps.storage)?;
    let mut state = STATE.load(deps.storage)?;

    let funds: BTreeMap<_, _> = info
        .funds
        .into_iter()
        .map(|v| (v.denom, v.amount))
        .collect();

    state.assert_funds(funds, &config.reserve_denom, &amount)?;
    state.total_supply = state.total_supply.checked_add(amount)?;

    STATE.save(deps.storage, &state)?;

    let resp = Response::new()
        .add_message(MsgMint {
            sender: receiver.clone(),
            amount: Some(coin(amount.u128(), config.denom).into()),
        })
        .add_attributes(vec![
            attr("method", "mint"),
            attr("executor", info.sender),
            attr("receiver", receiver),
            attr("amount", amount),
        ]);

    Ok(resp)
}

pub fn burn(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_paused()?;

    let config = CONFIG.load(deps.storage)?;
    let received = cw_utils::must_pay(&info, &config.denom)?;

    let mut state = STATE.load(deps.storage)?;

    // calculate redeem amount
    let payback = state.calc_redeem_amount(&config.reserve_denom, received)?;
    state.total_supply = state.total_supply.checked_sub(received)?;

    STATE.save(deps.storage, &state)?;

    let resp = Response::new()
        .add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: payback,
        })
        .add_attributes(vec![
            attr("method", "burn"),
            attr("executor", info.sender),
            attr("amount", received),
        ]);

    Ok(resp)
}

fn check_and_get_strategy(
    storage: &dyn Storage,
    now: u64,
    asset: &str,
    trade_amount: &Uint128,
) -> Result<TradeStrategy, ContractError> {
    match TRADE_STRATEGIES.may_load(storage, asset)? {
        Some(strategy) => {
            if &strategy.max_trade_amount < trade_amount {
                return Err(ContractError::TradeAmountExceeded {});
            }
            if strategy.last_traded_at + strategy.cool_down.unwrap_or_default() > now {
                return Err(ContractError::TradeCooldownNotFinished {});
            }

            Ok(strategy)
        }
        None => Err(ContractError::TradeStrategyNotSet {}),
    }
}

fn check_and_get_rebalance_info(storage: &dyn Storage) -> Result<RebalanceInfo, ContractError> {
    let rebalance_id = REBALANCE_LATEST_ID.load(storage)?;
    let rebalance = REBALANCES.load(storage, rebalance_id)?;
    if rebalance.finished {
        return Err(ContractError::RebalanceAlreadyFinished {});
    }

    Ok(rebalance)
}

fn check_and_simulate_trade(
    querier: &QuerierWrapper,
    contract: &Addr,
    token_in: &Uint128,
    routes: Vec<SwapAmountInRoute>,
    out_min: &Uint128,
) -> Result<Uint128, ContractError> {
    let resp: QuerySwapExactAmountInResponse = querier.query(
        &QuerySwapExactAmountInRequest {
            sender: contract.to_string(),
            pool_id: 0, // not used
            token_in: token_in.to_string(),
            routes,
        }
        .into(),
    )?;

    let token_out_amount = Uint128::from_str(&resp.token_out_amount)?;
    if out_min > &token_out_amount {
        return Err(ContractError::TradeSimulationFailed {});
    }

    Ok(token_out_amount)
}
