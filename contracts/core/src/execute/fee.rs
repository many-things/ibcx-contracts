use cosmwasm_std::{
    attr, coin, coins, Addr, BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response,
    Storage, Uint128,
};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgBurn, MsgMint};
use std::collections::BTreeMap;

use crate::{
    error::ContractError,
    state::{get_units, Fee, FEE, TOKEN, UNITS},
};

/// # Notice
/// * minter must be address of contract itself.
/// * receiver usally is address of info.sender
/// * returns set of CosmosMsgs to be executed
pub fn make_mint_msgs_with_fee_collection(
    fee: Fee,
    mint: Coin,
    minter: &Addr,
    receiver: &Addr,
) -> Result<Vec<CosmosMsg>, ContractError> {
    // mint tokens to minter
    let mut msgs = vec![MsgMint {
        sender: minter.clone().into_string(),
        amount: Some(mint.clone().into()),
    }
    .into()];

    // if mint fee is set, deduct the total mint amount to its ratio and send to collector and the rest to the minter
    // if mint fee is not set, send the total mint amount to the minter
    if let Some(ratio) = fee.mint {
        // calcalate the fee amount
        let fee_amount = (mint.amount * ratio).max(Uint128::one());

        // send fee to collector
        msgs.push(
            BankMsg::Send {
                to_address: fee.collector.to_string(),
                amount: coins(fee_amount.u128(), &mint.denom),
            }
            .into(),
        );

        // send the rest to receiver
        msgs.push(
            BankMsg::Send {
                to_address: receiver.to_string(),
                amount: coins(mint.amount.checked_sub(fee_amount)?.u128(), &mint.denom),
            }
            .into(),
        );
    } else {
        // send all to receiver
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

/// # Notice
/// * burner must be address of contract itself.
/// * returns set of CosmosMsgs to be executed
pub fn make_burn_msgs_with_fee_collection(
    fee: Fee,
    burn: Coin,
    burner: &Addr,
) -> Result<Vec<CosmosMsg>, ContractError> {
    let mut msgs = vec![];

    // if burn fee is set, deduct the total received amount to its ratio and send to collector and burns remaining amount
    // if mint fee is not set, burn the total received amount
    if let Some(ratio) = fee.burn {
        // calcalate the fee amount
        let fee_amount = (burn.amount * ratio).max(Uint128::one());

        // send fee to collector
        msgs.push(
            BankMsg::Send {
                to_address: fee.collector.to_string(),
                amount: coins(fee_amount.u128(), &burn.denom),
            }
            .into(),
        );

        // burn the rest
        msgs.push(
            MsgBurn {
                sender: burner.to_string(),
                amount: Some(coin(burn.amount.checked_sub(fee_amount)?.u128(), &burn.denom).into()),
            }
            .into(),
        );
    } else {
        // burn all
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

// calculates streaming fee and aggregate into fee state
pub fn collect_streaming_fee(storage: &mut dyn Storage, now: u64) -> Result<(), ContractError> {
    let mut fee = FEE.load(storage)?;

    let units = get_units(storage)?;
    // get units after deduction and collected units
    let (units, collected) = fee.calculate_streaming_fee(units, now)?;

    // if nothing collected, do nothing
    if let Some(collected) = collected {
        // merge previous collected units and current collected units.
        // use BTreeMap to keep order of denom - or not, it probably can cause consensus error
        let mut merge: BTreeMap<_, _> = fee.stream_collected.into_iter().collect();
        for (denom, unit) in collected {
            // upsert denom's collected unit
            merge
                .entry(denom)
                .and_modify(|e| *e += unit)
                .or_insert(unit);
        }

        // update fee state
        fee.stream_collected = merge.into_iter().collect();
        fee.stream_last_collected_at = now;
        FEE.save(storage, &fee)?;

        // update units state to deducted ones
        for (denom, unit) in units {
            UNITS.save(storage, denom, &unit)?;
        }
    }

    Ok(())
}

// notice: must call collect_streaming_fee before call this function
pub fn realize_streaming_fee(storage: &mut dyn Storage) -> Result<CosmosMsg, ContractError> {
    // state loader
    let mut fee = FEE.load(storage)?;
    let token = TOKEN.load(storage)?;

    // calculate collected unit and convert it to realize amount
    let collected = fee
        .stream_collected
        .iter()
        .map(|(denom, unit)| coin((*unit * token.total_supply).u128(), denom))
        .collect::<Vec<_>>();

    // make send msg to transfer collected fees
    let msg = BankMsg::Send {
        to_address: fee.collector.to_string(),
        amount: collected,
    }
    .into();

    // empty pending fee and save
    fee.stream_collected = vec![];
    FEE.save(storage, &fee)?;

    Ok(msg)
}

pub fn realize(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let fee = FEE.load(deps.storage)?;
    if fee.collector != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let msg = realize_streaming_fee(deps.storage)?;

    Ok(Response::new().add_message(msg).add_attributes(vec![
        attr("method", "realize"),
        attr("executor", info.sender),
    ]))
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
        test::{default_fee, default_token, register_units},
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

        register_units(
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
        let origin_assets = get_units(&storage).unwrap();

        fee.stream_last_collected_at = now;
        FEE.save(&mut storage, &fee).unwrap();
        collect_streaming_fee(&mut storage, now + (86400 * 365)).unwrap();

        let fee = FEE.load(&storage).unwrap();
        assert_eq!(fee.stream_last_collected_at, now + (86400 * 365));

        let assets = get_units(&storage).unwrap();
        for (denom, unit) in assets {
            let (_, collected) = fee
                .stream_collected
                .iter()
                .find(|(d, _)| d == &denom)
                .unwrap();
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
                    .stream_collected
                    .iter()
                    .map(|(denom, unit)| coin((*unit * token.total_supply).u128(), denom))
                    .collect::<Vec<_>>()
            }
            .into()
        );
        assert_eq!(FEE.load(&storage).unwrap().stream_collected, vec![]);
    }
}
