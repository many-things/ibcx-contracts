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
