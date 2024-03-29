/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
import { UseQueryOptions } from "@tanstack/react-query";
import { AirdropId, ClaimPayload, ListAirdropsQueryOptions, RangeOrder, GetAirdropResponse, GetClaimResponse, GetLabelResponse, LatestAirdropResponse, ListAirdropsResponse, ListClaimsResponse, ListLabelsResponse, VerifyClaimResponse } from "./Airdrop.types";
import { AirdropQueryClient } from "./Airdrop.client";
export interface AirdropReactQuery<TResponse, TData = TResponse> {
    client: AirdropQueryClient;
    options?: Omit<UseQueryOptions<TResponse, Error, TData>, "'queryKey' | 'queryFn' | 'initialData'"> & {
        initialData?: undefined;
    };
}
export interface AirdropListLabelsQuery<TData> extends AirdropReactQuery<ListLabelsResponse, TData> {
    args: {
        limit?: number;
        order?: RangeOrder;
        startAfter?: string;
    };
}
export declare function useAirdropListLabelsQuery<TData = ListLabelsResponse>({ client, args, options }: AirdropListLabelsQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
export interface AirdropGetLabelQuery<TData> extends AirdropReactQuery<GetLabelResponse, TData> {
    args: {
        label: string;
    };
}
export declare function useAirdropGetLabelQuery<TData = GetLabelResponse>({ client, args, options }: AirdropGetLabelQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
export interface AirdropListClaimsQuery<TData> extends AirdropReactQuery<ListClaimsResponse, TData> {
    args: {
        airdrop: AirdropId;
        limit?: number;
        order?: RangeOrder;
        startAfter?: string;
    };
}
export declare function useAirdropListClaimsQuery<TData = ListClaimsResponse>({ client, args, options }: AirdropListClaimsQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
export interface AirdropVerifyClaimQuery<TData> extends AirdropReactQuery<VerifyClaimResponse, TData> {
    args: {
        claim: ClaimPayload;
    };
}
export declare function useAirdropVerifyClaimQuery<TData = VerifyClaimResponse>({ client, args, options }: AirdropVerifyClaimQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
export interface AirdropGetClaimQuery<TData> extends AirdropReactQuery<GetClaimResponse, TData> {
    args: {
        airdrop: AirdropId;
        claimKey: string;
    };
}
export declare function useAirdropGetClaimQuery<TData = GetClaimResponse>({ client, args, options }: AirdropGetClaimQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
export interface AirdropLatestAirdropIdQuery<TData> extends AirdropReactQuery<LatestAirdropResponse, TData> {
}
export declare function useAirdropLatestAirdropIdQuery<TData = LatestAirdropResponse>({ client, options }: AirdropLatestAirdropIdQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
export interface AirdropListAirdropsQuery<TData> extends AirdropReactQuery<ListAirdropsResponse, TData> {
    args: {
        option: ListAirdropsQueryOptions;
    };
}
export declare function useAirdropListAirdropsQuery<TData = ListAirdropsResponse>({ client, args, options }: AirdropListAirdropsQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
export interface AirdropGetAirdropQuery<TData> extends AirdropReactQuery<GetAirdropResponse, TData> {
    args: {
        id: AirdropId;
    };
}
export declare function useAirdropGetAirdropQuery<TData = GetAirdropResponse>({ client, args, options }: AirdropGetAirdropQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
//# sourceMappingURL=Airdrop.react-query.d.ts.map