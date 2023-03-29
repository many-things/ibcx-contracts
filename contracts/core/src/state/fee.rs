use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Addr, Coin, Decimal, Env, Uint128};

use crate::error::ContractError;

use super::Units;

#[cw_serde]
pub struct StreamingFee {
    pub rate: Decimal,
    pub collected: Vec<Coin>,
    pub last_collected_at: u64,
}

impl StreamingFee {
    pub fn collect(
        &mut self,
        env: &Env,
        index_units: Units,
        total_supply: Uint128,
    ) -> Result<(Units, Option<Vec<Coin>>), ContractError> {
        let (new_units, fee_units) = self.collect_inner(index_units, env.block.time.seconds())?;

        match fee_units {
            None => Ok((new_units, None)),
            Some(fee_units) => {
                self.realize_inner(fee_units, total_supply)?;

                let collected = self.collected;
                self.collected = vec![];

                Ok((new_units, Some(collected)))
            }
        }
    }

    fn collect_inner(
        &mut self,
        index_units: Units,
        now_in_sec: u64,
    ) -> Result<(Units, Option<Units>), ContractError> {
        let delta = now_in_sec - self.last_collected_at;
        if delta == 0 {
            return Ok((index_units, None)); // not collected
        }

        let rate = (Decimal::one() + self.rate)
            .checked_pow(delta as u32)?
            .checked_sub(Decimal::one())?;

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

        self.last_collected_at = now_in_sec;

        Ok((new_units, Some(collected)))
    }

    fn realize_inner(
        &mut self,
        collected: Units,
        total_supply: Uint128,
    ) -> Result<(), ContractError> {
        let realize = collected
            .clone()
            .into_iter()
            .map(|(denom, unit)| (denom, unit * total_supply))
            .collect::<Vec<_>>();

        for (denom, amount) in realize {
            match self.collected.iter().position(|c| c.denom == denom) {
                Some(i) => {
                    let origin = self.collected.get_mut(i).unwrap();

                    origin.amount = origin.amount.checked_add(amount)?;
                }
                None => self.collected.push(coin(amount.u128(), denom)),
            }
        }

        Ok(())
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

    use cosmwasm_std::{coin, Decimal};

    use super::StreamingFee;

    #[test]
    fn test_streaming_fee_collect() {
        let percent_15 = Decimal::from_str("0.000000000047529").unwrap();

        let mut streaming_fee = StreamingFee {
            rate: percent_15,
            collected: vec![],
            last_collected_at: 0,
        };

        let index_units = vec![
            ("uatom".to_string(), Decimal::from_str("2.0").unwrap()),
            ("uosmo".to_string(), Decimal::from_str("1.0").unwrap()),
        ]
        .into();

        let (mut new_units, fee_units) = streaming_fee.collect_inner(index_units, 86400).unwrap();

        {
            let (new_units_2, fee_units_2) =
                streaming_fee.collect_inner(index_units, 86400).unwrap();
            assert_eq!(fee_units, None);
            assert_eq!(new_units, new_units_2);
        }

        let cases = [
            ("uatom", "0.003", "0.004", "1.997", "1.996"),
            ("uosmo", "0.0015", "0.0016", "0.9985", "0.9984"),
        ];

        for (denom, fee_gt, fee_lt, next_gt, next_lt) in cases {
            let (_, fee) = fee_units.unwrap().pop_key(denom).unwrap();
            assert!(fee < Decimal::from_str(fee_lt).unwrap());
            assert!(fee > Decimal::from_str(fee_gt).unwrap());

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
        };

        let index_units = vec![
            ("uatom".to_string(), Decimal::from_str("2.0").unwrap()),
            ("uosmo".to_string(), Decimal::from_str("1.0").unwrap()),
        ]
        .into();

        let (new_units, fee_units) = streaming_fee.collect_inner(index_units, 0).unwrap();
        assert_eq!(fee_units, None);
        assert_eq!(index_units, new_units);
    }

    #[test]
    fn test_streaming_fee_realize() {
        let mut streaming_fee = StreamingFee {
            rate: Decimal::one(),
            collected: vec![],
            last_collected_at: 0,
        };

        let collected_units = vec![
            ("uatom".to_string(), Decimal::from_str("2.15").unwrap()),
            ("uosmo".to_string(), Decimal::from_str("1.20").unwrap()),
        ]
        .into();

        streaming_fee
            .realize_inner(collected_units, 100u128.into())
            .unwrap();
        assert_eq!(
            streaming_fee.collected,
            vec![coin(215u128, "uatom"), coin(120u128, "uosmo")]
        );
    }
}
