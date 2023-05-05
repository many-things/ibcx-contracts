import sdk from "@many-things/ibcx-contracts-sdk";

import config from "../config";
import { LoadReport, makeClient } from "../util";

type DeployContractReport = {
  contracts: {
    core: string;
    periphery: string;
  };
};

async function main() {
  const signer = await config.getSigner();
  const [{ address: sender }] = await signer.getAccounts();

  const base = await makeClient(signer);

  const { contracts: addrs } = LoadReport<DeployContractReport>("4_deploy")!;

  const client = {
    core: new sdk.Core.CoreClient(base.m, sender, addrs.core),
    perp: new sdk.Periphery.PeripheryClient(base.m, sender, addrs.periphery),
    ...base,
  };

  const portfolio = await client.core.getPortfolio({});
  console.log(portfolio);
}

main().catch(console.error);
