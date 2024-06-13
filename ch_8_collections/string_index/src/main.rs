fn main() {
    let s1 = String::from("hello");
    let h = s1[0];
    // compiles to error cannot be indexed
    // strings are a wrapper on Vec<u8>
    // because can have multiple bytes become a single unicode character cannot 

    // better to iterate with 
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
