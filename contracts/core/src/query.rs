use std::{collections::BTreeMap, iter::FromIterator};

use cosmwasm_std::{to_binary, Binary, Decimal, Deps, Env, Order, StdResult, Uint128};
use cw_storage_plus::Bound;
use ibc_interface::{
    core::{
        AllocationResponse, ConfigResponse, ListAllocationResponse, ListRebalanceInfoResponse,
        ListStrategyResponse, PauseInfoResponse, PortfolioResponse, RebalanceInfoResponse,
        StrategyResponse,
    },
    get_and_check_limit,
    types::RangeOrder,
    DEFAULT_LIMIT, MAX_LIMIT,
};

use crate::{error::ContractError, state::PAUSED};

fn map_to_vec<T, U>(m: BTreeMap<T, U>) -> Vec<(T, U)> {
    m.into_iter().map(|(t, u)| (t, u)).collect()
}

pub fn config(deps: Deps, _env: Env) -> Result<Binary, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    Ok(to_binary(&ConfigResponse {
        gov: config.gov,
        denom: config.denom,
        reserve_denom: config.reserve_denom,
    })?)
}

pub fn pause_info(deps: Deps, _env: Env) -> Result<Binary, ContractError> {
    let pause_info = PAUSED.load(deps.storage)?;

    Ok(to_binary(&PauseInfoResponse {
        paused: pause_info.paused,
        expires_at: pause_info.expires_at,
    })?)
}

pub fn portfolio(deps: Deps, _env: Env) -> Result<Binary, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let mut state = STATE.load(deps.storage)?;

    let reserve_unit = state.total_reserve.checked_div(state.total_supply)?;

    state
        .assets
        .entry(config.reserve_denom)
        .and_modify(|v| *v += reserve_unit)
        .or_insert(reserve_unit);

    Ok(to_binary(&PortfolioResponse {
        total_supply: state.total_supply,
        assets: map_to_vec(state.assets),
    })?)
}

pub fn rebalance_info(deps: Deps, _env: Env, id: Option<u64>) -> Result<Binary, ContractError> {
    let id = id.unwrap_or(REBALANCE_LATEST_ID.load(deps.storage)?);
    let rebalance = REBALANCES.load(deps.storage, id)?;

    Ok(to_binary(&RebalanceInfoResponse {
        id,
        manager: rebalance.manager,
        init_status: map_to_vec(rebalance.from),
        deflation: map_to_vec(rebalance.deflation),
        amortization: map_to_vec(rebalance.amortization),
        finished: rebalance.finished,
    })?)
}

pub fn list_rebalance_info(
    deps: Deps,
    _env: Env,
    start_after: Option<u64>,
    limit: Option<u32>,
    order: Option<RangeOrder>,
) -> Result<Binary, ContractError> {
    let limit = get_and_check_limit(limit, MAX_LIMIT, DEFAULT_LIMIT)? as usize;
    let order = order.unwrap_or(RangeOrder::Asc).into();
    let bound = start_after.map(Bound::exclusive);
    let (min, max) = match order {
        Order::Ascending => (bound, None),
        Order::Descending => (None, bound),
    };

    let rebalances: Vec<(u64, RebalanceInfo)> = REBALANCES
        .range(deps.storage, min, max, order)
        .take(limit)
        .collect::<StdResult<_>>()?;

    let resps = rebalances
        .into_iter()
        .map(|(id, info)| RebalanceInfoResponse {
            id,
            manager: info.manager,
            init_status: Vec::from_iter(info.from.into_iter()),
            deflation: Vec::from_iter(info.deflation.into_iter()),
            amortization: Vec::from_iter(info.amortization.into_iter()),
            finished: info.finished,
        })
        .collect();

    Ok(to_binary(&ListRebalanceInfoResponse(resps))?)
}

pub fn strategy(deps: Deps, _env: Env, asset: String) -> Result<Binary, ContractError> {
    let strategy = TRADE_STRATEGIES.load(deps.storage, &asset)?;

    Ok(to_binary(&StrategyResponse {
        asset,
        routes: strategy.routes,
        cool_down: strategy.cool_down,
        max_trade_amount: strategy.max_trade_amount,
        last_traded_at: strategy.last_traded_at,
    })?)
}

pub fn list_strategy(
    deps: Deps,
    _env: Env,
    start_after: Option<String>,
    limit: Option<u32>,
    order: Option<RangeOrder>,
) -> Result<Binary, ContractError> {
    let limit = get_and_check_limit(limit, MAX_LIMIT, DEFAULT_LIMIT)? as usize;
    let order = order.unwrap_or(RangeOrder::Asc).into();
    let bound = start_after.map(|v| Bound::ExclusiveRaw(v.into_bytes()));
    let (min, max) = match order {
        Order::Ascending => (bound, None),
        Order::Descending => (None, bound),
    };

    let strategies: Vec<(String, TradeStrategy)> = TRADE_STRATEGIES
        .range(deps.storage, min, max, order)
        .take(limit)
        .collect::<StdResult<_>>()?;

    let resps = strategies
        .into_iter()
        .map(|(asset, strategy)| StrategyResponse {
            asset,
            routes: strategy.routes,
            cool_down: strategy.cool_down,
            max_trade_amount: strategy.max_trade_amount,
            last_traded_at: strategy.last_traded_at,
        })
        .collect();

    Ok(to_binary(&ListStrategyResponse(resps))?)
}

pub fn allocation(deps: Deps, _env: Env, asset: String) -> Result<Binary, ContractError> {
    let allocation = TRADE_ALLOCATIONS.load(deps.storage, &asset)?;
    let total = TRADE_TOTAL_ALLOCATION.load(deps.storage)?;
    let total_reserve = STATE.load(deps.storage)?.total_reserve;

    Ok(to_binary(&AllocationResponse {
        asset,
        allocation,
        ratio: Decimal::checked_from_ratio(allocation, total)?,
        extracted: total_reserve.checked_multiply_ratio(allocation, total)?,
    })?)
}

pub fn list_allocation(
    deps: Deps,
    _env: Env,
    start_after: Option<String>,
    limit: Option<u32>,
    order: Option<RangeOrder>,
) -> Result<Binary, ContractError> {
    let limit = get_and_check_limit(limit, MAX_LIMIT, DEFAULT_LIMIT)? as usize;
    let order = order.unwrap_or(RangeOrder::Asc).into();
    let bound = start_after.map(|v| Bound::ExclusiveRaw(v.into_bytes()));
    let (min, max) = match order {
        Order::Ascending => (bound, None),
        Order::Descending => (None, bound),
    };

    let allocations: Vec<(String, Uint128)> = TRADE_ALLOCATIONS
        .range(deps.storage, min, max, order)
        .take(limit)
        .collect::<StdResult<_>>()?;
    let total = TRADE_TOTAL_ALLOCATION.load(deps.storage)?;
    let total_reserve = STATE.load(deps.storage)?.total_reserve;

    let resps = allocations
        .into_iter()
        .map(|(asset, allocation)| {
            Ok(AllocationResponse {
                asset,
                allocation,
                ratio: Decimal::checked_from_ratio(allocation, total)?,
                extracted: total_reserve.checked_multiply_ratio(allocation, total)?,
            })
        })
        .collect::<Result<_, ContractError>>()?;

    Ok(to_binary(&ListAllocationResponse {
        allocations: resps,
        total,
        total_reserve,
    })?)
}
