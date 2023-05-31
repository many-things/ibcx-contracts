mod stable;
mod weighted;

use cosmwasm_std::{
    to_vec, Binary, Coin, ContractResult, CustomQuery, Decimal, Empty, QuerierWrapper,
    QueryRequest, StdError, StdResult, SystemResult, Uint128,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::PoolRequest;

pub use stable::{StablePool, StablePoolResponse};
pub use weighted::{WeightedPool, WeightedPoolResponse};

use crate::error::ContractError;

pub trait OsmosisPool {
    fn get_id(&self) -> u64;

    fn get_type(&self) -> &str;

    fn get_spread_factor(&self) -> StdResult<Decimal>;

    fn get_exit_fee(&self) -> StdResult<Decimal>;

    fn swap_exact_amount_in(
        &mut self,
        input_amount: Coin,
        output_denom: String,
        min_output_amount: Uint128,
        spread_factor: Decimal,
    ) -> Result<Uint128, ContractError>; // returns simulated amount out
    fn swap_exact_amount_out(
        &mut self,
        input_denom: String,
        max_input_amount: Uint128,
        output_amount: Coin,
        spread_factor: Decimal,
    ) -> Result<Uint128, ContractError>; // returns simulated amount in
}

fn raw_query<C: CustomQuery>(
    querier: &QuerierWrapper,
    request: &QueryRequest<C>,
) -> StdResult<Binary> {
    let raw = to_vec(request).map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
    })?;
    match querier.raw_query(&raw) {
        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
            "Querier system error: {}",
            system_err
        ))),
        SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!(
            "Querier contract error: {}",
            contract_err
        ))),
        SystemResult::Ok(ContractResult::Ok(value)) => Ok(value),
    }
}

// base64(`{"pool":{"@type":"/osmosis.gamm.v1beta1.Pool"`)
const PREFIX_WEIGHTED_POOL: &str = "eyJwb29sIjp7IkB0eXBlIjoiL29zbW9zaXMuZ2FtbS52MWJldGExLlBvb2wiL";
// base64(`{"pool":{"@type":"/osmosis.gamm.poolmodels.stableswap.v1beta1.Pool"`)
const PREFIX_STABLE_POOL: &str =
    "eyJwb29sIjp7IkB0eXBlIjoiL29zbW9zaXMuZ2FtbS5wb29sbW9kZWxzLnN0YWJsZXN3YXAudjFiZXRhMS5Qb29sIi";

pub fn query_pools(
    querier: &QuerierWrapper,
    pool_ids: Vec<u64>,
) -> Result<Vec<Box<dyn OsmosisPool>>, ContractError> {
    let raw_pool_resps = pool_ids
        .into_iter()
        .map(|v| Ok(raw_query::<Empty>(querier, &PoolRequest { pool_id: v }.into())?.to_base64()))
        .collect::<StdResult<Vec<_>>>()?;

    let pools = raw_pool_resps
        .into_iter()
        .map(|v| -> Result<Box<dyn OsmosisPool>, ContractError> {
            match v {
                v if v.starts_with(PREFIX_WEIGHTED_POOL) => Ok(Box::new(
                    WeightedPoolResponse::try_from(Binary::from_base64(&v)?)?.pool,
                )),
                v if v.starts_with(PREFIX_STABLE_POOL) => Ok(Box::new(
                    StablePoolResponse::try_from(Binary::from_base64(&v)?)?.pool,
                )),
                _ => Err(ContractError::UnsupportedPoolType),
            }
        })
        .collect::<Result<_, _>>()?;

    Ok(pools)
}
