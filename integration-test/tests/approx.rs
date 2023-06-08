mod setup;

use cosmwasm_std::{coin, Decimal, Uint128};
use ibcx_interface::{
    core,
    periphery::{
        self, SimulateBurnExactAmountInResponse, SimulateBurnExactAmountOutResponse,
        SimulateMintExactAmountInResponse,
    },
};
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

    let mut act_wasm_evts = act_res
        .events
        .into_iter()
        .filter(|v| v.ty == "wasm")
        .collect::<Vec<_>>();

    let finish_op_evt = act_wasm_evts.pop().unwrap();
    let act_token_in = test_res.swap_result_amount.amount
        - Uint128::from_str(&finish_op_evt.attributes[1].value).unwrap();

    let mint_evt = act_wasm_evts.pop().unwrap();
    let act_token_out = Uint128::from_str(&mint_evt.attributes[1].value).unwrap();

    println!("act_res.minted   => {}", act_token_out);
    println!("act_res.spent    => {}", act_token_in);
    println!("act_res.gas_used => {}", act_res.gas_info.gas_used);

    println!("===================================================================");

    println!(
        "test <-> act : error.out => {}",
        test_res.mint_amount.abs_diff(act_token_out)
    );
    println!(
        "test <-> act : error.in  => {}",
        test_res.swap_result_amount.amount.abs_diff(act_token_in),
    );
}

fn execute_burn_exact_amount_out(token_out: Uint128) {
    let env = setup(&[coin(10 * NORM, "uosmo")], 1);
    let owner = env.accs.first().unwrap();

    let wasm = Wasm::new(&env.app);

    let (uusd, uusd_pool) = unwrap_asset(env.assets.get("uusd"));
    let (ujpy, ujpy_pool) = unwrap_asset(env.assets.get("ujpy"));
    let (ukrw, _ukrw_pool) = unwrap_asset(env.assets.get("ukrw"));
    let (uatom, uatom_pool) = unwrap_asset(env.assets.get("uatom"));

    let swap_info = periphery::SwapInfosCompact(vec![
        periphery::SwapInfoCompact {
            key: format!("uosmo,{uatom}"),
            routes: vec![format!("{uatom_pool},{uatom}")],
        },
        periphery::SwapInfoCompact {
            key: format!("{uusd},{uatom}"),
            routes: vec![
                format!("{uusd_pool},uosmo"),
                format!("{uatom_pool},{uatom}"),
            ],
        },
        periphery::SwapInfoCompact {
            key: format!("{ujpy},{uatom}"),
            routes: vec![
                format!("{ujpy_pool},uosmo"),
                format!("{uatom_pool},{uatom}"),
            ],
        },
        periphery::SwapInfoCompact {
            key: format!("{ukrw},{uatom}"),
            routes: vec![
                format!("{},{ujpy}", env.stable_pool),
                format!("{ujpy_pool},uosmo"),
                format!("{uatom_pool},{uatom}"),
            ],
        },
    ]);

    let token_out_amount = Uint128::new(1000000) * token_out;

    let core_config: core::GetConfigResponse = wasm
        .query(&env.core_addr, &core::QueryMsg::GetConfig { time: None })
        .unwrap();
    let core_portfolio: core::GetPortfolioResponse = wasm
        .query(&env.core_addr, &core::QueryMsg::GetPortfolio { time: None })
        .unwrap();

    let mut funds_required = core_portfolio
        .units
        .into_iter()
        .map(|(denom, unit)| coin((token_out_amount * unit).u128(), denom))
        .collect::<Vec<_>>();
    funds_required.sort_by(|a, b| a.denom.cmp(&b.denom));

    wasm.execute(
        &env.core_addr,
        &core::ExecuteMsg::Mint {
            amount: token_out_amount,
            receiver: None,
            refund_to: None,
        },
        &funds_required,
        owner,
    )
    .unwrap();

    let test_res: SimulateBurnExactAmountOutResponse = wasm
        .query(
            &env.perp_addr,
            &periphery::QueryMsg::SimulateBurnExactAmountOut {
                core_addr: env.core_addr.clone(),
                output_asset: coin(token_out_amount.u128(), &uatom),
                swap_info: swap_info.clone(),
            },
        )
        .unwrap();

    println!("test__res.burn    => {}", test_res.burn_amount);
    println!("test__res.spent   => {}", test_res.swap_result_amount);

    let test2_res: SimulateBurnExactAmountInResponse = wasm
        .query(
            &env.perp_addr,
            &periphery::QueryMsg::SimulateBurnExactAmountIn {
                core_addr: env.core_addr.clone(),
                input_amount: test_res.burn_amount,
                output_asset: uatom,
                swap_info: swap_info.clone(),
            },
        )
        .unwrap();

    println!("test2_res.burn     => {}", test2_res.burn_amount);
    println!("test2_res.return   => {}", test2_res.swap_result_amount);

    let act_res = wasm
        .execute(
            &env.perp_addr,
            &periphery::ExecuteMsg::BurnExactAmountOut {
                core_addr: env.core_addr.clone(),
                output_asset: test_res.swap_result_amount.clone(),
                swap_info,
            },
            &[coin(test_res.burn_amount.u128(), core_config.index_denom)],
            owner,
        )
        .unwrap();

    let mut act_wasm_evts = act_res
        .events
        .into_iter()
        .filter(|v| v.ty == "wasm")
        .collect::<Vec<_>>();

    let finish_op_atom_evt = act_wasm_evts.pop().unwrap();
    let atom_return = Uint128::from_str(&finish_op_atom_evt.attributes[1].value).unwrap();
    let finish_op_ibcx_evt = act_wasm_evts.pop().unwrap();
    let ibcx_refund = Uint128::from_str(&finish_op_ibcx_evt.attributes[1].value).unwrap();

    println!("act_res.atom_return => {}", atom_return);
    println!("act_res.ibcx_refund => {}", ibcx_refund);
    println!("act_res.gas_used    => {}", act_res.gas_info.gas_used);

    println!("===================================================================");

    println!("test <-> act : error.burn  => {}", ibcx_refund);
    println!(
        "test <-> act : error.spent => {}",
        test_res.swap_result_amount.amount.abs_diff(atom_return)
    );

    println!("===================================================================");
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
fn test_burn() {
    for token_out in [
        Uint128::new(100000),
        Uint128::new(200000),
        Uint128::new(400000),
        Uint128::new(800000),
    ] {
        println!("simulate {}", token_out);
        execute_burn_exact_amount_out(token_out)
    }
}
