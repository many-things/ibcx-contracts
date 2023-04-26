import { SigningStargateClient } from "@cosmjs/stargate";
import { osmosis } from "osmojs";
import config from "./config";

const { createRPCQueryClient } = osmosis.ClientFactory;
const { createDenom, mint, burn, setBeforeSendHook } =
  osmosis.tokenfactory.v1beta1.MessageComposer.withTypeUrl;
const { createBalancerPool } =
  osmosis.gamm.poolmodels.balancer.v1beta1.MessageComposer.withTypeUrl;

async function main() {
  const signer = await config.getSigner();
  const [wallet] = await signer.getAccounts();

  const stargateClient = await SigningStargateClient.connectWithSigner(
    config.args.endpoint,
    signer
  );
  const queryClient = await createRPCQueryClient({
    rpcEndpoint: config.args.endpoint,
  });

  const msgCreatePool = createBalancerPool({
    sender: wallet.address,
    poolAssets: [
      { token: { denom: "", amount: "" }, weight: "1.0" },
      { token: { denom: "", amount: "" }, weight: "1.0" },
    ],
    futurePoolGovernor: config.args.addresses.dao,
  });
}

main().catch(console.error);
