import sdk from "@many-things/ibcx-contracts-sdk";

import config from "../config";
import { LoadReport, makeClient } from "../util";

type DeployContractReport = {
  contracts: {
    core: string;
    periphery: string;
  };
};

type CreatePoolReport = {
  poolIds: string[];
};

type CreateDenomReport = {
  denoms: {
    created: string;
    alias: string;
    origin: string;
  }[];
};

async function main() {
  const signer = await config.getSigner();
  const [{ address: sender }] = await signer.getAccounts();

  const base = await makeClient(signer);

  const { denoms } = LoadReport<CreateDenomReport>("1_setup")!;
  const { poolIds } = LoadReport<CreatePoolReport>("2_lping")!;
  const { core, periphery } =
    LoadReport<DeployContractReport>("4_deploy")!.contracts;

  const client = {
    b: base.m,
    q: base.q,
    cc: new sdk.Core.CoreClient(base.m, sender, core),
    qc: new sdk.Periphery.PeripheryClient(base.m, sender, periphery),
  };

  const { index_denom } = await client.cc.getConfig({});

  const mintRoutes = denoms
    .filter(({ origin }) => origin !== "uosmo")
    .map(({ created }, i) => ({
      key: `uosmo,${created}`,
      routes: [`${poolIds[i] || 0},uosmo`],
    }));

  const burnRoutes = denoms
    .filter(({ origin }) => origin !== "uosmo")
    .map(({ created }, i) => ({
      key: `${created},uosmo`,
      routes: [`${poolIds[i] || 0},uosmo`],
    }));

  const simMintResp = await client.qc.simulateMintExactAmountOut({
    coreAddr: core,
    inputAsset: "uosmo",
    outputAmount: `${1e6}`,
    swapInfo: mintRoutes,
  });
  console.log(simMintResp);

  const simBurnResp = await client.qc.simulateBurnExactAmountIn({
    coreAddr: core,
    inputAmount: `${1e6}`,
    outputAsset: "uosmo",
    swapInfo: burnRoutes,
  });
  console.log(simBurnResp);

  const mintResp = await client.qc.mintExactAmountOut(
    {
      coreAddr: core,
      inputAsset: "uosmo",
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
