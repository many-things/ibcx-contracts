use cosmwasm_std::{attr, Decimal, DepsMut, MessageInfo, Response, Storage};

use crate::{
    error::RebalanceError,
    state::{Rebalance, Units, CONFIG, FEE, INDEX_UNITS, REBALANCE, RESERVE_UNITS},
    StdResult,
};

fn freeze_streaming_fee(storage: &mut dyn Storage) -> StdResult<()> {
    let mut fee = FEE.load(storage)?;

    if let Some(streaming_fee) = fee.streaming_fee.as_mut() {
        streaming_fee.freeze = true;
    }

    FEE.save(storage, &fee)?;

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
    use crate::{
        state::{Fee, StreamingFee, FEE},
        test::mock_dependencies,
    };

    use super::freeze_streaming_fee;

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
        // todo!()
    }
}
