mod fee;
mod pause;
mod rebalance;
mod units;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Item, Map};

pub use fee::{Fee, StreamingFee};
pub use pause::PauseInfo;
pub use rebalance::{Rebalance, TradeInfo};
pub use units::Units;

#[cw_serde]
pub struct Config {
    pub gov: Addr,
    pub paused: PauseInfo,
    pub index_denom: String,
    pub reserve_denom: String,
}

pub const RESERVE_DENOM: &str = "reserve";

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const FEE_KEY: &str = "fee";
pub const FEE: Item<Fee> = Item::new(FEE_KEY);

pub const TOTAL_SUPPLY_KEY: &str = "total_supply";
pub const TOTAL_SUPPLY: Item<Uint128> = Item::new(TOTAL_SUPPLY_KEY);

pub const INDEX_UNITS_KEY: &str = "index_units";
pub const INDEX_UNITS: Item<Units> = Item::new(INDEX_UNITS_KEY);

pub const RESERVE_UNIT_KEY: &str = "reserve_unit";
pub const RESERVE_UNIT: Item<Decimal> = Item::new(RESERVE_UNIT_KEY);

pub const LATEST_REBALANCE_ID_KEY: &str = "latest_rebalance_id";
pub const LATEST_REBALANCE_ID: Item<u64> = Item::new(LATEST_REBALANCE_ID_KEY);

pub const REBALANCES_PREFIX: &str = "rebalances";
pub const REBALANCES: Map<u64, Rebalance> = Map::new(REBALANCES_PREFIX);

pub const TRADE_INFOS_PREFIX: &str = "trade_infos";
pub const TRADE_INFOS: Map<String, TradeInfo> = Map::new(TRADE_INFOS_PREFIX);

pub const RESERVE_BUFFER_PREFIX: &str = "reserve_buffer";
pub const RESERVE_BUFFER: Map<String, Uint128> = Map::new(RESERVE_BUFFER_PREFIX);

#[cfg(test)]
pub mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{Addr, Decimal, Env, StdResult, Storage};

    use super::{
        Config, Fee, PauseInfo, StreamingFee, Units, CONFIG, FEE, INDEX_UNITS, TOTAL_SUPPLY,
    };

    pub struct StateBuilder<'a> {
        pub config: Config,
        pub fee: Fee,
        pub index_units: &'a [(&'a str, &'a str)],
        pub total_supply: u128,
    }

    impl<'a> StateBuilder<'a> {
        pub fn new(env: &Env) -> Self {
            Self {
                config: mock_config(),
                fee: mock_fee(env, None, None, None),
                index_units: &[("uatom", "0.5"), ("uosmo", "0.3")],
                total_supply: 10e6 as u128,
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
                collected: Units(vec![]),
                last_collected_at: collected_at,
            });
            self
        }

        pub fn with_total_supply(mut self, total_supply: u128) -> Self {
            self.total_supply = total_supply;
            self
        }

        pub fn add_index_unit(mut self, denom: &str, unit: &str) -> Self {
            self.index_units = &[self.index_units, &[(denom, unit)]].concat();
            self
        }

        pub fn clear_index_units(mut self) -> Self {
            self.index_units = &[];
            self
        }

        pub fn build(self, storage: &mut dyn Storage) {
            CONFIG.save(storage, &self.config).unwrap();
            FEE.save(storage, &self.fee).unwrap();

            INDEX_UNITS
                .save(
                    storage,
                    &Units(
                        self.index_units
                            .into_iter()
                            .map(|(k, v)| Ok((k.to_string(), Decimal::from_str(v)?)))
                            .collect::<StdResult<_>>()
                            .unwrap(),
                    ),
                )
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
                Units(
                    self.index_units
                        .into_iter()
                        .map(|(k, v)| Ok((k.to_string(), Decimal::from_str(v).unwrap())))
                        .collect::<StdResult<_>>()
                        .unwrap()
                )
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
            mint_fee: mint_fee.map(|v| Decimal::from_str(v)).transpose().unwrap(),
            burn_fee: burn_fee.map(|v| Decimal::from_str(v)).transpose().unwrap(),
            streaming_fee: streaming_fee
                .map(|v| -> StdResult<_> {
                    Ok(StreamingFee {
                        rate: Decimal::from_str(v)?,
                        collected: Units(vec![]),
                        last_collected_at: env.block.time.seconds(),
                    })
                })
                .transpose()
                .unwrap(),
        }
    }
}
