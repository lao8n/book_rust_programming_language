fn main() {
    let x = 5;
    let y = Box::new(x); // in rust you have to explicitly allocate to the heap

    assert_eq!(5, x);
    assert_eq!(5, *y);
}