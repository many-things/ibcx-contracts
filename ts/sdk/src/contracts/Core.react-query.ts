/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { UseQueryOptions, useQuery } from "@tanstack/react-query";
import { Decimal, InstantiateMsg, FeePayload, StreamingFeePayload, ExecuteMsg, Uint128, GovMsg, SwapRoutes, RebalanceMsg, RebalanceTradeMsg, SwapRoute, QueryMsg, RangeOrder, Coin, MigrateMsg, Addr, GetConfigResponse, PausedResponse, GetFeeResponse, StreamingFeeResponse, GetPortfolioResponse, GetRebalanceResponse, RebalancePayload, GetTradeInfoResponse, TradeInfoPayload, ListTradeInfoResponse, SimulateBurnResponse, SimulateMintResponse } from "./Core.types";
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
export function useCoreSimulateBurnQuery<TData = SimulateBurnResponse>({
  client,
  args,
  options
}: CoreSimulateBurnQuery<TData>) {
  return useQuery<SimulateBurnResponse, Error, TData>(["coreSimulateBurn", client.contractAddress, JSON.stringify(args)], () => client.simulateBurn({
    amount: args.amount,
    time: args.time
  }), options);
}
export interface CoreSimulateMintQuery<TData> extends CoreReactQuery<SimulateMintResponse, TData> {
  args: {
    amount: Uint128;
    funds?: Coin[];
    time?: number;
  };
}
export function useCoreSimulateMintQuery<TData = SimulateMintResponse>({
  client,
  args,
  options
}: CoreSimulateMintQuery<TData>) {
  return useQuery<SimulateMintResponse, Error, TData>(["coreSimulateMint", client.contractAddress, JSON.stringify(args)], () => client.simulateMint({
    amount: args.amount,
    funds: args.funds,
    time: args.time
  }), options);
}
export interface CoreListTradeInfoQuery<TData> extends CoreReactQuery<ListTradeInfoResponse, TData> {
  args: {
    denomIn: string;
    limit?: number;
    order?: RangeOrder;
    startAfter?: string;
  };
}
export function useCoreListTradeInfoQuery<TData = ListTradeInfoResponse>({
  client,
  args,
  options
}: CoreListTradeInfoQuery<TData>) {
  return useQuery<ListTradeInfoResponse, Error, TData>(["coreListTradeInfo", client.contractAddress, JSON.stringify(args)], () => client.listTradeInfo({
    denomIn: args.denomIn,
    limit: args.limit,
    order: args.order,
    startAfter: args.startAfter
  }), options);
}
export interface CoreGetTradeInfoQuery<TData> extends CoreReactQuery<GetTradeInfoResponse, TData> {
  args: {
    denomIn: string;
    denomOut: string;
  };
}
export function useCoreGetTradeInfoQuery<TData = GetTradeInfoResponse>({
  client,
  args,
  options
}: CoreGetTradeInfoQuery<TData>) {
  return useQuery<GetTradeInfoResponse, Error, TData>(["coreGetTradeInfo", client.contractAddress, JSON.stringify(args)], () => client.getTradeInfo({
    denomIn: args.denomIn,
    denomOut: args.denomOut
  }), options);
}
export interface CoreGetRebalanceQuery<TData> extends CoreReactQuery<GetRebalanceResponse, TData> {}
export function useCoreGetRebalanceQuery<TData = GetRebalanceResponse>({
  client,
  options
}: CoreGetRebalanceQuery<TData>) {
  return useQuery<GetRebalanceResponse, Error, TData>(["coreGetRebalance", client.contractAddress], () => client.getRebalance(), options);
}
export interface CoreGetPortfolioQuery<TData> extends CoreReactQuery<GetPortfolioResponse, TData> {
  args: {
    time?: number;
  };
}
export function useCoreGetPortfolioQuery<TData = GetPortfolioResponse>({
  client,
  args,
  options
}: CoreGetPortfolioQuery<TData>) {
  return useQuery<GetPortfolioResponse, Error, TData>(["coreGetPortfolio", client.contractAddress, JSON.stringify(args)], () => client.getPortfolio({
    time: args.time
  }), options);
}
export interface CoreGetFeeQuery<TData> extends CoreReactQuery<GetFeeResponse, TData> {
  args: {
    time?: number;
  };
}
export function useCoreGetFeeQuery<TData = GetFeeResponse>({
  client,
  args,
  options
}: CoreGetFeeQuery<TData>) {
  return useQuery<GetFeeResponse, Error, TData>(["coreGetFee", client.contractAddress, JSON.stringify(args)], () => client.getFee({
    time: args.time
  }), options);
}
export interface CoreGetConfigQuery<TData> extends CoreReactQuery<GetConfigResponse, TData> {
  args: {
    time?: number;
  };
}
export function useCoreGetConfigQuery<TData = GetConfigResponse>({
  client,
  args,
  options
}: CoreGetConfigQuery<TData>) {
  return useQuery<GetConfigResponse, Error, TData>(["coreGetConfig", client.contractAddress, JSON.stringify(args)], () => client.getConfig({
    time: args.time
  }), options);
}
export interface CoreGetTotalSupplyQuery<TData> extends CoreReactQuery<Uint128, TData> {}
export function useCoreGetTotalSupplyQuery<TData = Uint128>({
  client,
  options
}: CoreGetTotalSupplyQuery<TData>) {
  return useQuery<Uint128, Error, TData>(["coreGetTotalSupply", client.contractAddress], () => client.getTotalSupply(), options);
}
export interface CoreGetBalanceQuery<TData> extends CoreReactQuery<Uint128, TData> {
  args: {
    account: string;
  };
}
export function useCoreGetBalanceQuery<TData = Uint128>({
  client,
  args,
  options
}: CoreGetBalanceQuery<TData>) {
  return useQuery<Uint128, Error, TData>(["coreGetBalance", client.contractAddress, JSON.stringify(args)], () => client.getBalance({
    account: args.account
  }), options);
}