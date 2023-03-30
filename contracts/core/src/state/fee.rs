use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Addr, Coin, Decimal, Uint128};

use crate::{
    error::{ContractError, ValidationError},
    StdResult,
};

use super::Units;

// APR 100%
const MAX_STREAMING_FEE_RATE: &str = "0.000000021979553";

fn rate_checker(field: &str, rate: Option<Decimal>) -> StdResult<()> {
    if rate.is_none() {
        return Ok(());
    }

    let rate = rate.unwrap();

    if !(Decimal::zero() < rate && rate < Decimal::one()) {
        return Err(ValidationError::InvalidFee {
            field: field.to_string(),
            reason: format!("{rate} is invalid"),
        }
        .into());
    }

    Ok(())
}

#[cw_serde]
#[derive(Default)]
pub struct StreamingFee {
    pub rate: Decimal,
    pub collected: Vec<Coin>,
    pub freeze: bool,
    pub last_collected_at: u64,
}

impl StreamingFee {
    pub fn collect(
        &mut self,
        index_units: Units,
        now_in_sec: u64,
        total_supply: Uint128,
    ) -> Result<(Units, Option<Vec<Coin>>), ContractError> {
        let delta = now_in_sec - self.last_collected_at;
        if self.freeze || delta == 0 {
            return Ok((index_units, None)); // not collected
        }

        let rate = (Decimal::one() + self.rate)
            .checked_pow(delta as u32)?
            .checked_sub(Decimal::one())?;

        // calculate new units with fee and amount of collect fees
        let (new_units, collected): (Units, Units) = index_units
            .into_iter()
            .map(|(denom, unit)| {
                let fee_unit = unit.checked_mul(rate)?;
                let new_unit = unit.checked_sub(fee_unit)?;

                Ok(((denom.clone(), new_unit), (denom, fee_unit)))
            })
            .collect::<Result<Vec<_>, ContractError>>()?
            .into_iter()
            .unzip();

        // apply to self.collected
        for (denom, amount) in collected
            .into_iter()
            .map(|(denom, unit)| (denom, unit * total_supply))
            .collect::<Vec<_>>()
        {
            match self.collected.iter().position(|c| c.denom == denom) {
                Some(i) => {
                    let origin = self.collected.get_mut(i).unwrap();

                    origin.amount = origin.amount.checked_add(amount)?;
                }
                None => self.collected.push(coin(amount.u128(), denom)),
            }
        }

        self.last_collected_at = now_in_sec;

        Ok((new_units, Some(self.collected.clone())))
    }
}

#[cw_serde]
pub struct Fee {
    // address of fee collector
    pub collector: Addr,
    pub mint_fee: Option<Decimal>,
    pub burn_fee: Option<Decimal>,
    pub streaming_fee: Option<StreamingFee>,
}

impl Fee {
    pub fn check_rates(&self) -> StdResult<()> {
        rate_checker("mint_fee", self.mint_fee)?;
        rate_checker("burn_fee", self.burn_fee)?;
        rate_checker("streaming_fee", self.streaming_fee.as_ref().map(|v| v.rate))?;

        if let Some(streaming_fee) = &self.streaming_fee {
            let max = Decimal::from_str(MAX_STREAMING_FEE_RATE)?;
            let rate = streaming_fee.rate;
            if max < rate {
                return Err(ValidationError::InvalidFee {
                    field: "streaming_fee".to_string(),
                    reason: format!("{rate} is greater than max rate {max}"),
                }
                .into());
            }
        }

        Ok(())
    }
}

impl Default for Fee {
    fn default() -> Self {
        Self {
            collector: Addr::unchecked(""),
            mint_fee: Default::default(),
            burn_fee: Default::default(),
            streaming_fee: Default::default(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{Decimal, Uint128};

    use crate::{error::ValidationError, state::Units};

    use super::{Fee, StreamingFee, MAX_STREAMING_FEE_RATE};

    #[test]
    fn test_streaming_fee_collect() {
        let percent_15 = Decimal::from_str("0.000000000047529").unwrap();

        let mut streaming_fee = StreamingFee {
            rate: percent_15,
            collected: vec![],
            last_collected_at: 0,
            freeze: false,
        };

        let index_units: Units = vec![("uatom", "20000.0"), ("uosmo", "10000.0")].into();

        let (mut new_units, fee_units) = streaming_fee
            .collect(index_units, 86400 * 365, 100u128.into())
            .unwrap();

        {
            let (new_units_2, fee_units_2) = streaming_fee
                .collect(new_units.clone(), 86400 * 365, 100u128.into())
                .unwrap();
            assert_eq!(fee_units_2, None);
            assert_eq!(new_units, new_units_2);
        }

        let cases = [
            ("uatom", 2998u128, 3000u128, "19970", "19970.1"),
            ("uosmo", 1498u128, 1500u128, "9985", "9985.1"),
        ];

        let mut fee_units = fee_units.unwrap();
        for (denom, fee_gt, fee_lt, next_gt, next_lt) in cases {
            let fee = match fee_units.iter().position(|v| v.denom == denom) {
                Some(i) => fee_units.remove(i),
                None => panic!("no way"),
            };
            assert!(fee.amount < Uint128::new(fee_lt));
            assert!(fee.amount > Uint128::new(fee_gt));

            let fee = match streaming_fee
                .collected
                .iter()
                .position(|v| v.denom == denom)
            {
                Some(i) => streaming_fee.collected.remove(i),
                None => panic!("no way"),
            };
            assert!(fee.amount < Uint128::new(fee_lt));
            assert!(fee.amount > Uint128::new(fee_gt));

            let (_, next) = new_units.pop_key(denom).unwrap();
            assert!(next < Decimal::from_str(next_lt).unwrap());
            assert!(next > Decimal::from_str(next_gt).unwrap());
        }
    }

    #[test]
    fn test_streaming_fee_collect_no_delta() {
        let mut streaming_fee = StreamingFee {
            rate: Decimal::one(),
            collected: vec![],
            last_collected_at: 0,
            freeze: false,
        };

        let index_units: Units = vec![("uatom", "2.0"), ("uosmo", "1.0")].into();

        let (new_units, fee_units) = streaming_fee
            .collect(index_units.clone(), 0, 100u128.into())
            .unwrap();
        assert_eq!(fee_units, None);
        assert_eq!(index_units, new_units);
    }

    #[test]
    fn test_streaming_fee_collect_freeze() {
        let mut streaming_fee = StreamingFee {
            rate: Decimal::one(),
            collected: vec![],
            last_collected_at: 0,
            freeze: true,
        };

        let index_units: Units = vec![("uatom", "2.0"), ("uosmo", "1.0")].into();

        let (new_units, fee_units) = streaming_fee
            .collect(index_units.clone(), 86400, 100u128.into())
            .unwrap();
        assert_eq!(fee_units, None);
        assert_eq!(index_units, new_units);
    }

    #[test]
    fn test_fee_check_rates() {
        let cases = [
            (
                Fee {
                    mint_fee: Some(Decimal::from_str("1.1").unwrap()),
                    ..Default::default()
                },
                Err(ValidationError::invalid_fee("mint_fee", "1.1 is invalid").into()),
            ),
            (
                Fee {
                    mint_fee: Some(Decimal::from_str("0.9").unwrap()),
                    ..Default::default()
                },
                Ok(()),
            ),
            (
                Fee {
                    burn_fee: Some(Decimal::from_str("1.1").unwrap()),
                    ..Default::default()
                },
                Err(ValidationError::invalid_fee("burn_fee", "1.1 is invalid").into()),
            ),
            (
                Fee {
                    mint_fee: Some(Decimal::from_str("0.9").unwrap()),
                    ..Default::default()
                },
                Ok(()),
            ),
            (
                Fee {
                    streaming_fee: Some(StreamingFee {
                        rate: Decimal::from_str("1.1").unwrap(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                Err(ValidationError::invalid_fee("streaming_fee", "1.1 is invalid").into()),
            ),
            (
                Fee {
                    streaming_fee: Some(StreamingFee {
                        rate: Decimal::from_str(MAX_STREAMING_FEE_RATE).unwrap()
                            + Decimal::from_str("0.000000000047529").unwrap(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                Err(ValidationError::invalid_fee(
                    "streaming_fee",
                    format!("0.000000022027082 is greater than max rate {MAX_STREAMING_FEE_RATE}"),
                )
                .into()),
            ),
            (
                Fee {
                    streaming_fee: Some(StreamingFee {
                        rate: Decimal::from_str("0.000000000047529").unwrap(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                Ok(()),
            ),
        ];

        for (fee, expected) in cases {
            assert_eq!(fee.check_rates(), expected);
        }
    }
}
