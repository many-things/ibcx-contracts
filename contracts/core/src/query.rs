// use cosmwasm_std::{to_binary, Coin, Deps, Env, QueryResponse, Uint128};
// use ibcx_interface::core::{
//     GetConfigResponse, GetFeeResponse, GetPauseInfoResponse, GetPortfolioResponse,
//     SimulateBurnResponse, SimulateMintResponse,
// };

// use crate::{
//     error::ContractError,
//     state::{Config, PauseInfo, CONFIG, FEE, TOTAL_SUPPLY},
// };

// pub fn balance(deps: Deps, _env: Env, account: String) -> Result<Uint128, ContractError> {
//     let Config { index_denom, .. } = CONFIG.load(deps.storage)?;

//     let resp = deps.querier.query_balance(account, index_denom)?;

//     Ok(resp.amount)
// }

// pub fn config(deps: Deps, _env: Env) -> Result<GetConfigResponse, ContractError> {
//     let Config {
//         gov,
//         index_denom,
//         reserve_denom,
//         ..
//     } = CONFIG.load(deps.storage)?;

//     Ok(GetConfigResponse {
//         gov,
//         index_denom,
//         reserve_denom,
//     })
// }

// pub fn fee(deps: Deps, env: Env, time: Option<u64>) -> Result<GetFeeResponse, ContractError> {
//     let time = time.unwrap_or_else(|| env.block.time.seconds());
//     let config = CONFIG.load(deps.storage)?;
//     let fee = FEE.load(deps.storage)?;
//     let (_, collected) = fee.calculate_streaming_fee(get_units(deps.storage)?, time)?;

//     let collected = collected.unwrap_or_default();
//     let realized = collected
//         .clone()
//         .into_iter()
//         .map(|(denom, unit)| (denom, token.total_supply * unit))
//         .collect::<Vec<_>>();

//     Ok(GetFeeResponse {
//         collector: fee.collector,
//         collected,
//         realized,
//         mint: fee.mint,
//         burn: fee.burn,
//         stream: fee.stream,
//         stream_last_collected_at: fee.stream_last_collected_at,
//     })
// }

// pub fn pause_info(deps: Deps, _env: Env) -> Result<GetPauseInfoResponse, ContractError> {
//     let Config {
//         paused: PauseInfo { paused, expires_at },
//         ..
//     } = CONFIG.load(deps.storage)?;

//     Ok(GetPauseInfoResponse { paused, expires_at })
// }

// pub fn portfolio(deps: Deps, env: Env) -> Result<QueryResponse, ContractError> {
//     let config = CONFIG.load(deps.storage)?;
//     let fee = FEE.load(deps.storage)?;
//     let total_supply = TOTAL_SUPPLY.load(deps.storage)?;

//     let now = env.block.time.seconds();
//     let assets = get_units(deps.storage)?;
//     let (assets, _) = fee.calculate_streaming_fee(assets, now)?;

//     Ok(to_binary(&GetPortfolioResponse {
//         total_supply,
//         units: assets.clone(),
//         assets: get_redeem_amounts(assets, &token.reserve_denom, token.total_supply)?,
//     })?)
// }

// pub fn simulate_mint(
//     deps: Deps,
//     env: Env,
//     amount: Uint128,
//     funds: Vec<Coin>,
// ) -> Result<QueryResponse, ContractError> {
//     let token = TOKEN.load(deps.storage)?;
//     let fee = FEE.load(deps.storage)?;

//     let now = env.block.time.seconds();
//     let assets = get_units(deps.storage)?;
//     let (assets, _) = fee.calculate_streaming_fee(assets, now)?;

//     let amount_spent = get_redeem_amounts(assets.clone(), &token.reserve_denom, amount)?;
//     let amount_with_fee = fee.mint.map(|v| amount * v).unwrap_or(amount);
//     let refund_amount = if !funds.is_empty() {
//         assert_units(assets, funds, amount_with_fee)?
//     } else {
//         vec![]
//     };

//     Ok(to_binary(&SimulateMintResponse {
//         mint_amount: amount_with_fee, // recognize user to mint entire amount
//         refund_amount,
//         fund_spent: amount_spent,
//     })?)
// }

// pub fn simulate_burn(
//     deps: Deps,
//     env: Env,
//     amount: Uint128,
// ) -> Result<QueryResponse, ContractError> {
//     let token = TOKEN.load(deps.storage)?;
//     let fee = FEE.load(deps.storage)?;

//     let now = env.block.time.seconds();
//     let assets = get_units(deps.storage)?;
//     let (assets, _) = fee.calculate_streaming_fee(assets, now)?;

//     let amount_with_fee = fee.burn.map(|v| amount * v).unwrap_or(amount);
//     let redeem_amount = get_redeem_amounts(assets, &token.reserve_denom, amount_with_fee)?;

//     Ok(to_binary(&SimulateBurnResponse {
//         burn_amount: amount, // recognize user to burn entire amount
//         redeem_amount,
//     })?)
// }
