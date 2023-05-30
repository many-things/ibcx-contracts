use std::ops::Div;
use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, StdResult, Uint128};
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmosisCoin;
use osmosis_std::types::osmosis::gamm;
use osmosis_std::types::osmosis::gamm::v1beta1::PoolAsset;
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

impl WeightedPool {
    fn generate_new_pool_assets(
        &self,
        input_denom: String,
        output_denom: String,
        input_value: Uint128,
        output_value: Uint128,
    ) -> Vec<PoolAsset> {
        let mut new_pool_assets: Vec<PoolAsset> = Vec::new();
        let before_input_amount = self
            .0
            .pool_assets
            .iter()
            .find(|v| v.token.as_ref().unwrap().denom == input_denom.clone())
            .unwrap()
            .token
            .as_ref()
            .unwrap()
            .amount
            .parse::<Uint128>()
            .unwrap();

        let before_output_amount = self
            .0
            .pool_assets
            .iter()
            .find(|v| v.token.as_ref().unwrap().denom == output_denom.clone())
            .unwrap()
            .token
            .as_ref()
            .unwrap()
            .amount
            .parse::<Uint128>()
            .unwrap();

        let after_input_amount = (before_input_amount + input_value).to_string();
        let after_output_amount = (before_output_amount + output_value).to_string();

        let temp_assets =
            self.0
                .pool_assets
                .iter()
                .map(|v| match v.token.as_ref().unwrap().denom.as_str() {
                    input_denom => new_pool_assets.push(PoolAsset {
                        token: Some(OsmosisCoin {
                            denom: input_denom.to_string(),
                            amount: after_input_amount.to_owned(),
                        }),
                        weight: v.weight.to_owned(),
                    }),
                    output_denom => new_pool_assets.push(PoolAsset {
                        token: Some(OsmosisCoin {
                            denom: output_denom.to_string(),
                            amount: after_output_amount.to_owned(),
                        }),
                        weight: v.weight.to_owned(),
                    }),
                    _ => new_pool_assets.push(v.clone()),
                });

        new_pool_assets
    }

    fn calc_out_amount_given_in(&self, input_amount: Coin, output_denom: String) -> Uint128 {
        let pool_assets = self.0.pool_assets.clone();
        let pool_params = self.0.pool_params.clone();

        let token_balance_out = pool_assets
            .iter()
            .find(|v| v.token.as_ref().unwrap().denom == output_denom)
            .unwrap()
            .token
            .as_ref()
            .unwrap()
            .amount
            .parse::<Uint128>()
            .unwrap();
        let token_balance_in = pool_assets
            .iter()
            .find(|v| v.token.as_ref().unwrap().denom == input_amount.denom)
            .unwrap()
            .token
            .as_ref()
            .unwrap()
            .amount
            .parse::<Uint128>()
            .unwrap();
        let spread_factor = Decimal::from_str(&pool_params.unwrap().swap_fee).unwrap();
        let token_amount_in = input_amount.amount;

        let token_weight_in = pool_assets
            .iter()
            .find(|v| v.token.as_ref().unwrap().denom == input_amount.denom)
            .map(|s| s.weight.parse::<Uint128>().unwrap())
            .unwrap();
        let token_weight_out = pool_assets
            .iter()
            .find(|v| v.token.as_ref().unwrap().denom == output_denom)
            .map(|s| s.weight.parse::<Uint128>().unwrap())
            .unwrap();

        let minus_spread_factor = Decimal::one() - spread_factor;
        let token_sub_in = Decimal::checked_from_ratio(
            token_balance_in,
            token_balance_in + minus_spread_factor * token_amount_in,
        )
        .unwrap();
        let token_weight_ratio =
            Decimal::checked_from_ratio(token_weight_in, token_weight_out).unwrap();

        let rust_token_weight_ratio =
            RustDecimal::from_str(&token_weight_ratio.to_string()).unwrap();
        let rust_token_sub_in = RustDecimal::from_str(&token_sub_in.to_string()).unwrap();

        let calculed_by_rust_decimal =
            Decimal::from_str(&rust_token_sub_in.powd(rust_token_weight_ratio).to_string())
                .unwrap();

        token_balance_out * (Decimal::one() - calculed_by_rust_decimal)
    }

    fn calc_in_amount_given_out(&self, input_denom: String, output_amount: Coin) -> Uint128 {
        let pool_assets = self.0.pool_assets.clone();
        let pool_params = self.0.pool_params.clone();

        let token_balance_out = pool_assets
            .iter()
            .find(|v| v.token.as_ref().unwrap().denom == output_amount.denom)
            .unwrap()
            .token
            .as_ref()
            .unwrap()
            .amount
            .parse::<Uint128>()
            .unwrap();
        let token_balance_in = pool_assets
            .iter()
            .find(|v| v.token.as_ref().unwrap().denom == input_denom)
            .unwrap()
            .token
            .as_ref()
            .unwrap()
            .amount
            .parse::<Uint128>()
            .unwrap();
        let spread_factor = Decimal::from_str(&pool_params.unwrap().swap_fee).unwrap();
        let token_amount_out = output_amount.amount;

        let token_weight_in = pool_assets
            .iter()
            .find(|v| v.token.as_ref().unwrap().denom == input_denom)
            .map(|s| s.weight.parse::<Uint128>().unwrap())
            .unwrap();
        let token_weight_out = pool_assets
            .iter()
            .find(|v| v.token.as_ref().unwrap().denom == output_amount.denom)
            .map(|s| s.weight.parse::<Uint128>().unwrap())
            .unwrap();

        let token_sub_out = token_balance_out - token_amount_out;
        let token_weight_ratio =
            Decimal::checked_from_ratio(token_weight_out, token_weight_in).unwrap();

        let rust_token_weight_ratio =
            RustDecimal::from_str(&token_weight_ratio.to_string()).unwrap();
        let rust_token_sub_out = RustDecimal::from_str(&token_sub_out.to_string()).unwrap();

        let calculed_by_rust_decimal =
            Decimal::from_str(&rust_token_sub_out.powd(rust_token_weight_ratio).to_string())
                .unwrap();

        let minus_spread_factor = Decimal::one() - spread_factor;

        let divided_token_out =
            (calculed_by_rust_decimal - Decimal::one()).div(minus_spread_factor);

        token_balance_in * divided_token_out
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
        min_output_amount: Uint128,
        spread_factor: Decimal,
    ) -> Result<Uint128, ContractError> {
        let amount_out = self.calc_out_amount_given_in(input_amount.clone(), output_denom.clone());

        let new_pool_assets = self.generate_new_pool_assets(
            input_amount.denom,
            output_denom.clone(),
            Uint128::zero() - input_amount.amount,
            amount_out,
        );

        self.0.pool_assets = new_pool_assets;

        Ok(amount_out)
    }

    fn swap_exact_amount_out(
        &mut self,
        input_denom: String,
        max_input_amount: Uint128,
        output_amount: Coin,
        spread_factor: Decimal,
    ) -> Result<Uint128, ContractError> {
        let amount_in = self.calc_in_amount_given_out(input_denom.clone(), output_amount.clone());

        let new_pool_assets = self.generate_new_pool_assets(
            input_denom.clone(),
            output_amount.denom,
            amount_in,
            Uint128::zero() - output_amount.amount,
        );

        self.0.pool_assets = new_pool_assets;

        Ok(amount_in)
    }
}
