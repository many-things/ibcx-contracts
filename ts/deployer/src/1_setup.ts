import { SigningStargateClient } from "@cosmjs/stargate";
import { osmosis } from "osmojs";
import config from "./config";

const { createRPCQueryClient } = osmosis.ClientFactory;
const { createDenom, mint, burn, setBeforeSendHook } =
  osmosis.tokenfactory.v1beta1.MessageComposer.withTypeUrl;

async function main() {
  const signer = await config.getSigner();
  const [wallet] = await signer.getAccounts();

  console.log(wallet.address);

  const stargateClient = await SigningStargateClient.connectWithSigner(
    config.args.endpoint,
    signer
  );
  const queryClient = await createRPCQueryClient({
    rpcEndpoint: config.args.endpoint,
  });

  const createDenomRes = await stargateClient.signAndBroadcast(
    wallet.address,
    [
      createDenom({ sender: wallet.address, subdenom: "utatom" }),
      createDenom({ sender: wallet.address, subdenom: "utosmo " }),
      createDenom({ sender: wallet.address, subdenom: "utjuno " }),
      createDenom({ sender: wallet.address, subdenom: "utscrt" }),
      createDenom({ sender: wallet.address, subdenom: "utevmos" }),
      createDenom({ sender: wallet.address, subdenom: "utstars" }),
      createDenom({ sender: wallet.address, subdenom: "utakt" }),
      createDenom({ sender: wallet.address, subdenom: "utaxl" }),
      createDenom({ sender: wallet.address, subdenom: "utregen" }),
      createDenom({ sender: wallet.address, subdenom: "utstrd" }),
      createDenom({ sender: wallet.address, subdenom: "utumee" }),
      createDenom({ sender: wallet.address, subdenom: "ution" }),
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
