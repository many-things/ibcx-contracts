/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { Coin, StdFee } from "@cosmjs/amino";
import { AirdropResponse, AirdropsResponse, Uint64, QualificationResponse } from "./Airdrop.types";
export interface AirdropReadOnlyInterface {
    contractAddress: string;
    airdrop: () => Promise<AirdropResponse>;
    airdrops: () => Promise<AirdropsResponse>;
    latestAirdropId: () => Promise<Uint64>;
    qualification: ({ beneficiary, merkleProof }: {
        beneficiary: string;
        merkleProof: string[];
    }) => Promise<QualificationResponse>;
}
export declare class AirdropQueryClient implements AirdropReadOnlyInterface {
    client: CosmWasmClient;
    contractAddress: string;
    constructor(client: CosmWasmClient, contractAddress: string);
    airdrop: () => Promise<AirdropResponse>;
    airdrops: () => Promise<AirdropsResponse>;
    latestAirdropId: () => Promise<Uint64>;
    qualification: ({ beneficiary, merkleProof }: {
        beneficiary: string;
        merkleProof: string[];
    }) => Promise<QualificationResponse>;
}
export interface AirdropInterface extends AirdropReadOnlyInterface {
    contractAddress: string;
    sender: string;
    regsiter: ({ label, merkleRoot }: {
        label: string;
        merkleRoot: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    claim: ({ airdropId, beneficiary, merkleProof }: {
        airdropId: number;
        beneficiary?: string;
        merkleProof: string[];
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export declare class AirdropClient extends AirdropQueryClient implements AirdropInterface {
    client: SigningCosmWasmClient;
    sender: string;
    contractAddress: string;
    constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string);
    regsiter: ({ label, merkleRoot }: {
        label: string;
        merkleRoot: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    claim: ({ airdropId, beneficiary, merkleProof }: {
        airdropId: number;
        beneficiary?: string;
        merkleProof: string[];
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
//# sourceMappingURL=Airdrop.client.d.ts.map