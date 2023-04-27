import { osmosis } from "osmojs";
const { createRPCQueryClient } = osmosis.ClientFactory;

import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import path from "path";
import { readFileSync, readdirSync } from "fs";
import { GasPrice } from "@cosmjs/stargate";

import config from "./config";
import { aminoTypes, registry } from "./codec";

const pwd = process.cwd();
const root = path.join(`${pwd}/../../`);
const artifactsPath = path.join(root, "artifacts");
const artifacts = readdirSync(artifactsPath)
  .filter((file) => file.includes(".wasm"))
  .map((v) => [v, path.join(artifactsPath, v)]);

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

  for (const [fileName, filePath] of artifacts) {
    const uploadRes = await cosmwasmClient.upload(
      wallet.address,
      readFileSync(filePath),
      "auto"
    );
    console.log({
      name: fileName,
      codeId: uploadRes.codeId,
      txHash: uploadRes.transactionHash,
    });
  }
}

main().catch(console.error);
