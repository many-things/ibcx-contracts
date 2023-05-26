use cosmwasm_schema::cw_serde;
use osmosis_std::types::osmosis::gamm;

use super::OsmosisPool;

#[cw_serde]
pub struct WeightedPool(gamm::v1beta1::Pool);

impl From<gamm::v1beta1::Pool> for WeightedPool {
    fn from(v: gamm::v1beta1::Pool) -> Self {
        Self(v)
    }
}

impl OsmosisPool for WeightedPool {
    fn swap_exact_amount_in(&mut self) {
        todo!()
    }

    fn swap_exact_amount_out(&mut self) {
        todo!()
    }
}
