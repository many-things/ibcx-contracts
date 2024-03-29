use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, StdResult, Uint256};
use cosmwasm_std::{Decimal256, Deps, StdError};
use ibcx_math::MathError;

use crate::PoolError;

use super::OsmosisPool;

fn to_dec(v: Uint256) -> Result<Decimal256, PoolError> {
    Ok(Decimal256::from_atomics(v, 0)?)
}

fn solve_constant_function_invariants(
    token_balance_fixed_before: Decimal256,
    token_balance_fixed_after: Decimal256,
    token_weight_fixed: Decimal256,
    token_balance_unknown_before: Decimal256,
    token_weight_unknown: Decimal256,
) -> Result<(Decimal256, bool), MathError> {
    let weight_ratio = token_weight_fixed / token_weight_unknown;

    let y = token_balance_fixed_before / token_balance_fixed_after;

    let y_to_weight_ratio = ibcx_math::pow(y, weight_ratio)?;

    let (paranthetical, neg) = ibcx_math::abs_diff_with_sign(Decimal256::one(), y_to_weight_ratio);

    let amount_y = token_balance_unknown_before * paranthetical;

    Ok((amount_y, neg))
}

#[cw_serde]
pub struct Pool {
    #[serde(rename = "@type")]
    pub type_url: String,
    pub address: String,
    pub id: String,
    pub future_pool_governor: String,
    pub pool_params: PoolParams,
    pub pool_assets: Vec<PoolAsset>,
    pub total_shares: Coin,
    pub total_weight: Uint256,
}

#[cw_serde]
pub struct PoolParams {
    pub swap_fee: Decimal,
    pub exit_fee: Decimal,
    pub smooth_weight_change_params: Option<PoolSmoothWeightChangeParams>,
}

#[cw_serde]
pub struct BigCoin {
    pub denom: String,
    pub amount: Uint256,
}

#[cw_serde]
pub struct PoolAsset {
    pub token: BigCoin,
    pub weight: Uint256,
}

#[cw_serde]
pub struct PoolSmoothWeightChangeParams {
    pub initial_pool_weights: Vec<PoolAsset>,
    pub target_pool_weights: Vec<PoolAsset>,
    pub start_time: String,
    pub duration: String,
}

struct PoolAssetTuple(pub (String, Uint256, Uint256));

impl From<PoolAssetTuple> for PoolAsset {
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

impl Pool {
    fn get_asset(&self, denom: &str) -> Result<PoolAsset, PoolError> {
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
        let before_input = self.get_asset(input_denom)?;
        let before_output = self.get_asset(output_denom)?;

        let after_input_amount = before_input.token.amount.checked_add(input_value)?;
        let after_output_amount = before_output.token.amount.checked_sub(output_value)?;

        for (i, asset) in self.pool_assets.clone().into_iter().enumerate() {
            self.pool_assets[i] = PoolAssetTuple(match asset.token.denom {
                d if d == input_denom => (d, after_input_amount, asset.weight),
                d if d == output_denom => (d, after_output_amount, asset.weight),
                d => (d, asset.token.amount, asset.weight),
            })
            .into();
        }

        Ok(())
    }

    /// tokenBalanceOut * [1 - { tokenBalanceIn / (tokenBalanceIn + (1 - spreadFactor) * tokenAmountIn)} ^ (tokenWeightIn / tokenWeightOut)]
    fn calc_out_amount_given_in(
        &self,
        input_amount: &Coin,
        output_denom: &str,
        spread_factor: Decimal,
    ) -> Result<Uint256, PoolError> {
        // parse pool assets
        let PoolAsset {
            token: token_out,
            weight: token_out_weight,
        } = self.get_asset(output_denom)?;
        let PoolAsset {
            token: token_in,
            weight: token_in_weight,
        } = self.get_asset(&input_amount.denom)?;

        let token_amount_in_after_fee = Uint256::from_uint128(input_amount.amount)
            * Decimal256::one().checked_sub(spread_factor.into())?;

        let pool_token_in_balance = token_in.amount;
        let pool_post_swap_in_balance = pool_token_in_balance + token_amount_in_after_fee;

        let (token_amount_out, neg) = solve_constant_function_invariants(
            to_dec(pool_token_in_balance)?,
            to_dec(pool_post_swap_in_balance)?,
            to_dec(token_in_weight)?,
            to_dec(token_out.amount)?,
            to_dec(token_out_weight)?,
        )?;

        if !neg {
            return Err(PoolError::invalid_math_approx(
                "token amount must be negative",
            ));
        }

        Ok(token_amount_out.to_uint_floor())
    }

    /// tokenBalanceIn * [{tokenBalanceOut / (tokenBalanceOut - tokenAmountOut)} ^ (tokenWeightOut / tokenWeightIn) -1] / tokenAmountIn
    fn calc_in_amount_given_out(
        &self,
        input_denom: &str,
        output_amount: &Coin,
        spread_factor: Decimal,
    ) -> Result<Uint256, PoolError> {
        // parse pool assets
        let PoolAsset {
            token: token_out,
            weight: token_out_weight,
        } = self.get_asset(&output_amount.denom)?;
        let PoolAsset {
            token: token_in,
            weight: token_in_weight,
        } = self.get_asset(input_denom)?;

        let pool_token_out_balance = token_out.amount;
        let pool_post_swap_out_balance =
            pool_token_out_balance - Uint256::from_uint128(output_amount.amount);

        let (token_amount_in, neg) = solve_constant_function_invariants(
            to_dec(pool_token_out_balance)?,
            to_dec(pool_post_swap_out_balance)?,
            to_dec(token_out_weight)?,
            to_dec(token_in.amount)?,
            to_dec(token_in_weight)?,
        )?;

        let token_amount_in_before_fee =
            token_amount_in / (Decimal256::one().checked_sub(spread_factor.into())?);

        let token_in_amount = token_amount_in_before_fee.to_uint_ceil();

        if neg {
            return Err(PoolError::invalid_math_approx(
                "token amount must be positive",
            ));
        }

        Ok(token_in_amount)
    }
}

impl OsmosisPool for Pool {
    fn get_id(&self) -> u64 {
        self.id.parse::<u64>().unwrap()
    }

    fn get_type(&self) -> &str {
        "weighted_pool"
    }

    fn get_spread_factor(&self) -> StdResult<Decimal> {
        Ok(self.pool_params.swap_fee)
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
    use crate::test::pool::{load_pools, load_pools_from_file};
    use crate::test::testdata;
    use crate::OsmosisPool;
    use crate::Pool;

    use std::{collections::BTreeMap, str::FromStr};

    use anyhow::anyhow;
    use cosmwasm_std::{coin, testing::mock_dependencies, Coin, Deps, Uint256};
    use ibcx_test_utils::App;
    use osmosis_std::types::osmosis::poolmanager::v1beta1::{
        EstimateSinglePoolSwapExactAmountInRequest, EstimateSinglePoolSwapExactAmountOutRequest,
    };
    use osmosis_test_tube::{Module, PoolManager};
    use rstest::rstest;

    #[derive(Clone, Debug)]
    struct SimulateOutGivnInCase<'a> {
        pub pool_id: u64,
        pub amount_in: &'a Coin,
        pub amount_out: &'a str,
    }

    fn calc_out(
        deps: Deps,
        pools: &mut BTreeMap<u64, Pool>,
        case: SimulateOutGivnInCase,
    ) -> anyhow::Result<Uint256> {
        if let Pool::Weighted(pool) = pools.get_mut(&case.pool_id).unwrap() {
            let amount_out = pool.swap_exact_amount_in(
                &deps,
                case.amount_in.clone(),
                case.amount_out.to_string(),
                Uint256::from_str("100")?,
                pool.get_spread_factor()?,
            )?;

            Ok(amount_out)
        } else {
            Err(anyhow!("pool type is not weighted"))
        }
    }

    #[derive(Clone, Debug)]
    struct SimulateInGivenOutCase<'a> {
        pub pool_id: u64,
        pub amount_in: &'a str,
        pub amount_out: &'a Coin,
    }

    fn calc_in(
        deps: Deps,
        pools: &mut BTreeMap<u64, Pool>,
        case: SimulateInGivenOutCase,
    ) -> anyhow::Result<Uint256> {
        if let Pool::Weighted(pool) = pools.get_mut(&case.pool_id).unwrap() {
            let amount_in = pool.swap_exact_amount_out(
                &deps,
                case.amount_in.to_string(),
                Uint256::from_str("100")?,
                case.amount_out.clone(),
                pool.get_spread_factor()?,
            )?;

            Ok(amount_in)
        } else {
            Err(anyhow!("pool type is not weighted"))
        }
    }

    #[rstest]
    #[case(
        1,
        coin(100_000_000, "uosmo"),
        "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2"
    )]
    #[case(2, coin(100_000_000, "uosmo"), "uion")]
    #[case(
        3,
        coin(100_000_000, "uosmo"),
        "ibc/1480B8FD20AD5FCAE81EA87584D269547DD4D436843C1D20F15E00EB64743EF4"
    )]
    #[case(
        584,
        coin(100_000_000, "uosmo"),
        "ibc/0954E1C28EB7AF5B72D24F3BC2B47BBB2FDF91BDDFD57B74B99E133AED40972A"
    )]
    #[case(
        722,
        coin(100_000_000, "uosmo"),
        "ibc/6AE98883D4D5D5FF9E50D7130F1305DA2FFA0C652D1DD9C123657C6B4EB2DF8A"
    )]
    fn test_sim_in(#[case] pool_id: u64, #[case] amount_in_osmo: Coin, #[case] amount_out: &str) {
        let case = SimulateOutGivnInCase {
            pool_id,
            amount_in: &amount_in_osmo,
            amount_out,
        };

        // ready local pool state
        let deps = mock_dependencies();
        let mut pools = load_pools(testdata("all-pools-after.json")).unwrap();

        // ready test tube
        let app = App::default();
        let pm = PoolManager::new(app.inner());
        load_pools_from_file(app.inner(), testdata("all-pools-after.json")).unwrap();

        let expected = calc_out(deps.as_ref(), &mut pools, case.clone()).unwrap();

        let actual_res = pm
            .query_single_pool_swap_exact_amount_in(&EstimateSinglePoolSwapExactAmountInRequest {
                pool_id: case.pool_id,
                token_in: case.amount_in.to_string(),
                token_out_denom: case.amount_out.to_string(),
            })
            .unwrap();
        let actual = Uint256::from_str(&actual_res.token_out_amount).unwrap();

        println!(
            "{} -> {}. expected: {}, actual: {}, diff: {}",
            case.amount_in,
            case.amount_out,
            expected,
            actual,
            expected.abs_diff(actual),
        )

        // assert_eq!(
        //     expected,
        //     actual,
        //     "{} -> {}. expected: {}, actual: {}, diff: {}",
        //     case.amount_in,
        //     case.amount_out,
        //     expected,
        //     actual,
        //     expected.abs_diff(actual),
        // );
    }

    #[rstest]
    #[case(
        1,
        "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2",
        coin(100_000_000, "uosmo")
    )]
    #[case(2, "uion", coin(100_000_000, "uosmo"))]
    #[case(
        3,
        "ibc/1480B8FD20AD5FCAE81EA87584D269547DD4D436843C1D20F15E00EB64743EF4",
        coin(100_000_000, "uosmo")
    )]
    #[case(
        584,
        "ibc/0954E1C28EB7AF5B72D24F3BC2B47BBB2FDF91BDDFD57B74B99E133AED40972A",
        coin(100_000_000, "uosmo")
    )]
    #[case(
        722,
        "ibc/6AE98883D4D5D5FF9E50D7130F1305DA2FFA0C652D1DD9C123657C6B4EB2DF8A",
        coin(100_000_000, "uosmo")
    )]
    fn test_sim_out(#[case] pool_id: u64, #[case] amount_in: &str, #[case] amount_out_osmo: Coin) {
        let case = SimulateInGivenOutCase {
            pool_id,
            amount_in,
            amount_out: &amount_out_osmo,
        };

        // ready local pool state
        let deps = mock_dependencies();
        let mut pools = load_pools(testdata("all-pools-after.json")).unwrap();

        // ready test tube
        let app = App::default();
        let pm = PoolManager::new(app.inner());
        load_pools_from_file(app.inner(), testdata("all-pools-after.json")).unwrap();

        let expected = calc_in(deps.as_ref(), &mut pools, case.clone()).unwrap();

        let actual_res = pm
            .query_single_pool_swap_exact_amount_out(&EstimateSinglePoolSwapExactAmountOutRequest {
                pool_id: case.pool_id,
                token_out: case.amount_out.to_string(),
                token_in_denom: case.amount_in.to_string(),
            })
            .unwrap();
        let actual = Uint256::from_str(&actual_res.token_in_amount).unwrap();

        println!(
            "{} -> {}. expected: {}, actual: {}, diff: {}",
            case.amount_in,
            case.amount_out,
            expected,
            actual,
            expected.abs_diff(actual),
        );

        // assert_eq!(
        //     expected,
        //     actual,
        //     "{} -> {}. expected: {}, actual: {}, diff: {}",
        //     case.amount_in,
        //     case.amount_out,
        //     expected,
        //     actual,
        //     expected.abs_diff(actual),
        // );
    }
}
