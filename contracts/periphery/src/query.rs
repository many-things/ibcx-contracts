use cosmwasm_std::{coin, Coin, Deps, Env, Uint128};
use ibcx_interface::{
    helpers::IbcCore,
    periphery::{
        extract_pool_ids, SimulateBurnExactAmountInResponse, SimulateMintExactAmountOutResponse,
        SwapInfo,
    },
};

use crate::{
    error::ContractError,
    msgs::{make_burn_swap_msgs, make_mint_swap_exact_out_msgs},
    pool::query_pools,
    sim::search_efficient,
};

pub fn simulate_mint_exact_amount_in(
    deps: Deps,
    _env: Env,
    core_addr: String,
    input_asset: Coin,
    swap_info: Vec<SwapInfo>,
) -> Result<SimulateMintExactAmountOutResponse, ContractError> {
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let _core_config = core.get_config(&deps.querier, None)?;
    let core_portfolio = core.get_portfolio(&deps.querier, None)?;

    let pool_ids = extract_pool_ids(swap_info.clone());
    let pools = query_pools(&deps, pool_ids)?;

    let (token_in, token_out, _) = search_efficient(
        &deps,
        &core_portfolio.units,
        input_asset.clone(),
        None,
        &pools,
        &swap_info,
    )?;

    Ok(SimulateMintExactAmountOutResponse {
        mint_amount: token_out,
        mint_spend_amount: vec![],
        swap_result_amount: coin(token_in.u128(), input_asset.denom),
    })
}

pub fn simulate_mint_exact_amount_out(
    deps: Deps,
    env: Env,
    core_addr: String,
    output_amount: Uint128,
    input_asset: String,
    swap_info: Vec<SwapInfo>,
) -> Result<SimulateMintExactAmountOutResponse, ContractError> {
    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_config = core.get_config(&deps.querier, None)?;

    // input & output
    let output = coin(output_amount.u128(), core_config.index_denom);

    let sim_resp = core.simulate_mint(&deps.querier, output.amount, None, None)?;
    let sim_amount_desired = sim_resp.fund_spent;

    let (_, refund) = make_mint_swap_exact_out_msgs(
        &deps,
        &env.contract.address,
        swap_info,
        sim_amount_desired.clone(),
        &coin(u64::MAX as u128, &input_asset),
    )?;

    Ok(SimulateMintExactAmountOutResponse {
        mint_amount: output.amount,
        mint_spend_amount: sim_amount_desired,
        swap_result_amount: coin(
            Uint128::from(u64::MAX).checked_sub(refund)?.u128(),
            input_asset,
        ),
    })
}

pub fn simulate_burn_exact_amount_in(
    deps: Deps,
    env: Env,
    core_addr: String,
    input_amount: Uint128,
    output_asset: String,
    swap_info: Vec<SwapInfo>,
) -> Result<SimulateBurnExactAmountInResponse, ContractError> {
    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);

    let sim_resp = core.simulate_burn(&deps.querier, input_amount, None)?;
    let expected = sim_resp.redeem_amount.clone();

    let (_, receive) = make_burn_swap_msgs(
        &deps,
        &env.contract.address,
        swap_info,
        expected,
        &coin(Uint128::zero().u128(), &output_asset),
    )?;

    Ok(SimulateBurnExactAmountInResponse {
        burn_amount: sim_resp.burn_amount,
        burn_redeem_amount: sim_resp.redeem_amount,
        swap_result_amount: coin(receive.u128(), &output_asset),
    })
}

pub fn simulate_burn_exact_amount_out(
    _deps: Deps,
    _env: Env,
    _core_addr: String,
    _output_asset: Coin,
    _swap_info: Vec<SwapInfo>,
) -> Result<SimulateBurnExactAmountInResponse, ContractError> {
    todo!()
}
