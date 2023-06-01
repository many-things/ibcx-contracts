mod setup;

use std::str::from_utf8;

use cosmwasm_std::{coin, Binary, Uint128};
use ibcx_interface::periphery;
use ibcx_periphery::pool::{StablePoolResponse, WeightedPoolResponse};
use osmosis_test_tube::{Module, Wasm};

use crate::setup::{setup, TestAsset, NORM};

fn unwrap_asset(asset: Option<&TestAsset>) -> (String, u64) {
    let TestAsset { denom, pool_id } = asset.unwrap();
    (denom.clone(), *pool_id)
}

#[test]
fn test_unmarshal() {
    let raw_weigthed_pool_enc = "eyJwb29sIjp7IkB0eXBlIjoiL29zbW9zaXMuZ2FtbS52MWJldGExLlBvb2wiLCJhZGRyZXNzIjoib3NtbzFhZDRyM3VoNXBkbjVwZ2c1aG5sNnU1dXRmZXFtcHdzdGx2Z3ZnMmgyamR6dHJjbndrcWdzM2hzODV6IiwiaWQiOiI0IiwicG9vbF9wYXJhbXMiOnsic3dhcF9mZWUiOiIwLjAxMDAwMDAwMDAwMDAwMDAwMCIsImV4aXRfZmVlIjoiMC4wMDAwMDAwMDAwMDAwMDAwMDAiLCJzbW9vdGhfd2VpZ2h0X2NoYW5nZV9wYXJhbXMiOm51bGx9LCJmdXR1cmVfcG9vbF9nb3Zlcm5vciI6IiIsInRvdGFsX3NoYXJlcyI6eyJkZW5vbSI6ImdhbW0vcG9vbC80IiwiYW1vdW50IjoiMTAwMDAwMDAwMDAwMDAwMDAwMDAwIn0sInBvb2xfYXNzZXRzIjpbeyJ0b2tlbiI6eyJkZW5vbSI6ImZhY3Rvcnkvb3NtbzFneHlndzVneTh5aHl1dTA1cWE5Zm1nYWR5eWFuZTg3cHJ3cDY1Zy91YXRvbSIsImFtb3VudCI6IjIzMDQ4ODAwMDAwMDAifSwid2VpZ2h0IjoiMTA3Mzc0MTgyNDAwMDAwMCJ9LHsidG9rZW4iOnsiZGVub20iOiJ1b3NtbyIsImFtb3VudCI6IjQwMDAwMDAwMDAwMDAwIn0sIndlaWdodCI6IjEwNzM3NDE4MjQwMDAwMDAifV0sInRvdGFsX3dlaWdodCI6IjIxNDc0ODM2NDgwMDAwMDAifX0=";
    let raw_weigthed_pool_bin = Binary::from_base64(raw_weigthed_pool_enc).unwrap();
    let raw_weighted_pool = from_utf8(&raw_weigthed_pool_bin.0).unwrap();

    let raw_stable_pool_enc = "eyJwb29sIjp7IkB0eXBlIjoiL29zbW9zaXMuZ2FtbS5wb29sbW9kZWxzLnN0YWJsZXN3YXAudjFiZXRhMS5Qb29sIiwiYWRkcmVzcyI6Im9zbW8xcGprdDkzZzlsaG50Y3B4azZwbjA0eHdhODdnZjIzd3BqZ2hqdWRxbDVwN24yZXh1amg3c3pyZHZ0YyIsImlkIjoiNSIsInBvb2xfcGFyYW1zIjp7InN3YXBfZmVlIjoiMC4wMTAwMDAwMDAwMDAwMDAwMDAiLCJleGl0X2ZlZSI6IjAuMDAwMDAwMDAwMDAwMDAwMDAwIn0sImZ1dHVyZV9wb29sX2dvdmVybm9yIjoib3NtbzFneHlndzVneTh5aHl1dTA1cWE5Zm1nYWR5eWFuZTg3cHJ3cDY1ZyIsInRvdGFsX3NoYXJlcyI6eyJkZW5vbSI6ImdhbW0vcG9vbC81IiwiYW1vdW50IjoiMTAwMDAwMDAwMDAwMDAwMDAwMDAwIn0sInBvb2xfbGlxdWlkaXR5IjpbeyJkZW5vbSI6ImZhY3Rvcnkvb3NtbzFneHlndzVneTh5aHl1dTA1cWE5Zm1nYWR5eWFuZTg3cHJ3cDY1Zy91anB5IiwiYW1vdW50IjoiNDA2NTYwMDAwMDAwMDAwMDAwIn0seyJkZW5vbSI6ImZhY3Rvcnkvb3NtbzFneHlndzVneTh5aHl1dTA1cWE5Zm1nYWR5eWFuZTg3cHJ3cDY1Zy91a3J3IiwiYW1vdW50IjoiMzk2OTgwMDAwMDAwMDAwMDAwMCJ9LHsiZGVub20iOiJmYWN0b3J5L29zbW8xZ3h5Z3c1Z3k4eWh5dXUwNXFhOWZtZ2FkeXlhbmU4N3Byd3A2NWcvdXVzZCIsImFtb3VudCI6IjI5NjAwMDAwMDAwMDAwMDAwMCJ9XSwic2NhbGluZ19mYWN0b3JzIjpbIjEiLCIxIiwiMSJdLCJzY2FsaW5nX2ZhY3Rvcl9jb250cm9sbGVyIjoib3NtbzFneHlndzVneTh5aHl1dTA1cWE5Zm1nYWR5eWFuZTg3cHJ3cDY1ZyJ9fQ==";
    let raw_stabel_pool_bin = Binary::from_base64(raw_stable_pool_enc).unwrap();
    let raw_stable_pool = from_utf8(&raw_stabel_pool_bin.0).unwrap();

    let weighted_pool: WeightedPoolResponse = serde_json_wasm::from_str(raw_weighted_pool).unwrap();
    println!("weighted_pool => {:?}", weighted_pool);

    let stable_pool: StablePoolResponse = serde_json::from_str(raw_stable_pool).unwrap();
    println!("stable_pool => {:?}", stable_pool);
}

#[test]
fn test_approx() {
    let env = setup(&[coin(10 * NORM, "uosmo")], 1);
    let owner = env.accs.first().unwrap();

    let wasm = Wasm::new(&env.app);

    let (uusd, uusd_pool) = unwrap_asset(env.assets.get("uusd"));
    let (ujpy, ujpy_pool) = unwrap_asset(env.assets.get("ujpy"));
    let (ukrw, ukrw_pool) = unwrap_asset(env.assets.get("ukrw"));
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
                    periphery::SwapInfoCompact {
                        key: format!("{uatom},{ukrw}"),
                        routes: vec![
                            format!("{uatom_pool},{uatom}"),
                            format!("{ukrw_pool},uosmo"),
                        ],
                    },
                    periphery::SwapInfoCompact {
                        key: format!("{uatom},uosmo"),
                        routes: vec![
                            format!("{uatom_pool},{uatom}"),
                            format!("{ujpy_pool},{ukrw}"),
                            format!("{},uosmo", env.stable_pool),
                        ],
                    },
                ]),
            },
            &[coin(10 * NORM, uatom)],
            owner,
        )
        .unwrap();

    println!("gas used => {:?}", resp.gas_info.gas_used);
}
