pub mod config;
pub mod rebalance;

use std::collections::BTreeMap;

use cosmwasm_std::{attr, coin, BankMsg, DepsMut, Env, MessageInfo, Response, Uint128};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgMint;

use crate::{
    error::ContractError,
    state::{CONFIG, PAUSED, STATE},
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
            sender: receiver,
            amount: Some(coin(amount.u128(), config.denom).into()),
        })
        .add_attributes(vec![
            attr("method", "mint"),
            attr("minter", info.sender),
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
            attr("burner", info.sender),
            attr("amount", received),
        ]);

    Ok(resp)
}
