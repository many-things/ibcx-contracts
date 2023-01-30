use cosmwasm_std::{entry_point, Env, MessageInfo, QueryResponse};
use cosmwasm_std::{Deps, DepsMut, Response};
use ibcx_interface::airdrop::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::{
    error::ContractError, execute, query, state::LATEST_AIRDROP_ID, CONTRACT_NAME, CONTRACT_VERSION,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    LATEST_AIRDROP_ID.save(deps.storage, &0)?;

    Ok(Default::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        Register {
            merkle_root,
            denom,
            label,
            bearer,
        } => execute::register(deps, info, merkle_root, denom, label, bearer),
        Fund { id } => execute::fund(deps, info, id),
        Claim(payload) => execute::claim(deps, info, payload),
        MultiClaim(payload) => execute::multi_claim(deps, info, payload),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use QueryMsg::*;

    match msg {
        GetAirdrop { id } => query::get_airdrop(deps, id),
        ListAirdrops {
            start_after,
            limit,
            order,
        } => query::list_airdrops(deps, start_after, limit, order),
        LatestAirdropId {} => query::latest_airdrop_id(deps),
        GetClaim { id, claim_proof } => query::get_claim(deps, id, claim_proof),
        ListClaims {
            id,
            start_after,
            limit,
            order,
        } => query::list_claims(deps, id, start_after, limit, order),
        CheckQualification {
            id,
            amount,
            claim_proof,
            merkle_proof,
        } => query::check_qualification(deps, id, amount, claim_proof, merkle_proof),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    if !msg.force.unwrap_or_default() {
        ibcx_utils::store_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    } else {
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    }

    Ok(Default::default())
}

#[cfg(test)]
mod test {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use crate::test::SENDER_OWNER;

    use super::*;

    #[test]
    fn init() {
        let mut deps = mock_dependencies();

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info(SENDER_OWNER, &[]),
            InstantiateMsg {},
        )
        .unwrap();

        let airdrop_id = LATEST_AIRDROP_ID.load(deps.as_ref().storage).unwrap();
        assert_eq!(airdrop_id, 0);

        let version = cw2::get_contract_version(deps.as_ref().storage).unwrap();
        assert_eq!(
            version,
            cw2::ContractVersion {
                contract: CONTRACT_NAME.to_string(),
                version: CONTRACT_VERSION.to_string()
            }
        );
    }
}
