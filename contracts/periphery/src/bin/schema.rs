use cosmwasm_schema::write_api;

use ibc_interface::periphery::{ExecuteMsg, InstantiateMsg, MigrateMsg};
fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        migrate: MigrateMsg,
    }
}
