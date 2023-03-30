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

        if current_unit < &target_unit {
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
        attr("method", "finalize"),
        attr("executor", info.sender),
        attr("finalized_at", env.block.height.to_string()),
    ];

    Ok(Response::new().add_attributes(attrs))
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_finalize() {}
}
