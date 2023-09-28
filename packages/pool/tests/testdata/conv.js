const fs = require("fs");
const { pools } = require("./all-pools.json");

const POOL_TYPE_WEIGHT = "/osmosis.gamm.v1beta1.Pool";
const POOL_TYPE_STABLE = "/osmosis.gamm.poolmodels.stableswap.v1beta1.Pool";

const after = {
  pools: pools
    // .filter((pool) =>
    //   [POOL_TYPE_WEIGHT, POOL_TYPE_STABLE].includes(pool["@type"])
    // )
    .map((pool) => {
      // no mutation
      if (pool["@type"] !== POOL_TYPE_STABLE) {
        return pool;
      }

      pool.scaling_factors = pool.scaling_factors.map((factor) =>
        Number(factor)
      );
      return pool;
    }),
};

fs.writeFileSync("./all-pools-after.json", JSON.stringify(after, null, 2));
