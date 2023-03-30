use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Uint128};
use ibcx_interface::{core::FeePayload, types::SwapRoutes};

use crate::{
    error::RebalanceError,
    state::{Config, Rebalance, StreamingFee, TradeInfo, CONFIG, FEE, REBALANCE, TRADE_INFOS},
    StdResult,
};

pub fn update_gov(deps: DepsMut, info: MessageInfo, new_gov: String) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    config.check_gov(&info.sender)?;

    CONFIG.save(
        deps.storage,
        &Config {
            gov: deps.api.addr_validate(&new_gov)?,
            ..config
        },
    )?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "gov::update_gov"),
        attr("executor", info.sender),
        attr("new_gov", new_gov),
    ]);

    Ok(resp)
}

pub fn update_fee(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_fee: FeePayload,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    config.check_gov(&info.sender)?;
    if REBALANCE.may_load(deps.storage)?.is_some() {
        return Err(RebalanceError::OnRebalancing.into());
    }

    // update
    let mut fee = FEE.load(deps.storage)?;

    fee.collector = deps.api.addr_validate(&new_fee.collector)?;
    fee.mint_fee = new_fee.mint_fee;
    fee.burn_fee = new_fee.burn_fee;
    fee.streaming_fee = new_fee.streaming_fee.map(|v| StreamingFee {
        rate: v.rate,
        collected: vec![],
        last_collected_at: env.block.time.seconds(),
        freeze: v.freeze,
    });
    fee.check_rates()?;

    FEE.save(deps.storage, &fee)?;

    // response
    let attrs = vec![
        attr("method", "gov::update_fee"),
        attr("executor", info.sender),
    ];

    let resp = Response::new().add_attributes(attrs);

    Ok(resp)
}

pub fn update_rebalance_manager(
    deps: DepsMut,
    info: MessageInfo,
    new_manager: Option<String>,
) -> StdResult<Response> {
    let rebalance = match REBALANCE.may_load(deps.storage)? {
        Some(r) => r,
        None => return Err(RebalanceError::NotOnRebalancing.into()),
    };

    REBALANCE.save(
        deps.storage,
        &Rebalance {
            manager: new_manager
                .clone()
                .map(|v| deps.api.addr_validate(&v))
                .transpose()?,
            ..rebalance
        },
    )?;

    // response
    let attrs = vec![
        attr("method", "gov::update_rebalance_manager"),
        attr("executor", info.sender),
        attr("new_manager", new_manager.as_deref().unwrap_or("none")),
    ];

    let resp = Response::new().add_attributes(attrs);

    Ok(resp)
}

pub fn update_reserve_denom(
    deps: DepsMut,
    info: MessageInfo,
    new_denom: String,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    config.check_gov(&info.sender)?;
    if REBALANCE.may_load(deps.storage)?.is_some() {
        return Err(RebalanceError::OnRebalancing.into());
    }

    CONFIG.save(
        deps.storage,
        &Config {
            reserve_denom: new_denom.clone(),
            ..config
        },
    )?;

    // response
    let attrs = vec![
        attr("method", "gov::update_reserve_denom"),
        attr("executor", info.sender),
        attr("new_denom", new_denom),
    ];

    let resp = Response::new().add_attributes(attrs);

    Ok(resp)
}

pub fn update_trade_info(
    deps: DepsMut,
    info: MessageInfo,
    denom_in: String,
    routes: SwapRoutes,
    cooldown: u64,
    max_trade_amount: Uint128,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    config.check_gov(&info.sender)?;

    let trade_info = TradeInfo {
        routes: routes.clone(),
        cooldown,
        max_trade_amount,
        last_traded_at: None,
    };

    TRADE_INFOS.save(deps.storage, (&denom_in, &routes.denom_last()), &trade_info)?;

    let attrs = vec![
        attr("method", "gov::update_trade_info"),
        attr("executor", info.sender),
        attr("denom_in", denom_in),
        attr("denom_out", routes.denom_last()),
        attr("routes", format!("{routes:?}")),
        attr("cooldown", cooldown.to_string()),
        attr("max_trade_amount", max_trade_amount.to_string()),
    ];

    let resp = Response::new().add_attributes(attrs);

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{
        attr,
        testing::{mock_env, mock_info},
        Addr, Decimal, Timestamp,
    };
    use ibcx_interface::core::{FeePayload, StreamingFeePayload};

    use crate::{
        error::{ContractError, RebalanceError},
        execute::gov::update::update_fee,
        state::{
            tests::{mock_config, StateBuilder},
            Fee, Rebalance, StreamingFee, CONFIG, FEE, REBALANCE,
        },
        test::mock_dependencies,
    };

    use super::{update_gov, update_rebalance_manager};

    #[test]
    fn test_update_gov() {
        let mut deps = mock_dependencies();

        let cases = [
            ("user", "new_gov", Err(ContractError::Unauthorized {})),
            (
                "gov",
                "new_gov",
                Ok(vec![
                    attr("method", "gov::update_gov"),
                    attr("executor", "gov"),
                    attr("new_gov", "new_gov"),
                ]),
            ),
        ];

        for (sender, new_gov, expected) in cases {
            CONFIG.save(deps.as_mut().storage, &mock_config()).unwrap();

            let resp = update_gov(deps.as_mut(), mock_info(sender, &[]), new_gov.to_string());
            assert_eq!(resp.map(|v| v.attributes), expected);
        }
    }

    #[test]
    fn test_update_fee() {
        let std_time = mock_env().block.time.seconds();
        let mut deps = mock_dependencies();

        FEE.save(deps.as_mut().storage, &Fee::default()).unwrap();
        CONFIG.save(deps.as_mut().storage, &mock_config()).unwrap();

        enum TestType {
            Collector,
            MintFee,
            BurnFee,
            StreamingFee,
            Error,
        }

        let fee_base = FeePayload {
            collector: "collector".to_string(),
            ..Default::default()
        };

        let cases = [
            (
                "gov",
                None,
                FeePayload {
                    collector: "new_collector".to_string(),
                    ..fee_base.clone()
                },
                TestType::Collector,
                Ok(vec![
                    attr("method", "gov::update_fee"),
                    attr("executor", "gov"),
                ]),
            ),
            (
                "gov",
                None,
                FeePayload {
                    mint_fee: Some(Decimal::from_str("0.1").unwrap()),
                    ..fee_base.clone()
                },
                TestType::MintFee,
                Ok(vec![
                    attr("method", "gov::update_fee"),
                    attr("executor", "gov"),
                ]),
            ),
            (
                "gov",
                None,
                FeePayload {
                    burn_fee: Some(Decimal::from_str("0.1").unwrap()),
                    ..fee_base.clone()
                },
                TestType::BurnFee,
                Ok(vec![
                    attr("method", "gov::update_fee"),
                    attr("executor", "gov"),
                ]),
            ),
            (
                "gov",
                None,
                FeePayload {
                    streaming_fee: Some(StreamingFeePayload {
                        rate: Decimal::from_str("0.1").unwrap(),
                        freeze: false,
                    }),
                    ..fee_base
                },
                TestType::StreamingFee,
                Ok(vec![
                    attr("method", "gov::update_fee"),
                    attr("executor", "gov"),
                ]),
            ),
            (
                "user",
                None,
                FeePayload::default(),
                TestType::Error,
                Err(ContractError::Unauthorized {}),
            ),
            (
                "gov",
                Some(Rebalance::default()),
                FeePayload::default(),
                TestType::Error,
                Err(RebalanceError::OnRebalancing.into()),
            ),
        ];

        for (sender, rebalance, fee, test, expected) in cases {
            match rebalance {
                Some(r) => REBALANCE.save(deps.as_mut().storage, &r).unwrap(),
                None => REBALANCE.remove(deps.as_mut().storage),
            }

            let mut env = mock_env();
            env.block.time = Timestamp::from_seconds(std_time);

            let resp = update_fee(deps.as_mut(), env, mock_info(sender, &[]), fee.clone());
            assert_eq!(resp.map(|v| v.attributes), expected);

            match test {
                TestType::Collector => assert_eq!(
                    FEE.load(deps.as_ref().storage).unwrap().collector,
                    Addr::unchecked(fee.collector)
                ),
                TestType::MintFee => assert_eq!(
                    FEE.load(deps.as_ref().storage).unwrap().mint_fee.unwrap(),
                    fee.mint_fee.unwrap()
                ),
                TestType::BurnFee => assert_eq!(
                    FEE.load(deps.as_ref().storage).unwrap().burn_fee.unwrap(),
                    fee.burn_fee.unwrap()
                ),
                TestType::StreamingFee => assert_eq!(
                    FEE.load(deps.as_ref().storage)
                        .unwrap()
                        .streaming_fee
                        .unwrap(),
                    fee.streaming_fee
                        .map(|v| StreamingFee {
                            rate: v.rate,
                            collected: vec![],
                            last_collected_at: std_time,
                            freeze: v.freeze,
                        })
                        .unwrap(),
                ),
                _ => {}
            }
        }
    }

    #[test]
    fn test_update_rebalance_manager() {
        let mut deps = mock_dependencies();

        StateBuilder::default()
            .with_rebalance(Rebalance {
                manager: Some(Addr::unchecked("manager")),
                ..Default::default()
            })
            .build(deps.as_mut().storage);

        let res = update_rebalance_manager(deps.as_mut(), mock_info("gov", &[]), None).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "gov::update_rebalance_manager"),
                attr("executor", "gov"),
                attr("new_manager", "none"),
            ]
        );
        assert_eq!(REBALANCE.load(deps.as_ref().storage).unwrap().manager, None);

        let res = update_rebalance_manager(
            deps.as_mut(),
            mock_info("gov", &[]),
            Some("manager".to_string()),
        )
        .unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "gov::update_rebalance_manager"),
                attr("executor", "gov"),
                attr("new_manager", "manager"),
            ]
        );
        assert_eq!(
            REBALANCE.load(deps.as_ref().storage).unwrap().manager,
            Some(Addr::unchecked("manager"))
        );
    }
}
