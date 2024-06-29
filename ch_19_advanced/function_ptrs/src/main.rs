fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg) // unlike closures fn is a type rather than a trait
    // fn pointers always implement all 3 of the closure traits 1. Fn 2. FnMut and FnOnce
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");
}