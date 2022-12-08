use cosmwasm_std::{attr, Coin, DepsMut, Env, MessageInfo, Response, Uint128};
use ibc_interface::core::RebalanceMsg;

use crate::{
    error::ContractError,
    state::{get_assets, Rebalance, GOV, LATEST_REBALANCE_ID, REBALANCES},
};

pub fn handle_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: RebalanceMsg,
) -> Result<Response, ContractError> {
    use RebalanceMsg::*;

    match msg {
        Init {
            manager,
            deflation,
            inflation,
        } => init(deps, env, info, manager, deflation, inflation),
        Trade { denom, amount } => trade(deps, env, info, denom, amount),
        Finalize {} => finalize(deps, env, info),
    }
}

fn check_duplication(x: Vec<Coin>, y: Vec<Coin>) -> Result<(), ContractError> {
    let mut y = y.into_iter();
    let f = x
        .into_iter()
        .filter(|xc| y.any(|yc| yc.denom == xc.denom))
        .collect::<Vec<_>>();
    if !f.is_empty() {
        return Err(ContractError::InvalidArgument(format!(
            "duplicated coin: {:?}",
            f
        )));
    }

    Ok(())
}

fn init(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    manager: String,
    deflation: Vec<Coin>,
    inflation: Vec<Coin>,
) -> Result<Response, ContractError> {
    if info.sender != GOV.load(deps.storage)? {
        return Err(ContractError::Unauthorized {});
    }

    let rebalance_id = LATEST_REBALANCE_ID.load(deps.storage)?;
    LATEST_REBALANCE_ID.save(deps.storage, &(rebalance_id + 1))?;

    check_duplication(deflation.clone(), inflation.clone())?;

    let assets = get_assets(deps.storage)?;
    let rebalance = Rebalance {
        manager: deps.api.addr_validate(&manager)?,
        snapshot: assets,
        deflation,
        inflation,
        finalized: false,
    };
    REBALANCES.save(deps.storage, rebalance_id, &rebalance)?;

    let resp = Response::new().add_attributes(vec![
        attr("action", "rebalance_init"),
        attr("executor", info.sender),
        attr("manager", manager),
        attr("rebalance_id", rebalance_id.to_string()),
    ]);

    Ok(resp)
}

fn trade(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    Ok(Default::default())
}

fn finalize(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    Ok(Default::default())
}
