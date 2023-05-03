import { GasPrice, SigningStargateClient } from "@cosmjs/stargate";
import { osmosis } from "osmojs";

import config from "../config";
import { registry, aminoTypes } from "../codec";
import { ExportReport, LoadReport } from "../util";

const { createRPCQueryClient } = osmosis.ClientFactory;
const { createBalancerPool } =
  osmosis.gamm.poolmodels.balancer.v1beta1.MessageComposer.withTypeUrl;

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
    m: await SigningStargateClient.connectWithSigner(
      config.args.endpoint,
      signer,
      { registry, aminoTypes, gasPrice: GasPrice.fromString("0.025uosmo") }
    ),
    q: await createRPCQueryClient({
      rpcEndpoint: config.args.endpoint,
    }),
  };

  const OSMO_AMOUNT = 2_000;

  const { denoms } = LoadReport<CreateDenomReport>("1_setup");

  const createPoolMsgs = denoms
    .filter(({ origin }) => origin !== "uosmo")
    .map((v) =>
      createBalancerPool({
        sender: wallet.address,
        poolParams: {
          swapFee: "10000000000000000",
          exitFee: "0",
        },
        poolAssets: [
          {
            token: {
              denom: "uosmo",
              amount: `${Math.floor(OSMO_AMOUNT * 1e6)}`,
            },
            weight: "1000000",
          },
          {
            token: {
              denom: v.created,
              amount: `${Math.floor(
                OSMO_AMOUNT * config.args.assets[v.origin].price * 1e6
              )}`,
            },
            weight: "1000000",
          },
        ],
        futurePoolGovernor: config.args.addresses.dao,
      })
    );

  const createPoolResp = await client.m.signAndBroadcast(
    wallet.address,
    createPoolMsgs,
    "auto"
  );
  console.log({
    action: "create-pool",
    txHash: createPoolResp.transactionHash,
  });

  const poolIds = [
    ...new Set(
      createPoolResp.events
        .filter((v) => v.type === "pool_created")
        .map((v) => v.attributes[0].value)
    ),
  ];

  ExportReport("2_lping", {
    txs: {
      create: createPoolResp,
    },
    poolIds,
  });
}

main().catch(console.error);
