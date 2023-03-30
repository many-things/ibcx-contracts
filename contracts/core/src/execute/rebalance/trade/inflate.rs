use cosmwasm_std::{attr, coin, Decimal, DepsMut, Env, MessageInfo, Response, Uint128};

use crate::{
    error::RebalanceError,
    state::{CONFIG, INDEX_UNITS, RESERVE_UNITS, TRADE_INFOS},
    StdResult,
};

use super::load_units;

// in the case of reserve denom, we can directly inflate the unit
pub fn inflate_reserve(
    deps: DepsMut,
    info: MessageInfo,
    reserve_denom: String,
) -> StdResult<Response> {
    // load dependencies
    let (index_units, reserve_units, total_supply) = load_units(deps.storage)?;

    let (_, reserve_unit) = *reserve_units.get_key(&reserve_denom).unwrap();
    let reserve_amount = reserve_unit * total_supply;

    let mut index_units = index_units;
    index_units.add_key(&reserve_denom, reserve_unit)?;

    let mut reserve_units = reserve_units;
    reserve_units.sub_key(&reserve_denom, reserve_unit)?;

    // state applier
    INDEX_UNITS.save(deps.storage, &index_units)?;
    RESERVE_UNITS.save(deps.storage, &reserve_units)?;

    // 1:1 conversion is done
    Ok(Response::new().add_attributes(vec![
        attr("method", "inflate"),
        attr("executor", info.sender),
        attr("denom", reserve_denom),
        attr("amount_in", reserve_amount.to_string()),
        attr("amount_out", reserve_amount.to_string()),
        attr("is_reserve", "true"),
    ]))
}

// reserve_unit -> index_unit (exact_amount_in)
pub fn inflate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    target_denom: String,
    amount_in: Uint128,
    min_amount_out: Uint128,
) -> StdResult<Response> {
    // state loader
    let config = CONFIG.load(deps.storage)?;
    let trade_info = TRADE_INFOS.load(deps.storage, (&config.reserve_denom, &target_denom))?;

    let (index_units, reserve_units, total_supply) = load_units(deps.storage)?;

    let (_, reserve_unit) = *reserve_units.get_key(&target_denom).unwrap();

    // check trade_info conditions
    trade_info.assert_cooldown(env.block.time.seconds())?;
    if trade_info.max_trade_amount < amount_in {
        return Err(RebalanceError::trade_error("inflate", "exceeds maximum trade limit").into());
    }

    // calculate available amount to swap
    let reserve_amount = reserve_unit * total_supply;
    if reserve_amount < amount_in {
        return Err(RebalanceError::trade_error("inflate", "insufficient amoount to swap").into());
    }

    let sim_amount_out = trade_info.routes.sim_swap_exact_in(
        &deps.querier,
        &env.contract.address,
        coin(amount_in.u128(), &config.reserve_denom),
    )?;
    if sim_amount_out < min_amount_out {
        return Err(RebalanceError::trade_error("inflate", "over slippage allowance").into());
    }

    // deduct & expand stored units
    let reserve_deduct_unit = Decimal::checked_from_ratio(amount_in, total_supply)?;
    let index_expand_unit = Decimal::checked_from_ratio(sim_amount_out, total_supply)?;

    // expand index unit
    let mut index_units = index_units;
    index_units.add_key(&target_denom, index_expand_unit)?;

    let mut reserve_units = reserve_units;
    reserve_units.sub_key(&target_denom, reserve_deduct_unit)?;

    // state applier
    let routes = trade_info.routes.clone();

    INDEX_UNITS.save(deps.storage, &index_units)?;
    RESERVE_UNITS.save(deps.storage, &reserve_units)?;
    TRADE_INFOS.save(
        deps.storage,
        (&config.reserve_denom, &target_denom),
        &trade_info.update_last_traded_at(env.block.time.seconds()),
    )?;

    // response
    let swap_msg = routes.msg_swap_exact_in(
        &env.contract.address,
        &config.reserve_denom,
        amount_in,
        sim_amount_out,
    );

    let attrs = vec![
        attr("method", "inflate"),
        attr("executor", info.sender),
        attr("denom", target_denom),
        attr("amount_in", amount_in.to_string()),
        attr("amount_out", sim_amount_out.to_string()),
        attr("is_reserve", "false"),
    ];

    Ok(Response::new().add_message(swap_msg).add_attributes(attrs))
}

#[cfg(test)]
mod tests {}
