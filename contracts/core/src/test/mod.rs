pub mod mock;
pub mod querier;

use std::marker::PhantomData;

use cosmwasm_std::{
    testing::{MockApi, MockQuerier, MockStorage},
    Empty, OwnedDeps,
};

use self::{mock::StargateQuerier, querier::CoreQuerier};

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
