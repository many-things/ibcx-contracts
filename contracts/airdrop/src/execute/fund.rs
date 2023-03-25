use crate::airdrop::{Airdrop, BearerAirdrop, OpenAirdrop};
use crate::error::ContractError;
use crate::state::{airdrops, load_airdrop};
use cosmwasm_std::{attr, Addr, Attribute, DepsMut, MessageInfo, Response};
use ibcx_interface::airdrop::{AirdropId, AirdropType};

pub fn fund(deps: DepsMut, info: MessageInfo, id: AirdropId) -> Result<Response, ContractError> {
    let (airdrop_id, airdrop) = load_airdrop(deps.storage, id)?;

    match airdrop {
        Airdrop::Open(inner) => fund_open(deps, info, airdrop_id, inner),
        Airdrop::Bearer(inner) => fund_bearer(deps, info, airdrop_id, inner),
    }
}

fn fund_event(sender: Addr, typ: AirdropType, id: u64, add: impl Into<u128>) -> Vec<Attribute> {
    vec![
        attr("action", "fund"),
        attr("executor", sender),
        attr("airdrop_type", typ),
        attr("airdrop_id", id.to_string()),
        attr("amount", add.into().to_string()),
    ]
}

fn fund_open(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    mut airdrop: OpenAirdrop,
) -> Result<Response, ContractError> {
    if airdrop.creator != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    if airdrop.closed_at.is_some() {
        return Err(ContractError::AirdropClosed {});
    }

    let additional_funds = cw_utils::must_pay(&info, &airdrop.denom)?;
    airdrop.total_amount = airdrop.total_amount.checked_add(additional_funds)?;

    // event attributes
    let attrs = fund_event(info.sender, AirdropType::Open, id, additional_funds);

    // apply to state
    airdrops().save(deps.storage, id, &airdrop.into())?;

    Ok(Response::new().add_attributes(attrs))
}

fn fund_bearer(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    mut airdrop: BearerAirdrop,
) -> Result<Response, ContractError> {
    if airdrop.creator != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    if airdrop.closed_at.is_some() {
        return Err(ContractError::AirdropClosed {});
    }

    let additional_funds = cw_utils::must_pay(&info, &airdrop.denom)?;
    airdrop.total_amount = airdrop.total_amount.checked_add(additional_funds)?;

    // event attributes
    let attrs = fund_event(info.sender, AirdropType::Bearer, id, additional_funds);

    // apply to state
    airdrops().save(deps.storage, id, &airdrop.into())?;

    Ok(Response::new().add_attributes(attrs))
}
