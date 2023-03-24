use crate::error::ContractError;
use cosmwasm_std::{Addr, Binary, Uint128};
use sha2::digest::generic_array::GenericArray;

pub fn sha256_digest(bz: impl AsRef<[u8]>) -> Result<[u8], ContractError> {
    sha2::Sha256::digest(bz)
        .as_slice()
        .try_into()
        .map_err(|_| ContractError::WrongLength {})
}

pub fn verify_bearer_sign(
    hash: &str,
    addr: &Addr,
    amount: Uint128,
    sign: &str,
    pubkey: Binary,
) -> Result<(), ContractError> {
    let verified = deps.api.secp256k1_verify(
        &sha256_digest(format!("{hash}/{addr}/{amount}").as_bytes())?,
        &hex::decode(sign)?,
        &pubkey,
    )?;
    if !verified {
        return Err(ContractError::InvalidClaimSignature {});
    }

    Ok(())
}

// verify merkle proof (from https://github.com/cosmwasm/cw-tokens/blob/master/contracts/cw20-merkle-airdrop/src/contract.rs)
pub fn verify_merkle_proof(
    root: &str,
    proof: Vec<String>,
    claim_proof: &str,
    amount: Uint128,
) -> Result<(), ContractError> {
    let user_input = format!("{claim_proof}:{amount}");

    let hash = sha256_digest(user_input.as_bytes())?;

    let hash = proof.into_iter().try_fold(hash, |hash, p| {
        let mut proof_buf = [0; 32];
        hex::decode_to_slice(p, &mut proof_buf)?;
        let mut hashes = [hash, proof_buf];
        hashes.sort_unstable();
        sha256_digest(hashes.concat())
    })?;

    let mut root_buf: [u8; 32] = [0; 32];
    hex::decode_to_slice(root, &mut root_buf)?;
    if root_buf != hash {
        return Err(ContractError::InvalidProof {});
    }

    Ok(())
}
