use std::{env::current_dir, fs::remove_dir_all};

use cosmwasm_schema::write_api;

use ibcx_interface::core::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
        migrate: MigrateMsg,
    }

    let mut raw_dir = current_dir().unwrap();
    raw_dir.push("schema");
    remove_dir_all(raw_dir.join("raw")).unwrap();
}
