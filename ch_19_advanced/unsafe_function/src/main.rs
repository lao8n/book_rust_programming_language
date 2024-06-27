fn main() {
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

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);


    // Foreign Function Interface FFI and Application Binary Interface ABI
    // other examples include accessing fields of a union
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // call rust eternally
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
}
