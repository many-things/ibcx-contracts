use std::{fs, path::Path, str::FromStr};

use cosmwasm_std::{coin, Decimal, Uint128};

use osmosis_test_tube::{
    cosmrs::proto::cosmos::bank::v1beta1::QueryBalanceRequest,
    osmosis_std::types::osmosis::tokenfactory::v1beta1::{
        MsgCreateDenom, MsgMint, QueryParamsRequest,
    },
    Account, Bank, Gamm, Module, OsmosisTestApp, SigningAccount, TokenFactory, Wasm,
};

use ibcx_interface::{core, periphery};

const NORM: u128 = 1_000_000_000_000_000;

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

fn main() {
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
    let base_path = Path::new("./target/wasm32-unknown-unknown/release/");
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
                    (uusd, Decimal::from_str("22.2").unwrap()),
                    (ujpy, Decimal::from_str("20.328").unwrap()),
                    (ukrw, Decimal::from_str("496.225").unwrap()),
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
        .query(&core_addr, &core::QueryMsg::GetConfig {})
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
            denom: config.index_denom,
        })
        .unwrap()
        .balance;
    assert_eq!(balance.unwrap().amount, "0");
}
