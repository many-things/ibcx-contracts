import { execSync } from "child_process";
import { osmosis } from "osmojs";

import config, { NETWORK } from "../config";
import { ExportReport, makeClient } from "../util";

const { createDenom } =
  osmosis.tokenfactory.v1beta1.MessageComposer.withTypeUrl;

async function main() {
  if (NETWORK === "mainnet") {
    throw Error("This script is only for testnet / localnet");
  }

  const signer = await config.getSigner();
  const [wallet] = await signer.getAccounts();

  console.log(wallet.address);

  const client = await makeClient(signer);

  const { balances } = await client.q.cosmos.bank.v1beta1.allBalances({
    address: wallet.address,
  });
  console.log(balances);

  const denoms = Object.entries(config.args.assets)
    .filter(([_, v]) => v.alias)
    .map(([origin, { alias }]) => ({ origin, alias }));

  const createDenomRes = await client.m.signAndBroadcast(
    wallet.address,
    denoms.map(({ alias: subdenom }) =>
      createDenom({ sender: wallet.address, subdenom: subdenom! })
    ),
    "auto"
  );
  console.log({
    action: "create denom",
    txHash: createDenomRes.transactionHash,
  });

  const created = createDenomRes.events
    .filter((v) => v.type === "create_denom")
    .map((v) => v.attributes[1].value);

  const mintCommand = (denom: string) =>
    `${
      config.command
    } tx tokenfactory mint ${1_000_000_000_000_000}${denom} --from ${
      config.args.keyring.name
    } ${
      config.args.keyring.backend
        ? "--keyring-backend " + config.args.keyring.backend
        : ""
    } --gas auto --gas-adjustment 1.2 --fees 1500uosmo -b block -y --output json`;

  const mintTxs: { txhash: string }[] = [];
  for (const denom of created) {
    const mintRes: { txhash: string } = JSON.parse(
      execSync(mintCommand(denom)).toString("utf-8")
    );

    console.log({
      action: `mint ${denom}`,
      txHash: mintRes.txhash,
    });

    mintTxs.push(mintRes);
  }

  ExportReport("1_setup", {
    txs: {
      create: createDenomRes,
      mint: mintTxs,
    },
    denoms: denoms.map((v) => ({
      created: created.find((c) => c.includes(v.alias!)),
      ...v,
    })),
  });
}

main().catch(console.error);
