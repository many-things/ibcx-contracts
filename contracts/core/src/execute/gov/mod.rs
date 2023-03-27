mod pause;
mod update;

use cosmwasm_std::{DepsMut, Response};
use cosmwasm_std::{Env, MessageInfo};
use ibcx_interface::core::GovMsg;

use crate::{error::ContractError, state::GOV};

pub fn handle_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: GovMsg,
) -> Result<Response, ContractError> {
    use GovMsg::*;

    if info.sender != GOV.load(deps.storage)? {
        return Err(ContractError::Unauthorized {});
    }

    match msg {
        Pause { expires_at } => pause::pause(deps, env, info, expires_at),
        Release {} => pause::release(deps, env, info),

        UpdateGov(new_gov) => update::update_gov(deps, info, new_gov),
        UpdateFeeStrategy(new_fee) => update::update_fee(deps, env, info, new_fee),
        UpdateReserveDenom(new_denom) => update::update_reserve_denom(deps, info, new_denom),
        UpdateTradeInfo {
            denom,
            routes,
            cooldown,
            max_trade_amount,
        } => update::update_trade_info(deps, info, denom, routes, cooldown, max_trade_amount),
    }
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        Addr,
    };

    use crate::test::{SENDER_ABUSER, SENDER_GOV};

    use super::*;

    #[test]
    fn test_handle_msg_check_authority() {
        let mut deps = mock_dependencies();

        let gov = Addr::unchecked(SENDER_GOV);
        GOV.save(deps.as_mut().storage, &gov).unwrap();

        let err = handle_msg(
            deps.as_mut(),
            mock_env(),
            mock_info(SENDER_ABUSER, &[]),
            GovMsg::Release {},
        )
        .unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});
    }
}
