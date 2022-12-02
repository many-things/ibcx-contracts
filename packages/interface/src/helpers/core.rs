use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_binary, Addr, Coin, CosmosMsg, CustomQuery, QuerierWrapper, StdResult, WasmMsg, WasmQuery,
};

use crate::{
    core::{ConfigResponse, ExecuteMsg, PauseInfoResponse, PortfolioResponse, QueryMsg},
    types::RangeOrder,
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

    pub fn get_config<CQ>(&self, querier: &QuerierWrapper<CQ>) -> StdResult<ConfigResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::Config {};

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }

    pub fn get_pause_info<CQ>(&self, querier: &QuerierWrapper<CQ>) -> StdResult<PauseInfoResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::PauseInfo {};

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }

    pub fn get_portfolio<CQ>(&self, querier: &QuerierWrapper<CQ>) -> StdResult<PortfolioResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::Portfolio {};

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }
}
