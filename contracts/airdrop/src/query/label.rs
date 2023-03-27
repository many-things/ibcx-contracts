use cosmwasm_std::{Deps, StdResult};
use ibcx_interface::{
    airdrop::{GetLabelResponse, ListLabelsResponse},
    range_option,
    types::RangeOrder,
};

use crate::{error::ContractError, state::LABELS};

pub fn get_label(deps: Deps, label: String) -> Result<GetLabelResponse, ContractError> {
    let airdrop_id = LABELS.load(deps.storage, &label)?;

    let splitted: Vec<_> = label.split('/').collect();
    let creator = splitted[0].to_string();
    let label = splitted[1..].join("/");

    Ok(GetLabelResponse {
        creator,
        label,
        airdrop_id,
    })
}

pub fn list_labels(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
    order: Option<RangeOrder>,
) -> Result<ListLabelsResponse, ContractError> {
    let label_map_conv = |item: StdResult<(String, u64)>| {
        let (label, airdrop_id) = item?;

        let splitted: Vec<_> = label.split('/').collect();
        let creator = splitted[0].to_string();
        let label = splitted[1..].join("/");

        Ok(GetLabelResponse {
            creator,
            label,
            airdrop_id,
        })
    };

    let start_after = start_after.as_deref();
    let ((min, max), limit, order) = range_option(start_after, limit, order)?;

    let query_resp = LABELS
        .range(deps.storage, min, max, order)
        .take(limit)
        .map(label_map_conv)
        .collect::<StdResult<_>>()?;

    Ok(ListLabelsResponse(query_resp))
}
