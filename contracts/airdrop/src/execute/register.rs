use crate::airdrop::{BearerAirdrop, OpenAirdrop};
use crate::error::ContractError;
use crate::state::{airdrops, save_label, LATEST_AIRDROP_ID};
use crate::verify::{pub_to_addr, sha256_digest};
use cosmwasm_std::{attr, Attribute, Binary, DepsMut, Env, MessageInfo, Response, Uint128};
use ibcx_interface::airdrop::{AirdropType, RegisterPayload};

pub fn register(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    payload: RegisterPayload,
) -> Result<Response, ContractError> {
    match payload {
        RegisterPayload::Open {
            merkle_root,
            denom,
            label,
        } => register_open(deps, env, info, merkle_root, denom, label),
        RegisterPayload::Bearer {
            merkle_root,
            denom,
            label,
            signer_pub,
            signer_sig,
        } => register_bearer(
            deps,
            env,
            info,
            merkle_root,
            denom,
            label,
            signer_pub,
            signer_sig,
        ),
    }
}

fn register_open_event(id: u64, airdrop: OpenAirdrop) -> Vec<Attribute> {
    vec![
        attr("action", "register"),
        attr("executor", &airdrop.creator),
        attr("id", id.to_string()),
        attr("type", AirdropType::Open),
        attr("merkle_root", airdrop.merkle_root),
        attr("total_amount", airdrop.total_amount.to_string()),
        attr("label", airdrop.label.unwrap_or_default()),
    ]
}

fn register_open(
    deps: DepsMut,
    env: Env,
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
    let label = label.map(|x| format!("{}/{x}", info.sender));

    // make open airdrop
    let total_amount = cw_utils::must_pay(&info, &denom)?;

    let airdrop = OpenAirdrop {
        creator: info.sender,

        denom,
        total_amount,
        total_claimed: Uint128::zero(),
        merkle_root,

        label,
        created_at: env.block.height,
        closed_at: None,
    };

    // event attributes
    let attrs = register_open_event(airdrop_id, airdrop.clone());

    // apply to state (LABELS, AIRDROP, LATEST_AIRDROP_ID)
    save_label(deps.storage, airdrop_id, &airdrop.label)?;
    airdrops().save(deps.storage, airdrop_id, &airdrop.into())?;
    LATEST_AIRDROP_ID.save(deps.storage, &(airdrop_id + 1))?;

    Ok(Response::new().add_attributes(attrs))
}

fn register_bearer_event(id: u64, airdrop: BearerAirdrop) -> Vec<Attribute> {
    vec![
        attr("action", "register"),
        attr("executor", &airdrop.creator),
        attr("id", id.to_string()),
        attr("type", AirdropType::Bearer),
        attr("signer", airdrop.signer),
        attr("merkle_root", airdrop.merkle_root),
        attr("total_amount", airdrop.total_amount.to_string()),
        attr("label", airdrop.label.unwrap_or_default()),
    ]
}

#[allow(clippy::too_many_arguments)]
// bearer airdrop registerer
fn register_bearer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    merkle_root: String,
    denom: String,
    label: Option<String>,
    signer_pub: String,
    signer_sig: String,
) -> Result<Response, ContractError> {
    // check merkle root length
    let mut root_buf: [u8; 32] = [0; 32];
    hex::decode_to_slice(&merkle_root, &mut root_buf)?;

    // fetch next airdrop id and increment it
    let airdrop_id = LATEST_AIRDROP_ID.load(deps.storage)?;

    // make label with tx sender
    let label = label.map(|x| format!("{}/{x}", info.sender));

    // verify signer_sig
    let signer_pub = hex::decode(signer_pub)?;
    let signer_sig = hex::decode(signer_sig)?;
    // TODO: do we have to make prefix injectable?
    let signer = pub_to_addr(signer_pub.clone().into(), "osmo")?;

    let digest = sha256_digest(signer.as_bytes())?;

    let verified =
        deps.api
            .secp256k1_verify(&digest, signer_sig.as_slice(), signer_pub.as_slice())?;
    if !verified {
        return Err(ContractError::invalid_signature("register"));
    }

    // make bearer airdrop
    let total_amount = cw_utils::must_pay(&info, &denom)?;

    let airdrop = BearerAirdrop {
        creator: info.sender,
        signer: deps.api.addr_validate(&signer)?,
        signer_pub: Binary::from(signer_pub),

        denom,
        total_amount,
        total_claimed: Uint128::zero(),
        merkle_root,

        label,
        created_at: env.block.height,
        closed_at: None,
    };

    // event attributes
    let attrs = register_bearer_event(airdrop_id, airdrop.clone());

    // apply to state (LABELS, AIRDROP, LATEST_AIRDROP_ID)
    save_label(deps.storage, airdrop_id, &airdrop.label)?;
    airdrops().save(deps.storage, airdrop_id, &airdrop.wrap())?;
    LATEST_AIRDROP_ID.save(deps.storage, &(airdrop_id + 1))?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "register"),
        attr("executor", &airdrop.creator),
        attr("type", airdrop.wrap().type_str()),
        attr("signer", airdrop.signer),
        attr("merkle_root", airdrop.merkle_root),
        attr("total_amount", airdrop.total_amount.to_string()),
        attr("label", airdrop.label.unwrap_or_default()),
    ]))
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{coin, testing::mock_dependencies_with_balances, Coin};

    const DENOM_BASE: u128 = 10 ^ 6;

    fn normalize_amount(amount: f32) -> u128 {
        (amount * DENOM_BASE as f32) as u128
    }

    #[test]
    fn test_register_open() {
        let tester_balance: &[Coin] = &[coin(normalize_amount(10.5), "ukrw")];
        let initial_balances = &[("tester", tester_balance)];

        let deps = mock_dependencies_with_balances(initial_balances);

        let balances = deps.as_ref().querier.query_all_balances("tester").unwrap();
        println!("{:?}", balances);
    }

    #[test]
    fn test_register_bearer() {}
}
