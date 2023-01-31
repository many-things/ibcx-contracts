use cosmwasm_std::{
    attr, coins, Addr, Api, BankMsg, CosmosMsg, DepsMut, MessageInfo, Response, Storage, Uint128,
};
use ibcx_interface::airdrop::{AirdropId, ClaimPayload, ClaimProofOptional};

use crate::{
    error::ContractError,
    state::{Airdrop, AIRDROPS, CLAIM_LOGS, LABELS, LATEST_AIRDROP_ID},
    verify_merkle_proof,
};

pub fn register(
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

pub fn fund(deps: DepsMut, info: MessageInfo, id: AirdropId) -> Result<Response, ContractError> {
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

    let received = cw_utils::must_pay(&info, &airdrop.denom)?;
    airdrop.total_amount = airdrop.total_amount.checked_add(received)?;

    AIRDROPS.save(deps.storage, airdrop_id, &airdrop)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "fund"),
        attr("executor", info.sender),
        attr("airdrop_id", airdrop_id.to_string()),
        attr("amount", received),
    ]))
}

fn _claim(
    api: &dyn Api,
    storage: &mut dyn Storage,
    sender: &Addr,
    id: AirdropId,
    amount: Uint128,
    claim_proof: ClaimProofOptional,
    merkle_proof: Vec<String>,
) -> Result<((u64, Airdrop), Addr, Uint128), ContractError> {
    let airdrop_id = match id {
        AirdropId::Id(id) => id,
        AirdropId::Label(label) => LABELS.load(storage, &label)?,
    };

    let (claim_proof, beneficiary, bearer_expected) = match claim_proof {
        ClaimProofOptional::Account(account) => {
            let beneficiary = account
                .map(|v| api.addr_validate(&v))
                .transpose()?
                .unwrap_or_else(|| sender.clone());

            (beneficiary.to_string(), beneficiary, false)
        }
        ClaimProofOptional::ClaimProof(proof) => (proof, sender.clone(), true),
    };

    // verify merkle proof (from https://github.com/cosmwasm/cw-tokens/blob/master/contracts/cw20-merkle-airdrop/src/contract.rs)
    let mut airdrop = AIRDROPS.load(storage, airdrop_id)?;
    if airdrop.closed {
        return Err(ContractError::AirdropClosed {});
    }

    if airdrop.bearer != bearer_expected {
        return Err(ContractError::InvalidArguments {
            arg: "claim_proof".to_string(),
            reason: "unexpected proof type".to_string(),
        });
    }

    if CLAIM_LOGS
        .may_load(storage, (airdrop_id, &claim_proof))?
        .is_some()
    {
        return Err(ContractError::AlreadyClaimed {
            airdrop_id,
            claimer: beneficiary,
        });
    }

    verify_merkle_proof(&airdrop.merkle_root, merkle_proof, &claim_proof, amount)?;

    CLAIM_LOGS.save(storage, (airdrop_id, &claim_proof), &amount)?;

    airdrop.total_claimed = airdrop.total_claimed.checked_add(amount)?;
    if airdrop.total_claimed > airdrop.total_amount {
        return Err(ContractError::InsufficientAirdropFunds {});
    }
    AIRDROPS.save(storage, airdrop_id, &airdrop)?;

    Ok(((airdrop_id, airdrop), beneficiary, amount))
}

pub fn claim(
    deps: DepsMut,
    info: MessageInfo,
    ClaimPayload {
        id,
        amount,
        claim_proof,
        merkle_proof,
    }: ClaimPayload,
) -> Result<Response, ContractError> {
    let ((airdrop_id, airdrop), beneficiary, amount) = _claim(
        deps.api,
        deps.storage,
        &info.sender,
        id,
        amount,
        claim_proof,
        merkle_proof,
    )?;

    Ok(Response::new()
        .add_message(BankMsg::Send {
            to_address: beneficiary.to_string(),
            amount: coins(amount.u128(), airdrop.denom),
        })
        .add_attributes(vec![
            attr("action", "claim"),
            attr("executor", info.sender),
            attr("airdrop_id", airdrop_id.to_string()),
            attr("beneficiary", beneficiary),
            attr("amount", amount),
        ]))
}

pub fn multi_claim(
    deps: DepsMut,
    info: MessageInfo,
    payload: Vec<ClaimPayload>,
) -> Result<Response, ContractError> {
    let mut msgs: Vec<CosmosMsg> = vec![];
    let mut airdrop_ids = vec![];
    let mut beneficiaries = vec![];
    let mut amounts = vec![];

    for ClaimPayload {
        id,
        amount,
        claim_proof,
        merkle_proof,
    } in payload
    {
        let ((airdrop_id, airdrop), beneficiary, amount) = _claim(
            deps.api,
            deps.storage,
            &info.sender,
            id,
            amount,
            claim_proof,
            merkle_proof,
        )?;

        msgs.push(
            BankMsg::Send {
                to_address: beneficiary.to_string(),
                amount: coins(amount.u128(), airdrop.denom),
            }
            .into(),
        );

        airdrop_ids.push(airdrop_id);
        beneficiaries.push(beneficiary.to_string());
        amounts.push(amount.to_string());
    }

    Ok(Response::new().add_messages(msgs).add_attributes(vec![
        attr("action", "multi_claim"),
        attr("executor", info.sender),
        attr("airdrop_ids", format!("{airdrop_ids:?}")),
        attr("beneficiaries", format!("{beneficiaries:?}")),
        attr("amounts", format!("{amounts:?}")),
    ]))
}

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

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        coin,
        testing::{mock_dependencies, mock_env, mock_info},
    };
    use ibcx_interface::airdrop::InstantiateMsg;

    use crate::{
        contract::instantiate,
        test::{make_airdrop, SAMPLE_ROOT_TEST, SENDER_OWNER},
    };

    use super::*;

    fn setup(deps: DepsMut) {
        instantiate(deps, mock_env(), mock_info("owner", &[]), InstantiateMsg {}).unwrap();
    }

    mod register {
        use super::*;

        fn assert_resp(resp: Response, sender: &str, airdrop: &Airdrop) {
            assert_eq!(
                resp.attributes,
                vec![
                    attr("action", "register"),
                    attr("executor", sender),
                    attr("merkle_root", airdrop.merkle_root.to_string()),
                    attr("total_amount", airdrop.total_amount.to_string()),
                    attr(
                        "label",
                        airdrop
                            .label
                            .as_ref()
                            .map(|v| format!("{sender}/{v}"))
                            .unwrap_or_default(),
                    ),
                    attr("bearer", airdrop.bearer.to_string()),
                ]
            );
        }

        #[test]
        fn test_with_open_and_id() {
            let mut deps = mock_dependencies();

            setup(deps.as_mut());

            // raw
            let airdrop = make_airdrop(
                SENDER_OWNER,
                SAMPLE_ROOT_TEST,
                "uosmo",
                1000000u128,
                0u128,
                false,
                None,
            );
            let resp = register(
                deps.as_mut(),
                mock_info(
                    SENDER_OWNER,
                    &[coin(airdrop.total_amount.u128(), &airdrop.denom)],
                ),
                airdrop.merkle_root.clone(),
                airdrop.denom.clone(),
                airdrop.label.clone(),
                Some(airdrop.bearer),
            )
            .unwrap();
            assert_resp(resp, SENDER_OWNER, &airdrop);

            let latest_airdrop_id = LATEST_AIRDROP_ID.load(deps.as_ref().storage).unwrap();
            assert_eq!(latest_airdrop_id, 1);
            assert_eq!(airdrop, AIRDROPS.load(deps.as_ref().storage, 0).unwrap());
        }

        #[test]
        fn test_with_bearer_and_label() {
            let mut deps = mock_dependencies();

            setup(deps.as_mut());

            // with bearer
            let label = "label".to_string();
            let airdrop = make_airdrop(
                SENDER_OWNER,
                SAMPLE_ROOT_TEST,
                "uatom",
                2000000000u128,
                0u128,
                true,
                Some(label.clone()),
            );
            let resp = register(
                deps.as_mut(),
                mock_info(
                    SENDER_OWNER,
                    &[coin(airdrop.total_amount.u128(), &airdrop.denom)],
                ),
                airdrop.merkle_root.clone(),
                airdrop.denom.clone(),
                airdrop.label.clone(),
                Some(airdrop.bearer),
            )
            .unwrap();
            assert_resp(resp, SENDER_OWNER, &airdrop);
            let label = format!("{SENDER_OWNER}/{label}");

            let latest_airdrop_id = LATEST_AIRDROP_ID.load(deps.as_ref().storage).unwrap();
            assert_eq!(latest_airdrop_id, 1);
            assert_eq!(
                Airdrop {
                    label: Some(label.clone()),
                    ..airdrop
                },
                AIRDROPS.load(deps.as_ref().storage, 0).unwrap()
            );

            let airdrop_id_from_label = LABELS.load(deps.as_ref().storage, &label).unwrap();
            assert_eq!(airdrop_id_from_label, 0);
        }

        #[test]
        fn test_check_label_duplication() {
            let mut deps = mock_dependencies();

            setup(deps.as_mut());

            // with bearer
            let label = "label".to_string();
            let airdrop = make_airdrop(
                SENDER_OWNER,
                SAMPLE_ROOT_TEST,
                "uatom",
                2000000000u128,
                0u128,
                true,
                Some(label.clone()),
            );

            let resp = register(
                deps.as_mut(),
                mock_info(
                    SENDER_OWNER,
                    &[coin(airdrop.total_amount.u128(), &airdrop.denom)],
                ),
                airdrop.merkle_root.clone(),
                airdrop.denom.clone(),
                airdrop.label.clone(),
                Some(airdrop.bearer),
            )
            .unwrap();
            assert_resp(resp, SENDER_OWNER, &airdrop);

            let err = register(
                deps.as_mut(),
                mock_info(
                    SENDER_OWNER,
                    &[coin(airdrop.total_amount.u128(), &airdrop.denom)],
                ),
                airdrop.merkle_root.clone(),
                airdrop.denom.clone(),
                airdrop.label.clone(),
                Some(airdrop.bearer),
            )
            .unwrap_err();
            assert_eq!(
                err,
                ContractError::KeyAlreadyExists {
                    typ: "label".to_string(),
                    key: format!("{SENDER_OWNER}/{label}"),
                }
            );
        }
    }

    mod fund {
        use cosmwasm_std::Deps;

        use super::*;

        fn setup(deps: DepsMut, label: &str) -> Airdrop {
            let airdrop = make_airdrop(
                SENDER_OWNER,
                SAMPLE_ROOT_TEST,
                "uosmo",
                1000000u128,
                0u128,
                false,
                Some(label.to_string()),
            );
            register(
                deps,
                mock_info(
                    SENDER_OWNER,
                    &[coin(airdrop.total_amount.u128(), &airdrop.denom)],
                ),
                airdrop.merkle_root.clone(),
                airdrop.denom.clone(),
                airdrop.label.clone(),
                Some(airdrop.bearer),
            )
            .unwrap();

            airdrop
        }

        fn assert_resp(
            deps: Deps,
            resp: Response,
            sender: &str,
            amount: u128,
            airdrop_id: AirdropId,
        ) {
            let conv = match airdrop_id {
                AirdropId::Id(id) => id,
                AirdropId::Label(label) => LABELS.load(deps.storage, &label).unwrap(),
            }
            .to_string();

            assert_eq!(
                resp.attributes,
                vec![
                    attr("action", "fund"),
                    attr("executor", sender),
                    attr("airdrop_id", conv),
                    attr("amount", amount.to_string()),
                ]
            );
        }

        #[test]
        fn test_by_id() {
            let mut deps = mock_dependencies();

            super::setup(deps.as_mut());

            let label = "test_label";
            let airdrop = setup(deps.as_mut(), label);

            let fund_amount = 100000000u128;
            let sender = mock_info(SENDER_OWNER, &[coin(fund_amount, &airdrop.denom)]);
            let resp = fund(deps.as_mut(), sender, AirdropId::id(0)).unwrap();
            assert_resp(
                deps.as_ref(),
                resp,
                SENDER_OWNER,
                fund_amount,
                AirdropId::id(0),
            );

            let expected_amount = airdrop.total_amount + Uint128::from(fund_amount);
            let fetched_airdrop = AIRDROPS.load(deps.as_ref().storage, 0).unwrap();
            assert_eq!(fetched_airdrop.total_amount, expected_amount);
        }

        #[test]
        fn test_by_label() {
            let mut deps = mock_dependencies();

            super::setup(deps.as_mut());

            let label = "test_label";
            let airdrop = setup(deps.as_mut(), label);
            let label = format!("{SENDER_OWNER}/{label}");

            let fund_amount = 100000000u128;
            let sender = mock_info(SENDER_OWNER, &[coin(fund_amount, &airdrop.denom)]);
            let resp = fund(deps.as_mut(), sender, AirdropId::label(&label)).unwrap();
            assert_resp(
                deps.as_ref(),
                resp,
                SENDER_OWNER,
                fund_amount,
                AirdropId::label(label),
            );

            let expected_amount = airdrop.total_amount + Uint128::from(fund_amount);
            let fetched_airdrop = AIRDROPS.load(deps.as_ref().storage, 0).unwrap();
            assert_eq!(fetched_airdrop.total_amount, expected_amount);
        }
    }

    mod claim {
        use cosmwasm_std::Deps;

        use crate::test::{
            get_bearer_claims, get_open_claims, Claim, SAMPLE_ROOT_BEARER, SAMPLE_ROOT_OPEN,
        };

        use super::*;

        fn setup(
            deps: DepsMut,
            sender: &str,
            label: Option<&str>,
            amount: Option<u128>,
            is_bearer: bool,
        ) -> (Vec<Claim>, Airdrop) {
            let claims = if is_bearer {
                get_bearer_claims("claimer")
            } else {
                get_open_claims()
            };

            let claim_total_amount = claims
                .iter()
                .fold(Uint128::zero(), |acc, c| acc + Uint128::from(c.amount));

            let airdrop = make_airdrop(
                sender,
                if is_bearer {
                    SAMPLE_ROOT_BEARER
                } else {
                    SAMPLE_ROOT_OPEN
                },
                "uosmo",
                claim_total_amount,
                0u128,
                is_bearer,
                label.map(|v| v.to_string()),
            );

            register(
                deps,
                mock_info(
                    sender,
                    &[coin(
                        amount.unwrap_or_else(|| airdrop.total_amount.u128()),
                        &airdrop.denom,
                    )],
                ),
                airdrop.merkle_root.clone(),
                airdrop.denom.clone(),
                airdrop.label.clone(),
                Some(airdrop.bearer),
            )
            .unwrap();

            (claims, airdrop)
        }

        fn assert_resp(deps: Deps, resp: Response, sender: &str, payload: ClaimPayload) {
            let conv = match payload.id.clone() {
                AirdropId::Id(id) => id,
                AirdropId::Label(label) => LABELS.load(deps.storage, &label).unwrap(),
            }
            .to_string();

            assert_eq!(
                resp.attributes,
                vec![
                    attr("action", "claim"),
                    attr("executor", sender),
                    attr("airdrop_id", conv),
                    attr(
                        "beneficiary",
                        match payload.claim_proof {
                            ClaimProofOptional::Account(acc) =>
                                acc.unwrap_or_else(|| sender.to_string()),
                            ClaimProofOptional::ClaimProof(_) => sender.to_string(),
                        }
                    ),
                    attr("amount", payload.amount.to_string()),
                ]
            )
        }

        fn assert_multi_resp(deps: Deps, resp: Response, sender: &str, payload: Vec<ClaimPayload>) {
            let (airdrop_ids, beneficiaries, amounts) = payload.into_iter().fold(
                (vec![], vec![], vec![]),
                |(mut airdrop_ids, mut beneficiaries, mut amounts), v| {
                    airdrop_ids.push(match v.id {
                        AirdropId::Id(id) => id,
                        AirdropId::Label(label) => LABELS.load(deps.storage, &label).unwrap(),
                    });

                    beneficiaries.push(match v.claim_proof {
                        ClaimProofOptional::Account(acc) => {
                            acc.unwrap_or_else(|| sender.to_string())
                        }
                        ClaimProofOptional::ClaimProof(_) => sender.to_string(),
                    });

                    amounts.push(v.amount.to_string());

                    (airdrop_ids, beneficiaries, amounts)
                },
            );

            assert_eq!(
                resp.attributes,
                vec![
                    attr("action", "multi_claim"),
                    attr("executor", sender),
                    attr("airdrop_ids", format!("{airdrop_ids:?}")),
                    attr("beneficiaries", format!("{beneficiaries:?}")),
                    attr("amounts", format!("{amounts:?}"))
                ]
            )
        }

        #[test]
        fn test_open_with_id() {
            let mut deps = mock_dependencies();

            super::setup(deps.as_mut());

            let label = None;
            let (claims, airdrop) = setup(deps.as_mut(), SENDER_OWNER, label, None, false);

            for claim in claims {
                let resp = claim.execute(deps.as_mut(), AirdropId::Id(0)).unwrap();
                assert_resp(
                    deps.as_ref(),
                    resp,
                    &claim.account,
                    claim.to_payload(AirdropId::Id(0)),
                );
            }

            let fetched_airdrop = AIRDROPS.load(deps.as_ref().storage, 0).unwrap();
            assert_eq!(fetched_airdrop.total_claimed, airdrop.total_amount);
            assert_eq!(fetched_airdrop.total_amount, fetched_airdrop.total_claimed);
        }

        #[test]
        fn test_multi_open_with_ids() {
            let mut deps = mock_dependencies();

            super::setup(deps.as_mut());

            let label = None;
            let (claims, _) = setup(deps.as_mut(), SENDER_OWNER, label, None, false);

            let payload = claims
                .into_iter()
                .map(|v| v.to_payload(AirdropId::id(0)))
                .collect::<Vec<_>>();

            let resp =
                multi_claim(deps.as_mut(), mock_info(SENDER_OWNER, &[]), payload.clone()).unwrap();
            assert_multi_resp(deps.as_ref(), resp, SENDER_OWNER, payload);
        }

        #[test]
        fn test_bearer_with_label() {
            let mut deps = mock_dependencies();

            super::setup(deps.as_mut());

            let label = Some("bearer");
            let (claims, airdrop) = setup(deps.as_mut(), SENDER_OWNER, label, None, true);
            let label = format!("{SENDER_OWNER}/{}", label.unwrap());

            for claim in claims {
                let resp = claim
                    .execute(deps.as_mut(), AirdropId::label(&label))
                    .unwrap();
                assert_resp(
                    deps.as_ref(),
                    resp,
                    &claim.account,
                    claim.to_payload(AirdropId::label(&label)),
                );
            }

            let fetched_airdrop = AIRDROPS.load(deps.as_ref().storage, 0).unwrap();
            assert_eq!(fetched_airdrop.total_claimed, airdrop.total_amount);
            assert_eq!(fetched_airdrop.total_amount, fetched_airdrop.total_claimed);
        }

        #[test]
        fn test_multi_bearer_with_labels() {
            let mut deps = mock_dependencies();

            super::setup(deps.as_mut());

            let label = Some("multi_bearer");
            let (claims, _) = setup(deps.as_mut(), SENDER_OWNER, label, None, false);
            let label = format!("{SENDER_OWNER}/{}", label.unwrap());

            let payload = claims
                .into_iter()
                .map(|v| v.to_payload(AirdropId::label(&label)))
                .collect::<Vec<_>>();

            let resp =
                multi_claim(deps.as_mut(), mock_info(SENDER_OWNER, &[]), payload.clone()).unwrap();
            assert_multi_resp(deps.as_ref(), resp, SENDER_OWNER, payload);
        }

        #[test]
        fn test_check_overflow() {
            let mut deps = mock_dependencies();

            super::setup(deps.as_mut());

            let label = Some("overflow");
            let (claims, _) = setup(deps.as_mut(), SENDER_OWNER, label, Some(1), false);
            let label = format!("{SENDER_OWNER}/{}", label.unwrap());

            let err = claims[0]
                .execute(deps.as_mut(), AirdropId::Label(label))
                .unwrap_err();
            assert_eq!(err, ContractError::InsufficientAirdropFunds {});
        }

        #[test]
        fn test_check_claim_proof_type() {
            let mut deps = mock_dependencies();

            super::setup(deps.as_mut());

            let label = Some("overflow");
            let (claims, _) = setup(deps.as_mut(), SENDER_OWNER, label, None, false);
            let label = format!("{SENDER_OWNER}/{}", label.unwrap());

            let mut claim = claims[0].clone();
            claim.claim_proof = ClaimProofOptional::claim_proof("");

            let err = claim
                .execute(deps.as_mut(), AirdropId::Label(label))
                .unwrap_err();
            assert_eq!(
                err,
                ContractError::InvalidArguments {
                    arg: "claim_proof".to_string(),
                    reason: "unexpected proof type".to_string()
                }
            );
        }

        #[test]
        fn test_check_double_spending() {
            let mut deps = mock_dependencies();

            super::setup(deps.as_mut());

            let label = Some("double_spending");
            let (claims, _) = setup(deps.as_mut(), SENDER_OWNER, label, None, false);
            let label = format!("{SENDER_OWNER}/{}", label.unwrap());

            // first claim
            claims[0]
                .execute(deps.as_mut(), AirdropId::label(&label))
                .unwrap();

            // second claim - should aborted
            let err = claims[0]
                .execute(deps.as_mut(), AirdropId::label(label))
                .unwrap_err();
            assert_eq!(
                err,
                ContractError::AlreadyClaimed {
                    airdrop_id: 0,
                    claimer: Addr::unchecked(&claims[0].account),
                }
            )
        }

        #[test]
        fn test_check_merkle_proof() {
            let mut deps = mock_dependencies();

            super::setup(deps.as_mut());

            let label = Some("double_spending");
            let (claims, _) = setup(deps.as_mut(), SENDER_OWNER, label, None, false);
            let label = format!("{SENDER_OWNER}/{}", label.unwrap());

            let mut claim_mixed = claims[1].clone();
            claim_mixed.merkle_proof = claims[2].merkle_proof.clone();

            let err = claim_mixed
                .execute(deps.as_mut(), AirdropId::Label(label))
                .unwrap_err();
            assert_eq!(err, ContractError::InvalidProof {});
        }
    }

    mod close {
        use cosmwasm_std::SubMsg;

        use super::*;

        #[test]
        fn test_close() {
            let mut deps = mock_dependencies();

            super::setup(deps.as_mut());

            let airdrop = make_airdrop(
                SENDER_OWNER,
                SAMPLE_ROOT_TEST,
                "uosmo",
                100u128,
                0u128,
                false,
                None,
            );

            register(
                deps.as_mut(),
                mock_info(
                    SENDER_OWNER,
                    &[coin(airdrop.total_amount.u128(), &airdrop.denom)],
                ),
                airdrop.merkle_root.clone(),
                airdrop.denom.clone(),
                airdrop.label.clone(),
                Some(airdrop.bearer),
            )
            .unwrap();

            assert_eq!(
                close(deps.as_mut(), mock_info("anyone", &[]), AirdropId::id(0)).unwrap_err(),
                ContractError::Unauthorized {}
            );

            let resp = close(
                deps.as_mut(),
                mock_info(SENDER_OWNER, &[]),
                AirdropId::id(0),
            )
            .unwrap();

            assert_eq!(
                resp.attributes,
                vec![
                    attr("method", "close"),
                    attr("executor", SENDER_OWNER),
                    attr("airdrop_id", "0"),
                    attr("redeemed", "100")
                ]
            );

            assert_eq!(
                resp.messages.first().unwrap(),
                &SubMsg::new(BankMsg::Send {
                    to_address: SENDER_OWNER.to_string(),
                    amount: vec![coin(100u128, &airdrop.denom)],
                })
            );
        }
    }
}
