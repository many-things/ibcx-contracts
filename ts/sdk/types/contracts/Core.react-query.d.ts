/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
import { UseQueryOptions } from "@tanstack/react-query";
import { Uint128, RangeOrder, Coin, GetConfigResponse, GetFeeResponse, GetPortfolioResponse, GetRebalanceResponse, GetTradeInfoResponse, ListTradeInfoResponse, SimulateBurnResponse, SimulateMintResponse } from "./Core.types";
import { CoreQueryClient } from "./Core.client";
export interface CoreReactQuery<TResponse, TData = TResponse> {
    client: CoreQueryClient;
    options?: Omit<UseQueryOptions<TResponse, Error, TData>, "'queryKey' | 'queryFn' | 'initialData'"> & {
        initialData?: undefined;
    };
}
export interface CoreSimulateBurnQuery<TData> extends CoreReactQuery<SimulateBurnResponse, TData> {
    args: {
        amount: Uint128;
        time?: number;
    };
}
export declare function useCoreSimulateBurnQuery<TData = SimulateBurnResponse>({ client, args, options }: CoreSimulateBurnQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
export interface CoreSimulateMintQuery<TData> extends CoreReactQuery<SimulateMintResponse, TData> {
    args: {
        amount: Uint128;
        funds?: Coin[];
        time?: number;
    };
}
export declare function useCoreSimulateMintQuery<TData = SimulateMintResponse>({ client, args, options }: CoreSimulateMintQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
export interface CoreListTradeInfoQuery<TData> extends CoreReactQuery<ListTradeInfoResponse, TData> {
    args: {
        denomIn: string;
        limit?: number;
        order?: RangeOrder;
        startAfter?: string;
    };
}
export declare function useCoreListTradeInfoQuery<TData = ListTradeInfoResponse>({ client, args, options }: CoreListTradeInfoQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
export interface CoreGetTradeInfoQuery<TData> extends CoreReactQuery<GetTradeInfoResponse, TData> {
    args: {
        denomIn: string;
        denomOut: string;
    };
}
export declare function useCoreGetTradeInfoQuery<TData = GetTradeInfoResponse>({ client, args, options }: CoreGetTradeInfoQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
export interface CoreGetRebalanceQuery<TData> extends CoreReactQuery<GetRebalanceResponse, TData> {
}
export declare function useCoreGetRebalanceQuery<TData = GetRebalanceResponse>({ client, options }: CoreGetRebalanceQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
export interface CoreGetPortfolioQuery<TData> extends CoreReactQuery<GetPortfolioResponse, TData> {
    args: {
        time?: number;
    };
}
export declare function useCoreGetPortfolioQuery<TData = GetPortfolioResponse>({ client, args, options }: CoreGetPortfolioQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
export interface CoreGetFeeQuery<TData> extends CoreReactQuery<GetFeeResponse, TData> {
    args: {
        time?: number;
    };
}
export declare function useCoreGetFeeQuery<TData = GetFeeResponse>({ client, args, options }: CoreGetFeeQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
export interface CoreGetConfigQuery<TData> extends CoreReactQuery<GetConfigResponse, TData> {
    args: {
        time?: number;
    };
}
export declare function useCoreGetConfigQuery<TData = GetConfigResponse>({ client, args, options }: CoreGetConfigQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
export interface CoreGetTotalSupplyQuery<TData> extends CoreReactQuery<Uint128, TData> {
}
export declare function useCoreGetTotalSupplyQuery<TData = Uint128>({ client, options }: CoreGetTotalSupplyQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
export interface CoreGetBalanceQuery<TData> extends CoreReactQuery<Uint128, TData> {
    args: {
        account: string;
    };
}
export declare function useCoreGetBalanceQuery<TData = Uint128>({ client, args, options }: CoreGetBalanceQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
//# sourceMappingURL=Core.react-query.d.ts.map