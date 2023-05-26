use cosmwasm_schema::cw_serde;
use osmosis_std::types::osmosis::gamm::poolmodels::stableswap;

use super::OsmosisPool;

#[cw_serde]
pub struct StablePool(stableswap::v1beta1::Pool);

impl From<stableswap::v1beta1::Pool> for StablePool {
    fn from(v: stableswap::v1beta1::Pool) -> Self {
        Self(v)
    }
}

impl OsmosisPool for StablePool {
    fn swap_exact_amount_in(&mut self) {
        todo!()
    }

    fn swap_exact_amount_out(&mut self) {
        todo!()
    }
}
