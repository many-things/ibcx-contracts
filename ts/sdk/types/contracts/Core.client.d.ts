/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { Uint128, GovMsg, RebalanceMsg, Coin, GetConfigResponse, GetFeeResponse, GetPauseInfoResponse, GetPortfolioResponse, SimulateBurnResponse, SimulateMintResponse } from "./Core.types";
export interface CoreReadOnlyInterface {
    contractAddress: string;
    getBalance: ({ account }: {
        account: string;
    }) => Promise<Uint128>;
    getConfig: () => Promise<GetConfigResponse>;
    getFee: ({ time }: {
        time?: number;
    }) => Promise<GetFeeResponse>;
    getPauseInfo: ({ time }: {
        time?: number;
    }) => Promise<GetPauseInfoResponse>;
    getPortfolio: ({ time }: {
        time?: number;
    }) => Promise<GetPortfolioResponse>;
    simulateMint: ({ amount, funds, time }: {
        amount: Uint128;
        funds: Coin[];
        time?: number;
    }) => Promise<SimulateMintResponse>;
    simulateBurn: ({ amount, time }: {
        amount: Uint128;
        time?: number;
    }) => Promise<SimulateBurnResponse>;
}
export declare class CoreQueryClient implements CoreReadOnlyInterface {
    client: CosmWasmClient;
    contractAddress: string;
    constructor(client: CosmWasmClient, contractAddress: string);
    getBalance: ({ account }: {
        account: string;
    }) => Promise<Uint128>;
    getConfig: () => Promise<GetConfigResponse>;
    getFee: ({ time }: {
        time?: number | undefined;
    }) => Promise<GetFeeResponse>;
    getPauseInfo: ({ time }: {
        time?: number | undefined;
    }) => Promise<GetPauseInfoResponse>;
    getPortfolio: ({ time }: {
        time?: number | undefined;
    }) => Promise<GetPortfolioResponse>;
    simulateMint: ({ amount, funds, time }: {
        amount: Uint128;
        funds: Coin[];
        time?: number | undefined;
    }) => Promise<SimulateMintResponse>;
    simulateBurn: ({ amount, time }: {
        amount: Uint128;
        time?: number | undefined;
    }) => Promise<SimulateBurnResponse>;
}
export interface CoreInterface extends CoreReadOnlyInterface {
    contractAddress: string;
    sender: string;
    mint: ({ amount, receiver, refundTo }: {
        amount: Uint128;
        receiver?: string;
        refundTo?: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    burn: ({ redeemTo }: {
        redeemTo?: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    realize: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    gov: (govMsg: GovMsg, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    rebalance: (rebalanceMsg: RebalanceMsg, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export declare class CoreClient extends CoreQueryClient implements CoreInterface {
    client: SigningCosmWasmClient;
    sender: string;
    contractAddress: string;
    constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string);
    mint: ({ amount, receiver, refundTo }: {
        amount: Uint128;
        receiver?: string | undefined;
        refundTo?: string | undefined;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    burn: ({ redeemTo }: {
        redeemTo?: string | undefined;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    realize: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    gov: (govMsg: GovMsg, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    rebalance: (rebalanceMsg: RebalanceMsg, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
//# sourceMappingURL=Core.client.d.ts.map