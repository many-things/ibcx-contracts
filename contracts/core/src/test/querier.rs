use cosmwasm_schema::serde::de::DeserializeOwned;
use cosmwasm_std::{
    from_json, testing::MockQuerier, CustomQuery, Empty, Querier, QuerierResult, QueryRequest,
    SystemError, SystemResult,
};

use super::mock::StargateQuerier;

pub struct CoreQuerier<'a, C: DeserializeOwned = Empty> {
    pub mq: MockQuerier<C>,
    pub stargate: StargateQuerier<'a>,
}

impl<C: CustomQuery + DeserializeOwned> Querier for CoreQuerier<'_, C> {
    fn raw_query(&self, bin_request: &[u8]) -> cosmwasm_std::QuerierResult {
        let request: QueryRequest<C> = match from_json(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {e}"),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}

impl<C: CustomQuery + DeserializeOwned> CoreQuerier<'_, C> {
    pub fn handle_query(&self, request: &QueryRequest<C>) -> QuerierResult {
        match &request {
            QueryRequest::Bank(_) => self.mq.handle_query(request),
            QueryRequest::Custom(_) => self.mq.handle_query(request),
            QueryRequest::Staking(_) => self.mq.handle_query(request),
            QueryRequest::Wasm(_) => self.mq.handle_query(request),
            QueryRequest::Stargate { path, data } => self.stargate.query(path, data),
            QueryRequest::Ibc(_) => todo!(),
            _ => SystemResult::Err(SystemError::UnsupportedRequest {
                kind: "unknown".to_string(),
            }),
        }
    }
}
