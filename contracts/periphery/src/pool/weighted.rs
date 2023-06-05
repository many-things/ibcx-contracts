use std::ops::Div;
use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{from_binary, Binary, Decimal256, Deps, StdError};
use cosmwasm_std::{Coin, Decimal, StdResult, Uint256};
use rust_decimal::Decimal as RustDecimal;
use rust_decimal::MathematicalOps;

use crate::error::ContractError;

use super::OsmosisPool;

/*
{
   "pool":{
        "@type":"/osmosis.gamm.v1beta1.Pool",
        "address":"osmo1ad4r3uh5pdn5pgg5hnl6u5utfeqmpwstlvgvg2h2jdztrcnwkqgs3hs85z",
        "id":"4",
        "pool_params":{
            "swap_fee":"0.010000000000000000",
            "exit_fee":"0.000000000000000000",
            "smooth_weight_change_params":null
        },
        "future_pool_governor":"",
        "total_shares":{
            "denom":"gamm/pool/4",
            "amount":"100000000000000000000"
        },
        "pool_assets":[
            {
                "token":{
                    "denom":"factory/osmo1gxygw5gy8yhyuu05qa9fmgadyyane87prwp65g/uatom",
                    "amount":"2304880000000"
                },
                "weight":"1073741824000000"
            },
            {
                "token":{
                    "denom":"uosmo",
                    "amount":"40000000000000"
                },
                "weight":"1073741824000000"
            }
        ],
        "total_weight":"2147483648000000"
    }
}
*/
#[cw_serde]
pub struct WeightedPoolResponse {
    pub pool: WeightedPool,
}

impl TryFrom<Binary> for WeightedPoolResponse {
    type Error = ContractError;

    fn try_from(v: Binary) -> Result<Self, Self::Error> {
        Ok(from_binary(&v)?)
    }
}

#[cw_serde]
pub struct WeightedPool {
    #[serde(rename = "@type")]
    pub type_url: String,
    pub address: String,
    pub id: String,
    pub future_pool_governor: String,
    pub pool_params: WeightedPoolParams,
    pub pool_assets: Vec<WeightedPoolAsset>,
    pub total_shares: Coin,
    pub total_weight: Uint256,
}

#[cw_serde]
pub struct WeightedPoolParams {
    pub swap_fee: Decimal,
    pub exit_fee: Decimal,
    pub smooth_weight_change_params: Option<Uint256>,
}

#[cw_serde]
pub struct BigCoin {
    pub denom: String,
    pub amount: Uint256,
}

#[cw_serde]
pub struct WeightedPoolAsset {
    pub token: BigCoin,
    pub weight: Uint256,
}

struct PoolAssetTuple(pub (String, Uint256, Uint256));

impl From<PoolAssetTuple> for WeightedPoolAsset {
    fn from(v: PoolAssetTuple) -> Self {
        Self {
            token: BigCoin {
                denom: v.0 .0,
                amount: v.0 .1,
            },
            weight: v.0 .2,
        }
    }
}

impl WeightedPool {
    fn get_asset(&self, denom: &str) -> Result<WeightedPoolAsset, ContractError> {
        Ok(self
            .pool_assets
            .iter()
            .find(|v| v.token.denom == denom)
            .ok_or_else(|| StdError::generic_err(format!("asset {denom} not found")))?
            .clone())
    }

    fn apply_new_pool_assets(
        &mut self,
        input_denom: &str,
        output_denom: &str,
        input_value: Uint256,
        output_value: Uint256,
    ) -> Result<(), ContractError> {
        let pool_assets = self.pool_assets.clone();

        let before_input = self.get_asset(input_denom)?;
        let before_output = self.get_asset(output_denom)?;

        let after_input_amount = before_input.token.amount.checked_add(input_value)?;
        let after_output_amount = before_output.token.amount.checked_sub(output_value)?;

        let new_pool_assets = pool_assets
            .into_iter()
            .map(|v| match v.token.denom {
                d if d == input_denom => (d, after_input_amount, v.weight),
                d if d == output_denom => (d, after_output_amount, v.weight),
                d => (d, v.token.amount, v.weight),
            })
            .map(|v| PoolAssetTuple(v).into())
            .collect::<Vec<_>>();

        self.pool_assets = new_pool_assets;

        Ok(())
    }

    fn calc_out_amount_given_in(
        &self,
        input_amount: &Coin,
        output_denom: &str,
        spread_factor: Decimal,
    ) -> Result<Uint256, ContractError> {
        let WeightedPoolAsset {
            token: token_out,
            weight: token_out_weight,
        } = self.get_asset(output_denom)?;
        let WeightedPoolAsset {
            token: token_in,
            weight: token_in_weight,
        } = self.get_asset(&input_amount.denom)?;

        let minus_spread_factor = Decimal256::one().checked_sub(spread_factor.into())?;
        let token_sub_in = Decimal256::checked_from_ratio(
            token_in.amount,
            token_in
                .amount
                .checked_add(input_amount.amount * minus_spread_factor)?,
        )?;

        let token_weight_ratio = Decimal256::checked_from_ratio(token_in_weight, token_out_weight)?;

        let rust_token_weight_ratio = RustDecimal::from_str(&token_weight_ratio.to_string())?;
        let rust_token_sub_in = RustDecimal::from_str(&token_sub_in.to_string())?;

        let calculed_by_rust_decimal =
            Decimal256::from_str(&rust_token_sub_in.powd(rust_token_weight_ratio).to_string())
                .unwrap();

        Ok(token_out.amount * (Decimal256::one() - calculed_by_rust_decimal))
    }

    fn calc_in_amount_given_out(
        &self,
        input_denom: &str,
        output_amount: &Coin,
        spread_factor: Decimal,
    ) -> Result<Uint256, ContractError> {
        let WeightedPoolAsset {
            token: token_out,
            weight: token_out_weight,
        } = self.get_asset(&output_amount.denom)?;
        let WeightedPoolAsset {
            token: token_in,
            weight: token_in_weight,
        } = self.get_asset(input_denom)?;

        let token_sub_out: Uint256 = token_out.amount.checked_sub(output_amount.amount.into())?;
        let token_weight_ratio = Decimal256::checked_from_ratio(token_out_weight, token_in_weight)?;

        let rust_token_weight_ratio = RustDecimal::from_str(&token_weight_ratio.to_string())?;
        let rust_token_sub_out = RustDecimal::from_str(&token_sub_out.to_string())?;

        let calculated_by_rust_decimal =
            Decimal256::from_str(&rust_token_sub_out.powd(rust_token_weight_ratio).to_string())?;
        let minus_spread_factor = Decimal256::one().checked_sub(spread_factor.into())?;
        let divided_token_out = Decimal256::from_str(&token_out.amount.to_string())?
            .div(calculated_by_rust_decimal)
            - Decimal256::one();
        let divded_minus_spread_factor = divided_token_out.div(minus_spread_factor);

        Ok(token_in.amount * divded_minus_spread_factor)
    }
}

impl OsmosisPool for WeightedPool {
    fn get_id(&self) -> u64 {
        self.id.parse::<u64>().unwrap()
    }
    fn get_type(&self) -> &str {
        "weighted_pool"
    }
    fn get_spread_factor(&self) -> StdResult<Decimal> {
        Ok(self.pool_params.swap_fee)
    }
    fn get_exit_fee(&self) -> StdResult<Decimal> {
        Ok(self.pool_params.exit_fee)
    }

    fn clone_box(&self) -> Box<dyn OsmosisPool> {
        Box::new(self.clone())
    }

    fn swap_exact_amount_in(
        &mut self,
        _deps: &Deps,
        input_amount: Coin,
        output_denom: String,
        _min_output_amount: Uint256,
        spread_factor: Decimal,
    ) -> Result<Uint256, ContractError> {
        // deps.api.debug(&format!(
        //     "{}.swap_exact_amount_in => input: {}, output: {}",
        //     self.get_type(),
        //     input_amount,
        //     output_denom
        // ));

        let amount_out =
            self.calc_out_amount_given_in(&input_amount, &output_denom, spread_factor)?;

        self.apply_new_pool_assets(
            &input_amount.denom,
            &output_denom,
            input_amount.amount.into(),
            amount_out,
        )?;

        Ok(amount_out)
    }

    fn swap_exact_amount_out(
        &mut self,
        _deps: &Deps,
        input_denom: String,
        _max_input_amount: Uint256,
        output_amount: Coin,
        spread_factor: Decimal,
    ) -> Result<Uint256, ContractError> {
        // deps.api.debug(&format!(
        //     "[{}] {}.swap_exact_amount_out => input: {}, output: {}",
        //     self.get_id(),
        //     self.get_type(),
        //     input_denom,
        //     output_amount,
        // ));

        let amount_in =
            self.calc_in_amount_given_out(&input_denom, &output_amount, spread_factor)?;

        self.apply_new_pool_assets(
            &input_denom,
            &output_amount.denom,
            amount_in,
            output_amount.amount.into(),
        )?;

        Ok(amount_in)
    }
}
