use std::collections::BTreeMap;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Coin, Decimal, StdResult, Uint128};

use crate::error::ContractError;

pub type Unit = (String, Decimal);

#[cw_serde]
#[derive(Default)]
pub struct Units(pub Vec<Unit>);

impl Units {
    pub fn add_key(&mut self, k: &str, v: Decimal) -> StdResult<()> {
        match self.0.iter().position(|(denom, _)| denom == k) {
            Some(i) => {
                let unit = self.0.get_mut(i).unwrap();

                unit.1 = unit.1.checked_add(v)?;
            }
            None => self.0.push((k.to_string(), v)),
        }

        Ok(())
    }

    pub fn pop_key(&mut self, key: &str) -> Option<Unit> {
        if let Some(i) = self.0.iter().position(|u| u.0 == key) {
            Some(self.0.remove(i))
        } else {
            None
        }
    }

    pub fn calc_require_amount(&self, index_amount: Uint128) -> Vec<Coin> {
        self.0
            .iter()
            .map(|(denom, unit)| coin((*unit * index_amount).u128(), denom))
            .collect::<Vec<_>>()
    }

    pub fn calc_refund_amount(
        &self,
        mut funds: Vec<Coin>,
        index_amount: Uint128,
    ) -> Result<Vec<Coin>, ContractError> {
        let required = self.calc_require_amount(index_amount);

        for Coin { denom, amount } in required {
            match funds.iter().position(|v| v.denom == denom) {
                Some(i) => {
                    let fund = funds.get_mut(i).unwrap();

                    fund.amount = fund.amount.checked_sub(amount)?;
                }
                None => return Err(ContractError::InsufficientFunds(denom)),
            }
        }

        Ok(funds.into_iter().filter(|v| !v.amount.is_zero()).collect())
    }

    pub fn check_duplicate(&self) -> bool {
        let mut uniq = BTreeMap::new();

        !self
            .0
            .iter()
            .all(|(denom, unit)| uniq.insert(denom.clone(), unit).is_none())
    }
}

#[cfg(test)]
mod tests {
    use super::Units;
    use cosmwasm_std::Decimal;

    #[test]
    fn test_pop_key() {
        let mut units = Units(vec![
            ("uatom".to_string(), Decimal::from_ratio(1u128, 2u128)),
            ("uosmo".to_string(), Decimal::from_ratio(1u128, 4u128)),
        ]);

        // pop uatom
        assert_eq!(
            units.pop_key("uatom"),
            Some(("uatom".to_string(), Decimal::from_ratio(1u128, 2u128)))
        );
        assert_eq!(
            units,
            Units(vec![(
                "uosmo".to_string(),
                Decimal::from_ratio(1u128, 4u128)
            )])
        );

        // pop once again
        assert_eq!(
            units.pop_key("uatom"),
            None,
            "should not be able to pop the same key twice"
        );

        // pop uosmo
        assert_eq!(
            units.pop_key("uosmo"),
            Some(("uosmo".to_string(), Decimal::from_ratio(1u128, 4u128)))
        );
        assert_eq!(units, Units(vec![]));
    }

    #[test]
    fn test_check_duplication() {
        let cases = [
            (
                Units(vec![
                    ("uatom".to_string(), Decimal::from_ratio(1u128, 2u128)),
                    ("uatom".to_string(), Decimal::from_ratio(1u128, 4u128)),
                ]),
                true,
            ),
            (
                Units(vec![
                    ("uatom".to_string(), Decimal::from_ratio(1u128, 2u128)),
                    ("uosmo".to_string(), Decimal::from_ratio(1u128, 4u128)),
                ]),
                false,
            ),
        ];

        for (units, expect) in cases {
            assert_eq!(units.check_duplicate(), expect);
        }
    }
}
