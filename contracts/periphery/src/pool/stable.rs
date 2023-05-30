use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Decimal256, StdError, StdResult, Uint128};
use osmosis_std::types::osmosis::gamm::poolmodels::stableswap;

use crate::error::ContractError;

use super::OsmosisPool;

#[cw_serde]
pub struct StablePool(stableswap::v1beta1::Pool);

impl From<stableswap::v1beta1::Pool> for StablePool {
    fn from(v: stableswap::v1beta1::Pool) -> Self {
        Self(v)
    }
}

impl StablePool {
    fn cfmm_constant(x: Decimal256, y: Decimal256) -> Result<Decimal256, ContractError> {
        Ok(x.checked_mul(y)?
            .checked_mul(x.checked_pow(2)?.checked_add(y.checked_pow(2)?)?)?)
    }

    fn cfmm_constant_multi_no_v(
        x_reserve: Decimal256,
        y_reserve: Decimal256,
        w_sum_squares: Decimal256,
    ) -> Result<Decimal256, ContractError> {
        Ok(
            Self::cfmm_constant_multi_no_vy(x_reserve, y_reserve, w_sum_squares)?
                .checked_mul(y_reserve)?,
        )
    }

    fn cfmm_constant_multi_no_vy(
        x_reserve: Decimal256,
        y_reserve: Decimal256,
        w_sum_squares: Decimal256,
    ) -> Result<Decimal256, ContractError> {
        if !x_reserve.gt(&Decimal256::zero())
            || !y_reserve.gt(&Decimal256::zero())
            || w_sum_squares.lt(&Decimal256::zero())
        {
            return Err(ContractError::Std(StdError::generic_err(
                "reserves must be greater than zero",
            )));
        }

        let xx = x_reserve.checked_pow(2)?;
        let yy = y_reserve.checked_pow(2)?;

        Ok(x_reserve.checked_mul(xx.checked_add(yy)?.checked_add(w_sum_squares)?)?)
    }

    fn cfmm_constant_multi(
        x_reserve: Decimal256,
        y_reserve: Decimal256,
        u: Decimal256,
        v: Decimal256,
    ) -> Result<Decimal256, ContractError> {
        if !u.gt(&Decimal256::zero()) {
            return Err(ContractError::Std(StdError::generic_err(
                "reserves must be greater than zero",
            )));
        }

        Ok(Self::cfmm_constant_multi_no_v(x_reserve, y_reserve, v)?.checked_mul(u)?)
    }

    fn solve_cfmm(
        x_reserve: Decimal256,
        y_reserve: Decimal256,
        rem_reserves: Vec<Decimal256>,
        y_in: Decimal256,
    ) -> Result<Decimal256, ContractError> {
        let w_sum_square = rem_reserves.into_iter().try_fold(
            Decimal256::zero(),
            |w_sum_squares, asset_reserve| {
                Ok::<_, ContractError>(w_sum_squares.checked_add(asset_reserve.checked_pow(2)?)?)
            },
        )?;

        Self::solve_cfmm_binary_search_multi(x_reserve, y_reserve, w_sum_square, y_in)
    }

    fn solve_cfmm_binary_search_multi(
        x_reserve: Decimal256,
        y_reserve: Decimal256,
        w_sum_squares: Decimal256,
        y_in: Decimal256,
    ) -> Result<Decimal256, ContractError> {
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
            return Err(ContractError::Std(StdError::generic_err(
                "reserves must be greater than zero",
            )));
        } else if !y_in.lt(&y_reserve) {
            return Err(ContractError::Std(StdError::generic_err(
                "cannot input more than pool reserves",
            )));
        }

        let k = Self::cfmm_constant_multi_no_v(x_reserve, y_reserve, w_sum_squares)?;
        let kk = k.checked_pow(2)?;

        let y_new = y_in.checked_add(y_reserve)?;

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
            return Err(ContractError::Std(StdError::generic_err(
                "cannot output more than pool reserves",
            )));
        }

        Ok(x_out)
    }

    fn calc_out_amount_given_in(
        &self,
        token_in: Coin,
        token_out_denom: String,
        swap_fee: Decimal,
    ) -> Result<Decimal256, ContractError> {
        // TODO
        Ok(Default::default())
    }

    fn calc_in_amount_given_out(
        &self,
        token_out: Coin,
        token_in_denom: String,
        swap_fee: Decimal,
    ) -> Result<Decimal256, ContractError> {
        // TODO
        Ok(Default::default())
    }
}

impl OsmosisPool for StablePool {
    fn get_id(&self) -> u64 {
        self.0.id
    }

    fn get_type(&self) -> &str {
        "stable_pool"
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

    // fn apply_new_liquidity(&mut self) {
    //     self.0.pool_liquidity
    // }

    fn swap_exact_amount_in(
        &mut self,
        input_amount: cosmwasm_std::Coin,
        output_denom: String,
        min_output_amount: cosmwasm_std::Uint128,
        spread_factor: Decimal,
    ) -> Result<Uint128, ContractError> {
        todo!()
    }

    fn swap_exact_amount_out(
        &mut self,
        input_denom: String,
        max_input_amount: cosmwasm_std::Uint128,
        output_amount: cosmwasm_std::Coin,
        spread_factor: Decimal,
    ) -> Result<Uint128, ContractError> {
        todo!()
    }
}
