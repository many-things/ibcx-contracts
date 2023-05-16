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

  const mintRoutes = Object.entries(config.args.assets)
    .filter(([origin]) => origin !== "uosmo")
    .map(([, { alias }], i) => ({
      key: `uosmo,${alias}`,
      routes: [`${poolIds[i] || 0},uosmo`],
    }));

  const simMintResp = await client.qc.simulateMintExactAmountOut({
    coreAddr: core,
    inputAsset: "uosmo",
    outputAmount: `${1e6}`,
    swapInfo: mintRoutes,
  });
  console.log(simMintResp);

  const mintResp = await client.qc.mintExactAmountOut(
    {
      coreAddr: core,
      inputAsset: "uosmo",
      outputAmount: `${1e6}`,
      swapInfo: mintRoutes,
    },
    "auto",
    undefined,
    [{ denom: "uosmo", amount: "110074876" }]
  );
  console.log({
    action: "mint",
    txHash: mintResp.transactionHash,
  });

  const burnRoutes = Object.entries(config.args.assets)
    .filter(([origin]) => origin !== "uosmo")
    .map(([, { alias }], i) => ({
      key: `${alias},uosmo`,
      routes: [`${poolIds[i] || 0},uosmo`],
    }));

  const simBurnResp = await client.qc.simulateBurnExactAmountIn({
    coreAddr: core,
    inputAmount: `${1e6}`,
    outputAsset: "uosmo",
    swapInfo: burnRoutes,
  });
  console.log(simBurnResp);

  const burnResp = await client.qc.burnExactAmountIn(
    {
      coreAddr: core,
      outputAsset: "uosmo",
      minOutputAmount: `${1e6}`,
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
