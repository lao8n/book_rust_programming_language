// crate dependencies use semantic versioning where 0.8.5 means 0.8.5 <= x < 0.9.0
use std::io;
use rand::Rng;
use std::cmp::Ordering;

// default imports are the prelude
fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // rust variables are immutable by default, unless mut 
        let mut guess = String::new(); // :: is an associated function on a type

        io::stdin()
            .read_line(&mut guess) // reference to avoid copying, also immutable by default
            // read_line returns an enum of io::Result, its variants are Ok and Err 
            .expect("Failed to read line"); // correct way is to handle the error - this just makes it crash

        println!("You guessed: {}", guess);

        // can shadow variables
        let guess :u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number){ // Ordering is another enum
            Ordering::Less => println!("Too small!"), // match expression has arms
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println! ("You win!");
                break;
            }
        }
    }
}
