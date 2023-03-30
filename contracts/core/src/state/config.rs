use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Env};

use crate::{assert_sender, error::ContractError};

use super::PauseInfo;

#[cw_serde]
pub struct Config {
    pub gov: Addr,
    pub paused: PauseInfo,
    pub index_denom: String,
    pub reserve_denom: String,
}

impl Config {
    pub fn check_gov(&self, sender: &Addr) -> Result<(), ContractError> {
        assert_sender(&self.gov, sender)?;
        Ok(())
    }

    pub fn assert_paused(&self, env: &Env) -> Result<(), ContractError> {
        self.paused.clone().refresh(env)?.assert_paused()?;
        Ok(())
    }

    pub fn assert_not_paused(&self, env: &Env) -> Result<(), ContractError> {
        self.paused.clone().refresh(env)?.assert_not_paused()?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            gov: Addr::unchecked(""),
            paused: Default::default(),
            index_denom: Default::default(),
            reserve_denom: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr, Timestamp};

    use crate::{error::ContractError, state::PauseInfo};

    use super::Config;

    #[test]
    fn test_check_gov() {
        let config = Config {
            gov: Addr::unchecked("gov"),
            ..Default::default()
        };

        let cases = [
            (Addr::unchecked("user"), Err(ContractError::Unauthorized {})),
            (Addr::unchecked("gov"), Ok(())),
        ];

        for (sender, expected) in cases {
            assert_eq!(config.check_gov(&sender), expected);
        }
    }

    #[test]
    fn test_assert_paused() {
        use ContractError as E;

        let std_time = mock_env().block.time.seconds();

        let cases = [
            (false, None, Err(E::NotPaused), Ok(())),
            (true, Some(std_time - 1), Err(E::NotPaused), Ok(())),
            (true, Some(std_time + 1), Ok(()), Err(E::Paused)),
            (true, None, Ok(()), Err(E::Paused)),
        ];

        for (paused, expiry, expect_p, expect_np) in cases {
            let config = Config {
                paused: PauseInfo {
                    paused,
                    expires_at: expiry,
                },
                ..Default::default()
            };

            let mut env = mock_env();
            env.block.time = Timestamp::from_seconds(std_time);

            assert_eq!(config.assert_paused(&env), expect_p);
            assert_eq!(config.assert_not_paused(&env), expect_np);
        }
    }
}
