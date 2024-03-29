/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
import { MsgExecuteContractEncodeObject } from "cosmwasm";
import { Uint128, SwapInfosCompact, Coin } from "./Periphery.types";
export interface PeripheryMessage {
    contractAddress: string;
    sender: string;
    mintExactAmountIn: ({ coreAddr, inputAsset, minOutputAmount, swapInfo }: {
        coreAddr: string;
        inputAsset: string;
        minOutputAmount: Uint128;
        swapInfo: SwapInfosCompact;
    }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
    mintExactAmountOut: ({ coreAddr, inputAsset, outputAmount, swapInfo }: {
        coreAddr: string;
        inputAsset: string;
        outputAmount: Uint128;
        swapInfo: SwapInfosCompact;
    }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
    burnExactAmountIn: ({ coreAddr, minOutputAmount, outputAsset, swapInfo }: {
        coreAddr: string;
        minOutputAmount: Uint128;
        outputAsset: string;
        swapInfo: SwapInfosCompact;
    }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
    burnExactAmountOut: ({ coreAddr, outputAsset, swapInfo }: {
        coreAddr: string;
        outputAsset: Coin;
        swapInfo: SwapInfosCompact;
    }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
    finishOperation: ({ refundAsset, refundTo }: {
        refundAsset: string;
        refundTo: string;
    }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
export declare class PeripheryMessageComposer implements PeripheryMessage {
    sender: string;
    contractAddress: string;
    constructor(sender: string, contractAddress: string);
    mintExactAmountIn: ({ coreAddr, inputAsset, minOutputAmount, swapInfo }: {
        coreAddr: string;
        inputAsset: string;
        minOutputAmount: Uint128;
        swapInfo: SwapInfosCompact;
    }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
    mintExactAmountOut: ({ coreAddr, inputAsset, outputAmount, swapInfo }: {
        coreAddr: string;
        inputAsset: string;
        outputAmount: Uint128;
        swapInfo: SwapInfosCompact;
    }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
    burnExactAmountIn: ({ coreAddr, minOutputAmount, outputAsset, swapInfo }: {
        coreAddr: string;
        minOutputAmount: Uint128;
        outputAsset: string;
        swapInfo: SwapInfosCompact;
    }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
    burnExactAmountOut: ({ coreAddr, outputAsset, swapInfo }: {
        coreAddr: string;
        outputAsset: Coin;
        swapInfo: SwapInfosCompact;
    }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
    finishOperation: ({ refundAsset, refundTo }: {
        refundAsset: string;
        refundTo: string;
    }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
//# sourceMappingURL=Periphery.message-composer.d.ts.map