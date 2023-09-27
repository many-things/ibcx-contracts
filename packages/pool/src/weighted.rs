use std::ops::Div;
use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{from_binary, Binary, Decimal256, Deps, StdError};
use cosmwasm_std::{Coin, Decimal, StdResult, Uint256};

use crate::PoolError;

use super::OsmosisPool;

#[cw_serde]
pub struct WeightedPoolResponse {
    pub pool: WeightedPool,
}

impl TryFrom<Binary> for WeightedPoolResponse {
    type Error = StdError;

    fn try_from(v: Binary) -> Result<Self, Self::Error> {
        from_binary(&v)
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
    pub smooth_weight_change_params: Option<WeightedPoolSmoothWeightChangeParams>,
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

#[cw_serde]
pub struct WeightedPoolSmoothWeightChangeParams {
    pub initial_pool_weights: Vec<WeightedPoolAsset>,
    pub target_pool_weights: Vec<WeightedPoolAsset>,
    pub start_time: String,
    pub duration: String,
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
    fn get_asset(&self, denom: &str) -> Result<WeightedPoolAsset, PoolError> {
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
    ) -> Result<(), PoolError> {
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
    ) -> Result<Uint256, PoolError> {
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
                .checked_add(Uint256::from(input_amount.amount) * minus_spread_factor)?,
        )?;

        let token_weight_ratio = Decimal256::checked_from_ratio(token_in_weight, token_out_weight)?;

        Ok(token_out.amount
            * (Decimal256::one() - ibcx_math::pow(token_sub_in, token_weight_ratio)?))
    }

    fn calc_in_amount_given_out(
        &self,
        input_denom: &str,
        output_amount: &Coin,
        spread_factor: Decimal,
    ) -> Result<Uint256, PoolError> {
        let WeightedPoolAsset {
            token: token_out,
            weight: token_out_weight,
        } = self.get_asset(&output_amount.denom)?;
        let WeightedPoolAsset {
            token: token_in,
            weight: token_in_weight,
        } = self.get_asset(input_denom)?;

        let token_sub_out = Decimal256::checked_from_ratio(
            token_out.amount.checked_sub(output_amount.amount.into())?,
            1u64,
        )?;
        let token_weight_ratio = Decimal256::checked_from_ratio(token_out_weight, token_in_weight)?;

        let calculated_by_rust_decimal = ibcx_math::pow(token_sub_out, token_weight_ratio)?;
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
    ) -> Result<Uint256, PoolError> {
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
    ) -> Result<Uint256, PoolError> {
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

#[cfg(test)]
mod test {
    use super::*;

    use crate::test::{load_pools, AllPoolsPool};

    use std::{collections::BTreeMap, str::FromStr};

    use anyhow::anyhow;
    use cosmwasm_std::{coin, testing::mock_dependencies, Coin, Deps, Uint256};

    fn calc_out(
        deps: Deps,
        pools: &mut BTreeMap<u64, AllPoolsPool>,
        pool_id: u64,
        input: Coin,
        output: &str,
    ) -> anyhow::Result<Uint256> {
        if let AllPoolsPool::Weighted(pool) = pools.get_mut(&pool_id).unwrap() {
            let amount_out = pool.swap_exact_amount_in(
                &deps,
                input,
                output.to_string(),
                Uint256::from_str("100")?,
                pool.get_spread_factor()?,
            )?;

            Ok(amount_out)
        } else {
            Err(anyhow!("pool type is not weighted"))
        }
    }

    #[test]
    fn test_simulation() -> anyhow::Result<()> {
        let deps = mock_dependencies();
        let mut pools = load_pools("./test/data/all-pools-after.json".into())?;

        let cases = [
            (
                1,
                coin(100_000_000_000, "uosmo"),
                "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2",
            ),
            (
                722,
                coin(100_000_000_000, "uosmo"),
                "ibc/6AE98883D4D5D5FF9E50D7130F1305DA2FFA0C652D1DD9C123657C6B4EB2DF8A",
            ),
            (
                584,
                coin(100_000_000_000, "uosmo"),
                "ibc/0954E1C28EB7AF5B72D24F3BC2B47BBB2FDF91BDDFD57B74B99E133AED40972A",
            ),
            (2, coin(100_000_000_000, "uosmo"), "uion"),
        ];

        for (pool_id, input, output) in cases {
            println!("Trying: {} -> {}", input, output);
            let res = calc_out(deps.as_ref(), &mut pools, pool_id, input.clone(), output)?;
            println!("=> {}{}\n", res, output);
        }

        Ok(())
    }
}
