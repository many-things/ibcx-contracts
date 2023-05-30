#![allow(dead_code)]

use std::{collections::BTreeMap, fs, path::Path, str::FromStr};

use cosmwasm_std::{coin, Coin, Decimal};
use ibcx_interface::{core, periphery};
use osmosis_test_tube::{
    osmosis_std::types::osmosis::tokenfactory::v1beta1::{
        MsgCreateDenom, MsgMint, QueryParamsRequest,
    },
    Account, Gamm, Module, OsmosisTestApp, Runner, SigningAccount, TokenFactory, Wasm,
};

pub const NORM: u128 = 40_000_000_000_000;

pub fn create_denom(
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

pub fn create_pool(
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

pub struct TestAsset {
    pub denom: String,
    pub pool_id: u64,
}

pub struct TestEnv<'a, R: Runner<'a>> {
    pub app: R,
    pub accs: Vec<SigningAccount>,

    pub core_addr: String,
    pub perp_addr: String,

    pub assets: BTreeMap<&'a str, TestAsset>,
}

pub fn setup(initial_fund: &[Coin], signer_count: u64) -> TestEnv<'static, OsmosisTestApp> {
    let app = OsmosisTestApp::new();

    let accs = app.init_accounts(initial_fund, signer_count).unwrap();
    let owner = accs.first().unwrap();

    // create denoms / provide liquidity
    let uusd = create_denom(&TokenFactory::new(&app), owner, "uusd");
    let uusd_pool = create_pool(&Gamm::new(&app), owner, &uusd, "uosmo", (74, 100));

    let ujpy = create_denom(&TokenFactory::new(&app), owner, "ujpy");
    let ujpy_pool = create_pool(&Gamm::new(&app), owner, &ujpy, "uosmo", (10164, 10000));

    let ukrw = create_denom(&TokenFactory::new(&app), owner, "ukrw");
    let ukrw_pool = create_pool(&Gamm::new(&app), owner, &ukrw, "uosmo", (99245, 10000));

    let uatom = create_denom(&TokenFactory::new(&app), owner, "uatom");
    let uatom_pool = create_pool(&Gamm::new(&app), owner, &uatom, "uosmo", (57622, 1000000));

    println!("uusd: {uusd_pool}, ujpy: {ujpy_pool}, ukrw: {ukrw_pool}, uatom: {uatom_pool}");

    // store codes
    let base_path = Path::new("../target/wasm32-unknown-unknown/release/");
    let core_path = base_path.join("ibcx_core.wasm");
    let perp_path = base_path.join("ibcx_periphery.wasm");

    let core_wasm = fs::read(core_path).unwrap();
    let core_store_resp = Wasm::new(&app).store_code(&core_wasm, None, owner).unwrap();
    let core_code = core_store_resp.data.code_id;

    let perp_wasm = fs::read(perp_path).unwrap();
    let perp_store_resp = Wasm::new(&app).store_code(&perp_wasm, None, owner).unwrap();
    let perp_code = perp_store_resp.data.code_id;

    println!("core: {core_code}, perp: {perp_code}");

    // instantiate codes
    let denom_creation_fee = TokenFactory::new(&app)
        .query_params(&QueryParamsRequest {})
        .unwrap()
        .params
        .unwrap()
        .denom_creation_fee;

    let core_addr = Wasm::new(&app)
        .instantiate(
            core_code,
            &core::InstantiateMsg {
                gov: owner.address(),
                fee: core::FeePayload {
                    collector: owner.address(),
                    mint_fee: Some(Decimal::from_ratio(5u64, 10000u64)),
                    burn_fee: Some(Decimal::from_ratio(15u64, 10000u64)),
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
            Some(&owner.address()),
            None,
            &[coin(
                denom_creation_fee[0].amount.parse().unwrap(),
                &denom_creation_fee[0].denom,
            )],
            owner,
        )
        .unwrap()
        .data
        .address;

    let perp_addr = Wasm::new(&app)
        .instantiate(
            perp_code,
            &periphery::InstantiateMsg {},
            Some(&owner.address()),
            None,
            &[],
            owner,
        )
        .unwrap()
        .data
        .address;

    println!("core: {core_addr}, perp: {perp_addr}");

    TestEnv {
        app,
        accs,
        core_addr,
        perp_addr,
        assets: BTreeMap::from([
            (
                "uusd",
                TestAsset {
                    denom: uusd,
                    pool_id: uusd_pool,
                },
            ),
            (
                "ujpy",
                TestAsset {
                    denom: ujpy,
                    pool_id: ujpy_pool,
                },
            ),
            (
                "ukrw",
                TestAsset {
                    denom: ukrw,
                    pool_id: ukrw_pool,
                },
            ),
            (
                "uatom",
                TestAsset {
                    denom: uatom,
                    pool_id: uatom_pool,
                },
            ),
        ]),
    }
}