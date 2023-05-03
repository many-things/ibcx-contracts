import { osmosis } from "osmojs";
const { createRPCQueryClient } = osmosis.ClientFactory;

import { GasPrice } from "@cosmjs/stargate";
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import CoreTypes from "@many-things/ibcx-contracts-sdk/types/contracts/Core.types";
import PeripheryTypes from "@many-things/ibcx-contracts-sdk/types/contracts/Periphery.types";

import config from "../config";
import { registry, aminoTypes } from "../codec";
import { ExportReport, LoadReport } from "../util";

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
  const [wallet] = await signer.getAccounts();

  const client = {
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
  const { codes } = LoadReport<StoreContractReport>("3_store");

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
    index_units: denoms.map(({ origin, created }) => [
      created,
      `${config.args.assets[origin].unit}`,
    ]),
    reserve_denom: "uosmo",
  };
  const initCoreRes = await client.m.instantiate(
    wallet.address,
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
    wallet.address,
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
