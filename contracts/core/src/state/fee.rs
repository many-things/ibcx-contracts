use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};
use ibcx_interface::types::Units;

use crate::error::ContractError;

#[cw_serde]
pub struct StreamingFee {
    pub rate: Decimal,
    pub collected: Units,
    pub last_collected_at: u64,
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{Addr, Decimal};
    use ibcx_interface::types::Units;

    use super::Fee;

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
}
