import { osmosis } from "osmojs";
const { createRPCQueryClient } = osmosis.ClientFactory;

import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { GasPrice } from "@cosmjs/stargate";
import sdk from "@many-things/ibcx-contracts-sdk";

import config from "../config";
import { aminoTypes, registry } from "../codec";
import { LoadReport } from "../util";

type CreateDenomReport = {
  denoms: {
    created: string;
    alias: string;
    origin: string;
  }[];
};

type DeployContractReport = {
  contracts: {
    core: string;
    periphery: string;
  };
};

async function main() {
  const signer = await config.getSigner();
  const [{ address: sender }] = await signer.getAccounts();

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

  const { denoms } = LoadReport<CreateDenomReport>("1_setup")!;
  const { contracts: addrs } = LoadReport<DeployContractReport>("4_deploy")!;

  const client = {
    core: new sdk.Core.CoreClient(base.m, sender, addrs.core),
    perp: new sdk.Periphery.PeripheryClient(base.m, sender, addrs.periphery),
    ...base,
  };

  await client.core.rebalance(
    {
      init: {
        deflation: [
          [denoms.find(({ origin }) => origin === "ujuno")!.created, "1.0"],
        ],
        inflation: [["", ""]],
        manager: sender,
      },
    },
    "auto"
  );
}

main().catch(console.error);
