extern crate rucp;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let outcome = rucp::run(args);
    match outcome {
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1)
        }
        _ =>{}
    }
}
