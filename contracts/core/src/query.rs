use cosmwasm_std::{to_binary, Binary, Deps, Env, Uint128};
use ibc_interface::core::{GetConfigResponse, GetPauseInfoResponse, GetPortfolioResponse};

use crate::{
    error::ContractError,
    state::{get_redeem_amounts, GOV, PAUSED, TOKEN},
};

pub fn config(deps: Deps, _env: Env) -> Result<Binary, ContractError> {
    let gov = GOV.load(deps.storage)?;
    let token = TOKEN.load(deps.storage)?;

    Ok(to_binary(&GetConfigResponse {
        gov,
        denom: token.denom,
        decimal: token.decimal,
        reserve_denom: token.reserve_denom,
    })?)
}

pub fn pause_info(deps: Deps, _env: Env) -> Result<Binary, ContractError> {
    let pause_info = PAUSED.load(deps.storage)?;

    Ok(to_binary(&GetPauseInfoResponse {
        paused: pause_info.paused,
        expires_at: pause_info.expires_at,
    })?)
}

pub fn portfolio(deps: Deps, _env: Env) -> Result<Binary, ContractError> {
    let token = TOKEN.load(deps.storage)?;
    let decimal = Uint128::new(10).checked_pow(token.decimal as u32)?;

    Ok(to_binary(&GetPortfolioResponse {
        total_supply: token.total_supply,
        assets: get_redeem_amounts(deps.storage, token.total_supply)?,
        units: get_redeem_amounts(deps.storage, decimal)?,
    })?)
}
