mod setup;

use cosmwasm_std::{coin, Decimal, Uint128};

use osmosis_std::types::{
    cosmos::bank::v1beta1::QueryBalanceRequest,
    osmosis::poolmanager::v1beta1::{
        EstimateSwapExactAmountInRequest, EstimateSwapExactAmountInResponse,
        EstimateSwapExactAmountOutRequest, EstimateSwapExactAmountOutResponse, SwapAmountInRoute,
        SwapAmountOutRoute,
    },
};
use osmosis_test_tube::{fn_query, Account, Bank, Module, Runner, Wasm};

use ibcx_interface::{core, periphery};

use crate::setup::{setup, unwrap_asset, NORM};

#[test]
fn test_integration() {
    let env = setup(&[coin(10 * NORM, "uosmo")], 1);
    let acc = env.accs.first().unwrap();

    let bank = Bank::new(&env.app);
    let wasm = Wasm::new(&env.app);

    let (uusd, uusd_pool) = unwrap_asset(env.assets.get("uusd"));
    let (ujpy, ujpy_pool) = unwrap_asset(env.assets.get("ujpy"));
    let (ukrw, ukrw_pool) = unwrap_asset(env.assets.get("ukrw"));
    let (uatom, uatom_pool) = unwrap_asset(env.assets.get("uatom"));

    // mint & burn (core)
    let config: core::GetConfigResponse = wasm
        .query(&env.core_addr, &core::QueryMsg::GetConfig { time: None })
        .unwrap();
    let portfolio: core::GetPortfolioResponse = wasm
        .query(&env.core_addr, &core::QueryMsg::GetPortfolio { time: None })
        .unwrap();

    let mut funds = portfolio
        .units
        .into_iter()
        .map(|(denom, unit)| coin((Uint128::new(1_000_000) * unit).u128(), denom))
        .collect::<Vec<_>>();
    funds.sort_by(|a, b| a.denom.cmp(&b.denom));

    wasm.execute(
        &env.core_addr,
        &core::ExecuteMsg::Mint {
            amount: Uint128::new(1_000_000),
            receiver: None,
            refund_to: None,
        },
        funds.as_slice(),
        acc,
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
        &env.core_addr,
        &core::ExecuteMsg::Burn { redeem_to: None },
        &[coin(1_000_000, &config.index_denom)],
        acc,
    )
    .unwrap();

    let balance = bank
        .query_balance(&QueryBalanceRequest {
            address: acc.address(),
            denom: config.index_denom.clone(),
        })
        .unwrap()
        .balance;
    assert_eq!(balance.unwrap().amount, "1500");

    // test estimation

    let estimate_in_resp = querier
        .estimate_swap_exact_amount_in(&EstimateSwapExactAmountInRequest {
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
            pool_id: uusd_pool,
            token_out: coin(estimate_in_resp.token_out_amount.parse().unwrap(), "uosmo")
                .to_string(),
            routes: vec![SwapAmountOutRoute {
                pool_id: uusd_pool,
                token_in_denom: uusd.clone(),
            }],
        })
        .unwrap();

    let estimate_multi_in_resp = querier
        .estimate_swap_exact_amount_in(&EstimateSwapExactAmountInRequest {
            pool_id: uatom_pool,
            token_in: coin(1_000_000, &uatom).to_string(),
            routes: vec![
                SwapAmountInRoute {
                    pool_id: uatom_pool,
                    token_out_denom: "uosmo".to_string(),
                },
                SwapAmountInRoute {
                    pool_id: uusd_pool,
                    token_out_denom: uusd.to_string(),
                },
            ],
        })
        .unwrap();
    println!(
        "{}",
        serde_json::to_string_pretty(&estimate_multi_in_resp).unwrap()
    );

    let estimate_multi_out_resp = querier
        .estimate_swap_exact_amount_out(&EstimateSwapExactAmountOutRequest {
            pool_id: uatom_pool,
            token_out: coin(
                estimate_multi_in_resp.token_out_amount.parse().unwrap(),
                &uatom,
            )
            .to_string(),
            routes: vec![
                SwapAmountOutRoute {
                    pool_id: uusd_pool,
                    token_in_denom: uusd.clone(),
                },
                SwapAmountOutRoute {
                    pool_id: uatom_pool,
                    token_in_denom: "uosmo".to_string(),
                },
            ],
        })
        .unwrap();
    println!(
        "{}",
        serde_json::to_string_pretty(&estimate_multi_out_resp).unwrap()
    );

    assert_eq!(estimate_out_resp.token_in_amount, format!("{}", 1_000_000));

    // mint / burn (periphery)
    let mint_burn_amount = 1_000_000_000;
    let mint_slippage: (u128, u128) = (10040, 10000); // 0.50
    let burn_slippage: (u128, u128) = (10050, 10000); // 0.50
    let pairs = [
        ("uosmo".to_string(), uatom_pool),
        (uusd, uusd_pool),
        (ujpy, ujpy_pool),
        (ukrw, ukrw_pool),
    ];

    let swap_info = (
        "uosmo",
        "single",
        periphery::SwapInfosCompact(
            pairs
                .iter()
                .map(|(denom, pool_id)| periphery::SwapInfoCompact {
                    key: format!("uosmo,{denom}"),
                    routes: vec![format!("{pool_id},uosmo")],
                })
                .collect::<Vec<_>>(),
        ),
    );

    let multihop_swap_info = (
        uatom.as_str(),
        "multihop",
        periphery::SwapInfosCompact(
            pairs
                .iter()
                .map(|(denom, pool_id)| {
                    if denom == "uosmo" {
                        periphery::SwapInfoCompact {
                            key: format!("{uatom},{denom}"),
                            routes: vec![format!("{uatom_pool},{uatom}")],
                        }
                    } else {
                        periphery::SwapInfoCompact {
                            key: format!("{uatom},{denom}"),
                            routes: vec![
                                format!("{uatom_pool},{uatom}"),
                                format!("{pool_id},uosmo"),
                            ],
                        }
                    }
                })
                .collect::<Vec<_>>(),
        ),
    );

    for (input, op_type, swap) in [swap_info, multihop_swap_info] {
        println!("============={op_type}=============");

        let sim_mint_resp: periphery::SimulateMintExactAmountOutResponse = wasm
            .query(
                &env.perp_addr,
                &periphery::QueryMsg::SimulateMintExactAmountOut {
                    core_addr: env.core_addr.clone(),
                    output_amount: Uint128::new(mint_burn_amount),
                    input_asset: input.to_string(),
                    swap_info: swap.clone(),
                },
            )
            .unwrap();
        println!("v1: {}", &sim_mint_resp.swap_result_amount);

        wasm.execute(
            &env.perp_addr,
            &periphery::ExecuteMsg::MintExactAmountOut {
                core_addr: env.core_addr.clone(),
                output_amount: Uint128::new(mint_burn_amount),
                input_asset: input.to_string(),
                swap_info: swap,
            },
            &[coin(
                sim_mint_resp.swap_result_amount.amount.u128() * mint_slippage.0 / mint_slippage.1,
                input,
            )],
            acc,
        )
        .unwrap();
    }

    let swap_info = (
        "uosmo",
        "single",
        periphery::SwapInfosCompact(
            pairs
                .iter()
                .map(|(denom, pool_id)| periphery::SwapInfoCompact {
                    key: format!("{denom},uosmo"),
                    routes: vec![format!("{pool_id},uosmo")],
                })
                .collect::<Vec<_>>(),
        ),
    );

    let multihop_swap_info = (
        uatom.as_str(),
        "multihop",
        periphery::SwapInfosCompact(
            pairs
                .iter()
                .map(|(denom, pool_id)| {
                    if denom == "uosmo" {
                        periphery::SwapInfoCompact {
                            key: format!("{denom},{uatom}"),
                            routes: vec![format!("{uatom_pool},{uatom}")],
                        }
                    } else {
                        periphery::SwapInfoCompact {
                            key: format!("{denom},{uatom}"),
                            routes: vec![
                                format!("{pool_id},uosmo"),
                                format!("{uatom_pool},{uatom}"),
                            ],
                        }
                    }
                })
                .collect::<Vec<_>>(),
        ),
    );

    for (output, op_type, swap) in [swap_info, multihop_swap_info] {
        println!("============={op_type}=============");

        let sim_burn_resp: periphery::SimulateBurnExactAmountInResponse = wasm
            .query(
                &env.perp_addr,
                &periphery::QueryMsg::SimulateBurnExactAmountIn {
                    core_addr: env.core_addr.clone(),
                    input_amount: Uint128::new(mint_burn_amount),
                    output_asset: output.to_string(),
                    swap_info: swap.clone(),
                },
            )
            .unwrap();

        println!("sim_result: {}", &sim_burn_resp.swap_result_amount);

        let act_burn_resp = wasm
            .execute(
                &env.perp_addr,
                &periphery::ExecuteMsg::BurnExactAmountIn {
                    core_addr: env.core_addr.clone(),
                    output_asset: output.to_string(),
                    min_output_amount: Decimal::from_ratio(
                        burn_slippage.1 - ((burn_slippage.0 - burn_slippage.1) * 2),
                        burn_slippage.1,
                    ) * sim_burn_resp.swap_result_amount.amount,
                    swap_info: swap,
                },
                &[coin(mint_burn_amount, &config.index_denom)],
                acc,
            )
            .unwrap();

        let act_burn_wasm_resp = act_burn_resp
            .events
            .into_iter()
            .filter(|v| v.ty == "wasm")
            .collect::<Vec<_>>();

        let finish_event = act_burn_wasm_resp
            .last()
            .unwrap()
            .attributes
            .iter()
            .find(|v| v.key == "amount")
            .unwrap();

        println!("act_result: {}", finish_event.value);
    }

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

    let uatom_balance = bank
        .query_balance(&QueryBalanceRequest {
            address: acc.address(),
            denom: uatom.to_string(),
        })
        .unwrap()
        .balance;

    println!("{index_balance:?}");
    println!("{uosmo_balance:?}");
    println!("{uatom_balance:?}");
}
