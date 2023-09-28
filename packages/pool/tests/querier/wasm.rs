use cosmwasm_std::{ContractResult, QuerierResult, SystemError, SystemResult, WasmQuery};
use osmosis_std::types::cosmwasm::wasm::v1::{
    QueryRawContractStateRequest, QueryRawContractStateResponse, QuerySmartContractStateRequest,
    QuerySmartContractStateResponse,
};
use osmosis_test_tube::{fn_query, Module, Runner};

pub struct WasmHandler<'a, R: Runner<'a>> {
    pub runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for WasmHandler<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R: Runner<'a>> WasmHandler<'a, R> {
    fn_query! {
        pub query_wasm_smart [QuerySmartContractStateRequest::TYPE_URL]: QuerySmartContractStateRequest => QuerySmartContractStateResponse
    }

    fn_query! {
        pub query_wasm_raw [QueryRawContractStateRequest::TYPE_URL]: QueryRawContractStateRequest => QueryRawContractStateResponse
    }

    pub fn handle(&self, query: WasmQuery) -> QuerierResult {
        match query {
            WasmQuery::Smart { contract_addr, msg } => match self
                .query_wasm_smart(&QuerySmartContractStateRequest {
                    address: contract_addr,
                    query_data: msg.to_vec(),
                })
                .map(|v| v.data)
                .map_err(|e| SystemError::InvalidRequest {
                    error: e.to_string(),
                    request: msg,
                }) {
                Ok(res) => SystemResult::Ok(ContractResult::Ok(res.into())),
                Err(err) => SystemResult::Err(err),
            },

            WasmQuery::Raw { contract_addr, key } => {
                match self
                    .query_wasm_raw(&QueryRawContractStateRequest {
                        address: contract_addr,
                        query_data: key.to_vec(),
                    })
                    .map(|v| v.data)
                    .map_err(|e| SystemError::InvalidRequest {
                        error: e.to_string(),
                        request: key,
                    }) {
                    Ok(res) => SystemResult::Ok(ContractResult::Ok(res.into())),
                    Err(err) => SystemResult::Err(err),
                }
            }

            _ => SystemResult::Err(SystemError::UnsupportedRequest {
                kind: format!("{:?}", query),
            }),
        }
    }
}
