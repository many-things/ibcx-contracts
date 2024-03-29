use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Binary, Coin, CosmosMsg, Decimal};
use osmosis_std::types::osmosis::gamm::{self, poolmodels::stableswap};
use osmosis_test_tube::{
    fn_execute,
    osmosis_std::types::{
        cosmos::base::v1beta1::Coin as ProtoCoin,
        osmosis::{
            concentratedliquidity::poolmodel::concentrated::v1beta1::{
                MsgCreateConcentratedPool, MsgCreateConcentratedPoolResponse,
            },
            gamm::poolmodels::{
                balancer::v1beta1::{MsgCreateBalancerPool, MsgCreateBalancerPoolResponse},
                stableswap::v1beta1::{MsgCreateStableswapPool, MsgCreateStableswapPoolResponse},
            },
        },
    },
    Account, Module, OsmosisTestApp, Runner, SigningAccount,
};

use crate::{ConcentratedPool, StablePool, WeightedPool};

#[cw_serde]
#[serde(untagged)]
pub enum Pool {
    CW {
        #[serde(rename = "@type")]
        type_url: String,
        contract_address: String,
        pool_id: String,
        code_id: String,
        instantiate_msg: Binary,
    },
    Stable(StablePool),
    Weighted(WeightedPool),
    Concentrated(ConcentratedPool),
}

#[allow(unused)]
pub enum PoolInfo {
    Stable {
        swap_fee: Option<Decimal>,
        exit_fee: Option<Decimal>,
        assets: Vec<Coin>,
        scaling_factors: Vec<u64>,
    },
    Balancer {
        swap_fee: Option<Decimal>,
        exit_fee: Option<Decimal>,
        assets: Vec<(Coin, u128)>,
    },
    Concentrated {
        denom0: String,
        denom1: String,
        tick_spacing: u64,
        spread_factor: Decimal,
    },
}

impl From<Pool> for PoolInfo {
    fn from(v: Pool) -> Self {
        match v {
            Pool::Stable(p) => Self::Stable {
                swap_fee: Some(p.pool_params.swap_fee),
                exit_fee: None,
                assets: p.pool_liquidity,
                scaling_factors: p.scaling_factors.into_iter().collect(),
            },
            Pool::Weighted(p) => {
                let pool_assets = p
                    .pool_assets
                    .into_iter()
                    .map(|v| {
                        (
                            coin(
                                v.token.amount.to_string().parse::<u128>().unwrap(),
                                v.token.denom,
                            ),
                            v.weight.to_string().parse::<u128>().unwrap(),
                        )
                    })
                    .collect::<Vec<_>>();

                let weight_sum = pool_assets.iter().map(|v| v.1).sum::<u128>();

                Self::Balancer {
                    swap_fee: Some(p.pool_params.swap_fee),
                    exit_fee: None,
                    assets: pool_assets
                        .into_iter()
                        .map(|(asset, weight)| (asset, weight_sum / weight))
                        .collect(),
                }
            }
            // TODO: make CL pool simulation work
            // TODO: make CW pool simulation work
            _ => Self::Balancer {
                swap_fee: None,
                exit_fee: None,
                assets: vec![(coin(1, "uosmo"), 100_000), (coin(1, "uion"), 100_000)],
            },
        }
    }
}

impl From<Pool> for crate::Pool {
    fn from(value: Pool) -> crate::Pool {
        match value {
            Pool::Stable(pool) => Self::Stable(StablePool {
                type_url: pool.type_url,
                address: pool.address,
                id: pool.id,
                pool_params: pool.pool_params,
                pool_liquidity: pool.pool_liquidity,
                scaling_factors: pool.scaling_factors,
                scaling_factor_controller: pool.scaling_factor_controller,
                total_shares: pool.total_shares,
                future_pool_governor: pool.future_pool_governor,
            }),
            Pool::Weighted(pool) => Self::Weighted(WeightedPool {
                type_url: pool.type_url,
                address: pool.address,
                id: pool.id,
                future_pool_governor: pool.future_pool_governor,
                pool_params: pool.pool_params,
                pool_assets: pool.pool_assets,
                total_shares: pool.total_shares,
                total_weight: pool.total_weight,
            }),
            // TODO: make CL pool simulation work
            // TODO: make CW pool simulation work
            _ => unimplemented!(),
        }
    }
}

impl PoolInfo {
    pub fn get_denoms(&self) -> Vec<String> {
        match self {
            Self::Stable { assets, .. } => assets.iter().map(|v| v.denom.clone()).collect(),
            Self::Balancer { assets, .. } => assets.iter().map(|v| v.0.denom.clone()).collect(),
            Self::Concentrated { denom0, denom1, .. } => vec![denom0.clone(), denom1.clone()],
        }
    }
}

const DEFAULT_SWAP_FEE: u128 = 10_000_000_000_000_000; // 0.01
const DEFAULT_EXIT_FEE: u128 = 0;

pub fn load_pools<P: AsRef<Path>>(path: P) -> anyhow::Result<BTreeMap<u64, crate::Pool>> {
    let read = fs::read_to_string(path)?;

    #[cw_serde]
    pub struct PoolsResponse {
        pub pools: Vec<Pool>,
    }

    let PoolsResponse { pools } = serde_json::from_str(&read)?;

    Ok(pools
        .into_iter()
        .flat_map(|v| match v.clone() {
            Pool::Stable(_) => Some(v),
            Pool::Weighted(_) => Some(v),
            _ => None,
        })
        .map(crate::Pool::from)
        .flat_map(|v| match v.clone() {
            crate::Pool::Stable(p) => Some((p.id.parse::<u64>().unwrap(), v)),
            crate::Pool::Weighted(p) => Some((p.id.parse::<u64>().unwrap(), v)),
            _ => None,
        })
        .collect())
}

pub fn load_pools_from_file(app: &OsmosisTestApp, path: PathBuf) -> anyhow::Result<()> {
    let pool = PoolManager::new(app);

    let file_dat = fs::read_to_string(path)?;

    #[cw_serde]
    pub struct PoolsResponse {
        pub pools: Vec<Pool>,
    }

    let PoolsResponse { pools } = serde_json::from_str(&file_dat)?;

    let mut denoms = Vec::from_iter(
        pools
            .iter()
            .cloned()
            .map(PoolInfo::from)
            .flat_map(|v| v.get_denoms())
            .collect::<BTreeSet<_>>(),
    );
    denoms.sort();

    let account = app.init_account(
        &denoms
            .into_iter()
            .map(|v| coin(u128::MAX, v))
            .collect::<Vec<_>>(),
    )?;

    let msgs = pools
        .iter()
        .cloned()
        .map(|v| pool.create_pool_msg(&account, v.into()))
        .collect::<Result<Vec<_>, _>>()?;

    app.execute_cosmos_msgs::<MsgCreateBalancerPoolResponse>(&msgs, &account)?;

    Ok(())
}

#[allow(unused)]
pub struct PoolManager<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for PoolManager<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> PoolManager<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! { create_stable_pool: MsgCreateStableswapPool => MsgCreateStableswapPoolResponse }
    fn_execute! { create_balancer_pool: MsgCreateBalancerPool => MsgCreateBalancerPoolResponse }
    fn_execute! { create_concentrated_pool: MsgCreateConcentratedPool => MsgCreateConcentratedPoolResponse }

    pub fn create_pool_msg(
        &self,
        signer: &SigningAccount,
        pool_info: PoolInfo,
    ) -> anyhow::Result<CosmosMsg> {
        let default_swap_fee = Decimal::from_atomics(DEFAULT_SWAP_FEE, 18)?;
        let default_exit_fee = Decimal::from_atomics(DEFAULT_EXIT_FEE, 18)?;

        match pool_info {
            PoolInfo::Stable {
                swap_fee,
                exit_fee,
                assets,
                scaling_factors,
            } => Ok(MsgCreateStableswapPool {
                sender: signer.address(),
                pool_params: Some(stableswap::v1beta1::PoolParams {
                    swap_fee: swap_fee.unwrap_or(default_swap_fee).atomics().to_string(),
                    exit_fee: exit_fee.unwrap_or(default_exit_fee).atomics().to_string(),
                }),
                initial_pool_liquidity: assets.into_iter().map(ProtoCoin::from).collect(),
                scaling_factors,
                future_pool_governor: signer.address(),
                scaling_factor_controller: signer.address(),
            }
            .into()),
            PoolInfo::Balancer {
                swap_fee,
                exit_fee,
                assets,
            } => Ok(MsgCreateBalancerPool {
                sender: signer.address(),
                pool_params: Some(gamm::v1beta1::PoolParams {
                    swap_fee: swap_fee.unwrap_or(default_swap_fee).atomics().to_string(),
                    exit_fee: exit_fee.unwrap_or(default_exit_fee).atomics().to_string(),
                    smooth_weight_change_params: None,
                }),
                pool_assets: assets
                    .into_iter()
                    .map(|(asset, weight)| gamm::v1beta1::PoolAsset {
                        token: Some(asset.into()),
                        weight: weight.to_string(),
                    })
                    .collect(),
                future_pool_governor: signer.address(),
            }
            .into()),
            PoolInfo::Concentrated {
                denom0,
                denom1,
                tick_spacing,
                spread_factor,
            } => Ok(MsgCreateConcentratedPool {
                sender: signer.address(),
                denom0,
                denom1,
                tick_spacing,
                spread_factor: spread_factor.atomics().to_string(),
            }
            .into()),
        }
    }
}
