use std::{fs, process::Command};

const TEST_FILE_PATH: &str = "src/testdata/all-pools.json";
const TEST_FILE_CONV_PATH: &str = "src/testdata/conv.js";
const ALL_POOL_QUERY_URL: &str = "https://lcd.osmosis.zone/osmosis/poolmanager/v1beta1/all-pools";

fn main() -> anyhow::Result<()> {
    match fs::metadata(TEST_FILE_PATH) {
        Ok(_) => {
            println!("File exists! skipping...");
            return Ok(());
        }
        Err(_) => println!("File does not exist! fetching..."),
    }

    let resp = reqwest::blocking::get(ALL_POOL_QUERY_URL)?.text()?;

    fs::write(TEST_FILE_PATH, resp)?;

    Command::new("node").arg(TEST_FILE_CONV_PATH).output()?;

    Ok(())
}
