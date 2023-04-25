/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
export interface InstantiateMsg {
}
export type ExecuteMsg = {
    register: RegisterPayload;
} | {
    fund: AirdropId;
} | {
    claim: ClaimPayload;
} | {
    close: AirdropId;
};
export type RegisterPayload = {
    open: {
        denom: string;
        label?: string | null;
        merkle_root: string;
    };
} | {
    bearer: {
        denom: string;
        label?: string | null;
        merkle_root: string;
        signer_pub: string;
        signer_sig: string;
    };
};
export type AirdropId = {
    id: number;
} | {
    label: string;
};
export type ClaimPayload = {
    open: {
        account?: string | null;
        airdrop: AirdropId;
        amount: Uint128;
        merkle_proof: string[];
    };
} | {
    bearer: {
        account?: string | null;
        airdrop: AirdropId;
        amount: Uint128;
        claim_hash: string;
        claim_sign: string;
        merkle_proof: string[];
    };
};
export type Uint128 = string;
export type QueryMsg = {
    get_airdrop: {
        id: AirdropId;
    };
} | {
    list_airdrops: {
        option: ListAirdropsQueryOptions;
    };
} | {
    latest_airdrop_id: {};
} | {
    get_claim: {
        airdrop: AirdropId;
        claim_key: string;
    };
} | {
    verify_claim: {
        claim: ClaimPayload;
    };
} | {
    list_claims: {
        airdrop: AirdropId;
        limit?: number | null;
        order?: RangeOrder | null;
        start_after?: string | null;
    };
} | {
    get_label: {
        label: string;
    };
} | {
    list_labels: {
        limit?: number | null;
        order?: RangeOrder | null;
        start_after?: string | null;
    };
};
export type ListAirdropsQueryOptions = {
    by_i_d: {
        limit?: number | null;
        order?: RangeOrder | null;
        start_after?: number | null;
    };
} | {
    by_type: {
        limit?: number | null;
        order?: RangeOrder | null;
        start_after?: number | null;
        type: AirdropType;
    };
} | {
    by_label: {
        limit?: number | null;
        order?: RangeOrder | null;
        start_after?: string | null;
    };
} | {
    by_creator: {
        creator: string;
        limit?: number | null;
        order?: RangeOrder | null;
        start_after?: number | null;
    };
};
export type RangeOrder = "asc" | "desc";
export type AirdropType = "open" | "bearer";
export interface MigrateMsg {
    force?: boolean | null;
}
export type GetAirdropResponse = {
    open: {
        closed_at?: number | null;
        created_at: number;
        creator: string;
        denom: string;
        id: number;
        label?: string | null;
        merkle_root: string;
        total_amount: Uint128;
        total_claimed: Uint128;
    };
} | {
    bearer: {
        closed_at?: number | null;
        created_at: number;
        creator: string;
        denom: string;
        id: number;
        label?: string | null;
        merkle_root: string;
        signer: string;
        signer_pub: string;
        total_amount: Uint128;
        total_claimed: Uint128;
    };
};
export interface GetClaimResponse {
    amount: Uint128;
    claim_key: string;
    id: number;
}
export interface GetLabelResponse {
    airdrop_id: number;
    creator: string;
    label: string;
}
export type LatestAirdropResponse = number;
export type ListAirdropsResponse = GetAirdropResponse[];
export type ListClaimsResponse = GetClaimResponse[];
export type ListLabelsResponse = GetLabelResponse[];
export interface VerifyClaimResponse {
    reason?: string | null;
    valid: boolean;
}
//# sourceMappingURL=Airdrop.types.d.ts.map