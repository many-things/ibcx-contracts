use std::collections::BTreeMap;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Coin, Decimal, StdResult, Uint128};

use crate::error::ContractError;

pub type Unit = (String, Decimal);

#[cw_serde]
#[derive(Default)]
pub struct Units(Vec<Unit>);

impl From<Vec<Unit>> for Units {
    fn from(v: Vec<Unit>) -> Self {
        Self(v)
    }
}

impl<'a> From<&'a [Unit]> for Units {
    fn from(v: &'a [Unit]) -> Self {
        Self(v.to_vec())
    }
}

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

    pub fn prune_zero(&mut self) {
        self.0.retain(|(_, unit)| !unit.is_zero());
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

    pub fn check_empty(&self) -> bool {
        self.0.iter().all(|(_, unit)| !unit.is_zero())
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
        let mut units: Units = vec![
            ("uatom".to_string(), Decimal::from_ratio(1u128, 2u128)),
            ("uosmo".to_string(), Decimal::from_ratio(1u128, 4u128)),
        ]
        .into();

        // pop uatom
        assert_eq!(
            units.pop_key("uatom"),
            Some(("uatom".to_string(), Decimal::from_ratio(1u128, 2u128)))
        );
        assert_eq!(
            units,
            vec![("uosmo".to_string(), Decimal::from_ratio(1u128, 4u128))].into()
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
        assert_eq!(units, vec![].into());
    }

    #[test]
    fn test_prune_zero() {
        let mut units: Units = vec![
            ("uatom".to_string(), Decimal::from_ratio(1u128, 2u128)),
            ("uosmo".to_string(), Decimal::from_ratio(1u128, 4u128)),
            ("ujuno".to_string(), Decimal::zero()),
        ]
        .into();

        units.prune_zero();

        assert_eq!(
            units,
            vec![
                ("uatom".to_string(), Decimal::from_ratio(1u128, 2u128)),
                ("uosmo".to_string(), Decimal::from_ratio(1u128, 4u128)),
            ]
            .into()
        );
    }

    #[test]
    fn test_check_empty() {
        let cases = [
            (
                vec![
                    ("uatom".to_string(), Decimal::from_ratio(1u128, 2u128)),
                    ("uosmo".to_string(), Decimal::zero()),
                ],
                false,
            ),
            (
                vec![
                    ("uatom".to_string(), Decimal::zero()),
                    ("uosmo".to_string(), Decimal::zero()),
                ],
                true,
            ),
            (vec![], true),
        ];

        for (units, expected) in cases {
            assert_eq!(Units::from(units).check_empty(), expected);
        }
    }

    #[test]
    fn test_check_duplication() {
        let cases = [
            (
                vec![
                    ("uatom".to_string(), Decimal::from_ratio(1u128, 2u128)),
                    ("uatom".to_string(), Decimal::from_ratio(1u128, 4u128)),
                ],
                true,
            ),
            (
                vec![
                    ("uatom".to_string(), Decimal::from_ratio(1u128, 2u128)),
                    ("uosmo".to_string(), Decimal::from_ratio(1u128, 4u128)),
                ],
                false,
            ),
        ];

        for (units, expect) in cases {
            assert_eq!(Units::from(units).check_duplicate(), expect);
        }
    }
}
