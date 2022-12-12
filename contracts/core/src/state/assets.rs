use std::collections::BTreeMap;

use cosmwasm_std::{coin, Coin, Decimal, Order, StdResult, Storage, Uint128};
use cw_storage_plus::Map;
use ibc_interface::MAX_LIMIT;

use crate::error::ContractError;

use super::{RESERVE_DENOM, TOKEN};

pub const ASSETS_PREFIX: &str = "assets";
pub const ASSETS: Map<String, Decimal> = Map::new(ASSETS_PREFIX);

pub fn assert_assets(
    storage: &dyn Storage,
    funds: Vec<Coin>,
    desired: Uint128,
) -> Result<Vec<Coin>, ContractError> {
    funds
        .iter()
        .map(|Coin { denom, amount }| {
            let mut unit = ASSETS.load(storage, denom.to_string())?;

            if denom == RESERVE_DENOM {
                unit += ASSETS
                    .may_load(storage, RESERVE_DENOM.to_string())?
                    .unwrap_or_default()
            }

            let refund = amount.checked_sub(unit * desired)?;

            Ok(coin(refund.u128(), denom))
        })
        .collect::<Result<_, _>>()
}

pub fn set_assets(
    storage: &mut dyn Storage,
    assets: Vec<(String, Decimal)>,
) -> Result<(), ContractError> {
    if assets.len() > MAX_LIMIT as usize {
        return Err(ContractError::InvalidAssetLength { limit: MAX_LIMIT });
    }

    for (denom, unit) in assets {
        if denom == RESERVE_DENOM {
            return Err(ContractError::DenomReserved {
                reserved: RESERVE_DENOM.to_string(),
            });
        }
        match ASSETS.may_load(storage, denom.clone())? {
            Some(_) => return Err(ContractError::DenomReserved { reserved: denom }),
            None => ASSETS.save(storage, denom, &unit)?,
        }
    }

    Ok(())
}

pub fn get_assets(storage: &dyn Storage) -> StdResult<Vec<(String, Decimal)>> {
    ASSETS
        .range(storage, None, None, Order::Ascending)
        .take(MAX_LIMIT as usize)
        .collect::<StdResult<_>>()
}

pub fn get_redeem_amounts(storage: &dyn Storage, desired: Uint128) -> StdResult<Vec<Coin>> {
    let mut assets: BTreeMap<_, _> = get_assets(storage)?
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
    use std::str::FromStr;

    use cosmwasm_std::testing::MockStorage;

    use crate::state::Token;

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

        set_assets(
            storage,
            vec![
                ("ueur".to_string(), Decimal::from_str("1.0").unwrap()),
                ("ukrw".to_string(), Decimal::from_str("1.2").unwrap()),
                ("uusd".to_string(), Decimal::from_str("1.5").unwrap()),
            ],
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
                ("ueur".to_string(), Decimal::from_str("1.0").unwrap()),
                ("ukrw".to_string(), Decimal::from_str("1.2").unwrap()),
                ("uusd".to_string(), Decimal::from_str("1.5").unwrap()),
            ]
        );
    }

    #[test]
    fn test_get_redeem_amounts() {
        let mut storage = MockStorage::new();

        setup_test(&mut storage);

        assert_eq!(
            get_redeem_amounts(&storage, Uint128::new(10)).unwrap(),
            vec![coin(10, "ueur"), coin(12, "ukrw"), coin(15, "uusd"),]
        );
    }
}
