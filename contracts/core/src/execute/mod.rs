mod fee;
mod gov;
mod rebalance;
mod token;

use cosmwasm_std::{Env, Storage};
pub use fee::{collect_streaming_fee, realize};
pub use gov::handle_msg as handle_gov_msg;
pub use rebalance::handle_msg as handle_rebalance_msg;
pub use token::{burn, mint};

use crate::{
    error::ContractError,
    state::{CONFIG, LATEST_REBALANCE_ID, REBALANCES},
};

pub fn assert_paused_and_update(
    storage: &mut dyn Storage,
    env: &Env,
    expect: bool,
) -> Result<(), ContractError> {
    let mut config = CONFIG.load(storage)?;

    let refreshed = config.paused.refresh(&env)?;
    if expect {
        refreshed = refreshed.assert_paused()?;
    } else {
        refreshed = refreshed.assert_not_paused()?;
    }

    config.paused = refreshed;

    CONFIG.save(storage, &config)?;

    Ok(())
}

// expect | finalized
//  true  |  true -> no (expected there's an ongoing rebalance, but latest rebalance has been finalized)
//  true  | false -> ok
//  false |  true -> ok
//  false | false -> no (expected there's no ongoing rebalance, but latest rebalance has not been finalized)
pub fn assert_ongoing_rebalance(storage: &dyn Storage, expect: bool) -> Result<(), ContractError> {
    let rebalance_id = LATEST_REBALANCE_ID.load(storage)?;
    let rebalance = REBALANCES.load(storage, rebalance_id)?;

    if rebalance.finalized == expect {
        if expect {
            return Err(ContractError::RebalanceFinalized {});
        } else {
            return Err(ContractError::RebalanceNotFinalized {});
        }
    }

    Ok(())
}
