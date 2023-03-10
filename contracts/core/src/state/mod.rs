mod rebalance;
mod units;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Env, StdResult, Storage, Uint128};
use cw_storage_plus::Item;
use ibcx_interface::types::Units;

use crate::error::ContractError;

pub use crate::state::rebalance::{
    Rebalance, TradeInfo, LATEST_REBALANCE_ID, REBALANCES, RESERVE_BUFFER, TRADE_INFOS,
};
pub use crate::state::units::{assert_units, get_redeem_amounts, get_units, set_units, UNITS};

pub const RESERVE_DENOM: &str = "reserve";

pub const GOV_KEY: &str = "gov";
pub const GOV: Item<Addr> = Item::new(GOV_KEY);

pub const FEE_KEY: &str = "fee";
pub const FEE: Item<Fee> = Item::new(FEE_KEY);

pub const TOKEN_KEY: &str = "token";
pub const TOKEN: Item<Token> = Item::new(TOKEN_KEY);

pub const PAUSED_KEY: &str = "paused";
pub const PAUSED: Item<PauseInfo> = Item::new(PAUSED_KEY);

#[cw_serde]
pub struct Fee {
    // address of fee collector
    pub collector: Addr,
    pub mint: Option<Decimal>,
    pub burn: Option<Decimal>,
    // secondly rate
    // ex) APY %0.15 = 1 - (1 + 0.0015)^(1 / (86400 * 365)) = 0.000000000047529
    pub stream: Option<Decimal>,
    pub stream_collected: Units,
    pub stream_last_collected_at: u64,
}

impl Fee {
    pub fn calculate_streaming_fee(
        &self,
        units: Units,
        now: u64,
    ) -> Result<(Units, Option<Units>), ContractError> {
        if let Some(stream) = self.stream {
            let elapsed = now - self.stream_last_collected_at;
            if elapsed > 0 {
                // 1. fetch rate & add one - ex) 1 + 0.000000000047529 => 1.000000000047529
                // 2. pow elapsed time - ex) (1.000000000047529)^86400 => 1.000004106521961
                // 3. subtract one - ex) 1.000004106521961 - 1 => 0.000004106521961
                let rate = (Decimal::one() + stream)
                    .checked_pow(elapsed as u32)?
                    .checked_sub(Decimal::one())?;

                let (after, fee) = units
                    .into_iter()
                    .map(|(denom, unit)| {
                        let after = unit.checked_mul(Decimal::one().checked_sub(rate)?)?;
                        Ok(((denom.clone(), after), (denom, unit.checked_sub(after)?)))
                    })
                    .collect::<Result<Vec<_>, ContractError>>()?
                    .into_iter()
                    .unzip();

                return Ok((after, Some(fee)));
            }
        }

        // return units
        Ok((units, None))
    }
}

#[cw_serde]
pub struct Token {
    pub denom: String,
    pub reserve_denom: String,
    pub total_supply: Uint128,
}

#[cw_serde]
#[derive(Default)]
pub struct PauseInfo {
    pub paused: bool,
    pub expires_at: Option<u64>,
}

impl PauseInfo {
    pub fn refresh(self, storage: &mut dyn Storage, env: &Env) -> StdResult<Self> {
        if self.paused {
            if let Some(expiry) = self.expires_at {
                if expiry <= env.block.time.seconds() {
                    PAUSED.save(storage, &Default::default())?;
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
mod test {
    use std::str::FromStr;

    use cosmwasm_std::{testing::mock_env, MemoryStorage};

    use super::*;

    #[test]
    fn test_fee_overflow() {
        let units = vec![
            ("uusd".to_string(), Decimal::from_ratio(10u128, 1u128)),
            ("ukrw".to_string(), Decimal::from_ratio(15u128, 1u128)),
            ("ujpy".to_string(), Decimal::from_ratio(20u128, 1u128)),
        ];

        let fee = Fee {
            collector: Addr::unchecked("collector"),
            mint: None,
            burn: None,
            // secondly rate
            // ex) APY %0.15 = 1 - (1 + 0.0015)^(1 / (86400 * 365)) = 0.000000000047529
            stream: Some(Decimal::from_str("0.000000000047529").unwrap()),
            stream_collected: Units::new(),
            stream_last_collected_at: 0,
        };

        // 1 day
        let day = 86400;
        fee.calculate_streaming_fee(units.clone(), day).unwrap();
        // 1 month
        let month = 30 * day;
        fee.calculate_streaming_fee(units.clone(), month).unwrap();
        // 1 year
        let year = 365 * day;
        fee.calculate_streaming_fee(units, year).unwrap();
    }

    #[test]
    fn test_refresh() {
        let mut storage = MemoryStorage::new();

        let env = mock_env();

        // not paused and none expiry
        let info = PauseInfo {
            paused: false,
            expires_at: None,
        };
        let info_after = info.clone().refresh(&mut storage, &env).unwrap();
        assert_eq!(info, info_after);

        // not paused and some expiry
        let info = PauseInfo {
            expires_at: Some(env.block.time.seconds()),
            ..info
        };
        let info_after = info.clone().refresh(&mut storage, &env).unwrap();
        assert_eq!(info, info_after);

        // paused but no expiry
        let info = PauseInfo {
            paused: true,
            expires_at: None,
        };
        let info_after = info.clone().refresh(&mut storage, &env).unwrap();
        assert_eq!(info, info_after);

        // paused but not expired
        let info = PauseInfo {
            expires_at: Some(env.block.time.seconds() + 1),
            ..info
        };
        let info_after = info.clone().refresh(&mut storage, &env).unwrap();
        assert_eq!(info, info_after);

        // paused and expired
        let info = PauseInfo {
            expires_at: Some(env.block.time.seconds() - 1),
            ..info
        };
        let info_after = info.refresh(&mut storage, &env).unwrap();
        assert!(!info_after.paused);
        assert_eq!(info_after.expires_at, None);
        assert_eq!(PAUSED.load(&storage).unwrap(), info_after);
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
