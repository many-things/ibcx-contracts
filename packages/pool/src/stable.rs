use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Decimal256, Deps, StdError, StdResult, Uint256};
use ibcx_interface::types::{SwapRoute, SwapRoutes};

use crate::PoolError;

use super::OsmosisPool;

#[cw_serde]
pub struct Pool {
    #[serde(rename = "@type")]
    pub type_url: String,
    pub address: String,
    pub id: String,

    pub pool_params: PoolParams,
    pub pool_liquidity: Vec<Coin>,

    pub scaling_factors: Vec<u64>,
    pub scaling_factor_controller: String,
    pub total_shares: Coin,
    pub future_pool_governor: String,
}

#[cw_serde]
pub struct PoolParams {
    pub swap_fee: Decimal,
    pub exit_fee: Decimal,
}

impl Pool {
    #[allow(dead_code)]
    fn cfmm_constant(x: Decimal256, y: Decimal256) -> Result<Decimal256, PoolError> {
        Ok(x.checked_mul(y)?
            .checked_mul(x.checked_pow(2)?.checked_add(y.checked_pow(2)?)?)?)
    }

    fn cfmm_constant_multi_no_v(
        x_reserve: Decimal256,
        y_reserve: Decimal256,
        w_sum_squares: Decimal256,
    ) -> Result<Decimal256, PoolError> {
        Ok(
            Self::cfmm_constant_multi_no_vy(x_reserve, y_reserve, w_sum_squares)?
                .checked_mul(y_reserve)?,
        )
    }

    fn cfmm_constant_multi_no_vy(
        x_reserve: Decimal256,
        y_reserve: Decimal256,
        w_sum_squares: Decimal256,
    ) -> Result<Decimal256, PoolError> {
        if !x_reserve.gt(&Decimal256::zero())
            || !y_reserve.gt(&Decimal256::zero())
            || w_sum_squares.lt(&Decimal256::zero())
        {
            return Err(PoolError::Std(StdError::generic_err(
                "reserves must be greater than zero",
            )));
        }

        let xx = x_reserve.checked_pow(2)?;
        let yy = y_reserve.checked_pow(2)?;

        Ok(x_reserve.checked_mul(xx.checked_add(yy)?.checked_add(w_sum_squares)?)?)
    }

    #[allow(dead_code)]
    fn cfmm_constant_multi(
        x_reserve: Decimal256,
        y_reserve: Decimal256,
        u: Decimal256,
        v: Decimal256,
    ) -> Result<Decimal256, PoolError> {
        if !u.gt(&Decimal256::zero()) {
            return Err(PoolError::Std(StdError::generic_err(
                "reserves must be greater than zero",
            )));
        }

        Ok(Self::cfmm_constant_multi_no_v(x_reserve, y_reserve, v)?.checked_mul(u)?)
    }

    fn solve_cfmm(
        x_reserve: Decimal256,
        y_reserve: Decimal256,
        rem_reserves: Vec<Decimal256>,
        y_in: (Decimal256, bool),
    ) -> Result<Decimal256, PoolError> {
        let w_sum_square = rem_reserves.into_iter().try_fold(
            Decimal256::zero(),
            |w_sum_squares, asset_reserve| {
                Ok::<_, PoolError>(w_sum_squares.checked_add(asset_reserve.checked_pow(2)?)?)
            },
        )?;

        Self::solve_cfmm_direct(x_reserve, y_reserve, w_sum_square, y_in)
    }

    fn solve_cfmm_direct(
        x_reserve: Decimal256,
        y_reserve: Decimal256,
        w_sum_squares: Decimal256,
        (y_in, is_y_in_neg): (Decimal256, bool),
    ) -> Result<Decimal256, PoolError> {
        let const_729 = Decimal256::from_str("729.0")?;
        let const_108 = Decimal256::from_str("108.0")?;
        let const_27 = Decimal256::from_str("27.0")?;
        let const_3 = Decimal256::from_str("3.0")?;
        let cube_root_two = Decimal256::from_str("2.0")?.sqrt().sqrt();

        if !x_reserve.gt(&Decimal256::zero())
            || !y_reserve.gt(&Decimal256::zero())
            || w_sum_squares.lt(&Decimal256::zero())
            || !y_in.gt(&Decimal256::zero())
        {
            return Err(PoolError::Std(StdError::generic_err(
                "reserves must be greater than zero",
            )));
        } else if !y_in.lt(&y_reserve) {
            return Err(PoolError::Std(StdError::generic_err(
                "cannot input more than pool reserves",
            )));
        }

        let k = Self::cfmm_constant_multi_no_v(x_reserve, y_reserve, w_sum_squares)?;
        let kk = k.checked_pow(2)?;

        let y_new = if is_y_in_neg {
            y_reserve.checked_sub(y_in)?
        } else {
            y_reserve.checked_add(y_in)?
        };

        let y2 = y_new.checked_pow(2)?;
        let y3 = y2.checked_mul(y_new)?;
        let y4 = y3.checked_mul(y_new)?;

        let wypy3 = (w_sum_squares.checked_mul(y_new)?).checked_add(y3)?;
        let wypy3_pow3 = wypy3.checked_pow(3)?;

        let sqrt_term = ((kk.checked_mul(y4)?.checked_mul(const_729)?)
            .checked_add(y3.checked_mul(const_108)?.checked_mul(wypy3_pow3)?)?)
        .sqrt(); // so lucky to have sqrt

        let cube_root_term = (sqrt_term.checked_add(k.checked_mul(y2)?.checked_mul(const_27)?)?)
            .sqrt()
            .sqrt(); // root 3

        let term1 =
            cube_root_term.checked_div(cube_root_two.checked_mul(const_3)?.checked_mul(y_new)?)?;

        let term2 = (cube_root_two.checked_mul(wypy3)?).checked_div(cube_root_term)?;

        let x_new = term1.checked_sub(term2)?;

        let x_out = x_reserve.checked_sub(x_new)?;

        if !x_out.lt(&x_reserve) {
            return Err(PoolError::Std(StdError::generic_err(
                "cannot output more than pool reserves",
            )));
        }

        Ok(x_out)
    }
}

#[allow(dead_code)]
impl Pool {
    fn get_scaling_factor(&self, idx: usize) -> u64 {
        self.scaling_factors[idx]
    }

    fn scaled_sorted_pool_reserves(
        &self,
        first: &str,
        second: &str,
    ) -> Result<Vec<Decimal256>, PoolError> {
        let pool_liquidity = self.pool_liquidity.clone();

        let mut cur_idx = 2;

        let mut indexed = pool_liquidity
            .into_iter()
            .enumerate()
            .map(|(i, v)| match &v.denom {
                d if d == first => (0, v, self.get_scaling_factor(i)),
                d if d == second => (1, v, self.get_scaling_factor(i)),
                _ => {
                    let ret = (cur_idx, v, self.get_scaling_factor(i));
                    cur_idx += 1;
                    ret
                }
            })
            .collect::<Vec<_>>();

        indexed.sort_by(|a, b| a.0.cmp(&b.0));

        Ok(indexed
            .into_iter()
            .map(|(_, liq, scale)| Decimal256::from_ratio(liq.amount, scale))
            .collect())
    }

    fn scale_coin(&self, Coin { denom, amount }: Coin) -> Result<Decimal256, PoolError> {
        let found = self.pool_liquidity.iter().position(|c| c.denom == denom);

        let scaling_factor = found
            .map(|i| self.get_scaling_factor(i))
            .ok_or_else(|| StdError::generic_err("scaling factor not found"))?;

        let scaled = Decimal256::checked_from_ratio(amount, scaling_factor)?;

        Ok(scaled)
    }

    fn descale_amount(&self, denom: &str, amount: Decimal256) -> Result<Decimal256, PoolError> {
        let found = self.pool_liquidity.iter().position(|c| c.denom == denom);

        let scaling_factor = found.map(|i| self.get_scaling_factor(i)).unwrap_or(1);
        let scaling_factor_dec = Decimal256::checked_from_ratio(scaling_factor, 1u64)?;

        let descaled = amount.checked_mul(scaling_factor_dec)?;

        Ok(descaled)
    }

    fn calc_out_amount_given_in(
        &self,
        token_in: Coin,
        token_out_denom: String,
        swap_fee: Decimal,
    ) -> Result<Decimal256, PoolError> {
        let reserves = self.scaled_sorted_pool_reserves(&token_in.denom, &token_out_denom)?;

        let (token_supplies, rem_reserves) = reserves.split_at(2);
        let token_in_supply = token_supplies[0];
        let token_out_supply = token_supplies[1];
        let token_in = self.scale_coin(token_in)?;

        let cfmm_in = token_in.checked_mul(Decimal::one().checked_sub(swap_fee)?.into())?;
        let cfmm_out = Self::solve_cfmm(
            token_out_supply,
            token_in_supply,
            rem_reserves.to_vec(),
            (cfmm_in, false),
        )?;

        let amount_out = self.descale_amount(&token_out_denom, cfmm_out)?;

        Ok(amount_out)
    }

    fn calc_in_amount_given_out(
        &self,
        token_out: Coin,
        token_in_denom: String,
        swap_fee: Decimal,
    ) -> Result<Decimal256, PoolError> {
        let reserves = self.scaled_sorted_pool_reserves(&token_in_denom, &token_out.denom)?;

        let (token_supplies, rem_reserves) = reserves.split_at(2);
        let token_in_supply = token_supplies[0];
        let token_out_supply = token_supplies[1];
        let token_out = self.scale_coin(token_out)?;

        let cfmm_out = Self::solve_cfmm(
            token_in_supply,
            token_out_supply,
            rem_reserves.to_vec(),
            (token_out, true),
        )?;
        let cfmm_in = cfmm_out.checked_div(Decimal::one().checked_sub(swap_fee)?.into())?;

        let amount_in = self.descale_amount(&token_in_denom, cfmm_in)?;

        Ok(amount_in)
    }
}

impl OsmosisPool for Pool {
    fn get_id(&self) -> u64 {
        self.id.parse::<u64>().unwrap()
    }

    fn get_type(&self) -> &str {
        "stable_pool"
    }

    fn get_spread_factor(&self) -> StdResult<Decimal> {
        Ok(self.pool_params.swap_fee)
    }

    fn clone_box(&self) -> Box<dyn OsmosisPool> {
        Box::new(self.clone())
    }

    fn swap_exact_amount_in(
        &mut self,
        deps: &Deps,
        input_amount: cosmwasm_std::Coin,
        output_denom: String,
        _min_output_amount: cosmwasm_std::Uint256,
        _spread_factor: Decimal,
    ) -> Result<Uint256, PoolError> {
        Ok(SwapRoutes(vec![SwapRoute {
            pool_id: self.get_id(),
            token_denom: output_denom,
        }])
        .sim_swap_exact_in(&deps.querier, &self.address, input_amount)?
        .into())

        // ============= TODO: use this
        //
        // let amount_out_dec =
        //     self.calc_out_amount_given_in(input_amount, output_denom, spread_factor)?;

        // let amount_out = amount_out_dec.to_uint_floor();

        // Ok(Uint256::from_str(&amount_out.to_string())?)
    }

    fn swap_exact_amount_out(
        &mut self,
        deps: &Deps,
        input_denom: String,
        _max_input_amount: cosmwasm_std::Uint256,
        output_amount: cosmwasm_std::Coin,
        _spread_factor: Decimal,
    ) -> Result<Uint256, PoolError> {
        Ok(SwapRoutes(vec![SwapRoute {
            pool_id: self.get_id(),
            token_denom: input_denom,
        }])
        .sim_swap_exact_out(&deps.querier, &self.address, output_amount)?
        .into())

        // ============= TODO: use this
        //
        // let amount_in_dec =
        //     self.calc_in_amount_given_out(output_amount, input_denom, spread_factor)?;

        // let amount_in = amount_in_dec.to_uint_floor();

        // Ok(Uint256::from_str(&amount_in.to_string())?)
    }
}
