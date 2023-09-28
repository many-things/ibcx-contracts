mod constants;
mod pool;

use osmosis_std::{
    shim::Any,
    types::osmosis::concentratedliquidity::{self},
};
use osmosis_test_tube::{cosmrs::proto::traits::Message, OsmosisTestApp};

use ibcx_pool::{OsmosisPool, PoolError};
use pool::load_pools_from_file;

#[test]
fn test_query_pools() -> anyhow::Result<()> {
    let app = OsmosisTestApp::new();

    let cl_param: concentratedliquidity::Params = app.get_param_set(
        "concentratedliquidity",
        concentratedliquidity::Params::TYPE_URL,
    )?;

    app.set_param_set(
        "concentratedliquidity",
        Any {
            type_url: concentratedliquidity::Params::TYPE_URL.to_string(),
            value: concentratedliquidity::Params {
                is_permissionless_pool_creation_enabled: true,
                ..cl_param
            }
            .encode_to_vec(),
        },
    )?;

    let pools = load_pools_from_file(&app, None)?;
    let pools = pools
        .into_iter()
        .map(|v| -> anyhow::Result<Box<dyn OsmosisPool>> {
            match v {
                ibcx_pool::Pool::Stable(p) => Ok(Box::new(p)),
                ibcx_pool::Pool::Weighted(p) => Ok(Box::new(p)),
                _ => Err(PoolError::UnsupportedPoolType.into()),
            }
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(())
}
