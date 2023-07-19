use std::env;

mod client;
mod common;
mod server;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        println!("No arguments received, exiting...");
        return;
    }

    let opt = &args[3];
    println!("Received option: {opt}");

    match opt.as_str() {
        "client" => client::send(),
        "server" => server::recv(),
        _ => println!("Invalid input!")
    }
}
