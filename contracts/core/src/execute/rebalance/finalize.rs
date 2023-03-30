use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Storage};

use crate::{
    assert_sender,
    error::RebalanceError,
    state::{FEE, INDEX_UNITS, REBALANCE, RESERVE_UNITS},
    StdResult,
};

fn unfreeze_streaming_fee(storage: &mut dyn Storage, now_in_sec: u64) -> StdResult<()> {
    let mut fee = FEE.load(storage)?;
    if let Some(streaming_fee) = fee.streaming_fee.as_mut() {
        streaming_fee.freeze = false;
        streaming_fee.last_collected_at = now_in_sec;
    }
    FEE.save(storage, &fee)?;

    Ok(())
}

pub fn finalize(deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
    unfreeze_streaming_fee(deps.storage, env.block.time.seconds())?;

    let rebalance = match REBALANCE.may_load(deps.storage)? {
        Some(v) => v,
        None => return Err(RebalanceError::NotOnRebalancing.into()),
    };
    if let Some(manager) = rebalance.manager {
        assert_sender(&manager, &info.sender)?;
    }

    let index_units = INDEX_UNITS.load(deps.storage)?;

    // check deflation
    for (denom, target_unit) in rebalance.deflation {
        let (_, current_unit) = index_units.get_key(&denom).unwrap();

        if &target_unit < current_unit {
            return Err(
                RebalanceError::unable_to_finalize("deflation condition did not met").into(),
            );
        }
    }

    // check reserve unit has been flushed to inflation units
    let reserve_units = RESERVE_UNITS.load(deps.storage)?;
    if !reserve_units.check_empty() {
        return Err(RebalanceError::unable_to_finalize("inflation condition did not met").into());
    }

    REBALANCE.remove(deps.storage);
    RESERVE_UNITS.remove(deps.storage);

    // response
    let attrs = vec![
        attr("method", "rebalance::finalize"),
        attr("executor", info.sender),
        attr("finalized_at", env.block.height.to_string()),
    ];

    Ok(Response::new().add_attributes(attrs))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        attr,
        testing::{mock_env, mock_info, MockStorage},
        Addr,
    };

    use crate::{
        error::{ContractError, RebalanceError},
        state::{Fee, Rebalance, StreamingFee, Units, FEE, INDEX_UNITS, REBALANCE, RESERVE_UNITS},
        test::mock_dependencies,
    };

    use super::{finalize, unfreeze_streaming_fee};

    #[test]
    fn test_unfreeze_streaming_fee() {
        let mut storage = MockStorage::new();

        FEE.save(
            &mut storage,
            &Fee {
                streaming_fee: None,
                ..Default::default()
            },
        )
        .unwrap();

        unfreeze_streaming_fee(&mut storage, 30).unwrap();
        assert!(FEE.load(&storage).unwrap().streaming_fee.is_none());

        FEE.save(
            &mut storage,
            &Fee {
                streaming_fee: Some(StreamingFee {
                    freeze: true,
                    last_collected_at: 10,
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .unwrap();

        unfreeze_streaming_fee(&mut storage, 12345).unwrap();

        let streaming_fee = FEE.load(&storage).unwrap().streaming_fee.unwrap();
        assert!(!streaming_fee.freeze);
        assert_eq!(streaming_fee.last_collected_at, 12345);
    }

    #[test]
    fn test_finalize() {
        let env = mock_env();
        let mut deps = mock_dependencies();

        FEE.save(deps.as_mut().storage, &Fee::default()).unwrap();

        let cases = [
            // sender = random, manager = none
            (
                "user",
                vec![("uatom", "0.89")].into(),
                vec![("uatom", "0.0")].into(),
                Some(Rebalance {
                    manager: None,
                    deflation: vec![("uatom", "0.9")].into(),
                    inflation: Units::default(),
                }),
                Ok(vec![
                    attr("method", "rebalance::finalize"),
                    attr("executor", "user"),
                    attr("finalized_at", env.block.height.to_string()),
                ]),
            ),
            // sender = random, manager = some
            (
                "user",
                vec![("uatom", "0.89")].into(),
                vec![("uatom", "0.0")].into(),
                Some(Rebalance {
                    manager: Some(Addr::unchecked("manager")),
                    deflation: vec![("uatom", "0.9")].into(),
                    inflation: Units::default(),
                }),
                Err(ContractError::Unauthorized),
            ),
            // sender = manager, manager = some
            (
                "manager",
                vec![("uatom", "0.89")].into(),
                vec![("uatom", "0.0")].into(),
                Some(Rebalance {
                    manager: Some(Addr::unchecked("manager")),
                    deflation: vec![("uatom", "0.9")].into(),
                    inflation: Units::default(),
                }),
                Ok(vec![
                    attr("method", "rebalance::finalize"),
                    attr("executor", "manager"),
                    attr("finalized_at", env.block.height.to_string()),
                ]),
            ),
            // index_unit > deflation
            (
                "manager",
                vec![("uatom", "0.91")].into(),
                vec![("uatom", "0.0")].into(),
                Some(Rebalance {
                    manager: Some(Addr::unchecked("manager")),
                    deflation: vec![("uatom", "0.9")].into(),
                    inflation: Units::default(),
                }),
                Err(RebalanceError::unable_to_finalize("deflation condition did not met").into()),
            ),
            // reserve_units != zero
            (
                "manager",
                vec![("uatom", "0.89")].into(),
                vec![("uatom", "0.01")].into(),
                Some(Rebalance {
                    manager: Some(Addr::unchecked("manager")),
                    deflation: vec![("uatom", "0.9")].into(),
                    inflation: Units::default(),
                }),
                Err(RebalanceError::unable_to_finalize("inflation condition did not met").into()),
            ),
        ];

        for (sender, index_units, reserve_units, rebalance, expected) in cases {
            INDEX_UNITS
                .save(deps.as_mut().storage, &index_units)
                .unwrap();

            RESERVE_UNITS
                .save(deps.as_mut().storage, &reserve_units)
                .unwrap();

            REBALANCE.remove(deps.as_mut().storage);
            if let Some(rebalance) = rebalance {
                REBALANCE.save(deps.as_mut().storage, &rebalance).unwrap();
            }

            let res = finalize(deps.as_mut(), env.clone(), mock_info(sender, &[]));
            assert_eq!(res.map(|v| v.attributes), expected);

            if expected.is_ok() {
                assert!(REBALANCE.may_load(deps.as_ref().storage).unwrap().is_none());
                assert!(RESERVE_UNITS
                    .may_load(deps.as_ref().storage)
                    .unwrap()
                    .is_none());
            }
        }
    }
}
