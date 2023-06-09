use cosmwasm_std::{coin, CosmosMsg};
use osmosis_test_tube::osmosis_std::{
    shim::{Duration, Timestamp},
    types::osmosis::{
        incentives::MsgCreateGauge,
        lockup::{LockQueryType, QueryCondition},
    },
};

/**
{
  "body": {
    "messages": [
      {
        "@type": "/osmosis.incentives.MsgCreateGauge",
        "is_perpetual": false,
        "owner": "osmo14n3a65fnqz9jve85l23al6m3pjugf0atvrfqh5",
        "distribute_to": {
          "lock_query_type": "ByDuration",
          "denom": "gamm/pool/1013",
          "duration": "1209600s",
          "timestamp": "1970-01-01T00:00:00Z"
        },
        "coins": [{ "denom": "uion", "amount": "10000" }],
        "start_time": "2023-04-13T19:00:00Z",
        "num_epochs_paid_over": "120"
      }
    ],
    "memo": "",
    "timeout_height": "0",
    "extension_options": [],
    "non_critical_extension_options": []
  },
  "auth_info": {
    "signer_infos": [],
    "fee": {
      "amount": [{ "denom": "uosmo", "amount": "450" }],
      "gas_limit": "179795",
      "payer": "",
      "granter": ""
    }
  },
  "signatures": []
}
 */
#[test]
fn test_cosmos_msg_to_json() {
    let msgs: Vec<CosmosMsg> = vec![MsgCreateGauge {
        is_perpetual: false,
        owner: "osmo1k8re7jwz6rnnwrktnejdwkwnncte7ek7gt29gvnl3sdrg9mtnqkse6nmqm".to_string(),
        distribute_to: Some(QueryCondition {
            lock_query_type: LockQueryType::ByDuration.into(),
            denom: "gamm/pool/1013".to_string(),
            duration: Some(Duration {
                seconds: 1209600,
                nanos: 0,
            }),
            timestamp: None,
        }),
        coins: vec![coin(70980000, "uion").into()],
        start_time: Some(Timestamp {
            seconds: 1684497600,
            nanos: 0,
        }),
        num_epochs_paid_over: 120,
    }
    .into()];

    println!("{}", serde_json::to_string_pretty(&msgs).unwrap());
}
