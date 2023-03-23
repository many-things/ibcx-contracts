use crate::error::ContractError;
use crate::state::{Airdrop, AIRDROPS, LABELS, LATEST_AIRDROP_ID};
use cosmwasm_std::{attr, DepsMut, MessageInfo, Response, Uint128};
use ibcx_interface::airdrop::RegisterPayload;

pub fn register(
    deps: DepsMut,
    info: MessageInfo,
    payload: RegisterPayload,
) -> Result<Response, ContractError> {
    match payload {
        RegisterPayload::Open {
            merkle_root,
            denom,
            label,
        } => register_open(deps, info, merkle_root, denom, label, None),
        RegisterPayload::Bearer {
            merkle_root,
            denom,
            label,
            signer,
        } => {
            panic!("not implemented")
        }
    }
}

fn register_open(
    deps: DepsMut,
    info: MessageInfo,
    merkle_root: String,
    denom: String,
    label: Option<String>,
    bearer: Option<bool>,
) -> Result<Response, ContractError> {
    let received = cw_utils::must_pay(&info, &denom)?;

    // check merkle root length
    let mut root_buf: [u8; 32] = [0; 32];
    hex::decode_to_slice(&merkle_root, &mut root_buf)?;

    let airdrop_id = LATEST_AIRDROP_ID.load(deps.storage)?;
    LATEST_AIRDROP_ID.save(deps.storage, &(airdrop_id + 1))?;

    let label = label.map(|v| format!("{}/{v}", info.sender));

    if let Some(label) = label.clone() {
        if LABELS.has(deps.storage, &label) {
            return Err(ContractError::KeyAlreadyExists {
                typ: "label".to_string(),
                key: label,
            });
        }

        LABELS.save(deps.storage, &label, &airdrop_id)?;
    }
    AIRDROPS.save(
        deps.storage,
        airdrop_id,
        &Airdrop {
            creator: info.sender.clone(),
            merkle_root: merkle_root.clone(),
            denom,
            total_amount: received,
            total_claimed: Uint128::zero(),
            bearer: bearer.unwrap_or(false),
            label: label.clone(),
            closed: false,
        },
    )?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "register"),
        attr("executor", info.sender),
        attr("merkle_root", merkle_root),
        attr("total_amount", received),
        attr("label", label.unwrap_or_default()),
        attr("bearer", bearer.unwrap_or(false).to_string()),
    ]))
}
