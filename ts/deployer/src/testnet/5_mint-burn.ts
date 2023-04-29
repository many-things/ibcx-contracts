import { osmosis } from "osmojs";
const { createRPCQueryClient } = osmosis.ClientFactory;

import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { GasPrice } from "@cosmjs/stargate";

import config from "../config";
import { aminoTypes, registry } from "../codec";

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

  const { balances } = await queryClient.cosmos.bank.v1beta1.allBalances({
    address: wallet.address,
  });
  console.log(balances);
}

main().catch(console.error);
