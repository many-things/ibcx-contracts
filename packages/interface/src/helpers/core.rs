use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr, Coin, CosmosMsg, StdResult, WasmMsg};

use crate::core::ExecuteMsg;

/// NoiCore is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[cw_serde]
pub struct NoiCore(pub Addr);

impl NoiCore {
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
}
