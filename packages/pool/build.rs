use std::{fs, process::Command};

fn main() -> anyhow::Result<()> {
    let resp =
        reqwest::blocking::get("https://lcd.osmosis.zone/osmosis/poolmanager/v1beta1/all-pools")?
            .text()?;

    fs::write("tests/testdata/all-pools.json", resp)?;

    Command::new("node")
        .arg("tests/testdata/conv.js")
        .output()?;

    Ok(())
}
