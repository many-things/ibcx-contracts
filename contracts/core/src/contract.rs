use cosmwasm_std::entry_point;
use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, Uint128};
use ibc_interface::core::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;

use crate::state::{set_assets, Token, GOV, TOKEN};
use crate::{error::ContractError, state::PAUSED, CONTRACT_NAME, CONTRACT_VERSION};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    TOKEN.save(
        deps.storage,
        &Token {
            denom: msg.denom.clone(),
            reserve_denom: msg.reserve_denom,
            total_supply: Uint128::zero(),
        },
    )?;

    GOV.save(deps.storage, &deps.api.addr_validate(&msg.gov)?)?;
    PAUSED.save(deps.storage, &Default::default())?;
    set_assets(deps.storage, msg.initial_assets)?;

    let resp = Response::new()
        .add_message(MsgCreateDenom {
            sender: env.contract.address.into_string(),
            subdenom: msg.denom,
        })
        .add_attribute("method", "instantiate");

    Ok(resp)
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{coin, Addr};
    use cw_multi_test::{App, ContractWrapper, Executor};
    use ibc_interface::core::InstantiateMsg;

    #[test]
    pub fn instantiate() {
        let mut app = App::default();
        let contract = ContractWrapper::new(super::execute, super::instantiate, super::query);
        let code = app.store_code(Box::new(contract));

        let code_owner = Addr::unchecked("owner");
        app.instantiate_contract(
            code,
            code_owner,
            &InstantiateMsg {
                gov: "gov".to_string(),
                denom: "uibc".to_string(),
                reserve_denom: "uosmo".to_string(),
                initial_assets: vec![coin(1000000, "uosmo"), coin(1000000, "uion")],
            },
            &[],
            "ibc_core",
            None,
        )
        .unwrap();
    }
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
        Mint { amount, receiver } => execute::mint(deps, env, info, amount, receiver),
        Burn {} => execute::burn(deps, env, info),
        Gov(msg) => execute::gov::handle_msg(deps, env, info, msg),
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use crate::query;
    use QueryMsg::*;

    match msg {
        Config {} => query::config(deps, env),
        PauseInfo {} => query::pause_info(deps, env),
        Portfolio {} => query::portfolio(deps, env),
    }
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Default::default())
}
