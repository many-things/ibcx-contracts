/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { Coin, StdFee } from "@cosmjs/amino";
import { InstantiateMsg, ExecuteMsg, AirdropId, Uint128, QueryMsg, RangeOrder, AirdropIdOptional, MigrateMsg, CheckQualificationResponse, GetAirdropResponse, Addr, GetClaimResponse, LatestAirdropResponse, ListAirdropsResponse, ListClaimsResponse } from "./Airdrop.types";
export interface AirdropReadOnlyInterface {
  contractAddress: string;
  getAirdrop: ({
    id
  }: {
    id: AirdropId;
  }) => Promise<GetAirdropResponse>;
  listAirdrops: ({
    limit,
    order,
    startAfter
  }: {
    limit?: number;
    order?: RangeOrder;
    startAfter: AirdropIdOptional;
  }) => Promise<ListAirdropsResponse>;
  latestAirdropId: () => Promise<LatestAirdropResponse>;
  getClaim: ({
    account,
    id
  }: {
    account: string;
    id: AirdropId;
  }) => Promise<GetClaimResponse>;
  listClaims: ({
    id,
    limit,
    order,
    startAfter
  }: {
    id: AirdropId;
    limit?: number;
    order?: RangeOrder;
    startAfter?: string;
  }) => Promise<ListClaimsResponse>;
  checkQualification: ({
    amount,
    beneficiary,
    claimProof,
    id,
    merkleProof
  }: {
    amount: Uint128;
    beneficiary?: string;
    claimProof?: string;
    id: AirdropId;
    merkleProof: string[];
  }) => Promise<CheckQualificationResponse>;
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
    this.listClaims = this.listClaims.bind(this);
    this.checkQualification = this.checkQualification.bind(this);
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
    limit,
    order,
    startAfter
  }: {
    limit?: number;
    order?: RangeOrder;
    startAfter: AirdropIdOptional;
  }): Promise<ListAirdropsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      list_airdrops: {
        limit,
        order,
        start_after: startAfter
      }
    });
  };
  latestAirdropId = async (): Promise<LatestAirdropResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      latest_airdrop_id: {}
    });
  };
  getClaim = async ({
    account,
    id
  }: {
    account: string;
    id: AirdropId;
  }): Promise<GetClaimResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_claim: {
        account,
        id
      }
    });
  };
  listClaims = async ({
    id,
    limit,
    order,
    startAfter
  }: {
    id: AirdropId;
    limit?: number;
    order?: RangeOrder;
    startAfter?: string;
  }): Promise<ListClaimsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      list_claims: {
        id,
        limit,
        order,
        start_after: startAfter
      }
    });
  };
  checkQualification = async ({
    amount,
    beneficiary,
    claimProof,
    id,
    merkleProof
  }: {
    amount: Uint128;
    beneficiary?: string;
    claimProof?: string;
    id: AirdropId;
    merkleProof: string[];
  }): Promise<CheckQualificationResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      check_qualification: {
        amount,
        beneficiary,
        claim_proof: claimProof,
        id,
        merkle_proof: merkleProof
      }
    });
  };
}
export interface AirdropInterface extends AirdropReadOnlyInterface {
  contractAddress: string;
  sender: string;
  regsiter: ({
    bearer,
    denom,
    label,
    merkleRoot
  }: {
    bearer?: boolean;
    denom: string;
    label?: string;
    merkleRoot: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  fund: ({
    id
  }: {
    id: AirdropId;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  claim: ({
    amount,
    beneficiary,
    claimProof,
    id,
    merkleProof
  }: {
    amount: Uint128;
    beneficiary?: string;
    claimProof?: string;
    id: AirdropId;
    merkleProof: string[];
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
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
    this.regsiter = this.regsiter.bind(this);
    this.fund = this.fund.bind(this);
    this.claim = this.claim.bind(this);
  }

  regsiter = async ({
    bearer,
    denom,
    label,
    merkleRoot
  }: {
    bearer?: boolean;
    denom: string;
    label?: string;
    merkleRoot: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      regsiter: {
        bearer,
        denom,
        label,
        merkle_root: merkleRoot
      }
    }, fee, memo, funds);
  };
  fund = async ({
    id
  }: {
    id: AirdropId;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      fund: {
        id
      }
    }, fee, memo, funds);
  };
  claim = async ({
    amount,
    beneficiary,
    claimProof,
    id,
    merkleProof
  }: {
    amount: Uint128;
    beneficiary?: string;
    claimProof?: string;
    id: AirdropId;
    merkleProof: string[];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      claim: {
        amount,
        beneficiary,
        claim_proof: claimProof,
        id,
        merkle_proof: merkleProof
      }
    }, fee, memo, funds);
  };
}