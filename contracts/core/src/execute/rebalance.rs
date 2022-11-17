use std::collections::BTreeMap;

use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, SubMsg, Uint128};
use ibc_interface::core::RebalanceMsg;
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin, osmosis::gamm::v1beta1::MsgSwapExactAmountIn,
};

use crate::{
    error::ContractError,
    state::{
        RebalanceInfo, CONFIG, REBALANCES, REBALANCE_LATEST_ID, STATE, TRADE_ALLOCATIONS,
        TRADE_STRATEGIES, TRADE_TOTAL_ALLOCATION,
    },
    REPLY_ID_REBALANCE,
};

use super::{check_and_get_rebalance_info, check_and_get_strategy, check_and_simulate_trade};

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
            amortization,
        } => init(deps, env, info, manager, deflation, amortization),
        Deflate {
            asset,
            amount_token_in,
            amount_reserve_min,
        } => deflate(deps, env, info, asset, amount_token_in, amount_reserve_min),
        Amortize {
            asset,
            amount_reserve_in,
            amount_token_min,
        } => amortize(deps, env, info, asset, amount_reserve_in, amount_token_min),
        Finish {} => finish(deps, env, info),
    }
}

fn init(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    manager: String,
    deflation: Vec<(String, Uint128)>,
    amortization: Vec<(String, Uint128)>,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let mut rebalance_id = REBALANCE_LATEST_ID.load(deps.storage)?;
    let rebalance = REBALANCES.may_load(deps.storage, rebalance_id)?;

    if let Some(r) = rebalance {
        if r.finished {
            rebalance_id += 1;
        } else {
            return Err(ContractError::RebalanceAlreadyOnGoing {});
        }
    }

    REBALANCES.save(
        deps.storage,
        rebalance_id,
        &RebalanceInfo::new(
            deps.api.addr_validate(&manager)?,
            state.assets,
            deflation,
            amortization,
        )?,
    )?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "rebalance_init"),
        attr("executor", info.sender),
        attr("manager", manager),
    ]);

    Ok(resp)
}

fn deflate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    asset: String,
    token_in: Uint128,
    out_min: Uint128,
) -> Result<Response, ContractError> {
    let mut strategy =
        check_and_get_strategy(deps.storage, env.block.time.seconds(), &asset, &token_in)?;
    strategy.last_traded_at = env.block.time.seconds();
    TRADE_STRATEGIES.save(deps.storage, &asset, &strategy)?;

    let rebalance = check_and_get_rebalance_info(deps.storage)?;

    // fetch actual / expected unit
    let mut state = STATE.load(deps.storage)?;
    let start_unit = rebalance.from.get(&asset).unwrap();
    let actual_unit = state.assets.get(&asset).unwrap();
    let expected_unit = start_unit.checked_sub(*rebalance.deflation.get(&asset).unwrap())?;
    if actual_unit <= &expected_unit {
        return Err(ContractError::RebalanceConditionFulfilled {});
    }

    // calculate gap between units
    let diff = actual_unit.checked_sub(expected_unit)?;
    let required_to_swap = diff.checked_mul(state.total_supply)?;
    if token_in > required_to_swap {
        return Err(ContractError::TradeAmountExceeded {});
    }

    // simulate it!
    let token_out_amount = check_and_simulate_trade(
        &deps.querier,
        &env.contract.address,
        &token_in,
        strategy.route_sell(),
        &out_min,
    )?;

    // update assets
    state.assets.insert(
        asset.clone(),
        actual_unit
            .checked_mul(state.total_supply)?
            .checked_sub(token_in)?
            .checked_div(state.total_supply)?,
    );
    state.total_reserve = state.total_reserve.checked_add(token_out_amount)?;

    STATE.save(deps.storage, &state)?;

    // update reserve token allocation for amortization
    let total_weight = rebalance
        .amortization
        .iter()
        .fold(Uint128::zero(), |v, (_, weight)| v + weight);
    let total_allocation = TRADE_TOTAL_ALLOCATION.load(deps.storage)?;

    TRADE_TOTAL_ALLOCATION.save(
        deps.storage,
        &total_allocation.checked_add(token_out_amount)?,
    )?;

    for (denom, weight) in rebalance.amortization {
        TRADE_ALLOCATIONS.update(deps.storage, &denom, |v| {
            Result::<Uint128, ContractError>::Ok(
                v.unwrap_or_default()
                    + token_out_amount.checked_multiply_ratio(weight, total_weight)?,
            )
        })?;
    }

    // setup swap message
    let swap_msg = MsgSwapExactAmountIn {
        sender: env.contract.address.to_string(),
        routes: strategy.route_sell(),
        token_in: Some(Coin {
            denom: asset,
            amount: token_in.to_string(),
        }),
        token_out_min_amount: out_min.to_string(),
    };

    let resp = Response::new()
        .add_submessage(SubMsg::reply_on_success(swap_msg, REPLY_ID_REBALANCE))
        .add_attributes(vec![
            attr("method", "trade_deflate"),
            attr("executor", info.sender),
        ]);

    Ok(resp)
}

fn amortize(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    asset: String,
    reserve_in: Uint128,
    out_min: Uint128,
) -> Result<Response, ContractError> {
    let mut strategy =
        check_and_get_strategy(deps.storage, env.block.time.seconds(), &asset, &out_min)?;
    strategy.last_traded_at = env.block.time.seconds();
    TRADE_STRATEGIES.save(deps.storage, &asset, &strategy)?;

    // check allocation
    let allocation = TRADE_ALLOCATIONS.load(deps.storage, &asset)?;
    if allocation < reserve_in {
        return Err(ContractError::RebalanceRanOutOfAllocation {});
    }

    // simulate it!
    let token_out_amount = check_and_simulate_trade(
        &deps.querier,
        &env.contract.address,
        &reserve_in,
        strategy.route_buy(),
        &out_min,
    )?;

    // update assets
    let mut state = STATE.load(deps.storage)?;
    let config = CONFIG.load(deps.storage)?;

    // calc token_unit
    let token_unit = state.assets.get(&asset).unwrap();
    state.assets.insert(
        asset.clone(),
        token_unit
            .checked_mul(state.total_supply)?
            .checked_add(token_out_amount)?
            .checked_div(state.total_supply)?,
    );
    state.total_reserve = state.total_reserve.checked_sub(reserve_in)?;

    STATE.save(deps.storage, &state)?;

    // update reserve token allocation for amortization
    let total_allocation = TRADE_TOTAL_ALLOCATION.load(deps.storage)?;
    let allocation_sub =
        reserve_in.checked_multiply_ratio(total_allocation, state.total_reserve)?;

    TRADE_TOTAL_ALLOCATION.save(deps.storage, &total_allocation.checked_sub(allocation_sub)?)?;
    TRADE_ALLOCATIONS.update(deps.storage, &asset, |v| match v {
        Some(v) => Ok(v.checked_sub(allocation_sub)?),
        None => Err(ContractError::TradeNoAllocation {}),
    })?;

    let msg = MsgSwapExactAmountIn {
        sender: env.contract.address.to_string(),
        routes: strategy.route_buy(),
        token_in: Some(Coin {
            denom: config.denom,
            amount: reserve_in.to_string(),
        }),
        token_out_min_amount: out_min.to_string(),
    };

    let resp = Response::new()
        .add_submessage(SubMsg::reply_on_success(msg, REPLY_ID_REBALANCE))
        .add_attributes(vec![
            attr("method", "trade_amortize"),
            attr("executor", info.sender),
        ]);

    Ok(resp)
}

fn finish(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let rebalance_id = REBALANCE_LATEST_ID.load(deps.storage)?;
    let rebalance = REBALANCES.may_load(deps.storage, rebalance_id)?;
    if rebalance.is_none() {
        return Err(ContractError::RebalanceInfoNotFound {});
    }

    // query all balances
    let balances_resp = deps.querier.query_all_balances(&env.contract.address)?;
    let balances: BTreeMap<_, _> = balances_resp
        .into_iter()
        .map(|data| (data.denom, data.amount))
        .collect();

    let mut rebalance = rebalance.unwrap();
    let state = STATE.load(deps.storage)?;

    // check rebalance has already finished
    if rebalance.finished {
        return Err(ContractError::RebalanceAlreadyFinished {});
    }

    // check simulated / actual unit
    let after_deflation = rebalance
        .from
        .iter()
        .map(|(asset, unit)| {
            let deflation = rebalance.deflation.get(asset).unwrap();
            Ok((asset.clone(), unit.checked_sub(*deflation)?))
        })
        .collect::<Result<BTreeMap<_, _>, ContractError>>()?;
    for (asset, unit) in state.assets.iter() {
        if after_deflation.get(asset).unwrap() != unit {
            return Err(ContractError::RebalanceValidationFailed {
                reason: "simulated unit does not match".to_string(),
            });
        }

        let balance = balances.get(asset).unwrap();
        if balance.checked_div(state.total_supply)? != unit {
            return Err(ContractError::RebalanceValidationFailed {
                reason: "actual unit does not match".to_string(),
            });
        }
    }

    // check balance of reserve token
    if !state.total_reserve.is_zero() {
        return Err(ContractError::RebalanceValidationFailed {
            reason: "reserve not drained".to_string(),
        });
    }

    rebalance.finished = true;

    REBALANCES.save(deps.storage, rebalance_id, &rebalance)?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "rebalance_finish"),
        attr("executor", info.sender),
    ]);

    Ok(resp)
}
