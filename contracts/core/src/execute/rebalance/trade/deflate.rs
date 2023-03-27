use cosmwasm_std::{attr, coin, Decimal, DepsMut, Env, MessageInfo, Response, Storage, Uint128};

use crate::{
    error::ContractError,
    state::{Rebalance, RESERVE_BUFFER, RESERVE_DENOM, TOKEN, TRADE_INFOS, UNITS},
};

use super::get_and_check_rebalance;

// apply distributed amount - is divided by its weight - to each portfolio assets
fn distribute_after_deflate(
    storage: &mut dyn Storage,
    rebalance: Rebalance,
    amount: Uint128,
) -> Result<(), ContractError> {
    let total_weight = rebalance
        .inflation
        .iter()
        .fold(Decimal::zero(), |acc, (_, w)| acc.checked_add(*w).unwrap());

    for (denom, weight) in rebalance.inflation {
        let mut buffer = RESERVE_BUFFER
            .load(storage, denom.clone())
            .unwrap_or_default();
        buffer = buffer.checked_add(weight.checked_div(total_weight)? * amount)?;
        RESERVE_BUFFER.save(storage, denom, &buffer)?;
    }

    Ok(())
}

// in the case of reserve denom, we don't need to swap. just apply it
pub fn deflate_reserve(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // load dependencies
    let token = TOKEN.load(deps.storage)?;
    let rebalance = get_and_check_rebalance(deps.storage, &info.sender)?;

    // fetch deflation info (especially target unit) from rebalance
    let (_, target_unit) = rebalance.get_deflation(&denom)?;

    // fetch current unit about actual denom & reserved denom
    let origin_unit = UNITS.load(deps.storage, denom.clone())?;
    let reserve_unit = UNITS
        .load(deps.storage, RESERVE_DENOM.to_string())
        .unwrap_or_default();

    // calculate swap unit & check if it is valid
    let swap_unit = Decimal::checked_from_ratio(amount, token.total_supply)?;
    if origin_unit.checked_sub(swap_unit)? < target_unit {
        return Err(ContractError::InvalidArgument(format!(
            "exceed max trade amount: {amount:?}",
        )));
    }

    // apply new units
    UNITS.save(
        deps.storage,
        denom.clone(),
        &origin_unit.checked_sub(swap_unit)?,
    )?;
    UNITS.save(
        deps.storage,
        RESERVE_DENOM.to_string(),
        &reserve_unit.checked_add(swap_unit)?,
    )?;

    // distribute to buffer
    distribute_after_deflate(deps.storage, rebalance, amount)?;

    // 1:1 conversion is done
    Ok(Response::new().add_attributes(vec![
        attr("method", "deflate_reserve"),
        attr("executor", info.sender),
        attr("denom", denom),
        attr("swap_unit", swap_unit.to_string()),
        attr("amount", amount),
    ]))
}

pub fn deflate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denom: String,
    amount: Uint128,
    max_amount_in: Uint128,
) -> Result<Response, ContractError> {
    // load dependencies
    let token = TOKEN.load(deps.storage)?;
    let rebalance = get_and_check_rebalance(deps.storage, &info.sender)?;

    // fetch deflation info (especially target unit) from rebalance
    let (_, target_unit) = rebalance.get_deflation(&denom)?;

    // load trade_info
    let mut trade_info = TRADE_INFOS.load(deps.storage, denom.clone())?;
    trade_info.checked_update_cooldown(env.block.time.seconds())?;
    if trade_info.max_trade_amount < amount {
        return Err(ContractError::InvalidArgument(format!(
            "exceed max trade amount: {}",
            trade_info.max_trade_amount
        )));
    }

    // load stored units
    let origin_unit = UNITS.load(deps.storage, denom.clone())?;
    let reserve_unit = UNITS
        .load(deps.storage, RESERVE_DENOM.to_string())
        .unwrap_or_default();

    let amount_gap = origin_unit.checked_sub(target_unit)? * token.total_supply;
    if amount_gap < amount {
        return Err(ContractError::InvalidArgument(format!(
            "insufficient amount: {amount_gap}",
        )));
    }

    // simulate
    let amount_in = trade_info.routes.sim_swap_exact_out(
        &deps.querier,
        &env.contract.address,
        coin(amount.u128(), &denom),
    )?;
    if max_amount_in < amount_in {
        return Err(ContractError::OverSlippageAllowance {});
    }

    // deduct & expand stored units
    let deduct_unit = Decimal::checked_from_ratio(amount_in, token.total_supply)?;
    let expand_unit = Decimal::checked_from_ratio(amount, token.total_supply)?;

    UNITS.save(
        deps.storage,
        denom.clone(),
        &(origin_unit.checked_sub(deduct_unit)?),
    )?;
    UNITS.save(
        deps.storage,
        RESERVE_DENOM.to_string(),
        &(reserve_unit.checked_add(expand_unit)?),
    )?;

    // distribute to buffer
    distribute_after_deflate(deps.storage, rebalance, amount)?;

    // build response
    Ok(Response::new()
        .add_message(trade_info.routes.msg_swap_exact_out(
            &env.contract.address,
            &denom,
            amount,
            amount_in,
        ))
        .add_attributes(vec![
            attr("method", "deflate"),
            attr("executor", info.sender),
            attr("denom", denom),
            attr("amount_in", amount_in),
            attr("amount_out", amount),
        ]))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{
        testing::{mock_env, mock_info},
        to_binary,
    };
    use ibcx_interface::types::{SwapRoute, SwapRoutes};
    use osmosis_std::types::osmosis::poolmanager::v1beta1::EstimateSwapExactAmountOutResponse;

    use crate::{
        execute::rebalance::test::setup,
        test::{default_trade_info, mock_dependencies, register_units},
    };

    use super::*;

    fn deflate(
        deps: DepsMut,
        sender: &str,
        denom: &str,
        amount: u128,
        max_amount_in: u128,
    ) -> Result<Response, ContractError> {
        super::deflate(
            deps,
            mock_env(),
            mock_info(sender, &[]),
            denom.to_string(),
            amount.into(),
            max_amount_in.into(),
        )
    }

    fn deflate_reserve(
        deps: DepsMut,
        sender: &str,
        denom: &str,
        amount: u128,
    ) -> Result<Response, ContractError> {
        super::deflate_reserve(
            deps,
            mock_info(sender, &[]),
            denom.to_string(),
            amount.into(),
        )
    }

    fn assert_state(
        storage: &dyn Storage,
        from: &str,
        deduct: &str,
        expand: &str,
        rebalance: Rebalance,
        distribute: u128,
    ) {
        // assert state - assets
        assert_eq!(
            UNITS.load(storage, from.to_string()).unwrap(),
            Decimal::from_str(deduct).unwrap()
        );
        assert_eq!(
            UNITS.load(storage, RESERVE_DENOM.to_string()).unwrap(),
            Decimal::from_str(expand).unwrap()
        );

        // assert state - distribution (reserve buffer)
        let total_weight = rebalance
            .inflation
            .iter()
            .fold(Decimal::zero(), |acc, (_, v)| acc + v);

        for (denom, weight) in rebalance.inflation {
            let expected = weight.checked_div(total_weight).unwrap() * Uint128::new(distribute);
            assert_eq!(RESERVE_BUFFER.load(storage, denom).unwrap(), expected);
        }
    }

    #[test]
    fn test_deflate() {
        let mut deps = mock_dependencies();

        deps.querier.stargate.register(
            "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountOut",
            |_| {
                to_binary(&EstimateSwapExactAmountOutResponse {
                    token_in_amount: "10000".to_string(),
                })
                .into()
            },
        );

        let (rebalance, _) = setup(
            deps.as_mut().storage,
            1,
            &[("ukrw", "1.0")],
            &[("uusd", "2"), ("ujpy", "3"), ("ueur", "5")],
            false,
        );

        register_units(
            deps.as_mut().storage,
            &[("ukrw", "1.1"), (RESERVE_DENOM, "0.0")],
        );

        let mut trade_info = default_trade_info();
        trade_info.routes = SwapRoutes(vec![SwapRoute {
            pool_id: 0,
            token_denom: RESERVE_DENOM.to_string(),
        }]);
        trade_info.last_traded_at = Some(mock_env().block.time.seconds() - trade_info.cooldown - 1);

        TRADE_INFOS
            .save(deps.as_mut().storage, "ukrw".to_string(), &trade_info)
            .unwrap();

        // execute
        let res = deflate(deps.as_mut(), "manager", "ukrw", 10000, 10000).unwrap();

        // assert attributes
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "deflate"),
                attr("executor", "manager"),
                attr("denom", "ukrw"),
                attr("amount_in", "10000"),
                attr("amount_out", "10000"),
            ]
        );

        assert_state(
            deps.as_ref().storage,
            "ukrw",
            "1.0",
            "0.1",
            rebalance,
            10000,
        );
    }

    #[test]
    fn test_deflate_reserve() {
        let mut deps = mock_dependencies();

        let (rebalance, _) = setup(
            deps.as_mut().storage,
            1,
            &[("uosmo", "1.0")],
            &[("uusd", "2"), ("ujpy", "3"), ("ueur", "5")],
            false,
        );

        register_units(deps.as_mut().storage, &[("uosmo", "1.1")]);

        let res = deflate_reserve(deps.as_mut(), "manager", "uosmo", 10000).unwrap();

        // assert attributes
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "deflate_reserve"),
                attr("executor", "manager"),
                attr("denom", "uosmo"),
                attr("swap_unit", "0.1"),
                attr("amount", "10000"),
            ]
        );

        // assert state - assets
        assert_state(
            deps.as_ref().storage,
            "uosmo",
            "1.0",
            "0.1",
            rebalance,
            10000,
        );
    }

    #[test]
    fn test_get_deflation() {
        let mut deps = mock_dependencies();

        setup(deps.as_mut().storage, 1, &[], &[], false);

        assert_eq!(
            deflate(deps.as_mut(), "manager", "ukrw", 100, 100).unwrap_err(),
            ContractError::InvalidArgument("cannot find deflation asset: ukrw".to_string())
        );
        assert_eq!(
            deflate_reserve(deps.as_mut(), "manager", "ukrw", 100).unwrap_err(),
            ContractError::InvalidArgument("cannot find deflation asset: ukrw".to_string())
        );
    }

    #[test]
    fn test_check_trade_cooldown() {
        let mut deps = mock_dependencies();

        setup(deps.as_mut().storage, 1, &[("ukrw", "1.0")], &[], false);

        let trade_info = default_trade_info();
        TRADE_INFOS
            .save(deps.as_mut().storage, "ukrw".to_string(), &trade_info)
            .unwrap();

        assert_eq!(
            deflate(deps.as_mut(), "manager", "ukrw", 100, 10000).unwrap_err(),
            ContractError::CooldownNotExpired {}
        );
    }

    #[test]
    fn test_check_max_trade_amount() {
        let mut deps = mock_dependencies();

        setup(deps.as_mut().storage, 1, &[("ukrw", "1.0")], &[], false);

        let env = mock_env();
        let mut trade_info = default_trade_info();
        trade_info.last_traded_at = Some(env.block.time.seconds() - trade_info.cooldown - 1);

        TRADE_INFOS
            .save(deps.as_mut().storage, "ukrw".to_string(), &trade_info)
            .unwrap();

        let trade_amount = trade_info.max_trade_amount.u128() + 1;
        assert_eq!(
            deflate(deps.as_mut(), "manager", "ukrw", trade_amount, trade_amount,).unwrap_err(),
            ContractError::InvalidArgument(format!(
                "exceed max trade amount: {}",
                trade_amount - 1
            )),
        );
    }

    #[test]
    fn test_check_amount_gap() {
        let mut deps = mock_dependencies();

        setup(deps.as_mut().storage, 1, &[("ukrw", "1.0")], &[], false);

        register_units(
            deps.as_mut().storage,
            &[("ukrw", "1.1"), (RESERVE_DENOM, "0.0")],
        );

        let mut trade_info = default_trade_info();
        trade_info.last_traded_at = Some(mock_env().block.time.seconds() - trade_info.cooldown - 1);

        TRADE_INFOS
            .save(deps.as_mut().storage, "ukrw".to_string(), &trade_info)
            .unwrap();

        // amount_gap = 10000
        assert_eq!(
            deflate(deps.as_mut(), "manager", "ukrw", 20000, 20000).unwrap_err(),
            ContractError::InvalidArgument(format!("insufficient amount: {}", 10000))
        );
    }

    #[test]
    fn test_check_simulte_result() {
        let mut deps = mock_dependencies();

        deps.querier.stargate.register(
            "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountOut",
            |_| {
                to_binary(&EstimateSwapExactAmountOutResponse {
                    token_in_amount: "10000".to_string(),
                })
                .into()
            },
        );

        setup(deps.as_mut().storage, 1, &[("ukrw", "1.0")], &[], false);

        register_units(
            deps.as_mut().storage,
            &[("ukrw", "1.1"), (RESERVE_DENOM, "0.0")],
        );

        let mut trade_info = default_trade_info();
        trade_info.routes = SwapRoutes(vec![SwapRoute {
            pool_id: 0,
            token_denom: RESERVE_DENOM.to_string(),
        }]);
        trade_info.last_traded_at = Some(mock_env().block.time.seconds() - trade_info.cooldown - 1);

        TRADE_INFOS
            .save(deps.as_mut().storage, "ukrw".to_string(), &trade_info)
            .unwrap();

        // amount_gap = 10000
        assert_eq!(
            deflate(deps.as_mut(), "manager", "ukrw", 10000, 10000).unwrap_err(),
            ContractError::OverSlippageAllowance {}
        );
    }
}
