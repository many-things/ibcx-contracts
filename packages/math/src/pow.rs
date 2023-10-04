use cosmwasm_std::{Decimal256, Uint256};

use crate::error::MathError;

const DEC_PRECISION: u128 = 1_000_000_000_000_000_000u128;
const POW_PRECISION: u128 = 10_000_000_000u128;

/// Calculates the power of a base to an exponent
/// This function contains Osmosis's `pow` implementation as closely as possible
pub fn pow(base: Decimal256, exp: Decimal256) -> Result<Decimal256, MathError> {
    if base.is_zero() {
        return Err(MathError::NegativeBase);
    }

    if base >= Decimal256::from_ratio(2u64, 1u64) {
        return Err(MathError::BaseTooLarge);
    }

    if exp == Decimal256::one() {
        return Ok(base);
    }

    let integer = exp.to_uint_floor();
    let fractional = {
        let x = exp.atomics();
        let y = Uint256::from(DEC_PRECISION);

        Decimal256::from_atomics(x - (y * (x / y)), base.decimal_places())
    }?;

    let integer_pow = pow_int(base, integer);
    if fractional.is_zero() {
        return Ok(integer_pow);
    }

    let fractional_pow = pow_approx(
        base,
        fractional,
        Decimal256::from_atomics(POW_PRECISION, base.decimal_places())?,
    )?;

    Ok(integer_pow * fractional_pow)
}

fn pow_int(mut base: Decimal256, power: Uint256) -> Decimal256 {
    if power <= Uint256::one() {
        return Decimal256::one();
    }

    let mut tmp = Decimal256::one();

    let mut i = power;

    loop {
        if i <= Uint256::one() {
            break;
        }

        if !i % Uint256::from(2u64) == Uint256::zero() {
            tmp *= base;
        }

        i /= Uint256::from(2u64);

        base *= base;
    }

    base * tmp
}

fn pow_approx(
    base: Decimal256,
    exp: Decimal256,
    prec: Decimal256,
) -> Result<Decimal256, MathError> {
    if exp.is_zero() {
        return Ok(Decimal256::zero());
    }

    let a = exp;
    let (x, x_neg) = abs_diff_with_sign(base, Decimal256::one());

    let mut term = Decimal256::one();
    let mut sum = Decimal256::one();
    let mut negative = false;

    let mut i = 1u64;

    loop {
        if term < prec {
            break;
        }

        let big_k = Decimal256::one() * Decimal256::from_ratio(i, 1u64);
        let (c, c_neg) = abs_diff_with_sign(a, big_k - Decimal256::one());

        term *= c * x;
        term /= big_k;

        if term.is_zero() {
            break;
        }

        if x_neg {
            negative = !negative;
        }

        if c_neg {
            negative = !negative;
        }

        if negative {
            sum -= term;
        } else {
            sum += term;
        }

        i += 1;
    }

    Ok(sum)
}

pub fn abs_diff_with_sign(x: Decimal256, y: Decimal256) -> (Decimal256, bool) {
    (x.abs_diff(y), x > y)
}

#[cfg(test)]
mod test {
    use super::*;

    use std::str::FromStr;

    use cosmwasm_std::Decimal256;

    #[test]
    fn test_pow() -> anyhow::Result<()> {
        struct Case {
            base: Decimal256,
            exp: Decimal256,
            res: Decimal256,
            err: Option<Decimal256>,
        }

        let cases = [
            Case {
                // medium base, small exp
                base: Decimal256::from_str("0.8")?,
                exp: Decimal256::from_str("0.32")?,
                res: Decimal256::from_str("0.93108385")?,
                err: Some(Decimal256::from_str("0.00000001")?),
            },
            Case {
                // zero exp
                base: Decimal256::from_str("0.8")?,
                exp: Decimal256::zero(),
                res: Decimal256::one(),
                err: None,
            },
            Case {
                // large base, small exp
                base: Decimal256::from_str("1.9999")?,
                exp: Decimal256::from_str("0.23")?,
                res: Decimal256::from_str("1.172821461")?,
                err: Some(Decimal256::from_str("0.00000001")?),
            },
            Case {
                // small base, large exp
                base: Decimal256::from_str("0.0000123")?,
                exp: Decimal256::from_str("123")?,
                res: Decimal256::zero(),
                err: None,
            },
            Case {
                // large base, large exp
                base: Decimal256::from_str("1.777")?,
                exp: Decimal256::from_str("20")?,
                res: Decimal256::from_str("98570.862372081602")?,
                err: Some(Decimal256::from_str("0.000000000001")?),
            },
            Case {
                // base equal one
                base: Decimal256::from_str("1")?,
                exp: Decimal256::from_str("123")?,
                res: Decimal256::one(),
                err: None,
            },
        ];

        for c in cases {
            let res = pow(c.base, c.exp)?;
            let diff = res.abs_diff(c.res);

            match c.err {
                Some(e) => assert!(diff <= e),
                None => assert!(diff.is_zero()),
            }
        }

        Ok(())
    }
}
