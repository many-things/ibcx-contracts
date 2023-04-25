/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
import * as _0 from "./Airdrop.types";
import * as _1 from "./Airdrop.client";
import * as _2 from "./Airdrop.message-composer";
import * as _3 from "./Airdrop.react-query";
import * as _4 from "./Core.types";
import * as _5 from "./Core.client";
import * as _6 from "./Core.message-composer";
import * as _7 from "./Core.react-query";
import * as _8 from "./Faucet.types";
import * as _9 from "./Faucet.client";
import * as _10 from "./Faucet.message-composer";
import * as _11 from "./Faucet.react-query";
import * as _13 from "./Periphery.client";
import * as _14 from "./Periphery.message-composer";
export declare namespace contracts {
    const Airdrop: {
        useAirdropListLabelsQuery<TData = _0.ListLabelsResponse>({ client, args, options }: _3.AirdropListLabelsQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
        useAirdropGetLabelQuery<TData_1 = _0.GetLabelResponse>({ client, args, options }: _3.AirdropGetLabelQuery<TData_1>): import("@tanstack/react-query").UseQueryResult<TData_1, Error>;
        useAirdropListClaimsQuery<TData_2 = _0.ListClaimsResponse>({ client, args, options }: _3.AirdropListClaimsQuery<TData_2>): import("@tanstack/react-query").UseQueryResult<TData_2, Error>;
        useAirdropVerifyClaimQuery<TData_3 = _0.VerifyClaimResponse>({ client, args, options }: _3.AirdropVerifyClaimQuery<TData_3>): import("@tanstack/react-query").UseQueryResult<TData_3, Error>;
        useAirdropGetClaimQuery<TData_4 = _0.GetClaimResponse>({ client, args, options }: _3.AirdropGetClaimQuery<TData_4>): import("@tanstack/react-query").UseQueryResult<TData_4, Error>;
        useAirdropLatestAirdropIdQuery<TData_5 = number>({ client, options }: _3.AirdropLatestAirdropIdQuery<TData_5>): import("@tanstack/react-query").UseQueryResult<TData_5, Error>;
        useAirdropListAirdropsQuery<TData_6 = _0.ListAirdropsResponse>({ client, args, options }: _3.AirdropListAirdropsQuery<TData_6>): import("@tanstack/react-query").UseQueryResult<TData_6, Error>;
        useAirdropGetAirdropQuery<TData_7 = _0.GetAirdropResponse>({ client, args, options }: _3.AirdropGetAirdropQuery<TData_7>): import("@tanstack/react-query").UseQueryResult<TData_7, Error>;
        AirdropMessageComposer: typeof _2.AirdropMessageComposer;
        AirdropQueryClient: typeof _1.AirdropQueryClient;
        AirdropClient: typeof _1.AirdropClient;
    };
    const Core: {
        useCoreSimulateBurnQuery<TData = _4.SimulateBurnResponse>({ client, args, options }: _7.CoreSimulateBurnQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
        useCoreSimulateMintQuery<TData_1 = _4.SimulateMintResponse>({ client, args, options }: _7.CoreSimulateMintQuery<TData_1>): import("@tanstack/react-query").UseQueryResult<TData_1, Error>;
        useCoreGetPortfolioQuery<TData_2 = _4.GetPortfolioResponse>({ client, args, options }: _7.CoreGetPortfolioQuery<TData_2>): import("@tanstack/react-query").UseQueryResult<TData_2, Error>;
        useCoreGetPauseInfoQuery<TData_3 = _4.GetPauseInfoResponse>({ client, args, options }: _7.CoreGetPauseInfoQuery<TData_3>): import("@tanstack/react-query").UseQueryResult<TData_3, Error>;
        useCoreGetFeeQuery<TData_4 = _4.GetFeeResponse>({ client, args, options }: _7.CoreGetFeeQuery<TData_4>): import("@tanstack/react-query").UseQueryResult<TData_4, Error>;
        useCoreGetConfigQuery<TData_5 = _4.GetConfigResponse>({ client, options }: _7.CoreGetConfigQuery<TData_5>): import("@tanstack/react-query").UseQueryResult<TData_5, Error>;
        useCoreGetBalanceQuery<TData_6 = string>({ client, args, options }: _7.CoreGetBalanceQuery<TData_6>): import("@tanstack/react-query").UseQueryResult<TData_6, Error>;
        CoreMessageComposer: typeof _6.CoreMessageComposer;
        CoreQueryClient: typeof _5.CoreQueryClient;
        CoreClient: typeof _5.CoreClient;
    };
    const Faucet: {
        useFaucetListRolesQuery<TData = _8.ListRolesResponse>({ client, args, options }: _11.FaucetListRolesQuery<TData>): import("@tanstack/react-query").UseQueryResult<TData, Error>;
        useFaucetGetRoleQuery<TData_1 = _8.GetRoleResponse>({ client, args, options }: _11.FaucetGetRoleQuery<TData_1>): import("@tanstack/react-query").UseQueryResult<TData_1, Error>;
        useFaucetGetLastTokenIdQuery<TData_2 = number>({ client, options }: _11.FaucetGetLastTokenIdQuery<TData_2>): import("@tanstack/react-query").UseQueryResult<TData_2, Error>;
        useFaucetListTokensQuery<TData_3 = _8.ListTokensResponse>({ client, args, options }: _11.FaucetListTokensQuery<TData_3>): import("@tanstack/react-query").UseQueryResult<TData_3, Error>;
        useFaucetGetTokenQuery<TData_4 = _8.GetTokenResponse>({ client, args, options }: _11.FaucetGetTokenQuery<TData_4>): import("@tanstack/react-query").UseQueryResult<TData_4, Error>;
        useFaucetListAliasesQuery<TData_5 = _8.ListAliasesResponse>({ client, args, options }: _11.FaucetListAliasesQuery<TData_5>): import("@tanstack/react-query").UseQueryResult<TData_5, Error>;
        FaucetMessageComposer: typeof _10.FaucetMessageComposer;
        FaucetQueryClient: typeof _9.FaucetQueryClient;
        FaucetClient: typeof _9.FaucetClient;
    };
    const Periphery: {
        PeripheryMessageComposer: typeof _14.PeripheryMessageComposer;
        PeripheryClient: typeof _13.PeripheryClient;
    };
}
//# sourceMappingURL=index.d.ts.map