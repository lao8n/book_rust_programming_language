fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // can do basic operations
    println!("Three hours in seconds is {THREE_HOURS_IN_SECONDS}");

    let x = 5; // immutable
    let x = x + 1; // shadowing - perform transformations but still immutable
    {
        let x = x * 2; // shadowing - can also change types
        println!("The value of x in the inner scope is {x}");
    }
    println!("The value of x is {x}");
}
