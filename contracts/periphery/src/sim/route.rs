use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, CosmosMsg, Decimal, Uint128};
use ibcx_interface::types::SwapRoutes;

use crate::error::ContractError;

#[cw_serde]
pub struct SimAmountOutRoute {
    pub amount_in: Coin,
    pub sim_amount_out: Uint128,
    pub routes: Option<SwapRoutes>,
}

#[cw_serde]
pub struct SimAmountOutRoutes(pub Vec<SimAmountOutRoute>);

impl IntoIterator for SimAmountOutRoutes {
    type Item = SimAmountOutRoute;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<Vec<SimAmountOutRoute>> for SimAmountOutRoutes {
    fn from(v: Vec<SimAmountOutRoute>) -> Self {
        Self(v)
    }
}

impl SimAmountOutRoutes {
    pub fn to_msgs(
        &self,
        contract: &Addr,
        min_output: Uint128,
    ) -> Result<Vec<CosmosMsg>, ContractError> {
        let total_receive_amount = self
            .0
            .iter()
            .fold(Uint128::zero(), |acc, v| acc + v.sim_amount_out);
        if total_receive_amount < min_output {
            return Err(ContractError::TradeAmountExceeded {});
        }

        let amplifier = Decimal::checked_from_ratio(min_output, total_receive_amount)?;

        let swap_msgs = self
            .0
            .iter()
            .filter_map(|r| {
                r.routes.as_ref().map(|routes| {
                    let min_amount_out = r.sim_amount_out * amplifier;
                    routes.msg_swap_exact_in(
                        contract,
                        &r.amount_in.denom,
                        r.amount_in.amount,
                        min_amount_out,
                    )
                })
            })
            .collect::<Vec<_>>();

        Ok(swap_msgs)
    }
}

#[cw_serde]
pub struct SimAmountInRoute {
    pub sim_amount_in: Uint128,
    pub amount_out: Coin,
    pub routes: Option<SwapRoutes>,
}

#[cw_serde]
pub struct SimAmountInRoutes(pub Vec<SimAmountInRoute>);

impl IntoIterator for SimAmountInRoutes {
    type Item = SimAmountInRoute;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<Vec<SimAmountInRoute>> for SimAmountInRoutes {
    fn from(v: Vec<SimAmountInRoute>) -> Self {
        Self(v)
    }
}

impl SimAmountInRoutes {
    pub fn to_msgs(
        &self,
        contract: &Addr,
        max_input: Uint128,
    ) -> Result<Vec<CosmosMsg>, ContractError> {
        let total_spend_amount = self
            .0
            .iter()
            .fold(Uint128::zero(), |acc, v| acc + v.sim_amount_in);
        if max_input < total_spend_amount {
            return Err(ContractError::TradeAmountExceeded {});
        }

        let amplifier = Decimal::checked_from_ratio(max_input, total_spend_amount)?;

        let swap_msgs = self
            .0
            .iter()
            .filter_map(|r| {
                r.routes.as_ref().map(|routes| {
                    let max_amount_in = r.sim_amount_in * amplifier;
                    let mut routes = routes.clone();
                    routes.0.reverse();
                    routes.msg_swap_exact_out(
                        contract,
                        &r.amount_out.denom,
                        r.amount_out.amount,
                        max_amount_in,
                    )
                })
            })
            .collect::<Vec<_>>();

        Ok(swap_msgs)
    }
}
