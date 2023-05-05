/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
export interface InstantiateMsg {
}
export type ExecuteMsg = {
    mint_exact_amount_out: {
        core_addr: string;
        input_asset: string;
        output_amount: Uint128;
        swap_info: SwapInfosCompact;
    };
} | {
    burn_exact_amount_in: {
        core_addr: string;
        min_output_amount: Uint128;
        output_asset: string;
        swap_info: SwapInfosCompact;
    };
};
export type Uint128 = string;
export type SwapInfosCompact = SwapInfoCompact[];
export interface SwapInfoCompact {
    key: string;
    routes: string[];
}
export type QueryMsg = {
    simulate_mint_exact_amount_out: {
        core_addr: string;
        input_asset: string;
        output_amount: Uint128;
        swap_info: SwapInfosCompact;
    };
} | {
    simulate_burn_exact_amount_in: {
        core_addr: string;
        input_amount: Uint128;
        output_asset: string;
        swap_info: SwapInfosCompact;
    };
};
export interface MigrateMsg {
    force?: boolean | null;
}
export interface SimulateBurnExactAmountInResponse {
    burn_amount: Uint128;
    burn_redeem_amount: Coin[];
    swap_result_amount: Coin;
}
export interface Coin {
    amount: Uint128;
    denom: string;
    [k: string]: unknown;
}
export interface SimulateMintExactAmountOutResponse {
    mint_amount: Uint128;
    mint_spend_amount: Coin[];
    swap_result_amount: Coin;
}
//# sourceMappingURL=Periphery.types.d.ts.map