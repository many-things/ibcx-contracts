/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { Uint128, SwapInfosCompact, Coin, SimulateBurnExactAmountInResponse, SimulateBurnExactAmountOutResponse, SimulateMintExactAmountInResponse, SimulateMintExactAmountOutResponse } from "./Periphery.types";
export interface PeripheryReadOnlyInterface {
    contractAddress: string;
    simulateMintExactAmountIn: ({ coreAddr, inputAsset, swapInfo }: {
        coreAddr: string;
        inputAsset: Coin;
        swapInfo: SwapInfosCompact;
    }) => Promise<SimulateMintExactAmountInResponse>;
    simulateMintExactAmountOut: ({ coreAddr, inputAsset, outputAmount, swapInfo }: {
        coreAddr: string;
        inputAsset: string;
        outputAmount: Uint128;
        swapInfo: SwapInfosCompact;
    }) => Promise<SimulateMintExactAmountOutResponse>;
    simulateBurnExactAmountIn: ({ coreAddr, inputAmount, outputAsset, swapInfo }: {
        coreAddr: string;
        inputAmount: Uint128;
        outputAsset: string;
        swapInfo: SwapInfosCompact;
    }) => Promise<SimulateBurnExactAmountInResponse>;
    simulateBurnExactAmountInV2: ({ coreAddr, inputAmount, outputAsset, swapInfo }: {
        coreAddr: string;
        inputAmount: Uint128;
        outputAsset: string;
        swapInfo: SwapInfosCompact;
    }) => Promise<SimulateBurnExactAmountInResponse>;
    simulateBurnExactAmountOut: ({ coreAddr, outputAsset, swapInfo }: {
        coreAddr: string;
        outputAsset: Coin;
        swapInfo: SwapInfosCompact;
    }) => Promise<SimulateBurnExactAmountOutResponse>;
}
export declare class PeripheryQueryClient implements PeripheryReadOnlyInterface {
    client: CosmWasmClient;
    contractAddress: string;
    constructor(client: CosmWasmClient, contractAddress: string);
    simulateMintExactAmountIn: ({ coreAddr, inputAsset, swapInfo }: {
        coreAddr: string;
        inputAsset: Coin;
        swapInfo: SwapInfosCompact;
    }) => Promise<SimulateMintExactAmountInResponse>;
    simulateMintExactAmountOut: ({ coreAddr, inputAsset, outputAmount, swapInfo }: {
        coreAddr: string;
        inputAsset: string;
        outputAmount: Uint128;
        swapInfo: SwapInfosCompact;
    }) => Promise<SimulateMintExactAmountOutResponse>;
    simulateBurnExactAmountIn: ({ coreAddr, inputAmount, outputAsset, swapInfo }: {
        coreAddr: string;
        inputAmount: Uint128;
        outputAsset: string;
        swapInfo: SwapInfosCompact;
    }) => Promise<SimulateBurnExactAmountInResponse>;
    simulateBurnExactAmountInV2: ({ coreAddr, inputAmount, outputAsset, swapInfo }: {
        coreAddr: string;
        inputAmount: Uint128;
        outputAsset: string;
        swapInfo: SwapInfosCompact;
    }) => Promise<SimulateBurnExactAmountInResponse>;
    simulateBurnExactAmountOut: ({ coreAddr, outputAsset, swapInfo }: {
        coreAddr: string;
        outputAsset: Coin;
        swapInfo: SwapInfosCompact;
    }) => Promise<SimulateBurnExactAmountOutResponse>;
}
export interface PeripheryInterface extends PeripheryReadOnlyInterface {
    contractAddress: string;
    sender: string;
    mintExactAmountIn: ({ coreAddr, inputAsset, minOutputAmount, swapInfo }: {
        coreAddr: string;
        inputAsset: string;
        minOutputAmount: Uint128;
        swapInfo: SwapInfosCompact;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    mintExactAmountOut: ({ coreAddr, inputAsset, outputAmount, swapInfo }: {
        coreAddr: string;
        inputAsset: string;
        outputAmount: Uint128;
        swapInfo: SwapInfosCompact;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    burnExactAmountIn: ({ coreAddr, minOutputAmount, outputAsset, swapInfo }: {
        coreAddr: string;
        minOutputAmount: Uint128;
        outputAsset: string;
        swapInfo: SwapInfosCompact;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    burnExactAmountOut: ({ coreAddr, outputAsset, swapInfo }: {
        coreAddr: string;
        outputAsset: Coin;
        swapInfo: SwapInfosCompact;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    finishOperation: ({ refundAsset, refundTo }: {
        refundAsset: string;
        refundTo: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export declare class PeripheryClient extends PeripheryQueryClient implements PeripheryInterface {
    client: SigningCosmWasmClient;
    sender: string;
    contractAddress: string;
    constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string);
    mintExactAmountIn: ({ coreAddr, inputAsset, minOutputAmount, swapInfo }: {
        coreAddr: string;
        inputAsset: string;
        minOutputAmount: Uint128;
        swapInfo: SwapInfosCompact;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    mintExactAmountOut: ({ coreAddr, inputAsset, outputAmount, swapInfo }: {
        coreAddr: string;
        inputAsset: string;
        outputAmount: Uint128;
        swapInfo: SwapInfosCompact;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    burnExactAmountIn: ({ coreAddr, minOutputAmount, outputAsset, swapInfo }: {
        coreAddr: string;
        minOutputAmount: Uint128;
        outputAsset: string;
        swapInfo: SwapInfosCompact;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    burnExactAmountOut: ({ coreAddr, outputAsset, swapInfo }: {
        coreAddr: string;
        outputAsset: Coin;
        swapInfo: SwapInfosCompact;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    finishOperation: ({ refundAsset, refundTo }: {
        refundAsset: string;
        refundTo: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
//# sourceMappingURL=Periphery.client.d.ts.map