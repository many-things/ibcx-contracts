mod bank;
mod wasm;

use std::{env, fs, path::PathBuf};

use cosmwasm_std::{
    coin, from_slice, to_binary, ContractResult, Empty, Querier, QuerierResult, QueryRequest,
    SystemError, SystemResult,
};

use osmosis_std::types::cosmwasm::wasm::v1::{
    QuerySmartContractStateRequest, QuerySmartContractStateResponse,
};
use osmosis_test_tube::{Module, OsmosisTestApp, Runner, Wasm};

pub struct TestTubeQuerier<'a> {
    pub app: &'a OsmosisTestApp,
    pub wasm_querier: String,
}

impl<'a> TestTubeQuerier<'a> {
    pub fn new(app: &'a OsmosisTestApp, querier_code_path: Option<PathBuf>) -> Self {
        let deployer = app.init_account(&[coin(100_000_000_000, "uosmo")]).unwrap();

        let wasm = Wasm::new(app);

        let store_resp = wasm
            .store_code(
                &fs::read(
                    querier_code_path
                        .unwrap_or("../../artifacts/ibcx_test_querier-aarch64.wasm".into()),
                )
                .unwrap(),
                None,
                &deployer,
            )
            .unwrap();

        let init_resp = wasm
            .instantiate(
                store_resp.data.code_id,
                &Empty {},
                None,
                None,
                &[],
                &deployer,
            )
            .unwrap();

        Self {
            app,
            wasm_querier: init_resp.data.address,
        }
    }
}

impl Querier for TestTubeQuerier<'_> {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<Empty> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {e}"),
                    request: bin_request.into(),
                })
            }
        };

        let res = self
            .app
            .query::<QuerySmartContractStateRequest, QuerySmartContractStateResponse>(
                "/cosmwasm.wasm.v1.Query/SmartContractState",
                &QuerySmartContractStateRequest {
                    address: self.wasm_querier.to_owned(),
                    query_data: to_binary(&request).unwrap().to_vec(),
                },
            )
            .unwrap();

        SystemResult::Ok(ContractResult::Ok(res.data.into()))
    }
}
