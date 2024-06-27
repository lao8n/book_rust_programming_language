fn main() {
    // 1. dereference a raw pointer 
    let mut num = 5;
    let r1 = &num as *const i32; // can create raw pointers in safe code
    let r2 = &mut num as *mut i32;
    // raw pointers 1. can ignore borrowing rules 2. aren't guaranteed to point to valid memory 3. are allowed to be null
    // 4. don't implement any automatic cleanup
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    // advantage is calling c code
}
