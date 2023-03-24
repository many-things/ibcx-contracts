use cosmwasm_std::{entry_point, Env, MessageInfo, QueryResponse};
use cosmwasm_std::{Deps, DepsMut, Response};
use ibcx_interface::airdrop::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::{error::ContractError, state::LATEST_AIRDROP_ID, CONTRACT_NAME, CONTRACT_VERSION};

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
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use crate::execute;
    use ExecuteMsg::*;

    match msg {
        Register(payload) => execute::register(deps, env, info, payload),

        Fund(airdrop) => execute::fund(deps, info, airdrop),

        Claim(payload) => execute::claim(deps, info, payload),

        Close(airdrop) => execute::close(deps, env, info, airdrop),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use crate::query;
    use QueryMsg::*;

    match msg {
        GetAirdrop(airdrop) => query::get_airdrop(deps, airdrop),
        ListAirdrops(option) => query::list_airdrops(deps, option),
        LatestAirdropId {} => query::latest_airdrop_id(deps),

        GetClaim { airdrop, claim_key } => query::get_claim(deps, airdrop, claim_key),
        VerifyClaim(payload) => query::verify_claim(deps, payload),
        ListClaims {
            airdrop,
            start_after,
            limit,
            order,
        } => query::list_claims(deps, airdrop, start_after, limit, order),

        GetLabel(label) => query::get_label(deps, label),
        ListLabels {
            start_after,
            limit,
            order,
        } => query::list_labels(deps, start_after, limit, order),
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

    use super::*;

    #[test]
    fn init() {
        let mut deps = mock_dependencies();

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("owner", &[]),
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
