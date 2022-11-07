use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_binary, Addr, Coin, CosmosMsg, CustomQuery, QuerierWrapper, StdResult, WasmMsg, WasmQuery,
};

use crate::{
    core::{
        AllocationResponse, ConfigResponse, ExecuteMsg, ListAllocationResponse,
        ListRebalanceInfoResponse, ListStrategyResponse, PauseInfoResponse, PortfolioResponse,
        QueryMsg, RebalanceInfoResponse, StrategyResponse,
    },
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

    pub fn get_rebalance_info<CQ>(
        &self,
        querier: &QuerierWrapper<CQ>,
        id: Option<u64>,
    ) -> StdResult<RebalanceInfoResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::RebalanceInfo { id };

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }

    pub fn list_rebalance_info<CQ>(
        &self,
        querier: &QuerierWrapper<CQ>,
        start_after: Option<u64>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    ) -> StdResult<ListRebalanceInfoResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::ListRebalanceInfo {
            start_after,
            limit,
            order,
        };

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }

    pub fn get_strategy<CQ>(
        &self,
        querier: &QuerierWrapper<CQ>,
        asset: String,
    ) -> StdResult<StrategyResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::Strategy { asset };

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }

    pub fn list_strategy<CQ>(
        &self,
        querier: &QuerierWrapper<CQ>,
        start_after: Option<String>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    ) -> StdResult<ListStrategyResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::ListStrategy {
            start_after,
            limit,
            order,
        };

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }

    pub fn get_allocation<CQ>(
        &self,
        querier: &QuerierWrapper<CQ>,
        asset: String,
    ) -> StdResult<AllocationResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::Allocation { asset };

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }

    pub fn list_allocation<CQ>(
        &self,
        querier: &QuerierWrapper<CQ>,
        start_after: Option<String>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    ) -> StdResult<ListAllocationResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::ListAllocation {
            start_after,
            limit,
            order,
        };

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }
}
