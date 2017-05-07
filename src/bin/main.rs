use std::env;
use std::process;

extern crate portpinger;

fn main() {
    process::exit(run(env::args().collect()));
}

fn run(args: Vec<String>) -> i32 {
    let code = match portpinger::tcp_ping("127.0.0.1:8080") {
        Err(err) => {
            println!("{}", err);
            -1
        }
        Ok(pong) => 0,
    };
    code
}
