use cosmwasm_schema::write_api;

use ibcx_interface::periphery::{ExecuteMsg, InstantiateMsg, MigrateMsg};
fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        migrate: MigrateMsg,
    }
}
