use cosmwasm_std::{coin, coins, Addr, BankMsg, Coin, CosmosMsg, Storage, Uint128};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgBurn, MsgMint};

use crate::{
    error::ContractError,
    state::{get_assets, Fee, ASSETS, FEE, TOKEN},
};

pub fn make_mint_msgs_with_fee_collection(
    fee: Fee,
    mint: Coin,
    minter: &Addr,
    receiver: &Addr,
) -> Result<Vec<CosmosMsg>, ContractError> {
    let mut msgs = vec![MsgMint {
        sender: minter.clone().into_string(),
        amount: Some(mint.clone().into()),
    }
    .into()];

    if let Some(ratio) = fee.mint {
        let fee_amount = (mint.amount * ratio).max(Uint128::one());

        msgs.push(
            BankMsg::Send {
                to_address: fee.collector.to_string(),
                amount: coins(fee_amount.u128(), &mint.denom),
            }
            .into(),
        );

        msgs.push(
            BankMsg::Send {
                to_address: receiver.to_string(),
                amount: coins(mint.amount.checked_sub(fee_amount)?.u128(), &mint.denom),
            }
            .into(),
        );
    } else {
        msgs.push(
            BankMsg::Send {
                to_address: receiver.to_string(),
                amount: vec![mint],
            }
            .into(),
        );
    }

    Ok(msgs)
}

pub fn make_burn_msgs_with_fee_collection(
    fee: Fee,
    burn: Coin,
    burner: &Addr,
) -> Result<Vec<CosmosMsg>, ContractError> {
    let mut msgs = vec![];

    if let Some(ratio) = fee.burn {
        let fee_amount = (burn.amount * ratio).max(Uint128::one());

        msgs.push(
            BankMsg::Send {
                to_address: fee.collector.to_string(),
                amount: coins(fee_amount.u128(), &burn.denom),
            }
            .into(),
        );

        msgs.push(
            MsgBurn {
                sender: burner.to_string(),
                amount: Some(coin(burn.amount.checked_sub(fee_amount)?.u128(), &burn.denom).into()),
            }
            .into(),
        );
    } else {
        msgs.push(
            MsgBurn {
                sender: burner.to_string(),
                amount: Some(burn.into()),
            }
            .into(),
        );
    }

    Ok(msgs)
}

// It's unclear to me what this function is doing. How does it affect the fees
// currently in the storage. It seems to be overriding them and not updating
// them
pub fn collect_streaming_fee(storage: &mut dyn Storage, now: u64) -> Result<(), ContractError> {
    let fee = FEE.load(storage)?;

    let assets = get_assets(storage)?;
    let (assets, collected) = fee.calculate_streaming_fee(assets, now)?;
    if let Some(collected) = collected {
        FEE.save(
            storage,
            &Fee {
                collected,
                stream_last_collected_at: now,
                ..fee
            },
        )?;

        for (denom, unit) in assets {
            ASSETS.save(storage, denom, &unit)?;
        }
    }

    Ok(())
}

// must call collect_streaming_fee before call this function
pub fn realize_streaming_fee(storage: &mut dyn Storage) -> Result<CosmosMsg, ContractError> {
    let fee = FEE.load(storage)?;
    let token = TOKEN.load(storage)?;
    let collected = fee
        .collected
        .iter()
        .map(|(denom, unit)| coin((*unit * token.total_supply).u128(), denom))
        .collect::<Vec<_>>();

    let msg = BankMsg::Send {
        to_address: fee.collector.to_string(),
        amount: collected,
    }
    .into();

    // empty pending fee
    FEE.save(
        storage,
        &Fee {
            collected: vec![],
            ..fee
        },
    )?;

    Ok(msg)
}

#[cfg(test)]
mod test {

    use std::str::FromStr;

    use cosmwasm_std::{
        testing::{mock_env, MockStorage},
        Decimal,
    };

    use crate::{
        state::RESERVE_DENOM,
        test::{default_fee, default_token, register_assets},
    };

    use super::*;

    #[test]
    fn test_make_mint_msgs() {
        let expected_mint_msg = MsgMint {
            sender: "minter".to_string(),
            amount: Some(coin(100, "uibcx").into()),
        }
        .into();
        let fee_ratio = Decimal::from_ratio(10u128, 100u128);
        let mint = coin(100, "uibcx");
        let minter = Addr::unchecked("minter");
        let receiver = Addr::unchecked("receiver");

        // with mint fee
        let fee = super::Fee {
            mint: Some(fee_ratio),
            ..default_fee()
        };
        let mut msgs =
            make_mint_msgs_with_fee_collection(fee, mint.clone(), &minter, &receiver).unwrap();
        assert_eq!(msgs.remove(0), expected_mint_msg);
        assert_eq!(
            msgs,
            vec![
                BankMsg::Send {
                    to_address: "collector".to_string(),
                    amount: vec![coin(10, "uibcx")],
                }
                .into(),
                BankMsg::Send {
                    to_address: "receiver".to_string(),
                    amount: vec![coin(90, "uibcx")],
                }
                .into()
            ]
        );

        // with no fee
        let fee = default_fee();
        let mut msgs = make_mint_msgs_with_fee_collection(fee, mint, &minter, &receiver).unwrap();
        assert_eq!(msgs.remove(0), expected_mint_msg);
        assert_eq!(
            msgs,
            vec![BankMsg::Send {
                to_address: "receiver".to_string(),
                amount: vec![coin(100, "uibcx")],
            }
            .into()]
        );
    }

    #[test]
    fn test_make_burn_msgs() {
        // with burn fee
        let fee_ratio = Decimal::from_ratio(10u128, 100u128);
        let fee = super::Fee {
            burn: Some(fee_ratio),
            ..default_fee()
        };
        let burn = coin(100, "uibcx");
        let burner = Addr::unchecked("burner");

        let expected_msgs = vec![
            BankMsg::Send {
                to_address: fee.collector.to_string(),
                amount: vec![coin(10, &burn.denom)],
            }
            .into(),
            MsgBurn {
                sender: burner.to_string(),
                amount: Some(coin(90, &burn.denom).into()),
            }
            .into(),
        ];

        let msgs = make_burn_msgs_with_fee_collection(fee, burn, &burner).unwrap();
        assert_eq!(msgs, expected_msgs);

        // with no fee
        let fee = default_fee();
        let burn = coin(100, "uibcx");
        let burner = Addr::unchecked("burner");

        let expected_msgs = vec![MsgBurn {
            sender: burner.to_string(),
            amount: Some(burn.clone().into()),
        }
        .into()];

        let msgs = make_burn_msgs_with_fee_collection(fee, burn, &burner).unwrap();
        assert_eq!(msgs, expected_msgs);
    }

    #[test]
    fn test_streaming_fee() {
        let mut storage = MockStorage::default();

        let env = mock_env();
        let now = env.block.time.seconds();

        register_assets(
            &mut storage,
            &[
                ("ukrw", "1.0"),
                ("uusd", "2.0"),
                ("ujpy", "3.0"),
                (RESERVE_DENOM, "4.0"),
            ],
        );

        // 1 - (1.0015)^(1/(86400*365))
        let rate = Decimal::from_str("0.000000000047529").unwrap();
        let mut fee = default_fee();

        // nothing happened
        FEE.save(&mut storage, &fee).unwrap();
        collect_streaming_fee(&mut storage, now).unwrap();
        assert_eq!(FEE.load(&storage).unwrap().stream_last_collected_at, 0);

        // nothing happened
        fee.stream = Some(rate);
        fee.stream_last_collected_at = now;
        FEE.save(&mut storage, &fee).unwrap();
        collect_streaming_fee(&mut storage, now).unwrap();
        assert_eq!(FEE.load(&storage).unwrap().stream_last_collected_at, now);

        // 1 year after
        let origin_assets = get_assets(&storage).unwrap();

        fee.stream_last_collected_at = now;
        FEE.save(&mut storage, &fee).unwrap();
        collect_streaming_fee(&mut storage, now + (86400 * 365)).unwrap();

        let fee = FEE.load(&storage).unwrap();
        assert_eq!(fee.stream_last_collected_at, now + (86400 * 365));

        let assets = get_assets(&storage).unwrap();
        for (denom, unit) in assets {
            let (_, collected) = fee.collected.iter().find(|(d, _)| d == &denom).unwrap();
            let (_, origin) = origin_assets.iter().find(|(d, _)| d == &denom).unwrap();

            assert_eq!(origin, unit + collected);
        }

        // realize
        let token = default_token();
        TOKEN.save(&mut storage, &token).unwrap();

        let msg = realize_streaming_fee(&mut storage).unwrap();
        assert_eq!(
            msg,
            BankMsg::Send {
                to_address: fee.collector.to_string(),
                amount: fee
                    .collected
                    .iter()
                    .map(|(denom, unit)| coin((*unit * token.total_supply).u128(), denom))
                    .collect::<Vec<_>>()
            }
            .into()
        );
        assert_eq!(FEE.load(&storage).unwrap().collected, vec![]);
    }
}
