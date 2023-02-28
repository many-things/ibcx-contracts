use cosmwasm_std::{to_binary, Coin, Deps, Env, QueryResponse, Uint128};
use ibcx_interface::core::{
    GetConfigResponse, GetFeeResponse, GetPauseInfoResponse, GetPortfolioResponse,
    SimulateBurnResponse, SimulateMintResponse,
};

use crate::{
    error::ContractError,
    state::{assert_units, get_redeem_amounts, get_units, FEE, GOV, PAUSED, TOKEN},
};

pub fn balance(deps: Deps, _env: Env, account: String) -> Result<QueryResponse, ContractError> {
    let token = TOKEN.load(deps.storage)?;

    let resp = deps.querier.query_balance(account, token.denom)?;

    Ok(to_binary(&resp.amount)?)
}

pub fn config(deps: Deps, _env: Env) -> Result<QueryResponse, ContractError> {
    let gov = GOV.load(deps.storage)?;
    let token = TOKEN.load(deps.storage)?;

    Ok(to_binary(&GetConfigResponse {
        gov,
        denom: token.denom,
        reserve_denom: token.reserve_denom,
    })?)
}

pub fn fee(deps: Deps, env: Env, time: Option<u64>) -> Result<QueryResponse, ContractError> {
    let time = time.unwrap_or_else(|| env.block.time.seconds());
    let token = TOKEN.load(deps.storage)?;
    let fee = FEE.load(deps.storage)?;
    let (_, collected) = fee.calculate_streaming_fee(get_units(deps.storage)?, time)?;

    let collected = collected.unwrap_or_default();
    let realized = collected
        .clone()
        .into_iter()
        .map(|(denom, unit)| (denom, token.total_supply * unit))
        .collect::<Vec<_>>();

    Ok(to_binary(&GetFeeResponse {
        collector: fee.collector,
        collected,
        realized,
        mint: fee.mint,
        burn: fee.burn,
        stream: fee.stream,
        stream_last_collected_at: fee.stream_last_collected_at,
    })?)
}

pub fn pause_info(deps: Deps, _env: Env) -> Result<QueryResponse, ContractError> {
    let pause_info = PAUSED.load(deps.storage)?;

    Ok(to_binary(&GetPauseInfoResponse {
        paused: pause_info.paused,
        expires_at: pause_info.expires_at,
    })?)
}

pub fn portfolio(deps: Deps, env: Env) -> Result<QueryResponse, ContractError> {
    let token = TOKEN.load(deps.storage)?;
    let fee = FEE.load(deps.storage)?;

    let now = env.block.time.seconds();
    let assets = get_units(deps.storage)?;
    let (assets, _) = fee.calculate_streaming_fee(assets, now)?;

    Ok(to_binary(&GetPortfolioResponse {
        total_supply: token.total_supply,
        units: assets.clone(),
        assets: get_redeem_amounts(assets, &token.reserve_denom, token.total_supply)?,
    })?)
}

pub fn simulate_mint(
    deps: Deps,
    env: Env,
    amount: Uint128,
    funds: Vec<Coin>,
) -> Result<QueryResponse, ContractError> {
    let token = TOKEN.load(deps.storage)?;
    let fee = FEE.load(deps.storage)?;

    let now = env.block.time.seconds();
    let assets = get_units(deps.storage)?;
    let (assets, _) = fee.calculate_streaming_fee(assets, now)?;

    let amount_spent = get_redeem_amounts(assets.clone(), &token.reserve_denom, amount)?;
    let amount_with_fee = fee.mint.map(|v| amount * v).unwrap_or(amount);
    let refund_amount = if !funds.is_empty() {
        assert_units(assets, funds, amount_with_fee)?
    } else {
        vec![]
    };

    Ok(to_binary(&SimulateMintResponse {
        mint_amount: amount_with_fee, // recognize user to mint entire amount
        refund_amount,
        fund_spent: amount_spent,
    })?)
}

pub fn simulate_burn(
    deps: Deps,
    env: Env,
    amount: Uint128,
) -> Result<QueryResponse, ContractError> {
    let token = TOKEN.load(deps.storage)?;
    let fee = FEE.load(deps.storage)?;

    let now = env.block.time.seconds();
    let assets = get_units(deps.storage)?;
    let (assets, _) = fee.calculate_streaming_fee(assets, now)?;

    let amount_with_fee = fee.burn.map(|v| amount * v).unwrap_or(amount);
    let redeem_amount = get_redeem_amounts(assets, &token.reserve_denom, amount_with_fee)?;

    Ok(to_binary(&SimulateBurnResponse {
        burn_amount: amount, // recognize user to burn entire amount
        redeem_amount,
    })?)
}
