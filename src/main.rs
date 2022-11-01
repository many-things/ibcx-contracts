use clap::Parser;
use cosmwasm_schema::write_api;
use std::env::current_dir;
use std::fs::create_dir_all;

#[derive(clap::Parser)] // requires `derive` feature
#[clap(author, version, about, long_about = None)]
enum Cli {
    Schema(Schema),
}

#[derive(clap::Args)]
#[clap(long_about = "Generates JSON schema of every contracts' interfaces")]
struct Schema {}

fn main() {
    match Cli::parse() {
        Cli::Schema(_) => {
            let mut out_dir = current_dir().unwrap();
            out_dir.push("schema");
            create_dir_all(&out_dir).unwrap();

            {
                use ibc_interface::core::*;

                write_api! {
                    name: "core", // TODO: refer directly from contract
                    version: "0.1.0", // TODO: refer directly from contract
                    instantiate: InstantiateMsg,
                    query: QueryMsg,
                    execute: ExecuteMsg,
                    migrate: MigrateMsg,
                }
            }
        }
    }
}
