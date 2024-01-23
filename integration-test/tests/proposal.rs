use cosmwasm_std::{to_json_binary, CosmosMsg, WasmMsg};

use ibcx_interface::core;

#[test]
fn test_proposal() {
    let msgs: Vec<CosmosMsg> = vec![WasmMsg::Migrate {
        contract_addr: "osmo14klwqgkmackvx2tqa0trtg69dmy0nrg4ntq4gjgw2za4734r5seqjqm4gm"
            .to_string(),
        new_code_id: 455,
        msg: to_json_binary(&core::MigrateMsg { force: None }).unwrap(),
    }
    .into()];

    println!("{}", serde_json_wasm::to_string(&msgs).unwrap());
}
