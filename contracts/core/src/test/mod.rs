pub mod mock;
pub mod querier;

use std::{marker::PhantomData, str::FromStr};

use cosmwasm_std::{
    testing::{mock_env, MockApi, MockQuerier, MockStorage},
    Addr, Decimal, Empty, OwnedDeps, Storage, Uint128,
};
use ibcx_interface::types::SwapRoutes;

use crate::state::{self, ASSETS};

use self::{mock::StargateQuerier, querier::CoreQuerier};

pub const SENDER_OWNER: &str = "owner";
pub const SENDER_GOV: &str = "gov";
pub const SENDER_ABUSER: &str = "abuser";
pub const SENDER_VALID: &str = "osmo10yaagy0faggta0085hkzr3ckq7p7z9996nrn0m";

pub const DENOM_DEFAULT: &str = "uibcx";
pub const DENOM_RESERVE: &str = "uosmo";

pub fn mock_dependencies() -> OwnedDeps<MockStorage, MockApi, CoreQuerier<'static>, Empty> {
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: CoreQuerier {
            mq: MockQuerier::default(),
            stargate: StargateQuerier::default(),
        },
        custom_query_type: PhantomData,
    }
}

pub fn to_assets(assets: &[(&str, &str)]) -> Vec<(String, Decimal)> {
    assets
        .iter()
        .map(|(k, v)| (k.to_string(), Decimal::from_str(v).unwrap()))
        .collect()
}

pub fn register_assets(storage: &mut dyn Storage, assets: &[(&str, &str)]) {
    for (denom, unit) in assets {
        ASSETS
            .save(
                storage,
                denom.to_string(),
                &Decimal::from_str(unit).unwrap(),
            )
            .unwrap();
    }
}

pub fn default_fee() -> state::Fee {
    state::Fee {
        collector: Addr::unchecked("collector"),
        mint: Default::default(),
        burn: Default::default(),
        stream: Default::default(),
        stream_last_collected_at: Default::default(),
    }
}

pub fn default_token() -> state::Token {
    state::Token {
        denom: DENOM_DEFAULT.to_string(),
        reserve_denom: DENOM_RESERVE.to_string(),
        total_supply: Uint128::new(100000),
    }
}

pub fn default_trade_info() -> state::TradeInfo {
    state::TradeInfo {
        routes: SwapRoutes(vec![]),
        cooldown: 86400,
        max_trade_amount: Uint128::new(100000),
        last_traded_at: Some(mock_env().block.time.seconds()),
    }
}
