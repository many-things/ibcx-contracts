use cosmwasm_std::{
    from_slice, to_binary, ContractResult, Empty, QueryRequest, SystemError, SystemResult,
};
use osmosis_std::types::cosmwasm::wasm::v1::{
    QuerySmartContractStateRequest, QuerySmartContractStateResponse,
};
use osmosis_test_tube::Runner;

use crate::App;

pub struct Querier<'a> {
    app: &'a App,
}

impl<'a> Querier<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl cosmwasm_std::Querier for Querier<'_> {
    fn raw_query(&self, bin_request: &[u8]) -> cosmwasm_std::QuerierResult {
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
            .inner()
            .query::<QuerySmartContractStateRequest, QuerySmartContractStateResponse>(
                "/cosmwasm.wasm.v1.Query/SmartContractState",
                &QuerySmartContractStateRequest {
                    address: self.app.wasm_querier().to_string(),
                    query_data: to_binary(&request).unwrap().to_vec(),
                },
            )
            .unwrap();

        SystemResult::Ok(ContractResult::Ok(res.data.into()))
    }
}
