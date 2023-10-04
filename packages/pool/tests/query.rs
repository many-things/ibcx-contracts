mod constants;
mod pool;
mod querier;

use std::{marker::PhantomData, str::FromStr};

use cosmwasm_std::{
    coin,
    testing::{MockApi, MockStorage},
    Decimal, Empty, OwnedDeps,
};
use ibcx_interface::periphery::{extract_pool_ids, SwapInfo, SwapInfosCompact};
use osmosis_std::{
    shim::Any,
    types::osmosis::concentratedliquidity::{self},
};
use osmosis_test_tube::{cosmrs::proto::traits::Message, OsmosisTestApp};

use ibcx_pool::{query_pools, Simulator};
use pool::load_pools_from_file;
use querier::TestTubeQuerier;

#[test]
fn test_query_pools() -> anyhow::Result<()> {
    let app = OsmosisTestApp::new();

    // make CL pool creation premissionless
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

    // apply pool state to test-tube chain
    load_pools_from_file(&app, None)?;

    // make deps
    let deps = OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: TestTubeQuerier::new(&app, None),
        custom_query_type: PhantomData::<Empty>,
    };

    let swap_info: Vec<SwapInfo> = SwapInfosCompact::new(&[
        (
            "uosmo,ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2",
            &["1,uosmo"],
        ),
        (
            "uosmo,ibc/6AE98883D4D5D5FF9E50D7130F1305DA2FFA0C652D1DD9C123657C6B4EB2DF8A",
            &["722,uosmo"],
        ),
        (
            "uosmo,ibc/0954E1C28EB7AF5B72D24F3BC2B47BBB2FDF91BDDFD57B74B99E133AED40972A",
            &["584,uosmo"],
        ),
        (
            "uosmo,ibc/46B44899322F3CD854D2D46DEEF881958467CDD4B3B10086DA49296BBED94BED",
            &["497,uosmo"],
        ),
        (
            "uosmo,ibc/987C17B11ABC2B20019178ACE62929FE9840202CE79498E29FE8E5CB02B7C0A4",
            &["604,uosmo"],
        ),
        (
            "uosmo,ibc/67795E528DF67C5606FC20F824EA39A6EF55BA133F4DC79C90A8C47A0901E17C",
            &["641,uosmo"],
        ),
        (
            "uosmo,ibc/1480B8FD20AD5FCAE81EA87584D269547DD4D436843C1D20F15E00EB64743EF4",
            &["3,uosmo"],
        ),
        (
            "uosmo,ibc/1DCC8A6CB5689018431323953344A9F6CC4D0BFB261E88C9F7777372C10CD076",
            &["42,uosmo"],
        ),
        (
            "uosmo,ibc/A8CA5EE328FA10C9519DF6057DA1F69682D28F7D0F5CCC7ECB72E3DCA2D157A4",
            &["806,uosmo"],
        ),
        (
            "uosmo,ibc/903A61A498756EA560B85A85132D3AEE21B5DEDD41213725D22ABF276EA6945E",
            &["812,uosmo"],
        ),
        ("uosmo,uion", &["2,uosmo"]),
    ])
    .into();

    let index_units = vec![
        (
            "ibc/1480B8FD20AD5FCAE81EA87584D269547DD4D436843C1D20F15E00EB64743EF4".to_string(),
            Decimal::from_str("4.946633179357422437")?,
        ),
        (
            "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2".to_string(),
            Decimal::from_str("2.255840230861511934")?,
        ),
        (
            "ibc/903A61A498756EA560B85A85132D3AEE21B5DEDD41213725D22ABF276EA6945E".to_string(),
            Decimal::from_str("1.174120469940882869")?,
        ),
        (
            "ibc/6AE98883D4D5D5FF9E50D7130F1305DA2FFA0C652D1DD9C123657C6B4EB2DF8A".to_string(),
            Decimal::from_str("12746115198434.500878905102618001")?,
        ),
        (
            "uion".to_string(),
            Decimal::from_str("0.001044791808981216")?,
        ),
        (
            "ibc/46B44899322F3CD854D2D46DEEF881958467CDD4B3B10086DA49296BBED94BED".to_string(),
            Decimal::from_str("11.135604648756401332")?,
        ),
        (
            "uosmo".to_string(),
            Decimal::from_str("29.93672805096916028")?,
        ),
        (
            "ibc/1DCC8A6CB5689018431323953344A9F6CC4D0BFB261E88C9F7777372C10CD076".to_string(),
            Decimal::from_str("7.937826225480782973")?,
        ),
        (
            "ibc/0954E1C28EB7AF5B72D24F3BC2B47BBB2FDF91BDDFD57B74B99E133AED40972A".to_string(),
            Decimal::from_str("7.658306995668883897")?,
        ),
        (
            "ibc/987C17B11ABC2B20019178ACE62929FE9840202CE79498E29FE8E5CB02B7C0A4".to_string(),
            Decimal::from_str("160.542290253493324757")?,
        ),
        (
            "ibc/A8CA5EE328FA10C9519DF6057DA1F69682D28F7D0F5CCC7ECB72E3DCA2D157A4".to_string(),
            Decimal::from_str("0.443976645359900486")?,
        ),
        (
            "ibc/67795E528DF67C5606FC20F824EA39A6EF55BA133F4DC79C90A8C47A0901E17C".to_string(),
            Decimal::from_str("102.022972150393847193")?,
        ),
    ];

    let deps_ref = deps.as_ref();
    let pools = query_pools(&deps.as_ref(), extract_pool_ids(swap_info.clone()))?;
    let simulator = Simulator::new(&deps_ref, &pools, &swap_info, &index_units);

    let sim_out = simulator.search_efficient_amount_for_input(coin(1_000_000, "uosmo"), None)?;

    println!("{}", serde_json::to_string_pretty(&sim_out)?);

    Ok(())
}
