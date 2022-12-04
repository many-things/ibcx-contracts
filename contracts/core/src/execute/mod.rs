pub mod gov;

use cosmwasm_std::{attr, coin, BankMsg, DepsMut, Env, MessageInfo, Response, Uint128};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgBurn, MsgMint};

use crate::{
    error::ContractError,
    state::{assert_assets, get_redeem_amounts, PAUSED, TOKEN},
};

pub fn mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
    receiver: Option<String>,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_paused()?;

    // validate!
    let receiver = receiver
        .map(|v| deps.api.addr_validate(&v))
        .transpose()?
        .unwrap_or_else(|| info.sender.clone());

    let mut token = TOKEN.load(deps.storage)?;
    let refund = assert_assets(deps.storage, info.funds, amount)?;

    token.total_supply = token.total_supply.checked_add(amount)?;
    TOKEN.save(deps.storage, &token)?;

    let mint_amount = coin(amount.u128(), token.denom);
    let mut send_amount = refund;
    send_amount.push(mint_amount.clone());
    let send_amount = send_amount
        .into_iter()
        .filter(|v| !v.amount.is_zero())
        .collect();

    let resp = Response::new()
        .add_message(MsgMint {
            sender: env.contract.address.into_string(),
            amount: Some(mint_amount.into()),
        })
        .add_message(BankMsg::Send {
            to_address: receiver.to_string(),
            amount: send_amount,
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

    let mut token = TOKEN.load(deps.storage)?;
    let received = cw_utils::must_pay(&info, &token.denom)?;
    let payback = get_redeem_amounts(deps.storage, received)?;

    token.total_supply = token.total_supply.checked_sub(received)?;
    TOKEN.save(deps.storage, &token)?;

    let resp = Response::new()
        .add_message(MsgBurn {
            sender: env.contract.address.to_string(),
            amount: Some(coin(received.u128(), token.denom).into()),
        })
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
