/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

export interface InstantiateMsg {}
export type ExecuteMsg = {
  mint_exact_amount_in: {
    core_addr: string;
    input_asset: string;
    min_output_amount: Uint128;
    swap_info: SwapInfosCompact;
  };
} | {
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
} | {
  burn_exact_amount_out: {
    core_addr: string;
    output_asset: Coin;
    swap_info: SwapInfosCompact;
  };
} | {
  finish_operation: {
    refund_asset: string;
    refund_to: string;
  };
};
export type Uint128 = string;
export type SwapInfosCompact = SwapInfoCompact[];
export interface SwapInfoCompact {
  key: string;
  routes: string[];
}
export interface Coin {
  amount: Uint128;
  denom: string;
  [k: string]: unknown;
}
export type QueryMsg = {
  simulate_mint_exact_amount_in: {
    core_addr: string;
    input_asset: Coin;
    swap_info: SwapInfosCompact;
  };
} | {
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
} | {
  simulate_burn_exact_amount_out: {
    core_addr: string;
    output_asset: Coin;
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
export interface SimulateBurnExactAmountOutResponse {
  burn_amount: Uint128;
  burn_redeem_amount: Coin[];
  swap_result_amount: Coin;
}
export interface SimulateMintExactAmountInResponse {
  mint_amount: Uint128;
  mint_spend_amount: Coin[];
  swap_result_amount: Coin;
}
export interface SimulateMintExactAmountOutResponse {
  mint_amount: Uint128;
  mint_spend_amount: Coin[];
  swap_result_amount: Coin;
}