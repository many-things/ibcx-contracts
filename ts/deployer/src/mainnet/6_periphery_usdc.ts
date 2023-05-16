import sdk from "@many-things/ibcx-contracts-sdk";

import config from "../config";
import { LoadReport, makeClient } from "../util";

type DeployContractReport = {
  contracts: {
    core: string;
    periphery: string;
  };
};

async function main() {
  const signer = await config.getSigner();
  const [{ address: sender }] = await signer.getAccounts();

  const base = await makeClient(signer);

  const usdc =
    "ibc/D189335C6E4A68B513C10AB227BF1C1D38C746766278BA3EEB4FB14124F1D858";
  const usdcPool = 678;
  const poolIds = [3, 1, 812, 722, 2, 497, 42, 584, 604, 806, 641];
  const { core, periphery } =
    LoadReport<DeployContractReport>("4_deploy")!.contracts;

  const client = {
    b: base.m,
    q: base.q,
    cc: new sdk.Core.CoreClient(base.m, sender, core),
    qc: new sdk.Periphery.PeripheryClient(base.m, sender, periphery),
  };

  const { index_denom } = await client.cc.getConfig({});

  const mintRoutes = [
    {
      key: `${usdc},uosmo`,
      routes: [`${usdcPool},${usdc}`],
    },
    ...Object.entries(config.args.assets)
      .filter(([origin]) => origin !== "uosmo")
      .map(([, { alias }], i) => ({
        key: `${usdc},${alias}`,
        routes: [`${usdcPool},${usdc}`, `${poolIds[i] || 0},uosmo`],
      })),
  ];

  const simMintResp = await client.qc.simulateMintExactAmountOut({
    coreAddr: core,
    inputAsset: usdc,
    outputAmount: `${1e6}`,
    swapInfo: mintRoutes,
  });
  console.log(simMintResp);

  const mintResp = await client.qc.mintExactAmountOut(
    {
      coreAddr: core,
      inputAsset: usdc,
      outputAmount: `${1e6}`,
      swapInfo: mintRoutes,
    },
    "auto",
    undefined,
    [{ denom: "uosmo", amount: `${1000 * 1e6}` }]
  );
  console.log({
    action: "mint",
    txHash: mintResp.transactionHash,
  });

  const burnRoutes = [
    {
      key: `uosmo,${usdc}`,
      routes: [`${usdcPool},${usdc}`],
    },
    ...Object.entries(config.args.assets)
      .filter(([origin]) => origin !== "uosmo")
      .map(([, { alias }], i) => ({
        key: `${alias},${usdc}`,
        routes: [`${poolIds[i] || 0},uosmo`, `${usdcPool},${usdc}`],
      })),
  ];

  const simBurnResp = await client.qc.simulateBurnExactAmountIn({
    coreAddr: core,
    inputAmount: `${1e6}`,
    outputAsset: usdc,
    swapInfo: burnRoutes,
  });
  console.log(simBurnResp);

  const burnResp = await client.qc.burnExactAmountIn(
    {
      coreAddr: core,
      outputAsset: usdc,
      minOutputAmount: `${Number(simBurnResp.swap_result_amount.amount) + 1e6}`,
      swapInfo: burnRoutes,
    },
    "auto",
    undefined,
    [{ denom: index_denom, amount: `${1e6}` }]
  );
  console.log({
    action: "burn",
    txHash: burnResp.transactionHash,
  });
}

main().catch(console.error);
