use cosmwasm_std::{attr, coin, Decimal, DepsMut, Env, MessageInfo, Response, Uint128};

use crate::{
    error::ContractError,
    state::{RESERVE_BUFFER, RESERVE_DENOM, TOKEN, TRADE_INFOS, UNITS},
};

use super::get_and_check_rebalance;

// in the case of reserve denom, we can directly inflate the unit
pub fn inflate_reserve(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // load dependencies
    let token = TOKEN.load(deps.storage)?;
    let rebalance = get_and_check_rebalance(deps.storage, &info.sender)?;

    rebalance.get_inflation(&denom)?;

    let buffer = RESERVE_BUFFER.load(deps.storage, denom.clone())?;
    if buffer < amount {
        return Err(ContractError::InvalidArgument(format!(
            "insufficient buffer: {buffer}",
        )));
    }
    RESERVE_BUFFER.save(deps.storage, denom.clone(), &(buffer.checked_sub(amount)?))?;

    let reserve_unit = UNITS.load(deps.storage, RESERVE_DENOM.to_string())?;
    let origin_unit = UNITS.load(deps.storage, denom.clone()).unwrap_or_default();
    let swap_unit = Decimal::checked_from_ratio(amount, token.total_supply)?;

    UNITS.save(
        deps.storage,
        denom.clone(),
        &origin_unit.checked_add(swap_unit)?,
    )?;
    UNITS.save(
        deps.storage,
        RESERVE_DENOM.to_string(),
        &reserve_unit.checked_sub(swap_unit)?,
    )?;

    // 1:1 conversion is done
    Ok(Response::new().add_attributes(vec![
        attr("method", "inflate_reserve"),
        attr("executor", info.sender),
        attr("denom", denom),
        attr("swap_unit", swap_unit.to_string()),
        attr("amount", amount),
    ]))
}

pub fn inflate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denom: String,
    amount: Uint128,
    min_amount_out: Uint128,
) -> Result<Response, ContractError> {
    // load dependencies
    let token = TOKEN.load(deps.storage)?;
    let rebalance = get_and_check_rebalance(deps.storage, &info.sender)?;

    // check denom by fetch
    rebalance.get_inflation(&denom)?;

    // load trade_info
    let mut trade_info = TRADE_INFOS.load(deps.storage, denom.clone())?;
    trade_info.checked_update_cooldown(env.block.time.seconds())?;
    if trade_info.max_trade_amount < amount {
        return Err(ContractError::InvalidArgument(format!(
            "exceed max trade amount: {}",
            trade_info.max_trade_amount
        )));
    }

    // deduct reserve buffer amount
    let buffer = RESERVE_BUFFER.load(deps.storage, denom.clone())?;
    if buffer < amount {
        return Err(ContractError::InvalidArgument(format!(
            "insufficient buffer: {buffer}",
        )));
    }

    // simulate
    let amount_out = trade_info.routes.sim_swap_exact_in(
        &deps.querier,
        &env.contract.address,
        coin(amount.u128(), &denom),
    )?;
    if amount_out < min_amount_out {
        return Err(ContractError::OverSlippageAllowance {});
    }

    // deduct & expand stored units
    let reserve_unit = UNITS.load(deps.storage, RESERVE_DENOM.to_string())?;
    let origin_unit = UNITS.load(deps.storage, denom.clone()).unwrap_or_default();

    let deduct_unit = Decimal::checked_from_ratio(amount, token.total_supply)?;
    let expand_unit = Decimal::checked_from_ratio(amount_out, token.total_supply)?;

    UNITS.save(
        deps.storage,
        RESERVE_DENOM.to_string(),
        &reserve_unit.checked_sub(deduct_unit)?,
    )?;
    UNITS.save(
        deps.storage,
        denom.clone(),
        &origin_unit.checked_add(expand_unit)?,
    )?;

    // deduct buffer
    RESERVE_BUFFER.save(deps.storage, denom.clone(), &buffer.checked_sub(amount)?)?;

    // build response
    Ok(Response::new()
        .add_message(trade_info.routes.msg_swap_exact_in(
            &env.contract.address,
            RESERVE_DENOM,
            amount,
            amount_out,
        ))
        .add_attributes(vec![
            attr("method", "inflate"),
            attr("executor", info.sender),
            attr("denom", denom),
            attr("amount_in", amount),
            attr("amount_out", amount_out),
        ]))
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use cosmwasm_std::{
        testing::{mock_env, mock_info},
        to_binary, Storage,
    };
    use ibcx_interface::types::{SwapRoute, SwapRoutes};
    use osmosis_std::types::osmosis::poolmanager::v1beta1::EstimateSwapExactAmountInResponse;

    use crate::{
        execute::rebalance::test::setup,
        test::{default_trade_info, mock_dependencies, register_units},
    };

    use super::*;

    fn inflate(
        deps: DepsMut,
        sender: &str,
        denom: &str,
        amount: u128,
        min_amount_out: u128,
    ) -> Result<Response, ContractError> {
        super::inflate(
            deps,
            mock_env(),
            mock_info(sender, &[]),
            denom.to_string(),
            amount.into(),
            min_amount_out.into(),
        )
    }

    fn inflate_reserve(
        deps: DepsMut,
        sender: &str,
        denom: &str,
        amount: u128,
    ) -> Result<Response, ContractError> {
        super::inflate_reserve(
            deps,
            mock_info(sender, &[]),
            denom.to_string(),
            amount.into(),
        )
    }

    fn assert_state(
        storage: &dyn Storage,
        to: &str,
        deduct: &str,
        expand: &str,
        buffer_after: u128,
    ) {
        // assert state - assets
        assert_eq!(
            UNITS.load(storage, RESERVE_DENOM.to_string()).unwrap(),
            Decimal::from_str(deduct).unwrap()
        );
        assert_eq!(
            UNITS.load(storage, to.to_string()).unwrap(),
            Decimal::from_str(expand).unwrap()
        );

        // assert state - distribution (reserve buffer)
        assert_eq!(
            RESERVE_BUFFER.load(storage, to.to_string()).unwrap(),
            Uint128::new(buffer_after)
        );
    }

    #[test]
    fn test_inflate() {
        let mut deps = mock_dependencies();

        deps.querier.stargate.register(
            "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountIn",
            |_| {
                to_binary(&EstimateSwapExactAmountInResponse {
                    token_out_amount: "1000".to_string(),
                })
                .into()
            },
        );

        setup(deps.as_mut().storage, 1, &[], &[("ukrw", "1.0")], false);

        register_units(deps.as_mut().storage, &[(RESERVE_DENOM, "1.0")]);

        let mut trade_info = default_trade_info();
        trade_info.routes = SwapRoutes(vec![SwapRoute {
            pool_id: 0,
            token_denom: RESERVE_DENOM.to_string(),
        }]);
        trade_info.last_traded_at = Some(mock_env().block.time.seconds() - trade_info.cooldown - 1);

        TRADE_INFOS
            .save(deps.as_mut().storage, "ukrw".to_string(), &trade_info)
            .unwrap();

        let buffer = Uint128::new(100000);
        RESERVE_BUFFER
            .save(deps.as_mut().storage, "ukrw".to_string(), &buffer)
            .unwrap();

        let res = inflate(deps.as_mut(), "manager", "ukrw", 10000, 10).unwrap();

        assert_eq!(
            res.attributes,
            vec![
                attr("method", "inflate"),
                attr("executor", "manager"),
                attr("denom", "ukrw"),
                attr("amount_in", "10000"),
                attr("amount_out", "1000"),
            ]
        );

        assert_state(deps.as_ref().storage, "ukrw", "0.9", "0.01", 90000);
    }

    #[test]
    fn test_inflate_reserve() {
        let mut deps = mock_dependencies();

        setup(deps.as_mut().storage, 1, &[], &[("uosmo", "1.0")], false);

        register_units(deps.as_mut().storage, &[(RESERVE_DENOM, "1.0")]);

        let buffer = Uint128::new(100000);
        RESERVE_BUFFER
            .save(deps.as_mut().storage, "uosmo".to_string(), &buffer)
            .unwrap();

        let res = inflate_reserve(deps.as_mut(), "manager", "uosmo", 10000).unwrap();

        assert_eq!(
            res.attributes,
            vec![
                attr("method", "inflate_reserve"),
                attr("executor", "manager"),
                attr("denom", "uosmo"),
                attr("swap_unit", "0.1"),
                attr("amount", "10000")
            ]
        );

        assert_state(deps.as_ref().storage, "uosmo", "0.9", "0.1", 90000);
    }

    #[test]
    fn test_get_inflation() {
        let mut deps = mock_dependencies();

        setup(deps.as_mut().storage, 1, &[], &[], false);

        assert_eq!(
            inflate(deps.as_mut(), "manager", "ukrw", 100, 100).unwrap_err(),
            ContractError::InvalidArgument("cannot find inflation asset: ukrw".to_string())
        );
    }

    #[test]
    fn test_check_max_trade_amount() {
        let mut deps = mock_dependencies();

        setup(deps.as_mut().storage, 1, &[], &[("ukrw", "1.0")], false);

        let mut trade_info = default_trade_info();
        trade_info.last_traded_at = Some(mock_env().block.time.seconds() - trade_info.cooldown - 1);

        TRADE_INFOS
            .save(deps.as_mut().storage, "ukrw".to_string(), &trade_info)
            .unwrap();

        assert_eq!(
            inflate(
                deps.as_mut(),
                "manager",
                "ukrw",
                trade_info.max_trade_amount.u128() + 1,
                100,
            )
            .unwrap_err(),
            ContractError::InvalidArgument(format!(
                "exceed max trade amount: {}",
                trade_info.max_trade_amount
            ))
        );
    }

    #[test]
    fn test_check_reserve_buffer() {
        let mut deps = mock_dependencies();

        setup(deps.as_mut().storage, 1, &[], &[("ukrw", "1.0")], false);

        let mut trade_info = default_trade_info();
        trade_info.last_traded_at = Some(mock_env().block.time.seconds() - trade_info.cooldown - 1);

        TRADE_INFOS
            .save(deps.as_mut().storage, "ukrw".to_string(), &trade_info)
            .unwrap();

        let buffer = Uint128::new(1000);
        RESERVE_BUFFER
            .save(deps.as_mut().storage, "ukrw".to_string(), &buffer)
            .unwrap();

        assert_eq!(
            inflate(
                deps.as_mut(),
                "manager",
                "ukrw",
                trade_info.max_trade_amount.u128() - 1,
                100,
            )
            .unwrap_err(),
            ContractError::InvalidArgument(format!("insufficient buffer: {buffer}",))
        );
    }

    #[test]
    fn test_check_simulate_result() {
        let mut deps = mock_dependencies();

        deps.querier.stargate.register(
            "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountIn",
            |_| {
                to_binary(&EstimateSwapExactAmountInResponse {
                    token_out_amount: "1000".to_string(),
                })
                .into()
            },
        );

        setup(deps.as_mut().storage, 1, &[], &[("ukrw", "1.0")], false);

        let mut trade_info = default_trade_info();
        trade_info.routes = SwapRoutes(vec![SwapRoute {
            pool_id: 0,
            token_denom: RESERVE_DENOM.to_string(),
        }]);
        trade_info.last_traded_at = Some(mock_env().block.time.seconds() - trade_info.cooldown - 1);

        TRADE_INFOS
            .save(deps.as_mut().storage, "ukrw".to_string(), &trade_info)
            .unwrap();

        let buffer = Uint128::new(100000);
        RESERVE_BUFFER
            .save(deps.as_mut().storage, "ukrw".to_string(), &buffer)
            .unwrap();

        assert_eq!(
            inflate(
                deps.as_mut(),
                "manager",
                "ukrw",
                trade_info.max_trade_amount.u128() - 1,
                1001,
            )
            .unwrap_err(),
            ContractError::OverSlippageAllowance {}
        );
    }
}
