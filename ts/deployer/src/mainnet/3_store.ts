import { osmosis } from "osmojs";
const { createRPCQueryClient } = osmosis.ClientFactory;

import { SigningCosmWasmClient, UploadResult } from "@cosmjs/cosmwasm-stargate";
import path from "path";
import { readFileSync, readdirSync } from "fs";
import { GasPrice } from "@cosmjs/stargate";

import config, { NETWORK } from "../config";
import { aminoTypes, registry } from "../codec";
import { ExportReport } from "../util";

const CONTRACTS = ["airdrop", "core", "periphery"];

const toName = (fileName: string) =>
  fileName.replace(".wasm", "").replace("ibcx_", "");
const pwd = process.cwd();
const root = path.join(`${pwd}/../../`);
const artifactsPath = path.join(
  root,
  NETWORK === "localnet" ? "target/wasm32-unknown-unknown/release" : "artifacts"
);
const artifacts = readdirSync(artifactsPath)
  .filter((file) => file.includes(".wasm"))
  .filter((file) => CONTRACTS.includes(toName(file)))
  .map((v) => [v, path.join(artifactsPath, v)]);
console.log(artifactsPath, "=>", artifacts);

async function main() {
  const signer = await config.getSigner();
  const [{ address: sender }] = await signer.getAccounts();

  const client = {
    m: await SigningCosmWasmClient.connectWithSigner(
      config.args.endpoint,
      signer,
      { registry, aminoTypes, gasPrice: GasPrice.fromString("0.025uosmo") }
    ),
    q: await createRPCQueryClient({
      rpcEndpoint: config.args.endpoint,
    }),
  };

  console.log(sender);

  const uploadTxs: UploadResult[] = [];
  const codes: { [contract: string]: number } = {};
  for (const [fileName, filePath] of artifacts) {
    const uploadRes = await client.m.upload(
      sender,
      readFileSync(filePath),
      "auto"
    );
    console.log({
      name: fileName,
      codeId: uploadRes.codeId,
      txHash: uploadRes.transactionHash,
    });

    uploadTxs.push(uploadRes);
    codes[toName(fileName)] = uploadRes.codeId;
  }

  ExportReport("3_store", { txs: { upload: uploadTxs }, codes });
}

main().catch(console.error);
