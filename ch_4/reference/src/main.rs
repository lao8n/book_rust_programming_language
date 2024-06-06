fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1} is {len}");
}
// unlike pointer types in C and C++, references in Rust are always valid
fn calculate_length(s: &String) -> usize {
    s.len()
}
// because it does not own it and the value it points to will not be dropped when the reference goes out of scope
// borrowing = creating a reference
// references are immutable by default - 