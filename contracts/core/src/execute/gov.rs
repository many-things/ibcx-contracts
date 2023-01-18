use cosmwasm_std::{attr, Env, MessageInfo, Uint128};
use cosmwasm_std::{DepsMut, Response};
use ibcx_interface::core::Fee;
use ibcx_interface::{core::GovMsg, types::SwapRoutes};

use crate::state::FEE;
use crate::{
    error::ContractError,
    state::{TradeInfo, ASSETS, GOV, PAUSED, RESERVE_DENOM, TOKEN, TRADE_INFOS},
};

pub fn handle_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: GovMsg,
) -> Result<Response, ContractError> {
    use GovMsg::*;

    if info.sender != GOV.load(deps.storage)? {
        return Err(ContractError::Unauthorized {});
    }

    match msg {
        Pause { expires_at } => pause(deps, env, info, expires_at),
        Release {} => release(deps, env, info),

        UpdateGov(new_gov) => update_gov(deps, info, new_gov),
        UpdateFeeStrategy(new_fee) => update_fee(deps, env, info, new_fee),
        UpdateReserveDenom(new_denom) => update_reserve_denom(deps, info, new_denom),
        UpdateTradeInfo {
            denom,
            routes,
            cooldown,
            max_trade_amount,
        } => update_trade_info(deps, info, denom, routes, cooldown, max_trade_amount),
    }
}

fn pause(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    expires_at: u64,
) -> Result<Response, ContractError> {
    let mut pause_info = PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    if env.block.time.seconds() >= expires_at {
        return Err(ContractError::InvalidArgument(
            "expires_at must be in the future".to_string(),
        ));
    }

    pause_info.paused = true;
    pause_info.expires_at = Some(expires_at);

    PAUSED.save(deps.storage, &pause_info)?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "gov::pause"),
        attr("executor", info.sender),
        attr("expires_at", pause_info.expires_at.unwrap().to_string()),
    ]);

    Ok(resp)
}

fn release(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_paused()?;

    PAUSED.save(deps.storage, &Default::default())?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "gov::release"),
        attr("executor", info.sender),
    ]);

    Ok(resp)
}

fn update_gov(
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

fn update_fee(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_fee: Fee,
) -> Result<Response, ContractError> {
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

    let resp = Response::new().add_attributes(vec![
        attr("method", "gov::update_fee"),
        attr("executor", info.sender),
        attr("new_fee", format!("{new_fee:?}")),
    ]);

    Ok(resp)
}

fn update_reserve_denom(
    deps: DepsMut,
    info: MessageInfo,
    new_denom: String,
) -> Result<Response, ContractError> {
    let mut token = TOKEN.load(deps.storage)?;
    let unit = ASSETS.load(deps.storage, RESERVE_DENOM.to_string())?;
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

fn update_trade_info(
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
mod test {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, Decimal, StdError,
    };

    use crate::{
        state::{self, PauseInfo, Token},
        test::{DENOM_DEFAULT, DENOM_RESERVE, SENDER_ABUSER, SENDER_GOV},
    };

    use super::*;

    fn default_fee() -> state::Fee {
        state::Fee {
            collector: Addr::unchecked("collector"),
            mint: Default::default(),
            burn: Default::default(),
            stream: Default::default(),
            stream_last_collected_at: Default::default(),
        }
    }

    fn default_token() -> Token {
        Token {
            denom: DENOM_DEFAULT.to_string(),
            reserve_denom: DENOM_RESERVE.to_string(),
            total_supply: Uint128::new(100),
        }
    }

    #[test]
    fn test_handle_msg_check_authority() {
        let mut deps = mock_dependencies();

        let gov = Addr::unchecked(SENDER_GOV);
        GOV.save(deps.as_mut().storage, &gov).unwrap();

        let err = handle_msg(
            deps.as_mut(),
            mock_env(),
            mock_info(SENDER_ABUSER, &[]),
            GovMsg::Release {},
        )
        .unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});
    }

    mod pause_and_release {

        use super::*;

        fn setup(deps: DepsMut, gov: Option<&str>) {
            let gov = Addr::unchecked(gov.unwrap_or(SENDER_GOV));
            GOV.save(deps.storage, &gov).unwrap();
            PAUSED.save(deps.storage, &Default::default()).unwrap();
        }

        fn assert_pause_resp(resp: Response, sender: &str, expires_at: u64) {
            assert_eq!(
                resp.attributes,
                vec![
                    attr("method", "gov::pause"),
                    attr("executor", sender),
                    attr("expires_at", expires_at.to_string())
                ]
            );
        }

        fn assert_release_resp(resp: Response, sender: &str) {
            assert_eq!(
                resp.attributes,
                vec![attr("method", "gov::release"), attr("executor", sender),]
            )
        }

        #[test]
        fn test_pause_and_release() {
            let mut deps = mock_dependencies();

            let env = mock_env();
            let now = env.block.time.seconds();

            setup(deps.as_mut(), Some(SENDER_GOV));

            let expire = now + 1000;
            let sender = mock_info(SENDER_GOV, &[]);

            // pause
            let resp = pause(deps.as_mut(), env.clone(), sender.clone(), expire).unwrap();
            assert_pause_resp(resp, SENDER_GOV, expire);

            let paused = PAUSED.load(deps.as_ref().storage).unwrap();
            assert!(paused.paused);
            assert_eq!(paused.expires_at, Some(expire));

            // release
            let resp = release(deps.as_mut(), env.clone(), sender).unwrap();
            assert_release_resp(resp, SENDER_GOV);

            let paused = PAUSED.load(deps.as_ref().storage).unwrap();
            assert!(!paused.paused);
            assert!(paused.expires_at.is_none());
        }

        #[test]
        fn test_past_expiry() {
            let mut deps = mock_dependencies();

            let env = mock_env();
            let now = env.block.time.seconds();

            setup(deps.as_mut(), Some(SENDER_GOV));

            let sender = mock_info(SENDER_GOV, &[]);
            let err = pause(deps.as_mut(), env, sender, now - 1).unwrap_err();
            assert!(
                matches!(err, ContractError::InvalidArgument(msg) if msg == "expires_at must be in the future")
            );
        }

        #[test]
        fn test_double_pause() {
            let mut deps = mock_dependencies();

            let env = mock_env();
            let now = env.block.time.seconds();

            setup(deps.as_mut(), Some(SENDER_GOV));

            let sender = mock_info(SENDER_GOV, &[]);

            // first pause
            pause(deps.as_mut(), env.clone(), sender.clone(), now + 1).unwrap();

            // second pause
            assert_eq!(
                pause(deps.as_mut(), env, sender, now + 1).unwrap_err(),
                ContractError::Paused {}
            );
        }

        #[test]
        fn test_not_found_pause_info() {
            let mut deps = mock_dependencies();

            let env = mock_env();

            setup(deps.as_mut(), Some(SENDER_GOV));

            let sender = mock_info(SENDER_GOV, &[]);

            PAUSED.remove(deps.as_mut().storage);

            assert_eq!(
                release(deps.as_mut(), env, sender).unwrap_err(),
                ContractError::Std(StdError::not_found("ibcx_core::state::PauseInfo"))
            );
        }

        #[test]
        fn test_not_paused() {
            let mut deps = mock_dependencies();

            let env = mock_env();
            let now = env.block.time.seconds();

            setup(deps.as_mut(), Some(SENDER_GOV));

            PAUSED
                .save(
                    deps.as_mut().storage,
                    &PauseInfo {
                        paused: true,
                        expires_at: Some(now - 1000),
                    },
                )
                .unwrap();

            assert!(matches!(
                release(deps.as_mut(), env.clone(), mock_info(SENDER_GOV, &[])).unwrap_err(),
                ContractError::NotPaused {},
            ));
        }
    }

    mod update {

        use crate::test::{SENDER_OWNER, SENDER_VALID};

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

            let fee = fee.unwrap_or(default_fee());
            FEE.save(deps.storage, &fee).unwrap();

            let token = token.unwrap_or(default_token());
            TOKEN.save(deps.storage, &token).unwrap();

            for (denom, unit) in assets {
                ASSETS
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
                ContractError::InvalidArgument(
                    "reserve_denom must be zero in portfolio".to_string()
                )
            );

            // success
            ASSETS
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
}
