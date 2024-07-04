// 5 things unsafe rust lets you do
// 3. access or modify a mutable static variable
// 4. implement an unsafe trait
// 5. access fields of a union
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

    // 2. call an unsafe function or method
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
    use std::slice;

    // wrapping unsafe method with safe method
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();
    
        assert!(mid <= len);
    
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    // 3. access or modify a mutable static variable

}
