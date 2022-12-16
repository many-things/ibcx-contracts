use cosmwasm_std::{
    attr, coin, entry_point, to_binary, BankMsg, Env, MessageInfo, Order, QueryResponse, Reply,
    StdResult,
};
use cw_storage_plus::Bound;
use ibc_alias::{Deps, DepsMut, Response, SubMsg};
use ibc_interface::{
    faucet::{
        Action, ExecuteMsg, GetLastTokenIdResponse, GetRoleResponse, GetTokenResponse,
        InstantiateMsg, ListAliasesResponse, ListRolesResponse, ListTokensResponse, MigrateMsg,
        QueryMsg, TokenCreationConfig,
    },
    get_and_check_limit,
    types::RangeOrder,
    DEFAULT_LIMIT, MAX_LIMIT,
};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{
    MsgBurn, MsgCreateDenom, MsgCreateDenomResponse, MsgMint,
};

use crate::{
    error::ContractError,
    state::{
        get_token, Token, TokenConfig, ALIASES, LAST_TOKEN_ID, ROLES, ROLES_GLOBAL,
        TMP_TOKEN_DENOM, TOKENS,
    },
    CONTRACT_NAME, CONTRACT_VERSION, REPLY_ID_CREATE_DENOM,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    LAST_TOKEN_ID.save(deps.storage, &0)?;

    Ok(Default::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        Create { denom, config } => {
            if get_token(deps.storage, denom.clone()).is_ok() {
                return Err(ContractError::TokenAlreadyExists(denom));
            }

            let config = match config {
                TokenCreationConfig::Managed { admin } => TokenConfig::Managed {
                    admin: deps.api.addr_validate(&admin)?,
                },
                TokenCreationConfig::Unmanaged {} => TokenConfig::Unmanaged {},
            };

            let token_id = LAST_TOKEN_ID.load(deps.storage)?;
            TMP_TOKEN_DENOM.save(deps.storage, &denom)?;
            TOKENS.save(
                deps.storage,
                token_id,
                &Token {
                    id: token_id,
                    denom_v: denom.clone(),
                    denom_r: "".to_string(),
                    config,
                },
            )?;

            Ok(Response::new()
                .add_submessage(SubMsg::reply_on_success(
                    MsgCreateDenom {
                        sender: env.contract.address.to_string(),
                        subdenom: denom.clone(),
                    },
                    REPLY_ID_CREATE_DENOM,
                ))
                .add_attributes(vec![
                    attr("method", "create"),
                    attr("denom", denom),
                    attr("token_id", token_id.to_string()),
                ]))
        }

        Mint { denom, amount } => {
            let token = get_token(deps.storage, denom.clone())?;
            token.check_role(deps.storage, &info.sender, Action::Mint)?;

            let mint_amount = coin(amount.u128(), &token.denom_r);
            Ok(Response::new()
                .add_message(MsgMint {
                    sender: env.contract.address.to_string(),
                    amount: Some(mint_amount.clone().into()),
                })
                .add_message(BankMsg::Send {
                    to_address: info.sender.to_string(),
                    amount: vec![mint_amount],
                })
                .add_attributes(vec![
                    attr("method", "mint"),
                    attr("executor", info.sender),
                    attr("denom", denom),
                    attr("amount", amount),
                ]))
        }
        Burn { denom } => {
            let token = get_token(deps.storage, denom)?;
            token.check_role(deps.storage, &info.sender, Action::Burn)?;
            let received = cw_utils::must_pay(&info, &token.denom_r)?;

            Ok(Response::new()
                .add_message(MsgBurn {
                    sender: env.contract.address.to_string(),
                    amount: Some(coin(received.u128(), &token.denom_r).into()),
                })
                .add_attributes(vec![
                    attr("method", "burn"),
                    attr("executor", info.sender),
                    attr("denom", token.denom_r),
                    attr("amount", received),
                ]))
        }

        Grant {
            denom,
            grantee,
            action,
        } => {
            let token = get_token(deps.storage, denom.clone())?;
            token.check_admin(&info.sender)?;

            ROLES.save(
                deps.storage,
                (
                    token.id,
                    deps.api.addr_validate(&grantee)?,
                    action.to_string(),
                ),
                &true,
            )?;

            Ok(Response::new().add_attributes(vec![
                attr("method", "grant"),
                attr("executor", info.sender),
                attr("denom", denom),
                attr("grantee", grantee),
                attr("action", action.to_string()),
            ]))
        }
        Revoke {
            denom,
            revokee,
            action,
        } => {
            let token = get_token(deps.storage, denom.clone())?;
            token.check_admin(&info.sender)?;

            ROLES.remove(
                deps.storage,
                (
                    token.id,
                    deps.api.addr_validate(&revokee)?,
                    action.to_string(),
                ),
            );

            Ok(Response::new().add_attributes(vec![
                attr("method", "revoke"),
                attr("executor", info.sender),
                attr("denom", denom),
                attr("revokee", revokee),
                attr("action", action.to_string()),
            ]))
        }
        Release { denom, action } => {
            let token = get_token(deps.storage, denom.clone())?;
            token.check_admin(&info.sender)?;

            ROLES_GLOBAL.save(deps.storage, (token.id, action.to_string()), &true)?;

            Ok(Response::new().add_attributes(vec![
                attr("method", "release"),
                attr("executor", info.sender),
                attr("denom", denom),
                attr("action", action.to_string()),
            ]))
        }

        Block { denom, action } => {
            let token = get_token(deps.storage, denom.clone())?;
            token.check_admin(&info.sender)?;

            ROLES_GLOBAL.remove(deps.storage, (token.id, action.to_string()));

            Ok(Response::new().add_attributes(vec![
                attr("method", "block"),
                attr("executor", info.sender),
                attr("denom", denom),
                attr("action", action.to_string()),
            ]))
        }
    }
}

#[entry_point]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_ID_CREATE_DENOM => {
            let reply_data = msg.result.unwrap().data.unwrap();
            let reply: MsgCreateDenomResponse = reply_data.try_into()?;

            let token_id = LAST_TOKEN_ID.load(deps.storage)?;
            LAST_TOKEN_ID.save(deps.storage, &(token_id + 1))?;

            let tmp_token = TMP_TOKEN_DENOM.load(deps.storage)?;
            TMP_TOKEN_DENOM.remove(deps.storage);

            let mut token = TOKENS.load(deps.storage, token_id)?;
            token.denom_r = reply.new_token_denom.clone();
            TOKENS.save(deps.storage, token_id, &token)?;

            ALIASES.save(deps.storage, reply.new_token_denom.clone(), &token_id)?;
            ALIASES.save(deps.storage, tmp_token.clone(), &token_id)?;

            Ok(Response::new().add_attributes(vec![
                attr("method", "reply_create"),
                attr("alias-v", tmp_token),
                attr("alias-r", reply.new_token_denom),
                attr("token_id", token_id.to_string()),
            ]))
        }
        _ => Err(ContractError::UnknownReplyId(msg.id)),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use QueryMsg::*;

    match msg {
        ListAliases {
            start_after,
            limit,
            order,
        } => {
            let limit = get_and_check_limit(limit, MAX_LIMIT, DEFAULT_LIMIT)? as usize;
            let order = order.unwrap_or(RangeOrder::Asc).into();
            let (min, max) = match order {
                Order::Ascending => (start_after.map(Bound::exclusive), None),
                Order::Descending => (None, start_after.map(Bound::exclusive)),
            };

            let resps = ALIASES
                .range(deps.storage, min, max, order)
                .take(limit)
                .collect::<StdResult<_>>()?;

            Ok(to_binary(&ListAliasesResponse(resps))?)
        }

        GetToken { denom } => {
            let token = get_token(deps.storage, denom)?;

            let config = match token.config {
                TokenConfig::Managed { admin } => TokenCreationConfig::Managed {
                    admin: admin.to_string(),
                },
                TokenConfig::Unmanaged {} => TokenCreationConfig::Unmanaged {},
            };

            Ok(to_binary(&GetTokenResponse {
                id: token.id,
                denom_v: token.denom_v,
                denom_r: token.denom_r,
                config,
            })?)
        }
        ListTokens {
            start_after,
            limit,
            order,
        } => {
            let limit = get_and_check_limit(limit, MAX_LIMIT, DEFAULT_LIMIT)? as usize;
            let order = order.unwrap_or(RangeOrder::Asc).into();
            let (min, max) = match order {
                Order::Ascending => (start_after.map(Bound::exclusive), None),
                Order::Descending => (None, start_after.map(Bound::exclusive)),
            };

            let resps = TOKENS
                .range(deps.storage, min, max, order)
                .take(limit)
                .map(|item| {
                    let (_, token) = item?;

                    Ok(GetTokenResponse {
                        id: token.id,
                        denom_v: token.denom_v,
                        denom_r: token.denom_r,
                        config: match token.config {
                            TokenConfig::Managed { admin } => TokenCreationConfig::Managed {
                                admin: admin.to_string(),
                            },
                            TokenConfig::Unmanaged {} => TokenCreationConfig::Unmanaged {},
                        },
                    })
                })
                .collect::<StdResult<Vec<_>>>()?;

            Ok(to_binary(&ListTokensResponse(resps))?)
        }
        GetLastTokenId {} => Ok(to_binary(&GetLastTokenIdResponse(
            LAST_TOKEN_ID.load(deps.storage)?,
        ))?),
        GetRole { denom, account } => {
            let token = get_token(deps.storage, denom.clone())?;
            let account = deps.api.addr_validate(&account)?;

            let actions = Action::VALUES.clone();

            let globals: Vec<bool> = actions
                .iter()
                .map(|v| {
                    let value = ROLES_GLOBAL
                        .may_load(deps.storage, (token.id, v.to_string()))?
                        .unwrap_or(false);
                    Ok(value)
                })
                .collect::<StdResult<_>>()?;
            let roles = actions
                .into_iter()
                .enumerate()
                .map(|(idx, v)| {
                    let has_role = ROLES
                        .may_load(deps.storage, (token.id, account.clone(), v.to_string()))?
                        .unwrap_or_default();
                    Ok((v, globals[idx] || has_role))
                })
                .collect::<StdResult<_>>()?;

            Ok(to_binary(&GetRoleResponse {
                denom,
                account: account.to_string(),
                roles,
            })?)
        }
        ListRoles {
            denom,
            start_after,
            limit,
            order,
        } => {
            let token = get_token(deps.storage, denom)?;
            let order = order.unwrap_or(RangeOrder::Asc).into();
            let limit = get_and_check_limit(limit, MAX_LIMIT, DEFAULT_LIMIT)? as usize;
            let start = start_after
                .map(|(account, action)| StdResult::Ok((deps.api.addr_validate(&account)?, action)))
                .transpose()?;
            let (min, max) = match order {
                Order::Ascending => (start.map(Bound::exclusive), None),
                Order::Descending => (None, start.map(Bound::exclusive)),
            };

            let actions = Action::VALUES.clone();

            let globals: Vec<bool> = actions
                .iter()
                .map(|v| {
                    let value = ROLES_GLOBAL
                        .may_load(deps.storage, (token.id, v.to_string()))?
                        .unwrap_or(false);
                    Ok(value)
                })
                .collect::<StdResult<_>>()?;

            let resps = ROLES
                .sub_prefix(token.id)
                .range(deps.storage, min, max, order)
                .take(limit)
                .map(|item| {
                    let ((account, action), v) = item?;

                    Ok((
                        account.to_string(),
                        action.clone(),
                        globals[Action::VALUES
                            .iter()
                            .position(|v| v.to_string() == action)
                            .unwrap()]
                            || v,
                    ))
                })
                .collect::<StdResult<_>>()?;

            Ok(to_binary(&ListRolesResponse(resps))?)
        }
    }
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Default::default())
}
