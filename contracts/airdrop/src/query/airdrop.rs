use cosmwasm_std::{Deps, StdResult};
use ibcx_interface::{
    airdrop::{
        AirdropId, GetAirdropResponse, LatestAirdropResponse, ListAirdropsQueryOptions,
        ListAirdropsResponse,
    },
    range_option,
};

use crate::{
    airdrop::Airdrop,
    error::ContractError,
    state::{airdrops, load_airdrop, LABELS, LATEST_AIRDROP_ID},
};

pub fn get_airdrop(deps: Deps, id: AirdropId) -> Result<GetAirdropResponse, ContractError> {
    Ok(Airdrop::to_resp(load_airdrop(deps.storage, id)?))
}

pub fn latest_airdrop_id(deps: Deps) -> Result<LatestAirdropResponse, ContractError> {
    Ok(LatestAirdropResponse(LATEST_AIRDROP_ID.load(deps.storage)?))
}

pub fn list_airdrops(
    deps: Deps,
    option: ListAirdropsQueryOptions,
) -> Result<ListAirdropsResponse, ContractError> {
    use ListAirdropsQueryOptions::*;

    let airdrop_map_conv = |res: StdResult<(u64, Airdrop)>| res.map(Airdrop::to_resp);
    let label_map_conv = |res: StdResult<(String, u64)>| -> StdResult<_> {
        let (_, id) = res?;
        let airdrop = airdrops().load(deps.storage, id)?;

        Ok(Airdrop::to_resp((id, airdrop)))
    };

    match option {
        ByID {
            start_after,
            limit,
            order,
        } => {
            let ((min, max), limit, order) = range_option(start_after, limit, order)?;

            let query_res = airdrops()
                .range(deps.storage, min, max, order)
                .take(limit)
                .map(airdrop_map_conv)
                .collect::<StdResult<_>>()?;

            Ok(ListAirdropsResponse(query_res))
        }

        ByType {
            typ: airdrop_type,
            start_after,
            limit,
            order,
        } => {
            let ((min, max), limit, order) = range_option(start_after, limit, order)?;

            let query_res = airdrops()
                .idx
                .by_type
                .prefix(&airdrop_type.to_string())
                .range(deps.storage, min, max, order)
                .take(limit)
                .map(airdrop_map_conv)
                .collect::<StdResult<_>>()?;

            Ok(ListAirdropsResponse(query_res))
        }

        ByLabel {
            start_after,
            limit,
            order,
        } => {
            let start_after = start_after.as_deref();
            let ((min, max), limit, order) = range_option(start_after, limit, order)?;

            let query_res = LABELS
                .range(deps.storage, min, max, order)
                .take(limit)
                .map(label_map_conv)
                .collect::<StdResult<_>>()?;

            Ok(ListAirdropsResponse(query_res))
        }

        ByCreator {
            creator,
            start_after,
            limit,
            order,
        } => {
            let ((min, max), limit, order) = range_option(start_after, limit, order)?;

            let query_res = airdrops()
                .idx
                .by_creator
                .prefix(deps.api.addr_validate(&creator)?)
                .range(deps.storage, min, max, order)
                .take(limit)
                .map(airdrop_map_conv)
                .collect::<StdResult<_>>()?;

            Ok(ListAirdropsResponse(query_res))
        }
    }
}
