mod errors;
mod netlink;
#[allow(dead_code)]
mod tc;
#[allow(dead_code)]
mod fq_codel;

fn main() {
    println!("Hello, world!");

    match tc::get_qdiscs() {
        Ok(qdiscs) => println!("qdiscs: {:?}", qdiscs),
        Err(error) => {
            println!("error: {}", error);
        },
    }
}
