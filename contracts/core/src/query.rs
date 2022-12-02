use cosmwasm_std::{to_binary, Binary, Deps, Env};
use ibc_interface::core::{ConfigResponse, PauseInfoResponse, PortfolioResponse};

use crate::{
    error::ContractError,
    state::{get_assets, get_redeem_assets, GOV, PAUSED, TOKEN},
};

pub fn config(deps: Deps, _env: Env) -> Result<Binary, ContractError> {
    let gov = GOV.load(deps.storage)?;
    let token = TOKEN.load(deps.storage)?;

    Ok(to_binary(&ConfigResponse {
        gov,
        denom: token.denom,
        reserve_denom: token.reserve_denom,
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
    let token = TOKEN.load(deps.storage)?;

    Ok(to_binary(&PortfolioResponse {
        total_supply: token.total_supply,
        assets: get_redeem_assets(deps.storage, token.total_supply)?,
        units: get_assets(deps.storage)?,
    })?)
}
