mod config;
mod fee;
mod pause;
mod rebalance;
mod units;

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

pub use config::Config;
pub use fee::{Fee, StreamingFee};
pub use pause::PauseInfo;
pub use rebalance::{Rebalance, TradeInfo};
pub use units::Units;

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const PENDING_GOV_KEY: &str = "pending_gov";
pub const PENDING_GOV: Item<Addr> = Item::new(PENDING_GOV_KEY);

pub const FEE_KEY: &str = "fee";
pub const FEE: Item<Fee> = Item::new(FEE_KEY);

pub const TOTAL_SUPPLY_KEY: &str = "total_supply";
pub const TOTAL_SUPPLY: Item<Uint128> = Item::new(TOTAL_SUPPLY_KEY);

pub const INDEX_UNITS_KEY: &str = "index_units";
pub const INDEX_UNITS: Item<Units> = Item::new(INDEX_UNITS_KEY);

pub const REBALANCE_KEY: &str = "rebalances";
pub const REBALANCE: Item<Rebalance> = Item::new(REBALANCE_KEY);

pub const RESERVE_UNITS_KEY: &str = "reserve_units";
pub const RESERVE_UNITS: Item<Units> = Item::new(RESERVE_UNITS_KEY);

pub const TRADE_INFOS_PREFIX: &str = "trade_infos";
pub const TRADE_INFOS: Map<(&str, &str), TradeInfo> = Map::new(TRADE_INFOS_PREFIX);

#[cfg(test)]
pub mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{Addr, Decimal, Env, StdResult, Storage};

    use super::{
        Config, Fee, PauseInfo, Rebalance, StreamingFee, TradeInfo, CONFIG, FEE, INDEX_UNITS,
        REBALANCE, RESERVE_UNITS, TOTAL_SUPPLY, TRADE_INFOS,
    };

    #[derive(Default, Clone)]
    pub struct StateBuilder<'a> {
        config: Option<Config>,
        fee: Option<Fee>,
        total_supply: Option<u128>,
        index_units: Option<Vec<(&'a str, &'a str)>>,

        rebalance: Option<Rebalance>,
        reserve_units: Option<Vec<(&'a str, &'a str)>>,
        trade_infos: Option<Vec<((&'a str, &'a str), TradeInfo)>>,
    }

    impl<'a> StateBuilder<'a> {
        pub fn with_config(mut self, config: Config) -> Self {
            self.config = Some(config);
            self
        }

        pub fn with_fee(mut self, fee: Fee) -> Self {
            self.fee = Some(fee);
            self
        }

        pub fn with_total_supply(mut self, total_supply: u128) -> Self {
            self.total_supply = Some(total_supply);
            self
        }

        pub fn add_index_unit(mut self, denom: &'a str, unit: &'a str) -> Self {
            if let Some(index_units) = self.index_units.as_mut() {
                index_units.push((denom, unit));
            } else {
                self.index_units = Some(vec![(denom, unit)]);
            }

            self
        }

        pub fn empty_index_units(mut self) -> Self {
            self.index_units = Some(vec![]);
            self
        }

        pub fn with_rebalance(mut self, rebalance: Rebalance) -> Self {
            self.rebalance = Some(rebalance);
            self
        }

        pub fn add_reserve_unit(mut self, denom: &'a str, unit: &'a str) -> Self {
            if let Some(reserve_units) = self.reserve_units.as_mut() {
                reserve_units.push((denom, unit));
            } else {
                self.reserve_units = Some(vec![(denom, unit)]);
            }

            self
        }

        pub fn empty_reserve_units(mut self) -> Self {
            self.reserve_units = Some(vec![]);
            self
        }

        pub fn add_trade_info(
            mut self,
            denom_in: &'a str,
            denom_out: &'a str,
            trade_info: TradeInfo,
        ) -> Self {
            if let Some(trade_infos) = self.trade_infos.as_mut() {
                trade_infos.push(((denom_in, denom_out), trade_info));
            } else {
                self.trade_infos = Some(vec![((denom_in, denom_out), trade_info)]);
            }

            self
        }

        pub fn build(self, storage: &mut dyn Storage) {
            if let Some(config) = self.config {
                CONFIG.save(storage, &config).unwrap();
            }

            if let Some(fee) = self.fee {
                FEE.save(storage, &fee).unwrap();
            }

            if let Some(total_supply) = self.total_supply {
                TOTAL_SUPPLY.save(storage, &total_supply.into()).unwrap();
            }

            if let Some(index_units) = self.index_units {
                INDEX_UNITS.save(storage, &index_units.into()).unwrap();
            }

            if let Some(rebalance) = self.rebalance {
                REBALANCE.save(storage, &rebalance).unwrap();
            }

            if let Some(reserve_units) = self.reserve_units {
                RESERVE_UNITS.save(storage, &reserve_units.into()).unwrap();
            }

            if let Some(trade_infos) = self.trade_infos {
                for ((denom_in, denom_out), trade_info) in trade_infos {
                    TRADE_INFOS
                        .save(storage, (denom_in, denom_out), &trade_info)
                        .unwrap();
                }
            }
        }
    }

    pub fn mock_config() -> Config {
        Config {
            gov: Addr::unchecked("gov"),
            paused: PauseInfo::default(),
            index_denom: "uibcx".to_string(),
            reserve_denom: "uosmo".to_string(),
        }
    }

    pub fn mock_fee(
        env: &Env,
        mint_fee: Option<&str>,
        burn_fee: Option<&str>,
        streaming_fee: Option<&str>,
    ) -> Fee {
        Fee {
            collector: Addr::unchecked("collector"),
            mint_fee: mint_fee.map(Decimal::from_str).transpose().unwrap(),
            burn_fee: burn_fee.map(Decimal::from_str).transpose().unwrap(),
            streaming_fee: streaming_fee
                .map(|v| -> StdResult<_> {
                    Ok(StreamingFee {
                        rate: Decimal::from_str(v)?,
                        collected: vec![],
                        last_collected_at: env.block.time.seconds(),
                        freeze: false,
                    })
                })
                .transpose()
                .unwrap(),
        }
    }
}
