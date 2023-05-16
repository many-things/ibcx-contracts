use cosmwasm_std::{Coin, Deps, Env, Timestamp, Uint128};
use ibcx_interface::{
    core::{
        GetConfigResponse, GetFeeResponse, GetPortfolioResponse, GetRebalanceResponse,
        GetTradeInfoResponse, ListTradeInfoResponse, PausedResponse, RebalancePayload,
        SimulateBurnResponse, SimulateMintResponse, StreamingFeeResponse, TradeInfoPayload,
    },
    range_option,
    types::RangeOrder,
};

use crate::{
    error::ContractError,
    state::{
        Config, PauseInfo, TradeInfo, CONFIG, FEE, INDEX_UNITS, PENDING_GOV, REBALANCE,
        TOTAL_SUPPLY, TRADE_INFOS,
    },
    StdResult,
};

pub fn get_balance(deps: Deps, _env: Env, account: String) -> StdResult<Uint128> {
    let Config { index_denom, .. } = CONFIG.load(deps.storage)?;

    let resp = deps.querier.query_balance(account, index_denom)?;

    Ok(resp.amount)
}

pub fn get_total_supply(deps: Deps, _env: Env) -> StdResult<Uint128> {
    Ok(TOTAL_SUPPLY.load(deps.storage)?)
}

pub fn get_config(deps: Deps, env: Env, time: Option<u64>) -> StdResult<GetConfigResponse> {
    let now_in_sec = env.block.time.seconds();
    let time_in_sec = time.unwrap_or(now_in_sec);

    let Config {
        gov,
        index_denom,
        reserve_denom,
        paused,
    } = CONFIG.load(deps.storage)?;

    let mut temp_env = env;

    temp_env.block.time = Timestamp::from_seconds(time_in_sec);

    let PauseInfo { paused, expires_at } = paused.refresh(&temp_env)?;

    let pending_gov = PENDING_GOV.may_load(deps.storage)?;

    Ok(GetConfigResponse {
        gov,
        pending_gov,
        paused: PausedResponse { paused, expires_at },
        index_denom,
        reserve_denom,
    })
}

pub fn get_fee(deps: Deps, env: Env, time: Option<u64>) -> StdResult<GetFeeResponse> {
    let now_in_sec = env.block.time.seconds();
    let time_in_sec = time.unwrap_or(now_in_sec);

    let total_supply = TOTAL_SUPPLY.load(deps.storage)?;

    let mut fee = FEE.load(deps.storage)?;

    if let Some(streaming_fee) = fee.streaming_fee.as_mut() {
        let index_units = INDEX_UNITS.load(deps.storage)?;
        streaming_fee.collect(index_units, time_in_sec, total_supply)?;
    }

    Ok(GetFeeResponse {
        collector: fee.collector,
        mint_fee: fee.mint_fee,
        burn_fee: fee.burn_fee,
        streaming_fee: fee.streaming_fee.map(|v| StreamingFeeResponse {
            rate: v.rate,
            collected: v.collected,
            freeze: v.freeze,
            last_collected_at: v.last_collected_at,
        }),
    })
}

pub fn get_portfolio(deps: Deps, env: Env, time: Option<u64>) -> StdResult<GetPortfolioResponse> {
    let now_in_sec = env.block.time.seconds();
    let time_in_sec = time.unwrap_or(now_in_sec);

    let total_supply = TOTAL_SUPPLY.load(deps.storage)?;
    let fee = FEE.load(deps.storage)?;

    let index_units = INDEX_UNITS.load(deps.storage)?;
    let index_units = fee
        .streaming_fee
        .map(|mut v| -> StdResult<_> {
            Ok(v.collect(index_units.clone(), time_in_sec, total_supply)?.0)
        })
        .transpose()?
        .unwrap_or(index_units);

    Ok(GetPortfolioResponse {
        total_supply,
        assets: index_units.calc_require_amount(total_supply),
        units: index_units.into(),
    })
}

pub fn get_rebalance(deps: Deps, _env: Env) -> StdResult<GetRebalanceResponse> {
    let rebalance = REBALANCE.may_load(deps.storage)?;

    Ok(GetRebalanceResponse {
        rebalance: rebalance.map(|v| RebalancePayload {
            manager: v.manager,
            deflation: v.deflation.to_vec(),
            inflation: v.inflation.to_vec(),
        }),
    })
}

fn conv_trade_info(denom_in: String, denom_out: String, trade_info: TradeInfo) -> TradeInfoPayload {
    TradeInfoPayload {
        denom_in,
        denom_out,
        routes: trade_info.routes,
        cooldown: trade_info.cooldown,
        max_trade_amount: trade_info.max_trade_amount,
        last_traded_at: trade_info.last_traded_at,
    }
}

pub fn get_trade_info(
    deps: Deps,
    denom_in: String,
    denom_out: String,
) -> StdResult<GetTradeInfoResponse> {
    let trade_info = TRADE_INFOS.may_load(deps.storage, (&denom_in, &denom_out))?;

    Ok(GetTradeInfoResponse {
        trade_info: trade_info.map(|v| conv_trade_info(denom_in, denom_out, v)),
    })
}

pub fn list_trade_info(
    deps: Deps,
    denom_in: String,
    start_after: Option<String>,
    limit: Option<u32>,
    order: Option<RangeOrder>,
) -> StdResult<ListTradeInfoResponse> {
    let ((min, max), limit, order) = range_option(start_after.as_deref(), limit, order)?;

    Ok(ListTradeInfoResponse(
        TRADE_INFOS
            .prefix(&denom_in)
            .range(deps.storage, min, max, order)
            .take(limit)
            .map(|v| {
                let (o, t) = v?;
                Ok(conv_trade_info(denom_in.clone(), o, t))
            })
            .collect::<StdResult<Vec<_>>>()?,
    ))
}

pub fn simulate_mint(
    deps: Deps,
    env: Env,
    amount: Uint128,
    funds: Vec<Coin>,
    time: Option<u64>,
) -> Result<SimulateMintResponse, ContractError> {
    let now_in_sec = env.block.time.seconds();
    let time_in_sec = time.unwrap_or(now_in_sec);

    let fee = FEE.load(deps.storage)?;
    let total_supply = TOTAL_SUPPLY.load(deps.storage)?;

    let index_units = INDEX_UNITS.load(deps.storage)?;
    let index_units = fee
        .streaming_fee
        .map(|mut v| -> StdResult<_> {
            Ok(v.collect(index_units.clone(), time_in_sec, total_supply)?.0)
        })
        .transpose()?
        .unwrap_or(index_units);

    let spent = index_units.calc_require_amount(amount);
    let refund = index_units.calc_refund_amount(funds, amount)?;
    let mint_fee = fee.mint_fee.map(|v| v * amount);
    let mint_send = amount.checked_sub(mint_fee.unwrap_or_default())?;

    Ok(SimulateMintResponse {
        mint_amount: mint_send,
        refund_amount: refund,
        fund_spent: spent,
    })
}

pub fn simulate_burn(
    deps: Deps,
    env: Env,
    amount: Uint128,
    time: Option<u64>,
) -> Result<SimulateBurnResponse, ContractError> {
    let now_in_sec = env.block.time.seconds();
    let time_in_sec = time.unwrap_or(now_in_sec);

    let fee = FEE.load(deps.storage)?;
    let total_supply = TOTAL_SUPPLY.load(deps.storage)?;

    let index_units = INDEX_UNITS.load(deps.storage)?;
    let index_units = fee
        .streaming_fee
        .map(|mut v| -> StdResult<_> {
            Ok(v.collect(index_units.clone(), time_in_sec, total_supply)?.0)
        })
        .transpose()?
        .unwrap_or(index_units);

    let burn_fee = fee.burn_fee.map(|v| v * amount);
    let burn_amount = amount.checked_sub(burn_fee.unwrap_or_default())?;
    let burn_send_amount = index_units.calc_require_amount(burn_amount);

    Ok(SimulateBurnResponse {
        burn_amount, // recognize user to burn entire amount
        redeem_amount: burn_send_amount,
    })
}
