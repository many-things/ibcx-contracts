use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Uint128};
use ibcx_interface::{core::Fee, types::SwapRoutes};

use crate::{
    error::ContractError,
    execute::fee::realize_streaming_fee,
    state::{TradeInfo, FEE, GOV, RESERVE_DENOM, TOKEN, TRADE_INFOS, UNITS},
};

pub fn update_gov(
    deps: DepsMut,
    info: MessageInfo,
    new_gov: String,
) -> Result<Response, ContractError> {
    GOV.save(deps.storage, &deps.api.addr_validate(&new_gov)?)?;

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
    new_fee: Fee,
) -> Result<Response, ContractError> {
    let msg = realize_streaming_fee(deps.storage)?;

    let mut fee = FEE.load(deps.storage)?;

    fee.collector = deps.api.addr_validate(&new_fee.collector)?;
    fee.mint = new_fee.mint;
    fee.burn = new_fee.burn;

    if let Some(stream) = new_fee.stream {
        fee.stream = Some(stream);
        // TODO: update fee
        fee.stream_last_collected_at = env.block.time.seconds();
    }

    FEE.save(deps.storage, &fee)?;

    let resp = Response::new().add_message(msg).add_attributes(vec![
        attr("method", "gov::update_fee"),
        attr("executor", info.sender),
        attr("new_fee", format!("{new_fee:?}")),
    ]);

    Ok(resp)
}

pub fn update_reserve_denom(
    deps: DepsMut,
    info: MessageInfo,
    new_denom: String,
) -> Result<Response, ContractError> {
    let mut token = TOKEN.load(deps.storage)?;
    let unit = UNITS.load(deps.storage, RESERVE_DENOM.to_string())?;
    if !unit.is_zero() {
        return Err(ContractError::InvalidArgument(
            "reserve_denom must be zero in portfolio".to_string(),
        ));
    }

    token.reserve_denom = new_denom.clone();

    TOKEN.save(deps.storage, &token)?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "gov::update_reserve_denom"),
        attr("executor", info.sender),
        attr("new_denom", new_denom),
    ]);

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
    TRADE_INFOS.save(
        deps.storage,
        denom.clone(),
        &TradeInfo {
            routes: routes.clone(),
            cooldown,
            max_trade_amount,
            last_traded_at: None,
        },
    )?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "gov::update_trade_info"),
        attr("executor", info.sender),
        attr("denom", denom),
        attr("routes", format!("{routes:?}")),
        attr("cooldown", cooldown.to_string()),
        attr("max_trade_amount", max_trade_amount.to_string()),
    ]);

    Ok(resp)
}

#[cfg(test)]
mod tests {

    use cosmwasm_std::{
        testing::{mock_env, mock_info},
        Addr, Decimal,
    };

    use crate::{
        state::{self, Token},
        test::{
            default_fee, default_token, mock_dependencies, DENOM_DEFAULT, SENDER_GOV, SENDER_OWNER,
            SENDER_VALID,
        },
    };

    use super::*;

    fn setup(
        deps: DepsMut,
        gov: impl Into<String>,
        fee: Option<state::Fee>,
        token: Option<Token>,
        assets: &[(&str, Decimal)],
    ) {
        let gov = Addr::unchecked(gov.into());
        GOV.save(deps.storage, &gov).unwrap();

        let fee = fee.unwrap_or_else(default_fee);
        FEE.save(deps.storage, &fee).unwrap();

        let token = token.unwrap_or_else(default_token);
        TOKEN.save(deps.storage, &token).unwrap();

        for (denom, unit) in assets {
            UNITS
                .save(deps.storage, denom.to_string(), &unit.clone())
                .unwrap();
        }
    }

    #[test]
    fn test_update_gov() {
        let mut deps = mock_dependencies();

        setup(deps.as_mut(), SENDER_GOV, None, None, &[]);

        let resp = update_gov(
            deps.as_mut(),
            mock_info(SENDER_GOV, &[]),
            SENDER_OWNER.to_string(),
        )
        .unwrap();

        assert_eq!(
            resp.attributes,
            vec![
                attr("method", "gov::update_gov"),
                attr("executor", SENDER_GOV),
                attr("new_gov", SENDER_OWNER),
            ]
        );

        assert_eq!(
            GOV.load(deps.as_ref().storage).unwrap().as_str(),
            SENDER_OWNER
        );
    }

    #[test]
    fn test_update_fee_strategy() {
        let mut deps = mock_dependencies();

        setup(deps.as_mut(), SENDER_GOV, None, None, &[]);

        let new_fee = Fee {
            collector: SENDER_VALID.to_string(),
            mint: Some(Decimal::from_ratio(10u128, 100u128)),
            burn: Some(Decimal::from_ratio(5u128, 100u128)),
            stream: Some(Decimal::from_ratio(1u128, 100u128)),
        };

        let resp = update_fee(
            deps.as_mut(),
            mock_env(),
            mock_info(SENDER_GOV, &[]),
            new_fee.clone(),
        )
        .unwrap();

        assert_eq!(
            resp.attributes,
            vec![
                attr("method", "gov::update_fee"),
                attr("executor", SENDER_GOV),
                attr("new_fee", format!("{new_fee:?}"))
            ]
        );

        assert_eq!(
            FEE.load(deps.as_ref().storage).unwrap(),
            state::Fee {
                collector: Addr::unchecked(new_fee.collector),
                stream_collected: vec![],
                mint: new_fee.mint,
                burn: new_fee.burn,
                stream: new_fee.stream,
                stream_last_collected_at: mock_env().block.time.seconds(),
            }
        );
    }

    #[test]
    fn test_update_reserve_denom() {
        let mut deps = mock_dependencies();

        // error
        setup(
            deps.as_mut(),
            SENDER_GOV,
            None,
            None,
            &[(RESERVE_DENOM, Decimal::one())],
        );

        assert_eq!(
            update_reserve_denom(
                deps.as_mut(),
                mock_info(SENDER_GOV, &[]),
                DENOM_DEFAULT.to_string()
            )
            .unwrap_err(),
            ContractError::InvalidArgument("reserve_denom must be zero in portfolio".to_string())
        );

        // success
        UNITS
            .save(
                deps.as_mut().storage,
                RESERVE_DENOM.to_string(),
                &Decimal::zero(),
            )
            .unwrap();

        let resp = update_reserve_denom(
            deps.as_mut(),
            mock_info(SENDER_GOV, &[]),
            DENOM_DEFAULT.to_string(),
        )
        .unwrap();

        assert_eq!(
            resp.attributes,
            vec![
                attr("method", "gov::update_reserve_denom"),
                attr("executor", SENDER_GOV),
                attr("new_denom", DENOM_DEFAULT),
            ]
        );

        assert_eq!(
            TOKEN.load(deps.as_ref().storage).unwrap().reserve_denom,
            DENOM_DEFAULT
        );
    }

    #[test]
    fn test_update_trade_info() {
        let mut deps = mock_dependencies();

        let resp = update_trade_info(
            deps.as_mut(),
            mock_info(SENDER_GOV, &[]),
            DENOM_DEFAULT.to_string(),
            SwapRoutes(vec![]),
            86400,
            Uint128::zero(),
        )
        .unwrap();

        assert_eq!(
            resp.attributes,
            vec![
                attr("method", "gov::update_trade_info"),
                attr("executor", SENDER_GOV),
                attr("denom", DENOM_DEFAULT),
                attr("routes", format!("{:?}", SwapRoutes(vec![]))),
                attr("cooldown", 86400.to_string()),
                attr("max_trade_amount", Uint128::zero().to_string()),
            ]
        );

        assert_eq!(
            TRADE_INFOS
                .load(deps.as_ref().storage, DENOM_DEFAULT.to_string())
                .unwrap(),
            TradeInfo {
                routes: SwapRoutes(vec![]),
                cooldown: 86400,
                max_trade_amount: Uint128::zero(),
                last_traded_at: None,
            }
        );
    }
}
