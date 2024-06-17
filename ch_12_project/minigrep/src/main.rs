use std::env; // std:env::arg_os if need non unicode characters
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect(); // collect makes iterator into vector
    let config = Config::build(&args).unwrap_or_else(|err| { // closure
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // programmers expect new to never fail
    fn build(args: &[String]) -> Result<Config, &'static str> { // errors should alwasy be string literals that have static lifetime
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}