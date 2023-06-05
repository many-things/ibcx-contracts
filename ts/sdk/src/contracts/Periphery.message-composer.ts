/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { MsgExecuteContractEncodeObject } from "cosmwasm";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import { InstantiateMsg, ExecuteMsg, Uint128, SwapInfosCompact, SwapInfoCompact, Coin, QueryMsg, MigrateMsg, SimulateBurnExactAmountInResponse, SimulateBurnExactAmountOutResponse, SimulateMintExactAmountInResponse, SimulateMintExactAmountOutResponse } from "./Periphery.types";
export interface PeripheryMessage {
  contractAddress: string;
  sender: string;
  mintExactAmountIn: ({
    coreAddr,
    inputAsset,
    minOutputAmount,
    swapInfo
  }: {
    coreAddr: string;
    inputAsset: string;
    minOutputAmount: Uint128;
    swapInfo: SwapInfosCompact;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  mintExactAmountOut: ({
    coreAddr,
    inputAsset,
    outputAmount,
    swapInfo
  }: {
    coreAddr: string;
    inputAsset: string;
    outputAmount: Uint128;
    swapInfo: SwapInfosCompact;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  burnExactAmountIn: ({
    coreAddr,
    minOutputAmount,
    outputAsset,
    swapInfo
  }: {
    coreAddr: string;
    minOutputAmount: Uint128;
    outputAsset: string;
    swapInfo: SwapInfosCompact;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  burnExactAmountOut: ({
    coreAddr,
    outputAsset,
    swapInfo
  }: {
    coreAddr: string;
    outputAsset: Coin;
    swapInfo: SwapInfosCompact;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  finishOperation: ({
    refundAsset,
    refundTo
  }: {
    refundAsset: string;
    refundTo: string;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
export class PeripheryMessageComposer implements PeripheryMessage {
  sender: string;
  contractAddress: string;

  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.mintExactAmountIn = this.mintExactAmountIn.bind(this);
    this.mintExactAmountOut = this.mintExactAmountOut.bind(this);
    this.burnExactAmountIn = this.burnExactAmountIn.bind(this);
    this.burnExactAmountOut = this.burnExactAmountOut.bind(this);
    this.finishOperation = this.finishOperation.bind(this);
  }

  mintExactAmountIn = ({
    coreAddr,
    inputAsset,
    minOutputAmount,
    swapInfo
  }: {
    coreAddr: string;
    inputAsset: string;
    minOutputAmount: Uint128;
    swapInfo: SwapInfosCompact;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          mint_exact_amount_in: {
            core_addr: coreAddr,
            input_asset: inputAsset,
            min_output_amount: minOutputAmount,
            swap_info: swapInfo
          }
        })),
        funds
      })
    };
  };
  mintExactAmountOut = ({
    coreAddr,
    inputAsset,
    outputAmount,
    swapInfo
  }: {
    coreAddr: string;
    inputAsset: string;
    outputAmount: Uint128;
    swapInfo: SwapInfosCompact;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          mint_exact_amount_out: {
            core_addr: coreAddr,
            input_asset: inputAsset,
            output_amount: outputAmount,
            swap_info: swapInfo
          }
        })),
        funds
      })
    };
  };
  burnExactAmountIn = ({
    coreAddr,
    minOutputAmount,
    outputAsset,
    swapInfo
  }: {
    coreAddr: string;
    minOutputAmount: Uint128;
    outputAsset: string;
    swapInfo: SwapInfosCompact;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          burn_exact_amount_in: {
            core_addr: coreAddr,
            min_output_amount: minOutputAmount,
            output_asset: outputAsset,
            swap_info: swapInfo
          }
        })),
        funds
      })
    };
  };
  burnExactAmountOut = ({
    coreAddr,
    outputAsset,
    swapInfo
  }: {
    coreAddr: string;
    outputAsset: Coin;
    swapInfo: SwapInfosCompact;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          burn_exact_amount_out: {
            core_addr: coreAddr,
            output_asset: outputAsset,
            swap_info: swapInfo
          }
        })),
        funds
      })
    };
  };
  finishOperation = ({
    refundAsset,
    refundTo
  }: {
    refundAsset: string;
    refundTo: string;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          finish_operation: {
            refund_asset: refundAsset,
            refund_to: refundTo
          }
        })),
        funds
      })
    };
  };
}