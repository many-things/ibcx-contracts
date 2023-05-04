import { osmosis } from "osmojs";
const { createRPCQueryClient } = osmosis.ClientFactory;

import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { GasPrice } from "@cosmjs/stargate";
import sdk from "@many-things/ibcx-contracts-sdk";

import { registry, aminoTypes } from "../codec";
import config from "../config";
import { LoadReport } from "../util";

type StoreContractReport = {
  codes: {
    airdrop: number;
    core: number;
    periphery: number;
  };
};

type DeployContractReport = {
  contracts: {
    core: string;
    periphery: string;
  };
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

  const { codes } = LoadReport<StoreContractReport>("3_store");
  const { contracts } = LoadReport<DeployContractReport>("4_deploy");

  const cc = new sdk.Core.CoreClient(base.m, wallet.address, contracts.core);
  const qc = new sdk.Periphery.PeripheryClient(
    base.m,
    wallet.address,
    contracts.periphery
  );

  const migrateCoreResp = await base.m.migrate(
    wallet.address,
    contracts.core,
    codes.core,
    { force: true },
    "auto"
  );
  console.log({
    action: "migrate core",
    txHash: migrateCoreResp.transactionHash,
  });

  const migratePeripheryResp = await base.m.migrate(
    wallet.address,
    contracts.periphery,
    codes.periphery,
    { force: true },
    "auto"
  );
  console.log({
    action: "migrate periphery",
    txHash: migratePeripheryResp.transactionHash,
  });
}

main().catch(console.error);
