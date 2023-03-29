use std::convert::identity;

use cosmwasm_std::{attr, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response};

use crate::{
    assert_sender,
    error::ContractError,
    state::{FEE, INDEX_UNITS, TOTAL_SUPPLY},
};

pub fn realize(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let fee = FEE.load(deps.storage)?;
    assert_sender(&fee.collector, &info.sender)?;

    let total_supply = TOTAL_SUPPLY.load(deps.storage)?;

    let mut msgs: Vec<Option<CosmosMsg>> = vec![];

    // collect streaming fee
    if let Some(mut streaming_fee) = fee.streaming_fee {
        let index_units = INDEX_UNITS.load(deps.storage)?;

        let (new_index_units, collected) =
            streaming_fee.collect(&env, index_units, total_supply)?;

        let msg = collected.map(|v| {
            BankMsg::Send {
                to_address: fee.collector.to_string(),
                amount: v,
            }
            .into()
        });

        INDEX_UNITS.save(deps.storage, &new_index_units)?;

        msgs.push(msg);
    }

    let msgs = msgs.into_iter().filter_map(identity).collect::<Vec<_>>();

    Ok(Response::new().add_messages(msgs).add_attributes(vec![
        attr("method", "realize"),
        attr("executor", info.sender),
    ]))
}
