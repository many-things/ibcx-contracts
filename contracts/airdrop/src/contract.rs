use cosmwasm_std::{
    attr, coins, entry_point, to_binary, Addr, Api, BankMsg, CosmosMsg, Env, MessageInfo, Order,
    QueryResponse, StdError, StdResult, Storage, Uint128,
};
use cosmwasm_std::{Deps, DepsMut, Response};
use cw_storage_plus::Bound;
use ibcx_interface::airdrop::ClaimPayload;
use ibcx_interface::{
    airdrop::{
        AirdropId, AirdropIdOptional, CheckQualificationResponse, ClaimProof, ClaimProofOptional,
        ExecuteMsg, GetAirdropResponse, GetClaimResponse, InstantiateMsg, LatestAirdropResponse,
        ListAirdropsResponse, ListClaimsResponse, MigrateMsg, QueryMsg,
    },
    get_and_check_limit,
    types::RangeOrder,
    DEFAULT_LIMIT, MAX_LIMIT,
};
use sha2::Digest;

use crate::{
    error::ContractError,
    state::{Airdrop, AIRDROPS, CLAIM_LOGS, LABELS, LATEST_AIRDROP_ID},
    CONTRACT_NAME, CONTRACT_VERSION,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    LATEST_AIRDROP_ID.save(deps.storage, &0)?;

    Ok(Default::default())
}

// verify merkle proof (from https://github.com/cosmwasm/cw-tokens/blob/master/contracts/cw20-merkle-airdrop/src/contract.rs)
fn verify_merkle_proof(
    root: &str,
    proof: Vec<String>,
    claim_proof: &str,
    amount: Uint128,
) -> Result<(), ContractError> {
    let user_input = format!("{}{}", claim_proof, amount);

    let hash = sha2::Sha256::digest(user_input.as_bytes())
        .as_slice()
        .try_into()
        .map_err(|_| ContractError::WrongLength {})?;

    let hash = proof.into_iter().try_fold(hash, |hash, p| {
        let mut proof_buf = [0; 32];
        hex::decode_to_slice(p, &mut proof_buf)?;
        let mut hashes = [hash, proof_buf];
        hashes.sort_unstable();
        sha2::Sha256::digest(&hashes.concat())
            .as_slice()
            .try_into()
            .map_err(|_| ContractError::WrongLength {})
    })?;

    let mut root_buf: [u8; 32] = [0; 32];
    hex::decode_to_slice(root, &mut root_buf)?;
    if root_buf != hash {
        return Err(ContractError::InvalidProof {});
    }

    Ok(())
}

fn claim(
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

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        Register {
            merkle_root,
            denom,
            label,
            bearer,
        } => {
            let received = cw_utils::must_pay(&info, &denom)?;

            // check merkle root length
            let mut root_buf: [u8; 32] = [0; 32];
            hex::decode_to_slice(&merkle_root, &mut root_buf)?;

            let airdrop_id = LATEST_AIRDROP_ID.load(deps.storage)?;
            LATEST_AIRDROP_ID.save(deps.storage, &(airdrop_id + 1))?;

            let label = label.map(|v| format!("{}/{}", info.sender, v));

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
                    merkle_root: merkle_root.clone(),
                    denom,
                    total_amount: received,
                    total_claimed: Uint128::zero(),
                    bearer: bearer.unwrap_or(false),
                    label: label.clone(),
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
        Fund { id } => {
            let airdrop_id = match id {
                AirdropId::Id(id) => id,
                AirdropId::Label(label) => LABELS.load(deps.storage, &label)?,
            };
            let mut airdrop = AIRDROPS.load(deps.storage, airdrop_id)?;

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
        Claim(ClaimPayload {
            id,
            amount,
            claim_proof,
            merkle_proof,
        }) => {
            let ((airdrop_id, airdrop), beneficiary, amount) = claim(
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
        MultiClaim(claims) => {
            let mut msgs: Vec<CosmosMsg> = vec![];
            let mut airdrop_ids = vec![];
            let mut beneficiaries = vec![];
            let mut amounts = vec![];

            for ClaimPayload {
                id,
                amount,
                claim_proof,
                merkle_proof,
            } in claims
            {
                let ((airdrop_id, airdrop), beneficiary, amount) = claim(
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
                        amount: coins(amount.u128(), &airdrop.denom),
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
                attr("airdrop_ids", format!("{:?}", airdrop_ids)),
                attr("beneficiaries", format!("{:?}", beneficiaries)),
                attr("amounts", format!("{:?}", amounts)),
            ]))
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use QueryMsg::*;

    match msg {
        GetAirdrop { id } => {
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
                bearer: airdrop.bearer,
            })?)
        }
        ListAirdrops {
            start_after,
            limit,
            order,
        } => {
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
                                bearer: airdrop.bearer,
                            })
                        })
                        .collect::<StdResult<_>>()?
                }
            };

            Ok(to_binary(&ListAirdropsResponse(resps))?)
        }
        LatestAirdropId {} => Ok(to_binary(&LatestAirdropResponse(
            LATEST_AIRDROP_ID.load(deps.storage)?,
        ))?),
        GetClaim { id, claim_proof } => {
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
                ClaimProof::ClaimProof(ref proof) => {
                    CLAIM_LOGS.load(deps.storage, (airdrop_id, proof))
                }
            }?;

            Ok(to_binary(&GetClaimResponse {
                amount,
                claim_proof,
            })?)
        }
        ListClaims {
            id,
            start_after,
            limit,
            order,
        } => {
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
        CheckQualification {
            id,
            amount,
            claim_proof,
            merkle_proof,
        } => {
            let airdrop_id = match id {
                AirdropId::Id(id) => id,
                AirdropId::Label(l) => LABELS.load(deps.storage, &l)?,
            };

            let (claim_proof, bearer_expected) = match claim_proof {
                ClaimProof::Account(account) => (account, false),
                ClaimProof::ClaimProof(proof) => (proof, true),
            };

            let airdrop = AIRDROPS.load(deps.storage, airdrop_id)?;
            if !(airdrop.bearer ^ bearer_expected) {
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
                verify_merkle_proof(&airdrop.merkle_root, merkle_proof, &claim_proof, amount)
                    .is_ok(),
            ))?)
        }
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    ibcx_utils::store_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Default::default())
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        coin,
        testing::{mock_dependencies, mock_env, mock_info},
        Coin,
    };

    use super::*;

    #[derive(Clone)]
    struct Claim {
        account: String,
        amount: u128,
        claim_proof: ClaimProofOptional,
        merkle_proof: Vec<String>,
    }

    impl Claim {
        fn new(
            account: impl Into<String>,
            amount: u128,
            claim_proof: ClaimProofOptional,
            merkle_proof: Vec<&str>,
        ) -> Self {
            Self {
                account: account.into(),
                amount,
                claim_proof,
                merkle_proof: merkle_proof.into_iter().map(|v| v.to_string()).collect(),
            }
        }

        fn to_payload(&self, airdrop_id: AirdropId) -> ClaimPayload {
            ClaimPayload {
                id: airdrop_id,
                amount: Uint128::from(self.amount),
                claim_proof: self.claim_proof.clone(),
                merkle_proof: self.merkle_proof.clone(),
            }
        }

        fn execute(&self, deps: DepsMut, airdrop_id: AirdropId) -> Result<Response, ContractError> {
            execute_claim(
                deps,
                &self.account,
                airdrop_id,
                self.amount,
                self.claim_proof.clone(),
                self.merkle_proof.clone(),
            )
        }
    }

    const SAMPLE_ROOT_TEST: &str =
        "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";
    const SAMPLE_ROOT_OPEN: &str =
        "87696533e495ec288f64fbcfb5508f14ed33c07c076fe2cd9074456484fe9e5e";
    const SAMPLE_ROOT_BEARER: &str =
        "e05ed933574870cefefffa975dfbad8fc4f0924086f8d6f96c9017a5731bb5fa";

    fn get_open_claims() -> Vec<Claim> {
        vec![
            Claim::new(
                "osmo10yaagy0faggta0085hkzr3ckq7p7z9996nrn0m",
                43904658,
                ClaimProofOptional::account("osmo10yaagy0faggta0085hkzr3ckq7p7z9996nrn0m"),
                vec![
                    "bb416c8705248d135a5c5b1db2d61adf0fcd232b258b57f077fc3e389def4b8d",
                    "a869545b9b418fdb973c0b83903ee99e1b27dc2dce75d7e10d263787ee3c97c1",
                    "d31db3d17297d25ec12e293c46dc54c3069df14e883ec24f13666123c1499cf3",
                    "de5c3d495e15d29a7b8cf3b8fe8a8bc7602fa1e3122debaf1bd01314d81b1dea",
                ],
            ),
            Claim::new(
                "osmo1phaxpevm5wecex2jyaqty2a4v02qj7qmlmzk5a",
                84794294,
                ClaimProofOptional::account("osmo1phaxpevm5wecex2jyaqty2a4v02qj7qmlmzk5a"),
                vec![
                    "02898441407b91279f1fd8de37dd214e970300f1f1040cbb933513dea3b75c15",
                    "7af343b691d61831c7532dccbf7fa476ce3a8269c5c93c834e7404976448869b",
                    "695956534ac375d1039af6583f60120d6d7cdd95c5ea7bd2953b80bb454c336b",
                    "28e923bb17fe7fb93b1bbfe7c1e75927ed39f20d978d62ece0f575e45e66d862",
                ],
            ),
            Claim::new(
                "osmo12smx2wdlyttvyzvzg54y2vnqwq2qjateuf7thj",
                22816641,
                ClaimProofOptional::account("osmo12smx2wdlyttvyzvzg54y2vnqwq2qjateuf7thj"),
                vec![
                    "d96108fe28f021ec3cf173966f4c42b36b405fbb147b060e0e034fdee78aca0d",
                    "e3a3cf4d86e87086372e55c1d25882d20f99e0a6ac1cb87eaa286206b74b953c",
                    "de5c3d495e15d29a7b8cf3b8fe8a8bc7602fa1e3122debaf1bd01314d81b1dea",
                ],
            ),
        ]
    }

    fn get_bearer_claims(sender: &str) -> Vec<Claim> {
        vec![
            Claim::new(
                sender,
                83576099,
                ClaimProofOptional::claim_proof(
                    "4c76049e0d90410060bbebb9562a223c461e04b99d2b5535e6b2aa91edcddee8",
                ),
                vec![
                    "1b290aa1e1b5b0eac2d3581a804cbee984652261dc29a589de09c5938ce15f76",
                    "e67ef76129d46785d89c970c1d92cc55a5541c2268f45ce2caf172163f3391ed",
                    "01d417c4c8e9421aa4c37a4828898fbaeb180cad9026e31b7a21965522d4806f",
                    "7fc962bc95a29db92e61e79a40d1ba27bbd3c8cbcbabc7228e67f38b2f133528",
                ],
            ),
            Claim::new(
                sender,
                14430018,
                ClaimProofOptional::claim_proof(
                    "aeff09ab18c01d444aed2273d1b1825cf5889f8d253df7235eabb1e52717bbe8",
                ),
                vec![
                    "824cc5d487a8306208ce09cb0448b2289803bb0c12a92a958e5eb85e8eb4468d",
                    "404fc9497469dbfbb3021efcdcbb244de21facecf21774a345c01e7e11540d53",
                    "b7446f5ad5a9694a47122a9d8afa9d24187e948ce3a2c4d1011550357d2ab403",
                    "4940ac27869b3c0d8dd1fb598c46882eb716be85acbc264aab9672b576e94f05",
                ],
            ),
            Claim::new(
                sender,
                53405648,
                ClaimProofOptional::claim_proof(
                    "509818a235b8f2463dcefefec5de502f0bd413fa51dbee63f657320d9118ceaa",
                ),
                vec![
                    "7708cf33aabaca0a15702a094b6a7db5339b4079d15d8aef28f582eec97aa2e8",
                    "d257d54ac607bd21844eba08c324d8c42fd382eb9c26294dcfd2aa27ffc68294",
                    "7de49485d71e14879141969366854c0e121fc7655e70a45ceb7e34557c42bab3",
                    "7fc962bc95a29db92e61e79a40d1ba27bbd3c8cbcbabc7228e67f38b2f133528",
                ],
            ),
        ]
    }

    fn make_airdrop(
        merkle_root: impl Into<String>,
        denom: impl Into<String>,
        total_amount: impl Into<Uint128>,
        total_claimed: impl Into<Uint128>,
        bearer: bool,
        label: Option<String>,
    ) -> Airdrop {
        Airdrop {
            merkle_root: merkle_root.into(),
            denom: denom.into(),
            total_amount: total_amount.into(),
            total_claimed: total_claimed.into(),
            bearer,
            label,
        }
    }

    fn execute_register(
        deps: DepsMut,
        sender: &str,
        airdrop: &Airdrop,
    ) -> Result<Response, ContractError> {
        let res = execute(
            deps,
            mock_env(),
            mock_info(sender, &[coin(airdrop.total_amount.u128(), &airdrop.denom)]),
            ExecuteMsg::Register {
                merkle_root: airdrop.merkle_root.clone(),
                denom: airdrop.denom.clone(),
                label: airdrop.label.clone(),
                bearer: Some(airdrop.bearer),
            },
        );

        if let Ok(ref resp) = res {
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
                            .map(|v| format!("{}/{}", sender, v))
                            .unwrap_or_default(),
                    ),
                    attr("bearer", airdrop.bearer.to_string()),
                ]
            );
        }

        res
    }

    fn execute_fund(
        deps: DepsMut,
        sender: &str,
        fund: Coin,
        airdrop_id: AirdropId,
    ) -> Result<Response, ContractError> {
        let conv = match airdrop_id.clone() {
            AirdropId::Id(id) => id,
            AirdropId::Label(label) => LABELS.load(deps.storage, &label).unwrap(),
        }
        .to_string();

        let res = execute(
            deps,
            mock_env(),
            mock_info(sender, &[fund.clone()]),
            ExecuteMsg::Fund { id: airdrop_id },
        );

        if let Ok(ref resp) = res {
            assert_eq!(
                resp.attributes,
                vec![
                    attr("action", "fund"),
                    attr("executor", sender),
                    attr("airdrop_id", conv),
                    attr("amount", fund.amount.to_string()),
                ]
            );
        }

        res
    }

    fn execute_claim(
        deps: DepsMut,
        sender: &str,
        airdrop_id: AirdropId,
        amount: u128,
        claim_proof: ClaimProofOptional,
        merkle_proof: Vec<String>,
    ) -> Result<Response, ContractError> {
        let conv = match airdrop_id.clone() {
            AirdropId::Id(id) => id,
            AirdropId::Label(label) => LABELS.load(deps.storage, &label).unwrap(),
        }
        .to_string();

        let res = execute(
            deps,
            mock_env(),
            mock_info(sender, &[]),
            ExecuteMsg::Claim(ClaimPayload {
                id: airdrop_id,
                amount: Uint128::from(amount),
                claim_proof: claim_proof.clone(),
                merkle_proof,
            }),
        );

        if let Ok(ref resp) = res {
            assert_eq!(
                resp.attributes,
                vec![
                    attr("action", "claim"),
                    attr("executor", sender),
                    attr("airdrop_id", conv),
                    attr(
                        "beneficiary",
                        match claim_proof {
                            ClaimProofOptional::Account(acc) =>
                                acc.unwrap_or_else(|| sender.to_string()),
                            ClaimProofOptional::ClaimProof(_) => sender.to_string(),
                        }
                    ),
                    attr("amount", amount.to_string()),
                ]
            )
        }

        res
    }

    fn execute_multi_claim(
        deps: DepsMut,
        sender: &str,
        payload: Vec<ClaimPayload>,
    ) -> Result<Response, ContractError> {
        let (airdrop_ids, beneficiaries, amounts) = payload.clone().into_iter().fold(
            (vec![], vec![], vec![]),
            |(mut airdrop_ids, mut beneficiaries, mut amounts), v| {
                airdrop_ids.push(match v.id {
                    AirdropId::Id(id) => id,
                    AirdropId::Label(label) => LABELS.load(deps.storage, &label).unwrap(),
                });

                beneficiaries.push(match v.claim_proof {
                    ClaimProofOptional::Account(acc) => acc.unwrap_or_else(|| sender.to_string()),
                    ClaimProofOptional::ClaimProof(_) => sender.to_string(),
                });

                amounts.push(v.amount.to_string());

                (airdrop_ids, beneficiaries, amounts)
            },
        );

        let res = execute(
            deps,
            mock_env(),
            mock_info(sender, &[]),
            ExecuteMsg::MultiClaim(payload),
        );

        if let Ok(ref resp) = res {
            assert_eq!(
                resp.attributes,
                vec![
                    attr("action", "multi_claim"),
                    attr("executor", sender),
                    attr("airdrop_ids", format!("{:?}", airdrop_ids)),
                    attr("beneficiaries", format!("{:?}", beneficiaries)),
                    attr("amounts", format!("{:?}", amounts))
                ]
            )
        }

        res
    }

    #[test]
    fn init() {
        let mut deps = mock_dependencies();

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("owner", &[]),
            InstantiateMsg {},
        )
        .unwrap();

        let airdrop_id = LATEST_AIRDROP_ID.load(deps.as_ref().storage).unwrap();
        assert_eq!(airdrop_id, 0);

        let version = cw2::get_contract_version(deps.as_ref().storage).unwrap();
        assert_eq!(
            version,
            cw2::ContractVersion {
                contract: CONTRACT_NAME.to_string(),
                version: CONTRACT_VERSION.to_string()
            }
        );
    }

    mod execute {

        use super::*;

        fn setup(deps: DepsMut) {
            instantiate(deps, mock_env(), mock_info("owner", &[]), InstantiateMsg {}).unwrap();
        }

        #[test]
        fn register() {
            let mut deps = mock_dependencies();

            setup(deps.as_mut());

            // raw
            let airdrop = make_airdrop(SAMPLE_ROOT_TEST, "uosmo", 1000000u128, 0u128, false, None);
            super::execute_register(deps.as_mut(), "owner", &airdrop).unwrap();

            let latest_airdrop_id = LATEST_AIRDROP_ID.load(deps.as_ref().storage).unwrap();
            assert_eq!(latest_airdrop_id, 1);
            assert_eq!(airdrop, AIRDROPS.load(deps.as_ref().storage, 0).unwrap());

            // with bearer
            let airdrop =
                make_airdrop(SAMPLE_ROOT_TEST, "uatom", 2000000000u128, 0u128, true, None);
            super::execute_register(deps.as_mut(), "owner", &airdrop).unwrap();

            let latest_airdrop_id = LATEST_AIRDROP_ID.load(deps.as_ref().storage).unwrap();
            assert_eq!(latest_airdrop_id, 2);
            assert_eq!(airdrop, AIRDROPS.load(deps.as_ref().storage, 1).unwrap());

            // with label
            let label = "test_label".to_string();
            let mut airdrop = make_airdrop(
                SAMPLE_ROOT_TEST,
                "uatom",
                2000000000u128,
                0u128,
                false,
                Some(label.clone()),
            );
            super::execute_register(deps.as_mut(), "owner", &airdrop).unwrap();
            airdrop.label = Some(format!("{}/{}", "owner", label));

            let latest_airdrop_id = LATEST_AIRDROP_ID.load(deps.as_ref().storage).unwrap();
            assert_eq!(latest_airdrop_id, 3);
            assert_eq!(airdrop, AIRDROPS.load(deps.as_ref().storage, 2).unwrap());

            let airdrop_id_from_label = LABELS
                .load(deps.as_ref().storage, &format!("{}/{}", "owner", label))
                .unwrap();
            assert_eq!(airdrop_id_from_label, 2);

            // check label duplication
            airdrop.label = Some(label.clone());
            let err = super::execute_register(deps.as_mut(), "owner", &airdrop).unwrap_err();
            assert_eq!(
                err,
                ContractError::KeyAlreadyExists {
                    typ: "label".to_string(),
                    key: format!("{}/{}", "owner", label)
                }
            );
        }

        #[test]
        fn fund() {
            let mut deps = mock_dependencies();

            setup(deps.as_mut());

            let label = "test_label".to_string();
            let mut airdrop = make_airdrop(
                SAMPLE_ROOT_TEST,
                "uosmo",
                1000000u128,
                0u128,
                false,
                Some(label.clone()),
            );
            super::execute_register(deps.as_mut(), "owner", &airdrop).unwrap();
            let label = format!("{}/{}", "owner", label);

            let fund_amount = 100000000u128;
            super::execute_fund(
                deps.as_mut(),
                "owner",
                coin(fund_amount, &airdrop.denom),
                AirdropId::Id(0),
            )
            .unwrap();

            airdrop.total_amount += Uint128::from(fund_amount);
            assert_eq!(
                AIRDROPS
                    .load(deps.as_ref().storage, 0)
                    .unwrap()
                    .total_amount,
                airdrop.total_amount
            );

            let fund_amount = 200000000u128;
            super::execute_fund(
                deps.as_mut(),
                "owner",
                coin(fund_amount, &airdrop.denom),
                AirdropId::Label(label),
            )
            .unwrap();

            airdrop.total_amount += Uint128::from(fund_amount);
            assert_eq!(
                AIRDROPS
                    .load(deps.as_ref().storage, 0)
                    .unwrap()
                    .total_amount,
                airdrop.total_amount
            );
        }

        #[test]
        fn claim() {
            let mut deps = mock_dependencies();

            setup(deps.as_mut());

            // open
            let claims = super::get_open_claims();
            let claim_total_amount = claims
                .iter()
                .fold(Uint128::zero(), |acc, c| acc + Uint128::from(c.amount));

            let airdrop = make_airdrop(
                SAMPLE_ROOT_OPEN,
                "uosmo",
                claim_total_amount,
                0u128,
                false,
                None,
            );
            super::execute_register(deps.as_mut(), "owner", &airdrop).unwrap();

            for claim in claims {
                claim.execute(deps.as_mut(), AirdropId::Id(0)).unwrap();
            }
            let fetched_airdrop = AIRDROPS.load(deps.as_ref().storage, 0).unwrap();
            assert_eq!(fetched_airdrop.total_claimed, claim_total_amount);
            assert_eq!(fetched_airdrop.total_amount, fetched_airdrop.total_claimed);

            // bearer
            let claims = super::get_bearer_claims("claimer");
            let claim_total_amount = claims
                .iter()
                .fold(Uint128::zero(), |acc, c| acc + Uint128::from(c.amount));

            let airdrop = make_airdrop(
                SAMPLE_ROOT_BEARER,
                "usomo",
                claim_total_amount,
                0u128,
                true,
                Some("airdrop".to_string()),
            );
            super::execute_register(deps.as_mut(), "owner", &airdrop).unwrap();

            for claim in claims {
                claim
                    .execute(
                        deps.as_mut(),
                        AirdropId::Label(format!("{}/{}", "owner", "airdrop")),
                    )
                    .unwrap();
            }

            let fetched_airdrop = AIRDROPS.load(deps.as_ref().storage, 1).unwrap();
            assert_eq!(fetched_airdrop.total_claimed, claim_total_amount);
            assert_eq!(fetched_airdrop.total_amount, fetched_airdrop.total_claimed);

            // check type of claim_proof
            let claims = super::get_open_claims();
            let claim_total_amount = claims
                .iter()
                .fold(Uint128::zero(), |acc, c| acc + Uint128::from(c.amount));

            let airdrop = make_airdrop(
                SAMPLE_ROOT_OPEN,
                "uosmo",
                claim_total_amount,
                0u128,
                false,
                None,
            );
            super::execute_register(deps.as_mut(), "owner", &airdrop).unwrap();

            claims[0].execute(deps.as_mut(), AirdropId::Id(2)).unwrap();

            let err = Claim::new(
                &claims[0].account,
                1000u128,
                ClaimProofOptional::claim_proof(SAMPLE_ROOT_TEST),
                vec![],
            )
            .execute(deps.as_mut(), AirdropId::Id(2))
            .unwrap_err();
            assert_eq!(
                err,
                ContractError::InvalidArguments {
                    arg: "claim_proof".to_string(),
                    reason: "unexpected proof type".to_string()
                }
            );

            // check double spending
            let err = claims[0]
                .execute(deps.as_mut(), AirdropId::Id(2))
                .unwrap_err();
            assert_eq!(
                err,
                ContractError::AlreadyClaimed {
                    airdrop_id: 2,
                    claimer: Addr::unchecked(&claims[0].account)
                }
            );

            // check merkle proof
            let mut claim_mixed = claims[1].clone();
            claim_mixed.merkle_proof = claims[2].merkle_proof.clone();

            let err = claim_mixed
                .execute(deps.as_mut(), AirdropId::Id(2))
                .unwrap_err();
            assert_eq!(err, ContractError::InvalidProof {});
        }

        #[test]
        fn multi_claim() {
            let mut deps = mock_dependencies();

            setup(deps.as_mut());

            // open
            let claims = super::get_open_claims();
            let claim_total_amount = claims
                .iter()
                .fold(Uint128::zero(), |acc, c| acc + Uint128::from(c.amount));

            let airdrop = make_airdrop(
                SAMPLE_ROOT_OPEN,
                "uosmo",
                claim_total_amount,
                0u128,
                false,
                None,
            );
            super::execute_register(deps.as_mut(), "owner", &airdrop).unwrap();
            super::execute_multi_claim(
                deps.as_mut(),
                "owner",
                claims
                    .iter()
                    .map(|v| v.to_payload(AirdropId::Id(0)))
                    .collect(),
            )
            .unwrap();

            // bearer
            let claims = super::get_bearer_claims("claimer");
            let claim_total_amount = claims
                .iter()
                .fold(Uint128::zero(), |acc, c| acc + Uint128::from(c.amount));

            let airdrop = make_airdrop(
                SAMPLE_ROOT_BEARER,
                "uosmo",
                claim_total_amount,
                0u128,
                true,
                None,
            );
            super::execute_register(deps.as_mut(), "owner", &airdrop).unwrap();
            super::execute_multi_claim(
                deps.as_mut(),
                "owner",
                claims
                    .iter()
                    .map(|v| v.to_payload(AirdropId::Id(1)))
                    .collect(),
            )
            .unwrap();
        }
    }

    mod query {
        use cosmwasm_std::{from_binary, StdError};

        use super::*;

        fn setup(deps: DepsMut) {
            instantiate(deps, mock_env(), mock_info("owner", &[]), InstantiateMsg {}).unwrap();
        }

        #[test]
        fn latest_airdrop_id() {
            let mut deps = mock_dependencies();

            setup(deps.as_mut());

            let resp: LatestAirdropResponse = from_binary(
                &query(deps.as_ref(), mock_env(), QueryMsg::LatestAirdropId {}).unwrap(),
            )
            .unwrap();
            assert_eq!(resp.0, 0);

            let airdrop = make_airdrop(SAMPLE_ROOT_OPEN, "uosmo", 10000u128, 0u128, false, None);
            super::execute_register(deps.as_mut(), "owner", &airdrop).unwrap();

            let resp: LatestAirdropResponse = from_binary(
                &query(deps.as_ref(), mock_env(), QueryMsg::LatestAirdropId {}).unwrap(),
            )
            .unwrap();
            assert_eq!(resp.0, 1);
        }

        #[test]
        fn query_airdrop() {
            let mut deps = mock_dependencies();

            setup(deps.as_mut());

            // check id not found
            let err = query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::GetAirdrop {
                    id: AirdropId::Id(0),
                },
            )
            .unwrap_err();
            assert_eq!(err, ContractError::Std(StdError::not_found("airdrop")));

            // check label not found
            let err = query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::GetAirdrop {
                    id: AirdropId::Label("test_label".to_string()),
                },
            )
            .unwrap_err();
            assert_eq!(err, ContractError::Std(StdError::not_found("label")));

            let airdrop_base =
                make_airdrop(SAMPLE_ROOT_OPEN, "uosmo", 10000u128, 0u128, false, None);

            let mut airdrop_open_labeled = airdrop_base.clone();
            airdrop_open_labeled.label = Some("open_labeled".to_string());

            let mut airdrop_open_unlabeled = airdrop_base.clone();
            airdrop_open_unlabeled.label = None;

            let mut airdrop_bearer_labeled = airdrop_base.clone();
            airdrop_bearer_labeled.bearer = true;
            airdrop_bearer_labeled.label = Some("bearer_labeled".to_string());

            let mut airdrop_bearer_unlabeled = airdrop_base;
            airdrop_bearer_unlabeled.bearer = true;
            airdrop_bearer_unlabeled.label = None;

            let airdrops = [
                airdrop_open_labeled.clone(),
                airdrop_open_unlabeled,
                airdrop_bearer_labeled.clone(),
                airdrop_bearer_unlabeled,
            ];

            airdrops.iter().for_each(|a| {
                super::execute_register(deps.as_mut(), "owner", a).unwrap();
            });

            // get airdrop

            // airdrop_open_unlabeled
            let resp: GetAirdropResponse = from_binary(
                &query(
                    deps.as_ref(),
                    mock_env(),
                    QueryMsg::GetAirdrop {
                        id: AirdropId::Id(1),
                    },
                )
                .unwrap(),
            )
            .unwrap();

            assert_eq!(resp.id, 1);
            assert_eq!(resp.label, None);
            assert_eq!(resp.total_amount, Uint128::from(10000u128));
            assert_eq!(resp.total_claimed, Uint128::zero());
            assert_eq!(resp.denom, "uosmo");
            assert!(!resp.bearer);

            // airdrop_bearer_labeled
            let resp: GetAirdropResponse = from_binary(
                &query(
                    deps.as_ref(),
                    mock_env(),
                    QueryMsg::GetAirdrop {
                        id: AirdropId::Label("owner/bearer_labeled".to_string()),
                    },
                )
                .unwrap(),
            )
            .unwrap();

            assert_eq!(resp.id, 2);
            assert_eq!(resp.label, Some("owner/bearer_labeled".to_string()));
            assert_eq!(resp.total_amount, Uint128::from(10000u128));
            assert_eq!(resp.total_claimed, Uint128::zero());
            assert_eq!(resp.denom, "uosmo");
            assert!(resp.bearer);

            // list airdrop
            let to_resp = |(i, v): (usize, Airdrop)| GetAirdropResponse {
                id: u64::try_from(i).unwrap(),
                label: v.label.map(|v| format!("{}/{}", "owner", v)),
                denom: v.denom,
                total_amount: v.total_amount,
                total_claimed: v.total_claimed,
                bearer: v.bearer,
            };

            let resp: ListAirdropsResponse = from_binary(
                &query(
                    deps.as_ref(),
                    mock_env(),
                    QueryMsg::ListAirdrops {
                        start_after: AirdropIdOptional::Id(None),
                        limit: None,
                        order: None,
                    },
                )
                .unwrap(),
            )
            .unwrap();

            assert_eq!(
                resp.0,
                airdrops
                    .into_iter()
                    .enumerate()
                    .map(to_resp)
                    .collect::<Vec<_>>()
            );

            let resp: ListAirdropsResponse = from_binary(
                &query(
                    deps.as_ref(),
                    mock_env(),
                    QueryMsg::ListAirdrops {
                        start_after: AirdropIdOptional::Label(None),
                        limit: None,
                        order: None,
                    },
                )
                .unwrap(),
            )
            .unwrap();

            assert_eq!(
                resp.0,
                [(2, airdrop_bearer_labeled), (0, airdrop_open_labeled)]
                    .into_iter()
                    .map(to_resp)
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        fn query_claim() {
            let mut deps = mock_dependencies();

            setup(deps.as_mut());

            // register airdrop & claim
            let claims = super::get_open_claims();
            let claim_total_amount = claims
                .iter()
                .fold(Uint128::zero(), |acc, c| acc + Uint128::from(c.amount));

            let airdrop = make_airdrop(
                SAMPLE_ROOT_OPEN,
                "uosmo",
                claim_total_amount,
                0u128,
                false,
                None,
            );
            super::execute_register(deps.as_mut(), "owner", &airdrop).unwrap();

            for claim in claims.clone() {
                claim.execute(deps.as_mut(), AirdropId::Id(0)).unwrap();
            }

            // get claim
            let unwrap_optional = |opt: ClaimProofOptional| match opt {
                ClaimProofOptional::Account(acc) => {
                    ClaimProof::Account(acc.unwrap_or_else(|| claims[0].account.clone()))
                }
                ClaimProofOptional::ClaimProof(proof) => ClaimProof::ClaimProof(proof),
            };

            let resp: GetClaimResponse = from_binary(
                &query(
                    deps.as_ref(),
                    mock_env(),
                    QueryMsg::GetClaim {
                        id: AirdropId::Id(0),
                        claim_proof: unwrap_optional(claims[0].claim_proof.clone()),
                    },
                )
                .unwrap(),
            )
            .unwrap();

            assert_eq!(
                resp.claim_proof,
                unwrap_optional(claims[0].claim_proof.clone())
            );
            assert_eq!(resp.amount, Uint128::from(claims[0].amount));

            // list claim
            let resp: ListClaimsResponse = from_binary(
                &query(
                    deps.as_ref(),
                    mock_env(),
                    QueryMsg::ListClaims {
                        id: AirdropId::Id(0),
                        start_after: None,
                        limit: None,
                        order: None,
                    },
                )
                .unwrap(),
            )
            .unwrap();

            // must sort & compare
            let mut sorted_claims = claims.clone();
            sorted_claims.sort_by(|a, b| a.account.cmp(&b.account));

            assert_eq!(
                resp.0,
                sorted_claims
                    .into_iter()
                    .map(|v| GetClaimResponse {
                        amount: Uint128::from(v.amount),
                        claim_proof: unwrap_optional(v.claim_proof)
                    })
                    .collect::<Vec<_>>()
            )
        }
    }
}
