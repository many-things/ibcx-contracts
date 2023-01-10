[ibc-contracts](../README.md) / [contracts](contracts.md) / contracts

# Namespace: contracts

[contracts](contracts.md).contracts

## Table of contents

### Variables

- [Airdrop](contracts.contracts-1.md#airdrop)
- [Compat](contracts.contracts-1.md#compat)
- [Core](contracts.contracts-1.md#core)
- [Faucet](contracts.contracts-1.md#faucet)
- [Periphery](contracts.contracts-1.md#periphery)

## Variables

### Airdrop

• `Const` **Airdrop**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `AirdropClient` | { `constructor`: (`client`: `SigningCosmWasmClient`, `sender`: `string`, `contractAddress`: `string`) => [`AirdropClient`](contracts.contracts-1.md#airdropclient) ; `client`: `SigningCosmWasmClient` ; `contractAddress`: `string` ; `sender`: `string` ; `checkQualification`: (`__namedParameters`: { `amount`: `string` ; `beneficiary?`: `string` ; `claimProof?`: `string` ; `id`: `AirdropId` ; `merkleProof`: `string`[]  }) => `Promise`<`boolean`\> ; `claim`: (`__namedParameters`: { `amount`: `string` ; `beneficiary?`: `string` ; `claimProof?`: `string` ; `id`: `AirdropId` ; `merkleProof`: `string`[]  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `fund`: (`__namedParameters`: { `id`: `AirdropId`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `getAirdrop`: (`__namedParameters`: { `id`: `AirdropId`  }) => `Promise`<`GetAirdropResponse`\> ; `getClaim`: (`__namedParameters`: { `account`: `string` ; `id`: `AirdropId`  }) => `Promise`<`GetClaimResponse`\> ; `latestAirdropId`: () => `Promise`<`number`\> ; `listAirdrops`: (`__namedParameters`: { `limit?`: `number` ; `order?`: `RangeOrder` ; `startAfter`: `AirdropIdOptional`  }) => `Promise`<`ListAirdropsResponse`\> ; `listClaims`: (`__namedParameters`: { `id`: `AirdropId` ; `limit?`: `number` ; `order?`: `RangeOrder` ; `startAfter?`: `string`  }) => `Promise`<`ListClaimsResponse`\> ; `regsiter`: (`__namedParameters`: { `bearer?`: `boolean` ; `denom`: `string` ; `label?`: `string` ; `merkleRoot`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\>  } |
| `AirdropQueryClient` | { `constructor`: (`client`: `CosmWasmClient`, `contractAddress`: `string`) => [`AirdropQueryClient`](contracts.contracts-1.md#airdropqueryclient) ; `client`: `CosmWasmClient` ; `contractAddress`: `string` ; `checkQualification`: (`__namedParameters`: { `amount`: `string` ; `beneficiary?`: `string` ; `claimProof?`: `string` ; `id`: `AirdropId` ; `merkleProof`: `string`[]  }) => `Promise`<`boolean`\> ; `getAirdrop`: (`__namedParameters`: { `id`: `AirdropId`  }) => `Promise`<`GetAirdropResponse`\> ; `getClaim`: (`__namedParameters`: { `account`: `string` ; `id`: `AirdropId`  }) => `Promise`<`GetClaimResponse`\> ; `latestAirdropId`: () => `Promise`<`number`\> ; `listAirdrops`: (`__namedParameters`: { `limit?`: `number` ; `order?`: `RangeOrder` ; `startAfter`: `AirdropIdOptional`  }) => `Promise`<`ListAirdropsResponse`\> ; `listClaims`: (`__namedParameters`: { `id`: `AirdropId` ; `limit?`: `number` ; `order?`: `RangeOrder` ; `startAfter?`: `string`  }) => `Promise`<`ListClaimsResponse`\>  } |

#### Defined in

[contracts/index.ts:18](https://github.com/many-things/ibc-contracts/blob/63a64ef/ts/sdk/src/contracts/index.ts#L18)

___

### Compat

• `Const` **Compat**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `CompatClient` | { `constructor`: (`client`: `SigningCosmWasmClient`, `sender`: `string`, `contractAddress`: `string`) => [`CompatClient`](contracts.contracts-1.md#compatclient) ; `client`: `SigningCosmWasmClient` ; `contractAddress`: `string` ; `sender`: `string` ; `estimateSwapExactAmountIn`: (`__namedParameters`: { `amount`: `Coin` ; `mode?`: `QueryMode` ; `routes`: `SwapRoutes` ; `sender`: `string`  }) => `Promise`<`string`\> ; `estimateSwapExactAmountOut`: (`__namedParameters`: { `amount`: `Coin` ; `mode?`: `QueryMode` ; `routes`: `SwapRoutes` ; `sender`: `string`  }) => `Promise`<`string`\> ; `queryMode`: () => `Promise`<`QueryModeResponse`\> ; `switchQueryMode`: (`fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\>  } |
| `CompatQueryClient` | { `constructor`: (`client`: `CosmWasmClient`, `contractAddress`: `string`) => [`CompatQueryClient`](contracts.contracts-1.md#compatqueryclient) ; `client`: `CosmWasmClient` ; `contractAddress`: `string` ; `estimateSwapExactAmountIn`: (`__namedParameters`: { `amount`: `Coin` ; `mode?`: `QueryMode` ; `routes`: `SwapRoutes` ; `sender`: `string`  }) => `Promise`<`string`\> ; `estimateSwapExactAmountOut`: (`__namedParameters`: { `amount`: `Coin` ; `mode?`: `QueryMode` ; `routes`: `SwapRoutes` ; `sender`: `string`  }) => `Promise`<`string`\> ; `queryMode`: () => `Promise`<`QueryModeResponse`\>  } |

#### Defined in

[contracts/index.ts:21](https://github.com/many-things/ibc-contracts/blob/63a64ef/ts/sdk/src/contracts/index.ts#L21)

___

### Core

• `Const` **Core**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `CoreClient` | { `constructor`: (`client`: `SigningCosmWasmClient`, `sender`: `string`, `contractAddress`: `string`) => [`CoreClient`](contracts.contracts-1.md#coreclient) ; `client`: `SigningCosmWasmClient` ; `contractAddress`: `string` ; `sender`: `string` ; `burn`: (`__namedParameters`: { `redeemTo?`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `getBalance`: (`__namedParameters`: { `account`: `string`  }) => `Promise`<`string`\> ; `getConfig`: () => `Promise`<`GetConfigResponse`\> ; `getPauseInfo`: () => `Promise`<`GetPauseInfoResponse`\> ; `getPortfolio`: () => `Promise`<`GetPortfolioResponse`\> ; `gov`: (`fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `mint`: (`__namedParameters`: { `amount`: `string` ; `receiver?`: `string` ; `refundTo?`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `rebalance`: (`fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `simulateBurn`: (`__namedParameters`: { `amount`: `string`  }) => `Promise`<`SimulateBurnResponse`\> ; `simulateMint`: (`__namedParameters`: { `amount`: `string` ; `funds`: `Coin`[]  }) => `Promise`<`SimulateMintResponse`\>  } |
| `CoreQueryClient` | { `constructor`: (`client`: `CosmWasmClient`, `contractAddress`: `string`) => [`CoreQueryClient`](contracts.contracts-1.md#corequeryclient) ; `client`: `CosmWasmClient` ; `contractAddress`: `string` ; `getBalance`: (`__namedParameters`: { `account`: `string`  }) => `Promise`<`string`\> ; `getConfig`: () => `Promise`<`GetConfigResponse`\> ; `getPauseInfo`: () => `Promise`<`GetPauseInfoResponse`\> ; `getPortfolio`: () => `Promise`<`GetPortfolioResponse`\> ; `simulateBurn`: (`__namedParameters`: { `amount`: `string`  }) => `Promise`<`SimulateBurnResponse`\> ; `simulateMint`: (`__namedParameters`: { `amount`: `string` ; `funds`: `Coin`[]  }) => `Promise`<`SimulateMintResponse`\>  } |

#### Defined in

[contracts/index.ts:24](https://github.com/many-things/ibc-contracts/blob/63a64ef/ts/sdk/src/contracts/index.ts#L24)

___

### Faucet

• `Const` **Faucet**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `FaucetClient` | { `constructor`: (`client`: `SigningCosmWasmClient`, `sender`: `string`, `contractAddress`: `string`) => [`FaucetClient`](contracts.contracts-1.md#faucetclient) ; `client`: `SigningCosmWasmClient` ; `contractAddress`: `string` ; `sender`: `string` ; `block`: (`__namedParameters`: { `action`: `Action` ; `denom`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `burn`: (`__namedParameters`: { `denom`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `create`: (`__namedParameters`: { `config`: `TokenCreationConfig` ; `denom`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `getLastTokenId`: () => `Promise`<`number`\> ; `getRole`: (`__namedParameters`: { `account`: `string` ; `denom`: `string`  }) => `Promise`<`GetRoleResponse`\> ; `getToken`: (`__namedParameters`: { `denom`: `string`  }) => `Promise`<`GetTokenResponse`\> ; `grant`: (`__namedParameters`: { `action`: `Action` ; `denom`: `string` ; `grantee`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `listAliases`: (`__namedParameters`: { `limit?`: `number` ; `order?`: `RangeOrder` ; `startAfter?`: `string`  }) => `Promise`<`ListAliasesResponse`\> ; `listRoles`: (`__namedParameters`: { `denom`: `string` ; `limit?`: `number` ; `order?`: `RangeOrder` ; `startAfter?`: `string`[][]  }) => `Promise`<`ListRolesResponse`\> ; `listTokens`: (`__namedParameters`: { `limit?`: `number` ; `order?`: `RangeOrder` ; `startAfter?`: `number`  }) => `Promise`<`ListTokensResponse`\> ; `mint`: (`__namedParameters`: { `amount`: `string` ; `denom`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `release`: (`__namedParameters`: { `action`: `Action` ; `denom`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `revoke`: (`__namedParameters`: { `action`: `Action` ; `denom`: `string` ; `revokee`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\>  } |
| `FaucetQueryClient` | { `constructor`: (`client`: `CosmWasmClient`, `contractAddress`: `string`) => [`FaucetQueryClient`](contracts.contracts-1.md#faucetqueryclient) ; `client`: `CosmWasmClient` ; `contractAddress`: `string` ; `getLastTokenId`: () => `Promise`<`number`\> ; `getRole`: (`__namedParameters`: { `account`: `string` ; `denom`: `string`  }) => `Promise`<`GetRoleResponse`\> ; `getToken`: (`__namedParameters`: { `denom`: `string`  }) => `Promise`<`GetTokenResponse`\> ; `listAliases`: (`__namedParameters`: { `limit?`: `number` ; `order?`: `RangeOrder` ; `startAfter?`: `string`  }) => `Promise`<`ListAliasesResponse`\> ; `listRoles`: (`__namedParameters`: { `denom`: `string` ; `limit?`: `number` ; `order?`: `RangeOrder` ; `startAfter?`: `string`[][]  }) => `Promise`<`ListRolesResponse`\> ; `listTokens`: (`__namedParameters`: { `limit?`: `number` ; `order?`: `RangeOrder` ; `startAfter?`: `number`  }) => `Promise`<`ListTokensResponse`\>  } |

#### Defined in

[contracts/index.ts:27](https://github.com/many-things/ibc-contracts/blob/63a64ef/ts/sdk/src/contracts/index.ts#L27)

___

### Periphery

• `Const` **Periphery**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `PeripheryClient` | { `constructor`: (`client`: `SigningCosmWasmClient`, `sender`: `string`, `contractAddress`: `string`) => [`PeripheryClient`](contracts.contracts-1.md#peripheryclient) ; `client`: `SigningCosmWasmClient` ; `contractAddress`: `string` ; `sender`: `string` ; `burnExactAmountIn`: (`__namedParameters`: { `coreAddr`: `string` ; `minOutputAmount`: `string` ; `outputAsset`: `string` ; `swapInfo`: `RouteKey`[][]  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `mintExactAmountOut`: (`__namedParameters`: { `coreAddr`: `string` ; `inputAsset`: `string` ; `outputAmount`: `string` ; `swapInfo`: `RouteKey`[][]  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\>  } |

#### Defined in

[contracts/index.ts:30](https://github.com/many-things/ibc-contracts/blob/63a64ef/ts/sdk/src/contracts/index.ts#L30)
