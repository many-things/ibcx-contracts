mod finalize;
mod init;
mod trade;

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use ibcx_interface::core::RebalanceMsg;

use crate::error::ContractError;

use finalize::finalize;
use init::init;
use trade::trade;

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
        } => init(deps, info, manager, deflation, inflation),
        Trade(msg) => trade(deps, env, info, msg),
        Finalize {} => finalize(deps, env, info),
    }
}

#[cfg(test)]
mod test {

    use cosmwasm_std::{Addr, Storage};

    use crate::state::Rebalance;
    use crate::{state::Token, test::default_fee};
    use crate::{state::FEE, test::default_token};
    use crate::{
        state::{LATEST_REBALANCE_ID, REBALANCES, TOKEN},
        test::to_units,
    };

    pub fn setup(
        storage: &mut dyn Storage,
        id: u64,
        deflation: &[(&str, &str)],
        inflation: &[(&str, &str)],
        finalized: bool,
    ) -> (Rebalance, Token) {
        let rebalance = Rebalance {
            manager: Addr::unchecked("manager"),
            deflation: to_units(deflation),
            inflation: to_units(inflation),
            finalized,
        };

        let token = default_token();

        LATEST_REBALANCE_ID.save(storage, &id).unwrap();
        REBALANCES.save(storage, id, &rebalance).unwrap();
        TOKEN.save(storage, &token).unwrap();
        FEE.save(storage, &default_fee()).unwrap();

        (rebalance, token)
    }
}
