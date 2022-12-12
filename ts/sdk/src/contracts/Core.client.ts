/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { Decimal, InstantiateMsg, ExecuteMsg, Uint128, GovMsg, SwapRoutes, RebalanceMsg, RebalanceTradeMsg, SwapRoute, QueryMsg, Coin, Addr, GetConfigResponse, GetPauseInfoResponse, GetPortfolioResponse, SimulateBurnResponse, SimulateMintResponse } from "./Core.types";
export interface CoreReadOnlyInterface {
  contractAddress: string;
  getConfig: () => Promise<GetConfigResponse>;
  getPauseInfo: () => Promise<GetPauseInfoResponse>;
  getPortfolio: () => Promise<GetPortfolioResponse>;
  simulateMint: ({
    amount,
    funds
  }: {
    amount: Uint128;
    funds: Coin[];
  }) => Promise<SimulateMintResponse>;
  simulateBurn: ({
    amount
  }: {
    amount: Uint128;
  }) => Promise<SimulateBurnResponse>;
}
export class CoreQueryClient implements CoreReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.getConfig = this.getConfig.bind(this);
    this.getPauseInfo = this.getPauseInfo.bind(this);
    this.getPortfolio = this.getPortfolio.bind(this);
    this.simulateMint = this.simulateMint.bind(this);
    this.simulateBurn = this.simulateBurn.bind(this);
  }

  getConfig = async (): Promise<GetConfigResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_config: {}
    });
  };
  getPauseInfo = async (): Promise<GetPauseInfoResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_pause_info: {}
    });
  };
  getPortfolio = async (): Promise<GetPortfolioResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_portfolio: {}
    });
  };
  simulateMint = async ({
    amount,
    funds
  }: {
    amount: Uint128;
    funds: Coin[];
  }): Promise<SimulateMintResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      simulate_mint: {
        amount,
        funds
      }
    });
  };
  simulateBurn = async ({
    amount
  }: {
    amount: Uint128;
  }): Promise<SimulateBurnResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      simulate_burn: {
        amount
      }
    });
  };
}
export interface CoreInterface extends CoreReadOnlyInterface {
  contractAddress: string;
  sender: string;
  mint: ({
    amount,
    receiver,
    refundTo
  }: {
    amount: Uint128;
    receiver?: string;
    refundTo?: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  burn: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  gov: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  rebalance: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export class CoreClient extends CoreQueryClient implements CoreInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.mint = this.mint.bind(this);
    this.burn = this.burn.bind(this);
    this.gov = this.gov.bind(this);
    this.rebalance = this.rebalance.bind(this);
  }

  mint = async ({
    amount,
    receiver,
    refundTo
  }: {
    amount: Uint128;
    receiver?: string;
    refundTo?: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      mint: {
        amount,
        receiver,
        refund_to: refundTo
      }
    }, fee, memo, funds);
  };
  burn = async (fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      burn: {}
    }, fee, memo, funds);
  };
  gov = async (fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      gov: {}
    }, fee, memo, funds);
  };
  rebalance = async (fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      rebalance: {}
    }, fee, memo, funds);
  };
}