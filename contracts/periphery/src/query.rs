use cosmwasm_std::{coin, Coin, Deps, Env, Uint128};
use ibcx_interface::{
    helpers::IbcCore,
    periphery::{
        extract_pool_ids, SimulateBurnExactAmountInResponse, SimulateBurnExactAmountOutResponse,
        SimulateMintExactAmountOutResponse, SwapInfo,
    },
};
use ibcx_pool::{query_pools, Simulator};

use crate::{deduct_fee, error::ContractError, expand_fee, make_unit_converter};

pub fn simulate_mint_exact_amount_in(
    deps: Deps,
    _env: Env,
    core_addr: String,
    input_asset: Coin,
    swap_info: Vec<SwapInfo>,
) -> Result<SimulateMintExactAmountOutResponse, ContractError> {
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_fee = core.get_fee(&deps.querier, None)?;
    let core_portfolio = core.get_portfolio(&deps.querier, None)?;

    let pool_ids = extract_pool_ids(swap_info.clone());
    let pools = query_pools(&deps, pool_ids)?;

    let sim = Simulator::new(&deps, &pools, &swap_info, &core_portfolio.units);
    let sim_res = sim
        .estimate_index_for_input(input_asset.clone(), None, None, None)?
        .max;

    // apply mint fee
    let mint_amount =
        (sim_res.est_min_token_out * expand_fee(core_fee.mint_fee)?) - Uint128::new(100); // FIXME: hacky calibration

    let conv = make_unit_converter(mint_amount);
    let mut mint_spend_amount: Vec<_> = core_portfolio.units.into_iter().map(conv).collect();
    mint_spend_amount.sort_by(|a, b| a.denom.cmp(&b.denom));

    Ok(SimulateMintExactAmountOutResponse {
        mint_amount,
        mint_spend_amount,
        swap_result_amount: coin(sim_res.max_token_in.u128(), input_asset.denom),
    })
}

pub fn simulate_mint_exact_amount_out(
    deps: Deps,
    _env: Env,
    core_addr: String,
    index_amount: Uint128,
    input_asset: String,
    swap_info: Vec<SwapInfo>,
) -> Result<SimulateMintExactAmountOutResponse, ContractError> {
    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_fee = core.get_fee(&deps.querier, None)?;
    let core_portfolio = core.get_portfolio(&deps.querier, None)?;

    let pool_ids = extract_pool_ids(swap_info.clone());
    let pools = query_pools(&deps, pool_ids)?;

    // apply mint fee
    let mint_amount = index_amount * deduct_fee(core_fee.mint_fee)?;

    let conv = make_unit_converter(mint_amount);
    let mut mint_spend_amount: Vec<_> =
        core_portfolio.units.clone().into_iter().map(conv).collect();
    mint_spend_amount.sort_by(|a, b| a.denom.cmp(&b.denom));

    let sim = Simulator::new(&deps, &pools, &swap_info, &core_portfolio.units);
    let sim_res = sim.estimate_input_for_index(&input_asset, mint_amount)?;

    Ok(SimulateMintExactAmountOutResponse {
        mint_amount,
        mint_spend_amount,
        swap_result_amount: coin(sim_res.total_input.u128(), input_asset),
    })
}

pub fn simulate_burn_exact_amount_in(
    deps: Deps,
    _env: Env,
    core_addr: String,
    index_amount: Uint128,
    output_asset: String,
    swap_info: Vec<SwapInfo>,
) -> Result<SimulateBurnExactAmountInResponse, ContractError> {
    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_fee = core.get_fee(&deps.querier, None)?;
    let core_portfolio = core.get_portfolio(&deps.querier, None)?;

    let pool_ids = extract_pool_ids(swap_info.clone());
    let pools = query_pools(&deps, pool_ids)?;

    // apply burn fee before simulating
    let burn_amount = index_amount * deduct_fee(core_fee.burn_fee)?;

    let conv = make_unit_converter(burn_amount);
    let mut burn_redeem_amount: Vec<_> =
        core_portfolio.units.clone().into_iter().map(conv).collect();
    burn_redeem_amount.sort_by(|a, b| a.denom.cmp(&b.denom));

    let sim = Simulator::new(&deps, &pools, &swap_info, &core_portfolio.units);
    let sim_res = sim.estimate_output_for_index(burn_amount, &output_asset)?;

    Ok(SimulateBurnExactAmountInResponse {
        burn_amount,
        burn_redeem_amount,
        swap_result_amount: coin(sim_res.total_output.u128(), &output_asset),
    })
}

pub fn simulate_burn_exact_amount_out(
    deps: Deps,
    _env: Env,
    core_addr: String,
    output_asset: Coin,
    swap_info: Vec<SwapInfo>,
) -> Result<SimulateBurnExactAmountOutResponse, ContractError> {
    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_fee = core.get_fee(&deps.querier, None)?;
    let core_portfolio = core.get_portfolio(&deps.querier, None)?;

    let pool_ids = extract_pool_ids(swap_info.clone());
    let pools = query_pools(&deps, pool_ids)?;

    let sim = Simulator::new(&deps, &pools, &swap_info, &core_portfolio.units);
    let sim_res = sim
        .estimate_index_for_output(output_asset.clone(), None, None, None)?
        .min;

    // apply burn fee after simulation
    let burn_amount = sim_res.est_min_token_in * expand_fee(core_fee.burn_fee)?;

    let conv = make_unit_converter(burn_amount);
    let mut burn_redeem_amount: Vec<_> = core_portfolio.units.into_iter().map(conv).collect();
    burn_redeem_amount.sort_by(|a, b| a.denom.cmp(&b.denom));

    Ok(SimulateBurnExactAmountOutResponse {
        burn_amount,
        burn_redeem_amount,
        swap_result_amount: coin(sim_res.max_token_out.u128(), output_asset.denom),
    })
}
