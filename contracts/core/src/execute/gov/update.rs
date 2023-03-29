use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Uint128};
use ibcx_interface::{core::FeePayload, types::SwapRoutes};

use crate::{
    error::ContractError,
    state::{Config, StreamingFee, TradeInfo, CONFIG, FEE, REBALANCE, RESERVE_UNITS, TRADE_INFOS},
};

pub fn update_gov(
    deps: DepsMut,
    info: MessageInfo,
    new_gov: String,
) -> Result<Response, ContractError> {
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
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    config.check_gov(&info.sender)?;
    if REBALANCE.may_load(deps.storage)?.is_some() {
        return Err(ContractError::RebalanceNotFinalized {});
    }

    // update
    let mut fee = FEE.load(deps.storage)?;

    fee.collector = deps.api.addr_validate(&new_fee.collector)?;
    fee.mint_fee = new_fee.mint_fee;
    fee.burn_fee = new_fee.burn_fee;
    fee.streaming_fee = new_fee.streaming_fee.map(|v| StreamingFee {
        rate: v,
        collected: vec![],
        last_collected_at: env.block.time.seconds(),
    });

    FEE.save(deps.storage, &fee)?;

    // response
    let attrs = vec![
        attr("method", "gov::update_fee"),
        attr("executor", info.sender),
        attr("new_fee", format!("{new_fee:?}")),
    ];

    let resp = Response::new().add_attributes(attrs);

    Ok(resp)
}

pub fn update_reserve_denom(
    deps: DepsMut,
    info: MessageInfo,
    new_denom: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    config.check_gov(&info.sender)?;
    if REBALANCE.may_load(deps.storage)?.is_some() {
        return Err(ContractError::RebalanceNotFinalized {});
    }

    let reserve_units = RESERVE_UNITS.load(deps.storage)?;
    if !reserve_units.check_empty() {
        return Err(ContractError::RebalanceNotFinalized {});
    }

    CONFIG.save(
        deps.storage,
        &Config {
            reserve_denom: new_denom,
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
    denom: String,
    routes: SwapRoutes,
    cooldown: u64,
    max_trade_amount: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    config.check_gov(&info.sender)?;

    let trade_info = TradeInfo {
        routes: routes.clone(),
        cooldown,
        max_trade_amount,
        last_traded_at: None,
    };

    TRADE_INFOS.save(deps.storage, denom.clone(), &trade_info)?;

    let attrs = vec![
        attr("method", "gov::update_trade_info"),
        attr("executor", info.sender),
        attr("denom", denom),
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
    use ibcx_interface::core::FeePayload;

    use crate::{
        error::ContractError,
        execute::gov::update::update_fee,
        state::{tests::mock_config, Fee, Rebalance, StreamingFee, CONFIG, FEE, REBALANCE},
        test::mock_dependencies,
    };

    use super::update_gov;

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

        enum TestType {
            Collector,
            MintFee,
            BurnFee,
            StreamingFee,
            Error,
        }

        let cases = [
            (
                "gov",
                None,
                FeePayload {
                    collector: "new_collector".to_string(),
                    ..Default::default()
                },
                TestType::Collector,
                Ok(vec![
                    attr("method", "gov::update_fee"),
                    attr("executor", "gov"),
                    // attr("new_fee", { NEW_FEE })
                ]),
            ),
            (
                "gov",
                None,
                FeePayload {
                    mint_fee: Some(Decimal::from_str("0.1").unwrap()),
                    ..Default::default()
                },
                TestType::MintFee,
                Ok(vec![
                    attr("method", "gov::update_fee"),
                    attr("executor", "gov"),
                    // attr("new_fee", { NEW_FEE })
                ]),
            ),
            (
                "gov",
                None,
                FeePayload {
                    burn_fee: Some(Decimal::from_str("0.1").unwrap()),
                    ..Default::default()
                },
                TestType::BurnFee,
                Ok(vec![
                    attr("method", "gov::update_fee"),
                    attr("executor", "gov"),
                    // attr("new_fee", { NEW_FEE })
                ]),
            ),
            (
                "gov",
                None,
                FeePayload {
                    streaming_fee: Some(Decimal::from_str("0.1").unwrap()),
                    ..Default::default()
                },
                TestType::StreamingFee,
                Ok(vec![
                    attr("method", "gov::update_fee"),
                    attr("executor", "gov"),
                    // attr("new_fee", { NEW_FEE })
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
                Err(ContractError::RebalanceNotFinalized {}),
            ),
        ];

        for (sender, rebalance, fee, test, expected) in cases {
            expected.map(|mut v| v.push(attr("new_fee", format!("{:?}", fee))));

            match rebalance {
                Some(r) => REBALANCE.save(deps.as_mut().storage, &r).unwrap(),
                None => REBALANCE.remove(deps.as_mut().storage),
            }

            let mut env = mock_env();
            env.block.time = Timestamp::from_seconds(std_time);

            let resp = update_fee(deps.as_mut(), env, mock_info(sender, &[]), fee);
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
                    FEE.load(deps.as_ref().storage).unwrap().mint_fee.unwrap(),
                    fee.burn_fee.unwrap()
                ),
                TestType::StreamingFee => assert_eq!(
                    FEE.load(deps.as_ref().storage)
                        .unwrap()
                        .streaming_fee
                        .unwrap(),
                    StreamingFee {
                        rate: fee.streaming_fee.unwrap(),
                        collected: vec![],
                        last_collected_at: std_time,
                    },
                ),
                _ => {}
            }
        }
    }
}
