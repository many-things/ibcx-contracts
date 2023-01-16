use cosmwasm_std::{to_binary, Coin, Deps, Env, QueryResponse, Uint128};
use ibcx_interface::core::{
    FeeResponse, GetConfigResponse, GetPauseInfoResponse, GetPortfolioResponse,
    SimulateBurnResponse, SimulateMintResponse,
};

use crate::{
    error::ContractError,
    state::{assert_assets, get_assets, get_redeem_amounts, FEE, GOV, PAUSED, TOKEN},
};

pub fn balance(deps: Deps, _env: Env, account: String) -> Result<QueryResponse, ContractError> {
    let token = TOKEN.load(deps.storage)?;

    let resp = deps.querier.query_balance(account, token.denom)?;

    Ok(to_binary(&resp.amount)?)
}

pub fn config(deps: Deps, _env: Env) -> Result<QueryResponse, ContractError> {
    let gov = GOV.load(deps.storage)?;
    let token = TOKEN.load(deps.storage)?;
    let fee = FEE.load(deps.storage)?;

    Ok(to_binary(&GetConfigResponse {
        gov,
        denom: token.denom,
        reserve_denom: token.reserve_denom,
        fee_strategy: FeeResponse {
            collector: fee.collector,
            mint: fee.mint,
            burn: fee.burn,
            stream: fee.stream,
            stream_last_collected_at: fee.stream_last_collected_at,
        },
    })?)
}

pub fn pause_info(deps: Deps, _env: Env) -> Result<QueryResponse, ContractError> {
    let pause_info = PAUSED.load(deps.storage)?;

    Ok(to_binary(&GetPauseInfoResponse {
        paused: pause_info.paused,
        expires_at: pause_info.expires_at,
    })?)
}

pub fn portfolio(deps: Deps, _env: Env) -> Result<QueryResponse, ContractError> {
    let token = TOKEN.load(deps.storage)?;

    Ok(to_binary(&GetPortfolioResponse {
        total_supply: token.total_supply,
        assets: get_redeem_amounts(deps.storage, token.total_supply)?,
        units: get_assets(deps.storage)?,
    })?)
}

pub fn simulate_mint(
    deps: Deps,
    _env: Env,
    amount: Uint128,
    funds: Vec<Coin>,
) -> Result<QueryResponse, ContractError> {
    let refund_amount = assert_assets(deps.storage, funds, amount)?;

    Ok(to_binary(&SimulateMintResponse {
        mint_amount: amount,
        refund_amount,
    })?)
}

pub fn simulate_burn(
    deps: Deps,
    _env: Env,
    amount: Uint128,
) -> Result<QueryResponse, ContractError> {
    let redeem_amount = get_redeem_amounts(deps.storage, amount)?;

    Ok(to_binary(&SimulateBurnResponse {
        burn_amount: amount,
        redeem_amount,
    })?)
}
