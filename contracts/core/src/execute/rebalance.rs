use cosmwasm_std::{attr, coin, Decimal, Env, MessageInfo, Uint128};
use cosmwasm_std::{DepsMut, Response};
use ibc_interface::core::{RebalanceMsg, RebalanceTradeMsg};

use crate::{
    error::ContractError,
    state::{
        get_assets, Rebalance, ASSETS, COMPAT, GOV, LATEST_REBALANCE_ID, REBALANCES,
        RESERVE_BUFFER, RESERVE_DENOM, TOKEN, TRADE_INFOS,
    },
};

pub fn handle_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: RebalanceMsg,
) -> Result<Response, ContractError> {
    use RebalanceMsg::*;

    match msg {
        Init {
            manager,
            deflation,
            inflation,
        } => init(deps, env, info, manager, deflation, inflation),
        Trade(msg) => trade(deps, env, info, msg),
        Finalize {} => finalize(deps, env, info),
    }
}

fn check_duplication(
    x: Vec<(String, Decimal)>,
    y: Vec<(String, Decimal)>,
) -> Result<(), ContractError> {
    let mut y = y.into_iter();
    let f = x
        .into_iter()
        .filter(|xc| y.any(|yc| yc.0 == xc.0))
        .collect::<Vec<_>>();
    if !f.is_empty() {
        return Err(ContractError::InvalidArgument(format!(
            "duplicated coin: {:?}",
            f
        )));
    }

    Ok(())
}

fn init(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    manager: String,
    deflation: Vec<(String, Decimal)>,
    inflation: Vec<(String, Decimal)>,
) -> Result<Response, ContractError> {
    if info.sender != GOV.load(deps.storage)? {
        return Err(ContractError::Unauthorized {});
    }

    let rebalance_id = LATEST_REBALANCE_ID.load(deps.storage)?;
    if let Some(r) = REBALANCES.may_load(deps.storage, rebalance_id)? {
        if !r.finalized {
            return Err(ContractError::RebalanceNotFinalized {});
        }
    }

    check_duplication(deflation.clone(), inflation.clone())?;

    let rebalance = Rebalance {
        manager: deps.api.addr_validate(&manager)?,
        deflation,
        inflation,
        finalized: false,
    };
    let assets = get_assets(deps.storage)?;
    rebalance.validate(assets)?;
    REBALANCES.save(deps.storage, rebalance_id, &rebalance)?;

    let resp = Response::new().add_attributes(vec![
        attr("action", "rebalance_init"),
        attr("executor", info.sender),
        attr("manager", manager),
        attr("rebalance_id", rebalance_id.to_string()),
    ]);

    Ok(resp)
}

fn trade(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: RebalanceTradeMsg,
) -> Result<Response, ContractError> {
    // load rebalance info
    let rebalance_id = LATEST_REBALANCE_ID.load(deps.storage)?;
    let rebalance = REBALANCES.load(deps.storage, rebalance_id)?;
    if rebalance.manager != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    if rebalance.finalized {
        return Err(ContractError::RebalanceFinalized {});
    }

    match msg {
        RebalanceTradeMsg::Deflate {
            denom,
            amount,
            max_amount_in,
        } => deflate(deps, env, info, denom, amount, max_amount_in, rebalance),
        RebalanceTradeMsg::Inflate {
            denom,
            amount,
            min_amount_out,
        } => inflate(deps, env, info, denom, amount, min_amount_out, rebalance),
    }
}

fn deflate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denom: String,
    amount: Uint128,
    max_amount_in: Uint128,
    rebalance: Rebalance,
) -> Result<Response, ContractError> {
    let (_, target_unit) = match rebalance.deflation.iter().find(|v| v.0 == denom) {
        Some(d) => d,
        None => {
            return Err(ContractError::InvalidArgument(format!(
                "invalid denom: {:?}",
                denom
            )))
        }
    };

    // load token
    let token = TOKEN.load(deps.storage)?;

    // handle if request asset is the same as reserve denom
    if token.reserve_denom == denom {
        let origin_unit = ASSETS.load(deps.storage, denom.clone())?;
        let reserve_unit = ASSETS.load(deps.storage, RESERVE_DENOM.to_string())?;
        let additional_unit = Decimal::checked_from_ratio(amount, token.total_supply)?;
        if origin_unit.checked_sub(additional_unit)? < *target_unit {
            return Err(ContractError::InvalidArgument(format!(
                "exceed max trade amount: {:?}",
                amount
            )));
        }

        ASSETS.save(
            deps.storage,
            denom.clone(),
            &origin_unit.checked_sub(additional_unit)?,
        )?;
        ASSETS.save(
            deps.storage,
            RESERVE_DENOM.to_string(),
            &reserve_unit.checked_add(additional_unit)?,
        )?;

        // 1:1 conversion is done
        return Ok(Response::new().add_attributes(vec![
            attr("method", "deflate"),
            attr("executor", info.sender),
            attr("origin_unit", origin_unit.to_string()),
            attr("target_unit", target_unit.to_string()),
            attr("amount", amount),
        ]));
    }

    // load trade_info
    let mut trade_info = TRADE_INFOS.load(deps.storage, denom.clone())?;
    trade_info.checked_update_cooldown(env.block.time.seconds())?;
    if trade_info.max_trade_amount < amount {
        return Err(ContractError::InvalidArgument(format!(
            "exceed max trade amount: {:?}",
            trade_info
        )));
    }

    // load stored units
    let origin_unit = ASSETS.load(deps.storage, denom.clone())?;
    let reserve_unit = ASSETS.load(deps.storage, RESERVE_DENOM.to_string())?;

    let amount_gap = origin_unit.checked_sub(*target_unit)? * token.total_supply;
    if amount_gap < amount {
        return Err(ContractError::InvalidArgument(format!(
            "insufficient amount: {:?}",
            amount_gap
        )));
    }

    // simulate
    let compat = COMPAT.load(deps.storage)?;
    let amount_in = trade_info.routes.sim_swap_exact_out(
        &deps.querier,
        &compat,
        &env.contract.address,
        coin(amount.u128(), &denom),
    )?;
    if max_amount_in < amount_in {
        return Err(ContractError::OverSlippageAllowance {});
    }

    // deduct & expand stored units
    let deduct_unit = Decimal::checked_from_ratio(amount_in, token.total_supply)?;
    let expand_unit = Decimal::checked_from_ratio(amount, token.total_supply)?;

    ASSETS.save(
        deps.storage,
        denom.clone(),
        &(origin_unit.checked_sub(deduct_unit)?),
    )?;
    ASSETS.save(
        deps.storage,
        RESERVE_DENOM.to_string(),
        &(reserve_unit.checked_add(expand_unit)?),
    )?;

    // distribute to buffer
    let total_weight = rebalance
        .inflation
        .iter()
        .fold(Decimal::zero(), |acc, (_, w)| acc.checked_add(*w).unwrap());

    for (denom, weight) in rebalance.inflation {
        let mut buffer = RESERVE_BUFFER.load(deps.storage, denom.clone())?;
        buffer = buffer.checked_add(weight.checked_div(total_weight)? * amount_in)?;
        RESERVE_BUFFER.save(deps.storage, denom, &buffer)?;
    }

    // build response
    Ok(Response::new()
        .add_message(trade_info.routes.msg_swap_exact_out(
            &env.contract.address,
            &denom,
            amount,
            amount_in,
        ))
        .add_attributes(vec![
            attr("method", "deflate"),
            attr("executor", info.sender),
            attr("denom", denom),
            attr("origin_unit", origin_unit.to_string()),
            attr("target_unit", target_unit.to_string()),
            attr("amount_in", amount_in),
            attr("amount_out", amount),
        ]))
}

fn inflate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denom: String,
    amount: Uint128,
    min_amount_out: Uint128,
    rebalance: Rebalance,
) -> Result<Response, ContractError> {
    if !rebalance.inflation.iter().any(|v| v.0 == denom) {
        return Err(ContractError::InvalidArgument(format!(
            "invalid denom: {:?}",
            denom
        )));
    }

    // load trade_info
    let mut trade_info = TRADE_INFOS.load(deps.storage, denom.clone())?;
    trade_info.checked_update_cooldown(env.block.time.seconds())?;
    if trade_info.max_trade_amount < amount {
        return Err(ContractError::InvalidArgument(format!(
            "exceed max trade amount: {:?}",
            trade_info
        )));
    }

    // deduct reserve buffer amount
    let buffer = RESERVE_BUFFER.load(deps.storage, denom.clone())?;
    if buffer < amount {
        return Err(ContractError::InvalidArgument(format!(
            "insufficient buffer: {:?}",
            buffer
        )));
    }

    // simulate
    let compat = COMPAT.load(deps.storage)?;
    let amount_out = trade_info.routes.sim_swap_exact_in(
        &deps.querier,
        &compat,
        &env.contract.address,
        coin(amount.u128(), &denom),
    )?;
    if min_amount_out < amount_out {
        return Err(ContractError::OverSlippageAllowance {});
    }

    // deduct & expand stored units
    let token = TOKEN.load(deps.storage)?;

    let reserve_unit = ASSETS.load(deps.storage, RESERVE_DENOM.to_string())?;
    let origin_unit = ASSETS.load(deps.storage, denom.clone())?;

    let deduct_unit = Decimal::checked_from_ratio(amount, token.total_supply)?;
    let expand_unit = Decimal::checked_from_ratio(amount_out, token.total_supply)?;

    ASSETS.save(
        deps.storage,
        RESERVE_DENOM.to_string(),
        &reserve_unit.checked_sub(deduct_unit)?,
    )?;
    ASSETS.save(
        deps.storage,
        denom.clone(),
        &origin_unit.checked_add(expand_unit)?,
    )?;

    // increase buffer
    RESERVE_BUFFER.save(deps.storage, denom.clone(), &buffer.checked_sub(amount)?)?;

    // build response
    Ok(Response::new()
        .add_message(trade_info.routes.msg_swap_exact_in(
            &env.contract.address,
            RESERVE_DENOM,
            amount,
            amount_out,
        ))
        .add_attributes(vec![
            attr("method", "inflate"),
            attr("executor", info.sender),
            attr("denom", denom),
            attr("amount_in", amount),
            attr("amount_out", amount_out),
        ]))
}

fn finalize(deps: DepsMut, _env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    let rebalance_id = LATEST_REBALANCE_ID.load(deps.storage)?;
    LATEST_REBALANCE_ID.save(deps.storage, &(rebalance_id + 1))?;

    let mut rebalance = REBALANCES.load(deps.storage, rebalance_id)?;

    // check deflation
    for (denom, target_unit) in rebalance.deflation.clone() {
        let current_unit = ASSETS.load(deps.storage, denom)?;
        if target_unit == current_unit {
            continue;
        } else {
            return Err(ContractError::UnableToFinalize {});
        }
    }

    // check inflation
    if !ASSETS
        .load(deps.storage, RESERVE_DENOM.to_string())?
        .is_zero()
    {
        return Err(ContractError::UnableToFinalize {});
    }

    rebalance.finalized = true;

    REBALANCES.save(deps.storage, rebalance_id, &rebalance)?;

    Ok(Default::default())
}
