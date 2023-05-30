use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, StdResult, Uint128};
use osmosis_std::types::osmosis::gamm;

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
    fn calc_out_amount_given_in(&self, input_amount: Coin, output_denom: String) -> Uint128 {
        let pool = self.0.clone();
        let pool_assets = pool.pool_assets;

        let token_balance_out = pool_assets
            .iter()
            .find(|v| v.token.unwrap().denom == output_denom)
            .unwrap()
            .token
            .unwrap()
            .amount
            .parse::<Uint128>()
            .unwrap();
        let token_balance_in = pool_assets
            .iter()
            .find(|v| v.token.unwrap().denom == input_amount.denom)
            .unwrap()
            .token
            .unwrap()
            .amount
            .parse::<Uint128>()
            .unwrap();
        let spread_factor = Decimal::from_str(&pool.pool_params.unwrap().swap_fee).unwrap();
        let token_amount_in = input_amount.amount;

        let pool_assets = pool.pool_assets;

        let token_weight_in = pool_assets
            .iter()
            .find(|v| v.token.unwrap().denom == input_amount.denom)
            .map(|s| s.weight.parse::<Uint128>().unwrap())
            .unwrap();
        let token_weight_out = pool_assets
            .iter()
            .find(|v| v.token.unwrap().denom == output_denom)
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

        token_balance_out * (Decimal::one() - token_sub_in.pow(token_weight_ratio))
    }

    fn calc_in_amount_given_out(&self, input_denom: String, output_amount: Coin) -> Uint128 {
        let pool = self.0.clone();
        let pool_assets = pool.pool_assets;

        let token_balance_out = pool_assets
            .iter()
            .find(|v| v.token.unwrap().denom == output_amount.denom)
            .unwrap()
            .token
            .unwrap()
            .amount
            .parse::<Uint128>()
            .unwrap();
        let token_balance_in = pool_assets
            .iter()
            .find(|v| v.token.unwrap().denom == input_denom)
            .unwrap()
            .token
            .unwrap()
            .amount
            .parse::<Uint128>()
            .unwrap();
        let spread_factor = Decimal::from_str(&pool.pool_params.unwrap().swap_fee).unwrap();
        let token_amount_out = output_amount.amount;

        let token_weight_in = pool_assets
            .iter()
            .find(|v| v.token.unwrap().denom == input_denom)
            .map(|s| s.weight.parse::<Uint128>().unwrap())
            .unwrap();
        let token_weight_out = pool_assets
            .iter()
            .find(|v| v.token.unwrap().denom == output_amount.denom)
            .map(|s| s.weight.parse::<Uint128>().unwrap())
            .unwrap();

        let token_sub_out = token_balance_out - token_amount_out;
        let token_weight_ratio =
            Decimal::checked_from_ratio(token_weight_out, token_weight_in).unwrap();
        let token_out_with_weight = token_sub_out.pow(token_weight_ratio);

        let minus_spread_factor = Decimal::one() - spread_factor;

        let divided_token_out = (token_out_with_weight - Uint128::new(1)) / minus_spread_factor;

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
        let amount_out = self.calc_out_amount_given_in(input_amount, output_denom);

        let before_input_amount = self
            .0
            .pool_assets
            .iter()
            .find(|v| v.token.unwrap().denom == input_amount.denom)
            .unwrap()
            .token
            .unwrap()
            .amount
            .parse::<Uint128>()
            .unwrap()
            .clone();

        let before_output_amount = self
            .0
            .pool_assets
            .iter()
            .find(|v| v.token.unwrap().denom == output_denom)
            .unwrap()
            .token
            .unwrap()
            .amount
            .parse::<Uint128>()
            .unwrap();

        self.0
            .pool_assets
            .iter()
            .find(|v| v.token.unwrap().denom == input_amount.denom)
            .unwrap()
            .token
            .unwrap()
            .amount = (before_input_amount + input_amount.amount).to_string();

        self.0
            .pool_assets
            .iter()
            .find(|v| v.token.unwrap().denom == output_denom)
            .unwrap()
            .token
            .unwrap()
            .amount = (before_output_amount - amount_out).to_string();

        Ok(amount_out)
    }

    fn swap_exact_amount_out(
        &mut self,
        input_denom: String,
        max_input_amount: cosmwasm_std::Uint128,
        output_amount: cosmwasm_std::Coin,
        spread_factor: Decimal,
    ) -> Result<Uint128, ContractError> {
        let amount_in = self.calc_in_amount_given_out(input_denom, output_amount);

        let before_input_amount = self
            .0
            .pool_assets
            .iter()
            .find(|v| v.token.unwrap().denom == input_denom)
            .unwrap()
            .token
            .unwrap()
            .amount
            .parse::<Uint128>()
            .unwrap()
            .clone();

        let before_output_amount = self
            .0
            .pool_assets
            .iter()
            .find(|v| v.token.unwrap().denom == output_amount.denom)
            .unwrap()
            .token
            .unwrap()
            .amount
            .parse::<Uint128>()
            .unwrap();

        self.0
            .pool_assets
            .iter()
            .find(|v| v.token.unwrap().denom == input_denom)
            .unwrap()
            .token
            .unwrap()
            .amount = (before_input_amount + amount_in).to_string();

        self.0
            .pool_assets
            .iter()
            .find(|v| v.token.unwrap().denom == output_amount.denom)
            .unwrap()
            .token
            .unwrap()
            .amount = (before_output_amount - output_amount.amount).to_string();

        Ok(amount_in)
    }
}
