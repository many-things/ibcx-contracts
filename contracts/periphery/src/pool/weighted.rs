use std::ops::Div;
use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::StdError;
use cosmwasm_std::{Coin, Decimal, StdResult, Uint128};
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmosisCoin;
use osmosis_std::types::osmosis::gamm;
use osmosis_std::types::osmosis::gamm::v1beta1::PoolAsset as OsmosisPoolAsset;
use rust_decimal::Decimal as RustDecimal;
use rust_decimal::MathematicalOps;

use crate::error::ContractError;

use super::OsmosisPool;

#[cw_serde]
pub struct WeightedPool(gamm::v1beta1::Pool);

impl From<gamm::v1beta1::Pool> for WeightedPool {
    fn from(v: gamm::v1beta1::Pool) -> Self {
        Self(v)
    }
}

struct PoolAsset {
    pub denom: String,
    pub amount: Uint128,
    pub weight: Uint128,
}

struct PoolAssetTuple(pub (String, Uint128, Uint128));

impl TryFrom<OsmosisPoolAsset> for PoolAsset {
    type Error = ContractError;

    fn try_from(v: OsmosisPoolAsset) -> Result<Self, Self::Error> {
        let OsmosisCoin { denom, amount } = v
            .token
            .ok_or_else(|| StdError::generic_err("token is none"))?;

        Ok(Self {
            denom,
            amount: Uint128::from_str(&amount)?,
            weight: Uint128::from_str(&v.weight)?,
        })
    }
}

impl From<PoolAsset> for OsmosisPoolAsset {
    fn from(v: PoolAsset) -> Self {
        Self {
            token: Some(OsmosisCoin {
                denom: v.denom,
                amount: v.amount.to_string(),
            }),
            weight: v.weight.to_string(),
        }
    }
}

impl From<PoolAssetTuple> for OsmosisPoolAsset {
    fn from(v: PoolAssetTuple) -> Self {
        Self {
            token: Some(OsmosisCoin {
                denom: v.0 .0,
                amount: v.0 .1.to_string(),
            }),
            weight: v.0 .2.to_string(),
        }
    }
}

impl WeightedPool {
    fn get_asset(&self, denom: &str) -> Result<PoolAsset, ContractError> {
        self.0
            .pool_assets
            .iter()
            .find(|v| {
                v.token
                    .as_ref()
                    .map(|v| v.denom == denom)
                    .unwrap_or_default()
            })
            .ok_or_else(|| StdError::generic_err(format!("asset {denom} not found")))?
            .clone()
            .try_into()
    }

    fn apply_new_pool_assets(
        &mut self,
        input_denom: &str,
        output_denom: &str,
        input_value: Uint128,
        output_value: Uint128,
    ) -> Result<(), ContractError> {
        let pool_assets = self
            .0
            .pool_assets
            .clone()
            .into_iter()
            .map(PoolAsset::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let before_input = self.get_asset(input_denom)?;
        let before_output = self.get_asset(output_denom)?;

        let after_input_amount = before_input.amount.checked_add(input_value)?;
        let after_output_amount = before_output.amount.checked_sub(output_value)?;

        let new_pool_assets = pool_assets
            .into_iter()
            .map(|v| match v.denom {
                d if d == input_denom => (d, after_input_amount, v.weight),
                d if d == output_denom => (d, after_output_amount, v.weight),
                d => (d, v.amount, v.weight),
            })
            .map(|v| OsmosisPoolAsset::from(PoolAssetTuple(v)))
            .collect::<Vec<_>>();

        self.0.pool_assets = new_pool_assets;

        Ok(())
    }

    fn calc_out_amount_given_in(
        &self,
        input_amount: &Coin,
        output_denom: &str,
        spread_factor: Decimal,
    ) -> Result<Uint128, ContractError> {
        let token_out = self.get_asset(output_denom)?;
        let token_in = self.get_asset(&input_amount.denom)?;

        let minus_spread_factor = Decimal::one().checked_sub(spread_factor)?;
        let token_sub_in = Decimal::checked_from_ratio(
            token_in.amount,
            token_in
                .amount
                .checked_add(token_in.amount * minus_spread_factor)?,
        )?;
        let token_weight_ratio = Decimal::checked_from_ratio(token_in.weight, token_out.weight)?;

        let rust_token_weight_ratio = RustDecimal::from_str(&token_weight_ratio.to_string())?;
        let rust_token_sub_in = RustDecimal::from_str(&token_sub_in.to_string())?;

        let calculed_by_rust_decimal =
            Decimal::from_str(&rust_token_sub_in.powd(rust_token_weight_ratio).to_string())
                .unwrap();

        Ok(token_out.amount * (Decimal::one() - calculed_by_rust_decimal))
    }

    fn calc_in_amount_given_out(
        &self,
        input_denom: &str,
        output_amount: &Coin,
        spread_factor: Decimal,
    ) -> Result<Uint128, ContractError> {
        let token_out = self.get_asset(&output_amount.denom)?;
        let token_in = self.get_asset(input_denom)?;

        let token_sub_out = token_out.amount;
        let token_weight_ratio = Decimal::checked_from_ratio(token_out.weight, token_in.weight)?;

        let rust_token_weight_ratio = RustDecimal::from_str(&token_weight_ratio.to_string())?;
        let rust_token_sub_out = RustDecimal::from_str(&token_sub_out.to_string())?;

        let calculed_by_rust_decimal =
            Decimal::from_str(&rust_token_sub_out.powd(rust_token_weight_ratio).to_string())?;

        let minus_spread_factor = Decimal::one() - spread_factor;

        let divided_token_out =
            (calculed_by_rust_decimal - Decimal::one()).div(minus_spread_factor);

        Ok(token_in.amount * divided_token_out)
    }
}

impl OsmosisPool for WeightedPool {
    fn get_id(&self) -> u64 {
        self.0.id
    }
    fn get_type(&self) -> &str {
        "weighted_pool"
    }
    fn get_spread_factor(&self) -> StdResult<Decimal> {
        Ok(self
            .0
            .pool_params
            .clone()
            .map(|v| Decimal::from_str(&v.swap_fee))
            .transpose()?
            .unwrap_or_default())
    }
    fn get_exit_fee(&self) -> StdResult<Decimal> {
        Ok(self
            .0
            .pool_params
            .clone()
            .map(|v| Decimal::from_str(&v.exit_fee))
            .transpose()?
            .unwrap_or_default())
    }

    fn swap_exact_amount_in(
        &mut self,
        input_amount: Coin,
        output_denom: String,
        _min_output_amount: Uint128,
        spread_factor: Decimal,
    ) -> Result<Uint128, ContractError> {
        let amount_out =
            self.calc_out_amount_given_in(&input_amount, &output_denom, spread_factor)?;

        self.apply_new_pool_assets(
            &input_amount.denom,
            &output_denom,
            input_amount.amount,
            amount_out,
        )?;

        Ok(amount_out)
    }

    fn swap_exact_amount_out(
        &mut self,
        input_denom: String,
        _max_input_amount: Uint128,
        output_amount: Coin,
        spread_factor: Decimal,
    ) -> Result<Uint128, ContractError> {
        let amount_in =
            self.calc_in_amount_given_out(&input_denom, &output_amount, spread_factor)?;

        self.apply_new_pool_assets(
            &input_denom,
            &output_amount.denom,
            amount_in,
            output_amount.amount,
        )?;

        Ok(amount_in)
    }
}
