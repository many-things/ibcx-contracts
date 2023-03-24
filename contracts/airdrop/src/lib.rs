use cosmwasm_std::Uint128;
use error::ContractError;
use sha2::Digest;

pub mod contract;
pub mod error;
pub mod execute;
pub mod query;
pub mod state;
pub mod verify;

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
        "597f35d2e2f4f5c02e31be44695da0c3e0ce03bbb212c6cfc4ef94d7d4940bb5";
    pub const SAMPLE_ROOT_BEARER: &str =
        "36aa7eff8f432a03572d519e71200f3cd20168bb4d3e678b27ca984a30feef81";

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
                "osmo1phaxpevm5wecex2jyaqty2a4v02qj7qmlmzk5a",
                1000,
                ClaimProofOptional::account("osmo1phaxpevm5wecex2jyaqty2a4v02qj7qmlmzk5a"),
                vec![
                    "7ea10756e42edf91a6fae6fa8a1acd00751c52c5e0f9d497a7abff7813512667",
                    "eda896591efa2cd33541930d90ea37449af60460ef8e527109ee9940238266ce",
                    "b712f5b328047024ff46b9e105ecb71dfcb9813088a87a7e6a46731e7db62638",
                    "eca3408c50efba13b12ec9b352e0403369ff423ee89f23d1f7ada03a90d7e84f",
                ],
            ),
            Claim::new(
                "osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks",
                1000,
                ClaimProofOptional::account("osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks"),
                vec![
                    "2a466d1b5ce03857a1c908737daed70c67a0b4a8883efc8afedb16d7f76c340f",
                    "bd8ca611132cc19133ee408646ec08f2e104b6bb670bce0c3070fa2f086120d8",
                    "7821cdd6c9c94cc63b7c6131711a9deebc02c71dd075ab555ab48d19fddd7aa9",
                    "eca3408c50efba13b12ec9b352e0403369ff423ee89f23d1f7ada03a90d7e84f",
                ],
            ),
            Claim::new(
                "osmo18s5lynnmx37hq4wlrw9gdn68sg2uxp5rgk26vv",
                1000,
                ClaimProofOptional::account("osmo18s5lynnmx37hq4wlrw9gdn68sg2uxp5rgk26vv"),
                vec!["c3f89415e48599f708ba5767dda0b20d52a7a35c85c239b0f17fc59431c796ad"],
            ),
        ]
    }

    pub fn get_bearer_claims(sender: &str) -> Vec<Claim> {
        vec![
            Claim::new(
                sender,
                10000,
                ClaimProofOptional::claim_proof(
                    "af3bbee2ddde2fa3e1e66b40036ece0aaccaafbe9801d7d20a25c2f28491ea32",
                ),
                vec![
                    "9fccf099a14171ad087464325aa6be356fc71d83743be8b8505894aa347fd3d3",
                    "80005d069b219d7e23920de9f584c6c7b16a45e067e4a1294eb7c682c2bec429",
                    "860643b0a8b8932a68738b5a18bc1b243e030a3b10c20779bbbf620d1ff39cf3",
                    "0a794b9b5e9d8c1db20800184764ec58bbea6d8ec43fbc41d30779e5a4592997",
                ],
            ),
            Claim::new(
                sender,
                10000,
                ClaimProofOptional::claim_proof(
                    "668857caefd64b5e716e7b99c625119aae9bcfed76dae38da983586148c584b8",
                ),
                vec![
                    "9ff01de88ca0ea4a38873c8ea9cfc09d41bac627ae01f9272e8c29c97ac742dd",
                    "d375f310c6a3dcfb5eceb51bf26bba3d8a2459d29b91583a70bf5469a77aa8f5",
                    "090d1756337269d841a4f7488aa37eef7840373eea516fecfcca430d04461fb5",
                    "0a794b9b5e9d8c1db20800184764ec58bbea6d8ec43fbc41d30779e5a4592997",
                ],
            ),
            Claim::new(
                sender,
                10000,
                ClaimProofOptional::claim_proof(
                    "d6981c4c287487efebcd93fe7695ad0a9317eb926e9c230ac643bce441389d2e",
                ),
                vec![
                    "e5f11726de09f158a299ff803dc193ba185880bfda243eb19560a99b720a0ff1",
                    "193bb1198b29ba3e925c9b6d45b7ded1714ba5b9eeae2d30ae733d6954cfe4cc",
                ],
            ),
        ]
    }
}
