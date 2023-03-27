use cosmwasm_std::{Deps, Order, StdResult};
use cw_storage_plus::Bound;
use ibcx_interface::{
    airdrop::{AirdropId, ClaimPayload, GetClaimResponse, ListClaimsResponse, VerifyClaimResponse},
    get_and_check_limit,
    types::RangeOrder,
    DEFAULT_LIMIT, MAX_LIMIT,
};

use crate::{
    error::ContractError,
    state::{load_airdrop, CLAIM_LOGS},
    verify::{sha256_digest, verify_merkle_proof},
};

pub fn get_claim(
    deps: Deps,
    id: AirdropId,
    claim_key: String,
) -> Result<GetClaimResponse, ContractError> {
    let (airdrop_id, _) = load_airdrop(deps.storage, id)?;

    let claim = CLAIM_LOGS.load(deps.storage, (airdrop_id, &claim_key))?;

    Ok(GetClaimResponse {
        id: airdrop_id,
        amount: claim,
        claim_key,
    })
}

pub fn list_claims(
    deps: Deps,
    id: AirdropId,
    start_after: Option<String>,
    limit: Option<u32>,
    order: Option<RangeOrder>,
) -> Result<ListClaimsResponse, ContractError> {
    let (airdrop_id, _) = load_airdrop(deps.storage, id)?;

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

            Ok(GetClaimResponse {
                id: airdrop_id,
                amount: v,
                claim_key: k,
            })
        })
        .collect::<StdResult<_>>()?;

    Ok(ListClaimsResponse(resps))
}

pub fn verify_claim(
    deps: Deps,
    payload: ClaimPayload,
) -> Result<VerifyClaimResponse, ContractError> {
    let resp = VerifyClaimResponse::default();

    match payload {
        ClaimPayload::Open {
            airdrop: id,
            amount,
            account,
            merkle_proof: proof,
        } => {
            let (_, airdrop) = load_airdrop(deps.storage, id)?;

            let airdrop = airdrop.unwrap_open()?;
            if airdrop.closed_at.is_some() {
                return Ok(resp.fail("airdrop is closed"));
            }

            let verify_result =
                verify_merkle_proof(&airdrop.merkle_root, proof, &account.unwrap(), amount);
            if let Err(e) = verify_result {
                return Ok(resp.fail(e));
            }
        }

        ClaimPayload::Bearer {
            airdrop: id,
            amount,
            account,
            claim_hash,
            claim_sign,
            merkle_proof: proof,
        } => {
            let account = account.unwrap();
            let (_, airdrop) = load_airdrop(deps.storage, id)?;

            let airdrop = airdrop.unwrap_bearer()?;
            if airdrop.closed_at.is_some() {
                return Ok(resp.fail("airdrop is closed"));
            }

            // validate claim hash
            let verify_result = verify_merkle_proof(&airdrop.merkle_root, proof, &account, amount);
            if let Err(e) = verify_result {
                return Ok(resp.fail(e));
            }

            // validate claimer
            let digest_str = format!("{claim_hash}/{}/{amount}", account);
            let digest = sha256_digest(digest_str.as_bytes())?;

            let verify_result = deps.api.secp256k1_verify(
                &digest,
                &hex::decode(claim_sign)?,
                &airdrop.signer_pub,
            )?;
            if !verify_result {
                return Ok(resp.fail("invalid claim signature"));
            }
        }
    }

    Ok(resp.ok())
}
