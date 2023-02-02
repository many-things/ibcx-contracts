# Changelog

## v0.1.2

- common
  - [#29](https://github.com/many-things/ibcx-contracts/pull/29) Add force migration rule
- airdrop
  - [#31](https://github.com/many-things/ibcx-contracts/pull/31) Closable airdrop
- core
  - [#25](https://github.com/many-things/ibcx-contracts/pull/25) Make simulation query to return fee-reflected result
  - [#30](https://github.com/many-things/ibcx-contracts/pull/30) Change the type of `funds` in `QueryMsg::SimulateMint`
- periphery
  - [#26](https://github.com/many-things/ibcx-contracts/pull/26) Use `simulate-mint` while executing `ExecuteMsg::MintExactAmountOut`

## v0.1.1

- core
  - [#25](https://github.com/many-things/ibcx-contracts/pull/25) Make every queries to return results that applied streaming fee
  - [#25](https://github.com/many-things/ibcx-contracts/pull/25) Detach query `QueryMsg::GetConfig` <-> `QueryMsg::GetFee`
  - [#26](https://github.com/many-things/ibcx-contracts/pull/25) Regenerate & publish SDK to v0.1.0
