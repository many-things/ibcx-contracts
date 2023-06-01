use std::collections::{BTreeMap, HashSet};

use cosmwasm_std::{
    attr, coin, to_binary, BankMsg, Coin, Env, MessageInfo, SubMsg, Uint128, WasmMsg,
};
use cosmwasm_std::{DepsMut, Response};
use ibcx_interface::periphery::{ExecuteMsg, RouteKey, SwapInfo};
use ibcx_interface::{core, helpers::IbcCore};

use crate::pool::{query_pools, OsmosisPool};
use crate::state::{Context, CONTEXT};
use crate::REPLY_ID_BURN_EXACT_AMOUNT_IN;
use crate::{error::ContractError, msgs::make_mint_swap_exact_out_msgs};

fn extract_pool_ids(swap_info: Vec<SwapInfo>) -> Vec<u64> {
    let mut pool_ids = swap_info
        .into_iter()
        .flat_map(|v| v.0 .1 .0.into_iter().map(|r| r.pool_id))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    pool_ids.sort();

    pool_ids
}

fn estimate_out_given_in(
    token_in: Coin,
    token_out: String,
    pools: &mut BTreeMap<u64, Box<dyn OsmosisPool>>,
    swap_info: &[SwapInfo],
) -> Result<Uint128, ContractError> {
    let SwapInfo((_, routes)) = swap_info
        .iter()
        .find(|SwapInfo((RouteKey((from, to)), _))| from == &token_in.denom && to == &token_out)
        .ok_or(ContractError::SwapRouteNotFound {
            from: token_in.denom.clone(),
            to: token_out,
        })?;

    let Coin {
        amount: amount_out, ..
    } = routes.0.iter().try_fold(token_in, |acc, route| {
        let pool = pools
            .get_mut(&route.pool_id)
            .ok_or(ContractError::PoolNotFound(route.pool_id))?;

        let spread_factor = pool.get_spread_factor()?;
        let amount_out = pool.swap_exact_amount_in(
            acc,
            route.token_denom.clone(),
            Uint128::zero(),
            spread_factor,
        )?;

        Ok::<_, ContractError>(coin(amount_out.u128(), &route.token_denom))
    })?;

    Ok(amount_out)
}

fn estimate_in_given_out(
    token_in: String,
    token_out: Coin,
    pools: &mut BTreeMap<u64, Box<dyn OsmosisPool>>,
    swap_info: &[SwapInfo],
) -> Result<Uint128, ContractError> {
    let SwapInfo((_, routes)) = swap_info
        .iter()
        .find(|SwapInfo((RouteKey((from, to)), _))| from == &token_in && to == &token_out.denom)
        .ok_or(ContractError::SwapRouteNotFound {
            from: token_in,
            to: token_out.denom.clone(),
        })?;

    let mut routes = routes.clone();
    routes.0.reverse();

    let Coin {
        amount: amount_out, ..
    } = routes.0.iter().try_fold(token_out, |acc, route| {
        let pool = pools
            .get_mut(&route.pool_id)
            .ok_or(ContractError::PoolNotFound(route.pool_id))?;

        let spread_factor = pool.get_spread_factor()?;
        let amount_in = pool.swap_exact_amount_out(
            route.token_denom.clone(),
            Uint128::new(u128::MAX),
            acc,
            spread_factor,
        )?;

        Ok::<_, ContractError>(coin(amount_in.u128(), &route.token_denom))
    })?;

    Ok(amount_out)
}

pub fn mint_exact_amount_in(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    core_addr: String,
    input_asset: String,
    _min_output_amount: Uint128,
    swap_info: Vec<SwapInfo>,
) -> Result<Response, ContractError> {
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_config = core.get_config(&deps.querier, None)?;
    let core_portfolio = core.get_portfolio(&deps.querier, None)?;
    deps.api.debug(&format!("{:?}", core_portfolio.units));

    let input_amount = cw_utils::must_pay(&info, &input_asset)?;
    let input_token = coin(input_amount.u128(), &input_asset);

    let pool_ids = extract_pool_ids(swap_info.clone());
    let pools = query_pools(&deps.querier, pool_ids)?;

    let mut pools_map = pools
        .clone()
        .into_iter()
        .map(|v| (v.get_id(), v))
        .collect::<BTreeMap<_, _>>();

    let token_one_in = core_portfolio
        .units
        .iter()
        .map(|(denom, unit)| coin((Uint128::new(1000000) * *unit).u128(), denom))
        .try_fold(Uint128::zero(), |acc, token_out| {
            let token_in =
                estimate_in_given_out(input_asset.clone(), token_out, &mut pools_map, &swap_info)?;

            Ok::<_, ContractError>(acc + token_in)
        })?;

    deps.api.debug(&format!("{}", token_one_in));

    Ok(Response::default())
}

pub fn mint_exact_amount_out(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    core_addr: String,
    output_amount: Uint128,
    input_asset: String,
    swap_info: Vec<SwapInfo>,
) -> Result<Response, ContractError> {
    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_config = core.get_config(&deps.querier, None)?;

    // input & output
    let max_input_amount = cw_utils::must_pay(&info, &input_asset)?;
    let max_input = coin(max_input_amount.u128(), &input_asset);
    let output = coin(output_amount.u128(), core_config.index_denom);

    let sim_resp = core.simulate_mint(&deps.querier, output.amount, None, None)?;
    let mut sim_amount_desired = sim_resp.fund_spent;
    sim_amount_desired.sort_by(|a, b| a.denom.cmp(&b.denom));

    let (swap_msgs, refund) = make_mint_swap_exact_out_msgs(
        &deps.querier,
        &env.contract.address,
        swap_info,
        sim_amount_desired.clone(),
        &max_input,
    )?;

    let finish_msg = WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        msg: to_binary(&ExecuteMsg::FinishOperation {
            refund_to: info.sender.to_string(),
            refund_asset: input_asset,
        })?,
        funds: vec![],
    };

    let mint_msg = core.call_with_funds(
        core::ExecuteMsg::Mint {
            amount: output.amount,
            receiver: Some(info.sender.to_string()),
            refund_to: Some(info.sender.to_string()),
        },
        sim_amount_desired,
    )?;

    let resp = Response::new()
        .add_messages(swap_msgs)
        .add_message(mint_msg)
        .add_message(finish_msg)
        .add_attributes(vec![
            attr("method", "mint_exact_amount_out"),
            attr("executor", info.sender),
            attr("max_input", max_input.to_string()),
            attr("output", output.to_string()),
            attr("refund", refund.to_string()),
        ]);

    Ok(resp)
}

pub fn burn_exact_amount_in(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    core_addr: String,
    output_asset: String,
    min_output_amount: Uint128,
    swap_info: Vec<SwapInfo>,
) -> Result<Response, ContractError> {
    // query to core contract
    let core = IbcCore(deps.api.addr_validate(&core_addr)?);
    let core_config = core.get_config(&deps.querier, None)?;

    // input & output
    let input_amount = cw_utils::must_pay(&info, &core_config.index_denom)?;
    let input = coin(input_amount.u128(), &core_config.index_denom);
    let min_output = coin(min_output_amount.u128(), output_asset);

    let expected = core
        .simulate_burn(&deps.querier, input.amount, None)?
        .redeem_amount;

    let burn_msg = core.call_with_funds(
        core::ExecuteMsg::Burn { redeem_to: None },
        vec![coin(input.amount.u128(), &core_config.index_denom)],
    )?;

    // save to context
    CONTEXT.save(
        deps.storage,
        &Context::Burn {
            core: core.addr(),
            sender: info.sender.clone(),
            input: input.clone(),
            min_output: min_output.clone(),
            redeem_amounts: expected,
            swap_info,
        },
    )?;

    let resp = Response::new()
        .add_submessage(SubMsg::reply_on_success(
            burn_msg,
            REPLY_ID_BURN_EXACT_AMOUNT_IN,
        ))
        .add_attributes(vec![
            attr("method", "burn_exact_amount_in"),
            attr("executor", info.sender),
            attr("input_amount", input.to_string()),
            attr("min_output_amount", min_output.to_string()),
        ]);

    Ok(resp)
}

pub fn burn_exact_amount_out(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _core_addr: String,
    _output_asset: String,
    _output_amount: Uint128,
    _swap_info: Vec<SwapInfo>,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

pub fn finish_operation(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    refund_to: String,
    refund_asset: String,
) -> Result<Response, ContractError> {
    assert_eq!(info.sender, env.contract.address, "internal function");
    deps.api.addr_validate(&refund_to)?;

    let balance = deps
        .querier
        .query_balance(env.contract.address, &refund_asset)?;

    deps.api
        .debug(format!("finish balance {balance:?}").as_str());

    let resp = Response::new()
        .add_attributes(vec![
            attr("method", "finish_operation"),
            attr("refund_to", &refund_to),
            attr("refund_asset", refund_asset),
            attr("amount", balance.amount),
        ])
        .add_message(BankMsg::Send {
            to_address: refund_to,
            amount: vec![balance],
        });

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, str::FromStr};
    use cosmwasm_std::{Binary, Decimal, coin, Uint128};
    use ibcx_interface::periphery::{SwapInfosCompact, SwapInfoCompact, SwapInfo};

    use crate::{pool::{OsmosisPool, PREFIX_WEIGHTED_POOL, PREFIX_STABLE_POOL, WeightedPoolResponse, StablePoolResponse}, error::ContractError};

    use super::estimate_in_given_out;

    fn mock_pools() -> Vec<Box<dyn OsmosisPool>> {
        let pools_raw_enc = [            
            "eyJwb29sIjp7IkB0eXBlIjoiL29zbW9zaXMuZ2FtbS52MWJldGExLlBvb2wiLCJhZGRyZXNzIjoib3NtbzE5ZTJtZjdjeXdrdjd6YXVnNm5rNWY4N2QwN2Z4cmRncmxhZHZ5bWgyZ3d2NWNydm0zdm5zdWV3aGg3IiwiaWQiOiIxIiwicG9vbF9wYXJhbXMiOnsic3dhcF9mZWUiOiIwLjAxMDAwMDAwMDAwMDAwMDAwMCIsImV4aXRfZmVlIjoiMC4wMDAwMDAwMDAwMDAwMDAwMDAiLCJzbW9vdGhfd2VpZ2h0X2NoYW5nZV9wYXJhbXMiOm51bGx9LCJmdXR1cmVfcG9vbF9nb3Zlcm5vciI6IiIsInRvdGFsX3NoYXJlcyI6eyJkZW5vbSI6ImdhbW0vcG9vbC8xIiwiYW1vdW50IjoiMTAwMDAwMDAwMDAwMDAwMDAwMDAwIn0sInBvb2xfYXNzZXRzIjpbeyJ0b2tlbiI6eyJkZW5vbSI6ImZhY3Rvcnkvb3NtbzFlNjJkMHVzamRwNnJkOTR0bDJwMGc3Z2h4ZGxlZG1jZzI5NGUyOS91dXNkIiwiYW1vdW50IjoiMjk2MDAwMDAwMDAwMDAifSwid2VpZ2h0IjoiMTA3Mzc0MTgyNDAwMDAwMCJ9LHsidG9rZW4iOnsiZGVub20iOiJ1b3NtbyIsImFtb3VudCI6IjQwMDAwMDAwMDAwMDAwIn0sIndlaWdodCI6IjEwNzM3NDE4MjQwMDAwMDAifV0sInRvdGFsX3dlaWdodCI6IjIxNDc0ODM2NDgwMDAwMDAifX0=",
            "eyJwb29sIjp7IkB0eXBlIjoiL29zbW9zaXMuZ2FtbS52MWJldGExLlBvb2wiLCJhZGRyZXNzIjoib3NtbzE4ZGRjc3E0anpmMzN4OWYzdnBsdjk3Nzl1dmpxNnlweDNlbjN3dDlzZDkzbmptYXM3eWtzdDZ6MzR1IiwiaWQiOiIyIiwicG9vbF9wYXJhbXMiOnsic3dhcF9mZWUiOiIwLjAxMDAwMDAwMDAwMDAwMDAwMCIsImV4aXRfZmVlIjoiMC4wMDAwMDAwMDAwMDAwMDAwMDAiLCJzbW9vdGhfd2VpZ2h0X2NoYW5nZV9wYXJhbXMiOm51bGx9LCJmdXR1cmVfcG9vbF9nb3Zlcm5vciI6IiIsInRvdGFsX3NoYXJlcyI6eyJkZW5vbSI6ImdhbW0vcG9vbC8yIiwiYW1vdW50IjoiMTAwMDAwMDAwMDAwMDAwMDAwMDAwIn0sInBvb2xfYXNzZXRzIjpbeyJ0b2tlbiI6eyJkZW5vbSI6ImZhY3Rvcnkvb3NtbzFlNjJkMHVzamRwNnJkOTR0bDJwMGc3Z2h4ZGxlZG1jZzI5NGUyOS91anB5IiwiYW1vdW50IjoiNDA2NTYwMDAwMDAwMDAifSwid2VpZ2h0IjoiMTA3Mzc0MTgyNDAwMDAwMCJ9LHsidG9rZW4iOnsiZGVub20iOiJ1b3NtbyIsImFtb3VudCI6IjQwMDAwMDAwMDAwMDAwIn0sIndlaWdodCI6IjEwNzM3NDE4MjQwMDAwMDAifV0sInRvdGFsX3dlaWdodCI6IjIxNDc0ODM2NDgwMDAwMDAifX0=",
            "eyJwb29sIjp7IkB0eXBlIjoiL29zbW9zaXMuZ2FtbS52MWJldGExLlBvb2wiLCJhZGRyZXNzIjoib3NtbzFhZnQ0MGwzbmFxN21jOWRhNTl5YzdtcjdwN3MwMmp4NWtzN3UzdWpjNm1odGw0dmxlNDdzbmVmMmhhIiwiaWQiOiIzIiwicG9vbF9wYXJhbXMiOnsic3dhcF9mZWUiOiIwLjAxMDAwMDAwMDAwMDAwMDAwMCIsImV4aXRfZmVlIjoiMC4wMDAwMDAwMDAwMDAwMDAwMDAiLCJzbW9vdGhfd2VpZ2h0X2NoYW5nZV9wYXJhbXMiOm51bGx9LCJmdXR1cmVfcG9vbF9nb3Zlcm5vciI6IiIsInRvdGFsX3NoYXJlcyI6eyJkZW5vbSI6ImdhbW0vcG9vbC8zIiwiYW1vdW50IjoiMTAwMDAwMDAwMDAwMDAwMDAwMDAwIn0sInBvb2xfYXNzZXRzIjpbeyJ0b2tlbiI6eyJkZW5vbSI6ImZhY3Rvcnkvb3NtbzE4OGF4bDlzMDluOTJseXdhMnJtM2dwazd1ZDBueDc0cGg5d2w2ZS91a3J3IiwiYW1vdW50IjoiMzk2OTgwMDAwMDAwMDAwIn0sIndlaWdodCI6IjEwNzM3NDE4MjQwMDAwMDAifSx7InRva2VuIjp7ImRlbm9tIjoidW9zbW8iLCJhbW91bnQiOiI0MDAwMDAwMDAwMDAwMCJ9LCJ3ZWlnaHQiOiIxMDczNzQxODI0MDAwMDAwIn1dLCJ0b3RhbF93ZWlnaHQiOiIyMTQ3NDgzNjQ4MDAwMDAwIn19",
            "eyJwb29sIjp7IkB0eXBlIjoiL29zbW9zaXMuZ2FtbS52MWJldGExLlBvb2wiLCJhZGRyZXNzIjoib3NtbzFhZDRyM3VoNXBkbjVwZ2c1aG5sNnU1dXRmZXFtcHdzdGx2Z3ZnMmgyamR6dHJjbndrcWdzM2hzODV6IiwiaWQiOiI0IiwicG9vbF9wYXJhbXMiOnsic3dhcF9mZWUiOiIwLjAxMDAwMDAwMDAwMDAwMDAwMCIsImV4aXRfZmVlIjoiMC4wMDAwMDAwMDAwMDAwMDAwMDAiLCJzbW9vdGhfd2VpZ2h0X2NoYW5nZV9wYXJhbXMiOm51bGx9LCJmdXR1cmVfcG9vbF9nb3Zlcm5vciI6IiIsInRvdGFsX3NoYXJlcyI6eyJkZW5vbSI6ImdhbW0vcG9vbC80IiwiYW1vdW50IjoiMTAwMDAwMDAwMDAwMDAwMDAwMDAwIn0sInBvb2xfYXNzZXRzIjpbeyJ0b2tlbiI6eyJkZW5vbSI6ImZhY3Rvcnkvb3NtbzFlNjJkMHVzamRwNnJkOTR0bDJwMGc3Z2h4ZGxlZG1jZzI5NGUyOS91YXRvbSIsImFtb3VudCI6IjIzMDQ4ODAwMDAwMDAifSwid2VpZ2h0IjoiMTA3Mzc0MTgyNDAwMDAwMCJ9LHsidG9rZW4iOnsiZGVub20iOiJ1b3NtbyIsImFtb3VudCI6IjQwMDAwMDAwMDAwMDAwIn0sIndlaWdodCI6IjEwNzM3NDE4MjQwMDAwMDAifV0sInRvdGFsX3dlaWdodCI6IjIxNDc0ODM2NDgwMDAwMDAifX0=",
            "eyJwb29sIjp7IkB0eXBlIjoiL29zbW9zaXMuZ2FtbS5wb29sbW9kZWxzLnN0YWJsZXN3YXAudjFiZXRhMS5Qb29sIiwiYWRkcmVzcyI6Im9zbW8xcGprdDkzZzlsaG50Y3B4azZwbjA0eHdhODdnZjIzd3BqZ2hqdWRxbDVwN24yZXh1amg3c3pyZHZ0YyIsImlkIjoiNSIsInBvb2xfcGFyYW1zIjp7InN3YXBfZmVlIjoiMC4wMTAwMDAwMDAwMDAwMDAwMDAiLCJleGl0X2ZlZSI6IjAuMDAwMDAwMDAwMDAwMDAwMDAwIn0sImZ1dHVyZV9wb29sX2dvdmVybm9yIjoib3NtbzFlNjJkMHVzamRwNnJkOTR0bDJwMGc3Z2h4ZGxlZG1jZzI5NGUyOSIsInRvdGFsX3NoYXJlcyI6eyJkZW5vbSI6ImdhbW0vcG9vbC81IiwiYW1vdW50IjoiMTAwMDAwMDAwMDAwMDAwMDAwMDAwIn0sInBvb2xfbGlxdWlkaXR5IjpbeyJkZW5vbSI6ImZhY3Rvcnkvb3NtbzFlNjJkMHVzamRwNnJkOTR0bDJwMGc3Z2h4ZGxlZG1jZzI5NGUyOS91anB5IiwiYW1vdW50IjoiNDA2NTYwMDAwMDAwMDAwMDAwIn0seyJkZW5vbSI6ImZhY3Rvcnkvb3NtbzFlNjJkMHVzamRwNnJkOTR0bDJwMGc3Z2h4ZGxlZG1jZzI5NGUyOS91a3J3IiwiYW1vdW50IjoiMzk2OTgwMDAwMDAwMDAwMDAwMCJ9LHsiZGVub20iOiJmYWN0b3J5L29zbW8xZTYyZDB1c2pkcDZyZDk0dGwycDBnN2doeGRsZWRtY2cyOTRlMjkvdXVzZCIsImFtb3VudCI6IjI5NjAwMDAwMDAwMDAwMDAwMCJ9XSwic2NhbGluZ19mYWN0b3JzIjpbIjEiLCIxIiwiMSJdLCJzY2FsaW5nX2ZhY3Rvcl9jb250cm9sbGVyIjoib3NtbzFlNjJkMHVzamRwNnJkOTR0bDJwMGc3Z2h4ZGxlZG1jZzI5NGUyOSJ9fQ==",
        ];

        pools_raw_enc
            .into_iter()
            .map(|v| -> Result<Box<dyn OsmosisPool>, ContractError> {
                match v {
                    v if v.starts_with(PREFIX_WEIGHTED_POOL) => Ok(Box::new(
                        WeightedPoolResponse::try_from(Binary::from_base64(v)?)?.pool,
                    )),
                    v if v.starts_with(PREFIX_STABLE_POOL) => Ok(Box::new(
                        StablePoolResponse::try_from(Binary::from_base64(v)?)?.pool,
                    )),
                    _ => Err(ContractError::UnsupportedPoolType),
                }
            })
            .collect::<Result<Vec<_>, _>>().unwrap()
    }

    fn mock_portfolio() -> Vec<(String, Decimal)> {
        vec![
            ("factory/osmo1e62d0usjdp6rd94tl2p0g7ghxdledmcg294e29/uusd".to_string(), Decimal::from_str("22.2").unwrap()), 
            ("factory/osmo1e62d0usjdp6rd94tl2p0g7ghxdledmcg294e29/ujpy".to_string(), Decimal::from_str("20.328").unwrap()), 
            ("factory/osmo1e62d0usjdp6rd94tl2p0g7ghxdledmcg294e29/ukrw".to_string(), Decimal::from_str("496.225").unwrap()),
        ]
    }

    #[test]
    fn test_estimate_swap() {
        let pools = mock_pools();
        let portfolio = mock_portfolio();

        let (uusd, uusd_pool) = ("factory/osmo1e62d0usjdp6rd94tl2p0g7ghxdledmcg294e29/uusd".to_string(), 1);
        let (ujpy, ujpy_pool) = ("factory/osmo1e62d0usjdp6rd94tl2p0g7ghxdledmcg294e29/ujpy".to_string(), 2);
        let (ukrw, ukrw_pool) = ("factory/osmo1e62d0usjdp6rd94tl2p0g7ghxdledmcg294e29/ukrw".to_string(), 3);
        let (uatom, uatom_pool) = ("factory/osmo1e62d0usjdp6rd94tl2p0g7ghxdledmcg294e29/uatom".to_string(), 4);
        let stable_pool = 5;

        let input_asset = uatom.clone();
        let swap_infos_compact = SwapInfosCompact(vec![
            SwapInfoCompact {
                key: format!("{uatom},{uusd}"),
                routes: vec![
                    format!("{uatom_pool},{uatom}"),
                    format!("{uusd_pool},uosmo"),
                ],
            },
            SwapInfoCompact {
                key: format!("{uatom},{ujpy}"),
                routes: vec![
                    format!("{uatom_pool},{uatom}"),
                    format!("{ujpy_pool},uosmo"),
                ],
            },
            SwapInfoCompact {
                key: format!("{uatom},{ukrw}"),
                routes: vec![
                    format!("{uatom_pool},{uatom}"),
                    format!("{ukrw_pool},uosmo"),
                ],
            },
            SwapInfoCompact {
                key: format!("{uatom},uosmo"),
                routes: vec![
                    format!("{uatom_pool},{uatom}"),
                    format!("{ujpy_pool},{ukrw}"),
                    format!("{},uosmo",stable_pool),
                ],
            },
        ]);

        let swap_info: Vec<SwapInfo> = swap_infos_compact.into();

        let mut pools_map = pools
            .clone()
            .into_iter()
            .map(|v| (v.get_id(), v))
            .collect::<BTreeMap<_, _>>();


        let token_one_in = portfolio
            .iter()
            .map(|(denom, unit)| coin((Uint128::new(1000000) * *unit).u128(), denom))
            .try_fold(Uint128::zero(), |acc, token_out| {
                let token_in = estimate_in_given_out(
                    input_asset.clone(),
                    token_out,
                    &mut pools_map,
                    &swap_info,
                )?;

                Ok::<_, ContractError>(acc + token_in)
            }).unwrap();

        println!("{}", token_one_in);
    }
}
