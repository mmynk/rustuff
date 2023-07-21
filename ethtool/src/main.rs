mod common;
mod ethtool;

fn main() {
    println!("Hello, world!");
    ethtool::connect();
}
