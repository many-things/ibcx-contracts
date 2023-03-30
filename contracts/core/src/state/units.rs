use std::{collections::BTreeMap, ops::Deref, str::FromStr, vec::IntoIter};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Coin, Decimal, StdError, Uint128};

use crate::{error::ContractError, StdResult};

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

impl<'a> From<Vec<(&'a str, &'a str)>> for Units {
    fn from(v: Vec<(&'a str, &'a str)>) -> Self {
        v.into_iter()
            .map(|(denom, unit)| (denom.to_string(), Decimal::from_str(unit).unwrap()))
            .collect()
    }
}

impl Extend<Unit> for Units {
    fn extend<T: IntoIterator<Item = Unit>>(&mut self, iter: T) {
        for elem in iter {
            self.0.push(elem)
        }
    }
}

impl Deref for Units {
    type Target = Vec<Unit>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for Units {
    type Item = Unit;
    type IntoIter = IntoIter<Unit>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Unit> for Units {
    fn from_iter<T: IntoIterator<Item = Unit>>(iter: T) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

impl Units {
    pub fn prettify(&self) -> String {
        self.iter()
            .map(|(denom, unit)| -> String { format!("(\"{denom}\",\"{unit}\")") })
            .fold("[".to_string(), |acc, s| acc + &s + ",")
            .trim_end_matches(',')
            .to_string()
            + "]"
    }

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

    pub fn sub_key(&mut self, k: &str, v: Decimal) -> StdResult<()> {
        match self.0.iter().position(|(denom, _)| denom == k) {
            Some(i) => {
                let unit = self.0.get_mut(i).unwrap();

                unit.1 = unit.1.checked_sub(v)?;
            }
            None => return Err(StdError::not_found("unit to sub").into()),
        }

        Ok(())
    }

    pub fn get_key(&self, key: &str) -> Option<&Unit> {
        if let Some(i) = self.0.iter().position(|u| u.0 == key) {
            self.0.get(i)
        } else {
            None
        }
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
        self.0.iter().all(|(_, unit)| unit.is_zero())
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
        let mut units: Units = vec![("uatom", "0.5"), ("uosmo", "0.25")].into();

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
        assert_eq!(units, Units::default());
    }

    #[test]
    fn test_prune_zero() {
        let mut units: Units = vec![("uatom", "0.5"), ("uosmo", "0.25"), ("ujuno", "0.0")].into();

        units.prune_zero();

        assert_eq!(units, vec![("uatom", "0.5"), ("uosmo", "0.25")].into());
    }

    #[test]
    fn test_check_empty() {
        let cases = [
            (vec![("uatom", "0.5"), ("uosmo", "0.0")], false),
            (vec![("uatom", "0.0"), ("uosmo", "0.0")], true),
            (vec![], true),
        ];

        for (units, expected) in cases {
            assert_eq!(Units::from(units).check_empty(), expected);
        }
    }

    #[test]
    fn test_check_duplication() {
        let cases = [
            (vec![("uatom", "0.5"), ("uatom", "0.25")], true),
            (vec![("uatom", "0.5"), ("uosmo", "0.25")], false),
        ];

        for (units, expect) in cases {
            assert_eq!(Units::from(units).check_duplicate(), expect);
        }
    }
}
