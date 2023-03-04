use std::collections::BTreeMap;

use cosmwasm_std::{coin, Coin, Decimal, Order, StdResult, Storage, Uint128};
use cw_storage_plus::Map;
use ibcx_interface::{types::Units, MAX_LIMIT};

use crate::error::ContractError;

use super::RESERVE_DENOM;

pub const UNITS_PREFIX: &str = "assets";
pub const UNITS: Map<String, Decimal> = Map::new(UNITS_PREFIX);

pub fn set_units(storage: &mut dyn Storage, assets: Units) -> Result<(), ContractError> {
    if assets.len() > MAX_LIMIT as usize {
        return Err(ContractError::InvalidAssetLength { limit: MAX_LIMIT });
    }

    for (denom, unit) in assets {
        if denom == RESERVE_DENOM {
            return Err(ContractError::DenomReserved {
                reserved: RESERVE_DENOM.to_string(),
            });
        }
        match UNITS.may_load(storage, denom.clone())? {
            Some(_) => return Err(ContractError::DenomReserved { reserved: denom }),
            None => UNITS.save(storage, denom, &unit)?,
        }
    }

    Ok(())
}

pub fn get_units(storage: &dyn Storage) -> StdResult<Units> {
    UNITS
        .range(storage, None, None, Order::Ascending)
        .take(MAX_LIMIT as usize)
        .collect::<StdResult<_>>()
}

pub fn assert_units(
    assets: Units,
    funds: Vec<Coin>,
    desired: Uint128,
) -> Result<Vec<Coin>, ContractError> {
    assets
        .into_iter()
        .map(|(denom, unit)| {
            let received = match funds.iter().find(|v| v.denom == denom) {
                Some(c) => c,
                None => return Err(ContractError::InsufficientFunds(denom)),
            };

            let refund = received
                .amount
                .checked_sub(unit * desired)
                .map_err(|_| ContractError::InsufficientFunds(denom.clone()))?;

            Ok(coin(refund.u128(), denom))
        })
        .collect()
}

pub fn get_redeem_amounts(
    assets: Units,
    reserve_denom: &str,
    desired: Uint128,
) -> StdResult<Vec<Coin>> {
    let mut assets: BTreeMap<_, _> = assets
        .into_iter()
        .map(|(denom, unit)| (denom, unit * desired))
        .collect();

    if assets.contains_key(reserve_denom) || assets.contains_key(RESERVE_DENOM) {
        let reserve_unit = assets.get(RESERVE_DENOM).copied().unwrap_or_default();
        assets
            .entry(reserve_denom.to_string())
            .and_modify(|v| *v += reserve_unit)
            .or_insert(reserve_unit);
    }

    Ok(assets
        .into_iter()
        .map(|(denom, amount)| coin(amount.u128(), denom))
        .collect())
}

#[cfg(test)]
mod test {
    use cosmwasm_std::testing::MockStorage;

    use crate::{
        state::{Token, TOKEN},
        test::{register_units, to_units},
    };

    use super::*;

    fn setup_test(storage: &mut dyn Storage) {
        TOKEN
            .save(
                storage,
                &Token {
                    denom: "test".to_string(),
                    reserve_denom: RESERVE_DENOM.to_string(),
                    total_supply: Uint128::new(1234),
                },
            )
            .unwrap();

        register_units(
            storage,
            &[("ueur", "1.0"), ("ukrw", "1.2"), ("uusd", "1.5")],
        );
    }

    #[test]
    fn test_set_assets() {
        let mut storage = MockStorage::new();

        // check limit exceeds
        let assets = to_units(
            [("ukrw", "1.0")]
                .repeat((MAX_LIMIT + 1).try_into().unwrap())
                .as_slice(),
        );
        let err = set_units(&mut storage, assets).unwrap_err();
        assert_eq!(err, ContractError::InvalidAssetLength { limit: MAX_LIMIT });

        // check reserved denom
        let assets = to_units(&[(RESERVE_DENOM, "1.0")]);
        let err = set_units(&mut storage, assets).unwrap_err();
        assert_eq!(
            err,
            ContractError::DenomReserved {
                reserved: RESERVE_DENOM.to_string()
            }
        );

        // check denom duplication
        let assets = to_units(&[("ukrw", "1.0"), ("ukrw", "1.0")]);
        let err = set_units(&mut storage, assets).unwrap_err();
        assert_eq!(
            err,
            ContractError::DenomReserved {
                reserved: "ukrw".to_string()
            }
        );
        UNITS.remove(&mut storage, "ukrw".to_string());

        // ok
        let assets = to_units(&[("ukrw", "1.0"), ("ueur", "1.2"), ("uusd", "1.5")]);
        set_units(&mut storage, assets.clone()).unwrap();
        for (denom, unit) in assets {
            assert_eq!(UNITS.load(&storage, denom).unwrap(), unit);
        }
    }

    #[test]
    fn test_get_assets() {
        let mut storage = MockStorage::new();

        setup_test(&mut storage);

        let assets = get_units(&storage).unwrap();

        assert_eq!(
            assets,
            to_units(&[("ueur", "1.0"), ("ukrw", "1.2"), ("uusd", "1.5"),])
        );
    }

    #[test]
    fn test_assert_assets() {
        let mut storage = MockStorage::new();

        setup_test(&mut storage);

        let refund = assert_units(
            get_units(&storage).unwrap(),
            vec![
                coin(12000, "ueur"),
                coin(15000, "ukrw"),
                coin(20000, "uusd"),
            ],
            Uint128::new(10000),
        )
        .unwrap();
        assert_eq!(
            refund,
            vec![coin(2000, "ueur"), coin(3000, "ukrw"), coin(5000, "uusd"),]
        );

        let err =
            assert_units(get_units(&storage).unwrap(), vec![], Uint128::new(10000)).unwrap_err();
        assert_eq!(err, ContractError::InsufficientFunds("ueur".to_string()));
    }

    #[test]
    fn test_get_redeem_amounts() {
        let mut storage = MockStorage::new();

        setup_test(&mut storage);

        assert_eq!(
            get_redeem_amounts(
                get_units(&storage).unwrap(),
                RESERVE_DENOM,
                Uint128::new(10)
            )
            .unwrap(),
            vec![coin(10, "ueur"), coin(12, "ukrw"), coin(15, "uusd"),]
        );
    }
}
