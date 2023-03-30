use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use ibcx_interface::types::SwapRoutes;

use crate::{
    error::{ContractError, RebalanceError, ValidationError},
    StdResult,
};

use super::Units;

#[cw_serde]
#[derive(Default)]
pub struct Rebalance {
    pub manager: Option<Addr>,
    pub deflation: Units,
    pub inflation: Units,
}

impl Rebalance {
    pub fn validate(&self, index_units: Units) -> Result<(), ContractError> {
        // check duplication
        if self.deflation.check_duplicate() {
            return Err(ValidationError::invalid_rebalance("deflation", "duplicate denom").into());
        }
        if self.inflation.check_duplicate() {
            return Err(ValidationError::invalid_rebalance("inflation", "duplicate denom").into());
        }

        // check index_units <-> deflation
        {
            let mut base_units = index_units;
            for deflation in self.deflation.iter() {
                let item = base_units.pop_key(&deflation.0);

                match item {
                    // check overflow
                    Some(item) => {
                        if item.1 < deflation.1 {
                            return Err(ValidationError::invalid_rebalance(
                                "deflation",
                                format!("overflow: {}", deflation.0),
                            )
                            .into());
                        }
                    }

                    // check missing denom
                    None => {
                        return Err(ValidationError::invalid_rebalance(
                            "deflation",
                            format!("missing denom: {}", deflation.0),
                        )
                        .into());
                    }
                }
            }
        }

        // check deflation <-> inflation
        {
            let conflict = self
                .deflation
                .clone()
                .into_iter()
                .chain(self.inflation.clone().into_iter())
                .collect::<Units>()
                .check_duplicate();
            if conflict {
                return Err(ValidationError::invalid_rebalance(
                    "deflation/inflation",
                    "denom conflict",
                )
                .into());
            }
        }

        Ok(())
    }
}

#[cw_serde]
pub struct TradeInfo {
    pub routes: SwapRoutes,
    pub cooldown: u64,
    pub max_trade_amount: Uint128,
    pub last_traded_at: Option<u64>,
}

impl TradeInfo {
    pub fn assert_cooldown(&self, now: u64) -> StdResult<()> {
        if self.last_traded_at.is_none() {
            return Ok(());
        }

        let t = self.last_traded_at.unwrap() + self.cooldown;

        if now < t {
            return Err(RebalanceError::OnTradeCooldown.into());
        }

        Ok(())
    }

    pub fn update_last_traded_at(self, now: u64) -> Self {
        Self {
            last_traded_at: Some(now),
            ..self
        }
    }
}

#[cfg(test)]
mod test {
    use crate::error::ValidationError;

    use super::Rebalance;

    #[test]
    fn test_rebalance_validate() {
        let cases = [
            (
                Rebalance {
                    deflation: vec![("uatom", "1.0"), ("uatom", "1.2")].into(),
                    ..Default::default()
                },
                vec![("uatom", "2.3")].into(),
                Err(ValidationError::invalid_rebalance("deflation", "duplicate denom").into()),
            ),
            (
                Rebalance {
                    inflation: vec![("uosmo", "1.0"), ("uosmo", "1.2")].into(),
                    ..Default::default()
                },
                vec![("uatom", "2.3")].into(),
                Err(ValidationError::invalid_rebalance("inflation", "duplicate denom").into()),
            ),
            (
                Rebalance {
                    deflation: vec![("uatom", "2.3")].into(),
                    ..Default::default()
                },
                vec![("uatom", "1.0")].into(),
                Err(ValidationError::invalid_rebalance("deflation", "overflow: uatom").into()),
            ),
            (
                Rebalance {
                    deflation: vec![("uosmo", "2.3")].into(),
                    ..Default::default()
                },
                vec![("uatom", "1.0")].into(),
                Err(ValidationError::invalid_rebalance("deflation", "missing denom: uosmo").into()),
            ),
            (
                Rebalance {
                    deflation: vec![("uatom", "1.3")].into(),
                    inflation: vec![("uatom", "1.2")].into(),
                    ..Default::default()
                },
                vec![("uatom", "2.3")].into(),
                Err(
                    ValidationError::invalid_rebalance("deflation/inflation", "denom conflict")
                        .into(),
                ),
            ),
            (
                Rebalance {
                    deflation: vec![("uatom", "1.3")].into(),
                    inflation: vec![("uosmo", "1.2")].into(),
                    ..Default::default()
                },
                vec![("uatom", "2.3")].into(),
                Ok(()),
            ),
        ];

        for (rebalance, index_units, expected) in cases {
            assert_eq!(rebalance.validate(index_units), expected);
        }
    }
}
