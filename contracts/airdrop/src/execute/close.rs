use crate::error::ContractError;
use crate::state::{AIRDROPS, LABELS};
use cosmwasm_std::{attr, coins, BankMsg, DepsMut, MessageInfo, Response};
use ibcx_interface::airdrop::AirdropId;

pub fn close(deps: DepsMut, info: MessageInfo, id: AirdropId) -> Result<Response, ContractError> {
    let airdrop_id = match id {
        AirdropId::Id(id) => id,
        AirdropId::Label(label) => LABELS.load(deps.storage, &label)?,
    };

    let mut airdrop = AIRDROPS.load(deps.storage, airdrop_id)?;
    if airdrop.creator != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    if airdrop.closed {
        return Err(ContractError::AirdropClosed {});
    }

    let redeem_amount = airdrop.total_amount.checked_sub(airdrop.total_claimed)?;

    airdrop.closed = true;

    AIRDROPS.save(deps.storage, airdrop_id, &airdrop)?;

    Ok(Response::new()
        .add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: coins(redeem_amount.u128(), airdrop.denom),
        })
        .add_attributes(vec![
            attr("method", "close"),
            attr("executor", info.sender),
            attr("airdrop_id", airdrop_id.to_string()),
            attr("redeemed", redeem_amount.to_string()),
        ]))
}
