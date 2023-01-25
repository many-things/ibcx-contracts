use std::collections::BTreeMap;

use cosmwasm_std::{Binary, ContractResult, QuerierResult, SystemResult};

type StargateHandler<'a> = Box<dyn Fn(&Binary) -> QuerierResult + 'a>;

#[derive(Default)]
pub struct StargateQuerier<'a> {
    pub handlers: BTreeMap<String, StargateHandler<'a>>,
}

impl<'a> StargateQuerier<'a> {
    pub fn register<F: Fn(&Binary) -> ContractResult<Binary> + 'a>(&mut self, k: &str, f: F) {
        self.handlers.insert(
            k.to_string(),
            Box::new(move |data| {
                let res = f(data);

                SystemResult::Ok(res)
            }),
        );
    }

    pub fn query(&self, path: &String, data: &Binary) -> QuerierResult {
        self.handlers.get(path).unwrap()(data)
    }
}
