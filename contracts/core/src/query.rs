use cosmwasm_std::{Coin, Deps, Env, Timestamp, Uint128};
use ibcx_interface::core::{
    GetConfigResponse, GetFeeResponse, GetPauseInfoResponse, GetPortfolioResponse,
    SimulateBurnResponse, SimulateMintResponse, StreamingFeeResponse,
};

use crate::{
    error::ContractError,
    state::{Config, PauseInfo, CONFIG, FEE, INDEX_UNITS, TOTAL_SUPPLY},
    StdResult,
};

pub fn get_balance(deps: Deps, _env: Env, account: String) -> StdResult<Uint128> {
    let Config { index_denom, .. } = CONFIG.load(deps.storage)?;

    let resp = deps.querier.query_balance(account, index_denom)?;

    Ok(resp.amount)
}

pub fn get_config(deps: Deps, _env: Env) -> StdResult<GetConfigResponse> {
    let Config {
        gov,
        index_denom,
        reserve_denom,
        ..
    } = CONFIG.load(deps.storage)?;

    Ok(GetConfigResponse {
        gov,
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

pub fn get_pause_info(deps: Deps, env: Env, time: Option<u64>) -> StdResult<GetPauseInfoResponse> {
    let now_in_sec = env.block.time.seconds();
    let time_in_sec = time.unwrap_or(now_in_sec);

    let Config { paused, .. } = CONFIG.load(deps.storage)?;

    let mut temp_env = env;

    temp_env.block.time = Timestamp::from_seconds(time_in_sec);

    let PauseInfo { paused, expires_at } = paused.refresh(&temp_env)?;

    Ok(GetPauseInfoResponse { paused, expires_at })
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
    let burn_send_amount = index_units.calc_require_amount(amount);

    Ok(SimulateBurnResponse {
        burn_amount, // recognize user to burn entire amount
        redeem_amount: burn_send_amount,
    })
}
