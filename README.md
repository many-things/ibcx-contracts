# ibcx-contracts

[![test](https://github.com/many-things/ibcx-contracts/actions/workflows/tester.yaml/badge.svg)](https://github.com/many-things/ibcx-contracts/actions/workflows/tester.yaml) [![lint](https://github.com/many-things/ibcx-contracts/actions/workflows/linter.yaml/badge.svg)](https://github.com/many-things/ibcx-contracts/actions/workflows/linter.yaml) [![codecov](https://codecov.io/gh/many-things/ibcx-contracts/branch/main/graph/badge.svg?token=NWZGJ8MBHE)](https://codecov.io/gh/many-things/ibcx-contracts) ![npm](https://img.shields.io/npm/v/@many-things/ibcx-contracts-sdk)

## Interchain Index Token Protocol

A fully-collateralized index token backed by a cap-weighted basket of Cosmos ecosystem coins.

## Components

| contract  | description                                  | path                                                    |
| --------- | -------------------------------------------- | ------------------------------------------------------- |
| Airdrop   | General contract manage airdrops             | [/contracts/airdrop](./contracts/airdrop/README.md)     |
| Core      | IBCX core contract                           | [/contracts/core](./contracts/core/README.md)           |
| Faucet    | RBAC frontend contract for `/x/tokenfactory` | [/contracts/faucet](./contracts/faucet/README.md)       |
| Periphery | Helper contract for core contract            | [/contracts/periphery](./contracts/periphery/README.md) |

## Utils

| name  | description                   | path                    |
| ----- | ----------------------------- | ----------------------- |
| SDK   | Auto generated Typescript SDK | [/ts/sdk](./ts/sdk)     |
| Tools | Merkle tree generator         | [/ts/tools](./ts/tools) |

## Deployments

Refer [state.json](./.beaker/state.json)
