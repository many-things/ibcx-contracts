mod claim;
mod close;
mod fund;
mod register;

pub use claim::claim;
pub use close::close;
pub use fund::fund;
pub use register::register;

#[cfg(test)]
mod tests {
    use cosmwasm_std::{coin, testing::mock_info, Addr, Binary, Coin, DepsMut, Env};
    use ibcx_interface::airdrop::RegisterPayload;

    use crate::airdrop::{Airdrop, BearerAirdrop, OpenAirdrop};

    use super::register;

    pub type Balances<'a> = &'a [(&'a str, &'a [Coin])];

    pub const DENOM_BASE: u128 = 10e6 as u128;

    pub fn normalize_amount(amount: f32) -> u128 {
        (amount * DENOM_BASE as f32) as u128
    }

    pub fn register_airdrop(deps: DepsMut, env: Env, airdrop: Airdrop, sign: Option<&str>) {
        match airdrop {
            Airdrop::Open(inner) => {
                // open
                let info_register_open = mock_info(
                    inner.creator.as_str(),
                    &[coin(inner.total_amount.u128(), &inner.denom)],
                );
                register(
                    deps,
                    env,
                    info_register_open,
                    RegisterPayload::open(&inner.merkle_root, &inner.denom, None),
                )
                .unwrap();
            }
            Airdrop::Bearer(inner) => {
                // bearer
                let info_register_bearer = mock_info(
                    inner.creator.as_str(),
                    &[coin(inner.total_amount.u128(), &inner.denom)],
                );

                register(
                    deps,
                    env,
                    info_register_bearer,
                    RegisterPayload::bearer(
                        &inner.merkle_root,
                        &inner.denom,
                        hex::encode(&inner.signer_pub),
                        sign.unwrap(),
                        None,
                    ),
                )
                .unwrap();
            }
        }
    }

    pub fn mock_open_airdrop(label: Option<&str>, created_at: u64) -> OpenAirdrop {
        let registerer = "tester".to_string();

        // local-out/osmo-staker.json
        let merkle_root = "597f35d2e2f4f5c02e31be44695da0c3e0ce03bbb212c6cfc4ef94d7d4940bb5";

        OpenAirdrop {
            creator: Addr::unchecked(&registerer),

            denom: "ukrw".to_string(),
            total_amount: normalize_amount(0.01).into(),
            total_claimed: normalize_amount(0.0).into(),
            merkle_root: merkle_root.to_string(),

            label: label.map(|v| format!("{registerer}/{v}")),
            created_at,
            closed_at: None,
        }
    }

    pub fn mock_bearer_airdrop(label: Option<&str>, created_at: u64) -> (BearerAirdrop, &str) {
        let registerer = "tester".to_string();

        // local-out/osmo-mission-1.json
        let merkle_root = "1cf8a2c703330168311d9b9a9bc9063865fda851fb513959416cb6d2bc98b4a7";

        // notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius
        let signer_addr = "osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks";
        let signer_pub = "02ec18c82501c5088119251679b538e9cf8eae502956cc862c7778aa148365e886";
        let signer_sig = "c8cccdaa7568544164b2bcbea55eaaaa7f52e63ff2e9f075d7419a4558f2ec5574f196d449304314f22a0803f4fc260c476a1380b6db72b7cb6976980b9a1a46";

        (
            BearerAirdrop {
                creator: Addr::unchecked(&registerer),
                signer: Addr::unchecked(signer_addr),
                signer_pub: Binary::from(hex::decode(signer_pub).unwrap()),

                denom: "ukrw".to_string(),
                total_amount: normalize_amount(0.1).into(),
                total_claimed: normalize_amount(0.0).into(),
                merkle_root: merkle_root.to_string(),

                label: label.map(|v| format!("{registerer}/{v}")),
                created_at,
                closed_at: None,
            },
            signer_sig,
        )
    }
}
