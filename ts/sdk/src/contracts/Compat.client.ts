/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { QueryMode, InstantiateMsg, ExecuteMsg, QueryMsg, Uint128, SwapRoutes, Coin, SwapRoute, AmountResponse, QueryModeResponse } from "./Compat.types";
export interface CompatReadOnlyInterface {
  contractAddress: string;
  queryMode: () => Promise<QueryModeResponse>;
  estimateSwapExactAmountIn: ({
    amount,
    mode,
    routes,
    sender
  }: {
    amount: Coin;
    mode?: QueryMode;
    routes: SwapRoutes;
    sender: string;
  }) => Promise<AmountResponse>;
  estimateSwapExactAmountOut: ({
    amount,
    mode,
    routes,
    sender
  }: {
    amount: Coin;
    mode?: QueryMode;
    routes: SwapRoutes;
    sender: string;
  }) => Promise<AmountResponse>;
}
export class CompatQueryClient implements CompatReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.queryMode = this.queryMode.bind(this);
    this.estimateSwapExactAmountIn = this.estimateSwapExactAmountIn.bind(this);
    this.estimateSwapExactAmountOut = this.estimateSwapExactAmountOut.bind(this);
  }

  queryMode = async (): Promise<QueryModeResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      query_mode: {}
    });
  };
  estimateSwapExactAmountIn = async ({
    amount,
    mode,
    routes,
    sender
  }: {
    amount: Coin;
    mode?: QueryMode;
    routes: SwapRoutes;
    sender: string;
  }): Promise<AmountResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      estimate_swap_exact_amount_in: {
        amount,
        mode,
        routes,
        sender
      }
    });
  };
  estimateSwapExactAmountOut = async ({
    amount,
    mode,
    routes,
    sender
  }: {
    amount: Coin;
    mode?: QueryMode;
    routes: SwapRoutes;
    sender: string;
  }): Promise<AmountResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      estimate_swap_exact_amount_out: {
        amount,
        mode,
        routes,
        sender
      }
    });
  };
}
export interface CompatInterface extends CompatReadOnlyInterface {
  contractAddress: string;
  sender: string;
  switchQueryMode: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export class CompatClient extends CompatQueryClient implements CompatInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.switchQueryMode = this.switchQueryMode.bind(this);
  }

  switchQueryMode = async (fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      switch_query_mode: {}
    }, fee, memo, funds);
  };
}