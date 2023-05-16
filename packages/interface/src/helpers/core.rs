use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_binary, Addr, Coin, CosmosMsg, CustomQuery, QuerierWrapper, StdResult, Uint128, WasmMsg,
    WasmQuery,
};

use crate::core::{
    ExecuteMsg, GetConfigResponse, GetFeeResponse, GetPortfolioResponse, QueryMsg,
    SimulateBurnResponse, SimulateMintResponse,
};

/// IbcCore is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[cw_serde]
pub struct IbcCore(pub Addr);

impl IbcCore {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<CM, T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg<CM>> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    pub fn call_with_funds<CM, T: Into<ExecuteMsg>>(
        &self,
        msg: T,
        funds: Vec<Coin>,
    ) -> StdResult<CosmosMsg<CM>> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds,
        }
        .into())
    }

    pub fn get_config<CQ>(
        &self,
        querier: &QuerierWrapper<CQ>,
        time: Option<u64>,
    ) -> StdResult<GetConfigResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::GetConfig { time };

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }

    pub fn get_fee<CQ>(
        &self,
        querier: &QuerierWrapper<CQ>,
        time: Option<u64>,
    ) -> StdResult<GetFeeResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::GetFee { time };

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }

    pub fn get_portfolio<CQ>(
        &self,
        querier: &QuerierWrapper<CQ>,
        time: Option<u64>,
    ) -> StdResult<GetPortfolioResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::GetPortfolio { time };

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }

    pub fn simulate_mint<CQ>(
        &self,
        querier: &QuerierWrapper<CQ>,
        amount: Uint128,
        funds: Option<Vec<Coin>>,
        time: Option<u64>,
    ) -> StdResult<SimulateMintResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::SimulateMint {
            amount,
            funds,
            time,
        };

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }

    pub fn simulate_burn<CQ>(
        &self,
        querier: &QuerierWrapper<CQ>,
        amount: Uint128,
        time: Option<u64>,
    ) -> StdResult<SimulateBurnResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::SimulateBurn { amount, time };

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }
}
