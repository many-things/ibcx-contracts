use cosmwasm_std::{attr, coin, Decimal, DepsMut, Env, MessageInfo, Response, Uint128};

use crate::{
    error::RebalanceError,
    state::{Units, CONFIG, INDEX_UNITS, REBALANCE, RESERVE_UNITS, TRADE_INFOS},
    StdResult,
};

use super::load_units;

fn deflate_index_unit(
    index_units: &mut Units,
    target_denom: &str,
    deflate_unit: Decimal,
) -> StdResult<()> {
    index_units.sub_key(target_denom, deflate_unit)?;
    Ok(())
}

fn inflate_reserve_unit(
    reserve_units: &mut Units,
    inflate_unit: Decimal,
    inflations: Units,
) -> StdResult<()> {
    let mut expand_unit = inflate_unit;
    let mut total_weight = inflations.iter().map(|(_, v)| *v).sum::<Decimal>();
    for (denom, weight) in inflations {
        let sub_unit = expand_unit * weight / total_weight;
        reserve_units.add_key(&denom, sub_unit)?;

        expand_unit = expand_unit.checked_sub(sub_unit)?;
        total_weight = total_weight.checked_sub(weight)?;
    }

    Ok(())
}

pub fn deflate_reserve(
    deps: DepsMut,
    info: MessageInfo,
    reserve_denom: String,
) -> StdResult<Response> {
    // state loader
    let rebalance = REBALANCE.load(deps.storage)?;

    let (index_units, reserve_units, total_supply) = load_units(deps.storage)?;

    let (_, target_unit) = *rebalance.deflation.get_key(&reserve_denom).unwrap();
    let (_, current_unit) = *index_units.get_key(&reserve_denom).unwrap();

    // calculation
    let unit_gap = current_unit.checked_sub(target_unit)?;
    let amount_gap = unit_gap * total_supply;

    // reflect changes
    let mut index_units = index_units;
    deflate_index_unit(&mut index_units, &reserve_denom, unit_gap)?;

    let mut reserve_units = reserve_units;
    inflate_reserve_unit(&mut reserve_units, unit_gap, rebalance.inflation)?;

    // state applier
    INDEX_UNITS.save(deps.storage, &index_units)?;
    RESERVE_UNITS.save(deps.storage, &reserve_units)?;

    // response
    let attrs = vec![
        attr("method", "deflate"),
        attr("executor", info.sender),
        attr("denom", reserve_denom),
        attr("amount_in", amount_gap.to_string()),
        attr("amount_out", amount_gap.to_string()),
        attr("is_reserve", "true"),
    ];

    Ok(Response::new().add_attributes(attrs))
}

// index_unit -> reserve_unit (exact_amount_out)
pub fn deflate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    target_denom: String,
    amount_out: Uint128,
    max_amount_in: Uint128,
) -> StdResult<Response> {
    // state loader
    let config = CONFIG.load(deps.storage)?;
    let rebalance = REBALANCE.load(deps.storage)?;
    let trade_info = TRADE_INFOS.load(deps.storage, (&target_denom, &config.reserve_denom))?;

    let (index_units, reserve_units, total_supply) = load_units(deps.storage)?;

    let (_, target_unit) = *rebalance.deflation.get_key(&target_denom).unwrap();
    let (_, current_unit) = *index_units.get_key(&target_denom).unwrap();

    // check trade_info conditions
    trade_info.assert_cooldown(env.block.time.seconds())?;
    if trade_info.max_trade_amount < amount_out {
        return Err(RebalanceError::trade_error("deflate", "exceeds maximum trade limit").into());
    }

    // calculate amount gap
    let unit_gap = current_unit.checked_sub(target_unit)?;
    let amount_gap = unit_gap * total_supply;

    // simulate how many tokens required
    let sim_amount_in = trade_info.routes.sim_swap_exact_out(
        &deps.querier,
        &env.contract.address,
        coin(amount_out.u128(), &config.reserve_denom),
    )?;

    if amount_gap < sim_amount_in {
        return Err(RebalanceError::trade_error("deflate", "insufficient amoount to swap").into());
    }

    if max_amount_in < sim_amount_in {
        return Err(RebalanceError::trade_error("deflate", "over slippage tolerance").into());
    }

    // deduct & expand stored units
    let index_deduct_unit = Decimal::checked_from_ratio(sim_amount_in, total_supply)?;
    let reserve_expand_unit = Decimal::checked_from_ratio(amount_out, total_supply)?;

    // deduct index unit
    let mut index_units = index_units;
    deflate_index_unit(&mut index_units, &target_denom, index_deduct_unit)?;

    // expand reserve units
    let mut reserve_units = reserve_units;
    inflate_reserve_unit(&mut reserve_units, reserve_expand_unit, rebalance.inflation)?;

    // state applier
    let routes = trade_info.routes.clone();

    INDEX_UNITS.save(deps.storage, &index_units)?;
    RESERVE_UNITS.save(deps.storage, &reserve_units)?;
    TRADE_INFOS.save(
        deps.storage,
        (&target_denom, &config.reserve_denom),
        &trade_info.update_last_traded_at(env.block.time.seconds()),
    )?;

    // response
    let swap_msg = routes.msg_swap_exact_out(
        &env.contract.address,
        &config.reserve_denom,
        amount_out,
        sim_amount_in,
    );

    let attrs = vec![
        attr("method", "deflate"),
        attr("executor", info.sender),
        attr("denom", target_denom),
        attr("amount_in", sim_amount_in.to_string()),
        attr("amount_out", amount_out.to_string()),
        attr("is_reserve", "false"),
    ];

    Ok(Response::new().add_message(swap_msg).add_attributes(attrs))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{attr, testing::mock_info};

    use crate::{
        state::{tests::StateBuilder, Rebalance, INDEX_UNITS, RESERVE_UNITS},
        test::mock_dependencies,
    };

    use super::deflate_reserve;

    #[test]
    fn test_deflate_reserve() {
        let mut deps = mock_dependencies();

        // deflate uatom (unit)     1.0 -> 0.8
        //               (amount) 10000 -> 8000
        // inflate ukrw  (unit)     0.0 -> 0.12
        //               (amount)     0 -> 1200
        // inflate ujpy  (unit)     0.0 -> 0.08
        //               (amount)     0 -> 800

        StateBuilder::default()
            .add_index_unit("uatom", "1.0")
            .empty_reserve_units()
            .with_total_supply(10000)
            .with_rebalance(Rebalance {
                deflation: vec![("uatom", "0.8")].into(),
                inflation: vec![("ukrw", "6"), ("ujpy", "4")].into(),
                ..Default::default()
            })
            .build(deps.as_mut().storage);

        let res = deflate_reserve(
            deps.as_mut(),
            mock_info("manager", &[]),
            "uatom".to_string(),
        )
        .unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "deflate"),
                attr("executor", "manager"),
                attr("denom", "uatom"),
                attr("amount_in", "2000"),
                attr("amount_out", "2000"),
                attr("is_reserve", "true"),
            ]
        );

        assert_eq!(
            INDEX_UNITS.load(deps.as_ref().storage).unwrap(),
            vec![("uatom", "0.8")].into()
        );
        assert_eq!(
            RESERVE_UNITS.load(deps.as_ref().storage).unwrap(),
            vec![("ukrw", "0.12"), ("ujpy", "0.08")].into()
        );
    }

    #[test]
    fn test_deflate() {}
}
