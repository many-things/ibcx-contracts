import {
  DirectSecp256k1HdWallet,
  DirectSecp256k1Wallet,
  OfflineSigner,
} from "@cosmjs/proto-signing";
import { readFileSync } from "fs";
import yaml from "js-yaml";

export const NETWORK = process.env.NETWORK || "localnet";

const chainId = (() => {
  switch (NETWORK) {
    case "testnet":
      return "osmo-test-4";
    case "localnet":
      return "localosmosis";
    case "mainnet":
      return "osmosis-1";
    default:
      throw Error("invalid network");
  }
})();

type ConfigArgs = {
  mnemonic?: string;
  privkey?: string;
  keyring: {
    name: string;
    backend?: string;
  };
  endpoint: string;

  addresses: {
    dao: string;
  };

  assets: {
    [denom: string]: {
      alias?: string;
      unit: number;
      price: number;
      weight: number;
    };
  };
};

class Config {
  constructor(public args: ConfigArgs) {}

  async getSigner(): Promise<OfflineSigner> {
    if (this.args.mnemonic) {
      return DirectSecp256k1HdWallet.fromMnemonic(this.args.mnemonic, {
        prefix: "osmo",
      });
    }

    if (this.args.privkey) {
      return DirectSecp256k1Wallet.fromKey(
        Buffer.from(this.args.privkey, "hex"),
        "osmo"
      );
    }

    throw Error("no mnemonic or privkey");
  }

  get command(): string {
    return `osmosisd --node ${this.args.endpoint} --chain-id ${chainId}`;
  }
}

const args = yaml.load(
  readFileSync(`${process.cwd()}/config.${NETWORK}.yaml`, "utf-8")
) as ConfigArgs;

const config = new Config(args);

export const CHAIN_ID = chainId;

export default config;
