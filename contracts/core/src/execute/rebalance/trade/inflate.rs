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
        return Err(RebalanceError::trade_error("inflate", "insufficient amount to swap").into());
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
mod tests {
    use cosmwasm_std::{
        attr,
        testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR},
        Addr, Attribute, SubMsg, Timestamp,
    };
    use ibcx_interface::types::SwapRoutes;

    use crate::{
        error::RebalanceError,
        state::{tests::StateBuilder, Config, TradeInfo, INDEX_UNITS, RESERVE_UNITS, TRADE_INFOS},
        test::mock_dependencies,
    };

    use super::{inflate, inflate_reserve};

    fn event_builder(
        sender: &str,
        denom: &str,
        amount_in: u128,
        amount_out: u128,
        is_reserve: bool,
    ) -> Vec<Attribute> {
        vec![
            attr("method", "inflate"),
            attr("executor", sender),
            attr("denom", denom),
            attr("amount_in", amount_in.to_string()),
            attr("amount_out", amount_out.to_string()),
            attr("is_reserve", if is_reserve { "true" } else { "false" }),
        ]
    }

    #[test]
    fn test_inflate_reserve() {
        let mut deps = mock_dependencies();

        // deflate uatom (reserve)  1.2 -> 0.0
        // inflate uatom (non-rsrv) 0.8 -> 2.0
        //               (amount)   400
        StateBuilder::default()
            .add_index_unit("uatom", "0.8")
            .add_reserve_unit("uatom", "1.2")
            .with_total_supply(10000)
            .build(deps.as_mut().storage);

        let res = inflate_reserve(
            deps.as_mut(),
            mock_info("manager", &[]),
            "uatom".to_string(),
        )
        .unwrap();
        assert_eq!(
            res.attributes,
            event_builder("manager", "uatom", 12000, 12000, true)
        );

        assert_eq!(
            INDEX_UNITS.load(deps.as_ref().storage).unwrap(),
            vec![("uatom", "2.0")].into()
        );
        assert_eq!(
            RESERVE_UNITS.load(deps.as_ref().storage).unwrap(),
            vec![("uatom", "0.0")].into()
        );
    }

    #[test]
    fn test_inflate() {
        let std_time = mock_env().block.time.seconds();
        let mut deps = mock_dependencies();

        // 2 : 1
        deps.querier.stargate.register_sim_swap_exact_in("0.5");

        // deflate uosmo (reserve)  1.0 -> 0.0
        //=========================================
        // trade uosmo -> ukrw
        //=========================================
        // inflate ukrw (unit)   0.8 -> 2.0
        //              (amount) 400
        let routes: SwapRoutes = vec![(0, "ukrw")].into();
        let cooldown = 60;
        let max_reserve_amount = 10000u128;
        let max_trade_amount = 20000u128;
        let builder = StateBuilder::default()
            .with_config(Config {
                reserve_denom: "uosmo".to_string(),
                ..Default::default()
            })
            .with_total_supply(10000)
            .empty_index_units()
            .add_reserve_unit("ukrw", "1.0")
            .add_trade_info(
                "uosmo",
                "ukrw",
                TradeInfo {
                    routes: routes.clone(),
                    cooldown,
                    max_trade_amount: max_trade_amount.into(),
                    last_traded_at: Some(std_time - cooldown),
                },
            );

        let cases = [
            (
                "manager",
                std_time,
                max_reserve_amount,
                max_reserve_amount / 2,
                Ok((vec![("ukrw", "0.5")].into(), vec![("ukrw", "0.0")].into())),
            ),
            (
                "manager",
                std_time,
                max_trade_amount + 1,
                (max_trade_amount + 1) / 2,
                Err(RebalanceError::trade_error("inflate", "exceeds maximum trade limit").into()),
            ),
            (
                "manager",
                std_time,
                max_reserve_amount + 1,
                (max_reserve_amount + 1) / 2,
                Err(RebalanceError::trade_error("inflate", "insufficient amount to swap").into()),
            ),
            (
                "manager",
                std_time,
                max_reserve_amount,
                max_reserve_amount / 2 + 1,
                Err(RebalanceError::trade_error("inflate", "over slippage allowance").into()),
            ),
        ];

        for (sender, time_in_sec, amount_in, min_amount_out, expected) in cases {
            builder.clone().build(deps.as_mut().storage);

            let mut env = mock_env();
            env.block.time = Timestamp::from_seconds(time_in_sec);

            let res = inflate(
                deps.as_mut(),
                env.clone(),
                mock_info(sender, &[]),
                "ukrw".to_string(),
                amount_in.into(),
                min_amount_out.into(),
            );
            match res {
                Ok(res) => {
                    let (index_units, reserve_units) = expected.unwrap();

                    assert_eq!(
                        res.messages,
                        vec![SubMsg::new(routes.msg_swap_exact_in(
                            &Addr::unchecked(MOCK_CONTRACT_ADDR),
                            "uosmo",
                            amount_in.into(),
                            min_amount_out.into()
                        ))]
                    );

                    assert_eq!(
                        res.attributes,
                        event_builder(sender, "ukrw", amount_in, min_amount_out, false)
                    );

                    assert_eq!(
                        INDEX_UNITS.load(deps.as_ref().storage).unwrap(),
                        index_units
                    );
                    assert_eq!(
                        RESERVE_UNITS.load(deps.as_ref().storage).unwrap(),
                        reserve_units
                    );
                    assert_eq!(
                        TRADE_INFOS
                            .load(deps.as_ref().storage, ("uosmo", "ukrw"))
                            .unwrap()
                            .last_traded_at,
                        Some(env.block.time.seconds())
                    )
                }
                Err(err) => assert_eq!(err, expected.unwrap_err()),
            }
        }
    }
}
