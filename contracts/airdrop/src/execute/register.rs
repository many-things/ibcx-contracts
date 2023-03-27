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
    airdrops().save(deps.storage, airdrop_id, &airdrop.into())?;
    LATEST_AIRDROP_ID.save(deps.storage, &(airdrop_id + 1))?;

    Ok(Response::new().add_attributes(attrs))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coin,
        testing::{
            mock_dependencies_with_balances, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
        },
        OwnedDeps,
    };
    use ibcx_interface::airdrop::{
        AirdropId, GetLabelResponse, InstantiateMsg, ListAirdropsQueryOptions, RegisterPayload,
    };

    use crate::{
        airdrop::Airdrop,
        contract::instantiate,
        execute::{
            register::{register_bearer_event, register_open_event},
            tests::{mock_bearer_airdrop, mock_open_airdrop, Balances},
        },
        query,
    };

    use super::register;

    fn assert_state(
        deps: OwnedDeps<MockStorage, MockApi, MockQuerier>,
        registerer: String,
        label: Option<&str>,
        expected: (u64, Airdrop),
    ) {
        // Check LABELS
        if let Some(label) = label {
            let get_label =
                query::get_label(deps.as_ref(), format!("{registerer}/{label}")).unwrap();
            assert_eq!(
                get_label,
                GetLabelResponse {
                    creator: registerer.clone(),
                    label: label.to_string(),
                    airdrop_id: 0
                }
            );
        }

        // Check AIRDROPS
        let get_airdrop = query::get_airdrop(deps.as_ref(), AirdropId::id(0)).unwrap();
        assert_eq!(get_airdrop, Airdrop::to_resp(expected.clone()));

        let list_airdrops_by_type = query::list_airdrops(
            deps.as_ref(),
            ListAirdropsQueryOptions::by_type(expected.1.type_of(), None, None, None),
        )
        .unwrap();
        assert_eq!(
            list_airdrops_by_type.0,
            vec![Airdrop::to_resp(expected.clone())]
        );

        let list_airdrops_by_creator = query::list_airdrops(
            deps.as_ref(),
            ListAirdropsQueryOptions::by_creator(&registerer, None, None, None),
        )
        .unwrap();
        assert_eq!(list_airdrops_by_creator.0, vec![Airdrop::to_resp(expected)]);
    }

    #[test]
    fn test_register_open() {
        let env = mock_env();

        let label = Some("test_open_airdrop");
        let mock_airdrop = mock_open_airdrop(label, env.block.height);

        let initial_balances: Balances = &[(
            mock_airdrop.creator.as_str(),
            &[coin(mock_airdrop.total_amount.u128(), &mock_airdrop.denom)],
        )];

        let mut deps = mock_dependencies_with_balances(initial_balances);

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("creator", &[]),
            InstantiateMsg {},
        )
        .unwrap();

        let info = mock_info(
            mock_airdrop.creator.as_str(),
            &[coin(mock_airdrop.total_amount.u128(), &mock_airdrop.denom)],
        );
        let payload = RegisterPayload::open(&mock_airdrop.merkle_root, &mock_airdrop.denom, label);

        let expected_registerer = mock_airdrop.creator.to_string();
        let expected: (u64, Airdrop) = (0u64, mock_airdrop.into());

        let resp = register(deps.as_mut(), env, info, payload).unwrap();
        assert_eq!(
            resp.attributes,
            register_open_event(expected.0, expected.1.clone().unwrap_open().unwrap())
        );
        assert_state(deps, expected_registerer, label, expected);
    }

    #[test]
    fn test_register_bearer() {
        let env = mock_env();

        let label = Some("test_bearer_airdrop");
        let (mock_airdrop, signer_sig) = mock_bearer_airdrop(label, env.block.height);

        let initial_balances: Balances = &[(
            mock_airdrop.creator.as_str(),
            &[coin(mock_airdrop.total_amount.u128(), &mock_airdrop.denom)],
        )];

        let mut deps = mock_dependencies_with_balances(initial_balances);

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("creator", &[]),
            InstantiateMsg {},
        )
        .unwrap();

        let info = mock_info(
            mock_airdrop.creator.as_str(),
            &[coin(mock_airdrop.total_amount.u128(), &mock_airdrop.denom)],
        );
        let payload = RegisterPayload::bearer(
            &mock_airdrop.merkle_root,
            &mock_airdrop.denom,
            hex::encode(&mock_airdrop.signer_pub),
            signer_sig,
            label,
        );

        let expected_registerer = mock_airdrop.creator.to_string();
        let expected: (u64, Airdrop) = (0u64, mock_airdrop.into());

        let resp = register(deps.as_mut(), env, info, payload).unwrap();
        assert_eq!(
            resp.attributes,
            register_bearer_event(expected.0, expected.1.clone().unwrap_bearer().unwrap())
        );
        assert_state(deps, expected_registerer, label, expected);
    }
}
