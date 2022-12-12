use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Uint128};
use ibc_interface::{core::GovMsg, types::SwapRoutes};

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

        UpdateReserveDenom { new_denom } => update_reserve_denom(deps, info, new_denom),
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

    token.reserve_denom = new_denom;

    TOKEN.save(deps.storage, &token)?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "gov::update_reserve_denom"),
        attr("executor", info.sender),
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
        denom,
        &TradeInfo {
            routes,
            cooldown,
            max_trade_amount,
            last_traded_at: None,
        },
    )?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "gov::update_trade_info"),
        attr("executor", info.sender),
    ]);

    Ok(resp)
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, Decimal, StdError,
    };

    use crate::state::{PauseInfo, Token};

    use super::*;

    #[test]
    fn test_pause() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let now = env.block.time.seconds();

        let gov = Addr::unchecked("gov");
        let abu = Addr::unchecked("abu");

        let info_gov = mock_info(gov.as_str(), &[]);
        let info_abu = mock_info(abu.as_str(), &[]);

        GOV.save(deps.as_mut().storage, &gov).unwrap();
        PAUSED
            .save(deps.as_mut().storage, &Default::default())
            .unwrap();

        let pause = |deps: DepsMut, env: Env, info: MessageInfo, expires_at: u64| {
            handle_msg(deps, env, info, GovMsg::Pause { expires_at })
        };

        // check arguments
        assert!(matches!(
            pause(deps.as_mut(), env.clone(), info_gov.clone(), now - 1000).unwrap_err(),
            ContractError::InvalidArgument(reason) if reason == "expires_at must be in the future",
        ));
        assert!(matches!(
            pause(deps.as_mut(), env.clone(), info_abu.clone(), now - 1000).unwrap_err(),
            ContractError::Unauthorized {},
        ));

        // check role
        assert!(matches!(
            pause(deps.as_mut(), env.clone(), info_abu, now + 1000).unwrap_err(),
            ContractError::Unauthorized {},
        ));
        pause(deps.as_mut(), env.clone(), info_gov.clone(), now + 1000).unwrap();

        // check already paused
        assert!(matches!(
            pause(deps.as_mut(), env, info_gov, now + 1000).unwrap_err(),
            ContractError::Paused {},
        ));

        assert_eq!(
            PAUSED.load(deps.as_ref().storage).unwrap(),
            PauseInfo {
                paused: true,
                expires_at: Some(now + 1000),
            }
        );
    }

    #[test]
    fn test_release() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let now = env.block.time.seconds();

        let gov = Addr::unchecked("gov");
        let info_gov = mock_info(gov.as_str(), &[]);
        let abu = Addr::unchecked("abu");
        let info_abu = mock_info(abu.as_str(), &[]);

        GOV.save(deps.as_mut().storage, &gov).unwrap();

        let release = |deps: DepsMut, env: Env, info: MessageInfo| {
            handle_msg(deps, env, info, GovMsg::Release {})
        };

        assert!(matches!(
            release(deps.as_mut(), env.clone(), info_abu).unwrap_err(),
            ContractError::Unauthorized {},
        ));
        assert!(matches!(
            release(deps.as_mut(), env.clone(), info_gov.clone()).unwrap_err(),
            ContractError::Std(StdError::NotFound { kind }) if kind == "ibc_core::state::PauseInfo",
        ));

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
            release(deps.as_mut(), env.clone(), info_gov.clone()).unwrap_err(),
            ContractError::NotPaused {},
        ));

        PAUSED
            .save(
                deps.as_mut().storage,
                &PauseInfo {
                    paused: true,
                    expires_at: Some(now + 1000),
                },
            )
            .unwrap();
        release(deps.as_mut(), env, info_gov).unwrap();

        PAUSED
            .load(deps.as_ref().storage)
            .unwrap()
            .assert_not_paused()
            .unwrap();
    }

    #[test]
    fn test_update_reserve_denom() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let gov = Addr::unchecked("gov");
        let info_gov = mock_info(gov.as_str(), &[]);
        let abu = Addr::unchecked("abu");
        let info_abu = mock_info(abu.as_str(), &[]);

        GOV.save(deps.as_mut().storage, &gov).unwrap();
        TOKEN
            .save(
                deps.as_mut().storage,
                &Token {
                    denom: "test".to_string(),
                    reserve_denom: "reserve".to_string(),
                    total_supply: Uint128::zero(),
                },
            )
            .unwrap();
        ASSETS
            .save(
                deps.as_mut().storage,
                RESERVE_DENOM.to_string(),
                &Decimal::from_ratio(10u128, 1u128),
            )
            .unwrap();

        let update_reserve_denom = |deps: DepsMut, info: MessageInfo, new_denom: String| {
            handle_msg(
                deps,
                env.clone(),
                info,
                GovMsg::UpdateReserveDenom { new_denom },
            )
        };

        assert!(matches!(
            update_reserve_denom(deps.as_mut(), info_abu, "no".to_string()).unwrap_err(),
            ContractError::Unauthorized {},
        ));
        assert!(matches!(
            update_reserve_denom(deps.as_mut(), info_gov.clone(), "no".to_string()).unwrap_err(),
            ContractError::InvalidArgument(reason) if reason == "reserve_denom must be zero in portfolio",
        ));

        ASSETS
            .save(
                deps.as_mut().storage,
                RESERVE_DENOM.to_string(),
                &Decimal::zero(),
            )
            .unwrap();

        update_reserve_denom(deps.as_mut(), info_gov, "yes".to_string()).unwrap();

        assert_eq!(
            TOKEN.load(deps.as_ref().storage).unwrap().reserve_denom,
            "yes"
        );
    }
}
