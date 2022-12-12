/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { Coin, StdFee } from "@cosmjs/amino";
import { SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { InstantiateMsg, ExecuteMsg, Uint128, SwapRoutes, SwapRoute, MigrateMsg } from "./Periphery.types";
export interface PeripheryInterface {
  contractAddress: string;
  sender: string;
  mintExactAmountOut: ({
    coreAddr,
    inputAsset,
    outputAmount,
    swapInfo
  }: {
    coreAddr: string;
    inputAsset: string;
    outputAmount: Uint128;
    swapInfo: string[][];
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  burnExactAmountIn: ({
    coreAddr,
    minOutputAmount,
    outputAsset,
    swapInfo
  }: {
    coreAddr: string;
    minOutputAmount: Uint128;
    outputAsset: string;
    swapInfo: string[][];
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export class PeripheryClient implements PeripheryInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.mintExactAmountOut = this.mintExactAmountOut.bind(this);
    this.burnExactAmountIn = this.burnExactAmountIn.bind(this);
  }

  mintExactAmountOut = async ({
    coreAddr,
    inputAsset,
    outputAmount,
    swapInfo
  }: {
    coreAddr: string;
    inputAsset: string;
    outputAmount: Uint128;
    swapInfo: string[][];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      mint_exact_amount_out: {
        core_addr: coreAddr,
        input_asset: inputAsset,
        output_amount: outputAmount,
        swap_info: swapInfo
      }
    }, fee, memo, funds);
  };
  burnExactAmountIn = async ({
    coreAddr,
    minOutputAmount,
    outputAsset,
    swapInfo
  }: {
    coreAddr: string;
    minOutputAmount: Uint128;
    outputAsset: string;
    swapInfo: string[][];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      burn_exact_amount_in: {
        core_addr: coreAddr,
        min_output_amount: minOutputAmount,
        output_asset: outputAsset,
        swap_info: swapInfo
      }
    }, fee, memo, funds);
  };
}