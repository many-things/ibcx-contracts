use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use ibcx_interface::types::SwapRoutes;

use crate::error::ContractError;

#[cw_serde]
pub struct Rebalance {
    pub manager: Addr,
    pub deflation: Vec<(String, Decimal)>,
    pub inflation: Vec<(String, Decimal)>,
    pub finalized: bool,
}

impl Rebalance {
    pub fn get_deflation(&self, denom: &str) -> Result<(String, Decimal), ContractError> {
        match self.deflation.iter().find(|v| v.0 == denom) {
            Some(v) => Ok(v.clone()),
            None => Err(ContractError::InvalidArgument(format!(
                "cannot find deflation asset: {denom}",
            ))),
        }
    }

    pub fn get_inflation(&self, denom: &str) -> Result<(String, Decimal), ContractError> {
        match self.inflation.iter().find(|v| v.0 == denom) {
            Some(v) => Ok(v.clone()),
            None => Err(ContractError::InvalidArgument(format!(
                "cannot find inflation asset: {denom}",
            ))),
        }
    }

    pub fn validate(&self, assets: Vec<(String, Decimal)>) -> Result<(), ContractError> {
        let prettify = |f: Vec<&(String, Decimal)>| {
            f.into_iter()
                .map(|(denom, unit)| -> String { format!("(\"{denom}\",\"{unit}\")") })
                .fold("[".to_string(), |acc, s| acc + &s + ",")
                .trim_end_matches(',')
                .to_string()
                + "]"
        };

        // check current asset & deflation
        let f = self
            .deflation
            .iter()
            .filter(|(denom_x, _)| !assets.iter().any(|(denom_y, _)| denom_x == denom_y))
            .collect::<Vec<_>>();
        if !f.is_empty() {
            return Err(ContractError::InvalidArgument(format!(
                "cannot deflate non-portfolio asset: {}",
                prettify(f)
            )));
        }

        // check overflow
        let f = self
            .deflation
            .iter()
            .filter(|(denom_x, coin_x)| {
                !assets
                    .iter()
                    .any(|(denom_y, coin_y)| denom_x == denom_y && coin_x < coin_y)
            })
            .collect::<Vec<_>>();
        if !f.is_empty() {
            return Err(ContractError::InvalidArgument(format!(
                "deflation overflow: {}",
                prettify(f)
            )));
        }

        // check duplication
        let f = self
            .inflation
            .iter()
            .filter(|(denom_x, _)| self.deflation.iter().any(|(denom_y, _)| denom_x == denom_y))
            .collect::<Vec<_>>();
        if !f.is_empty() {
            return Err(ContractError::InvalidArgument(format!(
                "duplicated coin: {}",
                prettify(f)
            )));
        }

        Ok(())
    }
}

impl Default for Rebalance {
    fn default() -> Self {
        Self {
            manager: Addr::unchecked(""),
            deflation: Default::default(),
            inflation: Default::default(),
            finalized: Default::default(),
        }
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
    pub fn checked_update_cooldown(&mut self, now: u64) -> Result<(), ContractError> {
        if let Some(last_trade_time) = self.last_traded_at {
            if now < last_trade_time + self.cooldown {
                return Err(ContractError::CooldownNotExpired {});
            }
        }

        self.last_traded_at = Some(now);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod rebalance {
        use crate::test::to_units;

        use super::*;

        #[test]
        fn test_validate() {
            // check current asset
            let rebalance = Rebalance {
                manager: Addr::unchecked("manager"),
                deflation: to_units(&[("ukrw", "0.5"), ("ujpy", "0.7"), ("ueur", "0.3")]),
                inflation: vec![],
                finalized: false,
            };

            let err = rebalance
                .validate(to_units(&[("uusd", "0.5"), ("ukrw", "0.7")]))
                .unwrap_err();
            assert_eq!(
                err,
                ContractError::InvalidArgument(
                    "cannot deflate non-portfolio asset: [(\"ujpy\",\"0.7\"),(\"ueur\",\"0.3\")]"
                        .to_string()
                )
            );

            // check overflow
            let rebalance = Rebalance {
                manager: Addr::unchecked("manager"),
                deflation: to_units(&[("ukrw", "1.0")]),
                inflation: vec![],
                finalized: false,
            };

            let err = rebalance
                .validate(to_units(&[("uusd", "0.5"), ("ukrw", "0.7")]))
                .unwrap_err();
            assert_eq!(
                err,
                ContractError::InvalidArgument(
                    "deflation overflow: [(\"ukrw\",\"1\")]".to_string()
                )
            );

            // check duplication
            let rebalance = Rebalance {
                manager: Addr::unchecked("manager"),
                deflation: to_units(&[("ukrw", "1.0")]),
                inflation: to_units(&[("ukrw", "1.0")]),
                finalized: false,
            };

            let err = rebalance
                .validate(to_units(&[("uusd", "0.5"), ("ukrw", "1.2")]))
                .unwrap_err();
            assert_eq!(
                err,
                ContractError::InvalidArgument("duplicated coin: [(\"ukrw\",\"1\")]".to_string())
            );

            // ok
            let rebalance = Rebalance {
                manager: Addr::unchecked("manager"),
                deflation: to_units(&[("ukrw", "0.5"), ("ujpy", "0.7")]),
                inflation: to_units(&[("uusd", "0.5"), ("ueur", "0.7")]),
                finalized: false,
            };
            rebalance
                .validate(to_units(&[("ukrw", "0.7"), ("ujpy", "1.0")]))
                .unwrap();
        }
    }

    mod trade_info {
        use super::*;

        #[test]
        fn test_checked_update_cooldown() {
            let now = 1000;
            let cooldown = 86400;

            // no last_traded_at
            let mut trade_info = TradeInfo {
                routes: SwapRoutes(vec![]),
                cooldown,
                max_trade_amount: Default::default(),
                last_traded_at: None,
            };
            trade_info.checked_update_cooldown(now).unwrap();
            assert_eq!(trade_info.last_traded_at, Some(now));

            // some last_traded_at and not expired
            let mut trade_info = TradeInfo {
                routes: SwapRoutes(vec![]),
                cooldown,
                max_trade_amount: Default::default(),
                last_traded_at: Some(now),
            };
            let err = trade_info
                .checked_update_cooldown(now + cooldown - 1)
                .unwrap_err();
            assert_eq!(err, ContractError::CooldownNotExpired {});

            // some last_traded_at and expired
            let mut trade_info = TradeInfo {
                routes: SwapRoutes(vec![]),
                cooldown,
                max_trade_amount: Default::default(),
                last_traded_at: Some(now),
            };
            trade_info
                .checked_update_cooldown(now + cooldown + 1)
                .unwrap();
            assert_eq!(trade_info.last_traded_at, Some(now + cooldown + 1));
        }
    }
}
