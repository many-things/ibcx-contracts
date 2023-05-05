import { DeliverTxResponse } from "@cosmjs/stargate";
import { osmosis } from "osmojs";
import Long from "long";
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";

import config from "../config";
import { ExportReport, LoadReport, makeClient } from "../util";

const { joinPool } = osmosis.gamm.v1beta1.MessageComposer.withTypeUrl;
const { createBalancerPool } =
  osmosis.gamm.poolmodels.balancer.v1beta1.MessageComposer.withTypeUrl;

type CreateDenomReport = {
  denoms: {
    created: string;
    alias: string;
    origin: string;
  }[];
};

type CreatePoolReport = {
  txs: {
    create: DeliverTxResponse;
    join: DeliverTxResponse[];
  };
  poolIds: string[];
};

async function addPool(
  client: SigningCosmWasmClient,
  sender: string,
  denoms: CreateDenomReport["denoms"],
  poolIds: string[]
): Promise<[DeliverTxResponse, string[]]> {
  const joinPoolMsgs = denoms
    .filter(({ origin }) => origin !== "uosmo")
    .map((_, i) =>
      joinPool({
        sender,
        poolId: Long.fromString(poolIds[i]),
        shareOutAmount: "100000000000000000000",
        tokenInMaxs: [],
      })
    );

  const joinPoolResp = await client.signAndBroadcast(
    sender,
    joinPoolMsgs,
    "auto"
  );
  console.log({
    action: "create-pool",
    txHash: joinPoolResp.transactionHash,
  });

  return [joinPoolResp, poolIds];
}

async function createPool(
  client: SigningCosmWasmClient,
  sender: string,
  denoms: CreateDenomReport["denoms"],
  amount: number
): Promise<[DeliverTxResponse, string[]]> {
  const createPoolMsgs = denoms
    .filter(({ origin }) => origin !== "uosmo")
    .map((v) =>
      createBalancerPool({
        sender,
        poolParams: {
          swapFee: "10000000000000000",
          exitFee: "0",
        },
        poolAssets: [
          {
            token: {
              denom: "uosmo",
              amount: `${Math.floor(amount * 1e6)}`,
            },
            weight: "1000000",
          },
          {
            token: {
              denom: v.created,
              amount: `${Math.floor(
                amount * config.args.assets[v.origin].price * 1e6
              )}`,
            },
            weight: "1000000",
          },
        ],
        futurePoolGovernor: config.args.addresses.dao,
      })
    );

  const createPoolResp = await client.signAndBroadcast(
    sender,
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

  return [createPoolResp, poolIds];
}

async function main() {
  const signer = await config.getSigner();
  const [{ address: sender }] = await signer.getAccounts();

  const client = await makeClient(signer);

  const OSMO_AMOUNT = 2_000;

  const { denoms } = LoadReport<CreateDenomReport>("1_setup")!;
  const report = LoadReport<CreatePoolReport>("2_lping");

  if (report) {
    const [tx] = await addPool(client.m, sender, denoms, report.poolIds);

    report.txs.join.push(tx);

    ExportReport<CreatePoolReport>("2_lping", report);
  } else {
    const [tx, poolIds] = await createPool(
      client.m,
      sender,
      denoms,
      OSMO_AMOUNT
    );

    ExportReport<CreatePoolReport>("2_lping", {
      txs: {
        create: tx,
        join: [],
      },
      poolIds,
    });
  }
}

main().catch(console.error);
