mod setup;

use cosmwasm_std::{coin, Uint128};
use ibcx_interface::periphery::{self, SimulateMintExactAmountInResponse};
use osmosis_test_tube::{Module, Wasm};
use std::str::FromStr;

use crate::setup::{setup, unwrap_asset, NORM};

fn execute_mint_exact_amount_in(token_in: Uint128) {
    let env = setup(&[coin(10 * NORM, "uosmo")], 1);
    let owner = env.accs.first().unwrap();

    let wasm = Wasm::new(&env.app);

    let (uusd, uusd_pool) = unwrap_asset(env.assets.get("uusd"));
    let (ujpy, ujpy_pool) = unwrap_asset(env.assets.get("ujpy"));
    let (ukrw, _ukrw_pool) = unwrap_asset(env.assets.get("ukrw"));
    let (uatom, uatom_pool) = unwrap_asset(env.assets.get("uatom"));

    let swap_info = periphery::SwapInfosCompact(vec![
        periphery::SwapInfoCompact {
            key: format!("{uatom},uosmo"),
            routes: vec![format!("{uatom_pool},{uatom}")],
        },
        periphery::SwapInfoCompact {
            key: format!("{uatom},{uusd}"),
            routes: vec![
                format!("{uatom_pool},{uatom}"),
                format!("{uusd_pool},uosmo"),
            ],
        },
        periphery::SwapInfoCompact {
            key: format!("{uatom},{ujpy}"),
            routes: vec![
                format!("{uatom_pool},{uatom}"),
                format!("{ujpy_pool},uosmo"),
            ],
        },
        periphery::SwapInfoCompact {
            key: format!("{uatom},{ukrw}"),
            routes: vec![
                format!("{uatom_pool},{uatom}"),
                format!("{ujpy_pool},uosmo"),
                format!("{},{ujpy}", env.stable_pool),
            ],
        },
    ]);

    let token_in_amount = Uint128::new(1000000) * token_in;

    let test_res: SimulateMintExactAmountInResponse = wasm
        .query(
            &env.perp_addr,
            &periphery::QueryMsg::SimulateMintExactAmountIn {
                core_addr: env.core_addr.clone(),
                input_asset: coin(token_in_amount.u128(), &uatom),
                swap_info: swap_info.clone(),
            },
        )
        .unwrap();

    println!("test_res_mint    => {}", test_res.mint_amount);
    println!("test_res_spent   => {}", test_res.swap_result_amount.amount);

    let act_res = wasm
        .execute(
            &env.perp_addr,
            &periphery::ExecuteMsg::MintExactAmountIn {
                core_addr: env.core_addr.clone(),
                input_asset: uatom.clone(),
                swap_info,
                min_output_amount: test_res.mint_amount - Uint128::new(100),
            },
            &[coin(test_res.swap_result_amount.amount.u128(), &uatom)],
            owner,
        )
        .unwrap();

    let wasm_evts = act_res
        .events
        .into_iter()
        .filter(|v| v.ty == "wasm")
        .collect::<Vec<_>>();

    println!(
        "wasm_evts: {}",
        serde_json::to_string_pretty(&wasm_evts).unwrap()
    );

    let wasm_evt = wasm_evts.last().unwrap();
    let act_token_in = test_res.swap_result_amount.amount
        - Uint128::from_str(&wasm_evt.attributes[1].value).unwrap();

    println!("act_res          => {}", act_token_in);
    println!("act_res.gas_used => {}", act_res.gas_info.gas_used);
}

#[test]
fn test_mint() {
    for token_in in [
        Uint128::new(100000),
        Uint128::new(200000),
        Uint128::new(400000),
        Uint128::new(800000),
    ] {
        println!("simulate {}", token_in);
        execute_mint_exact_amount_in(token_in)
    }
}

#[test]
fn test_burn() {}
