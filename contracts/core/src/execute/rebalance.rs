use cosmwasm_std::{attr, coin, DepsMut, Env, MessageInfo, Response};
use ibc_interface::core::RebalanceMsg;
use osmosis_std::types::osmosis::gamm::v1beta1::MsgSwapExactAmountIn;

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
            amortization,
        } => {
            let resp = Response::new().add_attributes(vec![
                attr("method", "rebalance_init"),
                attr("executor", info.sender),
                attr("manager", manager),
            ]);

            Ok(resp)
        }
        Trade {
            asset,
            amount,
            reserve_token_amount,
        } => {
            // let msg = MsgSwapExactAmountIn {
            //     sender: todo!(),
            //     routes: todo!(),
            //     token_in: Some(coin(amount, denom)),
            //     token_out_min_amount: min_amount_out.to_string(),
            // }
            // .into();

            let resp = Response::new()
                // .add_message(msg)
                .add_attributes(vec![
                    attr("method", "rebalance_trade"),
                    attr("executor", info.sender),
                ]);

            Ok(resp)
        }
        Finish {} => todo!(),
    }
}
