import { osmosis } from "osmojs";
const { createRPCQueryClient } = osmosis.ClientFactory;

import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { GasPrice } from "@cosmjs/stargate";
import { registry, aminoTypes } from "../codec";
import config from "../config";
import { LoadReport } from "../util";
import Long from "long";

const { exitPool } = osmosis.gamm.v1beta1.MessageComposer.withTypeUrl;

type CreatePoolReport = {
  poolIds: string[];
};

async function main() {
  const signer = await config.getSigner();
  const [wallet] = await signer.getAccounts();

  const base = {
    m: await SigningCosmWasmClient.connectWithSigner(
      config.args.endpoint,
      signer,
      { registry, aminoTypes, gasPrice: GasPrice.fromString("0.025uosmo") }
    ),
    q: await createRPCQueryClient({
      rpcEndpoint: config.args.endpoint,
    }),
  };

  const { poolIds } = LoadReport<CreatePoolReport>("2_lping")!;
  const { balances } = await base.q.cosmos.bank.v1beta1.allBalances({
    address: wallet.address,
  });

  const exitPoolMsgs = poolIds.map((v) =>
    exitPool({
      sender: wallet.address,
      poolId: Long.fromString(v),
      shareInAmount: Long.fromString(
        balances.find(({ denom }) => denom === `gamm/pool/${v}`)!.amount
      )
        .sub(1)
        .toString(),
      tokenOutMins: [],
    })
  );

  const exitPoolResp = await base.m.signAndBroadcast(
    wallet.address,
    exitPoolMsgs,
    "auto"
  );

  console.log({
    action: "exit pool",
    txhash: exitPoolResp.transactionHash,
  });
}

main().catch(console.error);
