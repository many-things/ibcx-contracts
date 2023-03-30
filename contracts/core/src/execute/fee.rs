use cosmwasm_std::{attr, BankMsg, CosmosMsg, DepsMut, MessageInfo, Response, Storage};

use crate::{
    assert_sender,
    state::{FEE, INDEX_UNITS, TOTAL_SUPPLY},
};

use crate::StdResult;

pub fn collect_streaming_fee(storage: &mut dyn Storage, now_in_sec: u64) -> StdResult<()> {
    let mut fee = FEE.load(storage)?;
    let total_supply = TOTAL_SUPPLY.load(storage)?;

    if let Some(streaming_fee) = fee.streaming_fee.as_mut() {
        let index_units = INDEX_UNITS.load(storage)?;
        let (new_index_units, _) = streaming_fee.collect(index_units, now_in_sec, total_supply)?;
        INDEX_UNITS.save(storage, &new_index_units)?;
    }

    FEE.save(storage, &fee)?;

    Ok(())
}

pub fn realize_streaming_fee(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let mut fee = FEE.load(deps.storage)?;
    assert_sender(&fee.collector, &info.sender)?;

    let mut msgs: Vec<CosmosMsg> = vec![];

    // collect streaming fee
    if let Some(mut streaming_fee) = fee.streaming_fee.as_mut() {
        msgs.push(
            BankMsg::Send {
                to_address: fee.collector.to_string(),
                amount: streaming_fee.collected.clone(),
            }
            .into(),
        );

        streaming_fee.collected = vec![];
    }

    FEE.save(deps.storage, &fee)?;

    let attrs = vec![attr("method", "realize"), attr("executor", info.sender)];

    Ok(Response::new().add_messages(msgs).add_attributes(attrs))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{
        attr, coin,
        testing::{mock_info, MockStorage},
        Addr, BankMsg, Decimal, StdError, SubMsg, Uint128,
    };

    use crate::{
        error::ContractError,
        state::{Fee, StreamingFee, Units, FEE, INDEX_UNITS, TOTAL_SUPPLY},
        test::mock_dependencies,
    };

    use super::{collect_streaming_fee, realize_streaming_fee};

    #[test]
    fn test_collect_streaming_fee() {
        let mut storage = MockStorage::new();

        FEE.save(
            &mut storage,
            &Fee {
                streaming_fee: Some(StreamingFee {
                    rate: Decimal::percent(1),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .unwrap();
        INDEX_UNITS
            .save(&mut storage, &Units::from(vec![("uatom", "10000")]))
            .unwrap();
        TOTAL_SUPPLY.save(&mut storage, &Uint128::one()).unwrap();

        // frozen
        FEE.update(&mut storage, |mut v| {
            if let Some(streaming_fee) = v.streaming_fee.as_mut() {
                streaming_fee.freeze = true
            };
            Ok::<_, StdError>(v)
        })
        .unwrap();
        collect_streaming_fee(&mut storage, 86400).unwrap();
        assert_eq!(
            INDEX_UNITS.load(&storage).unwrap(),
            Units::from(vec![("uatom", "10000")])
        );

        // zero delta
        FEE.update(&mut storage, |mut v| {
            if let Some(streaming_fee) = v.streaming_fee.as_mut() {
                streaming_fee.freeze = false
            };
            Ok::<_, StdError>(v)
        })
        .unwrap();
        collect_streaming_fee(&mut storage, 0).unwrap();
        assert_eq!(
            INDEX_UNITS.load(&storage).unwrap(),
            Units::from(vec![("uatom", "10000")])
        );

        // 1.1046221254
        collect_streaming_fee(&mut storage, 10).unwrap();

        let mut index_units = INDEX_UNITS.load(&storage).unwrap();
        let uatom_unit = index_units.pop_key("uatom").unwrap().1;
        assert!(uatom_unit > Decimal::from_str("8953.7").unwrap());
        assert!(uatom_unit < Decimal::from_str("8953.8").unwrap());

        let fee_after = FEE.load(&storage).unwrap();

        let uatom_collected = fee_after.streaming_fee.unwrap().collected.pop().unwrap();
        assert!(uatom_collected.amount >= Uint128::new(1046));
        assert!(uatom_collected.amount < Uint128::new(1047));
    }

    #[test]
    fn test_realize_streaming_fee() {
        let mut deps = mock_dependencies();

        let expected_collected = [("uatom", 10000u128), ("uosmo", 12345u128)]
            .into_iter()
            .map(|v| coin(v.1, v.0))
            .collect::<Vec<_>>();

        let cases = [
            (
                "user",
                Some(StreamingFee {
                    collected: expected_collected.clone(),
                    ..Default::default()
                }),
                Err(ContractError::Unauthorized),
                expected_collected.clone(),
            ),
            (
                "collector",
                None,
                Ok((
                    vec![],
                    vec![attr("method", "realize"), attr("executor", "collector")],
                )),
                expected_collected.clone(),
            ),
            (
                "collector",
                Some(StreamingFee {
                    collected: expected_collected.clone(),
                    ..Default::default()
                }),
                Ok((
                    vec![SubMsg::new(BankMsg::Send {
                        to_address: "collector".to_string(),
                        amount: expected_collected,
                    })],
                    vec![attr("method", "realize"), attr("executor", "collector")],
                )),
                vec![],
            ),
        ];

        for (sender, streaming_fee, expected, collected) in cases {
            FEE.save(
                deps.as_mut().storage,
                &Fee {
                    collector: Addr::unchecked("collector"),
                    streaming_fee: streaming_fee.clone(),
                    ..Default::default()
                },
            )
            .unwrap();

            let res = realize_streaming_fee(deps.as_mut(), mock_info(sender, &[]));
            assert_eq!(res.map(|v| (v.messages, v.attributes)), expected);

            if streaming_fee.is_some() {
                assert_eq!(
                    FEE.load(deps.as_ref().storage)
                        .unwrap()
                        .streaming_fee
                        .unwrap()
                        .collected,
                    collected
                );
            }
        }
    }
}
