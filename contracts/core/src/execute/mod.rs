mod fee;
mod gov;
mod rebalance;
mod token;

use cosmwasm_std::{Env, Storage};
pub use fee::{collect_streaming_fee, realize};
pub use gov::handle_msg as handle_gov_msg;
pub use rebalance::handle_msg as handle_rebalance_msg;
pub use token::{burn, mint};

use crate::{error::ContractError, state::CONFIG};

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
