mod config;
mod rebalance;

use cosmwasm_std::{attr, coin, BankMsg, Coin, DepsMut, Env, MessageInfo, Response, Uint128};
use ibc_interface::core::ExecuteMsg;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgMint;

use crate::{
    error::ContractError,
    state::{CONFIG, PAUSED},
};

pub fn handle_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        Mint { amount, receiver } => mint(deps, env, info, amount, receiver),
        Burn {} => burn(deps, env, info),
        Config(msg) => config::handle_msg(deps, env, info, msg),
        Rebalance(msg) => rebalance::handle_msg(deps, env, info, msg),
    }
}

fn mint(
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

    config.assert_funds(&info, &amount)?;

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

fn burn(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_paused()?;

    let config = CONFIG.load(deps.storage)?;

    let received = cw_utils::must_pay(&info, &config.denom)?;

    let payback: Vec<Coin> = config
        .assets
        .iter()
        .map(|(denom, unit)| coin((unit * received).u128(), denom.clone()))
        .collect();

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
