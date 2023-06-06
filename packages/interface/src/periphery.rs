use std::collections::HashSet;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Uint128};

use crate::types::{SwapRoute, SwapRoutes};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct RouteKey(pub (String, String));

#[cw_serde]
pub struct SwapInfo(pub (RouteKey, SwapRoutes));

pub fn extract_pool_ids(swap_info: Vec<SwapInfo>) -> Vec<u64> {
    let mut pool_ids = swap_info
        .into_iter()
        .flat_map(|v| v.0 .1 .0.into_iter().map(|r| r.pool_id))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    pool_ids.sort();

    pool_ids
}

#[cw_serde]
pub struct SwapInfoCompact {
    pub key: String,
    pub routes: Vec<String>,
}

impl From<SwapInfoCompact> for SwapInfo {
    fn from(v: SwapInfoCompact) -> Self {
        let keys: Vec<_> = v.key.split(',').collect();

        let denom_in = keys.first().unwrap().to_string();
        let denom_out = keys.last().unwrap().to_string();

        let routes: Vec<_> = v
            .routes
            .into_iter()
            .map(|v| {
                let splitted: Vec<_> = v.split(',').collect();

                let pool_id = splitted.first().unwrap().parse::<u64>().unwrap();
                let token_denom = splitted.last().unwrap().to_string();

                SwapRoute {
                    pool_id,
                    token_denom,
                }
            })
            .collect();

        Self((RouteKey((denom_in, denom_out)), SwapRoutes(routes)))
    }
}

#[cw_serde]
pub struct SwapInfosCompact(pub Vec<SwapInfoCompact>);

impl From<SwapInfosCompact> for Vec<SwapInfo> {
    fn from(vs: SwapInfosCompact) -> Self {
        vs.0.into_iter().map(|v| v.into()).collect()
    }
}

#[cw_serde]
pub enum ExecuteMsg {
    // fixed input
    // min output
    MintExactAmountIn {
        core_addr: String,
        input_asset: String,
        min_output_amount: Uint128,
        swap_info: SwapInfosCompact,
    },
    // max input
    // fixed output
    MintExactAmountOut {
        core_addr: String,
        output_amount: Uint128,
        input_asset: String,
        swap_info: SwapInfosCompact,
    },
    // fixed input
    // min output
    BurnExactAmountIn {
        core_addr: String,
        output_asset: String,
        min_output_amount: Uint128,
        swap_info: SwapInfosCompact,
    },
    // max input
    // fixed output
    BurnExactAmountOut {
        core_addr: String,
        output_asset: Coin,
        swap_info: SwapInfosCompact,
    },

    // internal
    FinishOperation {
        refund_to: String,
        refund_asset: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(SimulateMintExactAmountInResponse)]
    SimulateMintExactAmountIn {
        core_addr: String,
        input_asset: Coin,
        swap_info: SwapInfosCompact,
    },

    #[returns(SimulateMintExactAmountOutResponse)]
    SimulateMintExactAmountOut {
        core_addr: String,
        output_amount: Uint128,
        input_asset: String,
        swap_info: SwapInfosCompact,
    },

    #[returns(SimulateBurnExactAmountInResponse)]
    SimulateBurnExactAmountIn {
        core_addr: String,
        input_amount: Uint128,
        output_asset: String,
        swap_info: SwapInfosCompact,
    },

    #[returns(SimulateBurnExactAmountInResponse)]
    SimulateBurnExactAmountInV2 {
        core_addr: String,
        input_amount: Uint128,
        output_asset: String,
        swap_info: SwapInfosCompact,
    },

    #[returns(SimulateBurnExactAmountOutResponse)]
    SimulateBurnExactAmountOut {
        core_addr: String,
        output_asset: Coin,
        swap_info: SwapInfosCompact,
    },
}

#[cw_serde]
pub struct SimulateMintExactAmountInResponse {
    pub mint_amount: Uint128,
    pub mint_spend_amount: Vec<Coin>,
    pub swap_result_amount: Coin,
}

#[cw_serde]
pub struct SimulateMintExactAmountOutResponse {
    pub mint_amount: Uint128,
    pub mint_spend_amount: Vec<Coin>,
    pub swap_result_amount: Coin,
}

#[cw_serde]
pub struct SimulateBurnExactAmountInResponse {
    pub burn_amount: Uint128,
    pub burn_redeem_amount: Vec<Coin>,
    pub swap_result_amount: Coin,
}

#[cw_serde]
pub struct SimulateBurnExactAmountOutResponse {
    pub burn_amount: Uint128,
    pub burn_redeem_amount: Vec<Coin>,
    pub swap_result_amount: Coin,
}

#[cw_serde]
pub struct MigrateMsg {
    pub force: Option<bool>,
}

#[cfg(test)]
mod tests {
    use crate::{
        periphery::{RouteKey, SwapInfo, SwapInfosCompact},
        types::{SwapRoute, SwapRoutes},
    };

    use super::SwapInfoCompact;

    #[test]
    fn test_swap_info_compact() {
        let pool_id = 808;
        let denom_in = "uosmo".to_string();
        let denom_out = "factory/osmo1jjx3kvnf0jk3fu2twfgt8wld9qtzfw08nyvm65/uakt-v1".to_string();

        let compact = SwapInfoCompact {
            key: format!("{denom_in},{denom_out}"),
            routes: vec![format!("{pool_id},{denom_in}")],
        };

        let expected = SwapInfo((
            RouteKey((denom_in.clone(), denom_out)),
            SwapRoutes(vec![SwapRoute {
                pool_id,
                token_denom: denom_in,
            }]),
        ));

        let actual: SwapInfo = compact.into();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_swap_infos_compact() {
        let pool_id = 808;
        let denom_in = "uosmo".to_string();
        let denom_out = "factory/osmo1jjx3kvnf0jk3fu2twfgt8wld9qtzfw08nyvm65/uakt-v1".to_string();

        let compact = SwapInfoCompact {
            key: format!("{denom_in},{denom_out}"),
            routes: vec![format!("{pool_id},{denom_in}")],
        };

        let expected = SwapInfo((
            RouteKey((denom_in.clone(), denom_out)),
            SwapRoutes(vec![SwapRoute {
                pool_id,
                token_denom: denom_in,
            }]),
        ));
        let expected = vec![&expected]
            .repeat(10)
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();

        let actual: Vec<_> = SwapInfosCompact(
            vec![&compact]
                .repeat(10)
                .into_iter()
                .cloned()
                .collect::<Vec<_>>(),
        )
        .into();

        assert_eq!(expected, actual);
    }
}
