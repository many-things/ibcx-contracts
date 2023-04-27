import { osmosis } from "osmojs";
const { createRPCQueryClient } = osmosis.ClientFactory;

import { GasPrice } from "@cosmjs/stargate";
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import CoreTypes from "@many-things/ibcx-contracts-sdk/types/contracts/Core.types";
import PeripheryTypes from "@many-things/ibcx-contracts-sdk/types/contracts/Periphery.types";

import config from "./config";
import { AssetInfo } from "./portfolio";
import { registry, aminoTypes } from "./codec";

async function main() {
  const signer = await config.getSigner();
  const [wallet] = await signer.getAccounts();

  const cosmwasmClient = await SigningCosmWasmClient.connectWithSigner(
    config.args.endpoint,
    signer,
    { registry, aminoTypes, gasPrice: GasPrice.fromString("0.025uosmo") }
  );
  const queryClient = await createRPCQueryClient({
    rpcEndpoint: config.args.endpoint,
  });

  const { params } = await queryClient.osmosis.tokenfactory.v1beta1.params();

  // deploy core
  const initCoreMsg: CoreTypes.InstantiateMsg = {
    fee: {
      collector: config.args.addresses.dao,
      burn_fee: null,
      streaming_fee: null,
    },
    gov: config.args.addresses.dao,
    index_denom: "uibcx",
    index_units: AssetInfo.map(({ denom, unit }) => [denom, `${unit}`]),
    reserve_denom: "uosmo",
  };
  const initCoreRes = await cosmwasmClient.instantiate(
    wallet.address,
    config.args.codes.core,
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
  const initPeripheryRes = await cosmwasmClient.instantiate(
    wallet.address,
    config.args.codes.periphery,
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

  // const airdrop = new sdk.Airdrop.AirdropClient(cwc, "", "");
  // const core = new sdk.Core.CoreClient(cwc, "", "");
  // const faucet = new sdk.Faucet.FaucetClient(cwc, "", "");
  // const periphery = new sdk.Periphery.PeripheryClient(cwc, "", "");

  // console.log(config);
}

main().catch(console.error);
