use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> { // Box is trait object - means 'any kind of error'
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}