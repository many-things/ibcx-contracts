use std::{fs, path::Path, str::FromStr};

use cosmwasm_std::{coin, CosmosMsg, Decimal, Uint128};

use osmosis_std::{
    shim::{Duration, Timestamp},
    types::osmosis::{
        incentives::MsgCreateGauge,
        lockup::{LockQueryType, QueryCondition},
        poolmanager::v1beta1::{
            EstimateSwapExactAmountInRequest, EstimateSwapExactAmountInResponse,
            EstimateSwapExactAmountOutRequest, EstimateSwapExactAmountOutResponse,
            SwapAmountInRoute, SwapAmountOutRoute,
        },
    },
};
use osmosis_test_tube::{
    cosmrs::proto::cosmos::bank::v1beta1::QueryBalanceRequest,
    fn_query,
    osmosis_std::types::osmosis::tokenfactory::v1beta1::{
        MsgCreateDenom, MsgMint, QueryParamsRequest,
    },
    Account, Bank, Gamm, Module, OsmosisTestApp, Runner, SigningAccount, TokenFactory, Wasm,
};

use ibcx_interface::{core, periphery};

const NORM: u128 = 1_000_000_000_000_000;

pub struct Querier<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Querier<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Querier<'a, R>
where
    R: Runner<'a>,
{
    fn_query! {
        pub estimate_swap_exact_amount_in["/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountIn"]: EstimateSwapExactAmountInRequest => EstimateSwapExactAmountInResponse
    }

    fn_query! {
        pub estimate_swap_exact_amount_out["/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountOut"]: EstimateSwapExactAmountOutRequest => EstimateSwapExactAmountOutResponse
    }
}

fn create_denom(
    fact: &TokenFactory<OsmosisTestApp>,
    signer: &SigningAccount,
    denom: &str,
) -> String {
    let new_denom = fact
        .create_denom(
            MsgCreateDenom {
                sender: signer.address(),
                subdenom: denom.to_string(),
            },
            signer,
        )
        .unwrap()
        .data
        .new_token_denom;

    fact.mint(
        MsgMint {
            sender: signer.address(),
            amount: Some(coin(10 * NORM, &new_denom).into()),
        },
        signer,
    )
    .unwrap();

    new_denom
}

fn create_pool(
    pool: &Gamm<OsmosisTestApp>,
    signer: &SigningAccount,
    x_denom: &str,
    y_denom: &str,
    price: (u128, u128),
) -> u64 {
    pool.create_basic_pool(
        &[coin(NORM / price.1 * price.0, x_denom), coin(NORM, y_denom)],
        signer,
    )
    .unwrap()
    .data
    .pool_id
}

/**
{
  "body": {
    "messages": [
      {
        "@type": "/osmosis.incentives.MsgCreateGauge",
        "is_perpetual": false,
        "owner": "osmo14n3a65fnqz9jve85l23al6m3pjugf0atvrfqh5",
        "distribute_to": {
          "lock_query_type": "ByDuration",
          "denom": "gamm/pool/1013",
          "duration": "1209600s",
          "timestamp": "1970-01-01T00:00:00Z"
        },
        "coins": [{ "denom": "uion", "amount": "10000" }],
        "start_time": "2023-04-13T19:00:00Z",
        "num_epochs_paid_over": "120"
      }
    ],
    "memo": "",
    "timeout_height": "0",
    "extension_options": [],
    "non_critical_extension_options": []
  },
  "auth_info": {
    "signer_infos": [],
    "fee": {
      "amount": [{ "denom": "uosmo", "amount": "450" }],
      "gas_limit": "179795",
      "payer": "",
      "granter": ""
    }
  },
  "signatures": []
}
 */
#[test]
fn test_cosmos_msg_to_json() {
    let msgs: Vec<CosmosMsg> = vec![MsgCreateGauge {
        is_perpetual: false,
        owner: "osmo1k8re7jwz6rnnwrktnejdwkwnncte7ek7gt29gvnl3sdrg9mtnqkse6nmqm".to_string(),
        distribute_to: Some(QueryCondition {
            lock_query_type: LockQueryType::ByDuration.into(),
            denom: "gamm/pool/1013".to_string(),
            duration: Some(Duration {
                seconds: 1209600,
                nanos: 0,
            }),
            timestamp: None,
        }),
        coins: vec![coin(70980000, "uion").into()],
        start_time: Some(Timestamp {
            seconds: 1684497600,
            nanos: 0,
        }),
        num_epochs_paid_over: 120,
    }
    .into()];

    println!("{}", serde_json::to_string_pretty(&msgs).unwrap());
}

#[test]
fn test_integration() {
    let app = OsmosisTestApp::new();

    let acc = app.init_account(&[coin(10 * NORM, "uosmo")]).unwrap();

    let bank = Bank::new(&app);
    let wasm = Wasm::new(&app);
    let fact = TokenFactory::new(&app);
    let gamm = Gamm::new(&app);

    // create denoms / provide liquidity
    let uusd = create_denom(&fact, &acc, "uusd");
    let uusd_pool = create_pool(&gamm, &acc, &uusd, "uosmo", (74, 100));

    let ujpy = create_denom(&fact, &acc, "ujpy");
    let ujpy_pool = create_pool(&gamm, &acc, &ujpy, "uosmo", (10164, 10000));

    let ukrw = create_denom(&fact, &acc, "ukrw");
    let ukrw_pool = create_pool(&gamm, &acc, &ukrw, "uosmo", (99245, 10000));

    println!("uusd: {uusd_pool}, ujpy: {ujpy_pool}, ukrw: {ukrw_pool}");

    // store codes
    let base_path = Path::new("../target/wasm32-unknown-unknown/release/");
    let core_path = base_path.join("ibcx_core.wasm");
    let perp_path = base_path.join("ibcx_periphery.wasm");

    let core_wasm = fs::read(core_path).unwrap();
    let core_store_resp = wasm.store_code(&core_wasm, None, &acc).unwrap();
    let core_code = core_store_resp.data.code_id;

    let perp_wasm = fs::read(perp_path).unwrap();
    let perp_store_resp = wasm.store_code(&perp_wasm, None, &acc).unwrap();
    let perp_code = perp_store_resp.data.code_id;

    println!("core: {core_code}, perp: {perp_code}");

    // instantiate codes
    let denom_creation_fee = fact
        .query_params(&QueryParamsRequest {})
        .unwrap()
        .params
        .unwrap()
        .denom_creation_fee;

    let core_addr = wasm
        .instantiate(
            core_code,
            &core::InstantiateMsg {
                gov: acc.address(),
                fee: core::FeePayload {
                    collector: acc.address(),
                    mint_fee: None,
                    burn_fee: None,
                    streaming_fee: None,
                },
                index_denom: "uibcx".to_string(),
                index_units: vec![
                    (uusd.clone(), Decimal::from_str("22.2").unwrap()),
                    (ujpy.clone(), Decimal::from_str("20.328").unwrap()),
                    (ukrw.clone(), Decimal::from_str("496.225").unwrap()),
                ],
                reserve_denom: "uosmo".to_string(),
            },
            Some(&acc.address()),
            None,
            &[coin(
                denom_creation_fee[0].amount.parse().unwrap(),
                &denom_creation_fee[0].denom,
            )],
            &acc,
        )
        .unwrap()
        .data
        .address;

    let perp_addr = wasm
        .instantiate(
            perp_code,
            &periphery::InstantiateMsg {},
            Some(&acc.address()),
            None,
            &[],
            &acc,
        )
        .unwrap()
        .data
        .address;

    println!("core: {core_addr}, perp: {perp_addr}");

    // mint & burn (core)
    let config: core::GetConfigResponse = wasm
        .query(&core_addr, &core::QueryMsg::GetConfig { time: None })
        .unwrap();
    let portfolio: core::GetPortfolioResponse = wasm
        .query(&core_addr, &core::QueryMsg::GetPortfolio { time: None })
        .unwrap();

    let mut funds = portfolio
        .units
        .into_iter()
        .map(|(denom, unit)| coin((Uint128::new(1_000_000) * unit).u128(), denom))
        .collect::<Vec<_>>();
    funds.sort_by(|a, b| a.denom.cmp(&b.denom));

    wasm.execute(
        &core_addr,
        &core::ExecuteMsg::Mint {
            amount: Uint128::new(1_000_000),
            receiver: None,
            refund_to: None,
        },
        funds.as_slice(),
        &acc,
    )
    .unwrap();

    let balance = bank
        .query_balance(&QueryBalanceRequest {
            address: acc.address(),
            denom: config.index_denom.clone(),
        })
        .unwrap()
        .balance;
    assert_eq!(balance.unwrap().amount, "1000000");

    wasm.execute(
        &core_addr,
        &core::ExecuteMsg::Burn { redeem_to: None },
        &[coin(1_000_000, &config.index_denom)],
        &acc,
    )
    .unwrap();

    let balance = bank
        .query_balance(&QueryBalanceRequest {
            address: acc.address(),
            denom: config.index_denom.clone(),
        })
        .unwrap()
        .balance;
    assert_eq!(balance.unwrap().amount, "0");

    // test estimation

    let querier = Querier::new(&app);

    let estimate_in_resp = querier
        .estimate_swap_exact_amount_in(&EstimateSwapExactAmountInRequest {
            sender: acc.address(),
            pool_id: uusd_pool,
            token_in: coin(1_000_000, &uusd).to_string(),
            routes: vec![SwapAmountInRoute {
                pool_id: uusd_pool,
                token_out_denom: "uosmo".to_string(),
            }],
        })
        .unwrap();

    let estimate_out_resp = querier
        .estimate_swap_exact_amount_out(&EstimateSwapExactAmountOutRequest {
            sender: acc.address(),
            pool_id: uusd_pool,
            token_out: coin(estimate_in_resp.token_out_amount.parse().unwrap(), "uosmo")
                .to_string(),
            routes: vec![SwapAmountOutRoute {
                pool_id: uusd_pool,
                token_in_denom: uusd.clone(),
            }],
        })
        .unwrap();

    assert_eq!(estimate_out_resp.token_in_amount, format!("{}", 1_000_000));

    // mint / burn (periphery)

    let pairs = [(uusd, uusd_pool), (ujpy, ujpy_pool), (ukrw, ukrw_pool)];

    let swap_info = periphery::SwapInfosCompact(
        pairs
            .iter()
            .map(|(denom, pool_id)| periphery::SwapInfoCompact {
                key: format!("uosmo,{}", denom),
                routes: vec![format!("{},uosmo", pool_id)],
            })
            .collect::<Vec<_>>(),
    );

    let sim_mint_resp: periphery::SimulateMintExactAmountOutResponse = wasm
        .query(
            &perp_addr,
            &periphery::QueryMsg::SimulateMintExactAmountOut {
                core_addr: core_addr.clone(),
                output_amount: Uint128::new(1_000),
                input_asset: "uosmo".to_string(),
                swap_info: swap_info.clone(),
            },
        )
        .unwrap();

    wasm.execute(
        &perp_addr,
        &periphery::ExecuteMsg::MintExactAmountOut {
            core_addr: core_addr.clone(),
            output_amount: Uint128::new(1_000),
            input_asset: "uosmo".to_string(),
            swap_info,
        },
        &[coin(1_000_000, "uosmo")],
        &acc,
    )
    .unwrap();

    let swap_info = periphery::SwapInfosCompact(
        pairs
            .iter()
            .map(|(denom, pool_id)| periphery::SwapInfoCompact {
                key: format!("{},uosmo", denom),
                routes: vec![format!("{},uosmo", pool_id)],
            })
            .collect::<Vec<_>>(),
    );

    let sim_burn_resp: periphery::SimulateBurnExactAmountInResponse = wasm
        .query(
            &perp_addr,
            &periphery::QueryMsg::SimulateBurnExactAmountIn {
                core_addr: core_addr.clone(),
                input_amount: Uint128::new(1_000),
                output_asset: "uosmo".to_string(),
                swap_info: swap_info.clone(),
            },
        )
        .unwrap();

    wasm.execute(
        &perp_addr,
        &periphery::ExecuteMsg::BurnExactAmountIn {
            core_addr,
            output_asset: "uosmo".to_string(),
            min_output_amount: Uint128::new(50_000),
            swap_info,
        },
        &[coin(1_000, &config.index_denom)],
        &acc,
    )
    .unwrap();

    let index_balance = bank
        .query_balance(&QueryBalanceRequest {
            address: acc.address(),
            denom: config.index_denom,
        })
        .unwrap()
        .balance;

    let uosmo_balance = bank
        .query_balance(&QueryBalanceRequest {
            address: acc.address(),
            denom: "uosmo".to_string(),
        })
        .unwrap()
        .balance;

    println!("{index_balance:?}");
    println!("{uosmo_balance:?}");
    println!("{}", serde_json::to_string_pretty(&sim_mint_resp).unwrap());
    println!("{}", serde_json::to_string_pretty(&sim_burn_resp).unwrap());
}
