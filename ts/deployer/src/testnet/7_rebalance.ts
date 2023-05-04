import { osmosis, cosmwasm } from "osmojs";
const { createRPCQueryClient } = osmosis.ClientFactory;

import {
  ExecuteResult,
  SigningCosmWasmClient,
} from "@cosmjs/cosmwasm-stargate";
import { GasPrice } from "@cosmjs/stargate";
import sdk from "@many-things/ibcx-contracts-sdk";

import config from "../config";
import { aminoTypes, registry } from "../codec";
import { LoadReport } from "../util";

type CreateDenomReport = {
  denoms: {
    created: string;
    alias: string;
    origin: string;
  }[];
};

type CreatePoolReport = {
  poolIds: string[];
};

type DeployContractReport = {
  contracts: {
    core: string;
    periphery: string;
  };
};

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

  const { denoms } = LoadReport<CreateDenomReport>("1_setup")!;
  const { poolIds } = LoadReport<CreatePoolReport>("2_lping")!;
  const { contracts: addrs } = LoadReport<DeployContractReport>("4_deploy")!;

  const client = {
    core: new sdk.Core.CoreClient(base.m, sender, addrs.core),
    perp: new sdk.Periphery.PeripheryClient(base.m, sender, addrs.periphery),
    ...base,
  };

  const resolveDenom = (denom: string): [string, number] => {
    const i = denoms.findIndex(({ origin }) => origin === denom)!;
    return [denoms[i].created, i];
  };
  const resolvePool = (denom: string): number => {
    const [, i] = resolveDenom(denom);
    return Number(poolIds[i]);
  };

  const cfg = await client.core.getConfig({});
  const portfolio = await client.core.getPortfolio({});
  console.table(Object.fromEntries(portfolio.units));

  const tradeInfoResps: ExecuteResult[] = [];
  for (const denom of ["ujuno", "ustrd"]) {
    const updateTradeInfoResp = await client.core.gov(
      {
        update_trade_info: {
          cooldown: 1,
          denom: resolveDenom(denom)[0],
          max_trade_amount: `${100_000_000_000_000}`,
          routes: [
            { pool_id: resolvePool(denom), token_denom: cfg.reserve_denom },
          ],
        },
      },
      "auto"
    );
    console.log({
      action: "update_trade_info",
      txHash: updateTradeInfoResp.transactionHash,
    });
    tradeInfoResps.push(updateTradeInfoResp);
  }

  const initRebalanceResp = await client.core.rebalance(
    {
      init: {
        deflation: [[resolveDenom("ujuno")[0], "1.0"]], // make ujuno's unit to 1.0
        inflation: [[resolveDenom("ustrd")[0], "1.0"]], // distribute liquidated ujuno to ustrd
        manager: sender,
      },
    },
    "auto"
  );

  const deflateResp = await client.core.rebalance(
    {
      trade: {
        deflate: {
          amount_out: `${1e6}`,
          max_amount_in: `${100_000 * 1e6}`,
          target_denom: resolveDenom("ujuno")[0],
        },
      },
    },
    "auto"
  );

  const inflateResp = await client.core.rebalance(
    {
      trade: {
        inflate: {
          amount_in: `${1e6}`,
          min_amount_out: "0",
          target_denom: resolveDenom("ustrd")[0],
        },
      },
    },
    "auto"
  );

  const finishRebalanceResp = await client.core.rebalance(
    {
      finalize: {},
    },
    "auto"
  );
}

main().catch(console.error);
