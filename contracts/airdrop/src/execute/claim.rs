use crate::error::ContractError;
use crate::state::{assert_not_claimed, load_airdrop, Airdrop, AIRDROPS, CLAIM_LOGS, LABELS};
use crate::verify::{sha256_digest, verify_bearer_sign, verify_merkle_proof};
use crate::verify_merkle_proof;
use cosmwasm_std::{
    attr, coins, Addr, Api, BankMsg, CosmosMsg, DepsMut, MessageInfo, Response, Storage, Uint128,
};
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

fn assert_airdrop_type(airdrop: &Airdrop, expected: &str) -> Result<(), ContractError> {
    let airdrop_type = airdrop.type_str();

    if airdrop_type != expected {
        return Err(ContractError::InvalidAirdropType {
            expected: expected.to_string(),
            actual: airdrop_type.to_string(),
        });
    }

    Ok(())
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
    if airdrop.closed() {
        return Err(ContractError::AirdropClosed {});
    }

    // pre-validations
    assert_airdrop_type(&airdrop, "open")?;
    assert_not_claimed(deps.storage, airdrop_id, claimer.as_str())?;

    // verify claimer
    verify_merkle_proof(
        airdrop.merkle_root(),
        merkle_proof,
        claimer.as_str(),
        amount,
    )?;

    // claim
    airdrop.total_claimed = airdrop.total_claimed.checked_add(amount)?;
    if airdrop.total_claimed > airdrop.total_amount {
        return Err(ContractError::InsufficientAirdropFunds {});
    }

    // apply to state
    AIRDROPS.save(deps.storage, airdrop_id, &airdrop)?;
    CLAIM_LOGS.save(deps.storage, (airdrop_id, claimer.as_str()), &amount)?;

    // response
    let claim_msg = BankMsg::Send {
        to_address: claimer.to_string(),
        amount: coins(amount.u128(), airdrop.denom()),
    };

    let attrs = vec![
        attr("action", "claim"),
        attr("executor", sender),
        attr("airdrop_type", airdrop.type_str()),
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
    let (airdrop_id, mut airdrop) = load_airdrop(storage, id)?;
    if airdrop.closed() {
        return Err(ContractError::AirdropClosed {});
    }

    // pre-validations
    assert_airdrop_type(&airdrop, "bearer")?;
    assert_not_claimed(storage, airdrop_id, claimer.as_str())?;

    // get address of signer and its public key
    let (signer, signer_pub) = match airdrop {
        Airdrop::Bearer {
            signer, signer_pub, ..
        } => (signer, signer_pub),
        _ => unreachable!("must pass assert_airdrop_type"),
    };

    // verifications
    verify_merkle_proof(airdrop.merkle_root(), merkle_proof, &claim_hash, amount)?;

    verify_bearer_sign(&claim_hash, &claimer, amount, &claim_sign, signer_pub)?;

    // claim
    airdrop.total_claimed = airdrop.total_claimed.checked_add(amount)?;
    if airdrop.total_claimed > airdrop.total_amount {
        return Err(ContractError::InsufficientAirdropFunds {});
    }

    // apply to state
    AIRDROPS.save(deps.storage, airdrop_id, &airdrop)?;
    CLAIM_LOGS.save(deps.storage, (airdrop_id, &claim_hash), &amount)?;

    // response
    let claim_msg = BankMsg::Send {
        to_address: claimer.to_string(),
        amount: coins(amount.u128(), airdrop.denom()),
    };

    let attrs = vec![
        attr("action", "claim"),
        attr("executor", sender),
        attr("airdrop_type", airdrop.type_str()),
        attr("airdrop_id", airdrop_id.to_string()),
        attr("signer", signer.to_string()),
        attr("claimer", claimer),
        attr("amount", amount),
    ];

    Ok(Response::new().add_message(claim_msg).add_attributes(attrs))
}
