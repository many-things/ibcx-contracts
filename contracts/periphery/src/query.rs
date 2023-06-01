use cosmwasm_std::{coin, Coin, Deps, Env, Uint128};
use ibcx_interface::{
    helpers::IbcCore,
    periphery::{
        SimulateBurnExactAmountInResponse, SimulateMintExactAmountOutResponse, SwapInfosCompact,
    },
};

use crate::{
    error::ContractError,
    msgs::{make_burn_swap_msgs, make_mint_swap_exact_out_msgs},
};

pub fn simulate_mint_exact_amount_in(
    deps: Deps,
    env: Env,
    core_addr: String,
    input_amount: Uint128,
    output_asset: String,
    swap_info: SwapInfosCompact,
) -> Result<SimulateMintExactAmountOutResponse, ContractError> {
    todo!()
}

pub fn simulate_mint_exact_amount_out(
    deps: Deps,
    env: Env,
    core_addr: String,
    output_amount: Uint128,
    input_asset: String,
    swap_info: SwapInfosCompact,
) -> Result<SimulateMintExactAmountOutResponse, ContractError> {
    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_config = core.get_config(&deps.querier, None)?;

    // input & output
    let output = coin(output_amount.u128(), core_config.index_denom);

    let sim_resp = core.simulate_mint(&deps.querier, output.amount, None, None)?;
    let sim_amount_desired = sim_resp.fund_spent;

    let (_, refund) = make_mint_swap_exact_out_msgs(
        &deps.querier,
        &env.contract.address,
        swap_info.into(),
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
    swap_info: SwapInfosCompact,
) -> Result<SimulateBurnExactAmountInResponse, ContractError> {
    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);

    let sim_resp = core.simulate_burn(&deps.querier, input_amount, None)?;
    let expected = sim_resp.redeem_amount.clone();

    let (_, receive) = make_burn_swap_msgs(
        &deps.querier,
        &env.contract.address,
        swap_info.into(),
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
    deps: Deps,
    env: Env,
    core_addr: String,
    output_amount: Coin,
    input_asset: String,
    swap_info: SwapInfosCompact,
) -> Result<SimulateBurnExactAmountInResponse, ContractError> {
    todo!()
}
