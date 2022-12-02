use std::collections::BTreeMap;

use cosmwasm_std::{coin, Coin, Order, StdResult, Storage, Uint128};
use ibc_interface::MAX_LIMIT;

use crate::error::ContractError;

use super::{ASSETS, RESERVE_DENOM, TOKEN};

pub fn assert_assets(
    storage: &dyn Storage,
    funds: Vec<Coin>,
    desired: &Uint128,
) -> Result<Vec<Coin>, ContractError> {
    funds
        .iter()
        .map(|Coin { denom, amount }| {
            let og_unit = ASSETS
                .may_load(storage, denom.to_string())?
                .unwrap_or_default();

            let needed = if denom == RESERVE_DENOM {
                let rv_unit = ASSETS
                    .may_load(storage, RESERVE_DENOM.to_string())?
                    .unwrap_or_default();

                (og_unit + rv_unit) * desired
            } else {
                og_unit * desired
            };

            let refund = amount.checked_sub(needed)?;

            Ok(coin(refund.u128(), denom))
        })
        .collect::<Result<_, _>>()
}

pub fn set_assets(
    storage: &mut dyn Storage,
    assets: Vec<(String, Uint128)>,
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

pub fn get_redeem_assets(storage: &dyn Storage, desired: Uint128) -> StdResult<Vec<Coin>> {
    let mut assets: BTreeMap<_, _> = get_assets(storage)?
        .into_iter()
        .map(|Coin { denom, amount }| (denom, amount * desired))
        .collect();

    let token = TOKEN.load(storage)?;

    if assets.contains_key(&token.reserve_denom) || assets.contains_key(RESERVE_DENOM) {
        let reserve_unit = assets
            .get(&token.reserve_denom)
            .copied()
            .unwrap_or_default();
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
