mod setup;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, from_binary, Binary, Uint128};
use ibcx_interface::periphery;
use osmosis_test_tube::{
    osmosis_std::types::osmosis::gamm::v1beta1::QueryPoolResponse, Gamm, Module, Wasm,
};
use serde_json::Value;
use setup::TestAsset;
use std::{collections::BTreeMap, str};

use crate::setup::{setup, NORM};

fn unwrap_asset(asset: Option<&TestAsset>) -> (String, u64) {
    let TestAsset { denom, pool_id } = asset.unwrap();
    (denom.clone(), *pool_id)
}

#[cw_serde]
pub struct CustomQueryPoolResponse {
    pub pool: BTreeMap<String, Value>,
}

#[test]
fn test_unmarshal() {
    let raw = Binary::from_base64("eyJwb29sIjp7IkB0eXBlIjoiL29zbW9zaXMuZ2FtbS5wb29sbW9kZWxzLnN0YWJsZXN3YXAudjFiZXRhMS5Qb29sIiwiYWRkcmVzcyI6Im9zbW8xcGprdDkzZzlsaG50Y3B4azZwbjA0eHdhODdnZjIzd3BqZ2hqdWRxbDVwN24yZXh1amg3c3pyZHZ0YyIsImlkIjoiNSIsInBvb2xfcGFyYW1zIjp7InN3YXBfZmVlIjoiMC4wMTAwMDAwMDAwMDAwMDAwMDAiLCJleGl0X2ZlZSI6IjAuMDAwMDAwMDAwMDAwMDAwMDAwIn0sImZ1dHVyZV9wb29sX2dvdmVybm9yIjoib3NtbzF1ZHkyeXZ6cmtrdHdnNHI5dG40cDgybmplZmh5cGZsNWZjMDg4biIsInRvdGFsX3NoYXJlcyI6eyJkZW5vbSI6ImdhbW0vcG9vbC81IiwiYW1vdW50IjoiMTAwMDAwMDAwMDAwMDAwMDAwMDAwIn0sInBvb2xfbGlxdWlkaXR5IjpbeyJkZW5vbSI6ImZhY3Rvcnkvb3NtbzF1ZHkyeXZ6cmtrdHdnNHI5dG40cDgybmplZmh5cGZsNWZjMDg4bi91anB5IiwiYW1vdW50IjoiNDA2NTYwMDAwMDAwMDAwMDAwIn0seyJkZW5vbSI6ImZhY3Rvcnkvb3NtbzF1ZHkyeXZ6cmtrdHdnNHI5dG40cDgybmplZmh5cGZsNWZjMDg4bi91a3J3IiwiYW1vdW50IjoiMzk2OTgwMDAwMDAwMDAwMDAwMCJ9LHsiZGVub20iOiJmYWN0b3J5L29zbW8xdWR5Mnl2enJra3R3ZzRyOXRuNHA4Mm5qZWZoeXBmbDVmYzA4OG4vdXVzZCIsImFtb3VudCI6IjI5NjAwMDAwMDAwMDAwMDAwMCJ9XSwic2NhbGluZ19mYWN0b3JzIjpbIjEiLCIxIiwiMSJdLCJzY2FsaW5nX2ZhY3Rvcl9jb250cm9sbGVyIjoib3NtbzF1ZHkyeXZ6cmtrdHdnNHI5dG40cDgybmplZmh5cGZsNWZjMDg4biJ9fQ==").unwrap();
    let res: CustomQueryPoolResponse =
        serde_json::from_str(str::from_utf8(&raw.0).unwrap()).unwrap();
    println!("{:?}", res.pool);
}

#[test]
fn test_approx() {
    let env = setup(&[coin(10 * NORM, "uosmo")], 1);
    let owner = env.accs.first().unwrap();

    let gamm = Gamm::new(&env.app);
    let wasm = Wasm::new(&env.app);

    let (uusd, uusd_pool) = unwrap_asset(env.assets.get("uusd"));
    let (ujpy, ujpy_pool) = unwrap_asset(env.assets.get("ujpy"));
    let (ukrw, _ukrw_pool) = unwrap_asset(env.assets.get("ukrw"));
    let (uatom, uatom_pool) = unwrap_asset(env.assets.get("uatom"));

    let raw = Binary::from_base64("eyJwb29sIjp7IkB0eXBlIjoiL29zbW9zaXMuZ2FtbS5wb29sbW9kZWxzLnN0YWJsZXN3YXAudjFiZXRhMS5Qb29sIiwiYWRkcmVzcyI6Im9zbW8xcGprdDkzZzlsaG50Y3B4azZwbjA0eHdhODdnZjIzd3BqZ2hqdWRxbDVwN24yZXh1amg3c3pyZHZ0YyIsImlkIjoiNSIsInBvb2xfcGFyYW1zIjp7InN3YXBfZmVlIjoiMC4wMTAwMDAwMDAwMDAwMDAwMDAiLCJleGl0X2ZlZSI6IjAuMDAwMDAwMDAwMDAwMDAwMDAwIn0sImZ1dHVyZV9wb29sX2dvdmVybm9yIjoib3NtbzFtN2VrN3Y5ajBtam12cHVtZ3d1ZmxtMjdqZ2RxczVya2pxa241eSIsInRvdGFsX3NoYXJlcyI6eyJkZW5vbSI6ImdhbW0vcG9vbC81IiwiYW1vdW50IjoiMTAwMDAwMDAwMDAwMDAwMDAwMDAwIn0sInBvb2xfbGlxdWlkaXR5IjpbeyJkZW5vbSI6ImZhY3Rvcnkvb3NtbzFtN2VrN3Y5ajBtam12cHVtZ3d1ZmxtMjdqZ2RxczVya2pxa241eS91anB5IiwiYW1vdW50IjoiNDA2NTYwMDAwMDAwMDAwMDAwIn0seyJkZW5vbSI6ImZhY3Rvcnkvb3NtbzFtN2VrN3Y5ajBtam12cHVtZ3d1ZmxtMjdqZ2RxczVya2pxa241eS91a3J3IiwiYW1vdW50IjoiMzk2OTgwMDAwMDAwMDAwMDAwMCJ9LHsiZGVub20iOiJmYWN0b3J5L29zbW8xbTdlazd2OWowbWptdnB1bWd3dWZsbTI3amdkcXM1cmtqcWtuNXkvdXVzZCIsImFtb3VudCI6IjI5NjAwMDAwMDAwMDAwMDAwMCJ9XSwic2NhbGluZ19mYWN0b3JzIjpbMSwxLDFdLCJzY2FsaW5nX2ZhY3Rvcl9jb250cm9sbGVyIjoib3NtbzFtN2VrN3Y5ajBtam12cHVtZ3d1ZmxtMjdqZ2RxczVya2pxa241eSJ9fQ==").unwrap();
    let resp: QueryPoolResponse = from_binary(&raw).unwrap();
    println!("pool => {:?}", resp);

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
                            format!("{ujpy_pool},{ukrw}"),
                            format!("{},uosmo", env.stable_pool),
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
