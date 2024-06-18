// cargo run -- to poem.txt
// IGNORE_CASE=1 cargo run -- to poem.txt
// $Env:IGNORE_CASE=1; cargo run -- to poem.txt in powershell
// cargo run -- to poem.txt > output.txt but error messages still show in shell
use std::env; // std:env::arg_os if need non unicode characters
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| { // closure
        eprintln!("Problem parsing arguments: {err}"); // eprintln macro that prints to standard error stream
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}