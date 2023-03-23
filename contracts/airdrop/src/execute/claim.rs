use crate::error::ContractError;
use crate::state::{Airdrop, AIRDROPS, CLAIM_LOGS, LABELS};
use crate::verify_merkle_proof;
use cosmwasm_std::{
    attr, coins, Addr, Api, BankMsg, CosmosMsg, DepsMut, MessageInfo, Response, Storage, Uint128,
};
use ibcx_interface::airdrop::{AirdropId, ClaimPayload};

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
    payload: ClaimPayload,
) -> Result<Response, ContractError> {
    // let ((airdrop_id, airdrop), beneficiary, amount) = _claim(
    //     deps.api,
    //     deps.storage,
    //     &info.sender,
    //     id,
    //     amount,
    //     claim_proof,
    //     merkle_proof,
    // )?;

    match payload {
        ClaimPayload::Open {
            airdrop,
            amount,
            account,
            merkle_proof,
        } => claim_open(deps, info, airdrop, amount, account, merkle_proof),
        ClaimPayload::Bearer {
            airdrop,
            amount,
            claim_hash,
            claim_sign,
            merkle_proof,
        } => claim_bearer(
            deps,
            info,
            airdrop,
            amount,
            claim_hash,
            claim_sign,
            merkle_proof,
        ),
    }
}

fn claim_open(
    deps: DepsMut,
    info: MessageInfo,
    airdrop: AirdropId,
    amount: Uint128,
    account: Option<String>,
    merkle_proof: Vec<String>,
) -> Result<Response, ContractError> {
    panic!("not implemented");

    // Ok(Response::new()
    //     .add_message(BankMsg::Send {
    //         to_address: beneficiary.to_string(),
    //         amount: coins(amount.u128(), airdrop.denom),
    //     })
    //     .add_attributes(vec![
    //         attr("action", "claim"),
    //         attr("executor", info.sender),
    //         attr("airdrop_id", airdrop_id.to_string()),
    //         attr("beneficiary", beneficiary),
    //         attr("amount", amount),
    //     ]))
}

fn claim_bearer(
    deps: DepsMut,
    info: MessageInfo,
    airdrop: AirdropId,
    amount: Uint128,
    claim_hash: String,
    claim_sign: String,
    merkle_proof: Vec<String>,
) -> Result<Response, ContractError> {
    panic!("not implemented");

    // Ok(Response::new()
    //     .add_message(BankMsg::Send {
    //         to_address: beneficiary.to_string(),
    //         amount: coins(amount.u128(), airdrop.denom),
    //     })
    //     .add_attributes(vec![
    //         attr("action", "claim"),
    //         attr("executor", info.sender),
    //         attr("airdrop_id", airdrop_id.to_string()),
    //         attr("beneficiary", beneficiary),
    //         attr("amount", amount),
    //     ]))
}

pub fn claim_many(
    deps: DepsMut,
    info: MessageInfo,
    payload: Vec<ClaimPayload>,
) -> Result<Response, ContractError> {
    Ok(Response::default())
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
