import { osmosis } from "osmojs";
const { createRPCQueryClient } = osmosis.ClientFactory;

import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { GasPrice } from "@cosmjs/stargate";
import sdk from "@many-things/ibcx-contracts-sdk";

import config from "../config";
import { aminoTypes, registry } from "../codec";
import { LoadReport } from "../util";

type DeployContractReport = {
  contracts: {
    core: string;
    periphery: string;
  };
};

async function checkBalance(
  client: Awaited<ReturnType<typeof createRPCQueryClient>>,
  address: string,
  denom: string,
  expected: string,
  action?: string
) {
  const { balance: ibcBalance } = await client.cosmos.bank.v1beta1.balance({
    address,
    denom,
  });

  if (
    JSON.stringify(ibcBalance) !== JSON.stringify({ denom, amount: expected })
  ) {
    throw Error(`${action || "balance check"} failed`);
  }
}

async function main() {
  const signer = await config.getSigner();
  const [{ address: sender }] = await signer.getAccounts();

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

  const { contracts: addrs } = LoadReport<DeployContractReport>("4_deploy")!;

  const client = {
    core: new sdk.Core.CoreClient(base.m, sender, addrs.core),
    perp: new sdk.Periphery.PeripheryClient(base.m, sender, addrs.periphery),
    ...base,
  };

  const { index_denom } = await client.core.getConfig({});

  const { units } = await client.core.getPortfolio({});
  const funds = units
    .map(([denom, unit]) => ({
      denom,
      amount: `${Math.ceil(Number(unit) * 1e6)}`,
    }))
    .sort((a, b) => (a.denom < b.denom ? -1 : 1));

  const mintResp = await client.core.mint(
    { amount: `${1e6}` },
    "auto",
    undefined,
    funds
  );
  console.log({ action: "mint", txHash: mintResp.transactionHash });

  await checkBalance(client.q, sender, index_denom, `${1e6}`, "mint");

  const burnAmount = [{ denom: index_denom, amount: `${1e6}` }];
  const burnResp = await client.core.burn({}, "auto", undefined, burnAmount);
  console.log({ action: "burn", txHash: burnResp.transactionHash });

  await checkBalance(client.q, sender, index_denom, `${0}`, "burn");
}

main().catch(console.error);
