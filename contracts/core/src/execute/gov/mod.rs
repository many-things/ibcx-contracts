mod pause;
mod update;

use cosmwasm_std::{DepsMut, Response};
use cosmwasm_std::{Env, MessageInfo};
use ibcx_interface::core::GovMsg;

use crate::StdResult;

pub fn handle_msg(deps: DepsMut, env: Env, info: MessageInfo, msg: GovMsg) -> StdResult<Response> {
    use GovMsg::*;

    match msg {
        Pause { expires_at } => pause::pause(deps, env, info, expires_at),
        Release {} => pause::release(deps, env, info),

        UpdateGov(new_gov) => update::update_gov(deps, info, new_gov),
        AcceptGov {} => update::accept_gov(deps, info),
        RevokeGov {} => update::revoke_gov(deps, info),
        UpdateFeeStrategy(new_fee) => update::update_fee(deps, env, info, new_fee),
        UpdateRebalanceManager(new_manager) => {
            update::update_rebalance_manager(deps, info, new_manager)
        }
        UpdateReserveDenom(new_denom) => update::update_reserve_denom(deps, info, new_denom),
        UpdateTradeInfo {
            denom,
            routes,
            cooldown,
            max_trade_amount,
        } => update::update_trade_info(deps, info, denom, routes, cooldown, max_trade_amount),
    }
}
