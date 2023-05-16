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

async function main() {
  const signer = await config.getSigner();
  const [{ address: sender }] = await signer.getAccounts();
  console.log(sender);

  const client = await makeClient(signer);

  const { codes } = LoadReport<StoreContractReport>("3_store")!;

  const { params } = await client.q.osmosis.tokenfactory.v1beta1.params();

  // deploy core
  const initCoreMsg: CoreTypes.InstantiateMsg = {
    fee: {
      collector: config.args.addresses.dao,
      burn_fee: "0.15",
      streaming_fee: {
        rate: "0.000000000173926",
        freeze: false,
      },
    },
    gov: config.args.addresses.dao,
    index_denom: "uibcx",
    index_units: Object.entries(config.args.assets).map(
      ([origin, { alias, unit }]) => [alias || origin, `${unit}`]
    ),
    reserve_denom: "uosmo",
  };
  console.log(initCoreMsg);

  const initCoreRes = await client.m.instantiate(
    sender,
    codes.core,
    initCoreMsg,
    "IBCX Core <Product of ION DAO>",
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
    "IBCX Periphery <Product of ION DAO>",
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
