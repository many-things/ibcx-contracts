use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use ibc_interface::core::RebalanceMsg;

use crate::error::ContractError;

pub fn handle_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: RebalanceMsg,
) -> Result<Response, ContractError> {
    use RebalanceMsg::*;

    match msg {
        Init {
            manager,
            deflation,
            inflation,
        } => todo!(),
        Trade { denom, amount } => todo!(),
        Finalize {} => todo!(),
    }
}
