use crate::airdrop::{Airdrop, BearerAirdrop, OpenAirdrop};
use crate::error::ContractError;
use crate::state::{load_airdrop, AIRDROPS};
use cosmwasm_std::{attr, coins, BankMsg, DepsMut, Env, MessageInfo, Response};
use ibcx_interface::airdrop::AirdropId;

pub fn close(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: AirdropId,
) -> Result<Response, ContractError> {
    let (airdrop_id, airdrop) = load_airdrop(deps.storage, id)?;

    match airdrop {
        Airdrop::Open(inner) => close_open(deps, env, info, airdrop_id, inner),
        Airdrop::Bearer(inner) => close_bearer(deps, env, info, airdrop_id, inner),
    }
}

fn close_open(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    airdrop_id: u64,
    airdrop: OpenAirdrop,
) -> Result<Response, ContractError> {
    // validation
    if airdrop.creator != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    if airdrop.closed_at.is_some() {
        return Err(ContractError::AirdropClosed {});
    }

    let redeem_amount = airdrop.total_amount.checked_sub(airdrop.total_claimed)?;

    airdrop.closed_at = Some(env.block.height);

    // apply states
    AIRDROPS.save(deps.storage, airdrop_id, &airdrop.wrap())?;

    // response
    let send_msg = BankMsg::Send {
        to_address: airdrop.creator.to_string(),
        amount: coins(redeem_amount.u128(), airdrop.denom),
    };

    let attrs = vec![
        attr("method", "close"),
        attr("executor", info.sender),
        attr("airdrop_type", airdrop.wrap().type_str()),
        attr("airdrop_id", airdrop_id.to_string()),
        attr("redeemed", redeem_amount.to_string()),
    ];

    Ok(Response::new().add_message(send_msg).add_attributes(attrs))
}

fn close_bearer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    airdrop_id: u64,
    airdrop: BearerAirdrop,
) -> Result<Response, ContractError> {
    // validation
    if airdrop.creator != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    if airdrop.closed_at.is_some() {
        return Err(ContractError::AirdropClosed {});
    }

    let redeem_amount = airdrop.total_amount.checked_sub(airdrop.total_claimed)?;

    airdrop.closed_at = Some(env.block.height);

    // apply states
    AIRDROPS.save(deps.storage, airdrop_id, &airdrop.wrap())?;

    // response
    let send_msg = BankMsg::Send {
        to_address: airdrop.creator.to_string(),
        amount: coins(redeem_amount.u128(), airdrop.denom),
    };

    let attrs = vec![
        attr("method", "close"),
        attr("executor", info.sender),
        attr("airdrop_type", airdrop.wrap().type_str()),
        attr("airdrop_id", airdrop_id.to_string()),
        attr("redeemed", redeem_amount.to_string()),
    ];

    Ok(Response::new().add_message(send_msg).add_attributes(attrs))
}
