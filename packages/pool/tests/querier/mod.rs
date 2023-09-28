mod bank;
mod wasm;

use cosmwasm_std::{
    from_slice, ContractResult, Empty, Querier, QuerierResult, QueryRequest, SystemError,
    SystemResult,
};

use osmosis_test_tube::{Module, OsmosisTestApp, Runner};

use self::{bank::BankHandler, wasm::WasmHandler};

pub struct TestTubeQuerier<'a> {
    pub app: &'a OsmosisTestApp,
    pub bank: BankHandler<'a, OsmosisTestApp>,
    pub wasm: WasmHandler<'a, OsmosisTestApp>,
}

impl<'a> TestTubeQuerier<'a> {
    pub fn new(app: &'a OsmosisTestApp) -> Self {
        Self {
            app,
            bank: BankHandler::new(app),
            wasm: WasmHandler::new(app),
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

        match request {
            QueryRequest::Bank(query) => self.bank.handle(query),
            QueryRequest::Wasm(query) => self.wasm.handle(query),
            QueryRequest::Stargate { path, data } => {
                match self.app.raw_query(&path, data.to_vec()).map_err(|e| {
                    SystemError::InvalidRequest {
                        error: e.to_string(),
                        request: data,
                    }
                }) {
                    Ok(res) => SystemResult::Ok(ContractResult::Ok(res.into())),
                    Err(err) => SystemResult::Err(err),
                }
            }

            _ => SystemResult::Err(SystemError::UnsupportedRequest {
                kind: format!("{:?}", request),
            }),
        }
    }
}
