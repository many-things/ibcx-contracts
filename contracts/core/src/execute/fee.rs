use cosmwasm_std::{attr, BankMsg, CosmosMsg, DepsMut, MessageInfo, Response, Storage};

use crate::{
    assert_sender,
    state::{FEE, INDEX_UNITS, TOTAL_SUPPLY},
};

use crate::StdResult;

pub fn collect_streaming_fee(storage: &mut dyn Storage, now_in_sec: u64) -> StdResult<()> {
    let mut fee = FEE.load(storage)?;
    let total_supply = TOTAL_SUPPLY.load(storage)?;

    if let Some(streaming_fee) = fee.streaming_fee.as_mut() {
        let index_units = INDEX_UNITS.load(storage)?;
        let (new_index_units, _) = streaming_fee.collect(index_units, now_in_sec, total_supply)?;
        INDEX_UNITS.save(storage, &new_index_units)?;
    }

    FEE.save(storage, &fee)?;

    Ok(())
}

pub fn realize_streaming_fee(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let mut fee = FEE.load(deps.storage)?;
    assert_sender(&fee.collector, &info.sender)?;

    let mut msgs: Vec<CosmosMsg> = vec![];

    // collect streaming fee
    if let Some(mut streaming_fee) = fee.streaming_fee.as_mut() {
        msgs.push(
            BankMsg::Send {
                to_address: fee.collector.to_string(),
                amount: streaming_fee.collected.clone(),
            }
            .into(),
        );

        streaming_fee.collected = vec![];
    }

    FEE.save(deps.storage, &fee)?;

    Ok(Response::new().add_messages(msgs).add_attributes(vec![
        attr("method", "realize"),
        attr("executor", info.sender),
    ]))
}
