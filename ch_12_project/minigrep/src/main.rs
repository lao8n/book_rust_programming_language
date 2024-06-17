use std::env; // std:env::arg_os if need non unicode characters
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // collect makes iterator into vector
    let config = Config::build(&args).unwrap_or_else(|err| { // closure
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}