use crate::error::ContractError;
use crate::state::{load_airdrop, AIRDROPS, LABELS};
use cosmwasm_std::{attr, DepsMut, MessageInfo, Response};
use ibcx_interface::airdrop::AirdropId;

pub fn fund(deps: DepsMut, info: MessageInfo, id: AirdropId) -> Result<Response, ContractError> {
    let (airdrop_id, airdrop) = load_airdrop(deps.storage, id)?;
    if airdrop.creator() != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    if airdrop.closed() {
        return Err(ContractError::AirdropClosed {});
    }

    let additional_funds = cw_utils::must_pay(&info, &airdrop.denom)?;
    airdrop.total_amount = airdrop.total_amount.checked_add(additional_funds)?;

    AIRDROPS.save(deps.storage, airdrop_id, &airdrop)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "fund"),
        attr("executor", info.sender),
        attr("airdrop_id", airdrop_id.to_string()),
        attr("amount", received),
    ]))
}
