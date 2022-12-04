use std::collections::BTreeMap;

use cosmwasm_std::{coin, Coin, Decimal, Order, StdResult, Storage, Uint128};
use ibc_interface::MAX_LIMIT;

use crate::error::ContractError;

use super::{ASSETS, RESERVE_DENOM, TOKEN};

pub fn assert_assets(
    storage: &dyn Storage,
    funds: Vec<Coin>,
    desired: Uint128,
) -> Result<Vec<Coin>, ContractError> {
    let token = TOKEN.load(storage)?;
    let decimal = Uint128::new(10).checked_pow(token.decimal as u32)?;

    funds
        .iter()
        .map(|Coin { denom, amount }| {
            let mut unit = Decimal::from_ratio(
                ASSETS
                    .may_load(storage, denom.to_string())?
                    .unwrap_or_default(),
                decimal,
            );

            if denom == RESERVE_DENOM {
                unit += Decimal::from_ratio(
                    ASSETS
                        .may_load(storage, RESERVE_DENOM.to_string())?
                        .unwrap_or_default(),
                    decimal,
                );
            }

            let refund = amount.checked_sub(unit * desired)?;

            Ok(coin(refund.u128(), denom))
        })
        .collect::<Result<_, _>>()
}

pub fn set_assets(storage: &mut dyn Storage, assets: Vec<Coin>) -> Result<(), ContractError> {
    if assets.len() > MAX_LIMIT as usize {
        return Err(ContractError::InvalidAssetLength { limit: MAX_LIMIT });
    }

    for Coin { denom, amount } in assets {
        if denom == RESERVE_DENOM {
            return Err(ContractError::DenomReserved {
                reserved: RESERVE_DENOM.to_string(),
            });
        }
        match ASSETS.may_load(storage, denom.clone())? {
            Some(_) => return Err(ContractError::DenomReserved { reserved: denom }),
            None => ASSETS.save(storage, denom, &amount)?,
        }
    }

    Ok(())
}

pub fn get_assets(storage: &dyn Storage) -> StdResult<Vec<Coin>> {
    ASSETS
        .range(storage, None, None, Order::Ascending)
        .take(MAX_LIMIT as usize)
        .map(|item| {
            let (k, v) = item?;

            Ok(coin(v.u128(), k))
        })
        .collect::<StdResult<_>>()
}

pub fn get_units(storage: &dyn Storage) -> StdResult<Vec<(String, Decimal)>> {
    let token = TOKEN.load(storage)?;
    let decimal = Uint128::new(10).checked_pow(token.decimal as u32)?;

    Ok(get_assets(storage)?
        .into_iter()
        .map(|Coin { denom, amount }| (denom, Decimal::from_ratio(amount, decimal)))
        .collect())
}

pub fn get_redeem_amounts(storage: &dyn Storage, desired: Uint128) -> StdResult<Vec<Coin>> {
    let mut assets: BTreeMap<_, _> = get_units(storage)?
        .into_iter()
        .map(|(denom, unit)| (denom, unit * desired))
        .collect();

    let token = TOKEN.load(storage)?;

    if assets.contains_key(&token.reserve_denom) || assets.contains_key(RESERVE_DENOM) {
        let reserve_unit = assets.get(RESERVE_DENOM).copied().unwrap_or_default();
        assets
            .entry(token.reserve_denom)
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

    use crate::state::Token;

    use super::*;

    fn setup_test(storage: &mut dyn Storage) {
        TOKEN
            .save(
                storage,
                &Token {
                    denom: "test".to_string(),
                    decimal: 6,
                    reserve_denom: RESERVE_DENOM.to_string(),
                    total_supply: Uint128::new(1234),
                },
            )
            .unwrap();

        set_assets(
            storage,
            vec![coin(100, "ueur"), coin(10000, "ukrw"), coin(1000, "uusd")],
        )
        .unwrap();
    }

    #[test]
    fn test_get_assets() {
        let mut storage = MockStorage::new();

        setup_test(&mut storage);

        let assets = get_assets(&storage).unwrap();

        assert_eq!(
            assets,
            vec![
                ("ueur".to_string(), 100),
                ("ukrw".to_string(), 10000),
                ("uusd".to_string(), 1000),
            ]
            .into_iter()
            .map(|v| coin(v.1, v.0))
            .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn test_get_units() {
        let mut storage = MockStorage::new();

        setup_test(&mut storage);

        let token = TOKEN.load(&storage).unwrap();
        let units = get_units(&storage).unwrap();

        assert_eq!(
            units,
            vec![
                ("ueur".to_string(), 100),
                ("ukrw".to_string(), 10000),
                ("uusd".to_string(), 1000),
            ]
            .into_iter()
            .map(|(denom, amount)| (
                denom,
                Decimal::from_ratio(
                    Uint128::new(amount),
                    Uint128::new(10).checked_pow(token.decimal as u32).unwrap()
                )
            ))
            .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn test_get_redeem_amounts() {
        let mut storage = MockStorage::new();

        setup_test(&mut storage);

        let token = TOKEN.load(&storage).unwrap();
        let amounts = get_redeem_amounts(
            &storage,
            Uint128::new(10).checked_pow(token.decimal as u32).unwrap(),
        )
        .unwrap();
        let assets = get_assets(&storage).unwrap();

        assert_eq!(amounts, assets);
    }
}
