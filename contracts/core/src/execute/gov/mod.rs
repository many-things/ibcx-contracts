mod pause;
mod update;

use cosmwasm_std::{DepsMut, Response};
use cosmwasm_std::{Env, MessageInfo};
use ibcx_interface::core::GovMsg;

use crate::error::ContractError;

pub fn handle_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: GovMsg,
) -> Result<Response, ContractError> {
    use GovMsg::*;

    match msg {
        Pause { expires_at } => pause::pause(deps, env, info, expires_at),
        Release {} => pause::release(deps, env, info),

        UpdateGov(new_gov) => update::update_gov(deps, info, new_gov),
        UpdateFeeStrategy(new_fee) => update::update_fee(deps, env, info, new_fee),
        UpdateReserveDenom(new_denom) => update::update_reserve_denom(deps, info, new_denom),
        UpdateTradeInfo {
            denom,
            routes,
            cooldown,
            max_trade_amount,
        } => update::update_trade_info(deps, info, denom, routes, cooldown, max_trade_amount),
    }
}
