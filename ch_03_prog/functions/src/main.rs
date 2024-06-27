fn main() {
    let x = five();
    println!("The value of x is {x}");
}

fn five() -> i32 {
    5 // no semi-colon as it is an expression not a statement, an expression returns something, a statement does some action
}