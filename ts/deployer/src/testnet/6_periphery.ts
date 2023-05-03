import { osmosis } from "osmojs";
const { createRPCQueryClient } = osmosis.ClientFactory;

import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { GasPrice } from "@cosmjs/stargate";
import sdk from "@many-things/ibcx-contracts-sdk";

import config from "../config";
import { aminoTypes, registry } from "../codec";
import { LoadReport } from "../util";

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
  const [wallet] = await signer.getAccounts();

  const base = {
    m: await SigningCosmWasmClient.connectWithSigner(
      config.args.endpoint,
      signer,
      { registry, aminoTypes, gasPrice: GasPrice.fromString("0.025uosmo") }
    ),
    q: await createRPCQueryClient({
      rpcEndpoint: config.args.endpoint,
    }),
  };

  const { denoms } = LoadReport<CreateDenomReport>("1_setup");
  const { poolIds } = LoadReport<CreatePoolReport>("2_lping");
  const { core, periphery } =
    LoadReport<DeployContractReport>("4_deploy").contracts;

  const client = {
    b: base.m,
    q: base.q,
    cc: new sdk.Core.CoreClient(base.m, wallet.address, core),
    qc: new sdk.Periphery.PeripheryClient(base.m, wallet.address, periphery),
  };

  const { index_denom } = await client.cc.getConfig();

  const mintResp = await client.qc.mintExactAmountOut(
    {
      coreAddr: core,
      inputAsset: "uosmo",
      outputAmount: `${1e6}`,
      swapInfo: denoms
        .filter(({ origin }) => origin !== "uosmo")
        .map(({ created }, i) => [
          ["uosmo", created],
          [{ pool_id: Number(poolIds[i]) || 0, token_denom: "uosmo" }],
        ]),
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
      swapInfo: denoms
        .filter(({ origin }) => origin !== "uosmo")
        .map(({ created }, i) => [
          [created, "uosmo"],
          [{ pool_id: Number(poolIds[i]) || 0, token_denom: "uosmo" }],
        ]),
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
