use crate::error::ContractError;
use crate::state::{airdrops, assert_not_claimed, load_airdrop, CLAIM_LOGS};
use crate::verify::{sha256_digest, verify_merkle_proof};
use cosmwasm_std::{
    attr, coins, Addr, Attribute, BankMsg, DepsMut, MessageInfo, Response, Uint128,
};
use ibcx_interface::airdrop::{AirdropId, AirdropType, ClaimPayload};

pub fn claim(
    deps: DepsMut,
    info: MessageInfo,
    payload: ClaimPayload,
) -> Result<Response, ContractError> {
    match payload {
        ClaimPayload::Open {
            airdrop: id,
            amount,
            account,
            merkle_proof: proof,
        } => {
            // use tx sender if account is not provided
            let claimer = account
                .map(|x| deps.api.addr_validate(&x))
                .transpose()?
                .unwrap_or_else(|| info.sender.clone());

            claim_open(deps, info.sender, id, amount, claimer, proof)
        }

        ClaimPayload::Bearer {
            airdrop: id,
            amount,
            account,
            claim_hash: hash,
            claim_sign: sign,
            merkle_proof: proof,
        } => {
            // use tx sender if account is not provided
            let claimer = account
                .map(|x| deps.api.addr_validate(&x))
                .transpose()?
                .unwrap_or_else(|| info.sender.clone());

            claim_bearer(deps, info.sender, id, amount, claimer, hash, sign, proof)
        }
    }
}

fn claim_open_event(
    sender: Addr,
    typ: AirdropType,
    id: u64,
    claimer: Addr,
    amount: impl Into<u128>,
) -> Vec<Attribute> {
    vec![
        attr("action", "claim"),
        attr("executor", sender),
        attr("airdrop_type", typ),
        attr("airdrop_id", id.to_string()),
        attr("claimer", claimer),
        attr("amount", amount.into().to_string()),
    ]
}

fn claim_open(
    deps: DepsMut,
    sender: Addr,
    id: AirdropId,
    amount: Uint128,
    claimer: Addr,
    merkle_proof: Vec<String>,
) -> Result<Response, ContractError> {
    let (airdrop_id, airdrop) = load_airdrop(deps.storage, id)?;

    // pre-validations
    let mut airdrop = airdrop.unwrap_open()?;
    if airdrop.closed_at.is_some() {
        return Err(ContractError::AirdropClosed {});
    }

    assert_not_claimed(deps.storage, airdrop_id, claimer.as_str())?;

    // verify claimer
    verify_merkle_proof(&airdrop.merkle_root, merkle_proof, claimer.as_str(), amount)?;

    // claim
    airdrop.total_claimed = airdrop.total_claimed.checked_add(amount)?;
    if airdrop.total_claimed > airdrop.total_amount {
        return Err(ContractError::InsufficientAirdropFunds {});
    }

    // response
    let claim_msg = BankMsg::Send {
        to_address: claimer.to_string(),
        amount: coins(amount.u128(), &airdrop.denom),
    };

    let attrs = claim_open_event(
        sender,
        AirdropType::Open,
        airdrop_id,
        claimer.clone(),
        amount,
    );

    // apply to state
    airdrops().save(deps.storage, airdrop_id, &airdrop.into())?;
    CLAIM_LOGS.save(deps.storage, (airdrop_id, claimer.as_str()), &amount)?;

    Ok(Response::new().add_message(claim_msg).add_attributes(attrs))
}

fn claim_bearer_event(
    sender: Addr,
    typ: AirdropType,
    id: u64,
    signer: Addr,
    claimer: Addr,
    amount: impl Into<u128>,
) -> Vec<Attribute> {
    vec![
        attr("action", "claim"),
        attr("executor", sender),
        attr("airdrop_type", typ),
        attr("airdrop_id", id.to_string()),
        attr("signer", signer),
        attr("claimer", claimer),
        attr("amount", amount.into().to_string()),
    ]
}

#[allow(clippy::too_many_arguments)]
fn claim_bearer(
    deps: DepsMut,
    sender: Addr,
    id: AirdropId,
    amount: Uint128,
    claimer: Addr,
    claim_hash: String,
    claim_sign: String,
    merkle_proof: Vec<String>,
) -> Result<Response, ContractError> {
    let (airdrop_id, airdrop) = load_airdrop(deps.storage, id)?;

    // pre-validations
    let mut airdrop = airdrop.unwrap_bearer()?;
    if airdrop.closed_at.is_some() {
        return Err(ContractError::AirdropClosed {});
    }
    assert_not_claimed(deps.storage, airdrop_id, claim_hash.as_str())?;

    // verifications
    verify_merkle_proof(&airdrop.merkle_root, merkle_proof, &claim_hash, amount)?;

    let digest_str = format!("{claim_hash}/{claimer}/{amount}");
    let digest = sha256_digest(digest_str.as_bytes())?;

    let ok = deps
        .api
        .secp256k1_verify(&digest, &hex::decode(claim_sign)?, &airdrop.signer_pub)?;
    if !ok {
        return Err(ContractError::invalid_signature("claim_bearer"));
    }

    // claim
    airdrop.total_claimed = airdrop.total_claimed.checked_add(amount)?;
    if airdrop.total_claimed > airdrop.total_amount {
        return Err(ContractError::InsufficientAirdropFunds {});
    }

    // response
    let claim_msg = BankMsg::Send {
        to_address: claimer.to_string(),
        amount: coins(amount.u128(), &airdrop.denom),
    };

    let attrs = claim_bearer_event(
        sender,
        AirdropType::Bearer,
        airdrop_id,
        airdrop.signer.clone(),
        claimer,
        amount,
    );

    // apply to state
    airdrops().save(deps.storage, airdrop_id, &airdrop.into())?;
    CLAIM_LOGS.save(deps.storage, (airdrop_id, &claim_hash), &amount)?;

    Ok(Response::new().add_message(claim_msg).add_attributes(attrs))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{
            mock_dependencies_with_balances, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
        },
        Addr, OwnedDeps,
    };
    use ibcx_interface::airdrop::{AirdropType, ClaimPayload, InstantiateMsg};

    use crate::{
        contract::instantiate,
        error::ContractError,
        execute::{
            claim::claim_bearer_event,
            tests::{mock_bearer_airdrop, mock_open_airdrop, register_airdrop, Balances},
        },
    };

    use super::{claim, claim_open_event};

    fn setup(airdrop_type: AirdropType) -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
        let env = mock_env();

        let initial_balances: Balances = &[];

        let mut deps = mock_dependencies_with_balances(initial_balances);

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("creator", &[]),
            InstantiateMsg {},
        )
        .unwrap();

        match airdrop_type {
            AirdropType::Open => {
                let mock_airdrop = mock_open_airdrop(None, env.block.height);
                register_airdrop(deps.as_mut(), env, mock_airdrop.into(), None);
            }
            AirdropType::Bearer => {
                let (mock_airdrop, mock_airdrop_sign) = mock_bearer_airdrop(None, env.block.height);
                register_airdrop(
                    deps.as_mut(),
                    env,
                    mock_airdrop.into(),
                    Some(mock_airdrop_sign),
                );
            }
        }

        deps
    }

    #[test]
    fn test_claim_open() {
        let mut deps = setup(AirdropType::Open);

        let airdrop_claim_amount = 1000u128;
        let airdrop_sender_ok = Addr::unchecked("osmo1phaxpevm5wecex2jyaqty2a4v02qj7qmlmzk5a");
        let airdrop_sender_no = Addr::unchecked("osmo1z7huy904a3yf3aj8mxt5z6shy7dezrlw5gduju");

        let merkle_proof_valid: &[&str] = &[
            "7ea10756e42edf91a6fae6fa8a1acd00751c52c5e0f9d497a7abff7813512667",
            "eda896591efa2cd33541930d90ea37449af60460ef8e527109ee9940238266ce",
            "b712f5b328047024ff46b9e105ecb71dfcb9813088a87a7e6a46731e7db62638",
            "eca3408c50efba13b12ec9b352e0403369ff423ee89f23d1f7ada03a90d7e84f",
        ];
        let merkle_proof_invalid: &[&str] = &[
            "7ea10756e42edf91a6fae6fa8a1acd00751c52c5e0f9d497a7abff7813512667",
            "eda896591efa2cd33541930d90ea37449af60460ef8e527109ee9940238266ce",
            "eca3408c50efba13b12ec9b352e0403369ff423ee89f23d1f7ada03a90d7e84f",
            "b712f5b328047024ff46b9e105ecb71dfcb9813088a87a7e6a46731e7db62638",
        ];

        let cases = [
            // key = ok, amount = ok, proof = no
            (
                0u64,
                airdrop_sender_ok.as_str(),
                airdrop_claim_amount,
                merkle_proof_invalid,
                Some(ContractError::InvalidProof {}),
            ),
            // key = ok, amount = no, proof = ok
            (
                0u64,
                airdrop_sender_ok.as_str(),
                airdrop_claim_amount - 1,
                merkle_proof_valid,
                Some(ContractError::InvalidProof {}),
            ),
            // key = no, amount = ok, proof = ok
            (
                0u64,
                airdrop_sender_no.as_str(),
                airdrop_claim_amount,
                merkle_proof_valid,
                Some(ContractError::InvalidProof {}),
            ),
            // key = ok, amount = ok, proof = ok
            (
                0u64,
                airdrop_sender_ok.as_str(),
                airdrop_claim_amount,
                merkle_proof_valid,
                None,
            ),
            // key = ok, amount = ok, proof = ok ----- already claimed
            (
                0u64,
                airdrop_sender_ok.as_str(),
                airdrop_claim_amount,
                merkle_proof_valid,
                Some(ContractError::AlreadyClaimed {
                    airdrop_id: 0,
                    claim_key: airdrop_sender_ok.to_string(),
                }),
            ),
        ];
        for (id, key, amount, proof, expect_err) in cases {
            let resp = claim(
                deps.as_mut(),
                mock_info("anyone", &[]),
                ClaimPayload::open_id(id, amount, Some(key), proof),
            );
            match expect_err {
                Some(err) => assert_eq!(resp.unwrap_err(), err),
                None => assert_eq!(
                    resp.unwrap().attributes,
                    claim_open_event(
                        Addr::unchecked("anyone"),
                        AirdropType::Open,
                        id,
                        Addr::unchecked(key),
                        amount,
                    )
                ),
            }
        }
    }

    #[test]
    fn test_claim_bearer() {
        let mut deps = setup(AirdropType::Bearer);

        let airdrop_claim_amount = 10000u128;
        let airdrop_signer = Addr::unchecked("osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks");
        let airdrop_sender = Addr::unchecked("osmo1phaxpevm5wecex2jyaqty2a4v02qj7qmlmzk5a");

        let claim_hash_valid = "c2ae4a162c574a85c596cfda7c057c5aa15c7b4d6a5679f44fb365d76e63e24e";
        let claim_hash_invalid = "ba2b4d1a5891634c11f53917c7683bd9aff9b3c3595d93a88c0169bb201606c8";

        // signed by osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks
        let claim_sign_valid = "418e71e063fa630855ca431a6cf1b3ba7d7ef88364f5e24de6da1068071521542cdb061b66ec8a75c534720354f2347d64e3e0e75ca24edb0766f7b9336a3694";
        // signed by osmo1phaxpevm5wecex2jyaqty2a4v02qj7qmlmzk5a
        let claim_sign_invalid = "8037897fdde7d83152b4586440b16a9e36ae6fa07a7574c347282c204a8e147967721ecfcc671d27cfd5545420cfc17a3ee8f5563ceef82f8e0209e7a751ba9f";

        let merkle_proof_valid: &[&str] = &[
            "7673db0bbf54220efc87bf8f57b5bc0fa489fe4508fa6051ed938ccbb11f5e7b",
            "f4e567b6b15b97e312757c1a2bd5273a14f4300b4e5d3df8980f7a3d400e90ce",
            "3591dea5b253198a4ba4bbbc440a1fb955967a11cce4abe442f8867c224bff19",
            "2a301d9535b68c35e0ae38108085966fb8657b396c92df1b86d7e07c183436a6",
        ];
        let merkle_proof_invalid: &[&str] = &[
            "7673db0bbf54220efc87bf8f57b5bc0fa489fe4508fa6051ed938ccbb11f5e7b",
            "f4e567b6b15b97e312757c1a2bd5273a14f4300b4e5d3df8980f7a3d400e90ce",
            "2a301d9535b68c35e0ae38108085966fb8657b396c92df1b86d7e07c183436a6",
            "3591dea5b253198a4ba4bbbc440a1fb955967a11cce4abe442f8867c224bff19",
        ];

        let cases = [
            // amount = ok, sender = ok, hash = ok, sign = ok, proof = no
            (
                0u64,
                airdrop_claim_amount,
                airdrop_sender.as_str(),
                claim_hash_valid,
                claim_sign_valid,
                merkle_proof_invalid,
                Some(ContractError::InvalidProof {}),
            ),
            // amount = ok, sender = ok, hash = ok, sign = no, proof = ok
            (
                0u64,
                airdrop_claim_amount,
                airdrop_sender.as_str(),
                claim_hash_valid,
                claim_sign_invalid,
                merkle_proof_valid,
                Some(ContractError::invalid_signature("claim_bearer")),
            ),
            // amount = ok, sender = ok, hash = no, sign = ok, proof = ok
            (
                0u64,
                airdrop_claim_amount,
                airdrop_sender.as_str(),
                claim_hash_invalid,
                claim_sign_valid,
                merkle_proof_valid,
                Some(ContractError::InvalidProof {}),
            ),
            // amount = ok, sender = no, hash = ok, sign = ok, proof = ok
            (
                0u64,
                airdrop_claim_amount,
                airdrop_signer.as_str(),
                claim_hash_valid,
                claim_sign_valid,
                merkle_proof_valid,
                Some(ContractError::invalid_signature("claim_bearer")),
            ),
            // amount = no, sender = ok, hash = ok, sign = ok, proof = ok
            (
                0u64,
                airdrop_claim_amount - 1,
                airdrop_sender.as_str(),
                claim_hash_valid,
                claim_sign_valid,
                merkle_proof_valid,
                Some(ContractError::InvalidProof {}),
            ),
            // amount = ok, hash = ok, sign = ok, proof = ok
            (
                0u64,
                airdrop_claim_amount,
                airdrop_sender.as_str(),
                claim_hash_valid,
                claim_sign_valid,
                merkle_proof_valid,
                None,
            ),
            // amount = ok, hash = ok, sign = ok, proof = ok ---- already claimed
            (
                0u64,
                airdrop_claim_amount,
                airdrop_sender.as_str(),
                claim_hash_valid,
                claim_sign_valid,
                merkle_proof_valid,
                Some(ContractError::AlreadyClaimed {
                    airdrop_id: 0,
                    claim_key: claim_hash_valid.to_string(),
                }),
            ),
        ];

        for (id, amount, claimer, hash, sign, proof, expect_err) in cases {
            let resp = claim(
                deps.as_mut(),
                mock_info("anyone", &[]),
                ClaimPayload::bearer_id(id, amount, Some(claimer), hash, sign, proof),
            );

            match expect_err {
                Some(err) => assert_eq!(resp.unwrap_err(), err),
                None => assert_eq!(
                    resp.unwrap().attributes,
                    claim_bearer_event(
                        Addr::unchecked("anyone"),
                        AirdropType::Bearer,
                        0,
                        airdrop_signer.clone(),
                        Addr::unchecked(claimer),
                        amount
                    )
                ),
            }
        }
    }
}
