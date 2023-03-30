mod config;
mod fee;
mod pause;
mod rebalance;
mod units;

use cosmwasm_std::Uint128;
use cw_storage_plus::{Item, Map};

pub use config::Config;
pub use fee::{Fee, StreamingFee};
pub use pause::PauseInfo;
pub use rebalance::{Rebalance, TradeInfo};
pub use units::Units;

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

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
        RESERVE_UNITS, TOTAL_SUPPLY,
    };

    pub struct StateBuilder<'a> {
        pub config: Config,
        pub fee: Fee,
        pub total_supply: u128,
        pub index_units: Vec<(&'a str, &'a str)>,
        pub reserve_units: Vec<(&'a str, &'a str)>,
        pub rebalance: Option<Rebalance>,
        pub trade_infos: Vec<(String, TradeInfo)>,
    }

    impl<'a> StateBuilder<'a> {
        pub fn new(env: &Env) -> Self {
            Self {
                config: mock_config(),
                fee: mock_fee(env, None, None, None),
                total_supply: 10e6 as u128,

                index_units: [("uatom", "0.5"), ("uosmo", "0.3")].to_vec(),
                reserve_units: [("uatom", "0.1"), ("uosmo", "0.12")].to_vec(),

                rebalance: None,
                trade_infos: vec![
                    (
                        "uatom".to_string(),
                        TradeInfo {
                            routes: vec![(0, "uosmo")].into(),
                            cooldown: 86400u64,
                            max_trade_amount: (10e6 as u128).into(),
                            last_traded_at: Some(env.block.time.seconds()),
                        },
                    ),
                    (
                        "uosmo".to_string(),
                        TradeInfo {
                            routes: vec![(0, "uatom")].into(),
                            cooldown: 86400u64,
                            max_trade_amount: (10e6 as u128).into(),
                            last_traded_at: Some(env.block.time.seconds()),
                        },
                    ),
                ],
            }
        }

        pub fn with_config(mut self, new_config: Config) -> Self {
            self.config = new_config;
            self
        }

        pub fn with_fee_collector(mut self, collector: &str) -> Self {
            self.fee.collector = Addr::unchecked(collector);
            self
        }

        pub fn with_mint_fee(mut self, mint_fee: &str) -> Self {
            self.fee.mint_fee = Some(Decimal::from_str(mint_fee).unwrap());
            self
        }

        pub fn with_burn_fee(mut self, burn_fee: &str) -> Self {
            self.fee.burn_fee = Some(Decimal::from_str(burn_fee).unwrap());
            self
        }

        pub fn with_streaming_fee(mut self, streaming_fee: &str, collected_at: u64) -> Self {
            self.fee.streaming_fee = Some(StreamingFee {
                rate: Decimal::from_str(streaming_fee).unwrap(),
                collected: vec![],
                last_collected_at: collected_at,
                freeze: false,
            });
            self
        }

        pub fn with_total_supply(mut self, total_supply: u128) -> Self {
            self.total_supply = total_supply;
            self
        }

        pub fn add_index_unit(mut self, denom: &'a str, unit: &'a str) -> Self {
            self.index_units.push((denom, unit));
            self
        }

        pub fn clear_index_units(mut self) -> Self {
            self.index_units = vec![];
            self
        }

        pub fn add_reserve_unit(mut self, denom: &'a str, unit: &'a str) -> Self {
            self.reserve_units.push((denom, unit));
            self
        }

        pub fn clear_reserve_unit(mut self) -> Self {
            self.reserve_units = vec![];
            self
        }

        pub fn with_rebalance(mut self, rebalance: Rebalance) -> Self {
            self.rebalance = Some(rebalance);
            self
        }

        pub fn add_trade_info(mut self, denom: &str, trade_info: TradeInfo) -> Self {
            self.trade_infos.push((denom.to_string(), trade_info));
            self
        }

        pub fn clear_trade_infos(mut self) -> Self {
            self.trade_infos = vec![];
            self
        }

        pub fn build(self, storage: &mut dyn Storage) {
            CONFIG.save(storage, &self.config).unwrap();
            FEE.save(storage, &self.fee).unwrap();

            INDEX_UNITS.save(storage, &self.index_units.into()).unwrap();

            RESERVE_UNITS
                .save(storage, &self.reserve_units.into())
                .unwrap();

            TOTAL_SUPPLY
                .save(storage, &self.total_supply.into())
                .unwrap()
        }

        pub fn assert(self, storage: &dyn Storage) {
            let config = CONFIG.load(storage).unwrap();
            assert_eq!(config, self.config);

            let fee = FEE.load(storage).unwrap();
            assert_eq!(fee, self.fee);

            let index_units = INDEX_UNITS.load(storage).unwrap();
            assert_eq!(
                index_units,
                self.index_units
                    .into_iter()
                    .map(|(k, v)| Ok((k.to_string(), Decimal::from_str(v).unwrap())))
                    .collect::<StdResult<Vec<_>>>()
                    .unwrap()
                    .into()
            );

            let total_supply = TOTAL_SUPPLY.load(storage).unwrap();
            assert_eq!(total_supply.u128(), self.total_supply);
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
