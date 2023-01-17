use cosmwasm_std::{to_binary, Addr, Deps, Order, QueryResponse, StdError, StdResult, Uint128};
use cw_storage_plus::Bound;
use ibcx_interface::{
    airdrop::{
        AirdropId, AirdropIdOptional, CheckQualificationResponse, ClaimProof, GetAirdropResponse,
        GetClaimResponse, LatestAirdropResponse, ListAirdropsResponse, ListClaimsResponse,
    },
    get_and_check_limit,
    types::RangeOrder,
    DEFAULT_LIMIT, MAX_LIMIT,
};

use crate::{
    error::ContractError,
    state::{AIRDROPS, CLAIM_LOGS, LABELS, LATEST_AIRDROP_ID},
    verify_merkle_proof,
};

pub fn get_airdrop(deps: Deps, id: AirdropId) -> Result<QueryResponse, ContractError> {
    let airdrop_id = match id {
        AirdropId::Id(id) => Ok(id),
        AirdropId::Label(l) => LABELS
            .load(deps.storage, &l)
            .map_err(|_| StdError::not_found("label")),
    }?;
    let airdrop = AIRDROPS
        .load(deps.storage, airdrop_id)
        .map_err(|_| StdError::not_found("airdrop"))?;

    Ok(to_binary(&GetAirdropResponse {
        id: airdrop_id,
        label: airdrop.label,
        denom: airdrop.denom,
        total_amount: airdrop.total_amount,
        total_claimed: airdrop.total_claimed,
        merkle_root: airdrop.merkle_root,
        bearer: airdrop.bearer,
    })?)
}

pub fn list_airdrops(
    deps: Deps,
    start_after: AirdropIdOptional,
    limit: Option<u32>,
    order: Option<RangeOrder>,
) -> Result<QueryResponse, ContractError> {
    let limit = get_and_check_limit(limit, MAX_LIMIT, DEFAULT_LIMIT)? as usize;
    let order = order.unwrap_or(RangeOrder::Asc).into();

    let resps = match start_after {
        AirdropIdOptional::Id(id) => {
            let (min, max) = match order {
                Order::Ascending => (id.map(Bound::exclusive), None),
                Order::Descending => (None, id.map(Bound::exclusive)),
            };

            AIRDROPS
                .range(deps.storage, min, max, order)
                .take(limit)
                .map(|item| {
                    let (k, v) = item?;

                    Ok(GetAirdropResponse {
                        id: k,
                        label: v.label,
                        denom: v.denom,
                        total_amount: v.total_amount,
                        total_claimed: v.total_claimed,
                        merkle_root: v.merkle_root,
                        bearer: v.bearer,
                    })
                })
                .collect::<StdResult<_>>()?
        }
        AirdropIdOptional::Label(l) => {
            let start = l.as_deref();
            let (min, max) = match order {
                Order::Ascending => (start.map(Bound::exclusive), None),
                Order::Descending => (None, start.map(Bound::exclusive)),
            };

            LABELS
                .range(deps.storage, min, max, order)
                .take(limit)
                .map(|item| {
                    let (k, v) = item?;

                    let airdrop = AIRDROPS.load(deps.storage, v)?;

                    Ok(GetAirdropResponse {
                        id: v,
                        label: Some(k),
                        denom: airdrop.denom,
                        total_amount: airdrop.total_amount,
                        total_claimed: airdrop.total_claimed,
                        merkle_root: airdrop.merkle_root,
                        bearer: airdrop.bearer,
                    })
                })
                .collect::<StdResult<_>>()?
        }
    };

    Ok(to_binary(&ListAirdropsResponse(resps))?)
}

pub fn latest_airdrop_id(deps: Deps) -> Result<QueryResponse, ContractError> {
    Ok(to_binary(&LatestAirdropResponse(
        LATEST_AIRDROP_ID.load(deps.storage)?,
    ))?)
}

pub fn get_claim(
    deps: Deps,
    id: AirdropId,
    claim_proof: ClaimProof,
) -> Result<QueryResponse, ContractError> {
    let airdrop_id = match id {
        AirdropId::Id(id) => Ok(id),
        AirdropId::Label(l) => LABELS
            .load(deps.storage, &l)
            .map_err(|_| StdError::not_found("label")),
    }?;

    let amount = match claim_proof {
        ClaimProof::Account(ref account) => {
            deps.api.addr_validate(account)?;
            CLAIM_LOGS.load(deps.storage, (airdrop_id, account))
        }
        ClaimProof::ClaimProof(ref proof) => CLAIM_LOGS.load(deps.storage, (airdrop_id, proof)),
    }
    .map_err(|_| ContractError::Std(StdError::not_found("claim")))?;

    Ok(to_binary(&GetClaimResponse {
        amount,
        claim_proof,
    })?)
}

pub fn list_claims(
    deps: Deps,
    id: AirdropId,
    start_after: Option<String>,
    limit: Option<u32>,
    order: Option<RangeOrder>,
) -> Result<QueryResponse, ContractError> {
    let airdrop_id = match id {
        AirdropId::Id(id) => Ok(id),
        AirdropId::Label(l) => LABELS
            .load(deps.storage, &l)
            .map_err(|_| StdError::not_found("label")),
    }?;
    let airdrop = AIRDROPS
        .load(deps.storage, airdrop_id)
        .map_err(|_| StdError::not_found("airdrop"))?;

    let start = start_after.as_deref();
    let limit = get_and_check_limit(limit, MAX_LIMIT, DEFAULT_LIMIT)? as usize;
    let order = order.unwrap_or(RangeOrder::Asc).into();
    let (min, max) = match order {
        Order::Ascending => (start.map(Bound::exclusive), None),
        Order::Descending => (None, start.map(Bound::exclusive)),
    };

    let resps = CLAIM_LOGS
        .prefix(airdrop_id)
        .range(deps.storage, min, max, order)
        .take(limit)
        .map(|item| {
            let (k, v) = item?;

            let claim_proof = if airdrop.bearer {
                ClaimProof::ClaimProof(k)
            } else {
                ClaimProof::Account(k)
            };

            Ok(GetClaimResponse {
                amount: v,
                claim_proof,
            })
        })
        .collect::<StdResult<_>>()?;

    Ok(to_binary(&ListClaimsResponse(resps))?)
}

pub fn check_qualification(
    deps: Deps,
    id: AirdropId,
    amount: Uint128,
    claim_proof: ClaimProof,
    merkle_proof: Vec<String>,
) -> Result<QueryResponse, ContractError> {
    let airdrop_id = match id {
        AirdropId::Id(id) => id,
        AirdropId::Label(l) => LABELS.load(deps.storage, &l)?,
    };

    let (claim_proof, bearer_expected) = match claim_proof {
        ClaimProof::Account(account) => (account, false),
        ClaimProof::ClaimProof(proof) => (proof, true),
    };

    let airdrop = AIRDROPS.load(deps.storage, airdrop_id)?;
    if airdrop.bearer != bearer_expected {
        return Err(ContractError::InvalidArguments {
            arg: "claim_proof".to_string(),
            reason: "unexpected proof type".to_string(),
        });
    }

    if CLAIM_LOGS
        .may_load(deps.storage, (airdrop_id, &claim_proof))?
        .is_some()
    {
        return Err(ContractError::AlreadyClaimed {
            airdrop_id,
            claimer: Addr::unchecked("<simulation>"),
        });
    }

    Ok(to_binary(&CheckQualificationResponse(
        verify_merkle_proof(&airdrop.merkle_root, merkle_proof, &claim_proof, amount).is_ok(),
    ))?)
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        coin, from_binary,
        testing::{mock_dependencies, mock_env, mock_info},
        DepsMut,
    };
    use ibcx_interface::airdrop::{ClaimProofOptional, InstantiateMsg};

    use crate::{
        contract::instantiate,
        execute::register,
        state::Airdrop,
        test::{
            get_bearer_claims, get_open_claims, make_airdrop, Claim, SAMPLE_ROOT_BEARER,
            SAMPLE_ROOT_OPEN, SENDER_OWNER,
        },
    };

    use super::*;

    fn setup(deps: DepsMut) {
        instantiate(deps, mock_env(), mock_info("owner", &[]), InstantiateMsg {}).unwrap();
    }

    fn setup_airdrop(
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
                    amount.unwrap_or(airdrop.total_amount.u128()),
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

    mod airdrop {

        use super::*;

        fn resp_to_airdrop(resp: GetAirdropResponse) -> Airdrop {
            Airdrop {
                label: resp.label,
                denom: resp.denom,
                total_amount: resp.total_amount,
                total_claimed: resp.total_claimed,
                merkle_root: resp.merkle_root,
                bearer: resp.bearer,
            }
        }

        #[test]
        fn test_get_by_id() {
            let mut deps = mock_dependencies();

            setup(deps.as_mut());
            let (_, airdrop) = setup_airdrop(deps.as_mut(), SENDER_OWNER, None, None, false);

            let resp: GetAirdropResponse =
                from_binary(&get_airdrop(deps.as_ref(), AirdropId::id(0)).unwrap()).unwrap();
            assert_eq!(resp_to_airdrop(resp), airdrop);

            // check not found
            let err = get_airdrop(deps.as_ref(), AirdropId::id(1)).unwrap_err();
            assert_eq!(err, ContractError::Std(StdError::not_found("airdrop")));
        }

        #[test]
        fn test_list_by_id() {
            let mut deps = mock_dependencies();

            setup(deps.as_mut());

            let open_label = Some("1_list_query_open");
            let bearer_label = Some("2_list_query_bearer");

            let (_, airdrop_open_unlabeled) =
                setup_airdrop(deps.as_mut(), SENDER_OWNER, None, None, false);
            let (_, airdrop_open_labeled) =
                setup_airdrop(deps.as_mut(), SENDER_OWNER, open_label, None, false);
            let (_, airdrop_bearer_unlabeled) =
                setup_airdrop(deps.as_mut(), SENDER_OWNER, None, None, true);
            let (_, airdrop_bearer_labeled) =
                setup_airdrop(deps.as_mut(), SENDER_OWNER, bearer_label, None, true);

            let mut expected_resp = vec![
                airdrop_open_unlabeled,
                airdrop_open_labeled,
                airdrop_bearer_unlabeled,
                airdrop_bearer_labeled,
            ]
            .into_iter()
            .map(|mut v| {
                if let Some(label) = v.label {
                    v.label = Some(format!("{SENDER_OWNER}/{}", label));
                }

                v
            })
            .collect::<Vec<_>>();

            let query = |deps: Deps, order: RangeOrder| {
                list_airdrops(deps, AirdropIdOptional::Id(None), None, Some(order)).unwrap()
            };

            // asc
            let resp: ListAirdropsResponse =
                from_binary(&query(deps.as_ref(), RangeOrder::Asc)).unwrap();
            assert_eq!(
                resp.0.into_iter().map(resp_to_airdrop).collect::<Vec<_>>(),
                expected_resp
            );

            // flip
            expected_resp.reverse();

            // desc
            let resp: ListAirdropsResponse =
                from_binary(&query(deps.as_ref(), RangeOrder::Desc)).unwrap();
            assert_eq!(
                resp.0.into_iter().map(resp_to_airdrop).collect::<Vec<_>>(),
                expected_resp
            );
        }

        #[test]
        fn test_get_by_label() {
            let mut deps = mock_dependencies();

            setup(deps.as_mut());
            let label = Some("get_query");
            let (_, airdrop) = setup_airdrop(deps.as_mut(), SENDER_OWNER, label, None, false);
            let label = format!("{SENDER_OWNER}/{}", label.unwrap());

            let resp: GetAirdropResponse =
                from_binary(&get_airdrop(deps.as_ref(), AirdropId::label(&label)).unwrap())
                    .unwrap();
            assert_eq!(
                resp_to_airdrop(resp),
                Airdrop {
                    label: Some(label.clone()),
                    ..airdrop
                }
            );

            // check not found
            let err = get_airdrop(deps.as_ref(), AirdropId::label("nope")).unwrap_err();
            assert_eq!(err, ContractError::Std(StdError::not_found("label")));
        }

        #[test]
        fn test_list_by_label() {
            let mut deps = mock_dependencies();

            setup(deps.as_mut());

            let open_label = Some("1_list_query_open");
            let bearer_label = Some("2_list_query_bearer");

            let (_, airdrop_open_labeled) =
                setup_airdrop(deps.as_mut(), SENDER_OWNER, open_label, None, false);
            let (_, airdrop_bearer_labeled) =
                setup_airdrop(deps.as_mut(), SENDER_OWNER, bearer_label, None, true);

            // noise
            let _ = setup_airdrop(deps.as_mut(), SENDER_OWNER, None, None, false);
            let _ = setup_airdrop(deps.as_mut(), SENDER_OWNER, None, None, true);

            let mut expected_resp = vec![airdrop_open_labeled, airdrop_bearer_labeled]
                .into_iter()
                .map(|v| Airdrop {
                    label: Some(format!("{SENDER_OWNER}/{}", v.label.unwrap())),
                    ..v
                })
                .collect::<Vec<_>>();

            let query = |deps: Deps, order: RangeOrder| {
                list_airdrops(deps, AirdropIdOptional::Label(None), None, Some(order)).unwrap()
            };

            // asc
            let resp: ListAirdropsResponse =
                from_binary(&query(deps.as_ref(), RangeOrder::Asc)).unwrap();
            assert_eq!(
                resp.0.into_iter().map(resp_to_airdrop).collect::<Vec<_>>(),
                expected_resp
            );

            // flip
            expected_resp.reverse();

            // desc
            let resp: ListAirdropsResponse =
                from_binary(&query(deps.as_ref(), RangeOrder::Desc)).unwrap();
            assert_eq!(
                resp.0.into_iter().map(resp_to_airdrop).collect::<Vec<_>>(),
                expected_resp
            );
        }

        #[test]
        fn test_latest_airdrop_id() {
            let mut deps = mock_dependencies();

            setup(deps.as_mut());

            let resp: LatestAirdropResponse =
                from_binary(&latest_airdrop_id(deps.as_ref()).unwrap()).unwrap();
            assert_eq!(resp.0, 0);

            // register one airdrop
            setup_airdrop(deps.as_mut(), SENDER_OWNER, None, None, false);

            let resp: LatestAirdropResponse =
                from_binary(&latest_airdrop_id(deps.as_ref()).unwrap()).unwrap();
            assert_eq!(resp.0, 1);
        }
    }

    #[test]
    fn query_claim() {
        let mut deps = mock_dependencies();

        setup(deps.as_mut());

        // register airdrop & claim
        let open_label = Some("open_claim_test");
        let bearer_label = Some("bearer_claim_test");

        let (open_claims, _) = setup_airdrop(deps.as_mut(), SENDER_OWNER, open_label, None, false);
        let (bearer_claims, _) =
            setup_airdrop(deps.as_mut(), SENDER_OWNER, bearer_label, None, true);

        let open_label = format!("{SENDER_OWNER}/{}", open_label.unwrap());
        let bearer_label = format!("{SENDER_OWNER}/{}", bearer_label.unwrap());

        for idx in 0..open_claims.len() {
            open_claims[idx]
                .execute(deps.as_mut(), AirdropId::label(&open_label))
                .unwrap();
            bearer_claims[idx]
                .execute(deps.as_mut(), AirdropId::label(&bearer_label))
                .unwrap();
        }

        // utility
        let unwrap_optional = |claim: &Claim| match &claim.claim_proof {
            ClaimProofOptional::Account(acc) => {
                ClaimProof::Account(acc.clone().unwrap_or_else(|| claim.account.to_string()))
            }
            ClaimProofOptional::ClaimProof(proof) => ClaimProof::ClaimProof(proof.to_string()),
        };

        // get by id
        let resp: GetClaimResponse = from_binary(
            &get_claim(
                deps.as_ref(),
                AirdropId::Id(0),
                unwrap_optional(&open_claims[0]),
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(resp.claim_proof, unwrap_optional(&open_claims[0]));
        assert_eq!(resp.amount, Uint128::from(open_claims[0].amount));

        // get by label
        let resp: GetClaimResponse = from_binary(
            &get_claim(
                deps.as_ref(),
                AirdropId::label(&open_label),
                unwrap_optional(&open_claims[0]),
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(resp.claim_proof, unwrap_optional(&open_claims[0]));
        assert_eq!(resp.amount, Uint128::from(open_claims[0].amount));

        // check label exists
        let err = get_claim(
            deps.as_ref(),
            AirdropId::label("asdf"),
            unwrap_optional(&open_claims[0]),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            ContractError::Std(StdError::NotFound { kind })
            if kind == "label",
        ));

        // check claim exists
        let err = get_claim(
            deps.as_ref(),
            AirdropId::label(&open_label),
            unwrap_optional(&bearer_claims[0]),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            ContractError::Std(StdError::NotFound { kind })
            if kind == "claim",
        ));

        // list claim
        let airdrop_ids = [
            AirdropId::Id(0),
            AirdropId::label(&open_label),
            AirdropId::Id(1),
            AirdropId::Label(bearer_label),
        ];

        let query = |deps: Deps, id: AirdropId, order: RangeOrder| {
            list_claims(deps, id, None, None, Some(order)).unwrap()
        };

        let resps = airdrop_ids
            .into_iter()
            .map(|id| from_binary(&query(deps.as_ref(), id, RangeOrder::Asc)).unwrap())
            .collect::<Vec<ListClaimsResponse>>();

        let resp_open_id = resps.get(0).unwrap();
        let resp_open_label = resps.get(1).unwrap();
        let resp_bearer_id = resps.get(2).unwrap();
        let resp_bearer_label = resps.get(3).unwrap();

        // must sort & compare
        let cmp_claims = |a: &Claim, b: &Claim| {
            let a = unwrap_optional(a);
            let b = unwrap_optional(b);

            match (a, b) {
                (ClaimProof::Account(a), ClaimProof::Account(b)) => a.cmp(&b),
                (ClaimProof::ClaimProof(a), ClaimProof::ClaimProof(b)) => a.cmp(&b),
                _ => panic!("type doesn't match"),
            }
        };

        let mut sorted_claims = open_claims;
        sorted_claims.sort_by(|a, b| cmp_claims(a, b));

        let expected_resps = sorted_claims
            .into_iter()
            .map(|v| GetClaimResponse {
                amount: Uint128::from(v.amount),
                claim_proof: unwrap_optional(&v),
            })
            .collect::<Vec<_>>();

        assert_eq!(resp_open_id.0, expected_resps);
        assert_eq!(resp_open_label.0, expected_resps);

        let mut sorted_claims = bearer_claims;
        sorted_claims.sort_by(|a, b| cmp_claims(a, b));

        let expected_resps = sorted_claims
            .into_iter()
            .map(|v| GetClaimResponse {
                amount: Uint128::from(v.amount),
                claim_proof: unwrap_optional(&v),
            })
            .collect::<Vec<_>>();

        assert_eq!(resp_bearer_id.0, expected_resps);
        assert_eq!(resp_bearer_label.0, expected_resps);
    }

    #[test]
    fn test_check_qualification() {
        let mut deps = mock_dependencies();

        setup(deps.as_mut());
        let label = Some("airdrop");
        let (claims, _) = setup_airdrop(deps.as_mut(), SENDER_OWNER, label, None, false);

        let unwrap_optional = |claim: &Claim| match &claim.claim_proof {
            ClaimProofOptional::Account(acc) => {
                ClaimProof::Account(acc.clone().unwrap_or_else(|| claim.account.to_string()))
            }
            ClaimProofOptional::ClaimProof(proof) => ClaimProof::ClaimProof(proof.to_string()),
        };

        for claim in claims.clone() {
            let resp: CheckQualificationResponse = from_binary(
                &check_qualification(
                    deps.as_ref(),
                    AirdropId::Id(0),
                    Uint128::new(claim.amount),
                    unwrap_optional(&claim),
                    claim.merkle_proof.clone(),
                )
                .unwrap(),
            )
            .unwrap();
            assert!(resp.0);
        }

        let bearer_claims = get_bearer_claims("claimer");
        for claim in claims {
            let resp: CheckQualificationResponse = from_binary(
                &check_qualification(
                    deps.as_ref(),
                    AirdropId::Id(0),
                    Uint128::new(claim.amount),
                    unwrap_optional(&claim),
                    bearer_claims[0].merkle_proof.clone(),
                )
                .unwrap(),
            )
            .unwrap();
            assert!(!resp.0);
        }
    }
}
