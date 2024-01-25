use std::marker::PhantomData;

use cosmwasm_std::{
    coin,
    testing::{MockApi, MockStorage},
    Empty, OwnedDeps,
};

use osmosis_test_tube::{Module, OsmosisTestApp, Wasm};

use crate::QUERIER_BIN;

pub struct App {
    osmo_app: OsmosisTestApp,
    wasm_querier: String,
}

impl Default for App {
    fn default() -> Self {
        Self::new(OsmosisTestApp::new())
    }
}

impl App {
    pub fn new(osmo_app: OsmosisTestApp) -> Self {
        let deployer = osmo_app
            .init_account(&[coin(100_000_000_000, "uosmo")])
            .unwrap();

        let wasm = Wasm::new(&osmo_app);

        let store_resp = wasm.store_code(QUERIER_BIN, None, &deployer).unwrap();

        let init_resp = wasm
            .instantiate(
                store_resp.data.code_id,
                &Empty {},
                None,
                None,
                &[],
                &deployer,
            )
            .unwrap();

        Self {
            osmo_app,
            wasm_querier: init_resp.data.address,
        }
    }

    pub fn inner(&self) -> &OsmosisTestApp {
        &self.osmo_app
    }

    pub fn wasm_querier(&self) -> &str {
        &self.wasm_querier
    }

    pub fn deps(&self) -> OwnedDeps<MockStorage, MockApi, crate::Querier> {
        OwnedDeps {
            storage: MockStorage::default(),
            api: MockApi::default(),
            querier: crate::Querier::new(self),
            custom_query_type: PhantomData::<Empty>,
        }
    }
}
