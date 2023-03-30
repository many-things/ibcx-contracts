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
mod tests {}
