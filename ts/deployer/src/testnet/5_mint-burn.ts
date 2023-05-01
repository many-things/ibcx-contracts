import { osmosis } from "osmojs";
const { createRPCQueryClient } = osmosis.ClientFactory;

import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { GasPrice } from "@cosmjs/stargate";
import sdk from "@many-things/ibcx-contracts-sdk";

import config from "../config";
import { aminoTypes, registry } from "../codec";

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
  const [wallet] = await signer.getAccounts();

  const cosmwasmClient = await SigningCosmWasmClient.connectWithSigner(
    config.args.endpoint,
    signer,
    { registry, aminoTypes, gasPrice: GasPrice.fromString("0.025uosmo") }
  );
  const queryClient = await createRPCQueryClient({
    rpcEndpoint: config.args.endpoint,
  });

  const client = {
    b: cosmwasmClient,
    bq: queryClient,
    q: new sdk.Core.CoreQueryClient(cosmwasmClient, config.args.addresses.core),
    m: new sdk.Core.CoreMessageComposer(
      wallet.address,
      config.args.addresses.core
    ),
  };

  const cfg = await client.q.getConfig();
  console.log(cfg);

  const { units } = await client.q.getPortfolio({});
  const funds = units
    .map(([denom, unit]) => ({
      denom,
      amount: `${Math.ceil(Number(unit) * 1e6)}`,
    }))
    .sort((a, b) => (a.denom < b.denom ? -1 : 1));

  const mintMsg = client.m.mint(
    {
      amount: `${1e6}`,
      receiver: wallet.address,
      refundTo: wallet.address,
    },
    funds
  );

  const mintResp = await client.b.signAndBroadcast(
    wallet.address,
    [mintMsg],
    "auto"
  );
  console.log({ action: "mint", txHash: mintResp.transactionHash });

  await checkBalance(
    client.bq,
    wallet.address,
    cfg.index_denom,
    `${1e6}`,
    "mint"
  );

  const burnMsg = client.m.burn({}, [
    { denom: cfg.index_denom, amount: `${1e6}` },
  ]);

  const burnResp = await client.b.signAndBroadcast(
    wallet.address,
    [burnMsg],
    "auto"
  );
  console.log({ action: "burn", txHash: burnResp.transactionHash });

  await checkBalance(
    client.bq,
    wallet.address,
    cfg.index_denom,
    `${0}`,
    "burn"
  );
}

main().catch(console.error);
