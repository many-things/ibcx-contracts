#![allow(dead_code)]

use std::{collections::BTreeMap, fs, path::Path, str::FromStr};

use cosmwasm_std::{coin, Coin, Decimal, Uint128};
use ibcx_interface::{core, periphery};
use osmosis_test_tube::{
    fn_execute,
    osmosis_std::types::{
        cosmos::base::v1beta1::Coin as OsmosisCoin,
        osmosis::{
            gamm::poolmodels::stableswap::{
                self,
                v1beta1::{MsgCreateStableswapPool, MsgCreateStableswapPoolResponse},
            },
            tokenfactory::v1beta1::{MsgCreateDenom, MsgMint},
        },
    },
    Account, Gamm, Module, OsmosisTestApp, Runner, SigningAccount, TokenFactory, Wasm,
};

pub const NORM: u128 = 40_000_000_000_000;

pub struct StableSwap<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for StableSwap<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> StableSwap<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub create_stable_pool: MsgCreateStableswapPool => MsgCreateStableswapPoolResponse
    }
}

pub fn unwrap_asset(asset: Option<&TestAsset>) -> (String, u64) {
    let TestAsset { denom, pool_id } = asset.unwrap();
    (denom.clone(), *pool_id)
}

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
            amount: Some(coin(NORM * NORM, &new_denom).into()),
            mint_to_address: signer.address(),
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

pub fn create_stable_pool(
    stable: &StableSwap<OsmosisTestApp>,
    signer: &SigningAccount,
    initial_liquidity: Vec<Coin>,
    scaling_factors: Vec<u64>,
) -> u64 {
    stable
        .create_stable_pool(
            MsgCreateStableswapPool {
                sender: signer.address(),
                pool_params: Some(stableswap::v1beta1::PoolParams {
                    swap_fee: "10000000000000000".to_string(),
                    exit_fee: "0".to_string(),
                }),
                initial_pool_liquidity: initial_liquidity
                    .into_iter()
                    .map(OsmosisCoin::from)
                    .collect(),
                scaling_factors,
                future_pool_governor: signer.address(),
                scaling_factor_controller: signer.address(),
            },
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
    pub stable_pool: u64,
}

pub fn setup(initial_fund: &[Coin], signer_count: u64) -> TestEnv<'static, OsmosisTestApp> {
    let app = OsmosisTestApp::new();

    let accs = app.init_accounts(initial_fund, signer_count).unwrap();
    let owner = accs.first().unwrap();

    // create denoms / provide liquidity
    let uusd = create_denom(&TokenFactory::new(&app), owner, "uusd");
    let uusd_pool = create_pool(&Gamm::new(&app), owner, &uusd, "uosmo", (7400, 10000));

    let ujpy = create_denom(&TokenFactory::new(&app), owner, "ujpy");
    let ujpy_pool = create_pool(&Gamm::new(&app), owner, &ujpy, "uosmo", (10164, 10000));

    let ukrw = create_denom(&TokenFactory::new(&app), owner, "ukrw");
    let ukrw_pool = create_pool(&Gamm::new(&app), owner, &ukrw, "uosmo", (99245, 10000));

    let uatom = create_denom(&TokenFactory::new(&app), owner, "uatom");
    let uatom_pool = create_pool(&Gamm::new(&app), owner, &uatom, "uosmo", (57622, 1000000));

    let ujk_stable_pool = create_stable_pool(
        &StableSwap::new(&app),
        owner,
        vec![coin(10164, &ujpy), coin(99245, &ukrw), coin(7400, &uusd)]
            .into_iter()
            .map(|v| Coin {
                amount: v.amount * Uint128::from(NORM),
                ..v
            })
            .collect(),
        vec![1, 1, 1],
    );

    println!("uusd: {uusd_pool}, ujpy: {ujpy_pool}, ukrw: {ukrw_pool}, uatom: {uatom_pool}, ujk_stable: {ujk_stable_pool}");

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

    let core_addr = Wasm::new(&app)
        .instantiate(
            core_code,
            &core::InstantiateMsg {
                gov: owner.address(),
                fee: core::FeePayload {
                    collector: owner.address(),
                    mint_fee: None,
                    burn_fee: Some(Decimal::from_ratio(15u64, 10000u64)),
                    streaming_fee: None,
                },
                index_denom: "uibcx".to_string(),
                index_units: vec![
                    ("uosmo".to_string(), Decimal::one()),
                    (uusd.clone(), Decimal::from_str("22.2").unwrap()),
                    (ujpy.clone(), Decimal::from_str("20.328").unwrap()),
                    (ukrw.clone(), Decimal::from_str("496.225").unwrap()),
                ],
                reserve_denom: "uosmo".to_string(),
            },
            Some(&owner.address()),
            Some("label"),
            &[],
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
            Some("label"),
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
        stable_pool: ujk_stable_pool,
    }
}
