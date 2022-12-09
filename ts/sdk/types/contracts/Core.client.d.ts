/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { Uint128, Coin, GetConfigResponse, GetPauseInfoResponse, GetPortfolioResponse } from "./Core.types";
export interface CoreReadOnlyInterface {
    contractAddress: string;
    getConfig: () => Promise<GetConfigResponse>;
    getPauseInfo: () => Promise<GetPauseInfoResponse>;
    getPortfolio: () => Promise<GetPortfolioResponse>;
}
export declare class CoreQueryClient implements CoreReadOnlyInterface {
    client: CosmWasmClient;
    contractAddress: string;
    constructor(client: CosmWasmClient, contractAddress: string);
    getConfig: () => Promise<GetConfigResponse>;
    getPauseInfo: () => Promise<GetPauseInfoResponse>;
    getPortfolio: () => Promise<GetPortfolioResponse>;
}
export interface CoreInterface extends CoreReadOnlyInterface {
    contractAddress: string;
    sender: string;
    mint: ({ amount, receiver }: {
        amount: Uint128;
        receiver?: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    burn: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    gov: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    rebalance: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export declare class CoreClient extends CoreQueryClient implements CoreInterface {
    client: SigningCosmWasmClient;
    sender: string;
    contractAddress: string;
    constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string);
    mint: ({ amount, receiver }: {
        amount: Uint128;
        receiver?: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    burn: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    gov: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    rebalance: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
//# sourceMappingURL=Core.client.d.ts.map