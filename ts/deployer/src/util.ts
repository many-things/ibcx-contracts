import { osmosis } from "osmojs";
const { createRPCQueryClient } = osmosis.ClientFactory;

import { mkdirSync, readFileSync, rmSync, writeFileSync } from "fs";
import { join } from "path";
import { OfflineSigner } from "@cosmjs/proto-signing";
import { GasPrice } from "@cosmjs/stargate";

import config, { NETWORK } from "./config";
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { aminoTypes, registry } from "./codec";

export async function makeClient(signer: OfflineSigner) {
  return {
    m: await SigningCosmWasmClient.connectWithSigner(
      config.args.endpoint,
      signer,
      { registry, aminoTypes, gasPrice: GasPrice.fromString("0.025uosmo") }
    ),
    q: await createRPCQueryClient({
      rpcEndpoint: config.args.endpoint,
    }),
  };
}

export function ExportReport<T>(subject: string, output: T) {
  const base = join(process.cwd(), "out", subject, NETWORK);
  mkdirSync(base, { recursive: true });

  const fileLatest = "run-latest.json";
  rmSync(join(base, fileLatest), { force: true });

  const fileTime = `run-${Math.floor(new Date().getTime() / 1000)}.json`;

  const fileData = JSON.stringify(output, null, 2);
  writeFileSync(join(base, fileLatest), fileData);
  writeFileSync(join(base, fileTime), fileData);
}

export function LoadReport<T>(subject: string): T | undefined {
  const base = join(process.cwd(), "out", subject, NETWORK);

  try {
    return JSON.parse(readFileSync(join(base, "run-latest.json"), "utf-8"));
  } catch {
    return undefined;
  }
}
