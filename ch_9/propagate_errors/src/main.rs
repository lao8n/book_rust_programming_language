use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> { // ? only works if return type is Result or Option
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?; // if ever error return the error
    Ok(username)
    // or even shorter
    // fs::read_to_string("hello.txt")
}
// ? can also convert the error using the from function defined in the From trait