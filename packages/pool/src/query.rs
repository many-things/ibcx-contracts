use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    from_json, to_json_vec, Binary, CustomQuery, Deps, QuerierWrapper, QueryRequest, StdResult,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::PoolRequest;

use crate::{ConcentratedPool, OsmosisPool, PoolError, StablePool, WeightedPool};

fn raw_query<C: CustomQuery>(
    querier: &QuerierWrapper<C>,
    request: &QueryRequest<C>,
) -> StdResult<Binary> {
    use cosmwasm_std::{ContractResult, StdError, SystemResult};

    let raw = to_json_vec(request).map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {serialize_err}"))
    })?;
    match querier.raw_query(&raw) {
        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
            "Querier system error: {system_err}"
        ))),
        SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!(
            "Querier contract error: {contract_err}"
        ))),
        SystemResult::Ok(ContractResult::Ok(value)) => Ok(value),
    }
}

const POOL_WEIGHTED: &str = "eyJwb29sIjp7IkB0eXBlIjoiL29zbW9zaXMuZ2FtbS52MWJldGExLlBvb2wi";
const POOL_STABLE: &str =
    "eyJwb29sIjp7IkB0eXBlIjoiL29zbW9zaXMuZ2FtbS5wb29sbW9kZWxzLnN0YWJsZXN3YXAudjFiZXRhMS5Qb29sI";
const POOL_CONCENTRATED: &str =
    "eyJwb29sIjp7IkB0eXBlIjoiL29zbW9zaXMuY29uY2VudHJhdGVkbGlxdWlkaXR5LnYxYmV0YTEuUG9v";

#[derive(Debug)]
enum PoolType {
    Weighted,
    Stable,
    Concentrated,
}

fn decode_pool(v: Binary) -> Result<Option<Box<dyn OsmosisPool>>, PoolError> {
    let pool_type = match v.to_base64() {
        b if b.starts_with(POOL_WEIGHTED) => PoolType::Weighted,
        b if b.starts_with(POOL_STABLE) => PoolType::Stable,
        b if b.starts_with(POOL_CONCENTRATED) => PoolType::Concentrated,
        _ => return Ok(None),
    };

    println!("{}", String::from_utf8(v.to_vec()).unwrap());

    #[cw_serde]
    pub struct PoolResponse<T> {
        pub pool: T,
    }

    Ok(Some(match pool_type {
        PoolType::Weighted => Box::new(from_json::<PoolResponse<WeightedPool>>(&v)?.pool),
        PoolType::Stable => Box::new(from_json::<PoolResponse<StablePool>>(&v)?.pool),
        PoolType::Concentrated => Box::new(from_json::<PoolResponse<ConcentratedPool>>(&v)?.pool),
    }))
}

pub fn query_pools(
    deps: &Deps,
    pool_ids: Vec<u64>,
) -> Result<Vec<Box<dyn OsmosisPool>>, PoolError> {
    let pool_resps = pool_ids
        .into_iter()
        .map(|v| raw_query(&deps.querier, &PoolRequest { pool_id: v }.into()))
        .collect::<StdResult<Vec<_>>>()?;

    let pools = pool_resps
        .into_iter()
        .map(decode_pool)
        .collect::<Result<Vec<_>, PoolError>>()?;

    Ok(pools.into_iter().flatten().collect())
}
