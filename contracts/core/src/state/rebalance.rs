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
        // check empty
        if self.deflation.len() == 0 {
            return Err(ValidationError::invalid_rebalance("deflation", "empty").into());
        }
        if self.inflation.len() == 0 {
            return Err(ValidationError::invalid_rebalance("inflation", "empty").into());
        }

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
                .chain(self.inflation.clone())
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

impl Default for TradeInfo {
    fn default() -> Self {
        Self {
            routes: SwapRoutes(vec![]),
            cooldown: Default::default(),
            max_trade_amount: Default::default(),
            last_traded_at: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        error::{RebalanceError, ValidationError},
        state::Units,
    };

    use super::{Rebalance, TradeInfo};

    #[test]
    fn test_rebalance_validate() {
        let cases = [
            (
                Rebalance {
                    inflation: vec![("ukrw", "1.0")].into(),
                    ..Default::default()
                },
                Units::default(),
                Err(ValidationError::invalid_rebalance("deflation", "empty").into()),
            ),
            (
                Rebalance {
                    deflation: vec![("ukrw", "1.0")].into(),
                    ..Default::default()
                },
                Units::default(),
                Err(ValidationError::invalid_rebalance("inflation", "empty").into()),
            ),
            (
                Rebalance {
                    deflation: vec![("uatom", "1.0"), ("uatom", "1.2")].into(),
                    inflation: vec![("ukrw", "1.0")].into(),
                    ..Default::default()
                },
                vec![("uatom", "2.3")].into(),
                Err(ValidationError::invalid_rebalance("deflation", "duplicate denom").into()),
            ),
            (
                Rebalance {
                    deflation: vec![("ukrw", "1.0")].into(),
                    inflation: vec![("uosmo", "1.0"), ("uosmo", "1.2")].into(),
                    ..Default::default()
                },
                vec![("uatom", "2.3")].into(),
                Err(ValidationError::invalid_rebalance("inflation", "duplicate denom").into()),
            ),
            (
                Rebalance {
                    deflation: vec![("uatom", "2.3")].into(),
                    inflation: vec![("ukrw", "1.0")].into(),
                    ..Default::default()
                },
                vec![("uatom", "1.0")].into(),
                Err(ValidationError::invalid_rebalance("deflation", "overflow: uatom").into()),
            ),
            (
                Rebalance {
                    deflation: vec![("uosmo", "2.3")].into(),
                    inflation: vec![("ukrw", "1.0")].into(),
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

    #[test]
    fn test_trade_info_assert_cooldown() {
        let cases = [
            (60, None, 40, Ok(())),
            (60, Some(0), 40, Err(RebalanceError::OnTradeCooldown.into())),
            (60, Some(0), 70, Ok(())),
        ];

        for (cooldown, last_traded_at, now, expected) in cases {
            let trade_info = TradeInfo {
                cooldown,
                last_traded_at,
                ..Default::default()
            };

            assert_eq!(trade_info.assert_cooldown(now), expected);
        }
    }

    #[test]
    fn test_trade_info_update_last_traded_at() {
        // chaining
        assert_eq!(
            TradeInfo::default()
                .update_last_traded_at(12345)
                .last_traded_at,
            Some(12345)
        );
    }
}
