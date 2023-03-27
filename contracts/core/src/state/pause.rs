use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Env, StdResult};

use crate::error::ContractError;

#[cw_serde]
#[derive(Default)]
pub struct PauseInfo {
    pub paused: bool,
    pub expires_at: Option<u64>,
}

impl PauseInfo {
    pub fn refresh(self, env: &Env) -> StdResult<Self> {
        if self.paused {
            if let Some(expiry) = self.expires_at {
                if expiry <= env.block.time.seconds() {
                    return Ok(Default::default());
                }
            }
        }

        Ok(self)
    }

    pub fn assert_paused(self) -> Result<Self, ContractError> {
        if !self.paused {
            return Err(ContractError::NotPaused {});
        }

        Ok(self)
    }

    pub fn assert_not_paused(self) -> Result<Self, ContractError> {
        if self.paused {
            return Err(ContractError::Paused {});
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {

    use cosmwasm_std::testing::mock_env;

    use super::*;

    #[test]
    fn test_refresh() {
        let env = mock_env();

        // not paused and none expiry
        let info = PauseInfo {
            paused: false,
            expires_at: None,
        };
        let info_after = info.clone().refresh(&env).unwrap();
        assert_eq!(info, info_after);

        // not paused and some expiry
        let info = PauseInfo {
            expires_at: Some(env.block.time.seconds()),
            ..info
        };
        let info_after = info.clone().refresh(&env).unwrap();
        assert_eq!(info, info_after);

        // paused but no expiry
        let info = PauseInfo {
            paused: true,
            expires_at: None,
        };
        let info_after = info.clone().refresh(&env).unwrap();
        assert_eq!(info, info_after);

        // paused but not expired
        let info = PauseInfo {
            expires_at: Some(env.block.time.seconds() + 1),
            ..info
        };
        let info_after = info.clone().refresh(&env).unwrap();
        assert_eq!(info, info_after);

        // paused and expired
        let info = PauseInfo {
            expires_at: Some(env.block.time.seconds() - 1),
            ..info
        };
        let info_after = info.refresh(&env).unwrap();
        assert!(!info_after.paused);
        assert_eq!(info_after.expires_at, None);
    }

    #[test]
    fn test_assert_paused() {
        assert_eq!(
            PauseInfo {
                paused: false,
                expires_at: None,
            }
            .assert_paused()
            .unwrap_err(),
            ContractError::NotPaused {}
        );
    }

    #[test]
    fn test_assert_not_found() {
        assert_eq!(
            PauseInfo {
                paused: true,
                expires_at: None,
            }
            .assert_not_paused()
            .unwrap_err(),
            ContractError::Paused {}
        );
    }
}
