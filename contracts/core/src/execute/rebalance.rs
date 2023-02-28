use cosmwasm_std::{attr, coin, Addr, Decimal, Env, MessageInfo, Storage, SubMsg, Uint128};
use cosmwasm_std::{DepsMut, Response};
use ibcx_interface::core::{RebalanceMsg, RebalanceTradeMsg};

use crate::{
    error::ContractError,
    state::{
        get_units, Rebalance, GOV, LATEST_REBALANCE_ID, REBALANCES, RESERVE_BUFFER, RESERVE_DENOM,
        TOKEN, TRADE_INFOS, UNITS,
    },
};

use super::fee;

pub fn handle_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: RebalanceMsg,
) -> Result<Response, ContractError> {
    use RebalanceMsg::*;

    match msg {
        Init {
            manager,
            deflation,
            inflation,
        } => init(deps, info, manager, deflation, inflation),
        Trade(msg) => trade(deps, env, info, msg),
        Finalize {} => finalize(deps, env, info),
    }
}

/// initialize the rebalance
/// deflation: target unit of each denom to decrease
/// inflation: weight of each denom to distribute
///
/// basic flow of rebalance
///
///=========================================
/// [ DEFLATION ]            [ INFLATION ]
///-----------------------------------------
///     | A  ==\             /==>  D |
///     | B  ===> [RESERVE] ====>  E |
///     | C  ==/             \==>  F |
///=========================================
pub fn init(
    deps: DepsMut,
    info: MessageInfo,
    manager: String,
    deflation: Vec<(String, Decimal)>,
    inflation: Vec<(String, Decimal)>,
) -> Result<Response, ContractError> {
    // only governance can execute this
    if info.sender != GOV.load(deps.storage)? {
        return Err(ContractError::Unauthorized {});
    }

    // check if there is a ongoing rebalance
    let rebalance_id = LATEST_REBALANCE_ID
        .may_load(deps.storage)?
        .unwrap_or_default();
    if let Some(r) = REBALANCES.may_load(deps.storage, rebalance_id)? {
        if !r.finalized {
            return Err(ContractError::RebalanceNotFinalized {});
        }
    }

    // make new rebalance
    let rebalance = Rebalance {
        manager: deps.api.addr_validate(&manager)?,
        deflation,
        inflation,
        finalized: false,
    };

    // fetch current units and validate new rebalance
    let units = get_units(deps.storage)?;
    rebalance.validate(units)?;

    // save
    REBALANCES.save(deps.storage, rebalance_id, &rebalance)?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "rebalance_init"),
        attr("executor", info.sender),
        attr("manager", manager),
        attr("rebalance_id", rebalance_id.to_string()),
    ]);

    Ok(resp)
}

// deflate / inflate the target denom
pub fn trade(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: RebalanceTradeMsg,
) -> Result<Response, ContractError> {
    // realize streaming fee before rebalance to cleanup latest states
    let realize_msg = fee::realize_streaming_fee(deps.storage)?;

    // make message wrapper to place the realize msg at the top
    let wrap = |resp: Result<Response, ContractError>| {
        resp.map(|mut r| {
            r.messages.insert(0, SubMsg::new(realize_msg));
            r
        })
    };

    match msg {
        RebalanceTradeMsg::Deflate {
            denom,
            amount,
            max_amount_in,
        } => {
            let token = TOKEN.load(deps.storage)?;

            let reserve = token.reserve_denom;

            if reserve == denom {
                wrap(deflate_reserve(deps, info, denom, amount))
            } else {
                wrap(deflate(deps, env, info, denom, amount, max_amount_in))
            }
        }

        RebalanceTradeMsg::Inflate {
            denom,
            amount,
            min_amount_out,
        } => {
            let token = TOKEN.load(deps.storage)?;

            let reserve = token.reserve_denom;

            if reserve == denom {
                wrap(inflate_reserve(deps, info, denom, amount))
            } else {
                wrap(inflate(deps, env, info, denom, amount, min_amount_out))
            }
        }
    }
}

// fetch rebalance info & validate rebalance and check if the manager is valid
fn get_and_check_rebalance(
    storage: &dyn Storage,
    sender: &Addr,
) -> Result<Rebalance, ContractError> {
    let rebalance_id = LATEST_REBALANCE_ID.load(storage)?;
    let rebalance = REBALANCES.load(storage, rebalance_id)?;
    if &rebalance.manager != sender {
        return Err(ContractError::Unauthorized {});
    }
    if rebalance.finalized {
        return Err(ContractError::RebalanceFinalized {});
    }

    Ok(rebalance)
}

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

pub fn finalize(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let rebalance_id = LATEST_REBALANCE_ID.load(deps.storage)?;
    LATEST_REBALANCE_ID.save(deps.storage, &(rebalance_id + 1))?;

    let mut rebalance = REBALANCES.load(deps.storage, rebalance_id)?;

    // check deflation
    for (denom, target_unit) in rebalance.deflation.clone() {
        let current_unit = UNITS.load(deps.storage, denom)?;
        if target_unit == current_unit {
            continue;
        } else {
            return Err(ContractError::UnableToFinalize {});
        }
    }

    // check inflation
    if !UNITS
        .load(deps.storage, RESERVE_DENOM.to_string())?
        .is_zero()
    {
        return Err(ContractError::UnableToFinalize {});
    }

    rebalance.finalized = true;

    REBALANCES.save(deps.storage, rebalance_id, &rebalance)?;

    Ok(Response::new().add_attributes(vec![
        attr("method", "finalize"),
        attr("executor", info.sender),
        attr("finalized_at", env.block.height.to_string()),
    ]))
}

#[cfg(test)]
mod test {

    use cosmwasm_std::{
        testing::{mock_env, mock_info},
        to_binary, Addr, Storage,
    };
    use ibcx_interface::types::{SwapRoute, SwapRoutes};
    use osmosis_std::types::osmosis::gamm::v1beta1::{
        QuerySwapExactAmountInResponse, QuerySwapExactAmountOutResponse,
    };
    use std::str::FromStr;

    use crate::test::default_trade_info;
    use crate::test::mock_dependencies;
    use crate::test::{register_units, to_units, SENDER_GOV, SENDER_OWNER};
    use crate::{state::Token, test::default_fee};
    use crate::{state::FEE, test::default_token};

    use super::*;

    fn setup(
        storage: &mut dyn Storage,
        id: u64,
        deflation: &[(&str, &str)],
        inflation: &[(&str, &str)],
        finalized: bool,
    ) -> (Rebalance, Token) {
        let rebalance = Rebalance {
            manager: Addr::unchecked("manager"),
            deflation: to_units(deflation),
            inflation: to_units(inflation),
            finalized,
        };

        let token = default_token();

        LATEST_REBALANCE_ID.save(storage, &id).unwrap();
        REBALANCES.save(storage, id, &rebalance).unwrap();
        TOKEN.save(storage, &token).unwrap();
        FEE.save(storage, &default_fee()).unwrap();

        (rebalance, token)
    }

    mod init {

        use super::*;

        #[test]
        fn test_init() {
            let mut deps = mock_dependencies();

            GOV.save(deps.as_mut().storage, &Addr::unchecked(SENDER_GOV))
                .unwrap();

            register_units(deps.as_mut().storage, &[("ukrw", "1.0"), ("uusd", "1.8")]);

            let resp = init(
                deps.as_mut(),
                mock_info(SENDER_GOV, &[]),
                "manager".to_string(),
                to_units(&[("ukrw", "0.3")]),
                to_units(&[("ujpy", "1")]),
            )
            .unwrap();
            assert_eq!(
                resp.attributes,
                vec![
                    attr("method", "rebalance_init"),
                    attr("executor", SENDER_GOV),
                    attr("manager", "manager"),
                    attr("rebalance_id", "0"),
                ]
            );

            let rebalance = REBALANCES.load(deps.as_ref().storage, 0).unwrap();
            assert_eq!(
                rebalance,
                Rebalance {
                    manager: Addr::unchecked("manager"),
                    deflation: to_units(&[("ukrw", "0.3")]),
                    inflation: to_units(&[("ujpy", "1")]),
                    finalized: false
                }
            );
        }

        #[test]
        fn test_check_authority() {
            let mut deps = mock_dependencies();

            GOV.save(deps.as_mut().storage, &Addr::unchecked(SENDER_GOV))
                .unwrap();

            let err = init(
                deps.as_mut(),
                mock_info(SENDER_OWNER, &[]),
                "manager".to_string(),
                to_units(&[]),
                to_units(&[]),
            )
            .unwrap_err();
            assert_eq!(err, ContractError::Unauthorized {});
        }

        #[test]
        fn test_check_previous_rebalance() {
            let mut deps = mock_dependencies();

            GOV.save(deps.as_mut().storage, &Addr::unchecked(SENDER_GOV))
                .unwrap();

            REBALANCES
                .save(
                    deps.as_mut().storage,
                    0,
                    &Rebalance {
                        manager: Addr::unchecked("manager"),
                        deflation: vec![],
                        inflation: vec![],
                        finalized: false,
                    },
                )
                .unwrap();

            let err = init(
                deps.as_mut(),
                mock_info(SENDER_GOV, &[]),
                "manager".to_string(),
                to_units(&[]),
                to_units(&[]),
            )
            .unwrap_err();
            assert_eq!(err, ContractError::RebalanceNotFinalized {});
        }
    }

    mod trade {

        use super::*;

        fn trade(
            deps: DepsMut,
            sender: &str,
            msg: RebalanceTradeMsg,
        ) -> Result<Response, ContractError> {
            super::trade(deps, mock_env(), mock_info(sender, &[]), msg)
        }

        #[test]
        fn test_check_authority() {
            let mut deps = mock_dependencies();

            setup(deps.as_mut().storage, 1, &[], &[], false);

            let err = trade(
                deps.as_mut(),
                SENDER_OWNER,
                RebalanceTradeMsg::Deflate {
                    denom: "ukrw".to_string(),
                    amount: Uint128::new(100),
                    max_amount_in: Uint128::new(100),
                },
            )
            .unwrap_err();
            assert_eq!(err, ContractError::Unauthorized {});

            let err = trade(
                deps.as_mut(),
                SENDER_OWNER,
                RebalanceTradeMsg::Inflate {
                    denom: "ukrw".to_string(),
                    amount: Uint128::new(100),
                    min_amount_out: Uint128::new(100),
                },
            )
            .unwrap_err();
            assert_eq!(err, ContractError::Unauthorized {});
        }

        #[test]
        fn test_check_rebalnce_finalized() {
            let mut deps = mock_dependencies();

            setup(deps.as_mut().storage, 1, &[], &[], true);

            let err = trade(
                deps.as_mut(),
                "manager",
                RebalanceTradeMsg::Deflate {
                    denom: "ukrw".to_string(),
                    amount: Uint128::new(100),
                    max_amount_in: Uint128::new(100),
                },
            )
            .unwrap_err();
            assert_eq!(err, ContractError::RebalanceFinalized {});

            let err = trade(
                deps.as_mut(),
                "manager",
                RebalanceTradeMsg::Inflate {
                    denom: "ukrw".to_string(),
                    amount: Uint128::new(100),
                    min_amount_out: Uint128::new(100),
                },
            )
            .unwrap_err();
            assert_eq!(err, ContractError::RebalanceFinalized {});
        }

        mod deflate {

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
                    let expected =
                        weight.checked_div(total_weight).unwrap() * Uint128::new(distribute);
                    assert_eq!(RESERVE_BUFFER.load(storage, denom).unwrap(), expected);
                }
            }

            #[test]
            fn test_deflate() {
                let mut deps = mock_dependencies();

                deps.querier.stargate.register(
                    "/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountOut",
                    |_| {
                        to_binary(&QuerySwapExactAmountOutResponse {
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
                trade_info.last_traded_at =
                    Some(mock_env().block.time.seconds() - trade_info.cooldown - 1);

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
                trade_info.last_traded_at =
                    Some(env.block.time.seconds() - trade_info.cooldown - 1);

                TRADE_INFOS
                    .save(deps.as_mut().storage, "ukrw".to_string(), &trade_info)
                    .unwrap();

                let trade_amount = trade_info.max_trade_amount.u128() + 1;
                assert_eq!(
                    deflate(deps.as_mut(), "manager", "ukrw", trade_amount, trade_amount,)
                        .unwrap_err(),
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
                trade_info.last_traded_at =
                    Some(mock_env().block.time.seconds() - trade_info.cooldown - 1);

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
                    "/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountOut",
                    |_| {
                        to_binary(&QuerySwapExactAmountOutResponse {
                            token_in_amount: "100000".to_string(),
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
                trade_info.last_traded_at =
                    Some(mock_env().block.time.seconds() - trade_info.cooldown - 1);

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

        mod inflate {

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
                    "/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountIn",
                    |_| {
                        to_binary(&QuerySwapExactAmountInResponse {
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
                trade_info.last_traded_at =
                    Some(mock_env().block.time.seconds() - trade_info.cooldown - 1);

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
                trade_info.last_traded_at =
                    Some(mock_env().block.time.seconds() - trade_info.cooldown - 1);

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
                trade_info.last_traded_at =
                    Some(mock_env().block.time.seconds() - trade_info.cooldown - 1);

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
                    "/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountIn",
                    |_| {
                        to_binary(&QuerySwapExactAmountInResponse {
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
                trade_info.last_traded_at =
                    Some(mock_env().block.time.seconds() - trade_info.cooldown - 1);

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
    }

    mod finalize {
        use super::*;

        #[test]
        fn test_finalize() {
            let mut deps = mock_dependencies();

            setup(
                deps.as_mut().storage,
                1,
                &[("ukrw", "1.2"), ("ujpy", "1.5"), ("uusd", "1.3")],
                &[("uosmo", "1.0"), ("uatom", "3.14")],
                false,
            );

            register_units(
                deps.as_mut().storage,
                &[
                    ("ukrw", "1.2"),
                    ("ujpy", "1.5"),
                    ("uusd", "1.3"),
                    (RESERVE_DENOM, "0.0"),
                ],
            );

            let res = finalize(deps.as_mut(), mock_env(), mock_info("manager", &[])).unwrap();

            assert_eq!(
                res.attributes,
                vec![
                    attr("method", "finalize"),
                    attr("executor", "manager"),
                    attr("finalized_at", mock_env().block.height.to_string())
                ]
            );

            assert!(REBALANCES.load(deps.as_ref().storage, 1).unwrap().finalized);
        }
    }
}
