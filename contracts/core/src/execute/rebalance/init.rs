use cosmwasm_std::{attr, Decimal, DepsMut, MessageInfo, Response, Storage};

use crate::{
    error::RebalanceError,
    state::{Rebalance, Units, CONFIG, FEE, INDEX_UNITS, REBALANCE, RESERVE_UNITS},
    StdResult,
};

fn freeze_streaming_fee(storage: &mut dyn Storage) -> StdResult<()> {
    FEE.update(storage, |mut v| {
        if let Some(streaming_fee) = v.streaming_fee.as_mut() {
            streaming_fee.freeze = true;
        }
        StdResult::Ok(v)
    })?;

    Ok(())
}

// initialize the rebalance
// deflation: target unit of each denom to decrease
// inflation: weight of each denom to distribute
//
// basic flow of rebalance
//
//=========================================
// [ DEFLATION ]            [ INFLATION ]
//-----------------------------------------
//     | A  --\             /-->  D |
//     | B  ---> [RESERVE] ---->  E |
//     | C  --/             \-->  F |
//=========================================
pub fn init(
    deps: DepsMut,
    info: MessageInfo,
    manager: Option<String>,
    deflation: Vec<(String, Decimal)>,
    inflation: Vec<(String, Decimal)>,
) -> StdResult<Response> {
    freeze_streaming_fee(deps.storage)?;

    let config = CONFIG.load(deps.storage)?;

    config.check_gov(&info.sender)?;

    if REBALANCE.may_load(deps.storage)?.is_some() {
        return Err(RebalanceError::OnRebalancing.into());
    }

    // make new rebalance
    let rebalance = Rebalance {
        manager: manager
            .clone()
            .map(|v| deps.api.addr_validate(&v))
            .transpose()?,
        deflation: deflation.into(),
        inflation: inflation.into(),
    };

    // fetch current units and validate new rebalance
    let index_units = INDEX_UNITS.load(deps.storage)?;

    rebalance.validate(index_units)?;

    // save
    REBALANCE.save(deps.storage, &rebalance)?;
    RESERVE_UNITS.save(deps.storage, &Units::default())?;

    // response
    let attrs = vec![
        attr("method", "rebalance::init"),
        attr("executor", info.sender),
        attr("manager", manager.as_deref().unwrap_or("none")),
    ];

    let resp = Response::new().add_attributes(attrs);

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{attr, testing::mock_info, Addr};

    use crate::{
        error::RebalanceError,
        state::{
            Config, Fee, Rebalance, StreamingFee, Units, CONFIG, FEE, INDEX_UNITS, REBALANCE,
            RESERVE_UNITS,
        },
        test::mock_dependencies,
    };

    use super::{freeze_streaming_fee, init};

    #[test]
    fn test_freeze_streaming_fee() {
        let mut deps = mock_dependencies();

        FEE.save(
            deps.as_mut().storage,
            &Fee {
                streaming_fee: Some(StreamingFee {
                    freeze: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .unwrap();

        freeze_streaming_fee(deps.as_mut().storage).unwrap();

        assert!(
            FEE.load(deps.as_ref().storage)
                .unwrap()
                .streaming_fee
                .unwrap()
                .freeze
        );
    }

    #[test]
    fn test_init() {
        let mut deps = mock_dependencies();

        FEE.save(deps.as_mut().storage, &Fee::default()).unwrap();
        CONFIG
            .save(
                deps.as_mut().storage,
                &Config {
                    gov: Addr::unchecked("gov"),
                    ..Default::default()
                },
            )
            .unwrap();

        INDEX_UNITS
            .save(deps.as_mut().storage, &vec![("uatom", "1.1")].into())
            .unwrap();

        let cases = [
            (
                "gov",
                Some("manager"),
                false,
                Ok(vec![
                    attr("method", "rebalance::init"),
                    attr("executor", "gov"),
                    attr("manager", "manager"),
                ]),
            ),
            (
                "gov",
                None,
                false,
                Ok(vec![
                    attr("method", "rebalance::init"),
                    attr("executor", "gov"),
                    attr("manager", "none"),
                ]),
            ),
            ("gov", None, true, Err(RebalanceError::OnRebalancing.into())),
        ];

        for (sender, manager, rebalancing, expected) in cases {
            REBALANCE.remove(deps.as_mut().storage);
            if rebalancing {
                REBALANCE
                    .save(deps.as_mut().storage, &Default::default())
                    .unwrap();
            }

            let res = init(
                deps.as_mut(),
                mock_info(sender, &[]),
                manager.map(|v| v.to_string()),
                Units::from(vec![("uatom", "0.9")]).into(),
                Units::from(vec![("uosmo", "1.0")]).into(),
            );
            assert_eq!(res.map(|v| v.attributes), expected);

            if expected.is_ok() {
                assert_eq!(
                    REBALANCE.load(deps.as_ref().storage).unwrap(),
                    Rebalance {
                        manager: manager.map(Addr::unchecked),
                        deflation: Units::from(vec![("uatom", "0.9")]),
                        inflation: Units::from(vec![("uosmo", "1.0")]),
                    }
                );
                assert_eq!(
                    RESERVE_UNITS.load(deps.as_ref().storage).unwrap(),
                    Units::default()
                );
            }
        }
    }
}
