[ibc-contracts](../README.md) / [contracts](contracts.md) / contracts

# Namespace: contracts

[contracts](contracts.md).contracts

## Table of contents

### Variables

- [Airdrop](contracts.contracts-1.md#airdrop)
- [Core](contracts.contracts-1.md#core)
- [Faucet](contracts.contracts-1.md#faucet)
- [Periphery](contracts.contracts-1.md#periphery)

## Variables

### Airdrop

• `Const` **Airdrop**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `AirdropClient` | { `constructor`: (`client`: `SigningCosmWasmClient`, `sender`: `string`, `contractAddress`: `string`) => [`AirdropClient`](contracts.contracts-1.md#airdropclient) ; `client`: `SigningCosmWasmClient` ; `contractAddress`: `string` ; `sender`: `string` ; `checkQualification`: (`__namedParameters`: { `amount`: `string` ; `beneficiary`: `string` ; `id`: `AirdropId` ; `merkleProof`: `string`[]  }) => `Promise`<`boolean`\> ; `claim`: (`__namedParameters`: { `amount`: `string` ; `beneficiary?`: `string` ; `id`: `AirdropId` ; `merkleProof`: `string`[]  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `fund`: (`__namedParameters`: { `id`: `AirdropId`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `getAirdrop`: (`__namedParameters`: { `id`: `AirdropId`  }) => `Promise`<`GetAirdropResponse`\> ; `getClaim`: (`__namedParameters`: { `account`: `string` ; `id`: `AirdropId`  }) => `Promise`<`GetClaimResponse`\> ; `latestAirdropId`: () => `Promise`<`number`\> ; `listAirdrops`: (`__namedParameters`: { `limit?`: `number` ; `order?`: `RangeOrder` ; `startAfter`: `AirdropIdOptional`  }) => `Promise`<`ListAirdropsResponse`\> ; `listClaims`: (`__namedParameters`: { `id`: `AirdropId` ; `limit?`: `number` ; `order?`: `RangeOrder` ; `startAfter?`: `string`  }) => `Promise`<`ListClaimsResponse`\> ; `regsiter`: (`__namedParameters`: { `denom`: `string` ; `label?`: `string` ; `merkleRoot`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\>  } |
| `AirdropQueryClient` | { `constructor`: (`client`: `CosmWasmClient`, `contractAddress`: `string`) => [`AirdropQueryClient`](contracts.contracts-1.md#airdropqueryclient) ; `client`: `CosmWasmClient` ; `contractAddress`: `string` ; `checkQualification`: (`__namedParameters`: { `amount`: `string` ; `beneficiary`: `string` ; `id`: `AirdropId` ; `merkleProof`: `string`[]  }) => `Promise`<`boolean`\> ; `getAirdrop`: (`__namedParameters`: { `id`: `AirdropId`  }) => `Promise`<`GetAirdropResponse`\> ; `getClaim`: (`__namedParameters`: { `account`: `string` ; `id`: `AirdropId`  }) => `Promise`<`GetClaimResponse`\> ; `latestAirdropId`: () => `Promise`<`number`\> ; `listAirdrops`: (`__namedParameters`: { `limit?`: `number` ; `order?`: `RangeOrder` ; `startAfter`: `AirdropIdOptional`  }) => `Promise`<`ListAirdropsResponse`\> ; `listClaims`: (`__namedParameters`: { `id`: `AirdropId` ; `limit?`: `number` ; `order?`: `RangeOrder` ; `startAfter?`: `string`  }) => `Promise`<`ListClaimsResponse`\>  } |

#### Defined in

[contracts/index.ts:16](https://github.com/many-things/ibc-contracts/blob/f35927e/ts/sdk/src/contracts/index.ts#L16)

___

### Core

• `Const` **Core**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `CoreClient` | { `constructor`: (`client`: `SigningCosmWasmClient`, `sender`: `string`, `contractAddress`: `string`) => [`CoreClient`](contracts.contracts-1.md#coreclient) ; `client`: `SigningCosmWasmClient` ; `contractAddress`: `string` ; `sender`: `string` ; `burn`: (`fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `getConfig`: () => `Promise`<`GetConfigResponse`\> ; `getPauseInfo`: () => `Promise`<`GetPauseInfoResponse`\> ; `getPortfolio`: () => `Promise`<`GetPortfolioResponse`\> ; `gov`: (`fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `mint`: (`__namedParameters`: { `amount`: `string` ; `receiver?`: `string` ; `refundTo?`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `rebalance`: (`fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `simulateBurn`: (`__namedParameters`: { `amount`: `string`  }) => `Promise`<`SimulateBurnResponse`\> ; `simulateMint`: (`__namedParameters`: { `amount`: `string` ; `funds`: `Coin`[]  }) => `Promise`<`SimulateMintResponse`\>  } |
| `CoreQueryClient` | { `constructor`: (`client`: `CosmWasmClient`, `contractAddress`: `string`) => [`CoreQueryClient`](contracts.contracts-1.md#corequeryclient) ; `client`: `CosmWasmClient` ; `contractAddress`: `string` ; `getConfig`: () => `Promise`<`GetConfigResponse`\> ; `getPauseInfo`: () => `Promise`<`GetPauseInfoResponse`\> ; `getPortfolio`: () => `Promise`<`GetPortfolioResponse`\> ; `simulateBurn`: (`__namedParameters`: { `amount`: `string`  }) => `Promise`<`SimulateBurnResponse`\> ; `simulateMint`: (`__namedParameters`: { `amount`: `string` ; `funds`: `Coin`[]  }) => `Promise`<`SimulateMintResponse`\>  } |

#### Defined in

[contracts/index.ts:19](https://github.com/many-things/ibc-contracts/blob/f35927e/ts/sdk/src/contracts/index.ts#L19)

___

### Faucet

• `Const` **Faucet**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `FaucetClient` | { `constructor`: (`client`: `SigningCosmWasmClient`, `sender`: `string`, `contractAddress`: `string`) => [`FaucetClient`](contracts.contracts-1.md#faucetclient) ; `client`: `SigningCosmWasmClient` ; `contractAddress`: `string` ; `sender`: `string` ; `checkQualification`: (`__namedParameters`: { `amount`: `string` ; `beneficiary`: `string` ; `id`: `AirdropId` ; `merkleProof`: `string`[]  }) => `Promise`<`boolean`\> ; `claim`: (`__namedParameters`: { `amount`: `string` ; `beneficiary?`: `string` ; `id`: `AirdropId` ; `merkleProof`: `string`[]  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `fund`: (`__namedParameters`: { `id`: `AirdropId`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `getAirdrop`: (`__namedParameters`: { `id`: `AirdropId`  }) => `Promise`<`GetAirdropResponse`\> ; `getClaim`: (`__namedParameters`: { `account`: `string` ; `id`: `AirdropId`  }) => `Promise`<`GetClaimResponse`\> ; `latestAirdropId`: () => `Promise`<`number`\> ; `listAirdrops`: (`__namedParameters`: { `limit?`: `number` ; `order?`: `RangeOrder` ; `startAfter`: `AirdropIdOptional`  }) => `Promise`<`ListAirdropsResponse`\> ; `listClaims`: (`__namedParameters`: { `id`: `AirdropId` ; `limit?`: `number` ; `order?`: `RangeOrder` ; `startAfter?`: `string`  }) => `Promise`<`ListClaimsResponse`\> ; `regsiter`: (`__namedParameters`: { `denom`: `string` ; `label?`: `string` ; `merkleRoot`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\>  } |
| `FaucetQueryClient` | { `constructor`: (`client`: `CosmWasmClient`, `contractAddress`: `string`) => [`FaucetQueryClient`](contracts.contracts-1.md#faucetqueryclient) ; `client`: `CosmWasmClient` ; `contractAddress`: `string` ; `checkQualification`: (`__namedParameters`: { `amount`: `string` ; `beneficiary`: `string` ; `id`: `AirdropId` ; `merkleProof`: `string`[]  }) => `Promise`<`boolean`\> ; `getAirdrop`: (`__namedParameters`: { `id`: `AirdropId`  }) => `Promise`<`GetAirdropResponse`\> ; `getClaim`: (`__namedParameters`: { `account`: `string` ; `id`: `AirdropId`  }) => `Promise`<`GetClaimResponse`\> ; `latestAirdropId`: () => `Promise`<`number`\> ; `listAirdrops`: (`__namedParameters`: { `limit?`: `number` ; `order?`: `RangeOrder` ; `startAfter`: `AirdropIdOptional`  }) => `Promise`<`ListAirdropsResponse`\> ; `listClaims`: (`__namedParameters`: { `id`: `AirdropId` ; `limit?`: `number` ; `order?`: `RangeOrder` ; `startAfter?`: `string`  }) => `Promise`<`ListClaimsResponse`\>  } |

#### Defined in

[contracts/index.ts:22](https://github.com/many-things/ibc-contracts/blob/f35927e/ts/sdk/src/contracts/index.ts#L22)

___

### Periphery

• `Const` **Periphery**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `PeripheryClient` | { `constructor`: (`client`: `SigningCosmWasmClient`, `sender`: `string`, `contractAddress`: `string`) => [`PeripheryClient`](contracts.contracts-1.md#peripheryclient) ; `client`: `SigningCosmWasmClient` ; `contractAddress`: `string` ; `sender`: `string` ; `burnExactAmountIn`: (`__namedParameters`: { `coreAddr`: `string` ; `minOutputAmount`: `string` ; `outputAsset`: `string` ; `swapInfo`: `string`[][]  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `mintExactAmountOut`: (`__namedParameters`: { `coreAddr`: `string` ; `inputAsset`: `string` ; `outputAmount`: `string` ; `swapInfo`: `string`[][]  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\>  } |

#### Defined in

[contracts/index.ts:25](https://github.com/many-things/ibc-contracts/blob/f35927e/ts/sdk/src/contracts/index.ts#L25)
