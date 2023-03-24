use crate::error::ContractError;
use crate::state::{assert_not_claimed, load_airdrop, AIRDROPS, CLAIM_LOGS};
use crate::verify::{sha256_digest, verify_merkle_proof};
use cosmwasm_std::{attr, coins, Addr, BankMsg, DepsMut, MessageInfo, Response, Uint128};
use ibcx_interface::airdrop::{AirdropId, ClaimPayload};

pub fn claim(
    deps: DepsMut,
    info: MessageInfo,
    payload: ClaimPayload,
) -> Result<Response, ContractError> {
    let sender = &info.sender;

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
                .unwrap_or(sender.clone());

            claim_open(&deps, sender, id, amount, claimer, proof)
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
                .unwrap_or(sender.clone());

            claim_bearer(&deps, sender, id, amount, claimer, hash, sign, proof)
        }
    }
}

fn claim_open(
    deps: &DepsMut,
    sender: &Addr,
    id: AirdropId,
    amount: Uint128,
    claimer: Addr,
    merkle_proof: Vec<String>,
) -> Result<Response, ContractError> {
    let (airdrop_id, mut airdrop) = load_airdrop(deps.storage, id)?;

    // pre-validations
    let airdrop = airdrop.unwrap_open()?;
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

    // apply to state
    AIRDROPS.save(deps.storage, airdrop_id, &airdrop.wrap())?;
    CLAIM_LOGS.save(deps.storage, (airdrop_id, claimer.as_str()), &amount)?;

    // response
    let claim_msg = BankMsg::Send {
        to_address: claimer.to_string(),
        amount: coins(amount.u128(), airdrop.denom),
    };

    let attrs = vec![
        attr("action", "claim"),
        attr("executor", sender),
        attr("airdrop_type", airdrop.wrap().type_str()),
        attr("airdrop_id", airdrop_id.to_string()),
        attr("claimer", claimer),
        attr("amount", amount),
    ];

    Ok(Response::new().add_message(claim_msg).add_attributes(attrs))
}

fn claim_bearer(
    deps: &DepsMut,
    sender: &Addr,
    id: AirdropId,
    amount: Uint128,
    claimer: Addr,
    claim_hash: String,
    claim_sign: String,
    merkle_proof: Vec<String>,
) -> Result<Response, ContractError> {
    let (airdrop_id, mut airdrop) = load_airdrop(deps.storage, id)?;

    // pre-validations
    let airdrop = airdrop.unwrap_bearer()?;
    if airdrop.closed_at.is_some() {
        return Err(ContractError::AirdropClosed {});
    }
    assert_not_claimed(deps.storage, airdrop_id, claimer.as_str())?;

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

    // apply to state
    AIRDROPS.save(deps.storage, airdrop_id, &airdrop.wrap())?;
    CLAIM_LOGS.save(deps.storage, (airdrop_id, &claim_hash), &amount)?;

    // response
    let claim_msg = BankMsg::Send {
        to_address: claimer.to_string(),
        amount: coins(amount.u128(), airdrop.denom),
    };

    let attrs = vec![
        attr("action", "claim"),
        attr("executor", sender),
        attr("airdrop_type", airdrop.wrap().type_str()),
        attr("airdrop_id", airdrop_id.to_string()),
        attr("signer", airdrop.signer),
        attr("claimer", claimer),
        attr("amount", amount),
    ];

    Ok(Response::new().add_message(claim_msg).add_attributes(attrs))
}
