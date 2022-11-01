use cosmwasm_std::{
    attr, coin, entry_point, BankMsg, Coin, Deps, DepsMut, Env, MessageInfo, QueryResponse,
    Response,
};
use ibc_interface::core::{
    ConfigMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, RebalanceMsg,
};
use osmosis_std::types::osmosis::{
    gamm::v1beta1::{MsgSwapExactAmountIn, MsgSwapExactAmountOut},
    tokenfactory::v1beta1::{MsgCreateDenom, MsgMint},
};

use crate::{
    error::ContractError,
    execute::handle_msg,
    state::{Config, CONFIG, PAUSED},
    CONTRACT_NAME, CONTRACT_VERSION,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(
        deps.storage,
        &Config {
            gov: deps.api.addr_validate(&msg.gov)?,
            denom: msg.denom.clone(),
            reserve_denom: msg.reserve_denom,
            assets: msg.assets,
        },
    )?;

    PAUSED.save(deps.storage, &Default::default())?;

    let msg = MsgCreateDenom {
        sender: env.contract.address.into_string(),
        subdenom: msg.denom,
    };

    let resp = Response::new()
        .add_message(msg)
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
    crate::execute::handle_msg(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    Ok(Default::default())
}

#[entry_point]
pub fn migrate(deps: Deps, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Default::default())
}
