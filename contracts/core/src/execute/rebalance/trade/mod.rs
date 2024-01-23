mod deflate;
mod inflate;

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Storage, Uint128};
use ibcx_interface::core::RebalanceTradeMsg;

use deflate::{deflate, deflate_reserve};
use inflate::{inflate, inflate_reserve};

use crate::{
    assert_sender,
    error::RebalanceError,
    state::{Units, CONFIG, INDEX_UNITS, REBALANCE, RESERVE_UNITS, TOTAL_SUPPLY},
    StdResult,
};

pub fn load_units(storage: &dyn Storage) -> StdResult<(Units, Units, Uint128)> {
    let index_units = INDEX_UNITS.load(storage)?;
    let reserve_units = RESERVE_UNITS.load(storage)?;
    let total_supply = TOTAL_SUPPLY.load(storage)?;

    Ok((index_units, reserve_units, total_supply))
}

// deflate / inflate the target denom
pub fn trade(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: RebalanceTradeMsg,
) -> StdResult<Response> {
    use RebalanceTradeMsg::*;

    let rebalance = match REBALANCE.may_load(deps.storage)? {
        Some(v) => v,
        None => return Err(RebalanceError::NotOnRebalancing.into()),
    };
    if let Some(manager) = rebalance.manager {
        assert_sender(&manager, &info.sender)?;
    }

    match msg {
        Deflate {
            target_denom,
            amount_out,
            max_amount_in,
        } => {
            if CONFIG.load(deps.storage)?.reserve_denom == target_denom {
                deflate_reserve(deps, info, target_denom)
            } else {
                deflate(deps, env, info, target_denom, amount_out, max_amount_in)
            }
        }

        Inflate {
            target_denom,
            amount_in,
            min_amount_out,
        } => {
            if CONFIG.load(deps.storage)?.reserve_denom == target_denom {
                inflate_reserve(deps, info, target_denom)
            } else {
                inflate(deps, env, info, target_denom, amount_in, min_amount_out)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_env, mock_info},
        Addr, StdError, Uint128,
    };
    use ibcx_interface::core::RebalanceTradeMsg;

    use crate::{
        error::{ContractError, RebalanceError},
        state::{Rebalance, REBALANCE},
        test::mock_dependencies,
    };

    use super::trade;

    #[test]
    fn test_trade() {
        let mut deps = mock_dependencies();

        let cases = [
            (
                "user",
                None,
                None,
                Err(RebalanceError::NotOnRebalancing.into()),
            ),
            (
                "user",
                Some(Rebalance::default()),
                None,
                Err(StdError::not_found(
                    "type: ibcx_core::state::config::Config; key: [63, 6F, 6E, 66, 69, 67]",
                )
                .into()), // ok
            ),
            (
                "user",
                Some(Rebalance::default()),
                Some("manager"),
                Err(ContractError::Unauthorized),
            ),
            (
                "manager",
                Some(Rebalance::default()),
                Some("manager"),
                Err(StdError::not_found(
                    "type: ibcx_core::state::config::Config; key: [63, 6F, 6E, 66, 69, 67]",
                )
                .into()), // ok
            ),
        ];

        for (sender, rebalance, manager, expected) in cases {
            REBALANCE.remove(deps.as_mut().storage);
            if let Some(mut rebalance) = rebalance {
                rebalance.manager = manager.map(Addr::unchecked);
                REBALANCE.save(deps.as_mut().storage, &rebalance).unwrap();
            }

            let res = trade(
                deps.as_mut(),
                mock_env(),
                mock_info(sender, &[]),
                RebalanceTradeMsg::Deflate {
                    target_denom: "".to_string(),
                    amount_out: Uint128::zero(),
                    max_amount_in: Uint128::zero(),
                },
            );
            assert_eq!(res, expected);
        }
    }
}
