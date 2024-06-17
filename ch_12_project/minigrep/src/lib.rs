use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> { // Box<dyn Error> means the function will return a type that implements the Error trait, dyn is for dynamic
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // programmers expect new to never fail
    pub fn build(args: &[String]) -> Result<Config, &'static str> { // errors should alwasy be string literals that have static lifetime
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}