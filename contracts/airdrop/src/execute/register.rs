use crate::error::ContractError;
use crate::state::{save_label, Airdrop, AIRDROPS, LABELS, LATEST_AIRDROP_ID};
use cosmwasm_std::{attr, DepsMut, MessageInfo, Response, StdResult, Uint128};
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
        } => register_open(deps, info, merkle_root, denom, label),
        RegisterPayload::Bearer {
            merkle_root,
            denom,
            label,
            signer,
        } => register_bearer(deps, info, merkle_root, denom, label, signer),
    }
}

fn register_open(
    deps: DepsMut,
    info: MessageInfo,
    merkle_root: String,
    denom: String,
    label: Option<String>,
) -> Result<Response, ContractError> {
    // check merkle root length
    let mut root_buf: [u8; 32] = [0; 32];
    hex::decode_to_slice(&merkle_root, &mut root_buf)?;

    // fetch next airdrop id and increment it
    let airdrop_id = LATEST_AIRDROP_ID.load(deps.storage)?;

    // make label with tx sender
    let label = label.map(|x| format!("{}/{x}", sender));

    // make open airdrop
    let total_amount = cw_utils::must_pay(&info, &denom)?;

    let airdrop = Airdrop::Open {
        creator: info.sender,

        denom,
        total_amount,
        total_claimed: Uint128::zero(),
        merkle_root,

        label,
        closed: false,
    };

    // apply to state (LABELS, AIRDROP, LATEST_AIRDROP_ID)
    save_label(deps.storage, airdrop_id, airdrop.label())?;
    AIRDROPS.save(deps.storage, airdrop_id, &airdrop)?;
    LATEST_AIRDROP_ID.save(deps.storage, &(airdrop_id + 1))?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "register"),
        attr("executor", airdrop.creator()),
        attr("type", airdrop.type_str()),
        attr("merkle_root", airdrop.merkle_root()),
        attr("total_amount", airdrop.total_amount().to_string()),
        attr("label", airdrop.label().unwrap_or_default()),
    ]))
}

// bearer airdrop registerer
fn register_bearer(
    deps: DepsMut,
    info: MessageInfo,
    merkle_root: String,
    denom: String,
    label: Option<String>,
    signer: Option<String>,
) -> Result<Response, ContractError> {
    // use tx sender if signer is not provided
    let signer = signer
        .map(|x| deps.api.addr_validate(&x))
        .transpose()?
        .unwrap_or(info.sender.clone());

    // check merkle root length
    let mut root_buf: [u8; 32] = [0; 32];
    hex::decode_to_slice(&merkle_root, &mut root_buf)?;

    // fetch next airdrop id and increment it
    let airdrop_id = LATEST_AIRDROP_ID.load(deps.storage)?;

    // make label with tx sender
    let label = label.map(|x| format!("{}/{x}", sender));

    // make bearer airdrop
    let total_amount = cw_utils::must_pay(&info, &denom)?;

    let airdrop = Airdrop::Bearer {
        creator: info.sender,
        signer: signer.clone(),

        denom,
        total_amount,
        total_claimed: Uint128::zero(),
        merkle_root,

        label,
        closed: false,
    };

    // apply to state (LABELS, AIRDROP, LATEST_AIRDROP_ID)
    save_label(deps.storage, airdrop_id, airdrop.label())?;
    AIRDROPS.save(deps.storage, airdrop_id, &airdrop)?;
    LATEST_AIRDROP_ID.save(deps.storage, &(airdrop_id + 1))?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "register"),
        attr("executor", airdrop.creator()),
        attr("type", airdrop.type_str()),
        attr("signer", signer),
        attr("merkle_root", airdrop.merkle_root()),
        attr("total_amount", airdrop.total_amount().to_string()),
        attr("label", airdrop.label().unwrap_or_default()),
    ]))
}
