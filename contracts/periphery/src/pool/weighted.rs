use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;
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
    fn swap_exact_amount_in(
        &mut self,
        input_amount: cosmwasm_std::Coin,
        output_denom: String,
        min_output_amount: cosmwasm_std::Uint128,
        spread_factor: Decimal,
    ) {
        todo!()
    }

    fn swap_exact_amount_out(
        &mut self,
        input_denom: String,
        max_input_amount: cosmwasm_std::Uint128,
        output_amount: cosmwasm_std::Coin,
        spread_factor: Decimal,
    ) {
        todo!()
    }
}
