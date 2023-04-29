import { DirectSecp256k1HdWallet, OfflineSigner } from "@cosmjs/proto-signing";
import { readFileSync } from "fs";
import yaml from "js-yaml";

type ConfigArgs = {
  mnemonic: string;
  endpoint: string;

  addresses: {
    dao: string;
  };

  codes: {
    airdrop: number;
    core: number;
    faucet: number;
    periphery: number;
  };
};

class Config {
  constructor(public args: ConfigArgs) {}

  async getSigner(): Promise<OfflineSigner> {
    return DirectSecp256k1HdWallet.fromMnemonic(this.args.mnemonic, {
      prefix: "osmo",
    });
  }
}

const network = process.env.NETWORK || "testnet";

const args = yaml.load(
  readFileSync(`${process.cwd()}/config.${network}.yaml`, "utf-8")
) as ConfigArgs;

const config = new Config(args);

export default config;
