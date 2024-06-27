// in rust global variables are static variables
static HELLO_WORLD: &str = "Hello, world!";

// difference between constants and immutable static variables is that values in a static variable have a fixed address
// in memory

// mutable static variables are unsafe
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    println!("name is: {HELLO_WORLD}");

    add_to_count(3);

    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}
