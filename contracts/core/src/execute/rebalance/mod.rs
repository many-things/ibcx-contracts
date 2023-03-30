mod finalize;
mod init;
mod trade;

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use ibcx_interface::core::RebalanceMsg;

use crate::StdResult;

use finalize::finalize;
use init::init;
use trade::trade;

pub fn handle_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: RebalanceMsg,
) -> StdResult<Response> {
    use RebalanceMsg::*;

    match msg {
        Init {
            manager,
            deflation,
            inflation,
        } => init(deps, info, manager, deflation, inflation),
        Trade(msg) => trade(deps, env, info, msg),
        Finalize {} => finalize(deps, env, info),
    }
}
