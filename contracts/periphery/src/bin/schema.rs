use cosmwasm_schema::write_api;

use ibc_interface::{
    airdrop::MigrateMsg,
    periphery::{ExecuteMsg, InstantiateMsg},
};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        migrate: MigrateMsg,
    }
}
