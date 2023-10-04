use std::process::Command;

fn main() {
    Command::new("node")
        .arg("tests/testdata/conv.js")
        .output()
        .unwrap();
}
