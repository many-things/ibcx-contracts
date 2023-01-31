use cosmwasm_std::Uint128;
use error::ContractError;
use sha2::Digest;

pub mod contract;
pub mod error;
pub mod execute;
pub mod query;
pub mod state;

// verify merkle proof (from https://github.com/cosmwasm/cw-tokens/blob/master/contracts/cw20-merkle-airdrop/src/contract.rs)
pub fn verify_merkle_proof(
    root: &str,
    proof: Vec<String>,
    claim_proof: &str,
    amount: Uint128,
) -> Result<(), ContractError> {
    let user_input = format!("{claim_proof}{amount}");

    let hash = sha2::Sha256::digest(user_input.as_bytes())
        .as_slice()
        .try_into()
        .map_err(|_| ContractError::WrongLength {})?;

    let hash = proof.into_iter().try_fold(hash, |hash, p| {
        let mut proof_buf = [0; 32];
        hex::decode_to_slice(p, &mut proof_buf)?;
        let mut hashes = [hash, proof_buf];
        hashes.sort_unstable();
        sha2::Sha256::digest(hashes.concat())
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

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod test {
    use cosmwasm_std::{testing::mock_info, Addr, DepsMut, Response, Uint128};
    use ibcx_interface::airdrop::{AirdropId, ClaimPayload, ClaimProofOptional};

    use crate::{error::ContractError, execute, state::Airdrop};

    pub const SENDER_OWNER: &str = "owner";

    pub const SAMPLE_ROOT_TEST: &str =
        "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";
    pub const SAMPLE_ROOT_OPEN: &str =
        "87696533e495ec288f64fbcfb5508f14ed33c07c076fe2cd9074456484fe9e5e";
    pub const SAMPLE_ROOT_BEARER: &str =
        "e05ed933574870cefefffa975dfbad8fc4f0924086f8d6f96c9017a5731bb5fa";

    pub fn make_airdrop(
        creator: impl Into<String>,
        merkle_root: impl Into<String>,
        denom: impl Into<String>,
        total_amount: impl Into<Uint128>,
        total_claimed: impl Into<Uint128>,
        bearer: bool,
        label: Option<String>,
    ) -> Airdrop {
        Airdrop {
            creator: Addr::unchecked(creator),
            merkle_root: merkle_root.into(),
            denom: denom.into(),
            total_amount: total_amount.into(),
            total_claimed: total_claimed.into(),
            bearer,
            label,
            closed: false,
        }
    }

    #[derive(Clone)]
    pub struct Claim {
        pub account: String,
        pub amount: u128,
        pub claim_proof: ClaimProofOptional,
        pub merkle_proof: Vec<String>,
    }

    impl Claim {
        pub fn new(
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

        pub fn to_payload(&self, airdrop_id: AirdropId) -> ClaimPayload {
            ClaimPayload {
                id: airdrop_id,
                amount: Uint128::from(self.amount),
                claim_proof: self.claim_proof.clone(),
                merkle_proof: self.merkle_proof.clone(),
            }
        }

        pub fn execute(
            &self,
            deps: DepsMut,
            airdrop_id: AirdropId,
        ) -> Result<Response, ContractError> {
            execute::claim(
                deps,
                mock_info(&self.account, &[]),
                self.to_payload(airdrop_id),
            )
        }
    }

    pub fn get_open_claims() -> Vec<Claim> {
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

    pub fn get_bearer_claims(sender: &str) -> Vec<Claim> {
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
}
