use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

use crate::{
    error::ValidationError,
    state::{Config, PauseInfo, CONFIG},
    StdResult,
};

pub fn pause(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    expires_at: Option<u64>,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    config.check_gov(&info.sender)?;
    config.assert_not_paused(&env)?;

    if let Some(expires_at) = expires_at {
        if env.block.time.seconds() >= expires_at {
            return Err(ValidationError::invalid_pause_info("expiry must be in the future").into());
        }
    }

    let new_pause_info = PauseInfo {
        paused: true,
        expires_at,
    };

    CONFIG.save(
        deps.storage,
        &Config {
            paused: new_pause_info,
            ..config
        },
    )?;

    // response
    let attrs = vec![
        attr("method", "gov::pause"),
        attr("executor", info.sender),
        attr(
            "expires_at",
            expires_at
                .map(|v| v.to_string())
                .as_deref()
                .unwrap_or("never"),
        ),
    ];

    let resp = Response::new().add_attributes(attrs);

    Ok(resp)
}

pub fn release(deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    config.check_gov(&info.sender)?;
    config.assert_paused(&env)?;

    CONFIG.save(
        deps.storage,
        &Config {
            paused: Default::default(),
            ..config
        },
    )?;

    // response
    let attrs = vec![
        attr("method", "gov::release"),
        attr("executor", info.sender),
    ];

    let resp = Response::new().add_attributes(attrs);

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        attr,
        testing::{mock_dependencies, mock_env, mock_info},
        Timestamp,
    };

    use crate::{
        error::{ContractError, ValidationError},
        execute::gov::pause::release,
        state::{tests::mock_config, Config, PauseInfo, CONFIG},
    };

    use super::pause;

    #[test]
    fn test_pause() {
        let std_time = mock_env().block.time.seconds();

        let cases = [
            // not paused
            (
                PauseInfo {
                    paused: false,
                    expires_at: None,
                },
                None,
                std_time,
                Ok(vec![
                    attr("method", "gov::pause"),
                    attr("executor", "gov"),
                    attr("expires_at", "never"),
                ]),
            ),
            (
                PauseInfo {
                    paused: false,
                    expires_at: None,
                },
                Some(std_time - 1),
                std_time,
                Err(ValidationError::invalid_pause_info("expiry must be in the future").into()),
            ),
            // paused
            (
                PauseInfo {
                    paused: true,
                    expires_at: None,
                },
                None,
                std_time,
                Err(ContractError::Paused {}),
            ),
            (
                PauseInfo {
                    paused: true,
                    expires_at: Some(std_time - 1),
                },
                Some(std_time - 1),
                std_time,
                Err(ValidationError::invalid_pause_info("expiry must be in the future").into()),
            ),
            (
                PauseInfo {
                    paused: true,
                    expires_at: Some(std_time - 1),
                },
                Some(std_time + 1),
                std_time,
                Ok(vec![
                    attr("method", "gov::pause"),
                    attr("executor", "gov"),
                    attr("expires_at", (std_time + 1).to_string()),
                ]),
            ),
        ];

        let mut deps = mock_dependencies();

        for (paused, expiry, exec_at, expected) in cases {
            CONFIG
                .save(
                    deps.as_mut().storage,
                    &Config {
                        paused,
                        ..mock_config()
                    },
                )
                .unwrap();

            let mut env = mock_env();
            env.block.time = Timestamp::from_seconds(exec_at);

            let res = pause(deps.as_mut(), env, mock_info("gov", &[]), expiry);
            assert_eq!(res.map(|v| v.attributes), expected);
        }
    }

    #[test]
    fn test_release() {
        let std_time = mock_env().block.time.seconds();

        let cases = [
            // not paused
            (
                PauseInfo {
                    paused: false,
                    expires_at: None,
                },
                std_time,
                Err(ContractError::NotPaused {}),
            ),
            // paused
            (
                PauseInfo {
                    paused: true,
                    expires_at: None,
                },
                std_time,
                Ok(vec![
                    attr("method", "gov::release"),
                    attr("executor", "gov"),
                ]),
            ),
            (
                PauseInfo {
                    paused: true,
                    expires_at: Some(std_time - 1),
                },
                std_time,
                Err(ContractError::NotPaused {}),
            ),
        ];

        let mut deps = mock_dependencies();

        for (paused, exec_at, expected) in cases {
            CONFIG
                .save(
                    deps.as_mut().storage,
                    &Config {
                        paused,
                        ..mock_config()
                    },
                )
                .unwrap();

            let mut env = mock_env();
            env.block.time = Timestamp::from_seconds(exec_at);

            let res = release(deps.as_mut(), env, mock_info("gov", &[]));
            assert_eq!(res.map(|v| v.attributes), expected);
        }
    }
}
