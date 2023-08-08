use std::{process::Command, str};

pub fn ethtool(if_name: &str) {
    let output = Command::new("ethtool")
        .arg("-S")
        .arg(if_name)
        .output()
        .expect("failed to execute process");
    if output.status.success() {
        let response = String::from_utf8(output.stdout).expect("failed to parse output");
        println!("output: {response}");
    } else {
        println!("failed with error: {:#?}", output.stderr);
    }
}
