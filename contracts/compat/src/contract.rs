use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    attr, entry_point, to_binary, Env, MessageInfo, QueryResponse, Response, StdError, StdResult,
    Uint128,
};
use cosmwasm_std::{Deps, DepsMut};
use ibc_interface::compat::{
    AmountResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMode, QueryModeResponse, QueryMsg,
};
use osmo_bindings::{OsmosisQuery, Step, Swap, SwapAmount};
use osmosis_std::types::osmosis::gamm::v1beta1::{
    QuerySwapExactAmountInRequest, QuerySwapExactAmountInResponse, QuerySwapExactAmountOutRequest,
    QuerySwapExactAmountOutResponse,
};

use crate::{
    state::{GOV, QUERY_MODE},
    CONTRACT_NAME, CONTRACT_VERSION,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    GOV.save(deps.storage, &deps.api.addr_validate(&msg.gov)?)?;
    QUERY_MODE.save(deps.storage, &msg.mode)?;

    Ok(Response::new().add_attributes(vec![
        attr("method", "init"),
        attr("executor", info.sender),
        attr("gov", msg.gov),
        attr("mode", msg.mode.to_string()),
    ]))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    use ExecuteMsg::*;

    match msg {
        SwitchQueryMode(mode) => {
            if info.sender != GOV.load(deps.storage)? {
                return Err(StdError::generic_err("Unauthorized"));
            }

            QUERY_MODE.save(deps.storage, &mode)?;

            Ok(Response::new().add_attributes(vec![
                attr("method", "switch_query_mode"),
                attr("executor", info.sender),
                attr("mode", mode.to_string()),
            ]))
        }
    }
}

#[cw_serde]
pub struct SwapResponse {
    // If you query with SwapAmount::Input, this is SwapAmount::Output
    // If you query with SwapAmount::Output, this is SwapAmount::Input
    pub swap_amount: SwapAmount,
}

#[entry_point]
pub fn query(deps: Deps<OsmosisQuery>, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::QueryMode {} => {
            let mode = QUERY_MODE.load(deps.storage)?;

            to_binary(&QueryModeResponse { mode })
        }
        QueryMsg::EstimateSwapExactAmountIn {
            sender,
            amount,
            routes,
            mode,
        } => {
            let mode = mode.unwrap_or(QUERY_MODE.load(deps.storage)?);

            let token_out_amount = match mode {
                QueryMode::Stargate => {
                    let resp: QuerySwapExactAmountInResponse = deps.querier.query(
                        &QuerySwapExactAmountInRequest {
                            sender,
                            pool_id: routes.0.first().unwrap().pool_id,
                            token_in: amount.to_string(),
                            routes: routes.clone().into(),
                        }
                        .into(),
                    )?;

                    Uint128::from_str(&resp.token_out_amount)?
                }
                QueryMode::Binding => {
                    let mut route = routes
                        .0
                        .into_iter()
                        .map(|v| Step {
                            pool_id: v.pool_id,
                            denom_out: v.token_denom,
                        })
                        .collect::<Vec<_>>();
                    let first = route.remove(0);

                    let resp: SwapResponse = deps.querier.query(
                        &OsmosisQuery::EstimateSwap {
                            sender,
                            first: Swap::new(first.pool_id, amount.denom, &first.denom_out),
                            route,
                            amount: SwapAmount::In(amount.amount),
                        }
                        .into(),
                    )?;

                    resp.swap_amount.as_out()
                }
            };

            to_binary(&AmountResponse(token_out_amount))
        }
        QueryMsg::EstimateSwapExactAmountOut {
            sender,
            amount,
            routes,
            mode,
        } => {
            let mode = mode.unwrap_or(QUERY_MODE.load(deps.storage)?);

            let token_in_amount = match mode {
                QueryMode::Stargate => {
                    let resp: QuerySwapExactAmountOutResponse = deps.querier.query(
                        &QuerySwapExactAmountOutRequest {
                            sender,
                            pool_id: routes.0.first().unwrap().pool_id,
                            token_out: amount.to_string(),
                            routes: routes.clone().into(),
                        }
                        .into(),
                    )?;

                    Uint128::from_str(&resp.token_in_amount)?
                }
                QueryMode::Binding => {
                    let mut route = routes
                        .0
                        .into_iter()
                        .map(|v| Step {
                            pool_id: v.pool_id,
                            denom_out: v.token_denom,
                        })
                        .collect::<Vec<_>>();
                    route.reverse();
                    let first = route.remove(0);
                    let first_out = route
                        .first()
                        .map(|v| v.denom_out.clone())
                        .unwrap_or_else(|| amount.denom.clone());

                    // shift denoms
                    for i in 0..route.len() {
                        if i == route.len() - 1 {
                            route[i].denom_out = amount.denom.clone();
                        } else {
                            route[i].denom_out = route[i + 1].denom_out.clone();
                        }
                    }

                    let resp: SwapResponse = deps.querier.query(
                        &OsmosisQuery::EstimateSwap {
                            sender,
                            first: Swap::new(first.pool_id, first.denom_out, first_out),
                            route,
                            amount: SwapAmount::Out(amount.amount),
                        }
                        .into(),
                    )?;

                    resp.swap_amount.as_in()
                }
            };

            to_binary(&AmountResponse(token_in_amount))
        }
    }
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
