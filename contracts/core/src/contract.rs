use cosmwasm_std::{attr, entry_point, Reply};
use cosmwasm_std::{Deps, DepsMut, Response, SubMsg};
use cosmwasm_std::{Env, MessageInfo, QueryResponse, Uint128};
use ibcx_interface::core::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgCreateDenom, MsgCreateDenomResponse};

use crate::state::{set_assets, Fee, Token, FEE, GOV, TOKEN};
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
            reserve_denom: msg.reserve_denom,
            total_supply: Uint128::zero(),
        },
    )?;

    GOV.save(deps.storage, &deps.api.addr_validate(&msg.gov)?)?;
    FEE.save(
        deps.storage,
        &Fee {
            collector: deps.api.addr_validate(&msg.fee_strategy.collector)?,
            collected: vec![],
            mint: msg.fee_strategy.mint,
            burn: msg.fee_strategy.burn,
            stream: msg.fee_strategy.stream,
            stream_last_collected_at: env.block.time.seconds(),
        },
    )?;
    PAUSED.save(deps.storage, &Default::default())?;
    // Don't we need to check that the contract has received the ammount of to be set here?
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

    // Shouldn't any funds sent to the contract be stored in ASSETS before this is called?
    execute::collect_streaming_fee(deps.storage, env.block.time.seconds())?;

    match msg {
        Mint {
            amount,
            receiver,
            refund_to,
        } => execute::mint(deps, env, info, amount, receiver, refund_to),
        Burn { redeem_to } => execute::burn(deps, env, info, redeem_to),
        Realize {} => execute::realize(deps, env, info),
        Gov(msg) => execute::handle_gov_msg(deps, env, info, msg),
        Rebalance(msg) => execute::handle_rebalance_msg(deps, env, info, msg),
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
                attr("method", "reply_init"),
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
        GetBalance { account } => query::balance(deps, env, account),
        GetConfig {} => query::config(deps, env),
        GetFee { time } => query::fee(deps, env, time),
        GetPauseInfo {} => query::pause_info(deps, env),
        GetPortfolio {} => query::portfolio(deps, env),
        SimulateMint { amount, funds } => query::simulate_mint(deps, env, amount, funds),
        SimulateBurn { amount } => query::simulate_burn(deps, env, amount),
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
