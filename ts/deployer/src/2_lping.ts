import { GasPrice, SigningStargateClient } from "@cosmjs/stargate";
import { osmosis } from "osmojs";

import config from "./config";
import { registry, aminoTypes } from "./codec";
import Long from "long";
import { AssetInfo } from "./portfolio";

const { createRPCQueryClient } = osmosis.ClientFactory;
const { createDenom, mint, burn, setBeforeSendHook } =
  osmosis.tokenfactory.v1beta1.MessageComposer.withTypeUrl;
const { joinPool } = osmosis.gamm.v1beta1.MessageComposer.withTypeUrl;
const { createBalancerPool } =
  osmosis.gamm.poolmodels.balancer.v1beta1.MessageComposer.withTypeUrl;

async function main() {
  const signer = await config.getSigner();
  const [wallet] = await signer.getAccounts();

  const stargateClient = await SigningStargateClient.connectWithSigner(
    config.args.endpoint,
    signer,
    { registry, aminoTypes, gasPrice: GasPrice.fromString("0.025uosmo") }
  );
  const queryClient = await createRPCQueryClient({
    rpcEndpoint: config.args.endpoint,
  });

  const OSMO_AMOUNT = 1000;

  for (const { denom, poolID, price } of AssetInfo) {
    if (denom === "uosmo") {
      continue;
    }

    if (!poolID) {
      const createPoolMsg = createBalancerPool({
        sender: wallet.address,
        poolParams: {
          swapFee: "0",
          exitFee: "0",
        },
        poolAssets: [
          // 1000 OSMO
          {
            token: {
              denom: "uosmo",
              amount: `${Math.floor(OSMO_AMOUNT * 1e6)}`,
            },
            weight: "1",
          },
          {
            token: {
              denom,
              amount: `${Math.floor(OSMO_AMOUNT * price * 1e6)}`,
            },
            weight: "1",
          },
        ],
        futurePoolGovernor: config.args.addresses.dao,
      });
      const createPoolResp = await stargateClient.signAndBroadcast(
        wallet.address,
        [createPoolMsg],
        "auto"
      );
      console.log({
        action: "create-pool",
        txHash: createPoolResp.transactionHash,
      });
    } else {
      const joinPoolMsg = joinPool({
        sender: wallet.address,
        poolId: Long.fromNumber(poolID!),
        shareOutAmount: "100000000000000000000",
        tokenInMaxs: [
          { denom, amount: `${Math.floor(OSMO_AMOUNT * price * 1e6)}` },
          { denom: "uosmo", amount: `${Math.floor(OSMO_AMOUNT * 1e6)}` },
        ],
      });
      const joinPoolResp = await stargateClient.signAndBroadcast(
        wallet.address,
        [joinPoolMsg],
        "auto"
      );
      console.log({
        action: "join-pool",
        txHash: joinPoolResp.transactionHash,
      });
    }
  }
}

main().catch(console.error);
