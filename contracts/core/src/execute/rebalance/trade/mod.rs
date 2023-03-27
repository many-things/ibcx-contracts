mod deflate;
mod inflate;

use deflate::{deflate, deflate_reserve};
use inflate::{inflate, inflate_reserve};

use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, Storage, SubMsg};
use ibcx_interface::core::RebalanceTradeMsg;

use crate::{
    error::ContractError,
    execute::fee::realize_streaming_fee,
    state::{Rebalance, LATEST_REBALANCE_ID, REBALANCES, TOKEN},
};

// deflate / inflate the target denom
pub fn trade(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: RebalanceTradeMsg,
) -> Result<Response, ContractError> {
    // realize streaming fee before rebalance to cleanup latest states
    let realize_msg = realize_streaming_fee(deps.storage)?;

    // make message wrapper to place the realize msg at the top
    let wrap = |resp: Result<Response, ContractError>| {
        resp.map(|mut r| {
            r.messages.insert(0, SubMsg::new(realize_msg));
            r
        })
    };

    match msg {
        RebalanceTradeMsg::Deflate {
            denom,
            amount,
            max_amount_in,
        } => {
            let token = TOKEN.load(deps.storage)?;

            let reserve = token.reserve_denom;

            if reserve == denom {
                wrap(deflate_reserve(deps, info, denom, amount))
            } else {
                wrap(deflate(deps, env, info, denom, amount, max_amount_in))
            }
        }

        RebalanceTradeMsg::Inflate {
            denom,
            amount,
            min_amount_out,
        } => {
            let token = TOKEN.load(deps.storage)?;

            let reserve = token.reserve_denom;

            if reserve == denom {
                wrap(inflate_reserve(deps, info, denom, amount))
            } else {
                wrap(inflate(deps, env, info, denom, amount, min_amount_out))
            }
        }
    }
}

// fetch rebalance info & validate rebalance and check if the manager is valid
pub fn get_and_check_rebalance(
    storage: &dyn Storage,
    sender: &Addr,
) -> Result<Rebalance, ContractError> {
    let rebalance_id = LATEST_REBALANCE_ID.load(storage)?;
    let rebalance = REBALANCES.load(storage, rebalance_id)?;
    if &rebalance.manager != sender {
        return Err(ContractError::Unauthorized {});
    }
    if rebalance.finalized {
        return Err(ContractError::RebalanceFinalized {});
    }

    Ok(rebalance)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_env, mock_info},
        DepsMut, Response, Uint128,
    };
    use ibcx_interface::core::RebalanceTradeMsg;

    use crate::{
        error::ContractError,
        execute::rebalance::test::setup,
        test::{mock_dependencies, SENDER_OWNER},
    };

    fn trade(
        deps: DepsMut,
        sender: &str,
        msg: RebalanceTradeMsg,
    ) -> Result<Response, ContractError> {
        super::trade(deps, mock_env(), mock_info(sender, &[]), msg)
    }

    #[test]
    fn test_check_authority() {
        let mut deps = mock_dependencies();

        setup(deps.as_mut().storage, 1, &[], &[], false);

        let err = trade(
            deps.as_mut(),
            SENDER_OWNER,
            RebalanceTradeMsg::Deflate {
                denom: "ukrw".to_string(),
                amount: Uint128::new(100),
                max_amount_in: Uint128::new(100),
            },
        )
        .unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});

        let err = trade(
            deps.as_mut(),
            SENDER_OWNER,
            RebalanceTradeMsg::Inflate {
                denom: "ukrw".to_string(),
                amount: Uint128::new(100),
                min_amount_out: Uint128::new(100),
            },
        )
        .unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});
    }

    #[test]
    fn test_check_rebalnce_finalized() {
        let mut deps = mock_dependencies();

        setup(deps.as_mut().storage, 1, &[], &[], true);

        let err = trade(
            deps.as_mut(),
            "manager",
            RebalanceTradeMsg::Deflate {
                denom: "ukrw".to_string(),
                amount: Uint128::new(100),
                max_amount_in: Uint128::new(100),
            },
        )
        .unwrap_err();
        assert_eq!(err, ContractError::RebalanceFinalized {});

        let err = trade(
            deps.as_mut(),
            "manager",
            RebalanceTradeMsg::Inflate {
                denom: "ukrw".to_string(),
                amount: Uint128::new(100),
                min_amount_out: Uint128::new(100),
            },
        )
        .unwrap_err();
        assert_eq!(err, ContractError::RebalanceFinalized {});
    }
}
