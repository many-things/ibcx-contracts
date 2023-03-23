pub fn fund(deps: DepsMut, info: MessageInfo, id: AirdropId) -> Result<Response, ContractError> {
    let airdrop_id = match id {
        AirdropId::Id(id) => id,
        AirdropId::Label(label) => LABELS.load(deps.storage, &label)?,
    };

    let mut airdrop = AIRDROPS.load(deps.storage, airdrop_id)?;
    if airdrop.creator != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    if airdrop.closed {
        return Err(ContractError::AirdropClosed {});
    }

    let received = cw_utils::must_pay(&info, &airdrop.denom)?;
    airdrop.total_amount = airdrop.total_amount.checked_add(received)?;

    AIRDROPS.save(deps.storage, airdrop_id, &airdrop)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "fund"),
        attr("executor", info.sender),
        attr("airdrop_id", airdrop_id.to_string()),
        attr("amount", received),
    ]))
}
