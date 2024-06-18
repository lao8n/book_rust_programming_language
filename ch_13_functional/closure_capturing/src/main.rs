// 3 ways to pass in a parameter
// 1. borrow immutably
// 2. borrow mutably
// 3. take ownership
use std::thread;

// correspondingly can 
fn main() {
    // 1. borrow immutably
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    // 2. borrow mutably
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);
    // cannot have an immutable borrow here with println! as still borrowing mutably
    borrows_mutably();
    println!("After calling closure: {list:?}");

    // 3. move
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // we need the move because the main thread might finish before the child thread and then immutable reference would be invalid
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}