use crate::error::ContractError;
use bech32::ToBase32;
use cosmwasm_std::{Binary, Uint128};
use ripemd::{Digest, Ripemd160};
use sha2::Sha256;

pub fn sha256_digest(bz: impl AsRef<[u8]>) -> Result<[u8; 32], ContractError> {
    let mut hasher = Sha256::new();

    hasher.update(bz);

    hasher
        .finalize()
        .as_slice()
        .try_into()
        .map_err(|_| ContractError::WrongLength {})
}

pub fn ripemd160_digest(bz: impl AsRef<[u8]>) -> Result<[u8; 20], ContractError> {
    let mut hasher = Ripemd160::new();

    hasher.update(bz);

    hasher
        .finalize()
        .as_slice()
        .try_into()
        .map_err(|_| ContractError::WrongLength {})
}

pub fn pub_to_addr(pub_key: Binary, prefix: &str) -> Result<String, ContractError> {
    let sha_hash = sha256_digest(pub_key)?;
    let rip_hash = ripemd160_digest(sha_hash)?;

    let addr = bech32::encode(prefix, rip_hash.to_base32(), bech32::Variant::Bech32)
        .map_err(|_| ContractError::InvalidPubKey {})?;

    Ok(addr)
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

#[cfg(test)]
mod tests {
    use super::verify_merkle_proof;

    #[test]
    fn test_verify_merkle_proof() {
        verify_merkle_proof(
            "597f35d2e2f4f5c02e31be44695da0c3e0ce03bbb212c6cfc4ef94d7d4940bb5",
            vec![
                "7ea10756e42edf91a6fae6fa8a1acd00751c52c5e0f9d497a7abff7813512667".to_string(),
                "eda896591efa2cd33541930d90ea37449af60460ef8e527109ee9940238266ce".to_string(),
                "b712f5b328047024ff46b9e105ecb71dfcb9813088a87a7e6a46731e7db62638".to_string(),
                "eca3408c50efba13b12ec9b352e0403369ff423ee89f23d1f7ada03a90d7e84f".to_string(),
            ],
            "osmo1phaxpevm5wecex2jyaqty2a4v02qj7qmlmzk5a",
            1000u128.into(),
        )
        .unwrap();

        verify_merkle_proof(
            "597f35d2e2f4f5c02e31be44695da0c3e0ce03bbb212c6cfc4ef94d7d4940bb5",
            vec![
                "7ea10756e42edf91a6fae6fa8a1acd00751c52c5e0f9d497a7abff7813512667".to_string(),
                "eda896591efa2cd33541930d90ea37449af60460ef8e527109ee9940238266ce".to_string(),
                "b712f5b328047024ff46b9e105ecb71dfcb9813088a87a7e6a46731e7db62638".to_string(),
                "eca3408c50efba13b12ec9b352e0403369ff423ee89f23d1f7ada03a90d7e84f".to_string(),
            ],
            "osmo1phaxpevm5wecex2jyaqty2a4v02qj7qmlmzk5a",
            10000u128.into(),
        )
        .unwrap_err();

        verify_merkle_proof(
            "597f35d2e2f4f5c02e31be44695da0c3e0ce03bbb212c6cfc4ef94d7d4940bb5",
            vec![
                "7ea10756e42edf91a6fae6fa8a1acd00751c52c5e0f9d497a7abff7813512667".to_string(),
                "eda896591efa2cd33541930d90ea37449af60460ef8e527109ee9940238266ce".to_string(),
                "b712f5b328047024ff46b9e105ecb71dfcb9813088a87a7e6a46731e7db62638".to_string(),
                "eca3408c50efba13b12ec9b352e0403369ff423ee89f23d1f7ada03a90d7e84f".to_string(),
            ],
            "asdf",
            10000u128.into(),
        )
        .unwrap_err();

        verify_merkle_proof(
            "597f35d2e2f4f5c02e31be44695da0c3e0ce03bbb212c6cfc4ef94d7d4940bb5",
            vec![
                "7ea10756e42edf91a6fae6fa8a1acd00751c52c5e0f9d497a7abff7813512667".to_string(),
                "eda896591efa2cd33541930d90ea37449af60460ef8e527109ee9940238266ce".to_string(),
                "b712f5b328047024ff46b9e105ecb71dfcb9813088a87a7e6a46731e7db62638".to_string(),
            ],
            "osmo1phaxpevm5wecex2jyaqty2a4v02qj7qmlmzk5a",
            10000u128.into(),
        )
        .unwrap_err();
    }
}
