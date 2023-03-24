use cosmwasm_std::{Deps, QueryResponse, StdResult};
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
    to_binary,
};

fn to_resp((id, airdrop): (u64, Airdrop)) -> GetAirdropResponse {
    match airdrop {
        Airdrop::Open(inner) => GetAirdropResponse::Open {
            id,
            creator: inner.creator.to_string(),
            denom: inner.denom,
            total_amount: inner.total_amount,
            total_claimed: inner.total_claimed,
            merkle_root: inner.merkle_root,
            label: inner.label,
            created_at: inner.created_at,
            closed_at: inner.closed_at,
        },
        Airdrop::Bearer(inner) => GetAirdropResponse::Bearer {
            id,
            creator: inner.creator.to_string(),
            signer: inner.signer.to_string(),
            signer_pub: hex::encode(inner.signer_pub),

            denom: inner.denom,
            total_amount: inner.total_amount,
            total_claimed: inner.total_claimed,
            merkle_root: inner.merkle_root,
            label: inner.label,
            created_at: inner.created_at,
            closed_at: inner.closed_at,
        },
    }
}

pub fn get_airdrop(deps: Deps, id: AirdropId) -> Result<QueryResponse, ContractError> {
    to_binary(&to_resp(load_airdrop(deps.storage, id)?))
}

pub fn latest_airdrop_id(deps: Deps) -> Result<QueryResponse, ContractError> {
    to_binary(&LatestAirdropResponse(
        LATEST_AIRDROP_ID.load(deps.storage)?,
    ))
}

pub fn list_airdrops(
    deps: Deps,
    option: ListAirdropsQueryOptions,
) -> Result<QueryResponse, ContractError> {
    use ListAirdropsQueryOptions::*;

    let airdrop_map_conv = |res: StdResult<(u64, Airdrop)>| res.map(to_resp);
    let label_map_conv = |res: StdResult<(String, u64)>| -> StdResult<_> {
        let (_, id) = res?;
        let airdrop = airdrops().load(deps.storage, id)?;

        Ok(to_resp((id, airdrop)))
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

            to_binary(&ListAirdropsResponse(query_res))
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

            to_binary(&ListAirdropsResponse(query_res))
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

            to_binary(&ListAirdropsResponse(query_res))
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

            to_binary(&ListAirdropsResponse(query_res))
        }
    }
}
