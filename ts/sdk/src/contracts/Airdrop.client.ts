/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { Coin, StdFee } from "@cosmjs/amino";
import { InstantiateMsg, ExecuteMsg, RegisterPayload, AirdropId, ClaimPayload, Uint128, QueryMsg, ListAirdropsQueryOptions, RangeOrder, AirdropType, MigrateMsg, GetAirdropResponse, GetClaimResponse, GetLabelResponse, LatestAirdropResponse, ListAirdropsResponse, ListClaimsResponse, ListLabelsResponse, VerifyClaimResponse } from "./Airdrop.types";
export interface AirdropReadOnlyInterface {
  contractAddress: string;
  getAirdrop: ({
    id
  }: {
    id: AirdropId;
  }) => Promise<GetAirdropResponse>;
  listAirdrops: ({
    option
  }: {
    option: ListAirdropsQueryOptions;
  }) => Promise<ListAirdropsResponse>;
  latestAirdropId: () => Promise<LatestAirdropResponse>;
  getClaim: ({
    airdrop,
    claimKey
  }: {
    airdrop: AirdropId;
    claimKey: string;
  }) => Promise<GetClaimResponse>;
  verifyClaim: ({
    claim
  }: {
    claim: ClaimPayload;
  }) => Promise<VerifyClaimResponse>;
  listClaims: ({
    airdrop,
    limit,
    order,
    startAfter
  }: {
    airdrop: AirdropId;
    limit?: number;
    order?: RangeOrder;
    startAfter?: string;
  }) => Promise<ListClaimsResponse>;
  getLabel: ({
    label
  }: {
    label: string;
  }) => Promise<GetLabelResponse>;
  listLabels: ({
    limit,
    order,
    startAfter
  }: {
    limit?: number;
    order?: RangeOrder;
    startAfter?: string;
  }) => Promise<ListLabelsResponse>;
}
export class AirdropQueryClient implements AirdropReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.getAirdrop = this.getAirdrop.bind(this);
    this.listAirdrops = this.listAirdrops.bind(this);
    this.latestAirdropId = this.latestAirdropId.bind(this);
    this.getClaim = this.getClaim.bind(this);
    this.verifyClaim = this.verifyClaim.bind(this);
    this.listClaims = this.listClaims.bind(this);
    this.getLabel = this.getLabel.bind(this);
    this.listLabels = this.listLabels.bind(this);
  }

  getAirdrop = async ({
    id
  }: {
    id: AirdropId;
  }): Promise<GetAirdropResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_airdrop: {
        id
      }
    });
  };
  listAirdrops = async ({
    option
  }: {
    option: ListAirdropsQueryOptions;
  }): Promise<ListAirdropsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      list_airdrops: {
        option
      }
    });
  };
  latestAirdropId = async (): Promise<LatestAirdropResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      latest_airdrop_id: {}
    });
  };
  getClaim = async ({
    airdrop,
    claimKey
  }: {
    airdrop: AirdropId;
    claimKey: string;
  }): Promise<GetClaimResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_claim: {
        airdrop,
        claim_key: claimKey
      }
    });
  };
  verifyClaim = async ({
    claim
  }: {
    claim: ClaimPayload;
  }): Promise<VerifyClaimResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      verify_claim: {
        claim
      }
    });
  };
  listClaims = async ({
    airdrop,
    limit,
    order,
    startAfter
  }: {
    airdrop: AirdropId;
    limit?: number;
    order?: RangeOrder;
    startAfter?: string;
  }): Promise<ListClaimsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      list_claims: {
        airdrop,
        limit,
        order,
        start_after: startAfter
      }
    });
  };
  getLabel = async ({
    label
  }: {
    label: string;
  }): Promise<GetLabelResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_label: {
        label
      }
    });
  };
  listLabels = async ({
    limit,
    order,
    startAfter
  }: {
    limit?: number;
    order?: RangeOrder;
    startAfter?: string;
  }): Promise<ListLabelsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      list_labels: {
        limit,
        order,
        start_after: startAfter
      }
    });
  };
}
export interface AirdropInterface extends AirdropReadOnlyInterface {
  contractAddress: string;
  sender: string;
  register: (registerPayload: RegisterPayload, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  fund: (airdropId: AirdropId, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  claim: (claimPayload: ClaimPayload, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  close: (airdropId: AirdropId, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export class AirdropClient extends AirdropQueryClient implements AirdropInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.register = this.register.bind(this);
    this.fund = this.fund.bind(this);
    this.claim = this.claim.bind(this);
    this.close = this.close.bind(this);
  }

  register = async (registerPayload: RegisterPayload, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      register: registerPayload
    }, fee, memo, funds);
  };
  fund = async (airdropId: AirdropId, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      fund: airdropId
    }, fee, memo, funds);
  };
  claim = async (claimPayload: ClaimPayload, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      claim: claimPayload
    }, fee, memo, funds);
  };
  close = async (airdropId: AirdropId, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      close: airdropId
    }, fee, memo, funds);
  };
}