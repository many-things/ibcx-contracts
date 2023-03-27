use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

use crate::{error::ContractError, state::PAUSED};

pub fn pause(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    expires_at: u64,
) -> Result<Response, ContractError> {
    let mut pause_info = PAUSED
        .load(deps.storage)?
        .refresh(&env)?
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

pub fn release(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    PAUSED.load(deps.storage)?.refresh(&env)?.assert_paused()?;
    PAUSED.save(deps.storage, &Default::default())?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "gov::release"),
        attr("executor", info.sender),
    ]);

    Ok(resp)
}

#[cfg(test)]
mod tests {

    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, StdError,
    };

    use crate::{
        state::{PauseInfo, GOV},
        test::SENDER_GOV,
    };

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
        let resp = release(deps.as_mut(), env, sender).unwrap();
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
            ContractError::Std(StdError::not_found("ibcx_core::state::pause::PauseInfo"))
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
            release(deps.as_mut(), env, mock_info(SENDER_GOV, &[])).unwrap_err(),
            ContractError::NotPaused {},
        ));
    }
}
