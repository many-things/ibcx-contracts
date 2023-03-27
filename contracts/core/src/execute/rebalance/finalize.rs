use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

use crate::{
    error::ContractError,
    state::{LATEST_REBALANCE_ID, REBALANCES, RESERVE_DENOM, UNITS},
};

pub fn finalize(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let rebalance_id = LATEST_REBALANCE_ID.load(deps.storage)?;
    LATEST_REBALANCE_ID.save(deps.storage, &(rebalance_id + 1))?;

    let mut rebalance = REBALANCES.load(deps.storage, rebalance_id)?;

    // check deflation
    for (denom, target_unit) in rebalance.deflation.clone() {
        let current_unit = UNITS.load(deps.storage, denom)?;
        if target_unit == current_unit {
            continue;
        } else {
            return Err(ContractError::UnableToFinalize {});
        }
    }

    // check inflation
    if !UNITS
        .load(deps.storage, RESERVE_DENOM.to_string())?
        .is_zero()
    {
        return Err(ContractError::UnableToFinalize {});
    }

    rebalance.finalized = true;

    REBALANCES.save(deps.storage, rebalance_id, &rebalance)?;

    Ok(Response::new().add_attributes(vec![
        attr("method", "finalize"),
        attr("executor", info.sender),
        attr("finalized_at", env.block.height.to_string()),
    ]))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        attr,
        testing::{mock_env, mock_info},
    };

    use crate::{
        execute::rebalance::test::setup,
        state::{REBALANCES, RESERVE_DENOM},
        test::{mock_dependencies, register_units},
    };

    use super::*;

    #[test]
    fn test_finalize() {
        let mut deps = mock_dependencies();

        setup(
            deps.as_mut().storage,
            1,
            &[("ukrw", "1.2"), ("ujpy", "1.5"), ("uusd", "1.3")],
            &[("uosmo", "1.0"), ("uatom", "3.14")],
            false,
        );

        register_units(
            deps.as_mut().storage,
            &[
                ("ukrw", "1.2"),
                ("ujpy", "1.5"),
                ("uusd", "1.3"),
                (RESERVE_DENOM, "0.0"),
            ],
        );

        let res = finalize(deps.as_mut(), mock_env(), mock_info("manager", &[])).unwrap();

        assert_eq!(
            res.attributes,
            vec![
                attr("method", "finalize"),
                attr("executor", "manager"),
                attr("finalized_at", mock_env().block.height.to_string())
            ]
        );

        assert!(REBALANCES.load(deps.as_ref().storage, 1).unwrap().finalized);
    }
}
