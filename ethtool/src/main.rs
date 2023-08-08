use std::env;

mod common;
mod ethtool;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        println!("No arguments received, exiting...");
        return;
    }

    let if_name = &args[3];
    let ethtool = ethtool::Ethtool::init(if_name);
    let _ = ethtool.stats();
}
