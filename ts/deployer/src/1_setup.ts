import { GasPrice, SigningStargateClient } from "@cosmjs/stargate";
import { osmosis } from "osmojs";

import config from "./config";
import { registry, aminoTypes } from "./codec";

const { createRPCQueryClient } = osmosis.ClientFactory;
const { createDenom, mint, burn, setBeforeSendHook } =
  osmosis.tokenfactory.v1beta1.MessageComposer.withTypeUrl;

async function main() {
  const signer = await config.getSigner();
  const [wallet] = await signer.getAccounts();

  console.log(wallet.address);

  const stargateClient = await SigningStargateClient.connectWithSigner(
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

  const createDenomRes = await stargateClient.signAndBroadcast(
    wallet.address,
    [
      createDenom({ sender: wallet.address, subdenom: "ibcx-test0-uatom" }),
      createDenom({ sender: wallet.address, subdenom: "ibcx-test0-ujuno" }),
      createDenom({ sender: wallet.address, subdenom: "ibcx-test0-uscrt" }),
      createDenom({ sender: wallet.address, subdenom: "ibcx-test0-uevmos" }),
      createDenom({ sender: wallet.address, subdenom: "ibcx-test0-ustars" }),
      createDenom({ sender: wallet.address, subdenom: "ibcx-test0-uakt" }),
      createDenom({ sender: wallet.address, subdenom: "ibcx-test0-uaxl" }),
      createDenom({ sender: wallet.address, subdenom: "ibcx-test0-uregen" }),
      createDenom({ sender: wallet.address, subdenom: "ibcx-test0-ustrd" }),
      createDenom({ sender: wallet.address, subdenom: "ibcx-test0-uumee" }),
      createDenom({ sender: wallet.address, subdenom: "ibcx-test0-uion" }),
    ],
    "auto"
  );
  console.log({
    action: "create denom",
    txHash: createDenomRes.transactionHash,
  });

  const { denoms: denomsCreated } =
    await queryClient.osmosis.tokenfactory.v1beta1.denomsFromCreator({
      creator: wallet.address,
    });
  console.log(denomsCreated);
}

main().catch(console.error);
