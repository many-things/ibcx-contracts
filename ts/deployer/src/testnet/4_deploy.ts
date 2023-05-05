import { osmosis } from "osmojs";

import CoreTypes from "@many-things/ibcx-contracts-sdk/types/contracts/Core.types";
import PeripheryTypes from "@many-things/ibcx-contracts-sdk/types/contracts/Periphery.types";

import config from "../config";
import { ExportReport, LoadReport, makeClient } from "../util";

type StoreContractReport = {
  codes: {
    airdrop: number;
    core: number;
    periphery: number;
  };
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

  const client = await makeClient(signer);

  const { denoms } = LoadReport<CreateDenomReport>("1_setup")!;
  const { codes } = LoadReport<StoreContractReport>("3_store")!;

  const { params } = await client.q.osmosis.tokenfactory.v1beta1.params();

  // deploy core
  const initCoreMsg: CoreTypes.InstantiateMsg = {
    fee: {
      collector: config.args.addresses.dao,
      burn_fee: null,
      streaming_fee: null,
    },
    gov: config.args.addresses.dao,
    index_denom: "uibcx",
    index_units: Object.entries(config.args.assets).map(
      ([origin, { unit }]) => [
        denoms.find((d) => d.origin === origin)?.created || origin,
        `${unit}`,
      ]
    ),
    reserve_denom: "uosmo",
  };
  const initCoreRes = await client.m.instantiate(
    sender,
    codes.core,
    initCoreMsg,
    "ibcx-core",
    "auto",
    {
      admin: config.args.addresses.dao,
      funds: params?.denomCreationFee,
    }
  );
  console.log({
    action: "deploy ibcx-core",
    deployed: initCoreRes.contractAddress,
    txHash: initCoreRes.transactionHash,
  });

  // deploy periphery
  const initPeripheryMsg: PeripheryTypes.InstantiateMsg = {};
  const initPeripheryRes = await client.m.instantiate(
    sender,
    codes.periphery,
    initPeripheryMsg,
    "ibcx-periphery",
    "auto",
    {
      admin: config.args.addresses.dao,
    }
  );
  console.log({
    action: "deploy ibcx-periphery",
    deployed: initPeripheryRes.contractAddress,
    txHash: initPeripheryRes.transactionHash,
  });

  ExportReport("4_deploy", {
    txs: {
      core: initCoreRes,
      periphery: initPeripheryRes,
    },
    contracts: {
      core: initCoreRes.contractAddress,
      periphery: initPeripheryRes.contractAddress,
    },
  });
}

main().catch(console.error);
