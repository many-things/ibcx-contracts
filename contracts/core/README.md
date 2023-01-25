# IBCX Core

## Instantiation

Here's the example of InstantiateMsg

```json
{
  "gov": "osmo1t3vqry3989lxydpjw445dc8mspw6wgt9wv78a6",
  "denom": "uibcx",
  "reserve_denom": "uosmo",
  "initial_assets": [
    ["uatom", "11.2"],
    ["ujuno", "15.3"],
    ["uosmo", "151.8"]
  ],
  "fee_strategy": {
    "collector": "osmo1t3vqry3989lxydpjw445dc8mspw6wgt9wv78a6",
    "mint": "0.1", // or none
    "burn": "0.3", // or none
    "stream": "0.000000000047529" // or none
  }
}
```

## Features

- Mint / Burn
- Fee mechanism
  - Mint fee
  - Burn fee
  - Streaming fee
- Porfolio rebalancing (lifecycle)
  - Init
  - Trade
  - Finalize
- Governing utilities
  - Pause / Release contract
  - Change governance
  - Update fee strategy
  - Update reserve denom
  - Update trade info

## Queries

- Balance
- Config
- PauseInfo
- Portfolio
- Simulate
  - Mint
  - Burn

## How to test

### Setup localnet

`make localnet-start`

### Deploy core

`beaker wasm deploy ibcx-core --raw '{INSTANTIATE_MSG}'`

### Mint / Burn

```bash

# mint - put portfolio assets into funds
beaker wasm execute ibcx-core --raw '{"mint":{"amount":"100000"}}' --funds "{FUNDS}"

# burn - put uibcx token into funds
beaker wasm execute ibcx-core --raw '{"burn":{}}' --funds "{FUNDS}"

```

### Rebalance

| Prerequisites - liquidity pool for each porfolio assets ([docs](https://docs.osmosis.zone/osmosis-core/modules/gamm/client/docs/create-pool))

```bash

# init rebalance
beaker wasm execute ibcx-core --raw '{
    "rebalance": {
        "init": {
            "manager": "osmo1t3vqry3989lxydpjw445dc8mspw6wgt9wv78a6",
            "deflation": [
                ["ukrw", "0.7"],
                ["ujpy", "1.3"]
            ],
            "inflation": [
                ["uusd", "1"],
                ["ueur", "2"]
            ]
        }
    }
}'

# trade

beaker wasm execute ibcx-core --raw '{
    "rebalance": {
        "trade": {
            "deflate": {
                "denom": "ukrw",
                "amount": "1000",
                "max_amount_in", "1000"
            }
        }
    }
}'

beaker wasm execute ibcx-core --raw '{
    "rebalance": {
        "trade": {
            "inflate": {
                "denom": "uusd",
                "amount": "1000",
                "min_amount_out", "1000"
            }
        }
    }
}'

# finalize

beaker wasm execute ibcx-core --raw '{"rebalance":{"finalize":{}}}'

```
