use crate::airdrop::{Airdrop, BearerAirdrop, OpenAirdrop};
use crate::error::ContractError;
use crate::state::{load_airdrop, AIRDROPS};
use cosmwasm_std::{attr, DepsMut, MessageInfo, Response};
use ibcx_interface::airdrop::AirdropId;

pub fn fund(deps: DepsMut, info: MessageInfo, id: AirdropId) -> Result<Response, ContractError> {
    let (airdrop_id, airdrop) = load_airdrop(deps.storage, id)?;

    match airdrop {
        Airdrop::Open(inner) => fund_open(deps, info, airdrop_id, inner),
        Airdrop::Bearer(inner) => fund_bearer(deps, info, airdrop_id, inner),
    }
}

fn fund_open(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    airdrop: OpenAirdrop,
) -> Result<Response, ContractError> {
    if airdrop.creator != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    if airdrop.closed_at.is_some() {
        return Err(ContractError::AirdropClosed {});
    }

    let additional_funds = cw_utils::must_pay(&info, &airdrop.denom)?;
    airdrop.total_amount = airdrop.total_amount.checked_add(additional_funds)?;

    AIRDROPS.save(deps.storage, id, &airdrop.wrap())?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "fund"),
        attr("executor", info.sender),
        attr("airdrop_type", airdrop.wrap().type_str()),
        attr("airdrop_id", id.to_string()),
        attr("amount", additional_funds),
    ]))
}

fn fund_bearer(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    airdrop: BearerAirdrop,
) -> Result<Response, ContractError> {
    if airdrop.creator != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    if airdrop.closed_at.is_some() {
        return Err(ContractError::AirdropClosed {});
    }

    let additional_funds = cw_utils::must_pay(&info, &airdrop.denom)?;
    airdrop.total_amount = airdrop.total_amount.checked_add(additional_funds)?;

    AIRDROPS.save(deps.storage, id, &airdrop.wrap())?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "fund"),
        attr("executor", info.sender),
        attr("airdrop_type", airdrop.wrap().type_str()),
        attr("airdrop_id", id.to_string()),
        attr("amount", additional_funds),
    ]))
}
