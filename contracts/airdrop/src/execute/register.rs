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
        Addr, Binary, Coin, OwnedDeps, Uint128,
    };
    use ibcx_interface::airdrop::{
        AirdropId, GetLabelResponse, InstantiateMsg, ListAirdropsQueryOptions, RegisterPayload,
    };

    use crate::{
        airdrop::{Airdrop, BearerAirdrop, OpenAirdrop},
        contract::instantiate,
        execute::register::{register_bearer_event, register_open_event},
        query,
    };

    use super::register;

    type Balances<'a> = &'a [(&'a str, &'a [Coin])];

    const DENOM_BASE: u128 = 10e6 as u128;

    fn normalize_amount(amount: f32) -> u128 {
        (amount * DENOM_BASE as f32) as u128
    }

    fn assert_state(
        deps: OwnedDeps<MockStorage, MockApi, MockQuerier>,
        registerer: String,
        label: String,
        expected: (u64, Airdrop),
    ) {
        // Check LABELS
        let get_label = query::get_label(deps.as_ref(), format!("{registerer}/{label}")).unwrap();
        assert_eq!(
            get_label,
            GetLabelResponse {
                creator: registerer.clone(),
                label,
                airdrop_id: 0
            }
        );

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
        // setup test
        let registerer = "tester".to_string();
        let denom = "ukrw".to_string();
        let amount = normalize_amount(5.5);
        let merkle_root = "deadbeef".repeat(8);
        let label = "test_airdrop".to_string();

        let initial_balances: Balances = &[(registerer.as_str(), &[coin(amount, &denom)])];

        let mut deps = mock_dependencies_with_balances(initial_balances);

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("creator", &[]),
            InstantiateMsg {},
        )
        .unwrap();

        let env = mock_env();
        let info = mock_info(&registerer, &[coin(amount, &denom)]);
        let payload = RegisterPayload::open(&merkle_root, &denom, Some(&label));

        let expected: (u64, Airdrop) = (
            0u64,
            OpenAirdrop {
                creator: Addr::unchecked(&registerer),

                denom,
                total_amount: Uint128::from(amount),
                total_claimed: Uint128::zero(),
                merkle_root,

                label: Some(format!("{registerer}/{label}")),

                created_at: env.block.height,
                closed_at: None,
            }
            .into(),
        );

        let resp = register(deps.as_mut(), env, info, payload).unwrap();
        assert_eq!(
            resp.attributes,
            register_open_event(expected.0, expected.1.clone().unwrap_open().unwrap())
        );
        assert_state(deps, registerer, label, expected);
    }

    #[test]
    fn test_register_bearer() {
        // setup test
        let registerer = "tester".to_string();
        let denom = "ukrw".to_string();
        let amount = normalize_amount(5.5);
        let merkle_root = "deadbeef".repeat(8);

        // notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius
        let signer_addr = "osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks";
        let signer_pub = "02ec18c82501c5088119251679b538e9cf8eae502956cc862c7778aa148365e886";
        let signer_sig = "c8cccdaa7568544164b2bcbea55eaaaa7f52e63ff2e9f075d7419a4558f2ec5574f196d449304314f22a0803f4fc260c476a1380b6db72b7cb6976980b9a1a46";

        let label = "test_airdrop".to_string();

        let initial_balances: Balances = &[(registerer.as_str(), &[coin(amount, &denom)])];

        let mut deps = mock_dependencies_with_balances(initial_balances);

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("creator", &[]),
            InstantiateMsg {},
        )
        .unwrap();

        let env = mock_env();
        let info = mock_info(&registerer, &[coin(amount, &denom)]);
        let payload =
            RegisterPayload::bearer(&merkle_root, &denom, signer_pub, signer_sig, Some(&label));

        let expected: (u64, Airdrop) = (
            0u64,
            BearerAirdrop {
                creator: Addr::unchecked(&registerer),
                signer: deps.as_ref().api.addr_validate(signer_addr).unwrap(),
                signer_pub: Binary::from(hex::decode(signer_pub).unwrap()),

                denom,
                total_amount: Uint128::from(amount),
                total_claimed: Uint128::zero(),
                merkle_root,

                label: Some(format!("{registerer}/{label}")),

                created_at: env.block.height,
                closed_at: None,
            }
            .into(),
        );

        let resp = register(deps.as_mut(), env, info, payload).unwrap();
        assert_eq!(
            resp.attributes,
            register_bearer_event(expected.0, expected.1.clone().unwrap_bearer().unwrap())
        );
        assert_state(deps, registerer, label, expected);
    }
}
