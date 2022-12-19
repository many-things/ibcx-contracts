/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { SwapRoutes, Coin, AmountResponse } from "./Compat.types";
export interface CompatReadOnlyInterface {
    contractAddress: string;
    estimateSwapExactAmountIn: ({ amount, routes, sender }: {
        amount: Coin;
        routes: SwapRoutes;
        sender: string;
    }) => Promise<AmountResponse>;
    estimateSwapExactAmountOut: ({ amount, routes, sender }: {
        amount: Coin;
        routes: SwapRoutes;
        sender: string;
    }) => Promise<AmountResponse>;
}
export declare class CompatQueryClient implements CompatReadOnlyInterface {
    client: CosmWasmClient;
    contractAddress: string;
    constructor(client: CosmWasmClient, contractAddress: string);
    estimateSwapExactAmountIn: ({ amount, routes, sender }: {
        amount: Coin;
        routes: SwapRoutes;
        sender: string;
    }) => Promise<AmountResponse>;
    estimateSwapExactAmountOut: ({ amount, routes, sender }: {
        amount: Coin;
        routes: SwapRoutes;
        sender: string;
    }) => Promise<AmountResponse>;
}
export interface CompatInterface extends CompatReadOnlyInterface {
    contractAddress: string;
    sender: string;
    switchQueryMode: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export declare class CompatClient extends CompatQueryClient implements CompatInterface {
    client: SigningCosmWasmClient;
    sender: string;
    contractAddress: string;
    constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string);
    switchQueryMode: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
//# sourceMappingURL=Compat.client.d.ts.map