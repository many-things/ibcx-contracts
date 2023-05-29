mod setup;

use cosmwasm_std::{coin, Uint128};
use ibcx_interface::periphery;
use osmosis_test_tube::{Module, Wasm};
use setup::TestAsset;

use crate::setup::{setup, NORM};

fn unwrap_asset(asset: Option<&TestAsset>) -> (String, u64) {
    let TestAsset { denom, pool_id } = asset.unwrap();
    (denom.clone(), *pool_id)
}

#[test]
fn test_approx() {
    let env = setup(&[coin(10 * NORM, "uosmo")], 1);
    let owner = env.accs.first().unwrap();

    let wasm = Wasm::new(&env.app);

    let (uusd, uusd_pool) = unwrap_asset(env.assets.get("uusd"));
    let (ujpy, ujpy_pool) = unwrap_asset(env.assets.get("ujpy"));
    let (_ukrw, _ukrw_pool) = unwrap_asset(env.assets.get("ukrw"));
    let (uatom, uatom_pool) = unwrap_asset(env.assets.get("uatom"));

    let resp = wasm
        .execute(
            &env.perp_addr,
            &periphery::ExecuteMsg::MintExactAmountIn {
                core_addr: env.core_addr.clone(),
                input_asset: uatom.clone(),
                min_output_amount: Uint128::zero(),
                swap_info: periphery::SwapInfosCompact(vec![
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
                ]),
            },
            &[],
            owner,
        )
        .unwrap();

    println!("gas used => {:?}", resp.gas_info.gas_used);
}
