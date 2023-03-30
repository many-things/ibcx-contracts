use cosmwasm_schema::serde::Serialize;
use cosmwasm_std::{attr, entry_point, Reply};
use cosmwasm_std::{Deps, DepsMut, Response, SubMsg};
use cosmwasm_std::{Env, MessageInfo, QueryResponse, Uint128};
use ibcx_interface::core::{
    ExecuteMsg, GovMsg, InstantiateMsg, MigrateMsg, QueryMsg, RebalanceMsg,
};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgCreateDenom, MsgCreateDenomResponse};

use crate::state::{Config, Fee, StreamingFee, Units, CONFIG, FEE, INDEX_UNITS, TOTAL_SUPPLY};
use crate::StdResult;
use crate::{error::ContractError, CONTRACT_NAME, CONTRACT_VERSION, REPLY_ID_DENOM_CREATION};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let index_units: Units = msg.index_units.into();
    if index_units.check_duplicate() {
        return Err(ContractError::DenomDuplicated {});
    }

    // fee
    let fee = Fee {
        collector: deps.api.addr_validate(&msg.fee.collector)?,
        mint_fee: msg.fee.mint_fee,
        burn_fee: msg.fee.burn_fee,
        streaming_fee: msg.fee.streaming_fee.map(|v| StreamingFee {
            rate: v.rate,
            collected: vec![],
            last_collected_at: env.block.time.seconds(),
            freeze: v.freeze,
        }),
    };

    // config
    let config = Config {
        gov: deps.api.addr_validate(&msg.gov)?,
        paused: Default::default(),
        index_denom: "undefined".to_string(),
        reserve_denom: msg.reserve_denom,
    };

    // apply initial state
    FEE.save(deps.storage, &fee)?;
    CONFIG.save(deps.storage, &config)?;
    TOTAL_SUPPLY.save(deps.storage, &Uint128::zero())?;
    INDEX_UNITS.save(deps.storage, &index_units)?;

    // response
    let msg_create_denom = SubMsg::reply_on_success(
        MsgCreateDenom {
            sender: env.contract.address.into_string(),
            subdenom: config.index_denom,
        },
        REPLY_ID_DENOM_CREATION,
    );

    let resp = Response::new()
        .add_submessage(msg_create_denom)
        .add_attribute("method", "instantiate");

    Ok(resp)
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    use crate::execute;
    use ExecuteMsg::*;

    {
        use crate::execute::collect_streaming_fee as collect;

        let now_in_sec = env.block.time.seconds();

        match msg {
            Mint { .. } => collect(deps.storage, now_in_sec)?,
            Burn { .. } => collect(deps.storage, now_in_sec)?,
            Realize {} => collect(deps.storage, now_in_sec)?,
            Rebalance(RebalanceMsg::Init { .. }) => collect(deps.storage, now_in_sec)?,
            Gov(GovMsg::UpdateFeeStrategy(..)) => collect(deps.storage, now_in_sec)?,
            _ => {}
        };
    }

    match msg {
        Mint {
            amount,
            receiver,
            refund_to,
        } => execute::mint(deps, env, info, amount, receiver, refund_to),
        Burn { redeem_to } => execute::burn(deps, env, info, redeem_to),
        Realize {} => execute::realize_streaming_fee(deps, info),

        Gov(msg) => execute::handle_gov_msg(deps, env, info, msg),
        Rebalance(msg) => execute::handle_rebalance_msg(deps, env, info, msg),
    }
}

#[entry_point]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        REPLY_ID_DENOM_CREATION => {
            let reply_data = msg.result.unwrap().data.unwrap();
            let reply: MsgCreateDenomResponse = reply_data.try_into()?;

            let mut config = CONFIG.load(deps.storage)?;
            config.index_denom = reply.new_token_denom;
            CONFIG.save(deps.storage, &config)?;

            let resp = Response::new().add_attributes(vec![
                attr("method", "reply_instantiate"),
                attr("new_denom", config.index_denom),
            ]);

            Ok(resp)
        }
        _ => Err(ContractError::UnknownReplyId { id: msg.id }),
    }
}

fn to_binary<T: Serialize>(res: StdResult<T>) -> StdResult<QueryResponse> {
    match res {
        Ok(v) => Ok(cosmwasm_std::to_binary(&v)?),
        Err(e) => Err(e),
    }
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<QueryResponse> {
    // use crate::query;
    // use QueryMsg::*;

    // match msg {
    //     GetBalance { account } => to_binary(query::balance(deps, env, account)),
    //     GetConfig {} => to_binary(query::config(deps, env)),
    //     GetFee { time } => to_binary(query::fee(deps, env, time)),
    //     GetPauseInfo {} => to_binary(query::pause_info(deps, env)),
    //     GetPortfolio {} => to_binary(query::portfolio(deps, env)),
    //     SimulateMint { amount, funds } => to_binary(query::simulate_mint(deps, env, amount, funds)),
    //     SimulateBurn { amount } => to_binary(query::simulate_burn(deps, env, amount)),
    // }

    to_binary(Ok(()))
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> StdResult<Response> {
    if !msg.force.unwrap_or_default() {
        ibcx_utils::store_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    } else {
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    }

    Ok(Default::default())
}
