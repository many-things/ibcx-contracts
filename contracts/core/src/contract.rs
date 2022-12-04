use cosmwasm_std::{attr, entry_point, Reply, SubMsg};
use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, Uint128};
use ibc_interface::core::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgCreateDenom, MsgCreateDenomResponse};

use crate::state::{set_assets, Token, GOV, TOKEN};
use crate::REPLY_ID_DENOM_CREATION;
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
            decimal: msg.decimal,
            reserve_denom: msg.reserve_denom,
            total_supply: Uint128::zero(),
        },
    )?;

    GOV.save(deps.storage, &deps.api.addr_validate(&msg.gov)?)?;
    PAUSED.save(deps.storage, &Default::default())?;
    set_assets(deps.storage, msg.initial_assets)?;

    let resp = Response::new()
        .add_submessage(SubMsg::reply_on_success(
            MsgCreateDenom {
                sender: env.contract.address.into_string(),
                subdenom: msg.denom,
            },
            REPLY_ID_DENOM_CREATION,
        ))
        .add_attribute("method", "instantiate");

    Ok(resp)
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
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_ID_DENOM_CREATION => {
            let reply_data = msg.result.unwrap().data.unwrap();
            let reply: MsgCreateDenomResponse = reply_data.try_into()?;

            let mut token = TOKEN.load(deps.storage)?;
            token.denom = reply.new_token_denom;
            TOKEN.save(deps.storage, &token)?;

            let resp = Response::new().add_attributes(vec![
                attr("method", "init_reply"),
                attr("new_denom", token.denom),
            ]);

            Ok(resp)
        }
        _ => Err(ContractError::UnknownReplyId { id: msg.id }),
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
