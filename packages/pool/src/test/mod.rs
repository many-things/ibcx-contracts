pub mod pool;

use std::{
    env::current_dir,
    fs,
    path::{Path, PathBuf},
};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;
use ibcx_interface::periphery::{SwapInfo, SwapInfosCompact};

pub fn testdata(v: &str) -> PathBuf {
    let cwd = current_dir().unwrap();
    let database = cwd.join("src/testdata");
    database.join(v)
}

pub fn load_swap_info<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<SwapInfo>> {
    let swap_info_raw = fs::read_to_string(path)?;
    let swap_info: Vec<SwapInfo> = serde_json::from_str::<SwapInfosCompact>(&swap_info_raw)?.into();
    Ok(swap_info)
}

pub fn load_index_units<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<(String, Decimal)>> {
    #[cw_serde]
    struct IndexUnit {
        pub asset: String,
        pub unit: Decimal,
    }

    let index_units_raw = fs::read_to_string(path)?;
    let index_units: Vec<IndexUnit> = serde_json::from_str(&index_units_raw)?;

    Ok(index_units.into_iter().map(|v| (v.asset, v.unit)).collect())
}
