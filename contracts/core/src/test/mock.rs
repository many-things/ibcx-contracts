use std::{collections::BTreeMap, str::FromStr};

use cosmwasm_std::{
    to_json_binary, Binary, ContractResult, Decimal, QuerierResult, SystemResult, Uint128,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::{
    EstimateSwapExactAmountInRequest, EstimateSwapExactAmountInResponse,
    EstimateSwapExactAmountOutRequest, EstimateSwapExactAmountOutResponse,
};

type StargateHandler<'a> = Box<dyn Fn(&Binary) -> QuerierResult + 'a>;

#[derive(Default)]
pub struct StargateQuerier<'a> {
    pub handlers: BTreeMap<String, StargateHandler<'a>>,
}

impl<'a> StargateQuerier<'a> {
    pub fn register_raw<F: Fn(&Binary) -> ContractResult<Binary> + 'a>(&mut self, k: &str, f: F) {
        self.handlers.insert(
            k.to_string(),
            Box::new(move |data| {
                let res = f(data);

                SystemResult::Ok(res)
            }),
        );
    }

    pub fn register_sim_swap_exact_out(&mut self, price: &'a str) {
        self.register_raw(
            "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountOut",
            move |req| {
                let req: EstimateSwapExactAmountOutRequest = req.clone().try_into().unwrap();

                let position = req
                    .token_out
                    .find(|c: char| !c.is_ascii_digit())
                    .expect("did not find a split position");
                let (amount, _) = req.token_out.split_at(position);

                let price = Decimal::one() / Decimal::from_str(price).unwrap();
                let token_out_amount = Uint128::from_str(amount).unwrap();

                to_json_binary(&EstimateSwapExactAmountOutResponse {
                    token_in_amount: (price * token_out_amount).to_string(),
                })
                .into()
            },
        );
    }

    pub fn register_sim_swap_exact_in(&mut self, price: &'a str) {
        self.register_raw(
            "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountIn",
            move |req| {
                let req: EstimateSwapExactAmountInRequest = req.clone().try_into().unwrap();

                let position = req
                    .token_in
                    .find(|c: char| !c.is_ascii_digit())
                    .expect("did not find a split position");
                let (amount, _) = req.token_in.split_at(position);

                let price = Decimal::from_str(price).unwrap();
                let token_in_amount = Uint128::from_str(amount).unwrap();

                to_json_binary(&EstimateSwapExactAmountInResponse {
                    token_out_amount: (price * token_in_amount).to_string(),
                })
                .into()
            },
        );
    }

    pub fn query(&self, path: &String, data: &Binary) -> QuerierResult {
        self.handlers.get(path).unwrap()(data)
    }
}
