use cosmwasm_std::{
    coin, to_binary, AllBalanceResponse, BalanceResponse, BankQuery, QuerierResult, SystemError,
    SystemResult,
};

use osmosis_std::types::cosmos::{
    bank::v1beta1::{QueryAllBalancesRequest, QueryBalanceRequest},
    base::v1beta1::Coin as ProtoCoin,
};
use osmosis_test_tube::{Bank, Module, Runner};

pub struct BankHandler<'a, R: Runner<'a>> {
    pub bank: Bank<'a, R>,
}

impl<'a, R: Runner<'a>> BankHandler<'a, R> {
    pub fn new(runner: &'a R) -> Self {
        Self {
            bank: Bank::new(runner),
        }
    }

    pub fn handle(&self, query: BankQuery) -> QuerierResult {
        match query {
            BankQuery::Balance { address, denom } => {
                let resp = self
                    .bank
                    .query_balance(&QueryBalanceRequest { address, denom })
                    .unwrap();
                let ProtoCoin { amount, denom } = resp.balance.unwrap();

                SystemResult::Ok(
                    to_binary(&BalanceResponse {
                        amount: coin(amount.parse().unwrap(), denom),
                    })
                    .into(),
                )
            }
            BankQuery::AllBalances { address } => {
                let resp = self
                    .bank
                    .query_all_balances(&QueryAllBalancesRequest {
                        address,
                        pagination: None,
                    })
                    .unwrap();

                let balances = resp
                    .balances
                    .into_iter()
                    .map(|ProtoCoin { amount, denom }| coin(amount.parse().unwrap(), denom))
                    .collect::<Vec<_>>();

                SystemResult::Ok(to_binary(&AllBalanceResponse { amount: balances }).into())
            }
            _ => SystemResult::Err(SystemError::UnsupportedRequest {
                kind: format!("{:?}", query),
            }),
        }
    }
}
