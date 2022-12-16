use osmo_bindings::{OsmosisMsg, OsmosisQuery};

// Type aliases
pub type Response = cosmwasm_std::Response<OsmosisMsg>;
pub type SubMsg = cosmwasm_std::SubMsg<OsmosisMsg>;
pub type CosmosMsg = cosmwasm_std::CosmosMsg<OsmosisMsg>;
pub type Deps<'a> = cosmwasm_std::Deps<'a, OsmosisQuery>;
pub type DepsMut<'a> = cosmwasm_std::DepsMut<'a, OsmosisQuery>;
pub type QuerierWrapper<'a> = cosmwasm_std::QuerierWrapper<'a, OsmosisQuery>;
